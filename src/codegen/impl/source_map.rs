impl CodeGenerator {
    fn generate_source_map(&self, ast: &AstNode) -> String {
        let mut source_map = String::new();

        self.visit_ast_for_source_map(ast, &mut source_map, 0);

        source_map
    }

    /// Estimate the bytecode size a node will produce, mirroring the emission
    /// logic in node_gen.rs so source map offsets track real bytecode positions.
    fn estimate_node_bytecode_size(&self, node: &AstNode) -> usize {
        match &node.node_type {
            AstNodeType::Object { statements } | AstNodeType::Block { statements } => {
                statements.iter().map(|s| self.estimate_node_bytecode_size(s)).sum()
            }
            AstNodeType::Function { name, body, .. } => {
                // PUSHDATA1 + len_byte + name_bytes + body
                let entry = 2 + name.len();
                entry + self.estimate_node_bytecode_size(body)
            }
            AstNodeType::Assignment { targets, value } => {
                let value_size = self.estimate_node_bytecode_size(value);
                // Per target: PUSHDATA1 + len_byte + name_bytes + PUSH1
                let targets_size: usize = targets.iter().map(|t| 2 + t.len() + 1).sum();
                value_size + targets_size
            }
            AstNodeType::FunctionCall { name, arguments } => {
                let args_size: usize = arguments.iter().map(|a| self.estimate_node_bytecode_size(a)).sum();
                let call_size = match name.as_str() {
                    // Arithmetic builtins: single opcode byte
                    "add" | "sub" | "mul" | "div" => 1,
                    // Syscall builtins: SYSCALL + 4-byte interop hash
                    "keccak256" | "sstore" | "sload" => 5,
                    // Generic call: PUSHDATA1 + len + name + SYSCALL + 4-byte hash
                    _ => 2 + name.len() + 5,
                };
                args_size + call_size
            }
            AstNodeType::Literal { value } => {
                if let Ok(num) = value.parse::<u8>() {
                    if num <= 16 {
                        1 // PUSH0-PUSH16
                    } else {
                        3 // PUSHDATA1 + 0x01 + byte
                    }
                } else {
                    let data_len = if let Some(stripped) = value.strip_prefix("0x") {
                        // Hex: each pair of chars = 1 byte
                        stripped.len() / 2
                    } else {
                        value.len()
                    };
                    2 + data_len // PUSHDATA1 + len_byte + data
                }
            }
            AstNodeType::Identifier { name } => {
                2 + name.len() // PUSHDATA1 + len_byte + name_bytes
            }
            AstNodeType::If { condition, then_branch, else_branch } => {
                let cond_size = self.estimate_node_bytecode_size(condition);
                let then_size = self.estimate_node_bytecode_size(then_branch);
                let jmpifnot = 2; // JMPIFNOT + offset byte
                match else_branch {
                    Some(else_stmt) => {
                        let else_size = self.estimate_node_bytecode_size(else_stmt);
                        let jmp = 2; // JMP + offset byte
                        cond_size + jmpifnot + then_size + jmp + else_size
                    }
                    None => cond_size + jmpifnot + then_size,
                }
            }
            AstNodeType::For { init, condition, update, body } => {
                let init_size = init.as_ref().map_or(0, |n| self.estimate_node_bytecode_size(n));
                let cond_size = self.estimate_node_bytecode_size(condition);
                let update_size = update.as_ref().map_or(0, |n| self.estimate_node_bytecode_size(n));
                let body_size = self.estimate_node_bytecode_size(body);
                // init + condition + JMPIFNOT(2) + body + update + JMP(2)
                init_size + cond_size + 2 + body_size + update_size + 2
            }
            AstNodeType::Switch { expression, cases, default } => {
                let expr_size = self.estimate_node_bytecode_size(expression);
                let cases_size: usize = cases.iter().map(|c| {
                    self.estimate_node_bytecode_size(&c.value)
                        + self.estimate_node_bytecode_size(&c.body)
                        + 2 // comparison + conditional jump
                }).sum();
                let default_size = default.as_ref().map_or(0, |d| self.estimate_node_bytecode_size(d));
                expr_size + cases_size + default_size
            }
            AstNodeType::Leave => 1, // RET opcode
            AstNodeType::Break | AstNodeType::Continue => 0, // placeholder, no bytecode yet
        }
    }

    #[allow(clippy::only_used_in_recursion)]
    fn visit_ast_for_source_map(&self, node: &AstNode, source_map: &mut String, offset: usize) {
        if !source_map.is_empty() {
            source_map.push(';');
        }

        source_map.push_str(&format!("{}:{}:{}", offset, node.line, node.column));

        // Recursively visit child nodes with accumulated offsets
        match &node.node_type {
            AstNodeType::Object { statements } | AstNodeType::Block { statements } => {
                let mut current_offset = offset;
                for stmt in statements {
                    self.visit_ast_for_source_map(stmt, source_map, current_offset);
                    current_offset += self.estimate_node_bytecode_size(stmt);
                }
            }
            AstNodeType::Function { name, body, .. } => {
                // Body starts after the entry sequence: PUSHDATA1 + len + name
                let body_offset = offset + 2 + name.len();
                self.visit_ast_for_source_map(body, source_map, body_offset);
            }
            AstNodeType::Assignment { targets: _, value } => {
                self.visit_ast_for_source_map(value, source_map, offset);
            }
            AstNodeType::FunctionCall { arguments, .. } => {
                let mut current_offset = offset;
                for arg in arguments {
                    self.visit_ast_for_source_map(arg, source_map, current_offset);
                    current_offset += self.estimate_node_bytecode_size(arg);
                }
            }
            AstNodeType::If { condition, then_branch, else_branch } => {
                let mut current_offset = offset;
                self.visit_ast_for_source_map(condition, source_map, current_offset);
                current_offset += self.estimate_node_bytecode_size(condition);
                current_offset += 2; // JMPIFNOT + offset byte
                self.visit_ast_for_source_map(then_branch, source_map, current_offset);
                if let Some(else_stmt) = else_branch {
                    current_offset += self.estimate_node_bytecode_size(then_branch);
                    current_offset += 2; // JMP + offset byte
                    self.visit_ast_for_source_map(else_stmt, source_map, current_offset);
                }
            }
            AstNodeType::For { init, condition, update, body } => {
                let mut current_offset = offset;
                if let Some(init_node) = init {
                    self.visit_ast_for_source_map(init_node, source_map, current_offset);
                    current_offset += self.estimate_node_bytecode_size(init_node);
                }
                self.visit_ast_for_source_map(condition, source_map, current_offset);
                current_offset += self.estimate_node_bytecode_size(condition);
                current_offset += 2; // JMPIFNOT
                self.visit_ast_for_source_map(body, source_map, current_offset);
                current_offset += self.estimate_node_bytecode_size(body);
                if let Some(update_node) = update {
                    self.visit_ast_for_source_map(update_node, source_map, current_offset);
                }
            }
            AstNodeType::Switch { expression, cases, default } => {
                let mut current_offset = offset;
                self.visit_ast_for_source_map(expression, source_map, current_offset);
                current_offset += self.estimate_node_bytecode_size(expression);
                for case in cases {
                    self.visit_ast_for_source_map(&case.value, source_map, current_offset);
                    current_offset += self.estimate_node_bytecode_size(&case.value);
                    current_offset += 2; // comparison + conditional jump
                    self.visit_ast_for_source_map(&case.body, source_map, current_offset);
                    current_offset += self.estimate_node_bytecode_size(&case.body);
                }
                if let Some(default_node) = default {
                    self.visit_ast_for_source_map(default_node, source_map, current_offset);
                }
            }
            // Leaf nodes (Literal, Identifier, Leave, Break, Continue) have no children to visit
            AstNodeType::Literal { .. } | AstNodeType::Identifier { .. }
            | AstNodeType::Leave | AstNodeType::Break | AstNodeType::Continue => {}
        }
    }

    #[allow(clippy::only_used_in_recursion)]
    fn count_ast_nodes(&self, node: &AstNode) -> usize {
        let mut count = 1;

        match &node.node_type {
            AstNodeType::Object { statements } | AstNodeType::Block { statements } => {
                for stmt in statements {
                    count += self.count_ast_nodes(stmt);
                }
            }
            AstNodeType::Function { body, .. } => {
                count += self.count_ast_nodes(body);
            }
            AstNodeType::If {
                condition,
                then_branch,
                else_branch,
            } => {
                count += self.count_ast_nodes(condition);
                count += self.count_ast_nodes(then_branch);
                if let Some(else_stmt) = else_branch {
                    count += self.count_ast_nodes(else_stmt);
                }
            }
            _ => {}
        }

        count
    }
}
