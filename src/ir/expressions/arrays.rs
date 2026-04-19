fn lower_array_subscript_expression(
    expr: &Expression,
    array: &Expression,
    index: &Expression,
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) -> bool {
    if let Some(mapping) = resolve_mapping_access(expr, ctx) {
        let reference = mapping.to_storage_reference();
        emit_storage_load(&reference, ctx, instructions)
    } else if let Some(reference) = resolve_storage_reference(expr, ctx) {
        // Task #82: nested mapping-in-struct reads such as `slots[k].balances[a]`.
        emit_storage_load(&reference, ctx, instructions)
    } else if lower_expression(array, ctx, instructions) && lower_expression(index, ctx, instructions) {
        // Task #107 — emit an explicit bounds guard so array-index-OOB
        // reverts with the canonical EVM Panic(uint256) envelope
        //   keccak256("Panic(uint256)")[..4] || abi.encode(0x32)
        // (instead of the generic runtime "PICKITEM: index out of bounds"
        // fault which `catch Panic(uint code)` cannot bind). Skip when the
        // array expression is one of the dynamic-array surfaces whose native
        // lowering already covers the guard (string / bytes byte-indexing)
        // by only emitting when the inferred element type is non-bytes — the
        // SIZE instruction works uniformly on both Array and ByteString, so
        // the guard is safe in both cases, and we opt in for both.
        let tmp_id = ctx.next_label();
        let idx_local = ctx.allocate_local(
            format!("__aidx_{tmp_id}"),
            Some(ValueType::Integer { signed: false, bits: 256 }),
        );
        let arr_local = ctx.allocate_local(format!("__aarr_{tmp_id}"), None);

        // Stack is [array, index] — stash both into locals so we can bounds-
        // check without disturbing the eventual ArrayGet operands.
        instructions.push(Instruction::StoreLocal(idx_local));
        instructions.push(Instruction::StoreLocal(arr_local));

        // Guard 1: index < 0 → Panic(0x32).  Solidity `uint256` indices
        // cannot be negative, but the runtime's PICKITEM still handles
        // signed-int operands, so we emit the check for defensive parity
        // with Solidity's own lowering.
        let after_neg_label = ctx.next_label();
        instructions.push(Instruction::LoadLocal(idx_local));
        instructions.push(Instruction::PushLiteral(LiteralValue::Integer(BigInt::zero())));
        instructions.push(Instruction::BinaryOp(BinaryOperator::Lt));
        // JumpIf-on-false: skip the THROW when idx < 0 is false.
        instructions.push(Instruction::JumpIf { target: after_neg_label });
        emit_panic(0x32, instructions);
        instructions.push(Instruction::Label(after_neg_label));

        // Guard 2: index >= SIZE(array) → Panic(0x32).
        let ok_label = ctx.next_label();
        instructions.push(Instruction::LoadLocal(idx_local));
        instructions.push(Instruction::LoadLocal(arr_local));
        instructions.push(Instruction::GetSize);
        instructions.push(Instruction::BinaryOp(BinaryOperator::Ge));
        instructions.push(Instruction::JumpIf { target: ok_label });
        emit_panic(0x32, instructions);
        instructions.push(Instruction::Label(ok_label));

        // Re-push [array, index] for ArrayGet.
        instructions.push(Instruction::LoadLocal(arr_local));
        instructions.push(Instruction::LoadLocal(idx_local));
        instructions.push(Instruction::ArrayGet);
        true
    } else {
        false
    }
}

