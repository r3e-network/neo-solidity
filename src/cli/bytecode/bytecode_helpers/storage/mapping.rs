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

/// Task #82: derive the storage slot for a mapping-in-struct-field access such as
/// `slots[k].balances[a]`. Stack before: [trailing_keyN, ..., trailing_key0, outer_keyN, ..., outer_key0]
/// Stack after: [slot_bytes].
fn emit_struct_field_mapping_slot(
    bytecode: &mut Vec<u8>,
    module: &ir::Module,
    state_index: usize,
    key_types: &[ValueType],
    field_keys: &[[u8; 32]],
    trailing_key_types: &[ValueType],
    use_callt: bool,
    token_patches: &mut Vec<MethodTokenPatch>,
) {
    emit_struct_field_slot(
        bytecode,
        module,
        state_index,
        key_types,
        field_keys,
        use_callt,
        token_patches,
    );
    // Stack now: [trailing_keyN, ..., trailing_key0, field_slot].
    for key_type in trailing_key_types {
        bytecode.push(0x50); // swap slot <-> key
        emit_serialize_key(bytecode, key_type, use_callt, token_patches);
        bytecode.push(0x50); // swap key_bytes <-> slot
        bytecode.push(0x8B); // CAT
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

fn emit_load_struct_field_mapping_element(
    bytecode: &mut Vec<u8>,
    module: &ir::Module,
    state_index: usize,
    key_types: &[ValueType],
    field_keys: &[[u8; 32]],
    trailing_key_types: &[ValueType],
    value_type: &ValueType,
    use_callt: bool,
    token_patches: &mut Vec<MethodTokenPatch>,
) {
    emit_struct_field_mapping_slot(
        bytecode,
        module,
        state_index,
        key_types,
        field_keys,
        trailing_key_types,
        use_callt,
        token_patches,
    );
    emit_syscall(bytecode, "System.Storage.GetContext");
    emit_syscall(bytecode, "System.Storage.Get");
    emit_coerce_storage_value(bytecode, value_type);
}

fn emit_store_struct_field_mapping_element(
    bytecode: &mut Vec<u8>,
    module: &ir::Module,
    state_index: usize,
    key_types: &[ValueType],
    field_keys: &[[u8; 32]],
    trailing_key_types: &[ValueType],
    use_callt: bool,
    token_patches: &mut Vec<MethodTokenPatch>,
) {
    emit_struct_field_mapping_slot(
        bytecode,
        module,
        state_index,
        key_types,
        field_keys,
        trailing_key_types,
        use_callt,
        token_patches,
    );
    emit_syscall(bytecode, "System.Storage.GetContext");
    emit_syscall(bytecode, "System.Storage.Put");
}
