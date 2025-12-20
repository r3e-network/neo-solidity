fn emit_load_struct_array_element(
    bytecode: &mut Vec<u8>,
    module: &ir::Module,
    state_index: usize,
    key_types: &[ValueType],
    access: StructArrayElementAccess<'_>,
    use_callt: bool,
    token_patches: &mut Vec<MethodTokenPatch>,
) {
    // Stack before: [index, keyN, ..., key0]
    // Compute the base slot for the array length (struct field slot), then derive the element slot:
    // element_slot = keccak256(serialize(index) || field_slot)
    emit_struct_field_slot(
        bytecode,
        module,
        state_index,
        key_types,
        access.field_keys,
        use_callt,
        token_patches,
    );

    // Stack: [index, field_slot]
    bytecode.push(0x50); // SWAP -> [field_slot, index]
    emit_serialize_key(
        bytecode,
        &ValueType::Integer {
            signed: false,
            bits: 256,
        },
        use_callt,
        token_patches,
    ); // -> [field_slot, index_bytes]
    bytecode.push(0x50); // SWAP -> [index_bytes, field_slot]
    bytecode.push(0x8B); // CAT
    emit_native_contract_call(
        bytecode,
        ir::NativeContract::CryptoLib,
        "keccak256",
        1,
        use_callt,
        token_patches,
    );

    match access.element_type {
        ValueType::Struct { fields, .. } => {
            // Construct `[field0, field1, ...]` by loading each field from its derived slot.
            push_integer_bigint(bytecode, &BigInt::from(fields.len() as u64));
            bytecode.push(0xC3); // NEWARRAY

            // Stack: [element_slot, out_array]
            for (field_index, field) in fields.iter().enumerate() {
                // Derive the field slot from the element base slot:
                // element_field_slot = keccak256(field_key || element_slot)
                bytecode.push(0x4B); // OVER (duplicate element_slot)
                push_data(bytecode, &field.key);
                bytecode.push(0x50); // SWAP -> [element_slot, out_array, field_key, element_slot]
                bytecode.push(0x8B); // CAT
                emit_native_contract_call(
                    bytecode,
                    ir::NativeContract::CryptoLib,
                    "keccak256",
                    1,
                    use_callt,
                    token_patches,
                );

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
                bytecode.push(0x50); // SWAP -> [element_slot, out_array, out_array, field_value]
                push_integer_bigint(bytecode, &BigInt::from(field_index as u64));
                bytecode.push(0x50); // SWAP -> [element_slot, out_array, out_array, idx, value]
                bytecode.push(0xD0); // SETITEM
            }

            // Drop element_slot, leaving the struct array.
            bytecode.push(0x50); // SWAP -> [out_array, element_slot]
            bytecode.push(0x45); // DROP
        }
        _ => {
            emit_syscall(bytecode, "System.Storage.GetContext");
            emit_syscall(bytecode, "System.Storage.Get");
            emit_coerce_storage_value(bytecode, access.element_type);
        }
    }
}

fn emit_store_struct_array_element(
    bytecode: &mut Vec<u8>,
    module: &ir::Module,
    state_index: usize,
    key_types: &[ValueType],
    access: StructArrayElementAccess<'_>,
    use_callt: bool,
    token_patches: &mut Vec<MethodTokenPatch>,
) {
    // Stack before: [value, index, keyN, ..., key0]
    emit_struct_field_slot(
        bytecode,
        module,
        state_index,
        key_types,
        access.field_keys,
        use_callt,
        token_patches,
    );

    // Stack: [value, index, field_slot]
    bytecode.push(0x50); // SWAP -> [value, field_slot, index]
    emit_serialize_key(
        bytecode,
        &ValueType::Integer {
            signed: false,
            bits: 256,
        },
        use_callt,
        token_patches,
    ); // -> [value, field_slot, index_bytes]
    bytecode.push(0x50); // SWAP -> [value, index_bytes, field_slot]
    bytecode.push(0x8B); // CAT
    emit_native_contract_call(
        bytecode,
        ir::NativeContract::CryptoLib,
        "keccak256",
        1,
        use_callt,
        token_patches,
    );

    match access.element_type {
        ValueType::Struct { fields, .. } => {
            for (field_index, field) in fields.iter().enumerate() {
                // Derive the field slot from the element base slot:
                // element_field_slot = keccak256(field_key || element_slot)
                bytecode.push(0x4A); // DUP (element_slot)
                push_data(bytecode, &field.key);
                bytecode.push(0x50); // SWAP -> [value, element_slot, field_key, element_slot]
                bytecode.push(0x8B); // CAT
                emit_native_contract_call(
                    bytecode,
                    ir::NativeContract::CryptoLib,
                    "keccak256",
                    1,
                    use_callt,
                    token_patches,
                );

                // Extract the field value from the struct array: value[field_index]
                bytecode.push(0x12); // PUSH2 (duplicate struct value from depth 2)
                bytecode.push(0x4D); // PICK -> [..., field_slot, value]
                push_integer_bigint(bytecode, &BigInt::from(field_index as u64));
                bytecode.push(0xCE); // PICKITEM -> [..., field_slot, field_value]

                match &field.ty {
                    ValueType::Struct { fields, .. } => {
                        emit_store_struct_value_to_slot(
                            bytecode,
                            fields,
                            use_callt,
                            token_patches,
                        );
                    }
                    _ => {
                        // Store `field_value` into `field_slot`.
                        bytecode.push(0x50); // SWAP -> [..., field_value, field_slot]
                        emit_syscall(bytecode, "System.Storage.GetContext");
                        emit_syscall(bytecode, "System.Storage.Put");
                    }
                }
            }

            // Drop `[value, element_slot]`.
            bytecode.push(0x45); // DROP (element_slot)
            bytecode.push(0x45); // DROP (value)
        }
        _ => {
            emit_syscall(bytecode, "System.Storage.GetContext");
            emit_syscall(bytecode, "System.Storage.Put");
        }
    }
}
