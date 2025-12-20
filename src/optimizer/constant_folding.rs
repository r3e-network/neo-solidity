impl Optimizer {
    fn constant_folding(&mut self, ast: AstNode) -> Result<AstNode, CompilerError> {
        Ok(self.fold_constants_recursive(ast))
    }

    fn fold_constants_recursive(&mut self, node: AstNode) -> AstNode {
        match node.node_type {
            AstNodeType::FunctionCall { name, arguments } => {
                // Fold constant arithmetic operations
                if let Some(result) = self.evaluate_constant_expression(&name, &arguments) {
                    self.stats.folded_constants += 1;
                    AstNode {
                        node_type: AstNodeType::Literal {
                            value: result.to_string(),
                        },
                        line: node.line,
                        column: node.column,
                    }
                } else {
                    // Recursively optimize arguments
                    let optimized_args = arguments
                        .into_iter()
                        .map(|arg| self.fold_constants_recursive(arg))
                        .collect();

                    AstNode {
                        node_type: AstNodeType::FunctionCall {
                            name,
                            arguments: optimized_args,
                        },
                        line: node.line,
                        column: node.column,
                    }
                }
            }
            AstNodeType::Object { statements } => {
                let optimized_statements = statements
                    .into_iter()
                    .map(|stmt| self.fold_constants_recursive(stmt))
                    .collect();

                AstNode {
                    node_type: AstNodeType::Object {
                        statements: optimized_statements,
                    },
                    line: node.line,
                    column: node.column,
                }
            }
            AstNodeType::Block { statements } => {
                let optimized_statements = statements
                    .into_iter()
                    .map(|stmt| self.fold_constants_recursive(stmt))
                    .collect();

                AstNode {
                    node_type: AstNodeType::Block {
                        statements: optimized_statements,
                    },
                    line: node.line,
                    column: node.column,
                }
            }
            _ => node, // Return unchanged for other node types
        }
    }

    fn evaluate_constant_expression(&self, name: &str, arguments: &[AstNode]) -> Option<u64> {
        if arguments.len() != 2 {
            return None;
        }

        let arg1 = self.extract_constant(&arguments[0])?;
        let arg2 = self.extract_constant(&arguments[1])?;

        match name {
            "add" => Some(arg1.wrapping_add(arg2)),
            "sub" => Some(arg1.wrapping_sub(arg2)),
            "mul" => Some(arg1.wrapping_mul(arg2)),
            "div" if arg2 != 0 => Some(arg1 / arg2),
            "mod" if arg2 != 0 => Some(arg1 % arg2),
            "eq" => Some(if arg1 == arg2 { 1 } else { 0 }),
            "lt" => Some(if arg1 < arg2 { 1 } else { 0 }),
            "gt" => Some(if arg1 > arg2 { 1 } else { 0 }),
            "and" => Some(arg1 & arg2),
            "or" => Some(arg1 | arg2),
            "xor" => Some(arg1 ^ arg2),
            _ => None,
        }
    }

    fn extract_constant(&self, node: &AstNode) -> Option<u64> {
        if let AstNodeType::Literal { value } = &node.node_type {
            if let Some(stripped) = value.strip_prefix("0x") {
                u64::from_str_radix(stripped, 16).ok()
            } else {
                value.parse::<u64>().ok()
            }
        } else {
            None
        }
    }
}

