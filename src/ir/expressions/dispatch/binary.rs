fn fixed_len_bytes_be_from_hex_number(expr: &Expression, fixed_len: u16) -> Option<Vec<u8>> {
    let Expression::HexNumberLiteral(_, value, unit) = expr else {
        return None;
    };

    if unit.is_some() {
        return None;
    }

    let raw = value.trim().trim_start_matches("0x");
    let mut hex: String = raw
        .chars()
        .filter(|c| !c.is_whitespace() && *c != '_')
        .collect();
    if hex.is_empty() {
        return None;
    }

    if hex.len() % 2 == 1 {
        hex.insert(0, '0');
    }

    let bytes = hex_decode(&hex).ok()?;
    let fixed_len = fixed_len as usize;
    if bytes.len() > fixed_len {
        return None;
    }

    let mut out = vec![0u8; fixed_len - bytes.len()];
    out.extend_from_slice(&bytes);
    Some(out)
}

fn lower_bytes_eq_hex_number_literal(
    left: &Expression,
    right: &Expression,
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
    operator: BinaryOperator,
) -> Option<bool> {
    if !matches!(operator, BinaryOperator::Eq | BinaryOperator::Ne) {
        return None;
    }

    // Preserve Solidity left-to-right evaluation order while ensuring `bytesN` comparisons work
    // with hex-number literals like `0x01ffc9a7` (commonly used for ERC165 interface IDs).
    if let Some(ValueType::ByteArray {
        fixed_len: Some(fixed_len),
    }) = infer_type_from_expression(left, ctx)
    {
        let literal_expr = match right {
            Expression::Parenthesis(_, inner) => inner.as_ref(),
            other => other,
        };

        if let Some(bytes) = fixed_len_bytes_be_from_hex_number(literal_expr, fixed_len) {
            if lower_expression(left, ctx, instructions) {
                instructions.push(Instruction::PushLiteral(LiteralValue::ByteArray(bytes)));
                instructions.push(Instruction::BinaryOp(operator));
                return Some(true);
            }
            return Some(false);
        }
    }

    if let Some(ValueType::ByteArray {
        fixed_len: Some(fixed_len),
    }) = infer_type_from_expression(right, ctx)
    {
        let literal_expr = match left {
            Expression::Parenthesis(_, inner) => inner.as_ref(),
            other => other,
        };

        if let Some(bytes) = fixed_len_bytes_be_from_hex_number(literal_expr, fixed_len) {
            instructions.push(Instruction::PushLiteral(LiteralValue::ByteArray(bytes)));
            if lower_expression(right, ctx, instructions) {
                instructions.push(Instruction::BinaryOp(operator));
                return Some(true);
            }
            return Some(false);
        }
    }

    None
}

fn lower_binary_expr(
    left: &Expression,
    right: &Expression,
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
    operator: BinaryOperator,
) -> bool {
    if let Some(result) =
        lower_bytes_eq_hex_number_literal(left, right, ctx, instructions, operator)
    {
        return result;
    }

    if !lower_expression(left, ctx, instructions) || !lower_expression(right, ctx, instructions) {
        return false;
    }

    // Solidity 0.8+ panics on division/modulo by zero.
    if matches!(operator, BinaryOperator::Div | BinaryOperator::Mod) {
        let tmp_id = ctx.next_label();
        let rhs_local = ctx.allocate_local(format!("__div_rhs_{tmp_id}"), None);

        // Preserve left operand on the stack while we validate RHS.
        instructions.push(Instruction::StoreLocal(rhs_local)); // pops RHS

        // if rhs == 0 -> THROW("Panic: 0x12")
        let ok_label = ctx.next_label();
        instructions.push(Instruction::LoadLocal(rhs_local));
        instructions.push(Instruction::PushLiteral(LiteralValue::Integer(BigInt::zero())));
        instructions.push(Instruction::BinaryOp(BinaryOperator::Eq));
        instructions.push(Instruction::JumpIf { target: ok_label });
        instructions.push(Instruction::PushLiteral(LiteralValue::String(
            b"Panic: 0x12".to_vec(),
        )));
        instructions.push(Instruction::Throw);
        instructions.push(Instruction::Label(ok_label));

        // Reload RHS and perform the operation.
        instructions.push(Instruction::LoadLocal(rhs_local));
    }

    instructions.push(Instruction::BinaryOp(operator));
    true
}
