use super::*;

pub(crate) fn lower_abi_decode_direct(
    args: &[Expression],
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) -> Option<bool> {
    if args.len() != 2 {
        return None;
    }

    let types = abi_decode_value_types(&args[1], ctx)?;
    if types.is_empty() {
        return None;
    }

    // Each declared decode type must be either static (single- or multi-slot,
    // mirroring the canonical ENCODE side which emits per-field 32-byte slots
    // for all-static structs) or one of the dynamic types we know how to walk
    // (string / bytes / T[] for static T). Bug #24/#25 fix: previously we only
    // accepted static-1-slot types here and fell through to
    // `StdLib.deserialize` (which is Neo-native, NOT EVM-ABI-compatible) for
    // any dynamic type, producing an opaque StackItem callers couldn't
    // `.length` / re-decode. The multi-slot extension closes the same
    // asymmetry for all-static structs: `abi.encode(S(7,9))` produces 64
    // canonical bytes, so `abi.decode(buf, (S))` must walk those slots
    // instead of handing raw ABI bytes to `StdLib.deserialize`.
    let supported = types.iter().all(|value_type| {
        abi_static_slot_count(value_type).is_some()
            || abi_dynamic_decode_value_type_is_supported(value_type)
    });
    if !supported {
        return None;
    }

    let pre_len = instructions.len();
    if !lower_expression(&args[0], ctx, instructions) {
        instructions.truncate(pre_len);
        return Some(false);
    }

    let buffer_local = ctx.allocate_local("__abi_decode_buf".to_string(), None);
    instructions.push(Instruction::StoreLocal(buffer_local));

    let any_dynamic = types.iter().any(abi_dynamic_decode_value_type_is_supported);

    if !any_dynamic {
        // All declared types are static; total payload size is the SUM of
        // each type's slot count × 32 (multi-slot structs occupy one slot
        // per field, mirroring the encode side). Mismatches panic 0x41.
        let expected_bytes: usize = types
            .iter()
            .map(|value_type| abi_static_slot_count(value_type).unwrap_or(1) * 32)
            .sum();
        let decode_ok_label = ctx.next_label();
        instructions.push(Instruction::LoadLocal(buffer_local));
        instructions.push(Instruction::GetSize);
        instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
            BigInt::from(expected_bytes as u64),
        )));
        // Only an UNDER-length buffer reverts; `abi.decode` ignores trailing
        // bytes (matches solc/ethers). Use `<` rather than `!=`.
        instructions.push(Instruction::BinaryOp(BinaryOperator::Lt));
        instructions.push(Instruction::JumpIf {
            target: decode_ok_label,
        });
        emit_panic(0x41, instructions);
        instructions.push(Instruction::Label(decode_ok_label));
    }
    // For the dynamic case we cannot statically know the total size; the
    // per-slot decoders bound-check the length / offset reads themselves.

    if types.len() == 1 {
        let value_type = &types[0];
        if abi_dynamic_decode_value_type_is_supported(value_type) {
            emit_abi_decode_dynamic_top_level(buffer_local, value_type, ctx, instructions);
        } else {
            emit_abi_decode_static_slot(buffer_local, 0, value_type, ctx, instructions);
        }
        return Some(true);
    }

    let tuple_local = ctx.allocate_local(
        "__abi_decode_tuple".to_string(),
        Some(ValueType::Array(Box::new(ValueType::Any))),
    );
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
        BigInt::from(types.len() as u64),
    )));
    instructions.push(Instruction::NewArray {
        element_type: ValueType::Any,
    });
    instructions.push(Instruction::StoreLocal(tuple_local));

    // Track the running HEAD slot separately from the tuple index: a
    // multi-slot static member (all-static struct) consumes one head slot
    // per field, shifting every subsequent member's slot position.
    let mut head_slot = 0usize;
    for (index, value_type) in types.iter().enumerate() {
        instructions.push(Instruction::LoadLocal(tuple_local));
        instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
            BigInt::from(index as u64),
        )));
        if abi_dynamic_decode_value_type_is_supported(value_type) {
            // Tuple member is dynamic: head slot at index `head_slot` is a
            // u256 byte-offset (relative to the start of the encoded
            // buffer) pointing at the value's tail. Walk that offset to
            // reach the length+payload (string/bytes) or length+elements
            // (static-element array).
            emit_abi_decode_dynamic_tuple_member(
                buffer_local,
                head_slot,
                value_type,
                ctx,
                instructions,
            );
            head_slot += 1;
        } else {
            emit_abi_decode_static_slot(buffer_local, head_slot, value_type, ctx, instructions);
            head_slot += abi_static_slot_count(value_type).unwrap_or(1);
        }
        instructions.push(Instruction::ArraySet);
    }

    instructions.push(Instruction::LoadLocal(tuple_local));
    Some(true)
}

