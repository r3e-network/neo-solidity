impl SemanticAnalyzer {
    pub fn new() -> Self {
        Self
    }

    pub fn analyze(&mut self, ast: &AstNode) -> Result<SemanticResult, CompilerError> {
        let mut warnings = Vec::new();
        let mut suggestions = Vec::new();
        let mut errors = Vec::new();
        let mut security_issues = Vec::new();
        let mut optimization_opportunities = Vec::new();
        let mut hot_paths = Vec::new();

        // Analyze AST structure
        let complexity_metrics = self.analyze_complexity(ast);
        let estimated_gas = self.estimate_gas_usage(ast);

        // Check for common issues
        self.check_undefined_variables(ast, &mut errors);
        self.check_function_signatures(ast, &mut warnings);
        self.check_security_issues(ast, &mut security_issues);
        self.check_optimization_opportunities(ast, &mut optimization_opportunities);
        self.identify_hot_paths(ast, &mut hot_paths);

        // Generate suggestions
        if complexity_metrics.cyclomatic > 10 {
            suggestions.push("Consider breaking down complex functions".to_string());
        }

        if estimated_gas > 100000 {
            suggestions.push("Consider gas optimization techniques".to_string());
        }

        Ok(SemanticResult {
            warnings,
            suggestions,
            errors,
            complexity_metrics,
            security_issues,
            performance_metrics: PerformanceMetrics {
                estimated_gas,
                hot_paths,
                optimization_opportunities,
            },
        })
    }

    fn analyze_complexity(&self, ast: &AstNode) -> ComplexityMetrics {
        let mut function_count = 0;
        let mut cyclomatic = 1; // Base complexity
        let mut max_nesting_depth = 0;

        self.visit_node(ast, &mut |node, depth| {
            match &node.node_type {
                AstNodeType::Function { .. } => function_count += 1,
                AstNodeType::If { .. } => cyclomatic += 1,
                AstNodeType::For { .. } => cyclomatic += 1,
                AstNodeType::Switch { cases, .. } => cyclomatic += cases.len() as u32,
                _ => {}
            }

            if depth > max_nesting_depth {
                max_nesting_depth = depth;
            }
        });

        ComplexityMetrics {
            cyclomatic,
            function_count,
            max_nesting_depth,
        }
    }

    fn estimate_gas_usage(&self, ast: &AstNode) -> u64 {
        let mut gas = 0u64;

        self.visit_node(ast, &mut |node, _depth| {
            match &node.node_type {
                AstNodeType::FunctionCall { name, arguments } => {
                    // Estimate gas based on function type - more accurate NeoVM values
                    gas += match name.as_str() {
                        // Cryptographic operations (expensive on NeoVM)
                        "keccak256" => 700000,
                        "sha256" => 700000,
                        "ripemd160" => 700000,
                        "ecrecover" => 1100000,
                        // Storage operations
                        "sstore" => 1000000,
                        "sload" => 100000,
                        // Memory operations
                        "mstore" | "mload" => 10,
                        // External calls
                        "call" | "delegatecall" | "staticcall" => 10000,
                        // Arithmetic (cheap)
                        "add" | "sub" | "mul" => 10,
                        "div" | "mod" => 10,
                        // Comparison
                        "eq" | "lt" | "gt" | "slt" | "sgt" => 10,
                        // Bitwise
                        "and" | "or" | "xor" => 10,
                        "shl" | "shr" | "sar" => 10,
                        // Other
                        _ => 10,
                    };
                    gas += arguments.len() as u64 * 10;
                }
                AstNodeType::For { .. } => gas += 100,
                AstNodeType::If { .. } => gas += 10,
                AstNodeType::Assignment { .. } => gas += 10,
                _ => gas += 1,
            }
        });

        gas
    }

    fn check_undefined_variables(&self, ast: &AstNode, errors: &mut Vec<String>) {
        // Use a scope stack to properly track variable definitions
        let mut scope_stack: Vec<std::collections::HashSet<String>> =
            vec![std::collections::HashSet::new()];

        self.visit_node(ast, &mut |node, _depth| {
            // Enter new scope for blocks and functions
            match &node.node_type {
                AstNodeType::Function { params, .. } => {
                    // Add function parameters to a new scope
                    let mut function_scope = std::collections::HashSet::new();
                    for param in params {
                        function_scope.insert(param.clone());
                    }
                    scope_stack.push(function_scope);
                }
                AstNodeType::Block { .. } | AstNodeType::Object { .. } => {
                    // Enter a new block scope
                    scope_stack.push(std::collections::HashSet::new());
                }
                _ => {}
            }

            match &node.node_type {
                AstNodeType::Assignment { targets, .. } => {
                    // Add assigned variables to current scope
                    if let Some(current_scope) = scope_stack.last_mut() {
                        for target in targets {
                            current_scope.insert(target.clone());
                        }
                    }
                }
                AstNodeType::Identifier { name } => {
                    // Check if identifier is defined in any scope
                    let is_defined = scope_stack.iter().any(|scope| scope.contains(name));
                    if !is_defined && !self.is_builtin(name) {
                        // Only report if we're not in a function parameter context
                        // (functions create their own scope, so parameters should be there)
                        if !matches!(node.node_type, AstNodeType::Function { .. }) {
                            errors.push(format!("Undefined variable: {}", name));
                        }
                    }
                }
                _ => {}
            }
        });
    }

    fn check_function_signatures(&self, ast: &AstNode, warnings: &mut Vec<String>) {
        self.visit_node(ast, &mut |node, _depth| {
            if let AstNodeType::Function {
                name,
                params,
                returns,
                ..
            } = &node.node_type
            {
                if params.is_empty() && returns.is_empty() {
                    warnings.push(format!(
                        "Function '{}' has no parameters or return values",
                        name
                    ));
                }

                if name.len() > 32 {
                    warnings.push(format!("Function name '{}' is very long", name));
                }
            }
        });
    }

    fn check_security_issues(&self, ast: &AstNode, security_issues: &mut Vec<SecurityIssue>) {
        self.visit_node(ast, &mut |node, _depth| {
            if let AstNodeType::FunctionCall { name, arguments } = &node.node_type {
                match name.as_str() {
                    "div" | "mod" if arguments.len() == 2 => {
                        security_issues.push(SecurityIssue {
                            message: "Potential division by zero".to_string(),
                            severity: Severity::Medium,
                        });
                    }
                    "call" | "delegatecall" => {
                        security_issues.push(SecurityIssue {
                            message: "External call - review for reentrancy".to_string(),
                            severity: Severity::High,
                        });
                    }
                    _ => {}
                }
            }
        });
    }

    fn check_optimization_opportunities(&self, ast: &AstNode, opportunities: &mut Vec<String>) {
        // Track constant expressions
        let mut constant_expressions = std::collections::HashMap::new();

        self.visit_node(ast, &mut |node, _depth| {
            if let AstNodeType::FunctionCall { name, arguments } = &node.node_type {
                if arguments
                    .iter()
                    .all(|arg| matches!(arg.node_type, AstNodeType::Literal { .. }))
                {
                    let expr_key = format!("{}({})", name, arguments.len());
                    *constant_expressions.entry(expr_key.clone()).or_insert(0) += 1;

                    if constant_expressions[&expr_key] > 1 {
                        opportunities.push(format!(
                            "Constant expression '{}' appears multiple times",
                            expr_key
                        ));
                    }
                }
            }
        });
    }

    fn identify_hot_paths(&self, ast: &AstNode, hot_paths: &mut Vec<String>) {
        self.visit_node(ast, &mut |node, depth| match &node.node_type {
            AstNodeType::For { .. } if depth > 2 => {
                hot_paths.push("Nested loop detected".to_string());
            }
            AstNodeType::FunctionCall { name, .. } if name == "keccak256" => {
                hot_paths.push("Expensive cryptographic operation".to_string());
            }
            _ => {}
        });
    }
}
