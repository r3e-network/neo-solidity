fn emit_serialize_key(
    bytecode: &mut Vec<u8>,
    _key_type: &ValueType,
    use_callt: bool,
    token_patches: &mut Vec<MethodTokenPatch>,
) {
    emit_native_contract_call(
        bytecode,
        ir::NativeContract::StdLib,
        "serialize",
        1,
        use_callt,
        token_patches,
    );
}

fn emit_mapping_slot(
    bytecode: &mut Vec<u8>,
    module: &ir::Module,
    state_index: usize,
    key_types: &[ValueType],
    use_callt: bool,
    token_patches: &mut Vec<MethodTokenPatch>,
) {
    let base_slot = module
        .state_variables
        .get(state_index)
        .map(|state| state.storage_key.clone())
        .unwrap_or_else(|| vec![0u8; 32]);

    push_data(bytecode, &base_slot);

    for key_type in key_types {
        bytecode.push(0x50); // swap slot <-> key
        emit_serialize_key(bytecode, key_type, use_callt, token_patches);
        bytecode.push(0x50); // swap key_bytes <-> slot
        bytecode.push(0x8B); // concatenate key and slot
        emit_native_contract_call(
            bytecode,
            ir::NativeContract::CryptoLib,
            "keccak256",
            1,
            use_callt,
            token_patches,
        );
    }
}

fn emit_struct_field_slot(
    bytecode: &mut Vec<u8>,
    module: &ir::Module,
    state_index: usize,
    key_types: &[ValueType],
    field_keys: &[[u8; 32]],
    use_callt: bool,
    token_patches: &mut Vec<MethodTokenPatch>,
) {
    emit_mapping_slot(
        bytecode,
        module,
        state_index,
        key_types,
        use_callt,
        token_patches,
    );
    for field_key in field_keys {
        push_data(bytecode, field_key);
        bytecode.push(0x50); // swap slot and field key bytes
        bytecode.push(0x8B); // concatenate
        emit_native_contract_call(
            bytecode,
            ir::NativeContract::CryptoLib,
            "keccak256",
            1,
            use_callt,
            token_patches,
        );
    }
}

fn emit_load_mapping(
    bytecode: &mut Vec<u8>,
    module: &ir::Module,
    state_index: usize,
    key_types: &[ValueType],
    use_callt: bool,
    token_patches: &mut Vec<MethodTokenPatch>,
) {
    emit_mapping_slot(
        bytecode,
        module,
        state_index,
        key_types,
        use_callt,
        token_patches,
    );
    emit_syscall(bytecode, "System.Storage.GetContext");
    emit_syscall(bytecode, "System.Storage.Get");
    if let Some(value_type) =
        resolve_loaded_mapping_value_type(module, state_index, key_types.len())
    {
        emit_coerce_storage_value(bytecode, value_type);
    }
}

fn resolve_loaded_mapping_value_type(
    module: &ir::Module,
    state_index: usize,
    key_depth: usize,
) -> Option<&ValueType> {
    let mut current = &module.state_variables.get(state_index)?.ty;

    for _ in 0..key_depth {
        match current {
            ValueType::Mapping { value, .. } => {
                current = value.as_ref();
            }
            ValueType::Array(element) => {
                current = element.as_ref();
            }
            _ => return Some(current),
        }
    }

    Some(current)
}

fn emit_store_mapping(
    bytecode: &mut Vec<u8>,
    module: &ir::Module,
    state_index: usize,
    key_types: &[ValueType],
    use_callt: bool,
    token_patches: &mut Vec<MethodTokenPatch>,
) {
    emit_mapping_slot(
        bytecode,
        module,
        state_index,
        key_types,
        use_callt,
        token_patches,
    );
    emit_syscall(bytecode, "System.Storage.GetContext");
    emit_syscall(bytecode, "System.Storage.Put");
}

fn emit_store_mapping_array_deep_copy(
    bytecode: &mut Vec<u8>,
    module: &ir::Module,
    state_index: usize,
    key_types: &[ValueType],
    use_callt: bool,
    token_patches: &mut Vec<MethodTokenPatch>,
) {
    emit_mapping_slot(
        bytecode,
        module,
        state_index,
        key_types,
        use_callt,
        token_patches,
    );
    // `emit_mapping_slot` leaves `[array, slot]`; the helper below expects
    // `[slot, array]`.
    bytecode.push(0x50); // SWAP
    emit_store_array_value_deep_copy(bytecode, use_callt, token_patches);
}