/// Decode a single top-level dynamic value from the encoded buffer in
/// `buffer_local`.
///
/// Layout produced by `abi.encode(x)` for a single dynamic `x`:
///   `BE32(0x20) || <tail at offset 0x20>`
/// i.e. a leading 32-byte head holding the offset 0x20 (= 32) of the tail,
/// followed by the tail payload (length + bytes for `string`/`bytes`, or
/// length + elements for `T[]`).
///
/// Stack on exit: `[decoded_value]` (ByteString for string/bytes, Array
/// of element values for `T[]`).
pub(crate) fn emit_abi_decode_dynamic_top_level(
    buffer_local: usize,
    value_type: &ValueType,
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) {
    // Read the ACTUAL tail offset from head slot 0 rather than assuming the
    // canonical 0x20: a non-canonical (but valid) encoder may place the tail at
    // a different offset, and EVM decoders honor the encoded head pointer.
    let tmp_id = ctx.next_label();
    let offset_local = ctx.allocate_local(format!("__abi_dyn_top_off_{tmp_id}"), None);
    emit_abi_decode_u256_at(buffer_local, 0, ctx, instructions);
    instructions.push(Instruction::StoreLocal(offset_local));
    emit_abi_decode_dynamic_tail_runtime(
        buffer_local,
        offset_local,
        value_type,
        0,
        ctx,
        instructions,
    );
}

/// Decode a dynamic tuple member at head index `index`.
///
/// Reads the u256 offset from `buffer[index*32 .. index*32+32]` (which is
/// the byte offset of the member's tail relative to the start of the
/// encoded buffer) and dispatches to [`emit_abi_decode_dynamic_tail`] at
/// that runtime offset.
pub(crate) fn emit_abi_decode_dynamic_tuple_member(
    buffer_local: usize,
    index: usize,
    value_type: &ValueType,
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) {
    // Read the offset slot. We can fold it through a temp local.
    let tmp_id = ctx.next_label();
    let offset_local = ctx.allocate_local(format!("__abi_dyn_off_{tmp_id}"), None);
    emit_abi_decode_u256_at(buffer_local, index * 32, ctx, instructions);
    instructions.push(Instruction::StoreLocal(offset_local));
    emit_abi_decode_dynamic_tail_runtime(
        buffer_local,
        offset_local,
        value_type,
        0,
        ctx,
        instructions,
    );
}

/// Same as [`emit_abi_decode_dynamic_tail`] but with the tail offset
/// supplied at runtime via `offset_local`. Used for tuple members where
/// the offset is read from a head slot.
pub(crate) fn emit_abi_decode_dynamic_tail_runtime(
    buffer_local: usize,
    offset_local: usize,
    value_type: &ValueType,
    depth: usize,
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) {
    match value_type {
        ValueType::ByteArray { fixed_len: None } | ValueType::String => {
            emit_abi_decode_bytes_tail_runtime(buffer_local, offset_local, ctx, instructions);
        }
        ValueType::Array(element_type) if abi_static_slot_count(element_type).is_some() => {
            emit_abi_decode_static_element_array_tail_runtime(
                buffer_local,
                offset_local,
                element_type,
                ctx,
                instructions,
            );
        }
        // `T[]` with a dynamic element type (string[], bytes[], T[][]): each
        // head entry is itself a u256 offset (relative to the start of the
        // head section) pointing at that element's tail — decode recursively.
        ValueType::Array(element_type) => {
            emit_abi_decode_nested_array_tail_runtime(
                buffer_local,
                offset_local,
                element_type,
                depth,
                ctx,
                instructions,
            );
        }
        // Dynamic struct: walk the tuple head/tail at this offset.
        ValueType::Struct { fields, .. } => {
            emit_abi_decode_dynamic_struct_tail_runtime(
                buffer_local,
                offset_local,
                fields,
                depth,
                ctx,
                instructions,
            );
        }
        _ => {
            instructions.push(Instruction::PushLiteral(
                LiteralValue::ByteArray(Vec::new()),
            ));
        }
    }
}

