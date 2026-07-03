use super::*;
use crate::ir::ir_context::DiagnosticContext;

// Native Contract Call Handling
//
// This module handles lowering of Solidity native contract calls to NeoVM bytecode.
// See the source code for full implementation of NeoToken, GasToken, ContractManagement,
// PolicyContract, OracleContract, RoleManagement, Notary, Treasury, LedgerContract,
// CryptoLib, and StdLib native contract operations.
//
// Note: This file is large (~1300 lines) due to the complexity of native contract handling.
// Consider splitting by contract type in future refactoring.

pub(crate) struct NativeContractDescriptor {
    hash: [u8; 20],
    name: &'static str,
}

/// Record the standard "NativeCalls.<name> requires N argument(s), got M"
/// diagnostic when the actual arg count disagrees, and return `Some(false)`
/// so callers can early-return. The function returns `Some(false)` (not
/// `None` / `Option<bool>`) so it composes with the existing
/// `return Some(false)` shape used by every native-call lowering site
/// without changing the outer match signature.
fn check_arg_count(
    ctx: &mut impl DiagnosticContext,
    name: &str,
    args: &[Expression],
    expected: usize,
) -> Option<bool> {
    if args.len() != expected {
        ctx.record_error(format!(
            "NativeCalls.{name} requires {expected} argument(s), got {}",
            args.len()
        ));
        return Some(false);
    }
    None
}

pub(crate) const NATIVE_CONTRACTS: [NativeContractDescriptor; 11] = [
    NativeContractDescriptor {
        hash: [
            0xf5, 0x63, 0xea, 0x40, 0xbc, 0x28, 0x3d, 0x4d, 0x0e, 0x05, 0xc4, 0x8e, 0xa3, 0x05,
            0xb3, 0xf2, 0xa0, 0x73, 0x40, 0xef,
        ],
        name: "NeoToken",
    },
    NativeContractDescriptor {
        hash: [
            0xcf, 0x76, 0xe2, 0x8b, 0xd0, 0x06, 0x2c, 0x4a, 0x47, 0x8e, 0xe3, 0x55, 0x61, 0x01,
            0x13, 0x19, 0xf3, 0xcf, 0xa4, 0xd2,
        ],
        name: "GasToken",
    },
    NativeContractDescriptor {
        hash: [
            0xfd, 0xa3, 0xfa, 0x43, 0x46, 0xea, 0x53, 0x2a, 0x25, 0x8f, 0xc4, 0x97, 0xdd, 0xad,
            0xdb, 0x64, 0x37, 0xc9, 0xfd, 0xff,
        ],
        name: "ContractManagement",
    },
    NativeContractDescriptor {
        hash: [
            0x7b, 0xc6, 0x81, 0xc0, 0xa1, 0xf7, 0x1d, 0x54, 0x34, 0x57, 0xb6, 0x8b, 0xba, 0x8d,
            0x5f, 0x9f, 0xdd, 0x4e, 0x5e, 0xcc,
        ],
        name: "PolicyContract",
    },
    NativeContractDescriptor {
        hash: [
            0x58, 0x87, 0x17, 0x11, 0x7e, 0x0a, 0xa8, 0x10, 0x72, 0xaf, 0xab, 0x71, 0xd2, 0xdd,
            0x89, 0xfe, 0x7c, 0x4b, 0x92, 0xfe,
        ],
        name: "OracleContract",
    },
    NativeContractDescriptor {
        hash: [
            0xe2, 0x95, 0xe3, 0x91, 0x54, 0x4c, 0x17, 0x8a, 0xd9, 0x4f, 0x03, 0xec, 0x4d, 0xcd,
            0xff, 0x78, 0x53, 0x4e, 0xcf, 0x49,
        ],
        name: "RoleManagement",
    },
    NativeContractDescriptor {
        hash: [
            0x3b, 0xec, 0x35, 0x31, 0x11, 0x9b, 0xba, 0xd7, 0x6d, 0xd0, 0x44, 0x92, 0x0b, 0x0d,
            0xe6, 0xc3, 0x19, 0x4f, 0xe1, 0xc1,
        ],
        name: "Notary",
    },
    NativeContractDescriptor {
        hash: [
            0xc1, 0x3a, 0x56, 0xc9, 0x83, 0x53, 0xa7, 0xea, 0x6a, 0x32, 0x4d, 0x9a, 0x83, 0x5d,
            0x1b, 0x5b, 0xf2, 0x26, 0x63, 0x15,
        ],
        name: "Treasury",
    },
    NativeContractDescriptor {
        hash: [
            0xbe, 0xf2, 0x04, 0x31, 0x40, 0x36, 0x2a, 0x77, 0xc1, 0x50, 0x99, 0xc7, 0xe6, 0x4c,
            0x12, 0xf7, 0x00, 0xb6, 0x65, 0xda,
        ],
        name: "LedgerContract",
    },
    NativeContractDescriptor {
        hash: [
            0x1b, 0xf5, 0x75, 0xab, 0x11, 0x89, 0x68, 0x84, 0x13, 0x61, 0x0a, 0x35, 0xa1, 0x28,
            0x86, 0xcd, 0xe0, 0xb6, 0x6c, 0x72,
        ],
        name: "CryptoLib",
    },
    NativeContractDescriptor {
        hash: [
            0xc0, 0xef, 0x39, 0xce, 0xe0, 0xe4, 0xe9, 0x25, 0xc6, 0xc2, 0xa0, 0x6a, 0x79, 0xe1,
            0x44, 0x0d, 0xd8, 0x6f, 0xce, 0xac,
        ],
        name: "StdLib",
    },
];

pub(crate) fn emit_throw_with_message(instructions: &mut Vec<Instruction>, message: &str) {
    instructions.push(Instruction::PushLiteral(LiteralValue::String(
        message.as_bytes().to_vec(),
    )));
    instructions.push(Instruction::Throw);
}

