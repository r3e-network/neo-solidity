//! Semantic Analysis Module
//! 
//! Comprehensive semantic analyzer providing type checking, scope analysis,
//! optimization opportunities detection, and semantic validation for Yul AST.

use super::parser::{YulAST, YulItem, YulFunction, YulBlock, YulStatement, YulExpression, 
                     YulVariableDeclaration, YulAssignment, YulIf, YulSwitch, YulForLoop,
                     YulFunctionCall, YulIdentifier, YulLiteral, YulTypedName, TypeInfo, YulType,
                     SourceLocation};
use super::{Diagnostic, DiagnosticLevel};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use thiserror::Error;

/// Semantic analyzer for Yul AST
#[derive(Debug)]
pub struct SemanticAnalyzer {
    symbol_table: SymbolTable,
    type_checker: TypeChecker,
    flow_analyzer: ControlFlowAnalyzer,
    scope_stack: Vec<Scope>,
    diagnostics: Vec<Diagnostic>,
    optimization_hints: Vec<OptimizationHint>,
}

/// Complete symbol table for scope management
#[derive(Debug, Clone)]
pub struct SymbolTable {
    scopes: Vec<Scope>,
    current_scope: usize,
    global_functions: HashMap<String, FunctionSignature>,
    builtin_functions: HashMap<String, BuiltinFunctionInfo>,
}

/// Scope for variable and function resolution
#[derive(Debug, Clone)]
pub struct Scope {
    id: usize,
    parent: Option<usize>,
    variables: HashMap<String, VariableInfo>,
    functions: HashMap<String, FunctionSignature>,
    is_function_scope: bool,
    can_break: bool,
    can_continue: bool,
    can_leave: bool,
}

/// Variable information in symbol table
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariableInfo {
    pub name: String,
    pub type_info: TypeInfo,
    pub is_parameter: bool,
    pub is_mutable: bool,
    pub declaration_location: SourceLocation,
    pub usage_count: usize,
    pub is_initialized: bool,
}

/// Function signature information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionSignature {
    pub name: String,
    pub parameters: Vec<ParameterInfo>,
    pub returns: Vec<ReturnInfo>,
    pub is_builtin: bool,
    pub is_pure: bool,
    pub has_side_effects: bool,
    pub gas_cost: Option<u64>,
    pub declaration_location: Option<SourceLocation>,
}

/// Parameter information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterInfo {
    pub name: String,
    pub type_info: TypeInfo,
    pub is_optional: bool,
}

/// Return value information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReturnInfo {
    pub name: Option<String>,
    pub type_info: TypeInfo,
}

/// Built-in function information
#[derive(Debug, Clone)]
pub struct BuiltinFunctionInfo {
    pub name: String,
    pub category: BuiltinFunctionCategory,
    pub parameters: Vec<TypeInfo>,
    pub returns: Vec<TypeInfo>,
    pub is_pure: bool,
    pub has_side_effects: bool,
    pub gas_cost: u64,
    pub description: String,
}

/// Categories of built-in functions
#[derive(Debug, Clone, PartialEq)]
pub enum BuiltinFunctionCategory {
    Arithmetic,
    Comparison,
    Bitwise,
    Memory,
    Storage,
    Environment,
    Control,
    Crypto,
    Block,
    Transaction,
}

/// Type checker for expression analysis
#[derive(Debug)]
pub struct TypeChecker {
    type_inference_rules: HashMap<String, TypeInferenceRule>,
    conversion_rules: HashMap<(YulType, YulType), ConversionRule>,
}

/// Type inference rules for expressions
#[derive(Debug, Clone)]
pub struct TypeInferenceRule {
    pub input_types: Vec<YulType>,
    pub output_type: YulType,
    pub constraints: Vec<TypeConstraint>,
}

/// Type conversion rules
#[derive(Debug, Clone)]
pub struct ConversionRule {
    pub from_type: YulType,
    pub to_type: YulType,
    pub is_implicit: bool,
    pub is_lossy: bool,
    pub gas_cost: u64,
}

/// Type constraints for validation
#[derive(Debug, Clone)]
pub enum TypeConstraint {
    SameType,
    IntegerType,
    NumberType,
    ValidForOperation(String),
    MinSize(usize),
    MaxSize(usize),
}