/// Read the (length-/offset-shaped) u256 stored at byte offset
/// `byte_offset` inside `buffer_local` (BE-encoded 32-byte slot) and
/// push it as a NeoVM `Integer`.
///
/// The runtime's `CONVERT → Integer` of a >8-byte ByteArray returns a
/// signed-LE `ByteArray`, NOT an `Integer`, to preserve precision (see
/// `convert_item` in `runtime/execution/execution_impl_part3_conversion.rs`
/// and the Task #111 history). That breaks downstream callers (SUBSTR
/// `pop_usize`) which require a real Integer.
///
/// EVM-canonical length/offset slots are bounded by `usize::MAX` in
/// practice (calldata cannot exceed a few MB on any real chain, let alone
/// 2^64 bytes), so it is safe to read only the low 8 bytes of the slot
/// and rely on the ≤8-byte fast path of `convert_item` which produces an
/// `Integer(i64)`. The high 24 bytes of every well-formed length/offset
/// slot are zero — if a malicious / corrupted payload claims a length
/// exceeding `i64::MAX`, the subsequent `SUBSTR` will fault on bounds
/// anyway.
/// Reject a non-canonical length/offset slot whose HIGH 24 bytes are nonzero.
///
/// The slot readers trust only the low 8 bytes (calldata lengths/offsets never
/// exceed a few MB), but a crafted payload could set bytes [0..24) to encode a
/// value in `[2^64, 2^192)` whose low 64 bits happen to be small and in-bounds.
/// Reading only the low 8 bytes would then silently use the truncated value and
/// decode the wrong region instead of reverting. `offset_push` pushes the byte
/// offset of the slot start; Panic(0x41) fires when any high byte is set. A
/// conformant slot always has zero high bytes, so this never faults valid input.
pub(crate) fn emit_abi_decode_slot_high_bits_guard(
    buffer_local: usize,
    offset_push: Instruction,
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) {
    let ok = ctx.next_label();
    instructions.push(Instruction::LoadLocal(buffer_local));
    instructions.push(offset_push);
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
        BigInt::from(24u8),
    )));
    instructions.push(Instruction::Substr);
    instructions.push(Instruction::Convert {
        target: ConvertTarget::ByteArray,
    });
    instructions.push(Instruction::PushLiteral(LiteralValue::ByteArray(vec![
        0u8;
        24
    ])));
    instructions.push(Instruction::BinaryOp(BinaryOperator::Eq));
    instructions.push(Instruction::LogicalNot); // true when the high 24 bytes are NONZERO
                                                // JumpIf -> JMPIFNOT: branch to `ok` when the nonzero flag is FALSE (i.e.
                                                // the high bytes are all zero); otherwise fall through to the panic.
    instructions.push(Instruction::JumpIf { target: ok });
    emit_panic(0x41, instructions);
    instructions.push(Instruction::Label(ok));
}

pub(crate) fn emit_abi_decode_u256_at(
    buffer_local: usize,
    byte_offset: usize,
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) {
    emit_abi_decode_slot_high_bits_guard(
        buffer_local,
        Instruction::PushLiteral(LiteralValue::Integer(BigInt::from(byte_offset as u64))),
        ctx,
        instructions,
    );
    instructions.push(Instruction::LoadLocal(buffer_local));
    // The low 8 bytes of the 32-byte BE slot live at `byte_offset + 24`.
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
        BigInt::from((byte_offset + 24) as u64),
    )));
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
        BigInt::from(8u8),
    )));
    instructions.push(Instruction::Substr);
    instructions.push(Instruction::Convert {
        target: ConvertTarget::ByteArray,
    });
    // Reverse BE→LE so the ≤8-byte CONVERT→Integer path interprets the
    // bytes correctly.
    materialize_byte_array_buffer(ctx, instructions, true);
    instructions.push(Instruction::Convert {
        target: ConvertTarget::Integer,
    });
}

/// Read the u256 stored at the runtime byte offset held in `offset_local`.
/// Same low-8-bytes shortcut as [`emit_abi_decode_u256_at`].
pub(crate) fn emit_abi_decode_u256_at_runtime(
    buffer_local: usize,
    offset_local: usize,
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) {
    emit_abi_decode_slot_high_bits_guard(
        buffer_local,
        Instruction::LoadLocal(offset_local),
        ctx,
        instructions,
    );
    instructions.push(Instruction::LoadLocal(buffer_local));
    // adj_offset = offset_local + 24 (skip the 24 BE high-zero bytes).
    instructions.push(Instruction::LoadLocal(offset_local));
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
        BigInt::from(24u8),
    )));
    instructions.push(Instruction::BinaryOp(BinaryOperator::Add));
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
        BigInt::from(8u8),
    )));
    instructions.push(Instruction::Substr);
    instructions.push(Instruction::Convert {
        target: ConvertTarget::ByteArray,
    });
    materialize_byte_array_buffer(ctx, instructions, true);
    instructions.push(Instruction::Convert {
        target: ConvertTarget::Integer,
    });
}

