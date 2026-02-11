fn try_lower_selector_member_access(
    inner: &Expression,
    member: &Identifier,
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) -> Option<bool> {
    fn resolve_contract_type_name(expr: &Expression, ctx: &LoweringContext) -> Option<String> {
        match expr {
            Expression::Variable(type_name) if ctx.is_contract_type_name(&type_name.name) => {
                Some(type_name.name.clone())
            }
            Expression::MemberAccess(_, namespace_expr, type_name)
                if matches!(
                    namespace_expr.as_ref(),
                    Expression::Variable(namespace_id)
                        if !ctx.param_index_map.contains_key(&namespace_id.name)
                            && ctx.resolve_local(&namespace_id.name).is_none()
                            && !ctx.state_index_map.contains_key(&namespace_id.name)
                            && !ctx.is_contract_type_name(&namespace_id.name)
                ) && ctx.is_contract_type_name(&type_name.name) =>
            {
                Some(type_name.name.clone())
            }
            _ => None,
        }
    }

    if member.name != "selector" {
        return None;
    }

    // Solidity exposes function selectors via `TypeName.method.selector` (bytes4).
    // This is commonly used for interface receivers and low-level call encoding.
    //
    // The AST shape is: `MemberAccess(MemberAccess(Variable(TypeName), method), selector)`.
    if let Expression::MemberAccess(_, target_inner, target_method) = inner {
        if let Some(type_name) = resolve_contract_type_name(target_inner.as_ref(), ctx) {
            if let Some(selectors) = ctx.type_method_selectors(&type_name, &target_method.name) {
                if selectors.len() == 1 {
                    instructions.push(Instruction::PushLiteral(
                        LiteralValue::ByteArray(selectors[0].to_vec()),
                    ));
                    return Some(true);
                }

                ctx.record_error(format!(
                    "ambiguous selector '{}.{}': {} overload(s)",
                    type_name,
                    target_method.name,
                    selectors.len()
                ));
                return Some(false);
            }

            ctx.record_error(format!(
                "unknown selector '{}.{}'",
                type_name, target_method.name
            ));
            return Some(false);
        }
    }

    ctx.record_error("unsupported selector expression");
    Some(false)
}