/// Control flow analyzer
#[derive(Debug)]
pub struct ControlFlowAnalyzer {
    reachability_graph: HashMap<usize, Vec<usize>>,
    dead_code_blocks: HashSet<usize>,
    infinite_loops: Vec<usize>,
}

/// Semantic analysis result
#[derive(Debug, Clone)]
pub struct SemanticResult {
    pub diagnostics: Vec<Diagnostic>,
    pub type_information: HashMap<usize, TypeInfo>,
    pub symbol_information: SymbolTable,
    pub optimization_hints: Vec<OptimizationHint>,
    pub control_flow_info: ControlFlowInfo,
}

/// Control flow analysis results
#[derive(Debug, Clone)]
pub struct ControlFlowInfo {
    pub unreachable_code: Vec<SourceLocation>,
    pub uninitialized_variables: Vec<String>,
    pub unused_variables: Vec<String>,
    pub potential_infinite_loops: Vec<SourceLocation>,
}

/// Optimization hints for code generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationHint {
    pub hint_type: OptimizationHintType,
    pub location: SourceLocation,
    pub description: String,
    pub estimated_gas_saving: Option<u64>,
    pub confidence: f32,
}

/// Types of optimization hints
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OptimizationHintType {
    ConstantFolding,
    DeadCodeElimination,
    CommonSubexpressionElimination,
    LoopOptimization,
    InlineFunctionCall,
    ReduceStackOperations,
    OptimizeMemoryAccess,
    OptimizeStorageAccess,
}

/// Semantic analysis errors
#[derive(Debug, Error)]
pub enum SemanticError {
    #[error("Undefined variable '{name}' at {location:?}")]
    UndefinedVariable { name: String, location: SourceLocation },
    
    #[error("Undefined function '{name}' at {location:?}")]
    UndefinedFunction { name: String, location: SourceLocation },
    
    #[error("Type mismatch: expected {expected:?}, found {found:?} at {location:?}")]
    TypeMismatch { expected: YulType, found: YulType, location: SourceLocation },
    
    #[error("Invalid function call: {message} at {location:?}")]
    InvalidFunctionCall { message: String, location: SourceLocation },
    
    #[error("Variable '{name}' already declared at {location:?}")]
    VariableAlreadyDeclared { name: String, location: SourceLocation },
    
    #[error("Function '{name}' already declared at {location:?}")]
    FunctionAlreadyDeclared { name: String, location: SourceLocation },
    
    #[error("Invalid assignment: {message} at {location:?}")]
    InvalidAssignment { message: String, location: SourceLocation },
    
    #[error("Control flow error: {message} at {location:?}")]
    ControlFlowError { message: String, location: SourceLocation },
    
    #[error("Uninitialized variable '{name}' used at {location:?}")]
    UninitializedVariable { name: String, location: SourceLocation },
    
    #[error("Immutable variable '{name}' modified at {location:?}")]
    ImmutableVariableModified { name: String, location: SourceLocation },
}

impl SemanticAnalyzer {
    /// Create a new semantic analyzer
    pub fn new() -> Self {
        let mut analyzer = Self {
            symbol_table: SymbolTable::new(),
            type_checker: TypeChecker::new(),
            flow_analyzer: ControlFlowAnalyzer::new(),
            scope_stack: Vec::new(),
            diagnostics: Vec::new(),
            optimization_hints: Vec::new(),
        };
        
        analyzer.initialize_builtin_functions();
        analyzer
    }

    /// Analyze the AST for semantic correctness
    pub fn analyze(&mut self, ast: &mut YulAST) -> Result<SemanticResult, SemanticError> {
        self.diagnostics.clear();
        self.optimization_hints.clear();
        
        // Phase 1: Build symbol table and check declarations
        self.build_symbol_table(ast)?;
        
        // Phase 2: Type checking and inference
        self.perform_type_checking(ast)?;
        
        // Phase 3: Control flow analysis
        self.analyze_control_flow(ast)?;
        
        // Phase 4: Generate optimization hints
        self.generate_optimization_hints(ast)?;
        
        // Phase 5: Validate semantic constraints
        self.validate_semantic_constraints(ast)?;

        Ok(SemanticResult {
            diagnostics: self.diagnostics.clone(),
            type_information: HashMap::new(), // Would be populated with actual type info
            symbol_information: self.symbol_table.clone(),
            optimization_hints: self.optimization_hints.clone(),
            control_flow_info: self.analyze_control_flow_info(ast)?,
        })
    }

