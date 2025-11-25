use neo_solidity::ir::{self, LiteralValue, ValueType};
use neo_solidity::solidity::{ContractMetadata, FunctionKind, FunctionMetadata};
use num_bigint::BigInt;
use num_traits::{Signed, ToPrimitive, Zero};
use sha2::{Digest, Sha256};
use std::collections::HashMap;

#[derive(Clone, Debug)]
struct CallPatch {
    position: usize,
    target: String,
}

pub(crate) fn generate_contract_bytecode(
    metadata: &mut ContractMetadata,
    ir_module: &ir::Module,
    verbose: bool,
    optimizer_level: u8,
) -> Vec<u8> {
    let function_map: HashMap<_, _> = ir_module
        .functions
        .iter()
        .map(|function| (function.name.as_str(), function))
        .collect();

    if verbose {
        println!("  • Using optimizer level {}", optimizer_level.min(3));
    }

    let mut bytecode = Vec::new();
    let mut call_fixups: Vec<CallPatch> = Vec::new();

    for method in metadata.methods.iter_mut() {
        if matches!(method.kind, FunctionKind::Constructor) {
            continue;
        }

        let method_name = method.name.clone();
        let ir_function = match function_map.get(method_name.as_str()) {
            Some(func) => *func,
            None => {
                eprintln!(
                    "error: internal compiler error: missing IR for method '{}'",
                    method_name
                );
                std::process::exit(1);
            }
        };

        let instruction_count: usize = ir_function
            .basic_blocks
            .iter()
            .map(|block| block.instructions.len())
            .sum();

        let offset = bytecode.len() as u32;
        method.offset = offset;

        if verbose {
            println!(
                "  • Emitting method '{}' at offset {} ({} IR instruction(s))",
                method_name, offset, instruction_count
            );
        }

        let (function_bytes, patches) = emit_ir_function(ir_function, ir_module, method);
        let base_position = bytecode.len();
        bytecode.extend_from_slice(&function_bytes);

        for patch in patches {
            call_fixups.push(CallPatch {
                position: base_position + patch.position,
                target: patch.target,
            });
        }
    }

    let offset_map: HashMap<String, u32> = metadata
        .methods
        .iter()
        .filter(|method| !matches!(method.kind, FunctionKind::Constructor))
        .map(|method| (method.name.clone(), method.offset))
        .collect();

    for fixup in call_fixups {
        if let Some(target_offset) = offset_map.get(&fixup.target) {
            let bytes = target_offset.to_le_bytes();
            if fixup.position + 4 <= bytecode.len() {
                bytecode[fixup.position..fixup.position + 4].copy_from_slice(&bytes);
            }
        } else {
            eprintln!(
                "warning: unresolved call target '{}' (offset unavailable)",
                fixup.target
            );
        }
    }

    if bytecode.is_empty() {
        bytecode.push(0x40); // RET
    }

    bytecode
}

fn emit_ir_function(
    function: &ir::Function,
    module: &ir::Module,
    method: &FunctionMetadata,
) -> (Vec<u8>, Vec<CallPatch>) {
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
                } => emit_load_mapping(&mut local, module, *state_index, key_types),
                ir::Instruction::StoreMappingElement {
                    state_index,
                    key_types,
                } => emit_store_mapping(&mut local, module, *state_index, key_types),
                ir::Instruction::LoadStructField {
                    state_index,
                    key_types,
                    field_key,
                    ..
                } => emit_load_struct_field(&mut local, module, *state_index, key_types, field_key),
                ir::Instruction::StoreStructField {
                    state_index,
                    key_types,
                    field_key,
                    ..
                } => {
                    emit_store_struct_field(&mut local, module, *state_index, key_types, field_key)
                }
                ir::Instruction::LoadRuntimeValue(value) => {
                    emit_load_runtime_value(&mut local, value)
                }
                ir::Instruction::GetSize => local.push(0xCA),
                ir::Instruction::CallBuiltin { builtin, .. } => {
                    emit_builtin_call(&mut local, builtin);
                }
                ir::Instruction::CallFunction { name, .. } => {
                    local.push(0x34); // CALL
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
                ir::Instruction::NewArray { .. } => emit_new_array(&mut local),
                ir::Instruction::ArrayGet => emit_array_get(&mut local),
                ir::Instruction::ArraySet => emit_array_set(&mut local),
                ir::Instruction::BitwiseNot => {
                    local.push(0x90); // INVERT
                }
                ir::Instruction::Jump { target } => {
                    local.push(0x22); // JMP
                    let position = local.len();
                    local.extend_from_slice(&[0, 0, 0, 0]);
                    jump_patches.push((position, *target));
                }
                ir::Instruction::JumpIf { target } => {
                    local.push(0x26); // JMPIFNOT
                    let position = local.len();
                    local.extend_from_slice(&[0, 0, 0, 0]);
                    jump_patches.push((position, *target));
                }
                ir::Instruction::Label(label) => {
                    label_offsets.insert(*label, local.len() as u32);
                }
                ir::Instruction::Abort => {
                    local.push(0x38); // ABORT
                }
            }
        }
    }

    for (position, label) in jump_patches {
        let target_offset = label_offsets
            .get(&label)
            .copied()
            .unwrap_or(local.len() as u32);
        local[position..position + 4].copy_from_slice(&target_offset.to_le_bytes());
    }

    (local, call_patches)
}

