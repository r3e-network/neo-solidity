impl Optimizer {
    fn function_inlining(&mut self, ast: AstNode) -> Result<AstNode, CompilerError> {
        // Collect small functions for inlining (now includes parameter names)
        let mut inline_candidates: HashMap<String, InlineCandidate> = HashMap::new();
        self.collect_inline_candidates(&ast, &mut inline_candidates);

        Ok(self.inline_functions_recursive(ast, &inline_candidates))
    }

    fn collect_inline_candidates(
        &self,
        node: &AstNode,
        candidates: &mut HashMap<String, InlineCandidate>,
    ) {
        if let AstNodeType::Function {
            name, params, body, ..
        } = &node.node_type
        {
            // Only inline simple functions with few parameters
            if params.len() <= 2 && self.is_simple_function(body) {
                let cost = self.estimate_inline_cost(body);
                if cost <= self.inline_threshold {
                    candidates.insert(
                        name.clone(),
                        InlineCandidate {
                            params: params.clone(),
                            body: (**body).clone(),
                            cost,
                        },
                    );
                }
            }
        }

        // Recursively collect from child nodes
        match &node.node_type {
            AstNodeType::Object { statements } | AstNodeType::Block { statements } => {
                for stmt in statements {
                    self.collect_inline_candidates(stmt, candidates);
                }
            }
            _ => {}
        }
    }

    fn is_simple_function(&self, body: &AstNode) -> bool {
        match &body.node_type {
            AstNodeType::Block { statements } => statements.len() <= 3,
            _ => true,
        }
    }

    /// Estimate the cost of inlining a function body
    fn estimate_inline_cost(&self, body: &AstNode) -> usize {
        self.count_nodes(body)
    }

    fn inline_functions_recursive(
        &mut self,
        node: AstNode,
        candidates: &HashMap<String, InlineCandidate>,
    ) -> AstNode {
        match node.node_type {
            AstNodeType::FunctionCall { name, arguments } => {
                if let Some(candidate) = candidates.get(&name) {
                    // Validate argument count matches parameter count
                    if arguments.len() == candidate.params.len() {
                        // Build substitution map: parameter name -> argument expression
                        let substitutions: HashMap<String, AstNode> = candidate
                            .params
                            .iter()
                            .zip(arguments)
                            .map(|(param, arg)| (param.clone(), arg))
                            .collect();

                        // Inline the function with parameter substitution
                        self.stats.inlined_functions += 1;
                        Self::substitute_parameters(candidate.body.clone(), &substitutions)
                    } else {
                        // Argument count mismatch - don't inline, just optimize args
                        let optimized_args = arguments
                            .into_iter()
                            .map(|arg| self.inline_functions_recursive(arg, candidates))
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
                } else {
                    // Not an inline candidate - recursively optimize arguments
                    let optimized_args = arguments
                        .into_iter()
                        .map(|arg| self.inline_functions_recursive(arg, candidates))
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
                    .map(|stmt| self.inline_functions_recursive(stmt, candidates))
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
                    .map(|stmt| self.inline_functions_recursive(stmt, candidates))
                    .collect();

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

    /// Substitute parameter references with actual argument expressions
    fn substitute_parameters(node: AstNode, substitutions: &HashMap<String, AstNode>) -> AstNode {
        match node.node_type {
            AstNodeType::Identifier { ref name } => {
                // If this identifier is a parameter, replace it with the argument
                if let Some(replacement) = substitutions.get(name) {
                    replacement.clone()
                } else {
                    node
                }
            }
            AstNodeType::FunctionCall { name, arguments } => {
                let substituted_args = arguments
                    .into_iter()
                    .map(|arg| Self::substitute_parameters(arg, substitutions))
                    .collect();

                AstNode {
                    node_type: AstNodeType::FunctionCall {
                        name,
                        arguments: substituted_args,
                    },
                    line: node.line,
                    column: node.column,
                }
            }
            AstNodeType::Block { statements } => {
                let substituted_stmts = statements
                    .into_iter()
                    .map(|stmt| Self::substitute_parameters(stmt, substitutions))
                    .collect();

                AstNode {
                    node_type: AstNodeType::Block {
                        statements: substituted_stmts,
                    },
                    line: node.line,
                    column: node.column,
                }
            }
            AstNodeType::Object { statements } => {
                let substituted_stmts = statements
                    .into_iter()
                    .map(|stmt| Self::substitute_parameters(stmt, substitutions))
                    .collect();

                AstNode {
                    node_type: AstNodeType::Object {
                        statements: substituted_stmts,
                    },
                    line: node.line,
                    column: node.column,
                }
            }
            // For other node types, return as-is (could be extended for more complex AST)
            _ => node,
        }
    }
}