/// Decode a `string` / `bytes` tail at a runtime byte offset.
pub(crate) fn emit_abi_decode_bytes_tail_runtime(
    buffer_local: usize,
    offset_local: usize,
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) {
    let tmp_id = ctx.next_label();
    let len_local = ctx.allocate_local(format!("__abi_dec_rblen_{tmp_id}"), None);

    emit_abi_decode_u256_at_runtime(buffer_local, offset_local, ctx, instructions);
    instructions.push(Instruction::StoreLocal(len_local));

    instructions.push(Instruction::LoadLocal(buffer_local));
    // payload starts at offset_local + 32.
    instructions.push(Instruction::LoadLocal(offset_local));
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
        BigInt::from(32u8),
    )));
    instructions.push(Instruction::BinaryOp(BinaryOperator::Add));
    instructions.push(Instruction::LoadLocal(len_local));
    instructions.push(Instruction::Substr);
    instructions.push(Instruction::Convert {
        target: ConvertTarget::ByteArray,
    });
}

/// Decode a `T[]` tail at a runtime byte offset (tuple-member case).
pub(crate) fn emit_abi_decode_static_element_array_tail_runtime(
    buffer_local: usize,
    offset_local: usize,
    element_type: &ValueType,
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) {
    let element_byte_len = abi_static_slot_count(element_type).unwrap_or(1) * 32;
    let tmp_id = ctx.next_label();
    let len_local = ctx.allocate_local(format!("__abi_dec_ralen_{tmp_id}"), None);
    let base_local = ctx.allocate_local(format!("__abi_dec_rabase_{tmp_id}"), None);
    let arr_local = ctx.allocate_local(
        format!("__abi_dec_rarr_{tmp_id}"),
        Some(ValueType::Array(Box::new(element_type.clone()))),
    );
    let idx_local = ctx.allocate_local(format!("__abi_dec_raidx_{tmp_id}"), None);
    let elem_off_local = ctx.allocate_local(format!("__abi_dec_raelmoff_{tmp_id}"), None);

    emit_abi_decode_u256_at_runtime(buffer_local, offset_local, ctx, instructions);
    instructions.push(Instruction::StoreLocal(len_local));

    // base = offset_local + 32 (first element).
    instructions.push(Instruction::LoadLocal(offset_local));
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
        BigInt::from(32u8),
    )));
    instructions.push(Instruction::BinaryOp(BinaryOperator::Add));
    instructions.push(Instruction::StoreLocal(base_local));

    instructions.push(Instruction::LoadLocal(len_local));
    instructions.push(Instruction::NewArray {
        element_type: element_type.clone(),
    });
    instructions.push(Instruction::StoreLocal(arr_local));

    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
        BigInt::zero(),
    )));
    instructions.push(Instruction::StoreLocal(idx_local));

    let loop_label = ctx.next_label();
    let end_label = ctx.next_label();
    instructions.push(Instruction::Label(loop_label));
    instructions.push(Instruction::LoadLocal(idx_local));
    instructions.push(Instruction::LoadLocal(len_local));
    instructions.push(Instruction::BinaryOp(BinaryOperator::Lt));
    instructions.push(Instruction::JumpIf { target: end_label });

    // elem_off = base + idx * element_byte_len.
    instructions.push(Instruction::LoadLocal(idx_local));
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
        BigInt::from(element_byte_len as u64),
    )));
    instructions.push(Instruction::BinaryOp(BinaryOperator::Mul));
    instructions.push(Instruction::LoadLocal(base_local));
    instructions.push(Instruction::BinaryOp(BinaryOperator::Add));
    instructions.push(Instruction::StoreLocal(elem_off_local));

    instructions.push(Instruction::LoadLocal(arr_local));
    instructions.push(Instruction::LoadLocal(idx_local));
    emit_abi_decode_static_slot_at_runtime_offset(
        buffer_local,
        elem_off_local,
        element_type,
        ctx,
        instructions,
    );
    instructions.push(Instruction::ArraySet);

    instructions.push(Instruction::LoadLocal(idx_local));
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
        BigInt::one(),
    )));
    instructions.push(Instruction::BinaryOp(BinaryOperator::Add));
    instructions.push(Instruction::StoreLocal(idx_local));
    instructions.push(Instruction::Jump { target: loop_label });
    instructions.push(Instruction::Label(end_label));

    instructions.push(Instruction::LoadLocal(arr_local));
}