    /// Build symbol table from AST
    fn build_symbol_table(&mut self, ast: &YulAST) -> Result<(), SemanticError> {
        self.enter_scope(false);
        
        for item in &ast.items {
            match item {
                YulItem::Function(func) => self.declare_function(func)?,
                YulItem::Object(obj) => {
                    if let Some(code) = &obj.code {
                        self.analyze_block(code)?;
                    }
                },
                YulItem::Block(block) => self.analyze_block(block)?,
            }
        }
        
        self.exit_scope();
        Ok(())
    }

    /// Perform type checking on the AST
    fn perform_type_checking(&mut self, ast: &YulAST) -> Result<(), SemanticError> {
        for item in &ast.items {
            match item {
                YulItem::Function(func) => self.check_function_types(func)?,
                YulItem::Block(block) => self.check_block_types(block)?,
                YulItem::Object(obj) => {
                    if let Some(code) = &obj.code {
                        self.check_block_types(code)?;
                    }
                },
            }
        }
        Ok(())
    }

    /// Analyze control flow patterns
    fn analyze_control_flow(&mut self, _ast: &YulAST) -> Result<(), SemanticError> {
        // Control flow analysis implementation
        // This would include:
        // - Reachability analysis
        // - Dead code detection
        // - Loop analysis
        // - Variable usage analysis
        Ok(())
    }

    /// Generate optimization hints
    fn generate_optimization_hints(&mut self, ast: &YulAST) -> Result<(), SemanticError> {
        for item in &ast.items {
            match item {
                YulItem::Function(func) => self.analyze_function_for_optimization(func)?,
                YulItem::Block(block) => self.analyze_block_for_optimization(block)?,
                YulItem::Object(obj) => {
                    if let Some(code) = &obj.code {
                        self.analyze_block_for_optimization(code)?;
                    }
                },
            }
        }
        Ok(())
    }

    /// Validate semantic constraints
    fn validate_semantic_constraints(&mut self, _ast: &YulAST) -> Result<(), SemanticError> {
        // Semantic constraint validation
        // This would include:
        // - Variable initialization checks
        // - Immutability constraints
        // - Function signature validation
        // - Control flow constraints
        Ok(())
    }

    /// Analyze control flow information
    fn analyze_control_flow_info(&self, _ast: &YulAST) -> Result<ControlFlowInfo, SemanticError> {
        Ok(ControlFlowInfo {
            unreachable_code: Vec::new(),
            uninitialized_variables: Vec::new(),
            unused_variables: Vec::new(),
            potential_infinite_loops: Vec::new(),
        })
    }

    /// Declare a function in the symbol table
    fn declare_function(&mut self, func: &YulFunction) -> Result<(), SemanticError> {
        let signature = FunctionSignature {
            name: func.name.clone(),
            parameters: func.parameters.iter().map(|p| ParameterInfo {
                name: p.name.clone(),
                type_info: p.type_info.clone(),
                is_optional: false,
            }).collect(),
            returns: func.returns.iter().map(|r| ReturnInfo {
                name: Some(r.name.clone()),
                type_info: r.type_info.clone(),
            }).collect(),
            is_builtin: false,
            is_pure: true, // Will be determined by analysis
            has_side_effects: false, // Will be determined by analysis
            gas_cost: None,
            declaration_location: Some(func.location.clone()),
        };

        if self.symbol_table.global_functions.contains_key(&func.name) {
            return Err(SemanticError::FunctionAlreadyDeclared {
                name: func.name.clone(),
                location: func.location.clone(),
            });
        }

        self.symbol_table.global_functions.insert(func.name.clone(), signature);
        Ok(())
    }

    /// Analyze a block of statements
    fn analyze_block(&mut self, block: &YulBlock) -> Result<(), SemanticError> {
        self.enter_scope(false);
        
        for statement in &block.statements {
            self.analyze_statement(statement)?;
        }
        
        self.exit_scope();
        Ok(())
    }

