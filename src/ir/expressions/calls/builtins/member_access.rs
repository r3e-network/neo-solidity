fn try_lower_member_builtin(
    func: &Expression,
    args: &[Expression],
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) -> Option<bool> {
    let Expression::MemberAccess(_, inner, member) = func else {
        return None;
    };
    let Expression::Variable(base) = inner.as_ref() else {
        return None;
    };

    if base.name == "abi"
        && matches!(
            member.name.as_str(),
            "encodeWithSignature" | "encodeWithSelector"
        )
    {
        // Neo N3 contracts are invoked by method name + args (manifest ABI), not by raw
        // EVM calldata. We only support these helpers when they can be rewritten into a
        // Neo `System.Contract.Call` (either inlined into `address.call/staticcall(...)`,
        // or stored in a local `bytes` variable that is later passed to those calls).
        ctx.record_error(format!(
            "abi.{} is only supported for Neo contract calls (inline it into `address.call(...)` / `address.staticcall(...)`, or assign it to a local `bytes` variable that is later passed to those calls). Raw EVM calldata bytes are not supported on Neo N3; use `Syscalls.contractCall` / `Syscalls.contractCallWithFlags` / `NativeCalls.*` helpers when possible",
            member.name
        ));
        return Some(false);
    }

    if let Some(result) = try_lower_runtime_member_builtin(base, member, args, ctx, instructions) {
        return Some(result);
    }

    if let Some(result) = try_lower_syscalls_member_builtin(base, member, args, ctx, instructions)
    {
        return Some(result);
    }

    if let Some(result) = try_lower_storage_member_builtin(base, member, args, ctx, instructions) {
        return Some(result);
    }

    if let Some(result) = try_lower_neo_member_builtin(base, member, args, ctx, instructions) {
        return Some(result);
    }

    if let Some(result) =
        try_lower_nativecalls_member_builtin(base, member, args, ctx, instructions)
    {
        return Some(result);
    }

    None
}