/// Decode a `T[]` tail whose element type `T` is itself dynamic (string[],
/// bytes[], T[][]) at a runtime byte offset. The mirror of
/// [`emit_abi_dynamic_nested_array_tail`]: read the length, then for each
/// element read its offset (relative to the start of the head section) from
/// the head entry and decode the element tail recursively at that absolute
/// offset. Stack on exit: `[Array<decoded elements>]`.
pub(crate) fn emit_abi_decode_nested_array_tail_runtime(
    buffer_local: usize,
    offset_local: usize,
    element_type: &ValueType,
    depth: usize,
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) {
    // Six reusable scratch locals for this nesting depth (see the encode-side
    // `emit_abi_dynamic_nested_array_tail` for the pooling rationale).
    let scratch = ctx.abi_nested_scratch_locals(depth, 6);
    let len_local = scratch[0];
    let base_local = scratch[1];
    let arr_local = scratch[2];
    let idx_local = scratch[3];
    let head_off_local = scratch[4];
    let elem_off_local = scratch[5];

    // n = length word at the tail offset.
    emit_abi_decode_u256_at_runtime(buffer_local, offset_local, ctx, instructions);
    instructions.push(Instruction::StoreLocal(len_local));

    // base = offset_local + 32 (start of the head section; element offsets are
    // relative to here).
    instructions.push(Instruction::LoadLocal(offset_local));
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
        BigInt::from(32u8),
    )));
    instructions.push(Instruction::BinaryOp(BinaryOperator::Add));
    instructions.push(Instruction::StoreLocal(base_local));

    instructions.push(Instruction::LoadLocal(len_local));
    instructions.push(Instruction::NewArray {
        element_type: element_type.clone(),
    });
    instructions.push(Instruction::StoreLocal(arr_local));

    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
        BigInt::zero(),
    )));
    instructions.push(Instruction::StoreLocal(idx_local));

    let loop_label = ctx.next_label();
    let end_label = ctx.next_label();
    instructions.push(Instruction::Label(loop_label));
    instructions.push(Instruction::LoadLocal(idx_local));
    instructions.push(Instruction::LoadLocal(len_local));
    instructions.push(Instruction::BinaryOp(BinaryOperator::Lt));
    instructions.push(Instruction::JumpIf { target: end_label });

    // head_off = base + idx*32 (byte position of element idx's head entry).
    instructions.push(Instruction::LoadLocal(idx_local));
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
        BigInt::from(32u8),
    )));
    instructions.push(Instruction::BinaryOp(BinaryOperator::Mul));
    instructions.push(Instruction::LoadLocal(base_local));
    instructions.push(Instruction::BinaryOp(BinaryOperator::Add));
    instructions.push(Instruction::StoreLocal(head_off_local));

    // elem_off = base + relative_offset(head_off).
    instructions.push(Instruction::LoadLocal(base_local));
    emit_abi_decode_u256_at_runtime(buffer_local, head_off_local, ctx, instructions);
    instructions.push(Instruction::BinaryOp(BinaryOperator::Add));
    instructions.push(Instruction::StoreLocal(elem_off_local));

    // arr[idx] = decode_dynamic_tail(elem_off, element_type)  (recursive)
    instructions.push(Instruction::LoadLocal(arr_local));
    instructions.push(Instruction::LoadLocal(idx_local));
    emit_abi_decode_dynamic_tail_runtime(
        buffer_local,
        elem_off_local,
        element_type,
        depth + 1,
        ctx,
        instructions,
    );
    instructions.push(Instruction::ArraySet);

    instructions.push(Instruction::LoadLocal(idx_local));
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
        BigInt::one(),
    )));
    instructions.push(Instruction::BinaryOp(BinaryOperator::Add));
    instructions.push(Instruction::StoreLocal(idx_local));
    instructions.push(Instruction::Jump { target: loop_label });
    instructions.push(Instruction::Label(end_label));

    instructions.push(Instruction::LoadLocal(arr_local));
}

