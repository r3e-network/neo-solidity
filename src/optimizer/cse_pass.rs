impl Optimizer {
    /// Common subexpression elimination pass
    ///
    /// Walks the AST and replaces duplicate pure expressions with references
    /// to previously computed values, reducing redundant computation.
    pub fn common_subexpression_elimination(
        &mut self,
        ast: AstNode,
    ) -> Result<AstNode, CompilerError> {
        let mut cse = CseOptimizer::new();
        let result = self.cse_walk(ast, &mut cse);
        Ok(result)
    }

    fn cse_walk(&mut self, node: AstNode, cse: &mut CseOptimizer) -> AstNode {
        match node.node_type {
            AstNodeType::Block { statements } => {
                let optimized: Vec<AstNode> = statements
                    .into_iter()
                    .map(|s| self.cse_walk(s, cse))
                    .collect();
                AstNode {
                    node_type: AstNodeType::Block {
                        statements: optimized,
                    },
                    ..node
                }
            }
            AstNodeType::Object { statements } => {
                let optimized: Vec<AstNode> = statements
                    .into_iter()
                    .map(|s| self.cse_walk(s, cse))
                    .collect();
                AstNode {
                    node_type: AstNodeType::Object {
                        statements: optimized,
                    },
                    ..node
                }
            }
            AstNodeType::FunctionCall { ref name, ref arguments } => {
                // Check if this pure expression was already computed
                if let Some(cached_var) = cse.lookup(&node) {
                    self.stats.eliminated_instructions += 1;
                    return AstNode {
                        node_type: AstNodeType::Identifier {
                            name: cached_var.clone(),
                        },
                        line: node.line,
                        column: node.column,
                    };
                }

                // Recursively optimize arguments
                let name = name.clone();
                let opt_args: Vec<AstNode> = arguments
                    .clone()
                    .into_iter()
                    .map(|a| self.cse_walk(a, cse))
                    .collect();

                let optimized = AstNode {
                    node_type: AstNodeType::FunctionCall {
                        name,
                        arguments: opt_args,
                    },
                    line: node.line,
                    column: node.column,
                };

                // Register for future dedup
                cse.register(&optimized);
                optimized
            }
            AstNodeType::If {
                condition,
                then_branch,
                else_branch,
            } => {
                let cond = self.cse_walk(*condition, cse);
                let then_b = self.cse_walk(*then_branch, cse);
                let else_b = else_branch.map(|eb| Box::new(self.cse_walk(*eb, cse)));
                AstNode {
                    node_type: AstNodeType::If {
                        condition: Box::new(cond),
                        then_branch: Box::new(then_b),
                        else_branch: else_b,
                    },
                    ..node
                }
            }
            _ => node,
        }
    }
}
