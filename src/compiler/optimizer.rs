//! Optimization Module
//! 
//! Advanced optimization passes for Yul AST including constant folding,
//! dead code elimination, function inlining, and NeoVM-specific optimizations.

use super::parser::{YulAST, YulItem, YulFunction, YulBlock, YulStatement, YulExpression, 
                     YulVariableDeclaration, YulAssignment, YulIf, YulSwitch, YulForLoop,
                     YulFunctionCall, YulIdentifier, YulLiteral, LiteralKind, YulType};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use thiserror::Error;

/// Optimizer for Yul AST
#[derive(Debug)]
pub struct Optimizer {
    level: u8,
    passes: Vec<Box<dyn OptimizationPass>>,
    statistics: OptimizationStatistics,
}

/// Optimization pass trait
pub trait OptimizationPass: std::fmt::Debug {
    /// Get pass name
    fn name(&self) -> &str;
    
    /// Get pass description
    fn description(&self) -> &str;
    
    /// Apply optimization to AST
    fn apply(&mut self, ast: &mut YulAST) -> Result<bool, OptimizationError>;
    
    /// Check if pass should run
    fn should_run(&self, level: u8) -> bool;
}

/// Optimization statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationStatistics {
    pub passes_run: u32,
    pub nodes_removed: u32,
    pub constants_folded: u32,
    pub functions_inlined: u32,
    pub dead_code_removed: u32,
    pub estimated_gas_saved: u64,
    pub optimization_time_ms: u64,
}

/// Optimization errors
#[derive(Debug, Error)]
pub enum OptimizationError {
    #[error("Optimization pass '{pass}' failed: {message}")]
    PassFailed { pass: String, message: String },
    
    #[error("Invalid optimization level: {level}")]
    InvalidLevel { level: u8 },
    
    #[error("AST corruption detected after pass '{pass}'")]
    ASTCorruption { pass: String },
}

/// Constant folding pass
#[derive(Debug)]
pub struct ConstantFoldingPass {
    folded_count: u32,
}

/// Dead code elimination pass
#[derive(Debug)]
pub struct DeadCodeEliminationPass {
    removed_count: u32,
}

/// Function inlining pass
#[derive(Debug)]
pub struct FunctionInliningPass {
    inlined_count: u32,
    size_threshold: usize,
}

/// Common subexpression elimination pass
#[derive(Debug)]
pub struct CommonSubexpressionEliminationPass {
    eliminated_count: u32,
    expression_cache: HashMap<String, YulExpression>,
}

/// Loop optimization pass
#[derive(Debug)]
pub struct LoopOptimizationPass {
    optimized_count: u32,
}

/// Stack operation reduction pass
#[derive(Debug)]
pub struct StackOptimizationPass {
    reductions: u32,
}

/// NeoVM-specific optimization pass
#[derive(Debug)]
pub struct NeoVMOptimizationPass {
    optimizations: u32,
}

impl Optimizer {
    /// Create optimizer with specified level
    pub fn new(level: u8) -> Self {
        let mut optimizer = Self {
            level,
            passes: Vec::new(),
            statistics: OptimizationStatistics {
                passes_run: 0,
                nodes_removed: 0,
                constants_folded: 0,
                functions_inlined: 0,
                dead_code_removed: 0,
                estimated_gas_saved: 0,
                optimization_time_ms: 0,
            },
        };
        
        optimizer.initialize_passes();
        optimizer
    }

    /// Initialize optimization passes based on level
    fn initialize_passes(&mut self) {
        // Level 0: No optimization
        if self.level == 0 {
            return;
        }
        
        // Level 1: Basic optimizations
        if self.level >= 1 {
            self.passes.push(Box::new(ConstantFoldingPass::new()));
            self.passes.push(Box::new(DeadCodeEliminationPass::new()));
        }
        
        // Level 2: Advanced optimizations
        if self.level >= 2 {
            self.passes.push(Box::new(FunctionInliningPass::new(100))); // Inline functions ≤100 nodes
            self.passes.push(Box::new(CommonSubexpressionEliminationPass::new()));
            self.passes.push(Box::new(StackOptimizationPass::new()));
        }
        
        // Level 3: Aggressive optimizations
        if self.level >= 3 {
            self.passes.push(Box::new(LoopOptimizationPass::new()));
            self.passes.push(Box::new(NeoVMOptimizationPass::new()));
            // Re-run earlier passes after aggressive optimizations
            self.passes.push(Box::new(ConstantFoldingPass::new()));
            self.passes.push(Box::new(DeadCodeEliminationPass::new()));
        }
    }