fn append_default_value(bytecode: &mut Vec<u8>, value_type: &ValueType) {
    match value_type {
        ValueType::Integer { .. } => bytecode.push(0x10),
        ValueType::Boolean => bytecode.push(0x10),
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
        ValueType::Any => bytecode.push(0x0B), // NULL
    }
}

fn emit_load_parameter(bytecode: &mut Vec<u8>, _method: &FunctionMetadata, index: usize) {
    if index <= 6 {
        bytecode.push(0x78 + index as u8);
    } else {
        bytecode.push(0x7F); // LDARG
        bytecode.push(index as u8);
    }
}

fn emit_load_local(bytecode: &mut Vec<u8>, index: usize) {
    match index {
        0..=6 => bytecode.push(0x68 + index as u8),
        _ => {
            bytecode.push(0x6F); // LDLOC
            bytecode.push(index as u8);
        }
    }
}

fn emit_store_local(bytecode: &mut Vec<u8>, index: usize) {
    match index {
        0..=6 => bytecode.push(0x70 + index as u8),
        _ => {
            bytecode.push(0x77); // STLOC
            bytecode.push(index as u8);
        }
    }
}

fn emit_binary_op(bytecode: &mut Vec<u8>, operator: ir::BinaryOperator) {
    let opcode = match operator {
        ir::BinaryOperator::Add => 0x9E,
        ir::BinaryOperator::Sub => 0x9F,
        ir::BinaryOperator::Mul => 0xA0,
        ir::BinaryOperator::Div => 0xA1,
        ir::BinaryOperator::Mod => 0xA2,
        ir::BinaryOperator::BitAnd => 0x91,
        ir::BinaryOperator::BitOr => 0x92,
        ir::BinaryOperator::BitXor => 0x93,
        ir::BinaryOperator::Shl => 0xA8,
        ir::BinaryOperator::Shr => 0xA9,
        ir::BinaryOperator::Lt => 0xB5,
        ir::BinaryOperator::Le => 0xB6,
        ir::BinaryOperator::Gt => 0xB7,
        ir::BinaryOperator::Ge => 0xB8,
        ir::BinaryOperator::Eq => 0x97,
        ir::BinaryOperator::Ne => 0x98,
    };
    bytecode.push(opcode);
}

fn push_literal_value(bytecode: &mut Vec<u8>, literal: &LiteralValue) {
    match literal {
        LiteralValue::Integer(value) => push_integer_bigint(bytecode, value),
        LiteralValue::Boolean(true) => bytecode.push(0x11),
        LiteralValue::Boolean(false) => bytecode.push(0x10),
        LiteralValue::String(bytes) => push_data(bytecode, bytes),
        LiteralValue::ByteArray(bytes) => push_data(bytecode, bytes),
        LiteralValue::Address(bytes) => push_data(bytecode, bytes),
    }
}

fn push_integer_bigint(bytecode: &mut Vec<u8>, value: &BigInt) {
    if value.is_zero() {
        bytecode.push(0x10);
        return;
    }

    if *value == BigInt::from(-1) {
        bytecode.push(0x0F);
        return;
    }

    if value.is_positive() {
        if let Some(n) = value.to_u8() {
            if n <= 16 {
                bytecode.push(0x10 + n);
                return;
            }
        }
    }

    let mut bytes = value.to_signed_bytes_le();
    if bytes.is_empty() {
        bytes.push(0);
    }
    push_data(bytecode, &bytes);
}

fn emit_load_state(bytecode: &mut Vec<u8>, module: &ir::Module, index: usize) {
    let key = module
        .state_variables
        .get(index)
        .map(|state| state.storage_key.as_slice())
        .unwrap_or(&[]);

    emit_syscall(bytecode, "System.Storage.GetContext");
    push_data(bytecode, key);
    emit_syscall(bytecode, "System.Storage.Get");
}

