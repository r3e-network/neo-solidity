fn try_lower_runtime_member_access(
    inner: &Expression,
    member: &Identifier,
    ctx: &mut LoweringContext,
    instructions: &mut Vec<Instruction>,
) -> Option<bool> {
    match member.name.as_str() {
        "sender" => {
            if let Expression::Variable(base) = inner {
                if base.name == "msg" {
                    if ctx.function_name == "onNEP17Payment" {
                        instructions.push(Instruction::LoadParameter(0));
                    } else {
                        instructions
                            .push(Instruction::LoadRuntimeValue(RuntimeValue::MsgSender));
                    }
                    return Some(true);
                }
            }
            None
        }
        "value" => {
            if let Expression::Variable(base) = inner {
                if base.name == "msg" {
                    if ctx.function_name == "onNEP17Payment" {
                        instructions.push(Instruction::LoadParameter(1));
                    } else {
                        instructions.push(Instruction::LoadRuntimeValue(RuntimeValue::MsgValue));
                    }
                    return Some(true);
                }
            }
            None
        }
        "data" => {
            if let Expression::Variable(base) = inner {
                if base.name == "msg" {
                    if ctx.function_name == "onNEP17Payment" {
                        instructions.push(Instruction::LoadParameter(2));
                    } else {
                        instructions.push(Instruction::LoadRuntimeValue(RuntimeValue::MsgData));
                    }
                    return Some(true);
                }
            }
            None
        }
        "sig" => {
            if let Expression::Variable(base) = inner {
                if base.name == "msg" {
                    ctx.record_error_with_suggestion(
                        "msg.sig is not available on Neo N3. Neo dispatches by method name, not 4-byte function selectors",
                        "use string-based method identification instead",
                    );
                    return Some(false);
                }
            }
            None
        }
        "origin" => {
            if let Expression::Variable(base) = inner {
                if base.name == "tx" {
                    // Non-fatal warning: tx.origin compiles but has different semantics on Neo.
                    eprintln!(
                        "warning: tx.origin has different semantics on Neo N3. \
                         Neo uses multi-signature witnesses instead of a single origin. \
                         Consider using msg.sender or Runtime.CheckWitness() instead."
                    );
                    instructions.push(Instruction::LoadRuntimeValue(RuntimeValue::TxOrigin));
                    return Some(true);
                }
            }
            None
        }
        "gasprice" => {
            if let Expression::Variable(base) = inner {
                if base.name == "tx" {
                    // Neo N3 auto-compat: tx.gasprice → Policy.getFeePerByte()
                    eprintln!(
                        "warning: tx.gasprice auto-mapped to Policy.getFeePerByte() \
                         on Neo N3. Neo fees are determined by script size and syscall costs."
                    );
                    instructions.push(Instruction::CallBuiltin {
                        builtin: BuiltinCall::NativeCall {
                            contract: NativeContract::Policy,
                            method: "getFeePerByte".to_string(),
                        },
                        arg_count: 0,
                    });
                    return Some(true);
                }
            }
            None
        }
        "timestamp" => {
            if let Expression::Variable(base) = inner {
                if base.name == "block" {
                    instructions.push(Instruction::LoadRuntimeValue(RuntimeValue::BlockTimestamp));
                    return Some(true);
                }
            }
            None
        }
        "number" => {
            if let Expression::Variable(base) = inner {
                if base.name == "block" {
                    instructions.push(Instruction::LoadRuntimeValue(RuntimeValue::BlockNumber));
                    return Some(true);
                }
            }
            None
        }
        "chainid" => {
            if let Expression::Variable(base) = inner {
                if base.name == "block" {
                    // Solidity `block.chainid` is a uint256 chain identifier. Neo N3 exposes a
                    // network "magic" number via `System.Runtime.GetNetwork`; use that as the
                    // closest equivalent.
                    instructions.push(Instruction::CallBuiltin {
                        builtin: BuiltinCall::Syscall("System.Runtime.GetNetwork".to_string()),
                        arg_count: 0,
                    });
                    return Some(true);
                }
            }
            None
        }
        "coinbase" => {
            if let Expression::Variable(base) = inner {
                if base.name == "block" {
                    // Neo N3 auto-compat: block.coinbase → address(0)
                    // dBFT has no miner; return zero address as safe default.
                    eprintln!(
                        "warning: block.coinbase auto-mapped to address(0) on Neo N3 \
                         (dBFT consensus has no miner). Use NativeCalls.getNextBlockValidators() \
                         for validator info."
                    );
                    instructions.push(Instruction::PushLiteral(LiteralValue::Address(
                        [0u8; 20].to_vec(),
                    )));
                    return Some(true);
                }
            }
            None
        }
        "difficulty" | "prevrandao" => {
            if let Expression::Variable(base) = inner {
                if base.name == "block" {
                    // Neo N3 auto-compat: block.difficulty/prevrandao → Runtime.getRandom()
                    eprintln!(
                        "warning: block.{} auto-mapped to Runtime.getRandom() on Neo N3 \
                         (dBFT consensus has no PoW difficulty).",
                        member.name
                    );
                    instructions.push(Instruction::CallBuiltin {
                        builtin: BuiltinCall::Syscall(
                            "System.Runtime.GetRandom".to_string(),
                        ),
                        arg_count: 0,
                    });
                    return Some(true);
                }
            }
            None
        }
        "gaslimit" => {
            if let Expression::Variable(base) = inner {
                if base.name == "block" {
                    // Neo N3 auto-compat: block.gaslimit → Policy.getExecFeeFactor()
                    eprintln!(
                        "warning: block.gaslimit auto-mapped to Policy.getExecFeeFactor() \
                         on Neo N3. Neo uses GAS token for fees, not per-block gas limits."
                    );
                    instructions.push(Instruction::CallBuiltin {
                        builtin: BuiltinCall::NativeCall {
                            contract: NativeContract::Policy,
                            method: "getExecFeeFactor".to_string(),
                        },
                        arg_count: 0,
                    });
                    return Some(true);
                }
            }
            None
        }
        "basefee" => {
            if let Expression::Variable(base) = inner {
                if base.name == "block" {
                    // Neo N3 auto-compat: block.basefee → Policy.getFeePerByte()
                    eprintln!(
                        "warning: block.basefee auto-mapped to Policy.getFeePerByte() \
                         on Neo N3. Neo uses a fixed fee structure, not EIP-1559 base fees."
                    );
                    instructions.push(Instruction::CallBuiltin {
                        builtin: BuiltinCall::NativeCall {
                            contract: NativeContract::Policy,
                            method: "getFeePerByte".to_string(),
                        },
                        arg_count: 0,
                    });
                    return Some(true);
                }
            }
            None
        }
        _ => None,
    }
}