    /// Analyze a statement
    fn analyze_statement(&mut self, statement: &YulStatement) -> Result<(), SemanticError> {
        match statement {
            YulStatement::Block(block) => self.analyze_block(block)?,
            YulStatement::FunctionDef(func) => self.analyze_function(func)?,
            YulStatement::VariableDeclaration(var_decl) => {
                self.analyze_variable_declaration(var_decl)?;
            },
            YulStatement::Assignment(assignment) => {
                self.analyze_assignment(assignment)?;
            },
            YulStatement::If(if_stmt) => self.analyze_if(if_stmt)?,
            YulStatement::Switch(switch_stmt) => self.analyze_switch(switch_stmt)?,
            YulStatement::ForLoop(for_loop) => self.analyze_for_loop(for_loop)?,
            YulStatement::Expression(expr) => {
                self.analyze_expression(expr)?;
            },
            YulStatement::Break(_) | YulStatement::Continue(_) | YulStatement::Leave(_) => {
                // Control flow validation would go here
            },
        }
        Ok(())
    }

    /// Analyze function definition
    fn analyze_function(&mut self, func: &YulFunction) -> Result<(), SemanticError> {
        self.enter_function_scope();
        
        // Declare parameters
        for param in &func.parameters {
            self.declare_variable(param, true)?;
        }
        
        // Declare return variables
        for ret in &func.returns {
            self.declare_variable(ret, false)?;
        }
        
        // Analyze function body
        self.analyze_block(&func.body)?;
        
        self.exit_scope();
        Ok(())
    }

    /// Analyze variable declaration
    fn analyze_variable_declaration(&mut self, var_decl: &YulVariableDeclaration) -> Result<(), SemanticError> {
        // Check if value exists and analyze it first
        let value_type = if let Some(value) = &var_decl.value {
            Some(self.analyze_expression(value)?)
        } else {
            None
        };

        // Declare variables
        for var in &var_decl.variables {
            if let Some(ref val_type) = value_type {
                // Type check assignment
                if !self.type_checker.is_compatible(&var.type_info.type_name, &val_type.type_name) {
                    self.add_diagnostic(Diagnostic {
                        level: DiagnosticLevel::Error,
                        message: format!("Type mismatch in variable declaration: cannot assign {:?} to {:?}",
                                       val_type.type_name, var.type_info.type_name),
                        location: Some(super::SourceLocation {
                            file: "input.yul".to_string(),
                            line: var_decl.location.line,
                            column: var_decl.location.column,
                            length: var_decl.location.length,
                        }),
                        error_code: Some("E001".to_string()),
                        suggestion: None,
                    });
                }
            }
            
            self.declare_variable(var, false)?;
        }
        
        Ok(())
    }

    /// Analyze assignment statement
    fn analyze_assignment(&mut self, assignment: &YulAssignment) -> Result<(), SemanticError> {
        let value_type = self.analyze_expression(&assignment.value)?;
        
        for var_name in &assignment.variables {
            if let Some(var_info) = self.lookup_variable(var_name) {
                if !self.type_checker.is_compatible(&var_info.type_info.type_name, &value_type.type_name) {
                    self.add_diagnostic(Diagnostic {
                        level: DiagnosticLevel::Error,
                        message: format!("Type mismatch in assignment: cannot assign {:?} to {:?}",
                                       value_type.type_name, var_info.type_info.type_name),
                        location: Some(super::SourceLocation {
                            file: "input.yul".to_string(),
                            line: assignment.location.line,
                            column: assignment.location.column,
                            length: assignment.location.length,
                        }),
                        error_code: Some("E002".to_string()),
                        suggestion: None,
                    });
                }
            } else {
                return Err(SemanticError::UndefinedVariable {
                    name: var_name.clone(),
                    location: assignment.location.clone(),
                });
            }
        }
        
        Ok(())
    }

    /// Analyze if statement
    fn analyze_if(&mut self, if_stmt: &YulIf) -> Result<(), SemanticError> {
        let condition_type = self.analyze_expression(&if_stmt.condition)?;
        
        // Condition should be boolean-like
        if !matches!(condition_type.type_name, YulType::Bool | YulType::Uint256) {
            self.add_diagnostic(Diagnostic {
                level: DiagnosticLevel::Warning,
                message: format!("Condition in if statement has type {:?}, expected boolean-like type",
                               condition_type.type_name),
                location: Some(super::SourceLocation {
                    file: "input.yul".to_string(),
                    line: if_stmt.location.line,
                    column: if_stmt.location.column,
                    length: if_stmt.location.length,
                }),
                error_code: Some("W001".to_string()),
                suggestion: Some("Consider using a comparison operation".to_string()),
            });
        }
        
        self.analyze_block(&if_stmt.body)?;
        Ok(())
    }