/// Task #205 — deep-copy a dynamic-array value into a storage slot so the
/// read side sees:
/// - length at the base slot
/// - element `i` at `keccak256(serialize(i) || slot)`
///
/// Stack on entry: `[..., slot, array]`
/// Stack on exit:  `[...]`
fn emit_store_array_value_deep_copy(
    bytecode: &mut Vec<u8>,
    use_callt: bool,
    token_patches: &mut Vec<MethodTokenPatch>,
) {
    bytecode.push(0x4A); // DUP -> [..., slot, array, array]
    bytecode.push(0xCA); // SIZE -> [..., slot, array, length]

    let loop_start_pos = bytecode.len();

    bytecode.push(0x4A); // DUP -> [..., slot, array, i, i]
    bytecode.push(0x10); // PUSH0 -> [..., slot, array, i, i, 0]
    bytecode.push(0x97); // EQUAL -> [..., slot, array, i, (i==0)]
    let jmp_exit_pos = bytecode.len();
    bytecode.push(0x25); // JMPIF_L
    let jmp_exit_operand = bytecode.len();
    bytecode.extend_from_slice(&[0, 0, 0, 0]);

    bytecode.push(0x9D); // DEC -> [..., slot, array, current]

    bytecode.push(0x4A); // DUP -> [..., slot, array, current, current]
    emit_serialize_key(
        bytecode,
        &ValueType::Integer {
            signed: false,
            bits: 256,
        },
        use_callt,
        token_patches,
    );
    bytecode.push(0x13); // PUSH3
    bytecode.push(0x4D); // PICK -> [..., slot, array, current, current_bytes, slot]
    bytecode.push(0x8B); // CAT
    emit_native_contract_call(
        bytecode,
        ir::NativeContract::CryptoLib,
        "keccak256",
        1,
        use_callt,
        token_patches,
    );

    bytecode.push(0x12); // PUSH2
    bytecode.push(0x4D); // PICK -> [..., slot, array, current, element_slot, array]
    bytecode.push(0x12); // PUSH2
    bytecode.push(0x4D); // PICK -> [..., slot, array, current, element_slot, array, current]
    bytecode.push(0xCE); // PICKITEM -> [..., slot, array, current, element_slot, array[current]]

    bytecode.push(0x50); // SWAP -> [..., slot, array, current, array[current], element_slot]
    emit_syscall(bytecode, "System.Storage.GetContext");
    emit_syscall(bytecode, "System.Storage.Put");

    let jmp_back_pos = bytecode.len();
    bytecode.push(0x23); // JMP_L
    let jmp_back_operand = bytecode.len();
    bytecode.extend_from_slice(&[0, 0, 0, 0]);
    let rel_back = (loop_start_pos as i32)
        .checked_sub(jmp_back_pos as i32)
        .unwrap_or(0);
    bytecode[jmp_back_operand..jmp_back_operand + 4].copy_from_slice(&rel_back.to_le_bytes());

    let exit_pos = bytecode.len();
    let rel_exit = (exit_pos as i32)
        .checked_sub(jmp_exit_pos as i32)
        .unwrap_or(0);
    bytecode[jmp_exit_operand..jmp_exit_operand + 4].copy_from_slice(&rel_exit.to_le_bytes());

    bytecode.push(0x45); // DROP -> [..., slot, array]
    bytecode.push(0xCA); // SIZE -> [..., slot, length]
    bytecode.push(0x50); // SWAP -> [..., length, slot]
    emit_syscall(bytecode, "System.Storage.GetContext");
    emit_syscall(bytecode, "System.Storage.Put");
}

/// Task #82: derive the storage slot for a mapping-in-struct-field access such as
/// `slots[k].balances[a]`. Stack before: [trailing_keyN, ..., trailing_key0, outer_keyN, ..., outer_key0]
/// Stack after: [slot_bytes].
struct StructFieldMappingSlot<'a> {
    module: &'a ir::Module,
    state_index: usize,
    key_types: &'a [ValueType],
    field_keys: &'a [[u8; 32]],
    trailing_key_types: &'a [ValueType],
    use_callt: bool,
}

fn emit_struct_field_mapping_slot(
    bytecode: &mut Vec<u8>,
    slot: &StructFieldMappingSlot<'_>,
    token_patches: &mut Vec<MethodTokenPatch>,
) {
    emit_struct_field_slot(
        bytecode,
        slot.module,
        slot.state_index,
        slot.key_types,
        slot.field_keys,
        slot.use_callt,
        token_patches,
    );
    // Stack now: [trailing_keyN, ..., trailing_key0, field_slot].
    for key_type in slot.trailing_key_types {
        bytecode.push(0x50); // swap slot <-> key
        emit_serialize_key(bytecode, key_type, slot.use_callt, token_patches);
        bytecode.push(0x50); // swap key_bytes <-> slot
        bytecode.push(0x8B); // CAT
        emit_native_contract_call(
            bytecode,
            ir::NativeContract::CryptoLib,
            "keccak256",
            1,
            slot.use_callt,
            token_patches,
        );
    }
}

fn emit_load_struct_field_mapping_element(
    bytecode: &mut Vec<u8>,
    slot: &StructFieldMappingSlot<'_>,
    value_type: &ValueType,
    token_patches: &mut Vec<MethodTokenPatch>,
) {
    emit_struct_field_mapping_slot(bytecode, slot, token_patches);
    emit_syscall(bytecode, "System.Storage.GetContext");
    emit_syscall(bytecode, "System.Storage.Get");
    emit_coerce_storage_value(bytecode, value_type);
}

fn emit_store_struct_field_mapping_element(
    bytecode: &mut Vec<u8>,
    slot: &StructFieldMappingSlot<'_>,
    token_patches: &mut Vec<MethodTokenPatch>,
) {
    emit_struct_field_mapping_slot(bytecode, slot, token_patches);
    emit_syscall(bytecode, "System.Storage.GetContext");
    emit_syscall(bytecode, "System.Storage.Put");
}
