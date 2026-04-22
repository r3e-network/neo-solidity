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
                    ValueType::Array(_) => {
                        // Task #182 — a dynamic-array field inside a struct-array
                        // element must be deep-copied so that subsequent reads
                        // align with the storage layout expected by the read
                        // path:
                        //   * `buckets[idx].items.length` → `LoadStructField`
                        //     with `field_type = Array(_)`, which reads
                        //     `field_slot` and coerces the ByteString to an
                        //     Integer length (see
                        //     `emit_coerce_storage_value` `ValueType::Array`).
                        //   * `buckets[idx].items[i]` →
                        //     `LoadStructFieldMappingElement` computing
                        //     `keccak256(serialize(i) || field_slot)`.
                        //
                        // Stack on entry: [value, element_slot, field_slot, field_value].
                        // Leaving the Array stack item as a blob at
                        // `field_slot` surfaces the serde_json Array shape on
                        // the `.length` read (see previous diagnosis).
                        // Instead, iterate the Array, writing each element at
                        // its derived per-element slot, then write the length
                        // at `field_slot`.
                        emit_store_array_field_deep_copy(
                            bytecode,
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

/// Task #182 — deep-copy a dynamic-array field into per-element storage slots
/// and write the array length at the field base slot.
///
/// Stack on entry: `[..., slot, array]`
/// Stack on exit:  `[...]`
///
/// Mirrors the layout the matching read path derives:
///   * length is stored at `slot` (read via `LoadStructField` + the
///     `ValueType::Array` arm of `emit_coerce_storage_value`, which coerces
///     the stored ByteString to an Integer so the caller observes
///     `.length == N` rather than the raw serialized array).
///   * element `i` is stored at `keccak256(serialize(i) || slot)` (read via
///     `LoadStructFieldMappingElement`, which CATs `serialize(i)` onto the
///     field slot and hashes — see `emit_struct_field_mapping_slot`).
fn emit_store_array_field_deep_copy(
    bytecode: &mut Vec<u8>,
    use_callt: bool,
    token_patches: &mut Vec<MethodTokenPatch>,
) {
    // Stack: [..., slot, array]
    // Push length (SIZE on array duplicate).
    bytecode.push(0x4A); // DUP -> [..., slot, array, array]
    bytecode.push(0xCA); // SIZE -> [..., slot, array, length]

    // The loop walks `i = length` down to `1`, writing `array[i-1]` at the
    // derived per-element slot each iteration. Using a single descending
    // counter avoids juggling both `i` and `length` on the stack.
    //
    // Invariant at each loop header: stack = [..., slot, array, i] where
    // `i in [0..=length]` and elements `[i..length)` have already been
    // written.
    let loop_start_pos = bytecode.len();

    // if i == 0, fall through to the length-store path.
    bytecode.push(0x4A); // DUP -> [..., slot, array, i, i]
    bytecode.push(0x10); // PUSH0 -> [..., slot, array, i, i, 0]
    bytecode.push(0x97); // EQUAL -> [..., slot, array, i, (i==0)]
    let jmp_exit_pos = bytecode.len();
    bytecode.push(0x25); // JMPIF_L (if i==0, jump to length-store)
    let jmp_exit_operand = bytecode.len();
    bytecode.extend_from_slice(&[0, 0, 0, 0]);

    // Body: decrement and write `array[i-1]` at `keccak256(serialize(i-1) || slot)`.
    bytecode.push(0x9D); // DEC -> [..., slot, array, current]  (current = i-1)

    // Compute element_slot = keccak256(serialize(current) || slot).
    bytecode.push(0x4A); // DUP -> [..., slot, array, current, current]
    emit_serialize_key(
        bytecode,
        &ValueType::Integer {
            signed: false,
            bits: 256,
        },
        use_callt,
        token_patches,
    ); // -> [..., slot, array, current, current_bytes]
    // slot is at depth 3 (top=current_bytes, d1=current, d2=array, d3=slot).
    bytecode.push(0x13); // PUSH3
    bytecode.push(0x4D); // PICK -> [..., slot, array, current, current_bytes, slot]
    bytecode.push(0x8B); // CAT -> [..., slot, array, current, (current_bytes||slot)]
    emit_native_contract_call(
        bytecode,
        ir::NativeContract::CryptoLib,
        "keccak256",
        1,
        use_callt,
        token_patches,
    ); // -> [..., slot, array, current, element_slot]

    // Fetch `array[current]`.
    // After element_slot push: top=element_slot, d1=current, d2=array.
    bytecode.push(0x12); // PUSH2
    bytecode.push(0x4D); // PICK -> [..., slot, array, current, element_slot, array]
    // Now top=array, d1=element_slot, d2=current.
    bytecode.push(0x12); // PUSH2
    bytecode.push(0x4D); // PICK -> [..., slot, array, current, element_slot, array, current]
    bytecode.push(0xCE); // PICKITEM -> [..., slot, array, current, element_slot, array[current]]

    // Store: Put pops [value, key, context] (see src/runtime storage.put).
    bytecode.push(0x50); // SWAP -> [..., slot, array, current, array[current], element_slot]
    emit_syscall(bytecode, "System.Storage.GetContext");
    emit_syscall(bytecode, "System.Storage.Put");
    // Stack: [..., slot, array, current]

    // Jump back to loop_start.
    let jmp_back_pos = bytecode.len();
    bytecode.push(0x23); // JMP_L
    let jmp_back_operand = bytecode.len();
    bytecode.extend_from_slice(&[0, 0, 0, 0]);
    let rel_back = (loop_start_pos as i32)
        .checked_sub(jmp_back_pos as i32)
        .unwrap_or(0);
    bytecode[jmp_back_operand..jmp_back_operand + 4].copy_from_slice(&rel_back.to_le_bytes());

    // exit: i has reached 0, all elements have been copied.
    let exit_pos = bytecode.len();
    let rel_exit = (exit_pos as i32)
        .checked_sub(jmp_exit_pos as i32)
        .unwrap_or(0);
    bytecode[jmp_exit_operand..jmp_exit_operand + 4].copy_from_slice(&rel_exit.to_le_bytes());

    // Stack at exit: [..., slot, array, 0] (i == 0).
    bytecode.push(0x45); // DROP -> [..., slot, array]
    bytecode.push(0xCA); // SIZE -> [..., slot, length]
    // Store length as the Array "base" value so that the read path's
    // `emit_coerce_storage_value` ValueType::Array arm (which coerces to
    // Integer) observes the correct `.length`. Put expects
    // [value, key, context].
    bytecode.push(0x50); // SWAP -> [..., length, slot]
    emit_syscall(bytecode, "System.Storage.GetContext");
    emit_syscall(bytecode, "System.Storage.Put");
    // Stack: [...]
}