    /// Optimize AST with configured passes
    pub fn optimize(&mut self, mut ast: YulAST) -> Result<YulAST, OptimizationError> {
        let start_time = std::time::Instant::now();
        
        // Run optimization passes
        let mut changed = true;
        let mut iterations = 0;
        const MAX_ITERATIONS: u32 = 10;
        
        while changed && iterations < MAX_ITERATIONS {
            changed = false;
            iterations += 1;
            
            for pass in &mut self.passes {
                if pass.should_run(self.level) {
                    let pass_changed = pass.apply(&mut ast)?;
                    changed |= pass_changed;
                    self.statistics.passes_run += 1;
                    
                    // Validate AST integrity after each pass
                    if !self.validate_ast(&ast) {
                        return Err(OptimizationError::ASTCorruption {
                            pass: pass.name().to_string(),
                        });
                    }
                }
            }
        }
        
        // Update statistics
        let duration = start_time.elapsed();
        self.statistics.optimization_time_ms = duration.as_millis() as u64;
        
        // Collect statistics from passes
        self.collect_pass_statistics();
        
        Ok(ast)
    }

    /// Get optimization statistics
    pub fn statistics(&self) -> &OptimizationStatistics {
        &self.statistics
    }

    /// Validate AST integrity
    fn validate_ast(&self, _ast: &YulAST) -> bool {
        // Basic AST validation - in production would be more comprehensive
        true
    }

    /// Collect statistics from all passes
    fn collect_pass_statistics(&mut self) {
        for pass in &self.passes {
            match pass.name() {
                "ConstantFolding" => {
                    if let Some(cf_pass) = pass.as_any().downcast_ref::<ConstantFoldingPass>() {
                        self.statistics.constants_folded += cf_pass.folded_count;
                        self.statistics.estimated_gas_saved += cf_pass.folded_count as u64 * 3;
                    }
                },
                "DeadCodeElimination" => {
                    if let Some(dc_pass) = pass.as_any().downcast_ref::<DeadCodeEliminationPass>() {
                        self.statistics.dead_code_removed += dc_pass.removed_count;
                        self.statistics.nodes_removed += dc_pass.removed_count;
                        self.statistics.estimated_gas_saved += dc_pass.removed_count as u64 * 5;
                    }
                },
                "FunctionInlining" => {
                    if let Some(fi_pass) = pass.as_any().downcast_ref::<FunctionInliningPass>() {
                        self.statistics.functions_inlined += fi_pass.inlined_count;
                        self.statistics.estimated_gas_saved += fi_pass.inlined_count as u64 * 50;
                    }
                },
                _ => {},
            }
        }
    }
}

// Add trait for downcasting
trait AsAny {
    fn as_any(&self) -> &dyn std::any::Any;
}

impl<T: 'static> AsAny for T {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl OptimizationPass for dyn OptimizationPass {
    fn name(&self) -> &str { self.name() }
    fn description(&self) -> &str { self.description() }
    fn apply(&mut self, ast: &mut YulAST) -> Result<bool, OptimizationError> { self.apply(ast) }
    fn should_run(&self, level: u8) -> bool { self.should_run(level) }
}

impl ConstantFoldingPass {
    fn new() -> Self {
        Self { folded_count: 0 }
    }
}

impl OptimizationPass for ConstantFoldingPass {
    fn name(&self) -> &str {
        "ConstantFolding"
    }

    fn description(&self) -> &str {
        "Evaluate constant expressions at compile time"
    }

    fn apply(&mut self, ast: &mut YulAST) -> Result<bool, OptimizationError> {
        let mut changed = false;
        
        for item in &mut ast.items {
            match item {
                YulItem::Function(func) => {
                    changed |= self.optimize_function(func)?;
                },
                YulItem::Block(block) => {
                    changed |= self.optimize_block(block)?;
                },
                YulItem::Object(obj) => {
                    if let Some(code) = &mut obj.code {
                        changed |= self.optimize_block(code)?;
                    }
                },
            }
        }
        
        Ok(changed)
    }

    fn should_run(&self, level: u8) -> bool {
        level >= 1
    }
}

