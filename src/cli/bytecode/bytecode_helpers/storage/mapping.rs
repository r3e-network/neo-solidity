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
    if let Some(ir::StateVariable {
        ty: ValueType::Mapping { value, .. },
        ..
    }) = module.state_variables.get(state_index)
    {
        emit_coerce_storage_value(bytecode, value);
    }
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