fn emit_store_state(bytecode: &mut Vec<u8>, module: &ir::Module, index: usize) {
    let key = module
        .state_variables
        .get(index)
        .map(|state| state.storage_key.as_slice())
        .unwrap_or(&[]);

    push_data(bytecode, key);
    emit_syscall(bytecode, "System.Storage.GetContext");
    bytecode.push(0x51); // ROT
    bytecode.push(0x50); // SWAP
    emit_syscall(bytecode, "System.Storage.Put");
}

fn emit_serialize_key(bytecode: &mut Vec<u8>, _key_type: &ValueType) {
    emit_syscall(bytecode, "System.Runtime.Serialize");
}

fn emit_mapping_slot(
    bytecode: &mut Vec<u8>,
    module: &ir::Module,
    state_index: usize,
    key_types: &[ValueType],
) {
    let base_slot = module
        .state_variables
        .get(state_index)
        .map(|state| state.storage_key.clone())
        .unwrap_or_else(|| vec![0u8; 32]);

    push_data(bytecode, &base_slot);

    for key_type in key_types {
        bytecode.push(0x50); // swap slot <-> key
        emit_serialize_key(bytecode, key_type);
        bytecode.push(0x50); // swap key_bytes <-> slot
        bytecode.push(0x8B); // concatenate key and slot
        emit_syscall(bytecode, "System.Crypto.SHA256");
    }
}

fn emit_struct_field_slot(
    bytecode: &mut Vec<u8>,
    module: &ir::Module,
    state_index: usize,
    key_types: &[ValueType],
    field_key: &[u8; 32],
) {
    emit_mapping_slot(bytecode, module, state_index, key_types);
    push_data(bytecode, field_key);
    bytecode.push(0x50); // swap slot and field key bytes
    bytecode.push(0x8B); // concatenate
    emit_syscall(bytecode, "System.Crypto.SHA256");
}

fn emit_load_mapping(
    bytecode: &mut Vec<u8>,
    module: &ir::Module,
    state_index: usize,
    key_types: &[ValueType],
) {
    emit_mapping_slot(bytecode, module, state_index, key_types);
    emit_syscall(bytecode, "System.Storage.GetContext");
    bytecode.push(0x50); // SWAP
    emit_syscall(bytecode, "System.Storage.Get");
}

fn emit_store_mapping(
    bytecode: &mut Vec<u8>,
    module: &ir::Module,
    state_index: usize,
    key_types: &[ValueType],
) {
    emit_mapping_slot(bytecode, module, state_index, key_types);
    emit_syscall(bytecode, "System.Storage.GetContext");
    bytecode.push(0x50); // swap slot and context -> [value, context, slot]
    bytecode.push(0x51); // ROT -> [context, slot, value]
    emit_syscall(bytecode, "System.Storage.Put");
}

fn emit_load_struct_field(
    bytecode: &mut Vec<u8>,
    module: &ir::Module,
    state_index: usize,
    key_types: &[ValueType],
    field_key: &[u8; 32],
) {
    emit_struct_field_slot(bytecode, module, state_index, key_types, field_key);
    emit_syscall(bytecode, "System.Storage.GetContext");
    bytecode.push(0x50); // swap context and slot
    emit_syscall(bytecode, "System.Storage.Get");
}

fn emit_store_struct_field(
    bytecode: &mut Vec<u8>,
    module: &ir::Module,
    state_index: usize,
    key_types: &[ValueType],
    field_key: &[u8; 32],
) {
    emit_struct_field_slot(bytecode, module, state_index, key_types, field_key);
    emit_syscall(bytecode, "System.Storage.GetContext");
    bytecode.push(0x50); // swap slot and context -> [value, context, slot]
    bytecode.push(0x51); // ROT -> [context, slot, value]
    emit_syscall(bytecode, "System.Storage.Put");
}

fn emit_load_storage_dynamic(bytecode: &mut Vec<u8>) {
    emit_syscall(bytecode, "System.Storage.GetContext");
    bytecode.push(0x50); // SWAP -> [context, slot]
    emit_syscall(bytecode, "System.Storage.Get");
}

fn emit_new_array(bytecode: &mut Vec<u8>) {
    bytecode.push(0xC3); // NEWARRAY
}

fn emit_array_get(bytecode: &mut Vec<u8>) {
    bytecode.push(0xCE); // PICKITEM
}

fn emit_array_set(bytecode: &mut Vec<u8>) {
    bytecode.push(0xD0); // SETITEM
}

