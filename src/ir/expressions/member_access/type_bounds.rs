fn try_lower_type_bound_max(
    inner: &Expression,
    member: &Identifier,
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) -> Option<bool> {
    if member.name != "max" {
        return None;
    }

    // Solidity uses `type(uint256).max` (and `type(int256).max`) to query numeric
    // bounds. solang-parser represents `type(T)` as a function call to the `type`
    // keyword.
    if let Some(type_arg) = typeof_argument(inner) {
        match type_arg {
            Expression::Type(_, PtType::Uint(bits)) => {
                let mut value = BigInt::one();
                value <<= *bits as usize;
                value -= BigInt::one();
                instructions.push(Instruction::PushLiteral(LiteralValue::Integer(value)));
                return Some(true);
            }
            Expression::Type(_, PtType::Int(bits)) => {
                if *bits == 0 {
                    ctx.record_error("type(int0).max is invalid");
                    return Some(false);
                }

                let mut value = BigInt::one();
                value <<= (*bits as usize).saturating_sub(1);
                value -= BigInt::one();
                instructions.push(Instruction::PushLiteral(LiteralValue::Integer(value)));
                return Some(true);
            }
            _ => {
                ctx.record_error("unsupported type(...).max expression");
                return Some(false);
            }
        }
    }

    // Fallback: tolerate `uint256.max`-style expressions if they appear in the AST.
    if let Expression::Type(_, PtType::Uint(bits)) = inner {
        let mut value = BigInt::one();
        value <<= *bits as usize;
        value -= BigInt::one();
        instructions.push(Instruction::PushLiteral(LiteralValue::Integer(value)));
        return Some(true);
    }

    if let Expression::Type(_, PtType::Int(bits)) = inner {
        if *bits == 0 {
            ctx.record_error("type(int0).max is invalid");
            return Some(false);
        }

        let mut value = BigInt::one();
        value <<= (*bits as usize).saturating_sub(1);
        value -= BigInt::one();
        instructions.push(Instruction::PushLiteral(LiteralValue::Integer(value)));
        return Some(true);
    }

    None
}

fn try_lower_type_bound_min(
    inner: &Expression,
    member: &Identifier,
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) -> Option<bool> {
    if member.name != "min" {
        return None;
    }

    if let Some(type_arg) = typeof_argument(inner) {
        match type_arg {
            Expression::Type(_, PtType::Uint(_)) => {
                instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                    BigInt::zero(),
                )));
                return Some(true);
            }
            Expression::Type(_, PtType::Int(bits)) => {
                if *bits == 0 {
                    ctx.record_error("type(int0).min is invalid");
                    return Some(false);
                }

                let mut value = BigInt::one();
                value <<= (*bits as usize).saturating_sub(1);
                value = -value;
                instructions.push(Instruction::PushLiteral(LiteralValue::Integer(value)));
                return Some(true);
            }
            _ => {
                ctx.record_error("unsupported type(...).min expression");
                return Some(false);
            }
        }
    }

    if let Expression::Type(_, PtType::Uint(_)) = inner {
        instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
            BigInt::zero(),
        )));
        return Some(true);
    }

    if let Expression::Type(_, PtType::Int(bits)) = inner {
        if *bits == 0 {
            ctx.record_error("type(int0).min is invalid");
            return Some(false);
        }

        let mut value = BigInt::one();
        value <<= (*bits as usize).saturating_sub(1);
        value = -value;
        instructions.push(Instruction::PushLiteral(LiteralValue::Integer(value)));
        return Some(true);
    }

    None
}

fn try_lower_type_name(
    inner: &Expression,
    member: &Identifier,
    _ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) -> Option<bool> {
    if member.name != "name" {
        return None;
    }

    let type_arg = typeof_argument(inner)?;

    let name_str = match type_arg {
        Expression::Variable(id) => id.name.clone(),
        Expression::Type(_, pt_type) => format!("{pt_type}"),
        _ => return None,
    };

    instructions.push(Instruction::PushLiteral(LiteralValue::String(
        name_str.into_bytes(),
    )));
    Some(true)
}

fn try_lower_interface_id(
    inner: &Expression,
    member: &Identifier,
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) -> Option<bool> {
    if member.name != "interfaceId" {
        return None;
    }

    if let Some(type_arg) = typeof_argument(inner) {
        if let Expression::Variable(type_name) = type_arg {
            if !ctx.is_interface_type_name(&type_name.name) {
                ctx.record_error_with_suggestion(
                    format!(
                        "type({}).interfaceId is only supported for interface types",
                        type_name.name
                    ),
                    "interfaceId can only be computed for interface definitions, not contracts or other types",
                );
                return Some(false);
            }

            if let Some(interface_id) = ctx.interface_id_for_type(&type_name.name) {
                instructions.push(Instruction::PushLiteral(LiteralValue::ByteArray(
                    interface_id.to_vec(),
                )));
                return Some(true);
            }

            ctx.record_error_with_suggestion(
                format!(
                    "unable to compute interfaceId for '{}'",
                    type_name.name
                ),
                "ensure the interface has at least one function declaration",
            );
            return Some(false);
        }

        ctx.record_error("unsupported type(...).interfaceId expression");
        return Some(false);
    }

    ctx.record_error("interfaceId is only supported as `type(Interface).interfaceId`");
    Some(false)
}
