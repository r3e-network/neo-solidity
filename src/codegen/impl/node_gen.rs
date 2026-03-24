impl CodeGenerator {
    #[allow(clippy::only_used_in_recursion)]
    fn generate_node(
        &mut self,
        node: &AstNode,
        bytecode: &mut Vec<u8>,
        functions: &mut Vec<FunctionMeta>,
        events: &mut Vec<String>,
        estimated_gas: &mut u64,
    ) -> Result<(), CompilerError> {
        match &node.node_type {
            AstNodeType::Object { statements } | AstNodeType::Block { statements } => {
                for stmt in statements {
                    self.generate_node(stmt, bytecode, functions, events, estimated_gas)?;
                }
            }
            AstNodeType::Function {
                name,
                params,
                returns,
                body,
            } => {
                // Record the bytecode offset before emitting this function's entry.
                let raw_offset = bytecode.len();
                functions.push(FunctionMeta {
                    name: name.clone(),
                    params: params.clone(),
                    returns: returns.clone(),
                    raw_offset,
                });

                // Function entry: INITSLOT with local-count and param-count
                bytecode.push(0x56); // INITSLOT
                let initslot_local_pos = bytecode.len();
                bytecode.push(0x00); // local variable count placeholder (patched after body)
                bytecode.push(params.len() as u8); // parameter count

                // Register parameters as variables
                self.reset_variables();
                for param in params {
                    self.register_variable(param);
                }
                let params_count = params.len();

                // Generate function body
                self.generate_node(body, bytecode, functions, events, estimated_gas)?;

                // Patch the local variable count: total registered vars minus params
                let local_only = self.next_var_index.saturating_sub(params_count);
                bytecode[initslot_local_pos] = local_only as u8;

                *estimated_gas += 512; // NeoVM CALL opcode cost
            }
            AstNodeType::Assignment { targets, value } => {
                // Generate value expression first
                self.generate_node(value, bytecode, functions, events, estimated_gas)?;

                // Store to each target variable using index-based STLOC
                for target in targets {
                    // Get or create variable index
                    let var_index = match self.get_variable_index(target) {
                        Some(idx) => idx,
                        None => self.register_variable(target),
                    };

                    // Emit STLOC with variable index
                    emit_stloc(bytecode, var_index);
                }

                *estimated_gas += targets.len() as u64 * 8;
            }
            AstNodeType::FunctionCall { name, arguments } => {
                // Generate arguments
                for arg in arguments {
                    self.generate_node(arg, bytecode, functions, events, estimated_gas)?;
                }

                // Generate function call based on built-in type
                match name.as_str() {
                    "add" => {
                        bytecode.push(0x9E); // ADD
                        *estimated_gas += 8; // NeoVM arithmetic cost
                    }
                    "sub" => {
                        bytecode.push(0x9F); // SUB
                        *estimated_gas += 8;
                    }
                    "mul" => {
                        bytecode.push(0xA0); // MUL
                        *estimated_gas += 8;
                    }
                    "div" => {
                        bytecode.push(0xA1); // DIV
                        *estimated_gas += 8;
                    }
                    "mod" => {
                        bytecode.push(0xA2); // MOD
                        *estimated_gas += 8;
                    }
                    "and" => {
                        bytecode.push(0xA3); // AND
                        *estimated_gas += 8;
                    }
                    "or" => {
                        bytecode.push(0xA4); // OR
                        *estimated_gas += 8;
                    }
                    "xor" => {
                        bytecode.push(0xA5); // XOR
                        *estimated_gas += 8;
                    }
                    "shl" => {
                        bytecode.push(0xA8); // SHL
                        *estimated_gas += 8;
                    }
                    "shr" => {
                        bytecode.push(0xA9); // SHR
                        *estimated_gas += 8;
                    }
                    "keccak256" => {
                        // Keccak256 is a CryptoLib native method, route via System.Contract.Call
                        bytecode.push(0x0C); // PUSHDATA1
                        bytecode.push("keccak256".len() as u8);
                        bytecode.extend_from_slice("keccak256".as_bytes());
                        bytecode.push(0x41); // SYSCALL
                        bytecode.extend_from_slice(&interop_id_bytes("System.Contract.Call"));
                        *estimated_gas += 700000; // NeoVM contract call cost for crypto
                    }
                    "sha256" => {
                        bytecode.push(0x0C); // PUSHDATA1
                        bytecode.push("sha256".len() as u8);
                        bytecode.extend_from_slice("sha256".as_bytes());
                        bytecode.push(0x41); // SYSCALL
                        bytecode.extend_from_slice(&interop_id_bytes("System.Contract.Call"));
                        *estimated_gas += 700000;
                    }
                    "sload" => {
                        bytecode.push(0x41); // SYSCALL
                        bytecode.extend_from_slice(&interop_id_bytes("System.Storage.Get"));
                        *estimated_gas += 100; // NeoVM StorageGet cost
                    }
                    "sstore" => {
                        bytecode.push(0x41); // SYSCALL
                        bytecode.extend_from_slice(&interop_id_bytes("System.Storage.Put"));
                        *estimated_gas += 1000; // NeoVM StoragePut cost
                    }
                    _ => {
                        // Generic function call
                        bytecode.push(0x0C); // PUSHDATA1
                        bytecode.push(name.len() as u8);
                        bytecode.extend_from_slice(name.as_bytes());
                        bytecode.push(0x41); // SYSCALL
                        bytecode.extend_from_slice(&interop_id_bytes("System.Contract.Call"));
                        *estimated_gas += 512; // NeoVM contract call cost
                    }
                }
            }
            AstNodeType::Literal { value } => {
                if let Ok(num) = value.parse::<i64>() {
                    bytecode.extend_from_slice(&crate::codegen_helpers::encode_small_int(num));
                } else {
                    // String or hex literal
                    let data = if let Some(stripped) = value.strip_prefix("0x") {
                        hex::decode(stripped).unwrap_or_else(|_| value.as_bytes().to_vec())
                    } else {
                        value.as_bytes().to_vec()
                    };
                    bytecode.push(0x0C); // PUSHDATA1
                    bytecode.push(data.len() as u8);
                    bytecode.extend_from_slice(&data);
                }
                *estimated_gas += 1; // NeoVM PUSH cost
            }
            AstNodeType::Identifier { name } => {
                // Load variable by index from variable table
                let var_index = match self.get_variable_index(name) {
                    Some(idx) => idx,
                    None => {
                        return Err(CompilerError::CodegenError(
                            format!("undefined variable: {name}"),
                        ));
                    }
                };

                // Emit LDLOC with variable index
                emit_ldloc(bytecode, var_index);

                *estimated_gas += 1;
            }
            AstNodeType::If {
                condition,
                then_branch,
                else_branch,
            } => {
                // Generate condition
                self.generate_node(condition, bytecode, functions, events, estimated_gas)?;

                // JMPIFNOT to else/end
                bytecode.push(0x26); // JMPIFNOT
                let else_jump_pos = bytecode.len();
                bytecode.push(0x00); // Jump offset (patched below)

                // Generate then branch
                self.generate_node(then_branch, bytecode, functions, events, estimated_gas)?;

                if else_branch.is_some() {
                    // JMP to end
                    bytecode.push(0x22); // JMP
                    let end_jump_pos = bytecode.len();
                    bytecode.push(0x00); // Jump offset (patched below)

                    // Update else jump offset
                    bytecode[else_jump_pos] = (bytecode.len() - else_jump_pos + 1) as u8;

                    // Generate else branch
                    if let Some(else_stmt) = else_branch {
                        self.generate_node(else_stmt, bytecode, functions, events, estimated_gas)?;
                    }

                    // Update end jump offset
                    bytecode[end_jump_pos] = (bytecode.len() - end_jump_pos + 1) as u8;
                } else {
                    // Update else jump offset to end
                    bytecode[else_jump_pos] = (bytecode.len() - else_jump_pos + 1) as u8;
                }

                *estimated_gas += 8; // NeoVM JMP cost
            }
            AstNodeType::For {
                init,
                condition,
                update,
                body,
            } => {
                // Generate initialization
                if let Some(init_node) = init {
                    self.generate_node(init_node, bytecode, functions, events, estimated_gas)?;
                }

                let loop_start = bytecode.len();

                // Push loop context -- break uses deferred patching via break_patches
                self.loop_stack.push(LoopContext {
                    break_target: 0, // sentinel; break uses JMP_L with deferred patching
                    continue_target: loop_start,
                });

                // Generate condition (Box<AstNode>, not Option)
                self.generate_node(condition, bytecode, functions, events, estimated_gas)?;

                // JMPIFNOT_L to end (5-byte: opcode + 4-byte signed offset)
                bytecode.push(0x27); // JMPIFNOT_L
                let end_jump_pos = bytecode.len();
                bytecode.extend_from_slice(&[0x00; 4]); // 4-byte offset placeholder

                // Generate body (Box<AstNode>, not Option)
                self.generate_node(body, bytecode, functions, events, estimated_gas)?;

                // Generate update
                if let Some(update_node) = update {
                    self.generate_node(update_node, bytecode, functions, events, estimated_gas)?;
                }

                // Pop loop context
                self.loop_stack.pop();

                // Jump back to condition using JMP_L (4-byte signed offset)
                bytecode.push(0x23); // JMP_L
                let back_offset = loop_start as i32 - bytecode.len() as i32;
                bytecode.extend_from_slice(&back_offset.to_le_bytes());

                // Now we know the loop end -- patch the JMPIFNOT_L offset
                let loop_end = bytecode.len();
                let end_offset = loop_end as i32 - end_jump_pos as i32 + 1;
                bytecode[end_jump_pos..end_jump_pos + 4]
                    .copy_from_slice(&end_offset.to_le_bytes());

                // Patch all break JMP_L offsets emitted during body generation
                for patch_pos in self.drain_break_patches() {
                    let brk_offset = loop_end as i32 - patch_pos as i32;
                    bytecode[patch_pos..patch_pos + 4]
                        .copy_from_slice(&brk_offset.to_le_bytes());
                }

                *estimated_gas += 100; // Loop overhead
            }
            AstNodeType::Leave => {
                bytecode.push(0x40); // RET
                *estimated_gas += 1;
            }
            AstNodeType::Break => {
                if self.loop_stack.last().is_some() {
                    // Emit JMP_L with placeholder offset; record position for patching
                    bytecode.push(0x23); // JMP_L
                    let patch_pos = bytecode.len();
                    bytecode.extend_from_slice(&[0x00; 4]); // placeholder
                    self.add_break_patch(patch_pos);
                    *estimated_gas += 8;
                } else {
                    return Err(CompilerError::CodegenError(
                        "break statement outside of loop".to_string(),
                    ));
                }
            }
            AstNodeType::Continue => {
                if let Some(loop_ctx) = self.loop_stack.last() {
                    let continue_target = loop_ctx.continue_target;
                    // Use JMP_L (4-byte signed offset) for safety
                    bytecode.push(0x23); // JMP_L
                    let offset = continue_target as i32 - bytecode.len() as i32;
                    bytecode.extend_from_slice(&offset.to_le_bytes());
                    *estimated_gas += 8;
                } else {
                    return Err(CompilerError::CodegenError(
                        "continue statement outside of loop".to_string(),
                    ));
                }
            }
            _ => {
                // Handle other node types
                *estimated_gas += 1;
            }
        }

        Ok(())
    }
}