fn emit_load_runtime_value(bytecode: &mut Vec<u8>, value: &ir::RuntimeValue) {
    match value {
        ir::RuntimeValue::MsgSender => emit_syscall(bytecode, "System.Runtime.CallingScriptHash"),
        ir::RuntimeValue::BlockTimestamp => emit_syscall(bytecode, "System.Runtime.GetTime"),
    }
}

fn emit_builtin_call(bytecode: &mut Vec<u8>, builtin: &ir::BuiltinCall) {
    match builtin {
        ir::BuiltinCall::RuntimeNotify => emit_syscall(bytecode, "System.Runtime.Notify"),
        ir::BuiltinCall::RuntimeCheckWitness => {
            emit_syscall(bytecode, "System.Runtime.CheckWitness")
        }
        ir::BuiltinCall::Keccak256
        | ir::BuiltinCall::StorageFind
        | ir::BuiltinCall::TypeOf
        | ir::BuiltinCall::AbiEncode
        | ir::BuiltinCall::AbiEncodePacked
        | ir::BuiltinCall::AbiEncodeWithSignature
        | ir::BuiltinCall::AbiDecode => {}
    }
}

fn emit_event(bytecode: &mut Vec<u8>, module: &ir::Module, index: usize, arg_count: usize) {
    if module.events.get(index).is_some() {
        emit_event_payload(bytecode, arg_count);
    }
}

fn emit_event_by_name(bytecode: &mut Vec<u8>, _name: &str, arg_count: usize) {
    emit_event_payload(bytecode, arg_count);
}

fn emit_event_payload(bytecode: &mut Vec<u8>, arg_count: usize) {
    let total_items = arg_count.saturating_add(1);
    let total_bigint = BigInt::from(total_items);
    push_integer_bigint(bytecode, &total_bigint);
    bytecode.push(0xC0); // PACK
    emit_syscall(bytecode, "System.Runtime.Notify");
}

fn emit_syscall(bytecode: &mut Vec<u8>, name: &str) {
    bytecode.push(0x41);
    bytecode.extend_from_slice(&interop_id_bytes(name));
}

pub(crate) fn interop_id_bytes(name: &str) -> [u8; 4] {
    let mut hasher = Sha256::new();
    hasher.update(name.as_bytes());
    let digest = hasher.finalize();
    [digest[0], digest[1], digest[2], digest[3]]
}

