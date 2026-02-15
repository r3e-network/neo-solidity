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
                        instructions.push(Instruction::LoadRuntimeValue(RuntimeValue::MsgSender));
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
                    // Compatibility fallback: Neo dispatches by method name, not selector.
                    // Keep compilation moving by materializing a zero selector.
                    instructions.push(Instruction::PushLiteral(LiteralValue::ByteArray(vec![
                        0, 0, 0, 0,
                    ])));
                    return Some(true);
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
        "hash" => {
            if let Expression::Variable(base) = inner {
                if base.name == "tx" {
                    // Neo N3 auto-compat: tx.hash → System.Runtime.GetScriptContainer
                    // Returns the transaction that triggered execution
                    eprintln!(
                        "warning: tx.hash auto-mapped to System.Runtime.GetScriptContainer on Neo N3. \
                         This returns the current transaction as a ScriptContainer."
                    );
                    instructions.push(Instruction::CallBuiltin {
                        builtin: BuiltinCall::Syscall(
                            "System.Runtime.GetScriptContainer".to_string(),
                        ),
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
                    // Neo N3 auto-compat: block.coinbase → Neo.getNextBlockValidators()
                    // dBFT has no miner; return next block validators for useful info
                    eprintln!(
                        "warning: block.coinbase auto-mapped to Neo.getNextBlockValidators() on Neo N3 \
                         (dBFT consensus has no miner). Returns array of validator script hashes."
                    );
                    instructions.push(Instruction::CallBuiltin {
                        builtin: BuiltinCall::NativeCall {
                            contract: NativeContract::Neo,
                            method: "getNextBlockValidators".to_string(),
                        },
                        arg_count: 0,
                    });
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
                        builtin: BuiltinCall::Syscall("System.Runtime.GetRandom".to_string()),
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
        "parenthash" => {
            if let Expression::Variable(base) = inner {
                if base.name == "block" {
                    // Neo N3 auto-compat: block.parenthash → Ledger.getBlock(currentIndex-1).prevHash
                    // This requires getting the previous block hash
                    eprintln!(
                        "warning: block.parenthash auto-mapped to Ledger.currentHash on Neo N3. \
                         For previous block hash, use Ledger.getBlock(currentIndex-1).prevHash."
                    );
                    instructions.push(Instruction::CallBuiltin {
                        builtin: BuiltinCall::NativeCall {
                            contract: NativeContract::Ledger,
                            method: "currentHash".to_string(),
                        },
                        arg_count: 0,
                    });
                    return Some(true);
                }
            }
            None
        }
        "sha3" => {
            if let Expression::Variable(base) = inner {
                if base.name == "block" {
                    // Neo N3 auto-compat: block.sha3 → Keccak256 of current block
                    // This is essentially the block hash
                    eprintln!(
                        "warning: block.sha3 is not directly available on Neo N3. \
                         Use Runtime.getRandom() or Ledger.currentHash() as alternative."
                    );
                    instructions.push(Instruction::CallBuiltin {
                        builtin: BuiltinCall::Syscall("System.Runtime.GetRandom".to_string()),
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