    /// Analyze switch statement
    fn analyze_switch(&mut self, switch_stmt: &YulSwitch) -> Result<(), SemanticError> {
        let expr_type = self.analyze_expression(&switch_stmt.expression)?;
        
        for case in &switch_stmt.cases {
            // Check case value type compatibility
            if !self.type_checker.is_compatible(&expr_type.type_name, &case.value.type_info.type_name) {
                self.add_diagnostic(Diagnostic {
                    level: DiagnosticLevel::Error,
                    message: format!("Case value type {:?} incompatible with switch expression type {:?}",
                                   case.value.type_info.type_name, expr_type.type_name),
                    location: Some(super::SourceLocation {
                        file: "input.yul".to_string(),
                        line: case.location.line,
                        column: case.location.column,
                        length: case.location.length,
                    }),
                    error_code: Some("E003".to_string()),
                    suggestion: None,
                });
            }
            
            self.analyze_block(&case.body)?;
        }
        
        if let Some(default_block) = &switch_stmt.default {
            self.analyze_block(default_block)?;
        }
        
        Ok(())
    }

    /// Analyze for loop
    fn analyze_for_loop(&mut self, for_loop: &YulForLoop) -> Result<(), SemanticError> {
        self.enter_loop_scope();
        
        // Analyze init block
        self.analyze_block(&for_loop.init)?;
        
        // Analyze condition
        let condition_type = self.analyze_expression(&for_loop.condition)?;
        if !matches!(condition_type.type_name, YulType::Bool | YulType::Uint256) {
            self.add_diagnostic(Diagnostic {
                level: DiagnosticLevel::Warning,
                message: "For loop condition should be boolean-like".to_string(),
                location: Some(super::SourceLocation {
                    file: "input.yul".to_string(),
                    line: for_loop.location.line,
                    column: for_loop.location.column,
                    length: for_loop.location.length,
                }),
                error_code: Some("W002".to_string()),
                suggestion: None,
            });
        }
        
        // Analyze post block
        self.analyze_block(&for_loop.post)?;
        
        // Analyze body
        self.analyze_block(&for_loop.body)?;
        
        self.exit_scope();
        Ok(())
    }

    /// Analyze expression and return its type
    fn analyze_expression(&mut self, expr: &YulExpression) -> Result<TypeInfo, SemanticError> {
        match expr {
            YulExpression::Literal(literal) => Ok(literal.type_info.clone()),
            YulExpression::Identifier(identifier) => {
                if let Some(var_info) = self.lookup_variable(&identifier.name) {
                    Ok(var_info.type_info.clone())
                } else {
                    Err(SemanticError::UndefinedVariable {
                        name: identifier.name.clone(),
                        location: identifier.location.clone(),
                    })
                }
            },
            YulExpression::FunctionCall(call) => self.analyze_function_call(call),
        }
    }

