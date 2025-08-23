use crate::error::CompilerError;
use crate::parser::{AstNode, AstNodeType};

pub struct SemanticResult {
    pub warnings: Vec<String>,
    pub suggestions: Vec<String>,
    pub errors: Vec<String>,
    pub complexity_metrics: ComplexityMetrics,
    pub security_issues: Vec<SecurityIssue>,
    pub performance_metrics: PerformanceMetrics,
}

pub struct ComplexityMetrics {
    pub cyclomatic: u32,
    pub function_count: u32,
    pub max_nesting_depth: u32,
}

pub struct SecurityIssue {
    pub message: String,
    pub severity: Severity,
}

#[derive(Debug)]
pub enum Severity {
    Low,
    Medium,
    High,
    Critical,
}

pub struct PerformanceMetrics {
    pub estimated_gas: u64,
    pub hot_paths: Vec<String>,
    pub optimization_opportunities: Vec<String>,
}

pub struct SemanticAnalyzer;

impl Default for SemanticAnalyzer {
    fn default() -> Self {
        Self
    }
}

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
        let mut gas = 0;

        self.visit_node(ast, &mut |node, _depth| {
            match &node.node_type {
                AstNodeType::FunctionCall { name, arguments } => {
                    // Estimate gas based on function type
                    gas += match name.as_str() {
                        "keccak256" => 30,
                        "sha256" => 60,
                        "ecrecover" => 3000,
                        "sstore" => 20000,
                        "sload" => 800,
                        "mstore" | "mload" => 3,
                        _ => 3, // Basic operation
                    };
                    gas += arguments.len() as u64 * 3; // Argument handling
                }
                AstNodeType::For { .. } => gas += 100, // Loop overhead
                AstNodeType::If { .. } => gas += 10,   // Conditional overhead
                _ => gas += 1,                         // Basic instruction
            }
        });

        gas
    }

    fn check_undefined_variables(&self, ast: &AstNode, errors: &mut Vec<String>) {
        let mut defined_vars = std::collections::HashSet::new();

        self.visit_node(ast, &mut |node, _depth| match &node.node_type {
            AstNodeType::Assignment { targets, .. } => {
                for target in targets {
                    defined_vars.insert(target.clone());
                }
            }
            AstNodeType::Identifier { name } => {
                if !defined_vars.contains(name) && !self.is_builtin(name) {
                    errors.push(format!("Undefined variable: {}", name));
                }
            }
            _ => {}
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

    fn visit_node<F>(&self, node: &AstNode, visitor: &mut F)
    where
        F: FnMut(&AstNode, u32),
    {
        self.visit_node_recursive(node, visitor, 0);
    }

    #[allow(clippy::only_used_in_recursion)]
    fn visit_node_recursive<F>(&self, node: &AstNode, visitor: &mut F, depth: u32)
    where
        F: FnMut(&AstNode, u32),
    {
        visitor(node, depth);

        match &node.node_type {
            AstNodeType::Object { statements } | AstNodeType::Block { statements } => {
                for stmt in statements {
                    self.visit_node_recursive(stmt, visitor, depth + 1);
                }
            }
            AstNodeType::Function { body, .. } => {
                self.visit_node_recursive(body, visitor, depth + 1);
            }
            AstNodeType::If {
                condition,
                then_branch,
                else_branch,
            } => {
                self.visit_node_recursive(condition, visitor, depth + 1);
                self.visit_node_recursive(then_branch, visitor, depth + 1);
                if let Some(else_stmt) = else_branch {
                    self.visit_node_recursive(else_stmt, visitor, depth + 1);
                }
            }
            AstNodeType::For {
                init,
                condition,
                update,
                body,
            } => {
                if let Some(init_stmt) = init {
                    self.visit_node_recursive(init_stmt, visitor, depth + 1);
                }
                self.visit_node_recursive(condition, visitor, depth + 1);
                if let Some(update_stmt) = update {
                    self.visit_node_recursive(update_stmt, visitor, depth + 1);
                }
                self.visit_node_recursive(body, visitor, depth + 1);
            }
            AstNodeType::Switch {
                expression,
                cases,
                default,
            } => {
                self.visit_node_recursive(expression, visitor, depth + 1);
                for case in cases {
                    self.visit_node_recursive(&case.value, visitor, depth + 1);
                    self.visit_node_recursive(&case.body, visitor, depth + 1);
                }
                if let Some(default_stmt) = default {
                    self.visit_node_recursive(default_stmt, visitor, depth + 1);
                }
            }
            AstNodeType::FunctionCall { arguments, .. } => {
                for arg in arguments {
                    self.visit_node_recursive(arg, visitor, depth + 1);
                }
            }
            AstNodeType::Assignment { value, .. } => {
                self.visit_node_recursive(value, visitor, depth + 1);
            }
            _ => {}
        }
    }

    fn is_builtin(&self, name: &str) -> bool {
        matches!(
            name,
            "add"
                | "sub"
                | "mul"
                | "div"
                | "mod"
                | "eq"
                | "lt"
                | "gt"
                | "iszero"
                | "and"
                | "or"
                | "xor"
                | "not"
                | "byte"
                | "shl"
                | "shr"
                | "sar"
                | "keccak256"
                | "sha256"
                | "ripemd160"
                | "ecrecover"
                | "mload"
                | "mstore"
                | "sload"
                | "sstore"
                | "caller"
                | "callvalue"
                | "calldataload"
                | "calldatasize"
                | "gas"
                | "gasprice"
                | "origin"
                | "address"
                | "balance"
                | "timestamp"
                | "number"
                | "blockhash"
                | "coinbase"
                | "log0"
                | "log1"
                | "log2"
                | "log3"
                | "log4"
                | "call"
                | "delegatecall"
                | "staticcall"
                | "return"
                | "revert"
        )
    }
}
