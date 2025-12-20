fn emit_ir_function(
    function: &ir::Function,
    module: &ir::Module,
    method: &FunctionMetadata,
    use_callt: bool,
) -> (Vec<u8>, Vec<CallPatch>, Vec<MethodTokenPatch>) {
    use std::{collections::HashMap, convert::TryFrom};

    let mut local = Vec::new();
    let arg_count = method.parameters.len() as u8;
    let local_count = u8::try_from(function.local_count).unwrap_or(u8::MAX);
    if local_count > 0 || arg_count > 0 {
        local.push(0x57); // INITSLOT
        local.push(local_count);
        local.push(arg_count);
    }
    let mut label_offsets: HashMap<usize, u32> = HashMap::new();
    let mut jump_patches: Vec<(usize, usize)> = Vec::new();
    let mut call_patches: Vec<CallPatch> = Vec::new();
    let mut token_patches: Vec<MethodTokenPatch> = Vec::new();

    for block in &function.basic_blocks {
        for instruction in &block.instructions {
            match instruction {
                ir::Instruction::Drop(_) => local.push(0x45),
                ir::Instruction::LoadParameter(index) => {
                    emit_load_parameter(&mut local, method, *index)
                }
                ir::Instruction::PushLiteral(literal) => {
                    push_literal_value(&mut local, literal);
                }
                ir::Instruction::BinaryOp(operator) => emit_binary_op(&mut local, *operator),
                ir::Instruction::Return | ir::Instruction::ReturnVoid => local.push(0x40),
                ir::Instruction::ReturnDefault(value_type) => {
                    append_default_value(&mut local, value_type);
                    local.push(0x40);
                }
                ir::Instruction::LoadState(index) => emit_load_state(&mut local, module, *index),
                ir::Instruction::StoreState(index) => emit_store_state(&mut local, module, *index),
                ir::Instruction::LoadStorageDynamic => emit_load_storage_dynamic(&mut local),
                ir::Instruction::LoadLocal(index) => emit_load_local(&mut local, *index),
                ir::Instruction::StoreLocal(index) => emit_store_local(&mut local, *index),
                ir::Instruction::LoadMappingElement {
                    state_index,
                    key_types,
                } => emit_load_mapping(
                    &mut local,
                    module,
                    *state_index,
                    key_types,
                    use_callt,
                    &mut token_patches,
                ),
                ir::Instruction::StoreMappingElement {
                    state_index,
                    key_types,
                } => emit_store_mapping(
                    &mut local,
                    module,
                    *state_index,
                    key_types,
                    use_callt,
                    &mut token_patches,
                ),
                ir::Instruction::LoadStructField {
                    state_index,
                    key_types,
                    field_keys,
                    field_type,
                    ..
                } => emit_load_struct_field(
                    &mut local,
                    module,
                    *state_index,
                    key_types,
                    StructFieldAccess {
                        field_keys: field_keys.as_slice(),
                        ty: field_type,
                    },
                    use_callt,
                    &mut token_patches,
                ),
                ir::Instruction::StoreStructField {
                    state_index,
                    key_types,
                    field_keys,
                    field_type,
                    ..
                } => {
                    emit_store_struct_field(
                        &mut local,
                        module,
                        *state_index,
                        key_types,
                        StructFieldAccess {
                            field_keys: field_keys.as_slice(),
                            ty: field_type,
                        },
                        use_callt,
                        &mut token_patches,
                    )
                }
                ir::Instruction::LoadStructArrayElement {
                    state_index,
                    key_types,
                    field_keys,
                    element_type,
                } => emit_load_struct_array_element(
                    &mut local,
                    module,
                    *state_index,
                    key_types,
                    StructArrayElementAccess {
                        field_keys: field_keys.as_slice(),
                        element_type,
                    },
                    use_callt,
                    &mut token_patches,
                ),
                ir::Instruction::StoreStructArrayElement {
                    state_index,
                    key_types,
                    field_keys,
                    element_type,
                } => emit_store_struct_array_element(
                    &mut local,
                    module,
                    *state_index,
                    key_types,
                    StructArrayElementAccess {
                        field_keys: field_keys.as_slice(),
                        element_type,
                    },
                    use_callt,
                    &mut token_patches,
                ),
                ir::Instruction::LoadRuntimeValue(value) => {
                    emit_load_runtime_value(&mut local, value, use_callt, &mut token_patches)
                }
                ir::Instruction::GetSize => local.push(0xCA),
                ir::Instruction::CallBuiltin { builtin, arg_count } => {
                    emit_builtin_call(
                        &mut local,
                        builtin,
                        *arg_count,
                        use_callt,
                        &mut token_patches,
                    );
                }
                ir::Instruction::CallFunction { name, arg_count } => {
                    // NeoVM's `INITSLOT` assigns argument slots by popping values from the
                    // evaluation stack in order, meaning the callee sees the last-pushed value
                    // as `arg0`. Solidity evaluates arguments left-to-right, so call sites push
                    // `arg0, arg1, ...` onto the stack. Before calling, reverse the argument
                    // segment so that `arg0` is on top and parameters arrive in the expected
                    // order inside the callee.
                    if *arg_count > 1 {
                        push_integer_bigint(&mut local, &BigInt::from(*arg_count));
                        local.push(0x55); // REVERSEN
                    }
                    // NeoVM CALL_L uses a 4-byte signed offset relative to the
                    // beginning of the CALL_L instruction. We always use the
                    // wide form for simplicity.
                    local.push(0x35); // CALL_L
                    let patch_pos = local.len();
                    local.extend_from_slice(&[0, 0, 0, 0]);
                    call_patches.push(CallPatch {
                        position: patch_pos,
                        target: name.clone(),
                    });
                }
                ir::Instruction::EmitEvent {
                    event_index,
                    arg_count,
                } => emit_event(&mut local, module, *event_index, *arg_count),
                ir::Instruction::EmitEventByName { name, arg_count } => {
                    emit_event_by_name(&mut local, name, *arg_count)
                }
                ir::Instruction::Convert { target } => emit_convert(&mut local, *target),
                ir::Instruction::NewBuffer => emit_new_buffer(&mut local),
                ir::Instruction::NewArray { .. } => emit_new_array(&mut local),
                ir::Instruction::ArrayGet => emit_array_get(&mut local),
                ir::Instruction::ArraySet => emit_array_set(&mut local),
                ir::Instruction::MemCpy => {
                    local.push(0x89); // MEMCPY
                }
                ir::Instruction::ReverseItems => {
                    local.push(0xD1); // REVERSEITEMS
                }
                ir::Instruction::BitwiseNot => {
                    local.push(0x90); // INVERT
                }
                ir::Instruction::Try { catch_target } => {
                    // NeoVM TRY_L uses 4-byte signed offsets (catch, finally) relative to
                    // the beginning of the TRY_L instruction. We always emit the wide form
                    // and omit `finally` (offset = 0).
                    local.push(0x3C); // TRY_L
                    let position = local.len();
                    local.extend_from_slice(&[0, 0, 0, 0]); // catch offset placeholder
                    jump_patches.push((position, *catch_target));
                    local.extend_from_slice(&[0, 0, 0, 0]); // finally offset (absent)
                }
                ir::Instruction::EndTry { target } => {
                    // NeoVM ENDTRY_L uses a 4-byte signed offset relative to the beginning
                    // of the ENDTRY_L instruction. We always emit the wide form.
                    local.push(0x3E); // ENDTRY_L
                    let position = local.len();
                    local.extend_from_slice(&[0, 0, 0, 0]);
                    jump_patches.push((position, *target));
                }
                ir::Instruction::Jump { target } => {
                    // NeoVM JMP_L uses a 4-byte signed offset relative to the
                    // beginning of the JMP_L instruction. We always use the
                    // wide form for simplicity.
                    local.push(0x23); // JMP_L
                    let position = local.len();
                    local.extend_from_slice(&[0, 0, 0, 0]);
                    jump_patches.push((position, *target));
                }
                ir::Instruction::JumpIf { target } => {
                    // IR JumpIf branches when the condition is false.
                    // NeoVM JMPIFNOT_L uses a 4-byte signed offset relative to the
                    // beginning of the JMPIFNOT_L instruction.
                    local.push(0x27); // JMPIFNOT_L
                    let position = local.len();
                    local.extend_from_slice(&[0, 0, 0, 0]);
                    jump_patches.push((position, *target));
                }
                ir::Instruction::Label(label) => {
                    label_offsets.insert(*label, local.len() as u32);
                }
                ir::Instruction::AbortMsg => {
                    local.push(0xE0); // ABORTMSG
                }
                ir::Instruction::Abort => {
                    local.push(0x38); // ABORT
                }
                ir::Instruction::Throw => {
                    local.push(0x3A); // THROW
                }
            }
        }
    }

    for (position, label) in jump_patches {
        let target_offset = label_offsets
            .get(&label)
            .copied()
            .unwrap_or(local.len() as u32) as i32;
        // `position` points at the beginning of the 4-byte operand; the opcode
        // is immediately before it.
        let opcode_pos = (position - 1) as i32;
        let relative = target_offset
            .checked_sub(opcode_pos)
            .unwrap_or(0);
        local[position..position + 4].copy_from_slice(&relative.to_le_bytes());
    }

    (local, call_patches, token_patches)
}

fn append_default_value(bytecode: &mut Vec<u8>, value_type: &ValueType) {
    match value_type {
        ValueType::Integer { .. } => bytecode.push(0x10),
        // Solidity booleans are represented as 0/1 values on the stack. Use PUSH0 for the
        // default `false` to match numeric/ABI semantics.
        ValueType::Boolean => bytecode.push(0x10), // PUSH0
        ValueType::String => push_data(bytecode, &[]),
        ValueType::Address => push_data(bytecode, &[0u8; 20]),
        ValueType::ByteArray { fixed_len } => {
            if let Some(len) = fixed_len {
                let zeros = vec![0u8; *len as usize];
                push_data(bytecode, &zeros);
            } else {
                push_data(bytecode, &[]);
            }
        }
        ValueType::Array(_) => bytecode.push(0xC2), // NEWARRAY0
        ValueType::Mapping { .. } => bytecode.push(0xC8), // NEWMAP
        ValueType::Struct { .. } => bytecode.push(0xC5), // NEWSTRUCT0
        ValueType::Any => bytecode.push(0x0B),      // NULL
    }
}