pub(crate) fn emit_is_native_contract_check(
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
    contract_slot: usize,
) {
    let done_label = ctx.next_label();

    for contract in NATIVE_CONTRACTS.iter() {
        let next_label = ctx.next_label();
        instructions.push(Instruction::LoadLocal(contract_slot));
        instructions.push(Instruction::PushLiteral(LiteralValue::Address(
            contract.hash.to_vec(),
        )));
        instructions.push(Instruction::BinaryOp(BinaryOperator::Eq));
        instructions.push(Instruction::JumpIf { target: next_label });
        instructions.push(Instruction::PushLiteral(LiteralValue::Boolean(true)));
        instructions.push(Instruction::Jump { target: done_label });
        instructions.push(Instruction::Label(next_label));
    }

    instructions.push(Instruction::PushLiteral(LiteralValue::Boolean(false)));
    instructions.push(Instruction::Label(done_label));
}

pub(crate) fn emit_native_contract_name(
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
    contract_slot: usize,
) {
    let done_label = ctx.next_label();

    for contract in NATIVE_CONTRACTS.iter() {
        let next_label = ctx.next_label();
        instructions.push(Instruction::LoadLocal(contract_slot));
        instructions.push(Instruction::PushLiteral(LiteralValue::Address(
            contract.hash.to_vec(),
        )));
        instructions.push(Instruction::BinaryOp(BinaryOperator::Eq));
        instructions.push(Instruction::JumpIf { target: next_label });
        instructions.push(Instruction::PushLiteral(LiteralValue::String(
            contract.name.as_bytes().to_vec(),
        )));
        instructions.push(Instruction::Jump { target: done_label });
        instructions.push(Instruction::Label(next_label));
    }

    instructions.push(Instruction::PushLiteral(LiteralValue::String(
        b"Unknown".to_vec(),
    )));
    instructions.push(Instruction::Label(done_label));
}

