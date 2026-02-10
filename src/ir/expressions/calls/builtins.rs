include!("builtins/helpers.rs");
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
    let mut success = true;
    for arg in args {
        if !lower_expression(arg, ctx, instructions) {
            success = false;
        }
    }

    if success {
        instructions.push(Instruction::CallBuiltin {
            builtin: BuiltinCall::BytesConcat,
            arg_count: args.len(),
        });
    }

    Some(success)
}
