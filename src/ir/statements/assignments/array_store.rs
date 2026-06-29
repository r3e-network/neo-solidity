use super::*;

pub(crate) fn lower_array_store(
    target: &Expression,
    rhs: &Expression,
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) {
    if let Expression::ArraySubscript(_, array, Some(index)) = target {
        let checkpoint = instructions.len();
        // A `bytesN` element assigned from an integer-backed literal must be
        // stored as its canonical big-endian ByteArray (matching the
        // cast/param representation) — else N<32 faults / N==32 reverses when
        // the element is later ABI-encoded.
        let elem_ty = match infer_type_from_expression(array, ctx) {
            Some(ValueType::Array(e)) => Some(*e),
            _ => None,
        };
        if lower_expression(array, ctx, instructions)
            && lower_expression(index, ctx, instructions)
        {
            let coerced = elem_ty
                .as_ref()
                .is_some_and(|et| try_lower_bytesn_literal_canonical(rhs, et, ctx, instructions));
            if coerced || lower_expression(rhs, ctx, instructions) {
                // No Swap; Drop needed: real NeoVM MEMCPY pushes nothing.
                instructions.push(Instruction::ArraySet);
                return;
            }
        }
        instructions.truncate(checkpoint);
    }

    load_expression(rhs, ctx, instructions);
    instructions.push(Instruction::Drop(ValueType::Any));
}