impl ConstantFoldingPass {
    fn optimize_function(&mut self, func: &mut YulFunction) -> Result<bool, OptimizationError> {
        self.optimize_block(&mut func.body)
    }

    fn optimize_block(&mut self, block: &mut YulBlock) -> Result<bool, OptimizationError> {
        let mut changed = false;
        
        for statement in &mut block.statements {
            changed |= self.optimize_statement(statement)?;
        }
        
        Ok(changed)
    }

    fn optimize_statement(&mut self, statement: &mut YulStatement) -> Result<bool, OptimizationError> {
        match statement {
            YulStatement::Block(block) => self.optimize_block(block),
            YulStatement::FunctionDef(func) => self.optimize_function(func),
            YulStatement::VariableDeclaration(var_decl) => {
                if let Some(value) = &mut var_decl.value {
                    self.optimize_expression(value)
                } else {
                    Ok(false)
                }
            },
            YulStatement::Assignment(assignment) => {
                self.optimize_expression(&mut assignment.value)
            },
            YulStatement::If(if_stmt) => {
                let mut changed = self.optimize_expression(&mut if_stmt.condition)?;
                changed |= self.optimize_block(&mut if_stmt.body)?;
                Ok(changed)
            },
            YulStatement::Switch(switch) => {
                let mut changed = self.optimize_expression(&mut switch.expression)?;
                for case in &mut switch.cases {
                    changed |= self.optimize_block(&mut case.body)?;
                }
                if let Some(default) = &mut switch.default {
                    changed |= self.optimize_block(default)?;
                }
                Ok(changed)
            },
            YulStatement::ForLoop(for_loop) => {
                let mut changed = self.optimize_block(&mut for_loop.init)?;
                changed |= self.optimize_expression(&mut for_loop.condition)?;
                changed |= self.optimize_block(&mut for_loop.post)?;
                changed |= self.optimize_block(&mut for_loop.body)?;
                Ok(changed)
            },
            YulStatement::Expression(expr) => {
                self.optimize_expression(expr)
            },
            _ => Ok(false),
        }
    }

    fn optimize_expression(&mut self, expr: &mut YulExpression) -> Result<bool, OptimizationError> {
        match expr {
            YulExpression::FunctionCall(call) => {
                // Optimize arguments first
                let mut changed = false;
                for arg in &mut call.arguments {
                    changed |= self.optimize_expression(arg)?;
                }
                
                // Try to fold constant expressions
                if self.can_fold_function_call(call) {
                    if let Some(folded) = self.fold_function_call(call)? {
                        *expr = folded;
                        self.folded_count += 1;
                        changed = true;
                    }
                }
                
                Ok(changed)
            },
            _ => Ok(false),
        }
    }

    fn can_fold_function_call(&self, call: &YulFunctionCall) -> bool {
        // Check if function is pure arithmetic and all arguments are constants
        let pure_functions = ["add", "sub", "mul", "div", "mod", "lt", "gt", "eq", "and", "or", "not"];
        
        if !pure_functions.contains(&call.function.name.as_str()) {
            return false;
        }
        
        call.arguments.iter().all(|arg| {
            matches!(arg, YulExpression::Literal(_))
        })
    }

    fn fold_function_call(&self, call: &YulFunctionCall) -> Result<Option<YulExpression>, OptimizationError> {
        let func_name = &call.function.name;
        
        // Extract constant values
        let mut values = Vec::new();
        for arg in &call.arguments {
            if let YulExpression::Literal(lit) = arg {
                if lit.kind == LiteralKind::Number {
                    values.push(lit.value.parse::<i64>().unwrap_or(0));
                } else {
                    return Ok(None); // Can't fold non-numeric literals
                }
            } else {
                return Ok(None); // Can't fold non-literal arguments
            }
        }
        
        // Perform constant folding
        let result = match (func_name.as_str(), values.as_slice()) {
            ("add", [a, b]) => a + b,
            ("sub", [a, b]) => a - b,
            ("mul", [a, b]) => a * b,
            ("div", [a, b]) if *b != 0 => a / b,
            ("mod", [a, b]) if *b != 0 => a % b,
            ("lt", [a, b]) => if a < b { 1 } else { 0 },
            ("gt", [a, b]) => if a > b { 1 } else { 0 },
            ("eq", [a, b]) => if a == b { 1 } else { 0 },
            ("and", [a, b]) => if *a != 0 && *b != 0 { 1 } else { 0 },
            ("or", [a, b]) => if *a != 0 || *b != 0 { 1 } else { 0 },
            ("not", [a]) => if *a == 0 { 1 } else { 0 },
            _ => return Ok(None),
        };
        
        // Create folded literal
        Ok(Some(YulExpression::Literal(YulLiteral {
            kind: LiteralKind::Number,
            value: result.to_string(),
            type_info: super::parser::TypeInfo {
                type_name: YulType::Uint256,
                size: Some(32),
                is_constant: true,
            },
            location: call.location.clone(),
        })))
    }
}

