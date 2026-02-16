impl Optimizer {
    fn constant_folding(&mut self, ast: AstNode) -> Result<AstNode, CompilerError> {
        // Run multiple passes until no more folding occurs
        let mut current = ast;
        let mut prev_count = 0;
        
        loop {
            let before = self.stats.folded_constants;
            current = self.fold_constants_recursive(current);
            let after = self.stats.folded_constants;
            
            // Stop if no progress or max iterations reached
            if after == before || after - prev_count > 1000 {
                break;
            }
            prev_count = after;
        }
        
        Ok(current)
    }

    fn fold_constants_recursive(&mut self, node: AstNode) -> AstNode {
        match node.node_type {
            AstNodeType::FunctionCall { name, arguments } => {
                // First recursively optimize arguments
                let optimized_args: Vec<AstNode> = arguments
                    .into_iter()
                    .map(|arg| self.fold_constants_recursive(arg))
                    .collect();

                // Then try to fold the expression with optimized args
                if let Some(result) = self.evaluate_constant_expression(&name, &optimized_args) {
                    self.stats.folded_constants += 1;
                    AstNode {
                        node_type: AstNodeType::Literal {
                            value: result.to_string(),
                        },
                        line: node.line,
                        column: node.column,
                    }
                } else if let Some(simplified) = self.simplify_identity_ops(&name, optimized_args.clone(), node.line, node.column) {
                    // Try identity simplifications (x + 0 = x, x * 1 = x, etc.)
                    self.stats.folded_constants += 1;
                    simplified
                } else {
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
            AstNodeType::If { condition, then_branch, else_branch } => {
                let folded_cond = self.fold_constants_recursive(*condition);
                
                // If condition is constant, eliminate the branch
                if let Some(cond_val) = self.extract_constant(&folded_cond) {
                    self.stats.folded_constants += 1;
                    if cond_val != 0 {
                        self.fold_constants_recursive(*then_branch)
                    } else if let Some(eb) = else_branch {
                        self.fold_constants_recursive(*eb)
                    } else {
                        // No else branch and condition is false - return empty block
                        AstNode {
                            node_type: AstNodeType::Block { statements: vec![] },
                            line: node.line,
                            column: node.column,
                        }
                    }
                } else {
                    AstNode {
                        node_type: AstNodeType::If {
                            condition: Box::new(folded_cond),
                            then_branch: Box::new(self.fold_constants_recursive(*then_branch)),
                            else_branch: else_branch.map(|eb| Box::new(self.fold_constants_recursive(*eb))),
                        },
                        line: node.line,
                        column: node.column,
                    }
                }
            }
            _ => node,
        }
    }

    fn evaluate_constant_expression(&self, name: &str, arguments: &[AstNode]) -> Option<u64> {
        // Handle unary operations
        if arguments.len() == 1 {
            let arg = self.extract_constant(&arguments[0])?;
            return match name {
                "not" => Some(!arg),
                "iszero" => Some(if arg == 0 { 1 } else { 0 }),
                _ => None,
            };
        }

        if arguments.len() != 2 {
            return None;
        }

        let arg1 = self.extract_constant(&arguments[0])?;
        let arg2 = self.extract_constant(&arguments[1])?;

        match name {
            // Arithmetic
            "add" => Some(arg1.wrapping_add(arg2)),
            "sub" => Some(arg1.wrapping_sub(arg2)),
            "mul" => Some(arg1.wrapping_mul(arg2)),
            "div" if arg2 != 0 => Some(arg1 / arg2),
            "sdiv" if arg2 != 0 => Some(((arg1 as i64) / (arg2 as i64)) as u64),
            "mod" if arg2 != 0 => Some(arg1 % arg2),
            "smod" if arg2 != 0 => Some(((arg1 as i64) % (arg2 as i64)) as u64),
            "exp" => Some(arg1.wrapping_pow(arg2 as u32)),
            
            // Comparisons
            "eq" => Some(if arg1 == arg2 { 1 } else { 0 }),
            "lt" => Some(if arg1 < arg2 { 1 } else { 0 }),
            "gt" => Some(if arg1 > arg2 { 1 } else { 0 }),
            "slt" => Some(if (arg1 as i64) < (arg2 as i64) { 1 } else { 0 }),
            "sgt" => Some(if (arg1 as i64) > (arg2 as i64) { 1 } else { 0 }),
            "le" => Some(if arg1 <= arg2 { 1 } else { 0 }),
            "ge" => Some(if arg1 >= arg2 { 1 } else { 0 }),
            
            // Bitwise
            "and" => Some(arg1 & arg2),
            "or" => Some(arg1 | arg2),
            "xor" => Some(arg1 ^ arg2),
            "shl" => Some(arg2.wrapping_shl(arg1 as u32)),
            "shr" => Some(arg2.wrapping_shr(arg1 as u32)),
            "sar" => Some(((arg2 as i64).wrapping_shr(arg1 as u32)) as u64),
            
            _ => None,
        }
    }

    /// Simplify identity operations like x + 0 = x, x * 1 = x
    fn simplify_identity_ops(&self, name: &str, args: Vec<AstNode>, line: usize, column: usize) -> Option<AstNode> {
        if args.len() != 2 {
            return None;
        }

        let arg1_const = self.extract_constant(&args[0]);
        let arg2_const = self.extract_constant(&args[1]);

        match name {
            // x + 0 = 0 + x = x
            "add" => {
                if arg1_const == Some(0) {
                    return Some(args[1].clone());
                }
                if arg2_const == Some(0) {
                    return Some(args[0].clone());
                }
            }
            // x - 0 = x
            "sub" => {
                if arg2_const == Some(0) {
                    return Some(args[0].clone());
                }
            }
            // x * 1 = 1 * x = x, x * 0 = 0 * x = 0, x * 2^n = shl(n, x)
            "mul" => {
                if arg1_const == Some(1) {
                    return Some(args[1].clone());
                }
                if arg2_const == Some(1) {
                    return Some(args[0].clone());
                }
                if arg1_const == Some(0) || arg2_const == Some(0) {
                    return Some(AstNode {
                        node_type: AstNodeType::Literal { value: "0".to_string() },
                        line,
                        column,
                    });
                }
                // Strength reduction: x * 2^n → shl(n, x)
                if let Some(val) = arg2_const {
                    if let Some(shift) = StrengthReducer::reduce_mul_pow2(val) {
                        return Some(AstNode {
                            node_type: AstNodeType::FunctionCall {
                                name: "shl".to_string(),
                                arguments: vec![
                                    AstNode {
                                        node_type: AstNodeType::Literal { value: shift.to_string() },
                                        line, column,
                                    },
                                    args[0].clone(),
                                ],
                            },
                            line, column,
                        });
                    }
                }
                if let Some(val) = arg1_const {
                    if let Some(shift) = StrengthReducer::reduce_mul_pow2(val) {
                        return Some(AstNode {
                            node_type: AstNodeType::FunctionCall {
                                name: "shl".to_string(),
                                arguments: vec![
                                    AstNode {
                                        node_type: AstNodeType::Literal { value: shift.to_string() },
                                        line, column,
                                    },
                                    args[1].clone(),
                                ],
                            },
                            line, column,
                        });
                    }
                }
            }
            // x / 1 = x, x / 2^n = shr(n, x)
            "div" => {
                if arg2_const == Some(1) {
                    return Some(args[0].clone());
                }
                if let Some(val) = arg2_const {
                    if let Some(shift) = StrengthReducer::reduce_div_pow2(val) {
                        return Some(AstNode {
                            node_type: AstNodeType::FunctionCall {
                                name: "shr".to_string(),
                                arguments: vec![
                                    AstNode {
                                        node_type: AstNodeType::Literal { value: shift.to_string() },
                                        line, column,
                                    },
                                    args[0].clone(),
                                ],
                            },
                            line, column,
                        });
                    }
                }
            }
            "sdiv" => {
                if arg2_const == Some(1) {
                    return Some(args[0].clone());
                }
            }
            // x % 2^n = and(x, 2^n - 1)
            "mod" => {
                if let Some(val) = arg2_const {
                    if let Some(mask) = StrengthReducer::reduce_mod_pow2(val) {
                        return Some(AstNode {
                            node_type: AstNodeType::FunctionCall {
                                name: "and".to_string(),
                                arguments: vec![
                                    args[0].clone(),
                                    AstNode {
                                        node_type: AstNodeType::Literal { value: mask.to_string() },
                                        line, column,
                                    },
                                ],
                            },
                            line, column,
                        });
                    }
                }
            }
            // x & x = x, x | x = x
            "and" | "or" => {
                if self.nodes_equal(&args[0], &args[1]) {
                    return Some(args[0].clone());
                }
            }
            // x ^ x = 0
            "xor" => {
                if self.nodes_equal(&args[0], &args[1]) {
                    return Some(AstNode {
                        node_type: AstNodeType::Literal { value: "0".to_string() },
                        line,
                        column,
                    });
                }
            }
            _ => {}
        }
        None
    }

    /// Check if two AST nodes are structurally equal
    fn nodes_equal(&self, a: &AstNode, b: &AstNode) -> bool {
        match (&a.node_type, &b.node_type) {
            (AstNodeType::Literal { value: v1 }, AstNodeType::Literal { value: v2 }) => v1 == v2,
            (AstNodeType::Identifier { name: n1 }, AstNodeType::Identifier { name: n2 }) => n1 == n2,
            _ => false,
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

