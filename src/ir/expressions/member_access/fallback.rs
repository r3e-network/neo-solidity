fn lower_generic_member_access(
    expr: &Expression,
    inner: &Expression,
    member: &Identifier,
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) -> bool {
    // Enum value reference: `EnumName.VARIANT`.
    if let Expression::Variable(base) = inner {
        if let Some(variants) = ctx.enum_variant_map.get(&base.name) {
            if let Some(value) = variants.get(&member.name) {
                instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                    BigInt::from(*value),
                )));
                return true;
            }
        }
    }

    if let Some(reference) = resolve_storage_reference(expr, ctx) {
        return emit_storage_load(&reference, ctx, instructions);
    }

    // Memory struct field access: `tmp.field`
    if let Some(ValueType::Struct { fields, .. }) = infer_type_from_expression(inner, ctx) {
        if let Some(field_index) = fields
            .iter()
            .position(|field| field.name == member.name)
        {
            if !lower_expression(inner, ctx, instructions) {
                return false;
            }
            instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                BigInt::from(field_index as u64),
            )));
            instructions.push(Instruction::ArrayGet);
            return true;
        }
    }

    load_expression(inner, ctx, instructions);
    instructions.push(Instruction::Drop(ValueType::Any));
    instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
        BigInt::zero(),
    )));
    true
}