pub(crate) fn try_lower_nativecalls_member_builtin(
    base: &Identifier,
    member: &Identifier,
    args: &[Expression],
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) -> Option<bool> {
    if base.name != "NativeCalls" {
        return None;
    }

    match member.name.as_str() {
        "getCommittee" => {
            if let Some(false) = check_arg_count(ctx, "getCommittee", args, 0) {
                return Some(false);
            }

            // Neo native contract exposes committee members as ECPoint public keys.
            // The NativeCalls devpack returns `address[]`, so convert to UInt160 standard
            // accounts via System.Contract.CreateStandardAccount.
            let tmp_id = ctx.next_label();
            let committee_keys_slot = ctx.allocate_local(
                format!("__native_calls_committee_keys_{tmp_id}"),
                Some(ValueType::Any),
            );
            let committee_addrs_slot = ctx.allocate_local(
                format!("__native_calls_committee_addrs_{tmp_id}"),
                Some(ValueType::Array(Box::new(ValueType::Address))),
            );
            let index_slot = ctx.allocate_local(
                format!("__native_calls_committee_index_{tmp_id}"),
                Some(ValueType::Integer {
                    signed: false,
                    bits: 256,
                }),
            );

            instructions.push(Instruction::CallBuiltin {
                builtin: BuiltinCall::NativeCall {
                    contract: NativeContract::Neo,
                    method: "getCommittee".to_string(),
                },
                arg_count: 0,
            });
            instructions.push(Instruction::StoreLocal(committee_keys_slot));

            instructions.push(Instruction::LoadLocal(committee_keys_slot));
            instructions.push(Instruction::GetSize);
            instructions.push(Instruction::NewArray {
                element_type: ValueType::Address,
            });
            instructions.push(Instruction::StoreLocal(committee_addrs_slot));

            instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                BigInt::zero(),
            )));
            instructions.push(Instruction::StoreLocal(index_slot));

            let loop_label = ctx.next_label();
            let done_label = ctx.next_label();
            instructions.push(Instruction::Label(loop_label));
            instructions.push(Instruction::LoadLocal(index_slot));
            instructions.push(Instruction::LoadLocal(committee_keys_slot));
            instructions.push(Instruction::GetSize);
            instructions.push(Instruction::BinaryOp(BinaryOperator::Lt));
            instructions.push(Instruction::JumpIf { target: done_label });

            // committee_addrs[index] = CreateStandardAccount(committee_keys[index])
            instructions.push(Instruction::LoadLocal(committee_addrs_slot));
            instructions.push(Instruction::LoadLocal(index_slot));
            instructions.push(Instruction::LoadLocal(committee_keys_slot));
            instructions.push(Instruction::LoadLocal(index_slot));
            instructions.push(Instruction::ArrayGet);
            instructions.push(Instruction::CallBuiltin {
                builtin: BuiltinCall::Syscall("System.Contract.CreateStandardAccount".to_string()),
                arg_count: 1,
            });
            instructions.push(Instruction::ArraySet);

            instructions.push(Instruction::LoadLocal(index_slot));
            instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                BigInt::one(),
            )));
            instructions.push(Instruction::BinaryOp(BinaryOperator::Add));
            instructions.push(Instruction::StoreLocal(index_slot));
            instructions.push(Instruction::Jump { target: loop_label });

            instructions.push(Instruction::Label(done_label));
            instructions.push(Instruction::LoadLocal(committee_addrs_slot));
            Some(true)
        }
        "isCommittee" => {
            if let Some(false) = check_arg_count(ctx, "isCommittee", args, 1) {
                return Some(false);
            }

            // Neo native contract exposes committee members as ECPoint public keys. Derive
            // standard accounts via System.Contract.CreateStandardAccount and compare with
            // the supplied address.
            let tmp_id = ctx.next_label();
            let account_slot = ctx.allocate_local(
                format!("__native_calls_is_committee_account_{tmp_id}"),
                Some(ValueType::Address),
            );
            let committee_slot = ctx.allocate_local(
                format!("__native_calls_is_committee_committee_{tmp_id}"),
                Some(ValueType::Any),
            );
            let index_slot = ctx.allocate_local(
                format!("__native_calls_is_committee_index_{tmp_id}"),
                Some(ValueType::Integer {
                    signed: false,
                    bits: 256,
                }),
            );

            if !lower_expression(&args[0], ctx, instructions) {
                return Some(false);
            }
            instructions.push(Instruction::StoreLocal(account_slot));

            instructions.push(Instruction::CallBuiltin {
                builtin: BuiltinCall::NativeCall {
                    contract: NativeContract::Neo,
                    method: "getCommittee".to_string(),
                },
                arg_count: 0,
            });
            instructions.push(Instruction::StoreLocal(committee_slot));

            instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                BigInt::zero(),
            )));
            instructions.push(Instruction::StoreLocal(index_slot));

            let loop_label = ctx.next_label();
            let advance_label = ctx.next_label();
            let done_label = ctx.next_label();
            let end_label = ctx.next_label();

            instructions.push(Instruction::Label(loop_label));
            instructions.push(Instruction::LoadLocal(index_slot));
            instructions.push(Instruction::LoadLocal(committee_slot));
            instructions.push(Instruction::GetSize);
            instructions.push(Instruction::BinaryOp(BinaryOperator::Lt));
            instructions.push(Instruction::JumpIf { target: done_label });

            instructions.push(Instruction::LoadLocal(committee_slot));
            instructions.push(Instruction::LoadLocal(index_slot));
            instructions.push(Instruction::ArrayGet);
            instructions.push(Instruction::CallBuiltin {
                builtin: BuiltinCall::Syscall("System.Contract.CreateStandardAccount".to_string()),
                arg_count: 1,
            });
            instructions.push(Instruction::LoadLocal(account_slot));
            instructions.push(Instruction::BinaryOp(BinaryOperator::Eq));
            instructions.push(Instruction::JumpIf {
                target: advance_label,
            });

            instructions.push(Instruction::PushLiteral(LiteralValue::Boolean(true)));
            instructions.push(Instruction::Jump { target: end_label });

            instructions.push(Instruction::Label(advance_label));
            instructions.push(Instruction::LoadLocal(index_slot));
            instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                BigInt::one(),
            )));
            instructions.push(Instruction::BinaryOp(BinaryOperator::Add));
            instructions.push(Instruction::StoreLocal(index_slot));
            instructions.push(Instruction::Jump { target: loop_label });

            instructions.push(Instruction::Label(done_label));
            instructions.push(Instruction::PushLiteral(LiteralValue::Boolean(false)));
            instructions.push(Instruction::Label(end_label));
            Some(true)
        }
        "getNextBlockValidators" => {
            if let Some(false) = check_arg_count(ctx, "getNextBlockValidators", args, 0) {
                return Some(false);
            }

            // Neo native contract exposes validators as ECPoint public keys.
            // The NativeCalls devpack returns `address[]`, so convert to UInt160 standard
            // accounts via System.Contract.CreateStandardAccount.
            let tmp_id = ctx.next_label();
            let validator_keys_slot = ctx.allocate_local(
                format!("__native_calls_validator_keys_{tmp_id}"),
                Some(ValueType::Any),
            );
            let validator_addrs_slot = ctx.allocate_local(
                format!("__native_calls_validator_addrs_{tmp_id}"),
                Some(ValueType::Array(Box::new(ValueType::Address))),
            );
            let index_slot = ctx.allocate_local(
                format!("__native_calls_validator_index_{tmp_id}"),
                Some(ValueType::Integer {
                    signed: false,
                    bits: 256,
                }),
            );

            instructions.push(Instruction::CallBuiltin {
                builtin: BuiltinCall::NativeCall {
                    contract: NativeContract::Neo,
                    method: "getNextBlockValidators".to_string(),
                },
                arg_count: 0,
            });
            instructions.push(Instruction::StoreLocal(validator_keys_slot));

            instructions.push(Instruction::LoadLocal(validator_keys_slot));
            instructions.push(Instruction::GetSize);
            instructions.push(Instruction::NewArray {
                element_type: ValueType::Address,
            });
            instructions.push(Instruction::StoreLocal(validator_addrs_slot));

            instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                BigInt::zero(),
            )));
            instructions.push(Instruction::StoreLocal(index_slot));

            let loop_label = ctx.next_label();
            let done_label = ctx.next_label();
            instructions.push(Instruction::Label(loop_label));
            instructions.push(Instruction::LoadLocal(index_slot));
            instructions.push(Instruction::LoadLocal(validator_keys_slot));
            instructions.push(Instruction::GetSize);
            instructions.push(Instruction::BinaryOp(BinaryOperator::Lt));
            instructions.push(Instruction::JumpIf { target: done_label });

            // validator_addrs[index] = CreateStandardAccount(validator_keys[index])
            instructions.push(Instruction::LoadLocal(validator_addrs_slot));
            instructions.push(Instruction::LoadLocal(index_slot));
            instructions.push(Instruction::LoadLocal(validator_keys_slot));
            instructions.push(Instruction::LoadLocal(index_slot));
            instructions.push(Instruction::ArrayGet);
            instructions.push(Instruction::CallBuiltin {
                builtin: BuiltinCall::Syscall("System.Contract.CreateStandardAccount".to_string()),
                arg_count: 1,
            });
            instructions.push(Instruction::ArraySet);

            instructions.push(Instruction::LoadLocal(index_slot));
            instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                BigInt::one(),
            )));
            instructions.push(Instruction::BinaryOp(BinaryOperator::Add));
            instructions.push(Instruction::StoreLocal(index_slot));
            instructions.push(Instruction::Jump { target: loop_label });

            instructions.push(Instruction::Label(done_label));
            instructions.push(Instruction::LoadLocal(validator_addrs_slot));
            Some(true)
        }
        "isValidator" => {
            if let Some(false) = check_arg_count(ctx, "isValidator", args, 1) {
                return Some(false);
            }

            let tmp_id = ctx.next_label();
            let account_slot = ctx.allocate_local(
                format!("__native_calls_is_validator_account_{tmp_id}"),
                Some(ValueType::Address),
            );
            let validators_slot = ctx.allocate_local(
                format!("__native_calls_is_validator_keys_{tmp_id}"),
                Some(ValueType::Any),
            );
            let index_slot = ctx.allocate_local(
                format!("__native_calls_is_validator_index_{tmp_id}"),
                Some(ValueType::Integer {
                    signed: false,
                    bits: 256,
                }),
            );

            if !lower_expression(&args[0], ctx, instructions) {
                return Some(false);
            }
            instructions.push(Instruction::StoreLocal(account_slot));

            instructions.push(Instruction::CallBuiltin {
                builtin: BuiltinCall::NativeCall {
                    contract: NativeContract::Neo,
                    method: "getNextBlockValidators".to_string(),
                },
                arg_count: 0,
            });
            instructions.push(Instruction::StoreLocal(validators_slot));

            instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                BigInt::zero(),
            )));
            instructions.push(Instruction::StoreLocal(index_slot));

            let loop_label = ctx.next_label();
            let advance_label = ctx.next_label();
            let done_label = ctx.next_label();
            let end_label = ctx.next_label();

            instructions.push(Instruction::Label(loop_label));
            instructions.push(Instruction::LoadLocal(index_slot));
            instructions.push(Instruction::LoadLocal(validators_slot));
            instructions.push(Instruction::GetSize);
            instructions.push(Instruction::BinaryOp(BinaryOperator::Lt));
            instructions.push(Instruction::JumpIf { target: done_label });

            instructions.push(Instruction::LoadLocal(validators_slot));
            instructions.push(Instruction::LoadLocal(index_slot));
            instructions.push(Instruction::ArrayGet);
            instructions.push(Instruction::CallBuiltin {
                builtin: BuiltinCall::Syscall("System.Contract.CreateStandardAccount".to_string()),
                arg_count: 1,
            });
            instructions.push(Instruction::LoadLocal(account_slot));
            instructions.push(Instruction::BinaryOp(BinaryOperator::Eq));
            instructions.push(Instruction::JumpIf {
                target: advance_label,
            });

            instructions.push(Instruction::PushLiteral(LiteralValue::Boolean(true)));
            instructions.push(Instruction::Jump { target: end_label });

            instructions.push(Instruction::Label(advance_label));
            instructions.push(Instruction::LoadLocal(index_slot));
            instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                BigInt::one(),
            )));
            instructions.push(Instruction::BinaryOp(BinaryOperator::Add));
            instructions.push(Instruction::StoreLocal(index_slot));
            instructions.push(Instruction::Jump { target: loop_label });

            instructions.push(Instruction::Label(done_label));
            instructions.push(Instruction::PushLiteral(LiteralValue::Boolean(false)));
            instructions.push(Instruction::Label(end_label));
            Some(true)
        }
        "isNativeContract" => {
            if let Some(false) = check_arg_count(ctx, "isNativeContract", args, 1) {
                return Some(false);
            }

            let tmp_id = ctx.next_label();
            let contract_slot = ctx.allocate_local(
                format!("__native_calls_is_native_contract_{tmp_id}"),
                Some(ValueType::Address),
            );

            if !lower_expression(&args[0], ctx, instructions) {
                return Some(false);
            }
            instructions.push(Instruction::StoreLocal(contract_slot));

            emit_is_native_contract_check(ctx, instructions, contract_slot);
            Some(true)
        }
        "getNativeContractName" => {
            if let Some(false) = check_arg_count(ctx, "getNativeContractName", args, 1) {
                return Some(false);
            }

            let tmp_id = ctx.next_label();
            let contract_slot = ctx.allocate_local(
                format!("__native_calls_contract_name_{tmp_id}"),
                Some(ValueType::Address),
            );

            if !lower_expression(&args[0], ctx, instructions) {
                return Some(false);
            }
            instructions.push(Instruction::StoreLocal(contract_slot));

            emit_native_contract_name(ctx, instructions, contract_slot);
            Some(true)
        }
        "getAllNativeContracts" => {
            if let Some(false) = check_arg_count(ctx, "getAllNativeContracts", args, 0) {
                return Some(false);
            }

            let tmp_id = ctx.next_label();
            let contracts_slot = ctx.allocate_local(
                format!("__native_calls_all_contracts_{tmp_id}"),
                Some(ValueType::Array(Box::new(ValueType::Address))),
            );

            instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                BigInt::from(NATIVE_CONTRACTS.len() as u64),
            )));
            instructions.push(Instruction::NewArray {
                element_type: ValueType::Address,
            });
            instructions.push(Instruction::StoreLocal(contracts_slot));

            for (index, contract) in NATIVE_CONTRACTS.iter().enumerate() {
                instructions.push(Instruction::LoadLocal(contracts_slot));
                instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                    BigInt::from(index as u64),
                )));
                instructions.push(Instruction::PushLiteral(LiteralValue::Address(
                    contract.hash.to_vec(),
                )));
                instructions.push(Instruction::ArraySet);
            }

            instructions.push(Instruction::LoadLocal(contracts_slot));
            Some(true)
        }
        "estimateNativeCallGas" => {
            if let Some(false) = check_arg_count(ctx, "estimateNativeCallGas", args, 3) {
                return Some(false);
            }

            let tmp_id = ctx.next_label();
            let contract_slot = ctx.allocate_local(
                format!("__native_calls_estimate_contract_{tmp_id}"),
                Some(ValueType::Address),
            );
            let method_slot = ctx.allocate_local(
                format!("__native_calls_estimate_method_{tmp_id}"),
                Some(ValueType::String),
            );
            let params_slot = ctx.allocate_local(
                format!("__native_calls_estimate_params_{tmp_id}"),
                Some(ValueType::ByteArray { fixed_len: None }),
            );
            let result_slot = ctx.allocate_local(
                format!("__native_calls_estimate_result_{tmp_id}"),
                Some(ValueType::Integer {
                    signed: false,
                    bits: 256,
                }),
            );

            if !lower_expression(&args[0], ctx, instructions) {
                return Some(false);
            }
            instructions.push(Instruction::StoreLocal(contract_slot));

            if !lower_expression(&args[1], ctx, instructions) {
                return Some(false);
            }
            instructions.push(Instruction::StoreLocal(method_slot));

            if !lower_expression(&args[2], ctx, instructions) {
                return Some(false);
            }
            instructions.push(Instruction::StoreLocal(params_slot));

            instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                BigInt::from(1_000_000u64),
            )));
            instructions.push(Instruction::StoreLocal(result_slot));

            let end_label = ctx.next_label();

            // NEO contract heuristics
            let neo_skip_label = ctx.next_label();
            let neo_register_label = ctx.next_label();
            instructions.push(Instruction::LoadLocal(contract_slot));
            instructions.push(Instruction::PushLiteral(LiteralValue::Address(
                NATIVE_CONTRACTS[0].hash.to_vec(),
            )));
            instructions.push(Instruction::BinaryOp(BinaryOperator::Eq));
            instructions.push(Instruction::JumpIf {
                target: neo_skip_label,
            });

            instructions.push(Instruction::LoadLocal(method_slot));
            instructions.push(Instruction::PushLiteral(LiteralValue::String(
                b"vote".to_vec(),
            )));
            instructions.push(Instruction::BinaryOp(BinaryOperator::Eq));
            instructions.push(Instruction::JumpIf {
                target: neo_register_label,
            });
            instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                BigInt::from(100_000_000u64),
            )));
            instructions.push(Instruction::StoreLocal(result_slot));
            instructions.push(Instruction::Jump { target: end_label });

            instructions.push(Instruction::Label(neo_register_label));
            instructions.push(Instruction::LoadLocal(method_slot));
            instructions.push(Instruction::PushLiteral(LiteralValue::String(
                b"registerCandidate".to_vec(),
            )));
            instructions.push(Instruction::BinaryOp(BinaryOperator::Eq));
            instructions.push(Instruction::JumpIf {
                target: neo_skip_label,
            });
            instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                BigInt::from(1_000_000_000u64),
            )));
            instructions.push(Instruction::StoreLocal(result_slot));
            instructions.push(Instruction::Jump { target: end_label });

            instructions.push(Instruction::Label(neo_skip_label));

            // ContractManagement heuristics
            let cm_skip_label = ctx.next_label();
            let cm_update_label = ctx.next_label();
            instructions.push(Instruction::LoadLocal(contract_slot));
            instructions.push(Instruction::PushLiteral(LiteralValue::Address(
                NATIVE_CONTRACTS[2].hash.to_vec(),
            )));
            instructions.push(Instruction::BinaryOp(BinaryOperator::Eq));
            instructions.push(Instruction::JumpIf {
                target: cm_skip_label,
            });

            instructions.push(Instruction::LoadLocal(method_slot));
            instructions.push(Instruction::PushLiteral(LiteralValue::String(
                b"deploy".to_vec(),
            )));
            instructions.push(Instruction::BinaryOp(BinaryOperator::Eq));
            instructions.push(Instruction::JumpIf {
                target: cm_update_label,
            });
            instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                BigInt::from(500_000_000u64),
            )));
            instructions.push(Instruction::StoreLocal(result_slot));
            instructions.push(Instruction::Jump { target: end_label });

            instructions.push(Instruction::Label(cm_update_label));
            instructions.push(Instruction::LoadLocal(method_slot));
            instructions.push(Instruction::PushLiteral(LiteralValue::String(
                b"update".to_vec(),
            )));
            instructions.push(Instruction::BinaryOp(BinaryOperator::Eq));
            instructions.push(Instruction::JumpIf {
                target: cm_skip_label,
            });
            instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                BigInt::from(300_000_000u64),
            )));
            instructions.push(Instruction::StoreLocal(result_slot));
            instructions.push(Instruction::Jump { target: end_label });

            instructions.push(Instruction::Label(cm_skip_label));

            // Oracle heuristics
            let oracle_skip_label = ctx.next_label();
            instructions.push(Instruction::LoadLocal(contract_slot));
            instructions.push(Instruction::PushLiteral(LiteralValue::Address(
                NATIVE_CONTRACTS[4].hash.to_vec(),
            )));
            instructions.push(Instruction::BinaryOp(BinaryOperator::Eq));
            instructions.push(Instruction::JumpIf {
                target: oracle_skip_label,
            });

            instructions.push(Instruction::LoadLocal(method_slot));
            instructions.push(Instruction::PushLiteral(LiteralValue::String(
                b"request".to_vec(),
            )));
            instructions.push(Instruction::BinaryOp(BinaryOperator::Eq));
            instructions.push(Instruction::JumpIf {
                target: oracle_skip_label,
            });
            instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                BigInt::from(50_000_000u64),
            )));
            instructions.push(Instruction::StoreLocal(result_slot));
            instructions.push(Instruction::Jump { target: end_label });

            instructions.push(Instruction::Label(oracle_skip_label));
            instructions.push(Instruction::Label(end_label));
            instructions.push(Instruction::LoadLocal(result_slot));
            Some(true)
        }
        "batchNativeCalls" => {
            if let Some(false) = check_arg_count(ctx, "batchNativeCalls", args, 3) {
                return Some(false);
            }

            let tmp_id = ctx.next_label();
            let contracts_slot = ctx.allocate_local(
                format!("__native_calls_batch_contracts_{tmp_id}"),
                Some(ValueType::Any),
            );
            let methods_slot = ctx.allocate_local(
                format!("__native_calls_batch_methods_{tmp_id}"),
                Some(ValueType::Any),
            );
            let params_slot = ctx.allocate_local(
                format!("__native_calls_batch_params_{tmp_id}"),
                Some(ValueType::Any),
            );
            let length_slot = ctx.allocate_local(
                format!("__native_calls_batch_length_{tmp_id}"),
                Some(ValueType::Integer {
                    signed: false,
                    bits: 256,
                }),
            );

            if !lower_expression(&args[0], ctx, instructions) {
                return Some(false);
            }
            instructions.push(Instruction::StoreLocal(contracts_slot));

            if !lower_expression(&args[1], ctx, instructions) {
                return Some(false);
            }
            instructions.push(Instruction::StoreLocal(methods_slot));

            if !lower_expression(&args[2], ctx, instructions) {
                return Some(false);
            }
            instructions.push(Instruction::StoreLocal(params_slot));

            instructions.push(Instruction::LoadLocal(contracts_slot));
            instructions.push(Instruction::GetSize);
            instructions.push(Instruction::StoreLocal(length_slot));

            // contracts.length == methods.length
            let methods_fail = ctx.next_label();
            let methods_ok = ctx.next_label();
            instructions.push(Instruction::LoadLocal(methods_slot));
            instructions.push(Instruction::GetSize);
            instructions.push(Instruction::LoadLocal(length_slot));
            instructions.push(Instruction::BinaryOp(BinaryOperator::Eq));
            instructions.push(Instruction::JumpIf {
                target: methods_fail,
            });
            instructions.push(Instruction::Jump { target: methods_ok });
            instructions.push(Instruction::Label(methods_fail));
            emit_throw_with_message(instructions, "NativeCalls: array length mismatch");
            instructions.push(Instruction::Label(methods_ok));

            // contracts.length == params.length
            let params_fail = ctx.next_label();
            let params_ok = ctx.next_label();
            instructions.push(Instruction::LoadLocal(params_slot));
            instructions.push(Instruction::GetSize);
            instructions.push(Instruction::LoadLocal(length_slot));
            instructions.push(Instruction::BinaryOp(BinaryOperator::Eq));
            instructions.push(Instruction::JumpIf {
                target: params_fail,
            });
            instructions.push(Instruction::Jump { target: params_ok });
            instructions.push(Instruction::Label(params_fail));
            emit_throw_with_message(instructions, "NativeCalls: array length mismatch");
            instructions.push(Instruction::Label(params_ok));

            // contracts.length > 0
            let non_empty_label = ctx.next_label();
            instructions.push(Instruction::LoadLocal(length_slot));
            instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                BigInt::zero(),
            )));
            instructions.push(Instruction::BinaryOp(BinaryOperator::Eq));
            instructions.push(Instruction::JumpIf {
                target: non_empty_label,
            });
            emit_throw_with_message(instructions, "NativeCalls: empty arrays");
            instructions.push(Instruction::Label(non_empty_label));

            // contracts.length <= 10
            let length_fail = ctx.next_label();
            let length_ok_label = ctx.next_label();
            instructions.push(Instruction::LoadLocal(length_slot));
            instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                BigInt::from(10u8),
            )));
            instructions.push(Instruction::BinaryOp(BinaryOperator::Le));
            instructions.push(Instruction::JumpIf {
                target: length_fail,
            });
            instructions.push(Instruction::Jump {
                target: length_ok_label,
            });
            instructions.push(Instruction::Label(length_fail));
            emit_throw_with_message(instructions, "NativeCalls: too many calls");
            instructions.push(Instruction::Label(length_ok_label));

            let results_slot = ctx.allocate_local(
                format!("__native_calls_batch_results_{tmp_id}"),
                Some(ValueType::Array(Box::new(ValueType::ByteArray {
                    fixed_len: None,
                }))),
            );
            let index_slot = ctx.allocate_local(
                format!("__native_calls_batch_index_{tmp_id}"),
                Some(ValueType::Integer {
                    signed: false,
                    bits: 256,
                }),
            );
            let contract_slot = ctx.allocate_local(
                format!("__native_calls_batch_contract_{tmp_id}"),
                Some(ValueType::Address),
            );

            instructions.push(Instruction::LoadLocal(length_slot));
            instructions.push(Instruction::NewArray {
                element_type: ValueType::ByteArray { fixed_len: None },
            });
            instructions.push(Instruction::StoreLocal(results_slot));

            instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                BigInt::zero(),
            )));
            instructions.push(Instruction::StoreLocal(index_slot));

            let loop_label = ctx.next_label();
            let done_label = ctx.next_label();
            instructions.push(Instruction::Label(loop_label));
            instructions.push(Instruction::LoadLocal(index_slot));
            instructions.push(Instruction::LoadLocal(length_slot));
            instructions.push(Instruction::BinaryOp(BinaryOperator::Lt));
            instructions.push(Instruction::JumpIf { target: done_label });

            // contract = contracts[index]
            instructions.push(Instruction::LoadLocal(contracts_slot));
            instructions.push(Instruction::LoadLocal(index_slot));
            instructions.push(Instruction::ArrayGet);
            instructions.push(Instruction::StoreLocal(contract_slot));

            // require isNativeContract(contract)
            let native_ok = ctx.next_label();
            emit_is_native_contract_check(ctx, instructions, contract_slot);
            instructions.push(Instruction::JumpIf { target: native_ok });
            emit_throw_with_message(instructions, "NativeCalls: not a native contract");
            instructions.push(Instruction::Label(native_ok));

            // results[index] = contractCall(contract, method, params)
            instructions.push(Instruction::LoadLocal(results_slot));
            instructions.push(Instruction::LoadLocal(index_slot));
            instructions.push(Instruction::LoadLocal(contract_slot));
            instructions.push(Instruction::LoadLocal(methods_slot));
            instructions.push(Instruction::LoadLocal(index_slot));
            instructions.push(Instruction::ArrayGet);
            instructions.push(Instruction::LoadLocal(params_slot));
            instructions.push(Instruction::LoadLocal(index_slot));
            instructions.push(Instruction::ArrayGet);

            if ctx.is_safe {
                instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                    BigInt::from(0x05u8),
                )));
                instructions.push(Instruction::CallBuiltin {
                    builtin: BuiltinCall::ContractCallWithFlags,
                    arg_count: 4,
                });
            } else {
                instructions.push(Instruction::CallBuiltin {
                    builtin: BuiltinCall::ContractCall,
                    arg_count: 3,
                });
            }

            instructions.push(Instruction::ArraySet);

            instructions.push(Instruction::LoadLocal(index_slot));
            instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                BigInt::one(),
            )));
            instructions.push(Instruction::BinaryOp(BinaryOperator::Add));
            instructions.push(Instruction::StoreLocal(index_slot));
            instructions.push(Instruction::Jump { target: loop_label });

            instructions.push(Instruction::Label(done_label));
            instructions.push(Instruction::LoadLocal(results_slot));
            Some(true)
        }
        "getContractById" => {
            if let Some(false) = check_arg_count(ctx, "getContractById", args, 1) {
                return Some(false);
            }

            let tmp_id = ctx.next_label();
            let state_slot = ctx.allocate_local(
                format!("__native_calls_contract_state_{tmp_id}"),
                Some(ValueType::Any),
            );
            let result_slot = ctx.allocate_local(
                format!("__native_calls_contract_state_result_{tmp_id}"),
                Some(ValueType::Array(Box::new(ValueType::Any))),
            );

            if !lower_expression(&args[0], ctx, instructions) {
                return Some(false);
            }

            instructions.push(Instruction::CallBuiltin {
                builtin: BuiltinCall::NativeCall {
                    contract: NativeContract::ContractManagement,
                    method: "getContractById".to_string(),
                },
                arg_count: 1,
            });
            instructions.push(Instruction::StoreLocal(state_slot));

            instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                BigInt::from(4u8),
            )));
            instructions.push(Instruction::NewArray {
                element_type: ValueType::Any,
            });
            instructions.push(Instruction::StoreLocal(result_slot));

            // hash (index 2)
            instructions.push(Instruction::LoadLocal(result_slot));
            instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                BigInt::zero(),
            )));
            instructions.push(Instruction::LoadLocal(state_slot));
            instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                BigInt::from(2u8),
            )));
            instructions.push(Instruction::ArrayGet);
            instructions.push(Instruction::ArraySet);

            // nef (index 3)
            instructions.push(Instruction::LoadLocal(result_slot));
            instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                BigInt::one(),
            )));
            instructions.push(Instruction::LoadLocal(state_slot));
            instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                BigInt::from(3u8),
            )));
            instructions.push(Instruction::ArrayGet);
            instructions.push(Instruction::ArraySet);

            // manifest (index 4) serialized
            instructions.push(Instruction::LoadLocal(result_slot));
            instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                BigInt::from(2u8),
            )));
            instructions.push(Instruction::LoadLocal(state_slot));
            instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                BigInt::from(4u8),
            )));
            instructions.push(Instruction::ArrayGet);
            instructions.push(Instruction::CallBuiltin {
                builtin: BuiltinCall::NativeCall {
                    contract: NativeContract::StdLib,
                    method: "serialize".to_string(),
                },
                arg_count: 1,
            });
            instructions.push(Instruction::ArraySet);

            // updateCounter (index 1)
            instructions.push(Instruction::LoadLocal(result_slot));
            instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                BigInt::from(3u8),
            )));
            instructions.push(Instruction::LoadLocal(state_slot));
            instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                BigInt::one(),
            )));
            instructions.push(Instruction::ArrayGet);
            instructions.push(Instruction::ArraySet);

            instructions.push(Instruction::LoadLocal(result_slot));
            Some(true)
        }
        "getNetworkConfiguration" => {
            if let Some(false) = check_arg_count(ctx, "getNetworkConfiguration", args, 0) {
                return Some(false);
            }

            let tmp_id = ctx.next_label();
            let config_slot = ctx.allocate_local(
                format!("__native_calls_network_config_{tmp_id}"),
                Some(ValueType::Array(Box::new(ValueType::Any))),
            );

            instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                BigInt::from(6u8),
            )));
            instructions.push(Instruction::NewArray {
                element_type: ValueType::Any,
            });
            instructions.push(Instruction::StoreLocal(config_slot));

            // feePerByte
            instructions.push(Instruction::LoadLocal(config_slot));
            instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                BigInt::zero(),
            )));
            instructions.push(Instruction::CallBuiltin {
                builtin: BuiltinCall::NativeCall {
                    contract: NativeContract::Policy,
                    method: "getFeePerByte".to_string(),
                },
                arg_count: 0,
            });
            instructions.push(Instruction::ArraySet);

            // execFeeFactor
            instructions.push(Instruction::LoadLocal(config_slot));
            instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                BigInt::one(),
            )));
            instructions.push(Instruction::CallBuiltin {
                builtin: BuiltinCall::NativeCall {
                    contract: NativeContract::Policy,
                    method: "getExecFeeFactor".to_string(),
                },
                arg_count: 0,
            });
            instructions.push(Instruction::ArraySet);

            // storagePrice
            instructions.push(Instruction::LoadLocal(config_slot));
            instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                BigInt::from(2u8),
            )));
            instructions.push(Instruction::CallBuiltin {
                builtin: BuiltinCall::NativeCall {
                    contract: NativeContract::Policy,
                    method: "getStoragePrice".to_string(),
                },
                arg_count: 0,
            });
            instructions.push(Instruction::ArraySet);

            // gasPerBlock
            instructions.push(Instruction::LoadLocal(config_slot));
            instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                BigInt::from(3u8),
            )));
            instructions.push(Instruction::CallBuiltin {
                builtin: BuiltinCall::NativeCall {
                    contract: NativeContract::Neo,
                    method: "getGasPerBlock".to_string(),
                },
                arg_count: 0,
            });
            instructions.push(Instruction::ArraySet);

            // oraclePrice
            instructions.push(Instruction::LoadLocal(config_slot));
            instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                BigInt::from(4u8),
            )));
            instructions.push(Instruction::CallBuiltin {
                builtin: BuiltinCall::NativeCall {
                    contract: NativeContract::Oracle,
                    method: "getPrice".to_string(),
                },
                arg_count: 0,
            });
            instructions.push(Instruction::ArraySet);

            // minimumDeploymentFee
            instructions.push(Instruction::LoadLocal(config_slot));
            instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                BigInt::from(5u8),
            )));
            instructions.push(Instruction::CallBuiltin {
                builtin: BuiltinCall::NativeCall {
                    contract: NativeContract::ContractManagement,
                    method: "getMinimumDeploymentFee".to_string(),
                },
                arg_count: 0,
            });
            instructions.push(Instruction::ArraySet);

            instructions.push(Instruction::LoadLocal(config_slot));
            Some(true)
        }
        "getNativeContractManifest" => {
            if let Some(false) = check_arg_count(ctx, "getNativeContractManifest", args, 1) {
                return Some(false);
            }

            if !lower_expression(&args[0], ctx, instructions) {
                return Some(false);
            }
            instructions.push(Instruction::CallBuiltin {
                builtin: BuiltinCall::GetContract,
                arg_count: 1,
            });
            instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                BigInt::from(2u8),
            )));
            instructions.push(Instruction::ArrayGet);
            Some(true)
        }
        "safeNativeCall" => {
            if let Some(false) = check_arg_count(ctx, "safeNativeCall", args, 3) {
                return Some(false);
            }

            let tmp_id = ctx.next_label();
            let contract_slot = ctx.allocate_local(
                format!("__native_calls_safe_contract_{tmp_id}"),
                Some(ValueType::Address),
            );
            let method_slot = ctx.allocate_local(
                format!("__native_calls_safe_method_{tmp_id}"),
                Some(ValueType::String),
            );
            let params_slot = ctx.allocate_local(
                format!("__native_calls_safe_params_{tmp_id}"),
                Some(ValueType::ByteArray { fixed_len: None }),
            );
            let data_slot = ctx.allocate_local(
                format!("__native_calls_safe_data_{tmp_id}"),
                Some(ValueType::ByteArray { fixed_len: None }),
            );
            let tuple_slot = ctx.allocate_local(
                format!("__native_calls_safe_tuple_{tmp_id}"),
                Some(ValueType::Any),
            );

            if !lower_expression(&args[0], ctx, instructions) {
                return Some(false);
            }
            instructions.push(Instruction::StoreLocal(contract_slot));

            if !lower_expression(&args[1], ctx, instructions) {
                return Some(false);
            }
            instructions.push(Instruction::StoreLocal(method_slot));

            if !lower_expression(&args[2], ctx, instructions) {
                return Some(false);
            }
            instructions.push(Instruction::StoreLocal(params_slot));

            // require(isNativeContract(contract))
            let native_ok = ctx.next_label();
            emit_is_native_contract_check(ctx, instructions, contract_slot);
            instructions.push(Instruction::JumpIf { target: native_ok });
            emit_throw_with_message(instructions, "NativeCalls: not a native contract");
            instructions.push(Instruction::Label(native_ok));

            instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                BigInt::from(2u8),
            )));
            instructions.push(Instruction::NewArray {
                element_type: ValueType::Any,
            });
            instructions.push(Instruction::StoreLocal(tuple_slot));

            let catch_label = ctx.next_label();
            let end_label = ctx.next_label();
            instructions.push(Instruction::Try {
                catch_target: catch_label,
            });

            instructions.push(Instruction::LoadLocal(contract_slot));
            instructions.push(Instruction::LoadLocal(method_slot));
            instructions.push(Instruction::LoadLocal(params_slot));

            if ctx.is_safe {
                instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                    BigInt::from(0x05u8),
                )));
                instructions.push(Instruction::CallBuiltin {
                    builtin: BuiltinCall::ContractCallWithFlags,
                    arg_count: 4,
                });
            } else {
                instructions.push(Instruction::CallBuiltin {
                    builtin: BuiltinCall::ContractCall,
                    arg_count: 3,
                });
            }

            instructions.push(Instruction::StoreLocal(data_slot));

            instructions.push(Instruction::LoadLocal(tuple_slot));
            instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                BigInt::zero(),
            )));
            instructions.push(Instruction::PushLiteral(LiteralValue::Boolean(true)));
            instructions.push(Instruction::ArraySet);

            instructions.push(Instruction::LoadLocal(tuple_slot));
            instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                BigInt::one(),
            )));
            instructions.push(Instruction::LoadLocal(data_slot));
            instructions.push(Instruction::ArraySet);

            instructions.push(Instruction::EndTry { target: end_label });

            instructions.push(Instruction::Label(catch_label));
            instructions.push(Instruction::Drop(ValueType::Any));

            instructions.push(Instruction::LoadLocal(tuple_slot));
            instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                BigInt::zero(),
            )));
            instructions.push(Instruction::PushLiteral(LiteralValue::Boolean(false)));
            instructions.push(Instruction::ArraySet);

            instructions.push(Instruction::LoadLocal(tuple_slot));
            instructions.push(Instruction::PushLiteral(LiteralValue::Integer(
                BigInt::one(),
            )));
            instructions.push(Instruction::PushLiteral(
                LiteralValue::ByteArray(Vec::new()),
            ));
            instructions.push(Instruction::ArraySet);

            instructions.push(Instruction::EndTry { target: end_label });
            instructions.push(Instruction::Label(end_label));
            instructions.push(Instruction::LoadLocal(tuple_slot));
            Some(true)
        }
        _ => None,
    }
}