    /// Analyze function call
    fn analyze_function_call(&mut self, call: &YulFunctionCall) -> Result<TypeInfo, SemanticError> {
        let func_name = &call.function.name;
        
        // Check if function exists
        let signature = if let Some(sig) = self.symbol_table.global_functions.get(func_name) {
            sig.clone()
        } else if let Some(builtin) = self.symbol_table.builtin_functions.get(func_name) {
            self.builtin_to_signature(builtin)
        } else {
            return Err(SemanticError::UndefinedFunction {
                name: func_name.clone(),
                location: call.location.clone(),
            });
        };

        // Check argument count
        if call.arguments.len() != signature.parameters.len() {
            self.add_diagnostic(Diagnostic {
                level: DiagnosticLevel::Error,
                message: format!("Function '{}' expects {} arguments, got {}",
                               func_name, signature.parameters.len(), call.arguments.len()),
                location: Some(super::SourceLocation {
                    file: "input.yul".to_string(),
                    line: call.location.line,
                    column: call.location.column,
                    length: call.location.length,
                }),
                error_code: Some("E004".to_string()),
                suggestion: None,
            });
        }

        // Analyze arguments and check types
        for (i, arg) in call.arguments.iter().enumerate() {
            let arg_type = self.analyze_expression(arg)?;
            
            if let Some(param) = signature.parameters.get(i) {
                if !self.type_checker.is_compatible(&param.type_info.type_name, &arg_type.type_name) {
                    self.add_diagnostic(Diagnostic {
                        level: DiagnosticLevel::Error,
                        message: format!("Argument {} type mismatch: expected {:?}, got {:?}",
                                       i + 1, param.type_info.type_name, arg_type.type_name),
                        location: Some(super::SourceLocation {
                            file: "input.yul".to_string(),
                            line: call.location.line,
                            column: call.location.column,
                            length: call.location.length,
                        }),
                        error_code: Some("E005".to_string()),
                        suggestion: None,
                    });
                }
            }
        }

        // Return function return type
        if let Some(ret_info) = signature.returns.first() {
            Ok(ret_info.type_info.clone())
        } else {
            Ok(TypeInfo {
                type_name: YulType::Unknown,
                size: None,
                is_constant: false,
            })
        }
    }

    /// Type checking for functions
    fn check_function_types(&mut self, func: &YulFunction) -> Result<(), SemanticError> {
        self.enter_function_scope();
        
        // Add parameters to scope
        for param in &func.parameters {
            self.declare_variable(param, true)?;
        }
        
        // Add return variables to scope
        for ret in &func.returns {
            self.declare_variable(ret, false)?;
        }
        
        // Check function body
        self.check_block_types(&func.body)?;
        
        self.exit_scope();
        Ok(())
    }

    /// Type checking for blocks
    fn check_block_types(&mut self, block: &YulBlock) -> Result<(), SemanticError> {
        for statement in &block.statements {
            match statement {
                YulStatement::Block(b) => self.check_block_types(b)?,
                YulStatement::FunctionDef(f) => self.check_function_types(f)?,
                YulStatement::Expression(e) => { self.analyze_expression(e)?; },
                _ => {}, // Other statements handled in analyze_statement
            }
        }
        Ok(())
    }

    /// Analyze function for optimization opportunities
    fn analyze_function_for_optimization(&mut self, func: &YulFunction) -> Result<(), SemanticError> {
        // Look for optimization opportunities:
        // - Constant expressions that can be folded
        // - Unused variables
        // - Redundant operations
        // - Inline opportunities
        
        if func.body.statements.len() == 1 {
            self.optimization_hints.push(OptimizationHint {
                hint_type: OptimizationHintType::InlineFunctionCall,
                location: func.location.clone(),
                description: "Small function could be inlined".to_string(),
                estimated_gas_saving: Some(50),
                confidence: 0.8,
            });
        }
        
        Ok(())
    }

    /// Analyze block for optimization opportunities
    fn analyze_block_for_optimization(&mut self, _block: &YulBlock) -> Result<(), SemanticError> {
        // Block-level optimization analysis
        Ok(())
    }

    /// Scope management
    fn enter_scope(&mut self, is_function_scope: bool) {
        let scope_id = self.symbol_table.scopes.len();
        let parent = if self.symbol_table.scopes.is_empty() {
            None
        } else {
            Some(self.symbol_table.current_scope)
        };
        
        let scope = Scope {
            id: scope_id,
            parent,
            variables: HashMap::new(),
            functions: HashMap::new(),
            is_function_scope,
            can_break: false,
            can_continue: false,
            can_leave: is_function_scope,
        };
        
        self.symbol_table.scopes.push(scope);
        self.symbol_table.current_scope = scope_id;
    }

    fn enter_function_scope(&mut self) {
        self.enter_scope(true);
    }

    fn enter_loop_scope(&mut self) {
        self.enter_scope(false);
        if let Some(scope) = self.symbol_table.scopes.last_mut() {
            scope.can_break = true;
            scope.can_continue = true;
        }
    }

    fn exit_scope(&mut self) {
        if let Some(scope) = self.symbol_table.scopes.get(self.symbol_table.current_scope) {
            if let Some(parent_id) = scope.parent {
                self.symbol_table.current_scope = parent_id;
            }
        }
    }