/// Decode a DYNAMIC struct (≥1 dynamic field) tail at a runtime byte offset
/// into a fresh `StackItem::Array` of field values — the mirror of
/// [`emit_abi_dynamic_struct_tail`]. The head section layout is statically
/// known from the field types (static fields inline, dynamic fields one
/// offset word each), so the per-field head position is a compile-time
/// constant; dynamic fields read a relative offset and decode their tail
/// recursively. Stack on exit: `[Array<field values>]`.
pub(crate) fn emit_abi_decode_dynamic_struct_tail_runtime(
    buffer_local: usize,
    offset_local: usize,
    fields: &[StructField],
    depth: usize,
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) {
    let tmp_id = ctx.next_label();
    let struct_local = ctx.allocate_local(format!("__abi_dec_dstruct_{tmp_id}"), None);
    let field_off_local = ctx.allocate_local(format!("__abi_dec_dstruct_foff_{tmp_id}"), None);

    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
        BigInt::from(fields.len() as u64),
    )));
    instructions.push(Instruction::NewArray {
        element_type: ValueType::Any,
    });
    instructions.push(Instruction::StoreLocal(struct_local));

    let mut head_byte_offset = 0usize;
    for (field_index, field) in fields.iter().enumerate() {
        if abi_value_type_is_dynamic(&field.ty) {
            // head slot position = offset_local + head_byte_offset.
            instructions.push(Instruction::LoadLocal(offset_local));
            instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                BigInt::from(head_byte_offset as u64),
            )));
            instructions.push(Instruction::BinaryOp(BinaryOperator::Add));
            instructions.push(Instruction::StoreLocal(field_off_local));
            // absolute tail offset = offset_local + rel(head slot).
            instructions.push(Instruction::LoadLocal(offset_local));
            emit_abi_decode_u256_at_runtime(buffer_local, field_off_local, ctx, instructions);
            instructions.push(Instruction::BinaryOp(BinaryOperator::Add));
            instructions.push(Instruction::StoreLocal(field_off_local));
            // arr[field_index] = decode_dynamic_tail(field_off, field.ty)
            instructions.push(Instruction::LoadLocal(struct_local));
            instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                BigInt::from(field_index as u64),
            )));
            emit_abi_decode_dynamic_tail_runtime(
                buffer_local,
                field_off_local,
                &field.ty,
                depth + 1,
                ctx,
                instructions,
            );
            instructions.push(Instruction::ArraySet);
            head_byte_offset += 32;
        } else {
            // static field at offset_local + head_byte_offset.
            instructions.push(Instruction::LoadLocal(offset_local));
            instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                BigInt::from(head_byte_offset as u64),
            )));
            instructions.push(Instruction::BinaryOp(BinaryOperator::Add));
            instructions.push(Instruction::StoreLocal(field_off_local));
            instructions.push(Instruction::LoadLocal(struct_local));
            instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                BigInt::from(field_index as u64),
            )));
            emit_abi_decode_static_slot_at_runtime_offset(
                buffer_local,
                field_off_local,
                &field.ty,
                ctx,
                instructions,
            );
            instructions.push(Instruction::ArraySet);
            head_byte_offset += abi_static_slot_count(&field.ty).unwrap_or(1) * 32;
        }
    }

    instructions.push(Instruction::LoadLocal(struct_local));
}

/// Convert the little-endian byte Buffer on top of the stack into an
/// UNSIGNED integer.
///
/// NeoVM `CONVERT → Integer` interprets byte buffers as SIGNED
/// little-endian, so a canonical uint256 slot with the top bit set
/// (value >= 2^255) would decode negative — `abi.decode(abi.encode(
/// type(uint256).max), (uint256))` yielded -1. When the signed
/// interpretation is negative we append a single 0x00 sign byte before
/// the convert, matching the sign-byte discipline the wide bitwise path
/// already uses (`u256_bigint_to_stack_item` in
/// `runtime/execution/helpers/bitwise.rs`, Task #118). The append is
/// CONDITIONAL on negativity so the common case (< 2^255) keeps the
/// plain 32-byte convert.
///
/// Stack on entry: `[le_bytes_buffer]`.
/// Stack on exit:  `[unsigned_integer]`.
pub(crate) fn emit_le_buffer_to_unsigned_integer(
    _ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) {
    // The 32-byte little-endian slot buffer IS the conformant uint256
    // representation: a 32-byte two's-complement integer. Reading it as a signed
    // NeoVM integer yields the canonical value — a `uint256 >= 2^255` reads back
    // as its negative two's-complement, byte-identical to the same value produced
    // by a literal or by arithmetic, so `abi.decode(abi.encode(x)) == x` holds.
    //
    // The previous code appended a `0x00` sign byte to coerce values >= 2^255
    // into a POSITIVE 33-byte magnitude — the old unsigned-magnitude model, which
    // is non-conformant (a real node stores two's-complement) and not equal to
    // the two's-complement form the rest of the runtime uses.
    instructions.push(Instruction::Convert {
        target: ConvertTarget::Integer,
    });
}

