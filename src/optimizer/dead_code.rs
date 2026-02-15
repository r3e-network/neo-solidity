impl Optimizer {
    fn dead_code_elimination(&mut self, ast: AstNode) -> Result<AstNode, CompilerError> {
        let result = self.eliminate_dead_code_recursive(ast, false);
        Ok(self.remove_empty_blocks(result))
    }

    fn eliminate_dead_code_recursive(&mut self, node: AstNode, _after_return: bool) -> AstNode {
        match node.node_type {
            AstNodeType::Object { statements } => {
                let optimized = self.process_statement_list(statements);
                AstNode {
                    node_type: AstNodeType::Object {
                        statements: optimized,
                    },
                    line: node.line,
                    column: node.column,
                }
            }
            AstNodeType::Block { statements } => {
                let optimized = self.process_statement_list(statements);
                AstNode {
                    node_type: AstNodeType::Block {
                        statements: optimized,
                    },
                    line: node.line,
                    column: node.column,
                }
            }
            AstNodeType::If {
                condition,
                then_branch,
                else_branch,
            } => {
                // Recursively process branches
                let opt_then = self.eliminate_dead_code_recursive(*then_branch, false);
                let opt_else =
                    else_branch.map(|eb| Box::new(self.eliminate_dead_code_recursive(*eb, false)));

                // Check for empty branches
                let then_empty = self.is_empty_block(&opt_then);
                let else_empty = opt_else.as_ref().is_none_or(|eb| self.is_empty_block(eb));

                if then_empty && else_empty {
                    // Both branches empty - eliminate entire if
                    self.stats.eliminated_instructions += 1;
                    AstNode {
                        node_type: AstNodeType::Block { statements: vec![] },
                        line: node.line,
                        column: node.column,
                    }
                } else {
                    AstNode {
                        node_type: AstNodeType::If {
                            condition,
                            then_branch: Box::new(opt_then),
                            else_branch: opt_else,
                        },
                        line: node.line,
                        column: node.column,
                    }
                }
            }
            AstNodeType::Function {
                name,
                params,
                body,
                returns,
            } => {
                let opt_body = self.eliminate_dead_code_recursive(*body, false);
                AstNode {
                    node_type: AstNodeType::Function {
                        name,
                        params,
                        body: Box::new(opt_body),
                        returns,
                    },
                    line: node.line,
                    column: node.column,
                }
            }
            _ => node,
        }
    }

    /// Process a list of statements, removing dead code after terminators
    fn process_statement_list(&mut self, statements: Vec<AstNode>) -> Vec<AstNode> {
        let mut optimized = Vec::with_capacity(statements.len());
        let mut found_terminator = false;

        for stmt in statements {
            if found_terminator {
                // Skip dead code but preserve labels
                if !self.is_label(&stmt) {
                    self.stats.eliminated_instructions += 1;
                    continue;
                }
            }

            let opt_stmt = self.eliminate_dead_code_recursive(stmt, found_terminator);

            // Check for terminators
            if self.is_terminator(&opt_stmt) {
                found_terminator = true;
            }

            // Skip no-op statements
            if !self.is_noop(&opt_stmt) {
                optimized.push(opt_stmt);
            } else {
                self.stats.eliminated_instructions += 1;
            }
        }

        optimized
    }

    /// Check if a statement is a control flow terminator
    fn is_terminator(&self, node: &AstNode) -> bool {
        match &node.node_type {
            AstNodeType::Leave | AstNodeType::Break | AstNodeType::Continue => true,
            AstNodeType::FunctionCall { name, .. } => {
                matches!(
                    name.as_str(),
                    "return" | "revert" | "stop" | "invalid" | "selfdestruct" | "abort" | "throw"
                )
            }
            _ => false,
        }
    }

    /// Check if a node is a label (should be preserved)
    /// Note: Labels are not currently supported in this compiler's AST.
    /// This function exists for future extension when label support is added.
    fn is_label(&self, _node: &AstNode) -> bool {
        false
    }

    /// Check if a block is empty or contains only empty blocks
    #[allow(clippy::only_used_in_recursion)]
    fn is_empty_block(&self, node: &AstNode) -> bool {
        match &node.node_type {
            AstNodeType::Block { statements } => {
                statements.is_empty() || statements.iter().all(|s| self.is_empty_block(s))
            }
            _ => false,
        }
    }

    /// Check if a statement is a no-op (can be safely removed)
    fn is_noop(&self, node: &AstNode) -> bool {
        match &node.node_type {
            AstNodeType::Block { statements } => statements.is_empty(),
            AstNodeType::FunctionCall { name, arguments } => {
                // pop() with no side effects
                name == "pop" && arguments.iter().all(|a| self.is_pure_expression(a))
            }
            _ => false,
        }
    }

    /// Check if an expression has no side effects
    #[allow(clippy::only_used_in_recursion)]
    fn is_pure_expression(&self, node: &AstNode) -> bool {
        match &node.node_type {
            AstNodeType::Literal { .. } | AstNodeType::Identifier { .. } => true,
            AstNodeType::FunctionCall { name, arguments } => {
                // Pure operations
                let pure_ops = [
                    "add",
                    "sub",
                    "mul",
                    "div",
                    "mod",
                    "exp",
                    "eq",
                    "lt",
                    "gt",
                    "slt",
                    "sgt",
                    "le",
                    "ge",
                    "and",
                    "or",
                    "xor",
                    "not",
                    "shl",
                    "shr",
                    "sar",
                    "iszero",
                    "byte",
                    "signextend",
                ];
                pure_ops.contains(&name.as_str())
                    && arguments.iter().all(|a| self.is_pure_expression(a))
            }
            _ => false,
        }
    }

    /// Remove nested empty blocks
    fn remove_empty_blocks(&self, node: AstNode) -> AstNode {
        match node.node_type {
            AstNodeType::Block { statements } => {
                let filtered: Vec<AstNode> = statements
                    .into_iter()
                    .map(|s| self.remove_empty_blocks(s))
                    .filter(|s| !self.is_empty_block(s))
                    .collect();
                AstNode {
                    node_type: AstNodeType::Block {
                        statements: filtered,
                    },
                    line: node.line,
                    column: node.column,
                }
            }
            AstNodeType::Object { statements } => {
                let filtered: Vec<AstNode> = statements
                    .into_iter()
                    .map(|s| self.remove_empty_blocks(s))
                    .filter(|s| !self.is_empty_block(s))
                    .collect();
                AstNode {
                    node_type: AstNodeType::Object {
                        statements: filtered,
                    },
                    line: node.line,
                    column: node.column,
                }
            }
            _ => node,
        }
    }
}
