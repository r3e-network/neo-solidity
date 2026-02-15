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
        // Compatibility fallback: raw EVM calldata bytes are not first-class on Neo N3.
        // Evaluate arguments for side effects, then materialize empty bytes.
        let mut success = true;
        for arg in args {
            if !lower_expression(arg, ctx, instructions) {
                success = false;
            } else {
                instructions.push(Instruction::Drop(ValueType::Any));
            }
        }
        instructions.push(Instruction::PushLiteral(LiteralValue::ByteArray(Vec::new())));
        return Some(success);
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