fn push_data(bytecode: &mut Vec<u8>, data: &[u8]) {
    if data.len() <= u8::MAX as usize {
        bytecode.push(0x0C);
        bytecode.push(data.len() as u8);
    } else if data.len() <= u16::MAX as usize {
        bytecode.push(0x0D);
        bytecode.extend_from_slice(&(data.len() as u16).to_le_bytes());
    } else {
        bytecode.push(0x0E);
        bytecode.extend_from_slice(&(data.len() as u32).to_le_bytes());
    }
    bytecode.extend_from_slice(data);
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use neo_solidity::solidity::analyse_source;
    use neo_solidity::{
        frontend::VisibilityKind,
        solidity::{FunctionKind, FunctionMetadata, StateMutability},
    };

    #[test]
    fn mapping_code_generation_emits_storage_ops() {
        let source = r#"
        pragma solidity ^0.8.19;

        contract MappingExample {
            mapping(address => uint256) public balances;

            function setBalance(address owner, uint256 amount) public {
                balances[owner] = amount;
            }

            function getBalance(address owner) public view returns (uint256) {
                return balances[owner];
            }
        }
        "#;

        let mut metadata = analyse_source(source).expect("analysis failed");
        let ir_module = ir::Module::from_contract(&metadata).expect("IR lowering failed");
        let bytecode = generate_contract_bytecode(&mut metadata, &ir_module, false, 2);

        assert!(!bytecode.is_empty());

        let sha_id = interop_id_bytes("System.Crypto.SHA256");
        assert!(bytecode.windows(4).any(|window| window == sha_id));

        let put_id = interop_id_bytes("System.Storage.Put");
        assert!(bytecode.windows(4).any(|window| window == put_id));
        let get_id = interop_id_bytes("System.Storage.Get");
        assert!(bytecode.windows(4).any(|window| window == get_id));
    }

    #[test]
    fn event_emission_places_name_first_in_payload() {
        let source = r#"
        pragma solidity ^0.8.19;

        contract EventOrder {
            event Ping(uint256 a, uint256 b);

            function fire() public {
                emit Ping(1, 2);
            }
        }
        "#;

        let mut metadata = analyse_source(source).expect("analysis failed");
        let ir_module = ir::Module::from_contract(&metadata).expect("IR lowering failed");
        let bytecode = generate_contract_bytecode(&mut metadata, &ir_module, false, 2);

        let notify_id = interop_id_bytes("System.Runtime.Notify");
        let notify_sequence: Vec<u8> = std::iter::once(0x41u8).chain(notify_id).collect();
        assert!(
            bytecode
                .windows(notify_sequence.len())
                .any(|window| window == notify_sequence),
            "expected Runtime.Notify syscall"
        );

        let mut expected_sequence = vec![0x0C, 0x04];
        expected_sequence.extend_from_slice(b"Ping");
        expected_sequence.extend_from_slice(&[0x11, 0x12, 0x13, 0xC0]);

        assert!(
            bytecode
                .windows(expected_sequence.len())
                .any(|window| window == expected_sequence),
            "expected event name to be pushed before args and packed"
        );
    }

    #[test]
    fn empty_contract_emits_ret_instruction() {
        // No functions other than constructors -> should produce a single RET (0x40)
        let mut metadata = ContractMetadata {
            name: "Empty".to_string(),
            methods: vec![FunctionMetadata {
                name: "constructor".to_string(),
                kind: FunctionKind::Constructor,
                parameters: vec![],
                return_parameters: vec![],
                state_mutability: StateMutability::NonPayable,
                visibility: VisibilityKind::Public,
                offset: 0,
                body: None,
                selector: [0u8; 4],
            }],
            events: vec![],
            uses_storage: false,
            state_variables: vec![],
            structs: vec![],
        };

        let ir_module = ir::Module {
            functions: vec![],
            state_variables: vec![],
            events: vec![],
        };

        let bytecode = generate_contract_bytecode(&mut metadata, &ir_module, false, 2);
        assert_eq!(bytecode, vec![0x40], "should emit a lone RET");
    }

    #[test]
    fn call_fixups_patch_offsets() {
        let mut metadata = ContractMetadata {
            name: "Callers".to_string(),
            methods: vec![
                FunctionMetadata {
                    name: "foo".to_string(),
                    kind: FunctionKind::Regular,
                    parameters: vec![],
                    return_parameters: vec![],
                    state_mutability: StateMutability::View,
                    visibility: VisibilityKind::Public,
                    offset: 0,
                    body: None,
                    selector: [0u8; 4],
                },
                FunctionMetadata {
                    name: "bar".to_string(),
                    kind: FunctionKind::Regular,
                    parameters: vec![],
                    return_parameters: vec![],
                    state_mutability: StateMutability::View,
                    visibility: VisibilityKind::Public,
                    offset: 0,
                    body: None,
                    selector: [0u8; 4],
                },
            ],
            events: vec![],
            uses_storage: false,
            state_variables: vec![],
            structs: vec![],
        };

        let mut module = ir::Module {
            functions: vec![],
            state_variables: vec![],
            events: vec![],
        };

        module.functions.push(ir::Function {
            name: "foo".to_string(),
            kind: ir::FunctionKind::Regular,
            parameters: vec![],
            returns: vec![],
            basic_blocks: vec![ir::BasicBlock {
                instructions: vec![
                    ir::Instruction::CallFunction {
                        name: "bar".to_string(),
                        arg_count: 0,
                    },
                    ir::Instruction::ReturnVoid,
                ],
            }],
            local_count: 0,
        });
        module.functions.push(ir::Function {
            name: "bar".to_string(),
            kind: ir::FunctionKind::Regular,
            parameters: vec![],
            returns: vec![],
            basic_blocks: vec![ir::BasicBlock {
                instructions: vec![ir::Instruction::ReturnVoid],
            }],
            local_count: 0,
        });

        let bytecode = generate_contract_bytecode(&mut metadata, &module, false, 2);

        let foo_offset = metadata
            .methods
            .iter()
            .find(|m| m.name == "foo")
            .map(|m| m.offset)
            .unwrap();
        assert_eq!(foo_offset, 0, "foo should start at offset 0");

        let bar_offset = metadata
            .methods
            .iter()
            .find(|m| m.name == "bar")
            .map(|m| m.offset)
            .unwrap();
        assert!(
            bar_offset > 0,
            "bar should be placed after foo in bytecode sequence"
        );

        // Locate CALL opcode and patched offset (little endian immediately following)
        let call_pos = bytecode
            .iter()
            .position(|b| *b == 0x34)
            .expect("CALL opcode should be present");
        let patched = u32::from_le_bytes([
            bytecode[call_pos + 1],
            bytecode[call_pos + 2],
            bytecode[call_pos + 3],
            bytecode[call_pos + 4],
        ]);
        assert_eq!(
            patched, bar_offset,
            "CALL target should be patched with bar's offset"
        );
    }
}