fn lower_array_slice_expression(
    array: &Expression,
    start: Option<&Expression>,
    end: Option<&Expression>,
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) -> bool {
    // Task #95: Solidity `bytes` / `bytes memory` / `bytes calldata` slicing must
    // produce a contiguous ByteString, not an Array-of-bytes. Route bytes-typed
    // slices to a SUBSTR-based path; keep the element-wise Array copy path for
    // `T[]` dynamic arrays (where Solidity semantics require a new Array value).
    if is_bytes_slice_target(array, ctx) {
        return lower_bytes_slice_expression(array, start, end, ctx, instructions);
    }

    let array_local = ctx.allocate_local("__slice_array".to_string(), None);
    if !lower_expression(array, ctx, instructions) {
        return false;
    }
    instructions.push(Instruction::StoreLocal(array_local));

    let start_local = ctx.allocate_local("__slice_start".to_string(), None);
    if let Some(start_expr) = start {
        if !lower_expression(start_expr, ctx, instructions) {
            return false;
        }
    } else {
        instructions.push(Instruction::PushLiteral(LiteralValue::Integer(BigInt::zero())));
    }
    instructions.push(Instruction::StoreLocal(start_local));

    let end_local = ctx.allocate_local("__slice_end".to_string(), None);
    if let Some(end_expr) = end {
        if !lower_expression(end_expr, ctx, instructions) {
            return false;
        }
    } else {
        instructions.push(Instruction::LoadLocal(array_local));
        instructions.push(Instruction::GetSize);
    }
    instructions.push(Instruction::StoreLocal(end_local));

    // Clamp start to >= 0
    let clamp_start_label = ctx.next_label();
    let clamp_start_done = ctx.next_label();
    instructions.push(Instruction::LoadLocal(start_local));
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(BigInt::zero())));
    instructions.push(Instruction::BinaryOp(BinaryOperator::Ge));
    instructions.push(Instruction::JumpIf {
        target: clamp_start_label,
    });
    instructions.push(Instruction::Jump {
        target: clamp_start_done,
    });
    instructions.push(Instruction::Label(clamp_start_label));
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(BigInt::zero())));
    instructions.push(Instruction::StoreLocal(start_local));
    instructions.push(Instruction::Label(clamp_start_done));

    // Clamp end to array length
    let size_local = ctx.allocate_local("__slice_size".to_string(), None);
    instructions.push(Instruction::LoadLocal(array_local));
    instructions.push(Instruction::GetSize);
    instructions.push(Instruction::StoreLocal(size_local));

    let clamp_end_label = ctx.next_label();
    let clamp_end_done = ctx.next_label();
    instructions.push(Instruction::LoadLocal(end_local));
    instructions.push(Instruction::LoadLocal(size_local));
    instructions.push(Instruction::BinaryOp(BinaryOperator::Le));
    instructions.push(Instruction::JumpIf {
        target: clamp_end_label,
    });
    instructions.push(Instruction::Jump {
        target: clamp_end_done,
    });
    instructions.push(Instruction::Label(clamp_end_label));
    instructions.push(Instruction::LoadLocal(size_local));
    instructions.push(Instruction::StoreLocal(end_local));
    instructions.push(Instruction::Label(clamp_end_done));

    let len_local = ctx.allocate_local("__slice_len".to_string(), None);
    instructions.push(Instruction::LoadLocal(end_local));
    instructions.push(Instruction::LoadLocal(start_local));
    instructions.push(Instruction::BinaryOp(BinaryOperator::Sub));
    instructions.push(Instruction::StoreLocal(len_local));

    let clamp_label = ctx.next_label();
    let clamp_done = ctx.next_label();
    instructions.push(Instruction::LoadLocal(len_local));
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(BigInt::zero())));
    instructions.push(Instruction::BinaryOp(BinaryOperator::Ge));
    instructions.push(Instruction::JumpIf { target: clamp_label });
    instructions.push(Instruction::Jump { target: clamp_done });
    instructions.push(Instruction::Label(clamp_label));
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(BigInt::zero())));
    instructions.push(Instruction::StoreLocal(len_local));
    instructions.push(Instruction::Label(clamp_done));

    let element_type = infer_array_element_type(array, ctx).unwrap_or(ValueType::Any);
    let slice_array_type = ValueType::Array(Box::new(element_type.clone()));
    let out_local = ctx.allocate_local("__slice_out".to_string(), Some(slice_array_type));
    instructions.push(Instruction::LoadLocal(len_local));
    instructions.push(Instruction::NewArray { element_type });
    instructions.push(Instruction::StoreLocal(out_local));

    let idx_local = ctx.allocate_local("__slice_index".to_string(), None);
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(BigInt::zero())));
    instructions.push(Instruction::StoreLocal(idx_local));

    let loop_label = ctx.next_label();
    let end_label = ctx.next_label();

    instructions.push(Instruction::Label(loop_label));
    instructions.push(Instruction::LoadLocal(idx_local));
    instructions.push(Instruction::LoadLocal(len_local));
    instructions.push(Instruction::BinaryOp(BinaryOperator::Lt));
    instructions.push(Instruction::JumpIf { target: end_label });

    instructions.push(Instruction::LoadLocal(out_local));
    instructions.push(Instruction::LoadLocal(idx_local));
    instructions.push(Instruction::LoadLocal(array_local));
    instructions.push(Instruction::LoadLocal(start_local));
    instructions.push(Instruction::LoadLocal(idx_local));
    instructions.push(Instruction::BinaryOp(BinaryOperator::Add));
    instructions.push(Instruction::ArrayGet);
    instructions.push(Instruction::ArraySet);

    instructions.push(Instruction::LoadLocal(idx_local));
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(BigInt::from(1u8))));
    instructions.push(Instruction::BinaryOp(BinaryOperator::Add));
    instructions.push(Instruction::StoreLocal(idx_local));

    instructions.push(Instruction::Jump { target: loop_label });
    instructions.push(Instruction::Label(end_label));
    instructions.push(Instruction::LoadLocal(out_local));
    true
}

