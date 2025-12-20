fn emit_load_struct_value_from_slot(
    bytecode: &mut Vec<u8>,
    fields: &[ir::StructField],
    use_callt: bool,
    token_patches: &mut Vec<MethodTokenPatch>,
) {
    // Stack before: [base_slot]
    // Stack after:  [struct_array]
    push_integer_bigint(bytecode, &BigInt::from(fields.len() as u64));
    bytecode.push(0xC3); // NEWARRAY

    // Stack: [base_slot, out_array]
    for (field_index, field) in fields.iter().enumerate() {
        // Derive the field slot from the base slot:
        // field_slot = keccak256(field_key || base_slot)
        bytecode.push(0x4B); // OVER (duplicate base_slot)
        push_data(bytecode, &field.key);
        bytecode.push(0x50); // SWAP -> [base_slot, out_array, field_key, base_slot]
        bytecode.push(0x8B); // CAT
        emit_native_contract_call(
            bytecode,
            ir::NativeContract::CryptoLib,
            "keccak256",
            1,
            use_callt,
            token_patches,
        );

        // Stack: [base_slot, out_array, field_slot]
        match &field.ty {
            ValueType::Struct { fields, .. } => {
                emit_load_struct_value_from_slot(bytecode, fields, use_callt, token_patches);
            }
            _ => {
                emit_syscall(bytecode, "System.Storage.GetContext");
                emit_syscall(bytecode, "System.Storage.Get");
                emit_coerce_storage_value(bytecode, &field.ty);
            }
        }

        // Set `out_array[field_index] = field_value`, preserving the array reference.
        bytecode.push(0x4B); // OVER (duplicate out_array)
        bytecode.push(0x50); // SWAP -> [base_slot, out_array, out_array, field_value]
        push_integer_bigint(bytecode, &BigInt::from(field_index as u64));
        bytecode.push(0x50); // SWAP -> [base_slot, out_array, out_array, idx, value]
        bytecode.push(0xD0); // SETITEM
    }

    // Drop base_slot, leaving the struct array.
    bytecode.push(0x50); // SWAP -> [out_array, base_slot]
    bytecode.push(0x45); // DROP
}

fn emit_store_struct_value_to_slot(
    bytecode: &mut Vec<u8>,
    fields: &[ir::StructField],
    use_callt: bool,
    token_patches: &mut Vec<MethodTokenPatch>,
) {
    // Stack before: [base_slot, struct_array]
    // Stack after:  []
    for (field_index, field) in fields.iter().enumerate() {
        // Derive field_slot = keccak256(field_key || base_slot)
        bytecode.push(0x4B); // OVER (duplicate base_slot)
        push_data(bytecode, &field.key);
        bytecode.push(0x50); // SWAP
        bytecode.push(0x8B); // CAT
        emit_native_contract_call(
            bytecode,
            ir::NativeContract::CryptoLib,
            "keccak256",
            1,
            use_callt,
            token_patches,
        );

        // Stack: [base_slot, struct_array, field_slot]
        // Extract field_value = struct_array[field_index]
        bytecode.push(0x4B); // OVER (duplicate struct_array)
        push_integer_bigint(bytecode, &BigInt::from(field_index as u64));
        bytecode.push(0xCE); // PICKITEM -> [base_slot, struct_array, field_slot, field_value]

        match &field.ty {
            ValueType::Struct { fields, .. } => {
                emit_store_struct_value_to_slot(bytecode, fields, use_callt, token_patches);
            }
            _ => {
                // Store expects [value, key, context], so swap to put value on top of the slot.
                bytecode.push(0x50); // SWAP -> [base_slot, struct_array, field_value, field_slot]
                emit_syscall(bytecode, "System.Storage.GetContext");
                emit_syscall(bytecode, "System.Storage.Put");
            }
        }
    }

    // Drop `[base_slot, struct_array]`.
    bytecode.push(0x45); // DROP (struct_array)
    bytecode.push(0x45); // DROP (base_slot)
}
