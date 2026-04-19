fn lower_array_store(
    target: &Expression,
    rhs: &Expression,
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) {
    if let Expression::ArraySubscript(_, array, Some(index)) = target {
        let checkpoint = instructions.len();
        if lower_expression(array, ctx, instructions)
            && lower_expression(index, ctx, instructions)
            && lower_expression(rhs, ctx, instructions)
        {
            // Task #139 — `b[i] = bytes1(uint8(v))` on a `bytes memory` buffer.
            // `coerce_to_fixed_bytes` (invoked by the `bytesN(..)` cast, see
            // `is_fixed_bytes_cast_expr`) leaves the MEMCPY-returned destination
            // buffer on the stack BENEATH the canonical ByteString result.
            // SETITEM pops three items [collection, key, value]; without this
            // cleanup it would see [buffer, i, leaked_dst, value] and try to
            // SETITEM into `i` (an Integer) — producing
            // `"SETITEM: unsupported target Integer(0)"` at runtime. Mirrors
            // the Swap;Drop pattern used by `lower_compound_rhs` (compound.rs)
            // and `lower_binary_expr` (binary.rs). See fuzz harness
            // `batch62_ll3_keccak256_over_1kb_buffer`.
            if is_fixed_bytes_cast_expr(rhs) {
                instructions.push(Instruction::Swap);
                instructions.push(Instruction::Drop(ValueType::Any));
            }
            instructions.push(Instruction::ArraySet);
            return;
        }
        instructions.truncate(checkpoint);
    }

    load_expression(rhs, ctx, instructions);
    instructions.push(Instruction::Drop(ValueType::Any));
}

