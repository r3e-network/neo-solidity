impl Optimizer {
    fn dead_code_elimination(&mut self, ast: AstNode) -> Result<AstNode, CompilerError> {
        Ok(self.eliminate_dead_code_recursive(ast, false))
    }

    fn eliminate_dead_code_recursive(&mut self, node: AstNode, _after_return: bool) -> AstNode {
        match node.node_type {
            AstNodeType::Object { statements } => {
                let mut optimized_statements = Vec::new();
                let mut found_return = false;

                for stmt in statements {
                    if found_return {
                        self.stats.eliminated_instructions += 1;
                        continue; // Skip dead code after return
                    }

                    let optimized_stmt = self.eliminate_dead_code_recursive(stmt, found_return);

                    // Check if this statement is a return/revert
                    if let AstNodeType::FunctionCall { name, .. } = &optimized_stmt.node_type {
                        if name == "return" || name == "revert" {
                            found_return = true;
                        }
                    }

                    optimized_statements.push(optimized_stmt);
                }

                AstNode {
                    node_type: AstNodeType::Object {
                        statements: optimized_statements,
                    },
                    line: node.line,
                    column: node.column,
                }
            }
            AstNodeType::Block { statements } => {
                let mut optimized_statements = Vec::new();
                let mut found_return = false;

                for stmt in statements {
                    if found_return {
                        self.stats.eliminated_instructions += 1;
                        continue;
                    }

                    let optimized_stmt = self.eliminate_dead_code_recursive(stmt, found_return);

                    if let AstNodeType::FunctionCall { name, .. } = &optimized_stmt.node_type {
                        if name == "return" || name == "revert" {
                            found_return = true;
                        }
                    }

                    optimized_statements.push(optimized_stmt);
                }

                AstNode {
                    node_type: AstNodeType::Block {
                        statements: optimized_statements,
                    },
                    line: node.line,
                    column: node.column,
                }
            }
            _ => node,
        }
    }
}