/// Emit LDLOC instruction with proper index encoding
/// NeoVM uses 0x06-0x0C for indices 0-6, or 0x0E + index for larger indices
fn emit_ldloc(bytecode: &mut Vec<u8>, index: usize) {
    match index {
        0 => bytecode.push(0x06), // LDLOC0
        1 => bytecode.push(0x07), // LDLOC1
        2 => bytecode.push(0x08), // LDLOC2
        3 => bytecode.push(0x09), // LDLOC3
        4 => bytecode.push(0x0A), // LDLOC4
        5 => bytecode.push(0x0B), // LDLOC5
        6 => bytecode.push(0x0C), // LDLOC6
        _ => {
            bytecode.push(0x0E); // LDLOC with index
            bytecode.push(index as u8);
        }
    }
}

/// Emit STLOC instruction with proper index encoding
/// NeoVM uses 0x70-0x76 for indices 0-6, or 0x78 + index for larger indices
fn emit_stloc(bytecode: &mut Vec<u8>, index: usize) {
    match index {
        0 => bytecode.push(0x70), // STLOC0
        1 => bytecode.push(0x71), // STLOC1
        2 => bytecode.push(0x72), // STLOC2
        3 => bytecode.push(0x73), // STLOC3
        4 => bytecode.push(0x74), // STLOC4
        5 => bytecode.push(0x75), // STLOC5
        6 => bytecode.push(0x76), // STLOC6
        _ => {
            bytecode.push(0x78); // STLOC with index
            bytecode.push(index as u8);
        }
    }
}