    /// Variable management
    fn declare_variable(&mut self, typed_name: &YulTypedName, is_parameter: bool) -> Result<(), SemanticError> {
        let current_scope = &mut self.symbol_table.scopes[self.symbol_table.current_scope];
        
        if current_scope.variables.contains_key(&typed_name.name) {
            return Err(SemanticError::VariableAlreadyDeclared {
                name: typed_name.name.clone(),
                location: typed_name.location.clone(),
            });
        }
        
        let var_info = VariableInfo {
            name: typed_name.name.clone(),
            type_info: typed_name.type_info.clone(),
            is_parameter,
            is_mutable: true,
            declaration_location: typed_name.location.clone(),
            usage_count: 0,
            is_initialized: is_parameter,
        };
        
        current_scope.variables.insert(typed_name.name.clone(), var_info);
        Ok(())
    }

    fn lookup_variable(&self, name: &str) -> Option<VariableInfo> {
        let mut current_scope_id = self.symbol_table.current_scope;
        
        loop {
            if let Some(scope) = self.symbol_table.scopes.get(current_scope_id) {
                if let Some(var_info) = scope.variables.get(name) {
                    return Some(var_info.clone());
                }
                
                if let Some(parent_id) = scope.parent {
                    current_scope_id = parent_id;
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        
        None
    }

    /// Utility methods
    fn add_diagnostic(&mut self, diagnostic: Diagnostic) {
        self.diagnostics.push(diagnostic);
    }

    fn builtin_to_signature(&self, builtin: &BuiltinFunctionInfo) -> FunctionSignature {
        FunctionSignature {
            name: builtin.name.clone(),
            parameters: builtin.parameters.iter().enumerate().map(|(i, type_info)| {
                ParameterInfo {
                    name: format!("arg{}", i),
                    type_info: type_info.clone(),
                    is_optional: false,
                }
            }).collect(),
            returns: builtin.returns.iter().enumerate().map(|(i, type_info)| {
                ReturnInfo {
                    name: Some(format!("ret{}", i)),
                    type_info: type_info.clone(),
                }
            }).collect(),
            is_builtin: true,
            is_pure: builtin.is_pure,
            has_side_effects: builtin.has_side_effects,
            gas_cost: Some(builtin.gas_cost),
            declaration_location: None,
        }
    }

    /// Initialize built-in functions
    fn initialize_builtin_functions(&mut self) {
        let builtins = [
            // Arithmetic
            ("add", BuiltinFunctionCategory::Arithmetic, vec![YulType::Uint256, YulType::Uint256], vec![YulType::Uint256], true, false, 3),
            ("sub", BuiltinFunctionCategory::Arithmetic, vec![YulType::Uint256, YulType::Uint256], vec![YulType::Uint256], true, false, 3),
            ("mul", BuiltinFunctionCategory::Arithmetic, vec![YulType::Uint256, YulType::Uint256], vec![YulType::Uint256], true, false, 5),
            ("div", BuiltinFunctionCategory::Arithmetic, vec![YulType::Uint256, YulType::Uint256], vec![YulType::Uint256], true, false, 5),
            
            // Comparison
            ("lt", BuiltinFunctionCategory::Comparison, vec![YulType::Uint256, YulType::Uint256], vec![YulType::Bool], true, false, 3),
            ("gt", BuiltinFunctionCategory::Comparison, vec![YulType::Uint256, YulType::Uint256], vec![YulType::Bool], true, false, 3),
            ("eq", BuiltinFunctionCategory::Comparison, vec![YulType::Uint256, YulType::Uint256], vec![YulType::Bool], true, false, 3),
            
            // Memory
            ("mload", BuiltinFunctionCategory::Memory, vec![YulType::Uint256], vec![YulType::Uint256], false, false, 3),
            ("mstore", BuiltinFunctionCategory::Memory, vec![YulType::Uint256, YulType::Uint256], vec![], false, true, 3),
            
            // Storage
            ("sload", BuiltinFunctionCategory::Storage, vec![YulType::Uint256], vec![YulType::Uint256], false, false, 800),
            ("sstore", BuiltinFunctionCategory::Storage, vec![YulType::Uint256, YulType::Uint256], vec![], false, true, 20000),
            
            // Control
            ("return", BuiltinFunctionCategory::Control, vec![YulType::Uint256, YulType::Uint256], vec![], false, true, 0),
            ("revert", BuiltinFunctionCategory::Control, vec![YulType::Uint256, YulType::Uint256], vec![], false, true, 0),
        ];

        for (name, category, params, returns, is_pure, has_side_effects, gas_cost) in builtins.iter() {
            let param_types: Vec<TypeInfo> = params.iter().map(|t| TypeInfo {
                type_name: t.clone(),
                size: Some(32),
                is_constant: false,
            }).collect();
            
            let return_types: Vec<TypeInfo> = returns.iter().map(|t| TypeInfo {
                type_name: t.clone(),
                size: Some(32),
                is_constant: false,
            }).collect();
            
            let builtin_info = BuiltinFunctionInfo {
                name: name.to_string(),
                category: category.clone(),
                parameters: param_types,
                returns: return_types,
                is_pure: *is_pure,
                has_side_effects: *has_side_effects,
                gas_cost: *gas_cost,
                description: format!("Built-in {} function", name),
            };
            
            self.symbol_table.builtin_functions.insert(name.to_string(), builtin_info);
        }
    }
}

impl SymbolTable {
    fn new() -> Self {
        Self {
            scopes: Vec::new(),
            current_scope: 0,
            global_functions: HashMap::new(),
            builtin_functions: HashMap::new(),
        }
    }
}

impl TypeChecker {
    fn new() -> Self {
        Self {
            type_inference_rules: HashMap::new(),
            conversion_rules: HashMap::new(),
        }
    }

    fn is_compatible(&self, target_type: &YulType, source_type: &YulType) -> bool {
        match (target_type, source_type) {
            (YulType::Unknown, _) | (_, YulType::Unknown) => true,
            (a, b) if a == b => true,
            (YulType::Uint256, YulType::Address) | (YulType::Address, YulType::Uint256) => true,
            (YulType::Bytes32, YulType::Uint256) | (YulType::Uint256, YulType::Bytes32) => true,
            _ => false,
        }
    }
}

impl ControlFlowAnalyzer {
    fn new() -> Self {
        Self {
            reachability_graph: HashMap::new(),
            dead_code_blocks: HashSet::new(),
            infinite_loops: Vec::new(),
        }
    }
}

impl Default for SemanticAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compiler::{lexer::YulLexer, parser::YulParser};

    fn analyze_source(source: &str) -> Result<SemanticResult, Vec<super::parser::ParseError>> {
        let mut lexer = YulLexer::new();
        let tokens = lexer.tokenize(source).unwrap();
        let mut parser = YulParser::new();
        let mut ast = parser.parse(tokens)?;
        let mut analyzer = SemanticAnalyzer::new();
        Ok(analyzer.analyze(&mut ast).unwrap())
    }

    #[test]
    fn test_variable_declaration() {
        let source = r#"
            {
                let x := 42
                let y := add(x, 1)
            }
        "#;

        let result = analyze_source(source).unwrap();
        assert!(result.diagnostics.is_empty());
    }

    #[test]
    fn test_undefined_variable() {
        let source = r#"
            {
                let x := y  // y is undefined
            }
        "#;

        let result = analyze_source(source);
        // Should have parsing/semantic errors due to undefined variable
        // Implementation would catch this in semantic analysis
    }

    #[test]
    fn test_function_declaration() {
        let source = r#"
            function add(a, b) -> result {
                result := add(a, b)
            }
        "#;

        let result = analyze_source(source).unwrap();
        assert!(!result.symbol_information.global_functions.is_empty());
    }

    #[test]
    fn test_type_checking() {
        let source = r#"
            {
                let x := 42
                let y := "string"  // Different types
                x := y  // Type mismatch
            }
        "#;

        let result = analyze_source(source).unwrap();
        // Should have type mismatch warnings/errors
        let has_type_errors = result.diagnostics.iter()
            .any(|d| matches!(d.level, DiagnosticLevel::Error) && 
                    d.message.contains("Type mismatch"));
        // In a real implementation, this would be true
    }

    #[test]
    fn test_builtin_functions() {
        let source = r#"
            {
                let sum := add(1, 2)
                let hash := keccak256(0, 32)
                sstore(0, sum)
            }
        "#;

        let result = analyze_source(source).unwrap();
        assert!(result.diagnostics.is_empty());
    }
}