/// Decode a single static element at a runtime byte offset inside
/// `buffer_local`. Mirrors [`emit_abi_decode_static_slot`] but operates
/// at a runtime offset rather than a compile-time slot index.
pub(crate) fn emit_abi_decode_static_slot_at_runtime_offset(
    buffer_local: usize,
    offset_local: usize,
    value_type: &ValueType,
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) {
    match value_type {
        // Unsigned integers must NOT be read with NeoVM's signed-LE
        // CONVERT semantics — see `emit_le_buffer_to_unsigned_integer`.
        ValueType::Integer { signed: false, .. } => {
            instructions.push(Instruction::LoadLocal(buffer_local));
            instructions.push(Instruction::LoadLocal(offset_local));
            instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                BigInt::from(32u8),
            )));
            instructions.push(Instruction::Substr);
            instructions.push(Instruction::Convert {
                target: ConvertTarget::ByteArray,
            });
            materialize_byte_array_buffer(ctx, instructions, true);
            emit_le_buffer_to_unsigned_integer(ctx, instructions);
        }
        ValueType::Integer { signed: true, .. } | ValueType::Boolean => {
            instructions.push(Instruction::LoadLocal(buffer_local));
            instructions.push(Instruction::LoadLocal(offset_local));
            instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                BigInt::from(32u8),
            )));
            instructions.push(Instruction::Substr);
            instructions.push(Instruction::Convert {
                target: ConvertTarget::ByteArray,
            });
            materialize_byte_array_buffer(ctx, instructions, true);
            instructions.push(Instruction::Convert {
                target: ConvertTarget::Integer,
            });
        }
        // All-static struct element: decode each field from its consecutive
        // 32-byte slot into a fresh `StackItem::Array` (the Array-of-fields
        // shape the struct-constructor lowering produces and the encode
        // side reads back via ArrayGet).
        ValueType::Struct { fields, .. } => {
            let tmp_id = ctx.next_label();
            let struct_local = ctx.allocate_local(format!("__abi_dec_rstruct_{tmp_id}"), None);
            let field_off_local =
                ctx.allocate_local(format!("__abi_dec_rstruct_off_{tmp_id}"), None);
            instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                BigInt::from(fields.len() as u64),
            )));
            instructions.push(Instruction::NewArray {
                element_type: ValueType::Any,
            });
            instructions.push(Instruction::StoreLocal(struct_local));
            let mut field_byte_offset = 0usize;
            for (field_index, field) in fields.iter().enumerate() {
                // field_off = offset_local + field_byte_offset.
                instructions.push(Instruction::LoadLocal(offset_local));
                instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                    BigInt::from(field_byte_offset as u64),
                )));
                instructions.push(Instruction::BinaryOp(BinaryOperator::Add));
                instructions.push(Instruction::StoreLocal(field_off_local));

                instructions.push(Instruction::LoadLocal(struct_local));
                instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                    BigInt::from(field_index as u64),
                )));
                emit_abi_decode_static_slot_at_runtime_offset(
                    buffer_local,
                    field_off_local,
                    &field.ty,
                    ctx,
                    instructions,
                );
                instructions.push(Instruction::ArraySet);
                field_byte_offset += abi_static_slot_count(&field.ty).unwrap_or(1) * 32;
            }
            instructions.push(Instruction::LoadLocal(struct_local));
        }
        ValueType::Address => {
            // EVM ABI: addresses are 20 bytes left-padded with 12 zero
            // bytes inside a 32-byte slot, so the value lives in
            // `[slot+12 .. slot+32)`.
            instructions.push(Instruction::LoadLocal(buffer_local));
            instructions.push(Instruction::LoadLocal(offset_local));
            instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                BigInt::from(12u8),
            )));
            instructions.push(Instruction::BinaryOp(BinaryOperator::Add));
            instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                BigInt::from(20u8),
            )));
            instructions.push(Instruction::Substr);
            instructions.push(Instruction::Convert {
                target: ConvertTarget::ByteArray,
            });
            materialize_byte_array_buffer(ctx, instructions, true);
        }
        ValueType::ByteArray {
            fixed_len: Some(len),
        } => {
            let len = (*len).min(32) as usize;
            instructions.push(Instruction::LoadLocal(buffer_local));
            instructions.push(Instruction::LoadLocal(offset_local));
            instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                BigInt::from(len as u64),
            )));
            instructions.push(Instruction::Substr);
            instructions.push(Instruction::Convert {
                target: ConvertTarget::ByteArray,
            });
        }
        _ => {
            instructions.push(Instruction::LoadLocal(buffer_local));
            instructions.push(Instruction::LoadLocal(offset_local));
            instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                BigInt::from(32u8),
            )));
            instructions.push(Instruction::Substr);
            instructions.push(Instruction::Convert {
                target: ConvertTarget::ByteArray,
            });
        }
    }
}