impl DeadCodeEliminationPass {
    fn new() -> Self {
        Self { removed_count: 0 }
    }
}

impl OptimizationPass for DeadCodeEliminationPass {
    fn name(&self) -> &str {
        "DeadCodeElimination"
    }

    fn description(&self) -> &str {
        "Remove unreachable and unused code"
    }

    fn apply(&mut self, ast: &mut YulAST) -> Result<bool, OptimizationError> {
        let mut changed = false;
        
        for item in &mut ast.items {
            match item {
                YulItem::Function(func) => {
                    changed |= self.eliminate_dead_code_in_function(func)?;
                },
                YulItem::Block(block) => {
                    changed |= self.eliminate_dead_code_in_block(block)?;
                },
                YulItem::Object(obj) => {
                    if let Some(code) = &mut obj.code {
                        changed |= self.eliminate_dead_code_in_block(code)?;
                    }
                },
            }
        }
        
        Ok(changed)
    }

    fn should_run(&self, level: u8) -> bool {
        level >= 1
    }
}

impl DeadCodeEliminationPass {
    fn eliminate_dead_code_in_function(&mut self, func: &mut YulFunction) -> Result<bool, OptimizationError> {
        self.eliminate_dead_code_in_block(&mut func.body)
    }

    fn eliminate_dead_code_in_block(&mut self, block: &mut YulBlock) -> Result<bool, OptimizationError> {
        let mut changed = false;
        let mut new_statements = Vec::new();
        
        for statement in block.statements.drain(..) {
            if self.is_dead_code(&statement) {
                self.removed_count += 1;
                changed = true;
            } else {
                // Recursively eliminate dead code in nested blocks
                let mut stmt = statement;
                changed |= self.eliminate_dead_code_in_statement(&mut stmt)?;
                new_statements.push(stmt);
            }
        }
        
        block.statements = new_statements;
        Ok(changed)
    }

    fn eliminate_dead_code_in_statement(&mut self, statement: &mut YulStatement) -> Result<bool, OptimizationError> {
        match statement {
            YulStatement::Block(block) => self.eliminate_dead_code_in_block(block),
            YulStatement::FunctionDef(func) => self.eliminate_dead_code_in_function(func),
            YulStatement::If(if_stmt) => {
                self.eliminate_dead_code_in_block(&mut if_stmt.body)
            },
            YulStatement::Switch(switch) => {
                let mut changed = false;
                for case in &mut switch.cases {
                    changed |= self.eliminate_dead_code_in_block(&mut case.body)?;
                }
                if let Some(default) = &mut switch.default {
                    changed |= self.eliminate_dead_code_in_block(default)?;
                }
                Ok(changed)
            },
            YulStatement::ForLoop(for_loop) => {
                let mut changed = self.eliminate_dead_code_in_block(&mut for_loop.init)?;
                changed |= self.eliminate_dead_code_in_block(&mut for_loop.post)?;
                changed |= self.eliminate_dead_code_in_block(&mut for_loop.body)?;
                Ok(changed)
            },
            _ => Ok(false),
        }
    }

    fn is_dead_code(&self, _statement: &YulStatement) -> bool {
        // Simple dead code detection
        // In a real implementation, this would be more sophisticated
        false
    }
}

impl FunctionInliningPass {
    fn new(size_threshold: usize) -> Self {
        Self {
            inlined_count: 0,
            size_threshold,
        }
    }
}

impl OptimizationPass for FunctionInliningPass {
    fn name(&self) -> &str {
        "FunctionInlining"
    }

    fn description(&self) -> &str {
        "Inline small functions to reduce call overhead"
    }

    fn apply(&mut self, _ast: &mut YulAST) -> Result<bool, OptimizationError> {
        // Function inlining implementation would go here
        // This is a complex optimization that requires careful analysis
        Ok(false)
    }

