fn analyze_contract_calls(function: &ir::Function) -> Vec<ContractCallRequirement> {
    let mut has_contract_calls = false;
    for block in &function.basic_blocks {
        for instr in &block.instructions {
            if matches!(
                instr,
                ir::Instruction::CallBuiltin {
                    builtin: ir::BuiltinCall::ContractCall
                        | ir::BuiltinCall::ContractCallWithFlags,
                    ..
                }
            ) {
                has_contract_calls = true;
                break;
            }
            if let ir::Instruction::CallBuiltin {
                builtin: ir::BuiltinCall::Syscall(name),
                ..
            } = instr
            {
                if name == "System.Contract.Call" {
                    has_contract_calls = true;
                    break;
                }
            }
        }
        if has_contract_calls {
            break;
        }
    }

    if !has_contract_calls {
        return Vec::new();
    }

    let mut requirements = Vec::new();

    for block in &function.basic_blocks {
        // Conservative, block-local abstract interpretation.
        //
        // This avoids brittle full-CFG merging (which can fail around complex
        // control-flow / exception constructs) while still extracting literal
        // contract + method values at call sites that are built in the same block.
        let mut state = AbstractState::new(function.local_count);

        for instr in &block.instructions {
            if let ir::Instruction::CallBuiltin { builtin, arg_count } = instr {
                let (contract_from_end, method_from_end, expected_args) = match builtin {
                    ir::BuiltinCall::ContractCall => (3usize, 2usize, 3usize),
                    ir::BuiltinCall::ContractCallWithFlags => (4usize, 3usize, 4usize),
                    ir::BuiltinCall::Syscall(name) if name == "System.Contract.Call" => {
                        // Syscall stack order: [args, flags, method, hash]
                        (1usize, 2usize, 4usize)
                    }
                    _ => (0usize, 0usize, 0usize),
                };

                if expected_args > 0
                    && *arg_count == expected_args
                    && state.stack.len() >= expected_args
                {
                    let stack_len = state.stack.len();
                    let contract_value = &state.stack[stack_len - contract_from_end];
                    if matches!(contract_value, AbstractValue::ExecutingScriptHash) {
                        // Neo N3 always allows contracts to call themselves, but the contract
                        // script hash is not known at compile time (it depends on the deployer).
                        // Do not emit wildcard permissions for self-calls.
                        //
                        // Note: native contract calls are still tracked separately and will
                        // emit explicit permissions as required.
                        continue;
                    }

                    let contract = match contract_value {
                        AbstractValue::Literal(lit) => descriptor_from_literal(lit),
                        AbstractValue::ExecutingScriptHash | AbstractValue::Unknown => None,
                    };
                    let method = match &state.stack[stack_len - method_from_end] {
                        AbstractValue::Literal(lit) => method_name_from_literal(lit),
                        AbstractValue::ExecutingScriptHash => None,
                        AbstractValue::Unknown => None,
                    };
                    requirements.push(ContractCallRequirement { contract, method });
                } else if expected_args > 0 {
                    // If the stack model is out of sync, fall back to a fully dynamic
                    // requirement (safe over-approximation).
                    requirements.push(ContractCallRequirement {
                        contract: None,
                        method: None,
                    });
                }
            }

            if apply_instruction(&mut state, instr).is_err() {
                // Keep analysis progressing even when the stack model diverges, but do not
                // attempt to preserve literal information after an error.
                state.stack.clear();
                for slot in &mut state.locals {
                    *slot = AbstractValue::Unknown;
                }
            }
        }
    }

    requirements
}