pub(crate) fn abi_decode_value_types(
    types_expr: &Expression,
    ctx: &LoweringContext,
) -> Option<Vec<ValueType>> {
    match types_expr {
        Expression::List(_, params) => params
            .iter()
            .map(|(_, param)| {
                param
                    .as_ref()
                    .and_then(|parameter| infer_type_from_expression(&parameter.ty, ctx))
            })
            .collect(),
        Expression::Parenthesis(_, inner) => {
            infer_type_from_expression(inner, ctx).map(|ty| vec![ty])
        }
        other => infer_type_from_expression(other, ctx).map(|ty| vec![ty]),
    }
}

pub(crate) fn emit_abi_decode_static_slot(
    buffer_local: usize,
    index: usize,
    value_type: &ValueType,
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) {
    match value_type {
        // Unsigned integers must NOT be read with NeoVM's signed-LE
        // CONVERT semantics — see `emit_le_buffer_to_unsigned_integer`.
        ValueType::Integer { signed: false, .. } => {
            emit_abi_decode_slot_slice(buffer_local, index, 0, 32, instructions);
            instructions.push(Instruction::Convert {
                target: ConvertTarget::ByteArray,
            });
            materialize_byte_array_buffer(ctx, instructions, true);
            emit_le_buffer_to_unsigned_integer(ctx, instructions);
        }
        // Signed intN: the canonical slot is sign-extended big-endian, so
        // the signed-LE CONVERT after the reversal is exactly right.
        ValueType::Integer { signed: true, .. } | ValueType::Boolean => {
            emit_abi_decode_slot_slice(buffer_local, index, 0, 32, instructions);
            instructions.push(Instruction::Convert {
                target: ConvertTarget::ByteArray,
            });
            materialize_byte_array_buffer(ctx, instructions, true);
            instructions.push(Instruction::Convert {
                target: ConvertTarget::Integer,
            });
        }
        // All-static struct: decode each field from consecutive head slots
        // starting at `index` into a fresh `StackItem::Array` (the
        // Array-of-fields shape the struct-constructor lowering produces
        // and the encode side reads back via ArrayGet). Mirrors
        // `lower_static_abi_slots_for_expr` on the encode side.
        ValueType::Struct { fields, .. } => {
            let tmp_id = ctx.next_label();
            let struct_local = ctx.allocate_local(format!("__abi_dec_struct_{tmp_id}"), None);
            instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                BigInt::from(fields.len() as u64),
            )));
            instructions.push(Instruction::NewArray {
                element_type: ValueType::Any,
            });
            instructions.push(Instruction::StoreLocal(struct_local));
            let mut field_slot = index;
            for (field_index, field) in fields.iter().enumerate() {
                instructions.push(Instruction::LoadLocal(struct_local));
                instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                    BigInt::from(field_index as u64),
                )));
                emit_abi_decode_static_slot(buffer_local, field_slot, &field.ty, ctx, instructions);
                instructions.push(Instruction::ArraySet);
                field_slot += abi_static_slot_count(&field.ty).unwrap_or(1);
            }
            instructions.push(Instruction::LoadLocal(struct_local));
        }
        ValueType::Address => {
            emit_abi_decode_slot_slice(buffer_local, index, 12, 20, instructions);
            instructions.push(Instruction::Convert {
                target: ConvertTarget::ByteArray,
            });
            materialize_byte_array_buffer(ctx, instructions, true);
        }
        ValueType::ByteArray {
            fixed_len: Some(len),
        } => {
            let len = (*len).min(32) as usize;
            emit_abi_decode_slot_slice(buffer_local, index, 0, len, instructions);
            instructions.push(Instruction::Convert {
                target: ConvertTarget::ByteArray,
            });
        }
        _ => {
            emit_abi_decode_slot_slice(buffer_local, index, 0, 32, instructions);
            instructions.push(Instruction::Convert {
                target: ConvertTarget::ByteArray,
            });
        }
    }
}
