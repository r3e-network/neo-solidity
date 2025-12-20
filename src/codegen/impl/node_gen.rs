impl CodeGenerator {
    #[allow(clippy::only_used_in_recursion)]
    fn generate_node(
        &mut self,
        node: &AstNode,
        bytecode: &mut Vec<u8>,
        functions: &mut Vec<String>,
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
                params: _,
                returns: _,
                body,
            } => {
                functions.push(name.clone());

                // Function entry
                bytecode.push(0x0C); // PUSHDATA1
                bytecode.push(name.len() as u8);
                bytecode.extend_from_slice(name.as_bytes());

                // Generate function body
                self.generate_node(body, bytecode, functions, events, estimated_gas)?;

                *estimated_gas += 50; // Function call overhead
            }
            AstNodeType::Assignment { targets, value } => {
                // Generate value expression
                self.generate_node(value, bytecode, functions, events, estimated_gas)?;

                // Store to variables using stack operations
                for target in targets {
                    bytecode.push(0x0C); // PUSHDATA1
                    bytecode.push(target.len() as u8);
                    bytecode.extend_from_slice(target.as_bytes());
                    bytecode.push(0x51); // PUSH1 for variable storage
                }

                *estimated_gas += targets.len() as u64 * 10;
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
                        *estimated_gas += 3;
                    }
                    "sub" => {
                        bytecode.push(0x9F); // SUB
                        *estimated_gas += 3;
                    }
                    "mul" => {
                        bytecode.push(0xA0); // MUL
                        *estimated_gas += 5;
                    }
                    "div" => {
                        bytecode.push(0xA1); // DIV
                        *estimated_gas += 5;
                    }
                    "keccak256" => {
                        bytecode.push(0x41); // SYSCALL
                        bytecode.extend_from_slice(&interop_id_bytes("Neo.Crypto.Keccak256"));
                        *estimated_gas += 30;
                    }
                    "sstore" => {
                        bytecode.push(0x41); // SYSCALL
                        bytecode.extend_from_slice(&interop_id_bytes("System.Storage.Put"));
                        *estimated_gas += 20000;
                    }
                    "sload" => {
                        bytecode.push(0x41); // SYSCALL
                        bytecode.extend_from_slice(&interop_id_bytes("System.Storage.Get"));
                        *estimated_gas += 800;
                    }
                    _ => {
                        // Generic function call
                        bytecode.push(0x0C); // PUSHDATA1
                        bytecode.push(name.len() as u8);
                        bytecode.extend_from_slice(name.as_bytes());
                        bytecode.push(0x41); // SYSCALL
                        bytecode.extend_from_slice(&interop_id_bytes("System.Contract.Call"));
                        *estimated_gas += 1000;
                    }
                }
            }
            AstNodeType::Literal { value } => {
                if let Ok(num) = value.parse::<u8>() {
                    if num <= 16 {
                        bytecode.push(0x50 + num); // PUSH0-PUSH16
                    } else {
                        bytecode.push(0x0C); // PUSHDATA1
                        bytecode.push(0x01);
                        bytecode.push(num);
                    }
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

                *estimated_gas += 3;
            }
            AstNodeType::Identifier { name } => {
                // Load variable by pushing identifier
                bytecode.push(0x0C); // PUSHDATA1
                bytecode.push(name.len() as u8);
                bytecode.extend_from_slice(name.as_bytes());

                *estimated_gas += 3;
            }
            AstNodeType::If {
                condition,
                then_branch,
                else_branch,
            } => {
                // Generate condition
                self.generate_node(condition, bytecode, functions, events, estimated_gas)?;

                // JMPIFNOT to else/end
                bytecode.push(0x23); // JMPIFNOT
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
                    bytecode[else_jump_pos] = (bytecode.len() - else_jump_pos - 1) as u8;

                    // Generate else branch
                    if let Some(else_stmt) = else_branch {
                        self.generate_node(else_stmt, bytecode, functions, events, estimated_gas)?;
                    }

                    // Update end jump offset
                    bytecode[end_jump_pos] = (bytecode.len() - end_jump_pos - 1) as u8;
                } else {
                    // Update else jump offset to end
                    bytecode[else_jump_pos] = (bytecode.len() - else_jump_pos - 1) as u8;
                }

                *estimated_gas += 10;
            }
            _ => {
                // Handle other node types
                *estimated_gas += 1;
            }
        }

        Ok(())
    }
}