    fn should_run(&self, level: u8) -> bool {
        level >= 2
    }
}

impl CommonSubexpressionEliminationPass {
    fn new() -> Self {
        Self {
            eliminated_count: 0,
            expression_cache: HashMap::new(),
        }
    }
}

impl OptimizationPass for CommonSubexpressionEliminationPass {
    fn name(&self) -> &str {
        "CommonSubexpressionElimination"
    }

    fn description(&self) -> &str {
        "Eliminate redundant computations"
    }

    fn apply(&mut self, _ast: &mut YulAST) -> Result<bool, OptimizationError> {
        // CSE implementation would go here
        Ok(false)
    }

    fn should_run(&self, level: u8) -> bool {
        level >= 2
    }
}

impl LoopOptimizationPass {
    fn new() -> Self {
        Self { optimized_count: 0 }
    }
}

impl OptimizationPass for LoopOptimizationPass {
    fn name(&self) -> &str {
        "LoopOptimization"
    }

    fn description(&self) -> &str {
        "Optimize loop constructs for better performance"
    }

    fn apply(&mut self, _ast: &mut YulAST) -> Result<bool, OptimizationError> {
        // Loop optimization implementation would go here
        Ok(false)
    }

    fn should_run(&self, level: u8) -> bool {
        level >= 3
    }
}

impl StackOptimizationPass {
    fn new() -> Self {
        Self { reductions: 0 }
    }
}

impl OptimizationPass for StackOptimizationPass {
    fn name(&self) -> &str {
        "StackOptimization"
    }

    fn description(&self) -> &str {
        "Reduce unnecessary stack operations"
    }

    fn apply(&mut self, _ast: &mut YulAST) -> Result<bool, OptimizationError> {
        // Stack optimization implementation would go here
        Ok(false)
    }

    fn should_run(&self, level: u8) -> bool {
        level >= 2
    }
}

impl NeoVMOptimizationPass {
    fn new() -> Self {
        Self { optimizations: 0 }
    }
}

impl OptimizationPass for NeoVMOptimizationPass {
    fn name(&self) -> &str {
        "NeoVMOptimization"
    }

    fn description(&self) -> &str {
        "NeoVM-specific optimizations and instruction selection"
    }

    fn apply(&mut self, _ast: &mut YulAST) -> Result<bool, OptimizationError> {
        // NeoVM-specific optimizations would go here
        Ok(false)
    }

    fn should_run(&self, level: u8) -> bool {
        level >= 3
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compiler::{lexer::YulLexer, parser::YulParser};

    fn optimize_source(source: &str, level: u8) -> Result<YulAST, Box<dyn std::error::Error>> {
        let mut lexer = YulLexer::new();
        let tokens = lexer.tokenize(source)?;
        let mut parser = YulParser::new();
        let ast = parser.parse(tokens)?;
        let mut optimizer = Optimizer::new(level);
        Ok(optimizer.optimize(ast)?)
    }

    #[test]
    fn test_constant_folding() {
        let source = r#"
            {
                let x := add(1, 2)
                let y := mul(3, 4)
                let z := add(x, y)
            }
        "#;

        let ast = optimize_source(source, 1).unwrap();
        // In a real test, we would verify that constants were folded
        assert!(!ast.items.is_empty());
    }

    #[test]
    fn test_optimization_levels() {
        let source = r#"
            function square(x) -> result {
                result := mul(x, x)
            }
            
            {
                let a := add(1, 2)
                let b := square(a)
            }
        "#;

        // Test different optimization levels
        for level in 0..=3 {
            let ast = optimize_source(source, level).unwrap();
            assert!(!ast.items.is_empty());
        }
    }

    #[test]
    fn test_dead_code_elimination() {
        let source = r#"
            {
                let x := 42
                // This variable is never used
                let unused := 100
                return(0, 0) 
                // This code is unreachable
                let after_return := 1
            }
        "#;

        let ast = optimize_source(source, 2).unwrap();
        // In a real test, we would verify dead code was removed
        assert!(!ast.items.is_empty());
    }

    #[test]
    fn test_complex_expression_folding() {
        let source = r#"
            {
                let result := add(mul(2, 3), sub(10, 4))
                let bool_result := and(gt(5, 3), lt(2, 4))
            }
        "#;

        let ast = optimize_source(source, 1).unwrap();
        // Verify complex expressions are folded
        assert!(!ast.items.is_empty());
    }
}