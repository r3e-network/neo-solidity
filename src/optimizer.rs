use crate::error::CompilerError;
use crate::parser::{AstNode, AstNodeType};
use std::collections::HashMap;

pub struct Optimizer {
    level: u8,
    stats: OptimizationStats,
}

pub struct OptimizationStats {
    pub eliminated_instructions: u32,
    pub inlined_functions: u32,
    pub folded_constants: u32,
}

impl Optimizer {
    pub fn new(level: u8) -> Self {
        Self { 
            level,
            stats: OptimizationStats {
                eliminated_instructions: 0,
                inlined_functions: 0,
                folded_constants: 0,
            }
        }
    }
    
    pub fn optimize(&mut self, mut ast: AstNode) -> Result<AstNode, CompilerError> {
        match self.level {
            0 => Ok(ast), // No optimization
            1 => {
                ast = self.constant_folding(ast)?;
                Ok(ast)
            }
            2 => {
                ast = self.constant_folding(ast)?;
                ast = self.dead_code_elimination(ast)?;
                Ok(ast)
            }
            3 => {
                ast = self.constant_folding(ast)?;
                ast = self.dead_code_elimination(ast)?;
                ast = self.function_inlining(ast)?;
                Ok(ast)
            }
            _ => Err(CompilerError::CodegenError("Invalid optimization level".to_string())),
        }
    }
    
    pub fn get_stats(&self) -> &OptimizationStats {
        &self.stats
    }
    
    fn constant_folding(&mut self, ast: AstNode) -> Result<AstNode, CompilerError> {
        Ok(self.fold_constants_recursive(ast))
    }
    
    fn fold_constants_recursive(&mut self, node: AstNode) -> AstNode {
        let optimized_node = match node.node_type {
            AstNodeType::FunctionCall { name, arguments } => {
                // Fold constant arithmetic operations
                if let Some(result) = self.evaluate_constant_expression(&name, &arguments) {
                    self.stats.folded_constants += 1;
                    AstNode {
                        node_type: AstNodeType::Literal { value: result.to_string() },
                        line: node.line,
                        column: node.column,
                    }
                } else {
                    // Recursively optimize arguments
                    let optimized_args = arguments.into_iter()
                        .map(|arg| self.fold_constants_recursive(arg))
                        .collect();
                    
                    AstNode {
                        node_type: AstNodeType::FunctionCall { name, arguments: optimized_args },
                        line: node.line,
                        column: node.column,
                    }
                }
            }
            AstNodeType::Object { statements } => {
                let optimized_statements = statements.into_iter()
                    .map(|stmt| self.fold_constants_recursive(stmt))
                    .collect();
                
                AstNode {
                    node_type: AstNodeType::Object { statements: optimized_statements },
                    line: node.line,
                    column: node.column,
                }
            }
            AstNodeType::Block { statements } => {
                let optimized_statements = statements.into_iter()
                    .map(|stmt| self.fold_constants_recursive(stmt))
                    .collect();
                
                AstNode {
                    node_type: AstNodeType::Block { statements: optimized_statements },
                    line: node.line,
                    column: node.column,
                }
            }
            _ => node, // Return unchanged for other node types
        };
        
        optimized_node
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
            if value.starts_with("0x") {
                u64::from_str_radix(&value[2..], 16).ok()
            } else {
                value.parse::<u64>().ok()
            }
        } else {
            None
        }
    }
    
    fn dead_code_elimination(&mut self, ast: AstNode) -> Result<AstNode, CompilerError> {
        Ok(self.eliminate_dead_code_recursive(ast, false))
    }
    
    fn eliminate_dead_code_recursive(&mut self, node: AstNode, after_return: bool) -> AstNode {
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
                    node_type: AstNodeType::Object { statements: optimized_statements },
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
                    node_type: AstNodeType::Block { statements: optimized_statements },
                    line: node.line,
                    column: node.column,
                }
            }
            _ => node,
        }
    }
    
    fn function_inlining(&mut self, ast: AstNode) -> Result<AstNode, CompilerError> {
        // Collect small functions for inlining
        let mut inline_candidates = HashMap::new();
        self.collect_inline_candidates(&ast, &mut inline_candidates);
        
        Ok(self.inline_functions_recursive(ast, &inline_candidates))
    }
    
    fn collect_inline_candidates(&self, node: &AstNode, candidates: &mut HashMap<String, AstNode>) {
        if let AstNodeType::Function { name, params, body, .. } = &node.node_type {
            // Only inline simple functions with few parameters
            if params.len() <= 2 && self.is_simple_function(body) {
                candidates.insert(name.clone(), (**body).clone());
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
    
    fn inline_functions_recursive(&mut self, node: AstNode, candidates: &HashMap<String, AstNode>) -> AstNode {
        match node.node_type {
            AstNodeType::FunctionCall { name, arguments } => {
                if let Some(function_body) = candidates.get(&name) {
                    // Inline the function
                    self.stats.inlined_functions += 1;
                    function_body.clone()
                } else {
                    // Recursively optimize arguments
                    let optimized_args = arguments.into_iter()
                        .map(|arg| self.inline_functions_recursive(arg, candidates))
                        .collect();
                    
                    AstNode {
                        node_type: AstNodeType::FunctionCall { name, arguments: optimized_args },
                        line: node.line,
                        column: node.column,
                    }
                }
            }
            AstNodeType::Object { statements } => {
                let optimized_statements = statements.into_iter()
                    .map(|stmt| self.inline_functions_recursive(stmt, candidates))
                    .collect();
                
                AstNode {
                    node_type: AstNodeType::Object { statements: optimized_statements },
                    line: node.line,
                    column: node.column,
                }
            }
            _ => node,
        }
    }
}