/// Task #95: Returns true when the sliced expression's IR type is a
/// Solidity `bytes` (`ValueType::ByteArray`) — a contiguous byte-string —
/// rather than a `T[]` dynamic array (`ValueType::Array(_)`). Covers
/// `bytes memory`, `bytes calldata`, `bytes storage`, and byte-string
/// literals such as `hex"..."`.
fn is_bytes_slice_target(array: &Expression, ctx: &LoweringContext) -> bool {
    matches!(
        infer_type_from_expression(array, ctx),
        Some(ValueType::ByteArray { .. })
    )
}

/// Task #95: Lower `bytes b[start:end]` to a contiguous ByteString using
/// NeoVM SUBSTR (opcode 0x8C). SUBSTR stack order (bottom -> top):
/// `[bytes, index, count]` → `[bytes.sub(index, count)]`.
///
/// Both `start` and `end` are clamped to `[0, len(bytes)]` and `end` is
/// further clamped to `>= start` so out-of-range slices yield an empty
/// ByteString instead of trapping — matches Solidity's saturating
/// semantics on `bytes` views and avoids a NeoVM "SUBSTR: out of bounds"
/// fault for degenerate ranges.
fn lower_bytes_slice_expression(
    array: &Expression,
    start: Option<&Expression>,
    end: Option<&Expression>,
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) -> bool {
    // Evaluate the source bytes once and stash in a local — we need the
    // length both for end-clamping and for the final SUBSTR push.
    let bytes_local = ctx.allocate_local(
        "__bytes_slice_src".to_string(),
        Some(ValueType::ByteArray { fixed_len: None }),
    );
    if !lower_expression(array, ctx, instructions) {
        return false;
    }
    instructions.push(Instruction::StoreLocal(bytes_local));

    // len = SIZE(bytes)
    let size_local = ctx.allocate_local("__bytes_slice_size".to_string(), None);
    instructions.push(Instruction::LoadLocal(bytes_local));
    instructions.push(Instruction::GetSize);
    instructions.push(Instruction::StoreLocal(size_local));

    // Evaluate `start` (default 0) and clamp to [0, len].
    let start_local = ctx.allocate_local("__bytes_slice_start".to_string(), None);
    if let Some(start_expr) = start {
        if !lower_expression(start_expr, ctx, instructions) {
            return false;
        }
    } else {
        instructions.push(Instruction::PushLiteral(LiteralValue::Integer(BigInt::zero())));
    }
    instructions.push(Instruction::StoreLocal(start_local));

    // NOTE on branch shape: IR `JumpIf { target }` jumps when the cond on
    // the stack is FALSE (lowers to JMPIFNOT_L). So the pattern
    //     push <cond>; JumpIf clamp; Jump done; Label clamp; <fix-up>; Label done
    // runs <fix-up> iff <cond> is false.

    // start < 0 → start = 0
    let start_clamp_lo = ctx.next_label();
    let start_clamp_lo_done = ctx.next_label();
    instructions.push(Instruction::LoadLocal(start_local));
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(BigInt::zero())));
    instructions.push(Instruction::BinaryOp(BinaryOperator::Ge));
    instructions.push(Instruction::JumpIf { target: start_clamp_lo });
    instructions.push(Instruction::Jump { target: start_clamp_lo_done });
    instructions.push(Instruction::Label(start_clamp_lo));
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(BigInt::zero())));
    instructions.push(Instruction::StoreLocal(start_local));
    instructions.push(Instruction::Label(start_clamp_lo_done));

    // start > len → start = len
    let start_clamp_hi = ctx.next_label();
    let start_clamp_hi_done = ctx.next_label();
    instructions.push(Instruction::LoadLocal(start_local));
    instructions.push(Instruction::LoadLocal(size_local));
    instructions.push(Instruction::BinaryOp(BinaryOperator::Le));
    instructions.push(Instruction::JumpIf { target: start_clamp_hi });
    instructions.push(Instruction::Jump { target: start_clamp_hi_done });
    instructions.push(Instruction::Label(start_clamp_hi));
    instructions.push(Instruction::LoadLocal(size_local));
    instructions.push(Instruction::StoreLocal(start_local));
    instructions.push(Instruction::Label(start_clamp_hi_done));

    // Evaluate `end` (default = len) and clamp to [start, len].
    let end_local = ctx.allocate_local("__bytes_slice_end".to_string(), None);
    if let Some(end_expr) = end {
        if !lower_expression(end_expr, ctx, instructions) {
            return false;
        }
    } else {
        instructions.push(Instruction::LoadLocal(size_local));
    }
    instructions.push(Instruction::StoreLocal(end_local));

    // end > len → end = len
    let end_clamp_hi = ctx.next_label();
    let end_clamp_hi_done = ctx.next_label();
    instructions.push(Instruction::LoadLocal(end_local));
    instructions.push(Instruction::LoadLocal(size_local));
    instructions.push(Instruction::BinaryOp(BinaryOperator::Le));
    instructions.push(Instruction::JumpIf { target: end_clamp_hi });
    instructions.push(Instruction::Jump { target: end_clamp_hi_done });
    instructions.push(Instruction::Label(end_clamp_hi));
    instructions.push(Instruction::LoadLocal(size_local));
    instructions.push(Instruction::StoreLocal(end_local));
    instructions.push(Instruction::Label(end_clamp_hi_done));

    // end < start → end = start (produces a zero-length slice rather than
    // a negative count that would trap SUBSTR).
    let end_clamp_lo = ctx.next_label();
    let end_clamp_lo_done = ctx.next_label();
    instructions.push(Instruction::LoadLocal(end_local));
    instructions.push(Instruction::LoadLocal(start_local));
    instructions.push(Instruction::BinaryOp(BinaryOperator::Ge));
    instructions.push(Instruction::JumpIf { target: end_clamp_lo });
    instructions.push(Instruction::Jump { target: end_clamp_lo_done });
    instructions.push(Instruction::Label(end_clamp_lo));
    instructions.push(Instruction::LoadLocal(start_local));
    instructions.push(Instruction::StoreLocal(end_local));
    instructions.push(Instruction::Label(end_clamp_lo_done));

    // SUBSTR expects [bytes, index, count] on the stack.
    instructions.push(Instruction::LoadLocal(bytes_local));
    instructions.push(Instruction::LoadLocal(start_local));
    instructions.push(Instruction::LoadLocal(end_local));
    instructions.push(Instruction::LoadLocal(start_local));
    instructions.push(Instruction::BinaryOp(BinaryOperator::Sub));
    instructions.push(Instruction::Substr);
    // SUBSTR yields a Buffer on NeoVM; coerce to ByteString so equality /
    // ABI-return layers see a canonical contiguous bytes value.
    instructions.push(Instruction::Convert {
        target: ConvertTarget::ByteArray,
    });
    true
}

fn lower_array_literal_expression(
    elements: &[Expression],
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) -> bool {
    let element_type = infer_literal_array_element_type(elements);
    let array_local = ctx.allocate_local(
        "__array_literal".to_string(),
        Some(ValueType::Array(Box::new(element_type.clone()))),
    );
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(BigInt::from(
        elements.len(),
    ))));
    instructions.push(Instruction::NewArray { element_type });
    instructions.push(Instruction::StoreLocal(array_local));

    for (index, element) in elements.iter().enumerate() {
        instructions.push(Instruction::LoadLocal(array_local));
        instructions.push(Instruction::PushLiteral(LiteralValue::Integer(BigInt::from(
            index as u64,
        ))));
        if !lower_expression(element, ctx, instructions) {
            instructions.push(Instruction::PushLiteral(LiteralValue::Integer(BigInt::zero())));
        }
        instructions.push(Instruction::ArraySet);
    }

    instructions.push(Instruction::LoadLocal(array_local));
    true
}
