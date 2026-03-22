use std::collections::HashMap;

use crate::error::CompilerError;
use crate::parser::{AstNode, AstNodeType};

use super::types::{InlineCandidate, Optimizer};

impl Optimizer {
    pub(super) fn function_inlining(&mut self, ast: AstNode) -> Result<AstNode, CompilerError> {
        // Collect small functions for inlining
        let mut inline_candidates: HashMap<String, InlineCandidate> = HashMap::new();
        self.collect_inline_candidates(&ast, &mut inline_candidates);

        // Count call sites to make better inlining decisions
        let call_counts = self.count_call_sites(&ast);

        // Filter candidates based on call frequency and cost
        let filtered_candidates: HashMap<String, InlineCandidate> = inline_candidates
            .into_iter()
            .filter(|(name, candidate)| {
                let calls = call_counts.get(name).copied().unwrap_or(0);
                // Inline if: small function OR called only once OR total cost acceptable
                candidate.cost <= 10 || calls <= 1 || candidate.cost * calls <= self.inline_threshold * 2
            })
            .collect();

        // Run multiple inlining passes for recursive inlining
        let mut result = ast;
        for _ in 0..3 {
            let before = self.stats.inlined_functions;
            result = self.inline_functions_recursive(result, &filtered_candidates);
            if self.stats.inlined_functions == before {
                break; // No more inlining possible
            }
        }

        Ok(result)
    }

    /// Count how many times each function is called
    fn count_call_sites(&self, node: &AstNode) -> HashMap<String, usize> {
        let mut counts = HashMap::new();
        self.count_calls_recursive(node, &mut counts);
        counts
    }

    #[allow(clippy::only_used_in_recursion)]
    fn count_calls_recursive(&self, node: &AstNode, counts: &mut HashMap<String, usize>) {
        match &node.node_type {
            AstNodeType::FunctionCall { name, arguments } => {
                *counts.entry(name.clone()).or_insert(0) += 1;
                for arg in arguments {
                    self.count_calls_recursive(arg, counts);
                }
            }
            AstNodeType::Object { statements } | AstNodeType::Block { statements } => {
                for stmt in statements {
                    self.count_calls_recursive(stmt, counts);
                }
            }
            AstNodeType::If { condition, then_branch, else_branch } => {
                self.count_calls_recursive(condition, counts);
                self.count_calls_recursive(then_branch, counts);
                if let Some(eb) = else_branch {
                    self.count_calls_recursive(eb, counts);
                }
            }
            AstNodeType::Function { body, .. } => {
                self.count_calls_recursive(body, counts);
            }
            _ => {}
        }
    }

    fn collect_inline_candidates(
        &self,
        node: &AstNode,
        candidates: &mut HashMap<String, InlineCandidate>,
    ) {
        if let AstNodeType::Function { name, params, body, .. } = &node.node_type {
            // More generous inlining criteria
            if params.len() <= 4 && self.is_inlineable_function(body, name) {
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

        match &node.node_type {
            AstNodeType::Object { statements } | AstNodeType::Block { statements } => {
                for stmt in statements {
                    self.collect_inline_candidates(stmt, candidates);
                }
            }
            _ => {}
        }
    }

    /// Check if a function is suitable for inlining
    fn is_inlineable_function(&self, body: &AstNode, name: &str) -> bool {
        // Don't inline recursive functions
        if self.contains_call_to(body, name) {
            return false;
        }
        self.is_simple_function(body)
    }

    /// Check if body contains a call to the named function (recursion check)
    #[allow(clippy::only_used_in_recursion)]
    fn contains_call_to(&self, node: &AstNode, target: &str) -> bool {
        match &node.node_type {
            AstNodeType::FunctionCall { name, arguments } => {
                if name == target {
                    return true;
                }
                arguments.iter().any(|a| self.contains_call_to(a, target))
            }
            AstNodeType::Block { statements } | AstNodeType::Object { statements } => {
                statements.iter().any(|s| self.contains_call_to(s, target))
            }
            AstNodeType::If { condition, then_branch, else_branch } => {
                self.contains_call_to(condition, target) ||
                self.contains_call_to(then_branch, target) ||
                else_branch.as_ref().is_some_and(|eb| self.contains_call_to(eb, target))
            }
            _ => false,
        }
    }

    fn is_simple_function(&self, body: &AstNode) -> bool {
        match &body.node_type {
            AstNodeType::Block { statements } => statements.len() <= 5,
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
                // First recursively process arguments
                let optimized_args: Vec<AstNode> = arguments
                    .into_iter()
                    .map(|arg| self.inline_functions_recursive(arg, candidates))
                    .collect();

                if let Some(candidate) = candidates.get(&name) {
                    if optimized_args.len() == candidate.params.len() {
                        let substitutions: HashMap<String, AstNode> = candidate
                            .params
                            .iter()
                            .zip(optimized_args)
                            .map(|(param, arg)| (param.clone(), arg))
                            .collect();

                        self.stats.inlined_functions += 1;
                        Self::substitute_parameters(candidate.body.clone(), &substitutions)
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
                let optimized = statements
                    .into_iter()
                    .map(|stmt| self.inline_functions_recursive(stmt, candidates))
                    .collect();
                AstNode {
                    node_type: AstNodeType::Object { statements: optimized },
                    line: node.line,
                    column: node.column,
                }
            }
            AstNodeType::Block { statements } => {
                let optimized = statements
                    .into_iter()
                    .map(|stmt| self.inline_functions_recursive(stmt, candidates))
                    .collect();
                AstNode {
                    node_type: AstNodeType::Block { statements: optimized },
                    line: node.line,
                    column: node.column,
                }
            }
            AstNodeType::If { condition, then_branch, else_branch } => {
                AstNode {
                    node_type: AstNodeType::If {
                        condition: Box::new(self.inline_functions_recursive(*condition, candidates)),
                        then_branch: Box::new(self.inline_functions_recursive(*then_branch, candidates)),
                        else_branch: else_branch.map(|eb|
                            Box::new(self.inline_functions_recursive(*eb, candidates))
                        ),
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
                    node_type: AstNodeType::FunctionCall { name, arguments: substituted_args },
                    line: node.line,
                    column: node.column,
                }
            }
            AstNodeType::Block { statements } => {
                let substituted = statements
                    .into_iter()
                    .map(|stmt| Self::substitute_parameters(stmt, substitutions))
                    .collect();
                AstNode {
                    node_type: AstNodeType::Block { statements: substituted },
                    line: node.line,
                    column: node.column,
                }
            }
            AstNodeType::Object { statements } => {
                let substituted = statements
                    .into_iter()
                    .map(|stmt| Self::substitute_parameters(stmt, substitutions))
                    .collect();
                AstNode {
                    node_type: AstNodeType::Object { statements: substituted },
                    line: node.line,
                    column: node.column,
                }
            }
            AstNodeType::If { condition, then_branch, else_branch } => {
                AstNode {
                    node_type: AstNodeType::If {
                        condition: Box::new(Self::substitute_parameters(*condition, substitutions)),
                        then_branch: Box::new(Self::substitute_parameters(*then_branch, substitutions)),
                        else_branch: else_branch.map(|eb|
                            Box::new(Self::substitute_parameters(*eb, substitutions))
                        ),
                    },
                    line: node.line,
                    column: node.column,
                }
            }
            _ => node,
        }
    }
}
