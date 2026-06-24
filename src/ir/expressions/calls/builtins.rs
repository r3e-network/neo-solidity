include!("builtins/abi_predicates.rs");
include!("builtins/abi_buffers.rs");
include!("builtins/abi_encode.rs");
include!("builtins/abi_decode.rs");
include!("builtins/abi_structs_notify.rs");
include!("builtins/native_serialize.rs");
include!("builtins/member_access.rs");
include!("builtins/member_runtime.rs");
include!("builtins/member_syscalls.rs");
include!("builtins/member_storage.rs");
include!("builtins/member_neo.rs");
include!("builtins/member_nativecalls.rs");
include!("builtins/resolved.rs");

fn try_lower_builtin_call(
    func: &Expression,
    args: &[Expression],
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) -> Option<bool> {
    // Solidity 0.8.x type-level concat: `bytes.concat(a, b, ...)` / `string.concat(a, b, ...)`
    // solang-parser represents these as MemberAccess(Type(DynamicBytes|String), "concat").
    if let Some(result) = try_lower_type_concat(func, args, ctx, instructions) {
        return Some(result);
    }

    if let Some(result) = try_lower_member_builtin(func, args, ctx, instructions) {
        return Some(result);
    }

    try_lower_resolved_builtin_call(func, args, ctx, instructions)
}

/// Handle `bytes.concat(a, b, ...)` and `string.concat(a, b, ...)`.
///
/// These are Solidity 0.8.x type-level functions. Each argument is lowered onto
/// the stack, then chained with NeoVM CAT opcodes. Zero arguments produce an
/// empty byte string; one argument is a pass-through.
fn try_lower_type_concat(
    func: &Expression,
    args: &[Expression],
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) -> Option<bool> {
    let Expression::MemberAccess(_, inner, member) = func else {
        return None;
    };
    if member.name != "concat" {
        return None;
    }
    // Match `bytes.concat(...)` or `string.concat(...)` where inner is a Type node.
    let is_bytes_or_string = matches!(
        inner.as_ref(),
        Expression::Type(_, PtType::DynamicBytes | PtType::String)
    );
    if !is_bytes_or_string {
        return None;
    }

    // Zero args: push empty byte array.
    if args.is_empty() {
        instructions.push(Instruction::PushLiteral(LiteralValue::ByteArray(vec![])));
        return Some(true);
    }

    // Lower all arguments, then emit BytesConcat builtin (CAT chain).
    //
    // Task #76 — `bytesN(..)` / `address(..)` casts route through
    // `coerce_to_fixed_bytes`, which leaves the MEMCPY-returned destination
    // buffer on the stack BENEATH the canonical ByteString (MEMCPY pushes
    // `dst` back per C-style memcpy semantics — see
    // src/runtime/execution/execution_impl_part3_bytes.rs::memcpy_bytes).
    // Chaining CAT across `N` such args would splice the leaked buffer of
    // arg `k+1` between `canonical_k` and `canonical_{k+1}`, producing
    // wrong content (e.g. `bytes.concat(bytes32(1), bytes32(2))` yields
    // `bytes32(2) || bytes32(2)` instead of `bytes32(1) || bytes32(2)`).
    // Mirror the Task #66 packed-encoding fix in resolved.rs: follow each
    // leaky arg with `Swap; Drop` to discard the leaked buffer.
    let mut success = true;
    for arg in args {
        if !lower_expression(arg, ctx, instructions) {
            success = false;
            continue;
        }
        // No Swap; Drop needed: real NeoVM MEMCPY pushes nothing.
    }

    if success {
        instructions.push(Instruction::CallBuiltin {
            builtin: BuiltinCall::BytesConcat,
            arg_count: args.len(),
        });
        // On real NeoVM, CAT yields a Buffer (0x30) while ABI parameters,
        // hex literals, keccak256 results, and storage reads are ByteStrings
        // (0x28). StdLib.serialize embeds the stack-item type byte, so an
        // un-canonicalized concat result used as a mapping key would derive
        // a different storage slot than the same bytes presented as a
        // ByteString. Coerce to ByteString so equality / mapping-key /
        // ABI-return layers all see a canonical contiguous bytes value
        // (mirrors the SUBSTR slice coercion in
        // `src/ir/expressions/arrays.rs`).
        instructions.push(Instruction::Convert {
            target: ConvertTarget::ByteArray,
        });
    }

    Some(success)
}
