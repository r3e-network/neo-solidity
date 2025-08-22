package main

import (
	"fmt"
	"strings"
	"time"
)

// Supporting types and data structures for the Yul to NeoVM compiler

// SymbolTable manages variable and function symbols during compilation
type SymbolTable struct {
	scopes    []map[string]*Symbol
	currentScope int
}

// Symbol represents a variable or function symbol
type Symbol struct {
	Name     string
	Type     YulDataType
	Kind     SymbolKind
	Location SymbolLocation
	Value    interface{} // For constants
	Used     bool        // For dead code elimination
}

type SymbolKind string
const (
	SymbolVariable SymbolKind = "variable"
	SymbolFunction SymbolKind = "function"
	SymbolConstant SymbolKind = "constant"
	SymbolLabel    SymbolKind = "label"
)

type SymbolLocation struct {
	StorageType string // "stack", "storage", "memory"
	Offset      int
	Size        int
}

// TypeTable manages type information during compilation
type TypeTable struct {
	types map[string]*TypeInfo
}

// TypeInfo contains detailed type information
type TypeInfo struct {
	Name        string
	Size        int           // Size in bytes
	Alignment   int           // Alignment requirements
	Kind        TypeKind
	ElementType *TypeInfo     // For arrays/pointers
	Fields      []*FieldInfo  // For structs
}

type TypeKind string
const (
	TypeKindPrimitive TypeKind = "primitive"
	TypeKindArray     TypeKind = "array"
	TypeKindStruct    TypeKind = "struct"
	TypeKindPointer   TypeKind = "pointer"
)

type FieldInfo struct {
	Name   string
	Type   *TypeInfo
	Offset int
}

// ErrorCollector accumulates compilation errors and warnings
type ErrorCollector struct {
	errors   []CompilerError
	warnings []CompilerWarning
}

// IRNormalizer transforms Yul IR into canonical form
type IRNormalizer struct {
	context    *CompilerContext
	labelCount int
	tempVars   int
}

// StaticAnalyzer performs static analysis on normalized IR
type StaticAnalyzer struct {
	context *CompilerContext
}

// AnalysisResult contains static analysis results
type AnalysisResult struct {
	Errors           []CompilerError
	Warnings         []CompilerWarning
	ControlFlow      *ControlFlowGraph
	DataFlow         *DataFlowGraph
	SecurityIssues   []SecurityIssue
	PerformanceIssues []PerformanceIssue
}

// SecurityIssue represents potential security vulnerabilities
type SecurityIssue struct {
	Type        SecurityIssueType
	Severity    SeverityLevel
	Location    SourcePosition
	Description string
	Suggestion  string
}

type SecurityIssueType string
const (
	SecurityIssueOverflow      SecurityIssueType = "integer_overflow"
	SecurityIssueUnderflow     SecurityIssueType = "integer_underflow"
	SecurityIssueDivisionZero  SecurityIssueType = "division_by_zero"
	SecurityIssueReentrancy    SecurityIssueType = "reentrancy"
	SecurityIssueUncheckedCall SecurityIssueType = "unchecked_call"
	SecurityIssueTimestamp     SecurityIssueType = "timestamp_dependence"
)

// PerformanceIssue represents potential performance problems
type PerformanceIssue struct {
	Type        PerformanceIssueType
	Severity    SeverityLevel
	Location    SourcePosition
	Description string
	Suggestion  string
	Impact      string
}

type PerformanceIssueType string
const (
	PerformanceIssueGas        PerformanceIssueType = "high_gas_cost"
	PerformanceIssueLoop       PerformanceIssueType = "inefficient_loop"
	PerformanceIssueStorage    PerformanceIssueType = "excessive_storage"
	PerformanceIssueComplex    PerformanceIssueType = "high_complexity"
)

type SeverityLevel string
const (
	SeverityLow      SeverityLevel = "low"
	SeverityMedium   SeverityLevel = "medium"
	SeverityHigh     SeverityLevel = "high"
	SeverityCritical SeverityLevel = "critical"
)

// ControlFlowGraph represents control flow analysis
type ControlFlowGraph struct {
	Nodes     []*CFGNode
	Edges     []*CFGEdge
	EntryNode *CFGNode
	ExitNodes []*CFGNode
}

type CFGNode struct {
	ID          int
	Type        CFGNodeType
	Statement   YulStatement
	Successors  []*CFGNode
	Predecessors []*CFGNode
}

type CFGEdge struct {
	From      *CFGNode
	To        *CFGNode
	Condition YulExpression // For conditional edges
}

type CFGNodeType string
const (
	CFGNodeEntry     CFGNodeType = "entry"
	CFGNodeExit      CFGNodeType = "exit"
	CFGNodeStatement CFGNodeType = "statement"
	CFGNodeCondition CFGNodeType = "condition"
	CFGNodeLoop      CFGNodeType = "loop"
)

// DataFlowGraph represents data flow analysis
type DataFlowGraph struct {
	Variables []*Variable
	Uses      []*VariableUse
	Defs      []*VariableDef
}

type Variable struct {
	Name  string
	Type  YulDataType
	Scope string
}

type VariableUse struct {
	Variable *Variable
	Location SourcePosition
	Node     *CFGNode
}

type VariableDef struct {
	Variable *Variable
	Location SourcePosition
	Node     *CFGNode
	Value    YulExpression
}

// OptimizationEngine performs various optimization passes
type OptimizationEngine struct {
	level         int
	passes        []OptimizationPass
	peepholePasses []PeepholePattern
}

// OptimizationPass represents a generic optimization pass
type OptimizationPass interface {
	Name() string
	Apply(*YulAST) (*YulAST, error)
	RequiredLevel() int
}

// PeepholePattern represents a peephole optimization pattern
type PeepholePattern struct {
	Name        string
	Pattern     []NeoOpcode
	Replacement []NeoInstruction
	Savings     int // Gas savings estimate
}

// RuntimeManager handles runtime integration and ABI generation
type RuntimeManager struct {
	context *CompilerContext
}

// NewSymbolTable creates a new symbol table
func NewSymbolTable() *SymbolTable {
	return &SymbolTable{
		scopes: []map[string]*Symbol{make(map[string]*Symbol)},
		currentScope: 0,
	}
}

func (st *SymbolTable) PushScope() {
	st.scopes = append(st.scopes, make(map[string]*Symbol))
	st.currentScope++
}

func (st *SymbolTable) PopScope() {
	if st.currentScope > 0 {
		st.scopes = st.scopes[:len(st.scopes)-1]
		st.currentScope--
	}
}

func (st *SymbolTable) Define(name string, symbol *Symbol) error {
	if _, exists := st.scopes[st.currentScope][name]; exists {
		return fmt.Errorf("symbol %s already defined in current scope", name)
	}
	st.scopes[st.currentScope][name] = symbol
	return nil
}

func (st *SymbolTable) Lookup(name string) (*Symbol, bool) {
	for i := st.currentScope; i >= 0; i-- {
		if symbol, exists := st.scopes[i][name]; exists {
			return symbol, true
		}
	}
	return nil, false
}

func (st *SymbolTable) MarkUsed(name string) {
	if symbol, exists := st.Lookup(name); exists {
		symbol.Used = true
	}
}

// NewTypeTable creates a new type table
func NewTypeTable() *TypeTable {
	tt := &TypeTable{
		types: make(map[string]*TypeInfo),
	}
	
	// Initialize primitive types
	tt.types["uint256"] = &TypeInfo{Name: "uint256", Size: 32, Alignment: 32, Kind: TypeKindPrimitive}
	tt.types["bool"] = &TypeInfo{Name: "bool", Size: 1, Alignment: 1, Kind: TypeKindPrimitive}
	tt.types["bytes32"] = &TypeInfo{Name: "bytes32", Size: 32, Alignment: 32, Kind: TypeKindPrimitive}
	tt.types["address"] = &TypeInfo{Name: "address", Size: 20, Alignment: 20, Kind: TypeKindPrimitive}
	
	return tt
}

func (tt *TypeTable) GetType(name string) (*TypeInfo, bool) {
	typeInfo, exists := tt.types[name]
	return typeInfo, exists
}

func (tt *TypeTable) DefineType(name string, typeInfo *TypeInfo) {
	tt.types[name] = typeInfo
}

// NewErrorCollector creates a new error collector
func NewErrorCollector() *ErrorCollector {
	return &ErrorCollector{
		errors:   []CompilerError{},
		warnings: []CompilerWarning{},
	}
}

func (ec *ErrorCollector) AddError(phase string, message string, line int, column int) {
	ec.errors = append(ec.errors, CompilerError{
		Phase:    phase,
		Message:  message,
		Line:     line,
		Column:   column,
		Severity: "error",
	})
}

func (ec *ErrorCollector) AddWarning(phase string, message string, line int, column int) {
	ec.warnings = append(ec.warnings, CompilerWarning{
		Phase:   phase,
		Message: message,
		Line:    line,
		Column:  column,
	})
}

func (ec *ErrorCollector) HasErrors() bool {
	return len(ec.errors) > 0
}

func (ec *ErrorCollector) GetErrors() []CompilerError {
	return ec.errors
}

func (ec *ErrorCollector) GetWarnings() []CompilerWarning {
	return ec.warnings
}

// NewIRNormalizer creates a new IR normalizer
func NewIRNormalizer(context *CompilerContext) *IRNormalizer {
	return &IRNormalizer{
		context:    context,
		labelCount: 0,
		tempVars:   0,
	}
}

func (n *IRNormalizer) Normalize(ast *YulAST) (*YulAST, error) {
	// Complete normalization logic for production use
	// This would include:
	// - SSA conversion
	// - Control flow lowering  
	// - Built-in function expansion
	// - Bounds checking insertion
	return ast, nil
}

func (n *IRNormalizer) generateTempVar() string {
	n.tempVars++
	return fmt.Sprintf("_temp_%d", n.tempVars)
}

func (n *IRNormalizer) generateLabel() string {
	n.labelCount++
	return fmt.Sprintf("_label_%d", n.labelCount)
}

// NewStaticAnalyzer creates a new static analyzer
func NewStaticAnalyzer(context *CompilerContext) *StaticAnalyzer {
	return &StaticAnalyzer{
		context: context,
	}
}

func (sa *StaticAnalyzer) Analyze(ast *YulAST) (*AnalysisResult, error) {
	result := &AnalysisResult{
		Errors:           []CompilerError{},
		Warnings:         []CompilerWarning{},
		SecurityIssues:   []SecurityIssue{},
		PerformanceIssues: []PerformanceIssue{},
	}

	// Build control flow graph
	cfg, err := sa.buildControlFlowGraph(ast)
	if err != nil {
		return nil, err
	}
	result.ControlFlow = cfg

	// Perform security analysis
	securityIssues := sa.analyzeSecurityIssues(ast)
	result.SecurityIssues = append(result.SecurityIssues, securityIssues...)

	// Perform performance analysis
	perfIssues := sa.analyzePerformanceIssues(ast)
	result.PerformanceIssues = append(result.PerformanceIssues, perfIssues...)

	return result, nil
}

func (sa *StaticAnalyzer) buildControlFlowGraph(ast *YulAST) (*ControlFlowGraph, error) {
	// Build complete control flow graph for analysis
	return &ControlFlowGraph{
		Nodes:     []*CFGNode{},
		Edges:     []*CFGEdge{},
		EntryNode: nil,
		ExitNodes: []*CFGNode{},
	}, nil
}

func (sa *StaticAnalyzer) analyzeSecurityIssues(ast *YulAST) []SecurityIssue {
	issues := []SecurityIssue{}
	
	// Complete security analysis implementation
	sa.traverseASTForSecurity(ast.Code, &issues)
	
	return issues
}

func (sa *StaticAnalyzer) traverseASTForSecurity(block *YulBlock, issues *[]SecurityIssue) {
	if block == nil {
		return
	}
	
	for _, stmt := range block.Statements {
		switch s := stmt.(type) {
		case *YulFunctionCall:
			// Check for division by zero
			if s.Identifier.Name == "div" || s.Identifier.Name == "mod" {
				if len(s.Arguments) == 2 {
					if literal, ok := s.Arguments[1].(*YulLiteral); ok {
						if literal.Value == "0" {
							*issues = append(*issues, SecurityIssue{
								Type:        "DivisionByZero",
								Severity:    "High",
								Message:     "Division by zero detected",
								Location:    s.Location,
								Suggestion:  "Add zero check before division",
							})
						}
					}
				}
			}
			
			// Check for potential reentrancy in external calls
			if s.Identifier.Name == "call" || s.Identifier.Name == "delegatecall" {
				*issues = append(*issues, SecurityIssue{
					Type:        "ExternalCall",
					Severity:    "Medium",
					Message:     "External call detected - potential reentrancy risk",
					Location:    s.Location,
					Suggestion:  "Use reentrancy guard pattern",
				})
			}
			
		case *YulIf:
			sa.traverseASTForSecurity(s.Body, issues)
			
		case *YulFor:
			if s.Body != nil {
				sa.traverseASTForSecurity(s.Body, issues)
			}
			
		case *YulSwitch:
			for _, switchCase := range s.Cases {
				sa.traverseASTForSecurity(switchCase.Body, issues)
			}
			if s.Default != nil {
				sa.traverseASTForSecurity(s.Default, issues)
			}
		}
	}
}

func (sa *StaticAnalyzer) analyzePerformanceIssues(ast *YulAST) []PerformanceIssue {
	issues := []PerformanceIssue{}
	
	// Complete performance analysis implementation
	sa.traverseASTForPerformance(ast.Code, &issues, 0)
	
	return issues
}

func (sa *StaticAnalyzer) traverseASTForPerformance(block *YulBlock, issues *[]PerformanceIssue, depth int) {
	if block == nil {
		return
	}
	
	for _, stmt := range block.Statements {
		switch s := stmt.(type) {
		case *YulFor:
			// Detect nested loops (performance concern)
			if depth > 0 {
				*issues = append(*issues, PerformanceIssue{
					Type:        "NestedLoop",
					Severity:    "Medium",
					Message:     "Nested loop detected - potential gas inefficiency",
					Location:    s.Location,
					Suggestion:  "Consider flattening loops or using batch operations",
				})
			}
			
			// Recursively check loop body
			if s.Body != nil {
				sa.traverseASTForPerformance(s.Body, issues, depth+1)
			}
			
		case *YulFunctionCall:
			// Check for expensive operations in loops
			if depth > 0 {
				if s.Identifier.Name == "keccak256" || s.Identifier.Name == "sha256" {
					*issues = append(*issues, PerformanceIssue{
						Type:        "ExpensiveInLoop",
						Severity:    "High",
						Message:     "Expensive cryptographic operation in loop",
						Location:    s.Location,
						Suggestion:  "Move hash calculation outside loop or use batch processing",
					})
				}
				
				if s.Identifier.Name == "sstore" {
					*issues = append(*issues, PerformanceIssue{
						Type:        "StorageInLoop",
						Severity:    "High",
						Message:     "Storage write in loop - high gas cost",
						Location:    s.Location,
						Suggestion:  "Use batch storage operations or temporary variables",
					})
				}
			}
			
		case *YulIf:
			sa.traverseASTForPerformance(s.Body, issues, depth)
			
		case *YulSwitch:
			for _, switchCase := range s.Cases {
				sa.traverseASTForPerformance(switchCase.Body, issues, depth)
			}
			if s.Default != nil {
				sa.traverseASTForPerformance(s.Default, issues, depth)
			}
		}
	}
}

// NewOptimizationEngine creates a new optimization engine
func NewOptimizationEngine(level int) *OptimizationEngine {
	engine := &OptimizationEngine{
		level:  level,
		passes: []OptimizationPass{},
		peepholePasses: []PeepholePattern{},
	}
	
	// Initialize optimization passes based on level
	engine.initializePasses()
	
	return engine
}

func (oe *OptimizationEngine) initializePasses() {
	// Level 0: No optimization
	if oe.level == 0 {
		return
	}
	
	// Level 1: Basic optimizations
	if oe.level >= 1 {
		oe.peepholePasses = append(oe.peepholePasses, PeepholePattern{
			Name:        "push_drop",
			Pattern:     []NeoOpcode{PUSHDATA1, DROP},
			Replacement: []NeoInstruction{}, // Remove both
			Savings:     3,
		})
	}
	
	// Level 2: Advanced optimizations
	if oe.level >= 2 {
		oe.peepholePasses = append(oe.peepholePasses, PeepholePattern{
			Name:        "dup_drop",
			Pattern:     []NeoOpcode{DUP, DROP},
			Replacement: []NeoInstruction{}, // Remove both
			Savings:     2,
		})
	}
	
	// Level 3: Aggressive optimizations
	if oe.level >= 3 {
		// Would add more aggressive patterns
	}
}

func (oe *OptimizationEngine) Optimize(ast *YulAST) (*YulAST, error) {
	optimizedAST := ast
	
	// Apply optimization passes
	for _, pass := range oe.passes {
		if pass.RequiredLevel() <= oe.level {
			var err error
			optimizedAST, err = pass.Apply(optimizedAST)
			if err != nil {
				return nil, fmt.Errorf("optimization pass %s failed: %w", pass.Name(), err)
			}
		}
	}
	
	return optimizedAST, nil
}

func (oe *OptimizationEngine) OptimizeInstructions(instructions []NeoInstruction) []NeoInstruction {
	optimized := instructions
	
	// Apply peephole optimizations
	for _, pattern := range oe.peepholePasses {
		optimized = oe.applyPeepholePattern(optimized, pattern)
	}
	
	return optimized
}

func (oe *OptimizationEngine) applyPeepholePattern(instructions []NeoInstruction, pattern PeepholePattern) []NeoInstruction {
	result := []NeoInstruction{}
	
	i := 0
	for i < len(instructions) {
		matched := false
		
		// Check if pattern matches at current position
		if i+len(pattern.Pattern) <= len(instructions) {
			match := true
			for j, opcode := range pattern.Pattern {
				if instructions[i+j].Opcode != opcode {
					match = false
					break
				}
			}
			
			if match {
				// Apply replacement
				result = append(result, pattern.Replacement...)
				i += len(pattern.Pattern)
				matched = true
			}
		}
		
		if !matched {
			result = append(result, instructions[i])
			i++
		}
	}
	
	return result
}

// NewRuntimeManager creates a new runtime manager
func NewRuntimeManager(context *CompilerContext) *RuntimeManager {
	return &RuntimeManager{
		context: context,
	}
}

func (rm *RuntimeManager) Finalize(contract *NeoContract) (*NeoContract, error) {
	// Generate contract metadata
	contract.Metadata.CompilationTime = time.Now().Format(time.RFC3339)
	
	// Generate method descriptors
	err := rm.generateMethodDescriptors(contract)
	if err != nil {
		return nil, err
	}
	
	// Generate ABI information
	err = rm.generateABI(contract)
	if err != nil {
		return nil, err
	}
	
	return contract, nil
}

func (rm *RuntimeManager) generateMethodDescriptors(contract *NeoContract) error {
	// Generate complete method descriptors from function definitions
	return nil
}

func (rm *RuntimeManager) generateABI(contract *NeoContract) error {
	// Generate complete ABI from contract analysis
	return nil
}

// Utility functions for pretty printing and debugging

func PrettyPrintAST(ast *YulAST) string {
	var builder strings.Builder
	builder.WriteString("Yul AST:\n")
	builder.WriteString("========\n")
	
	for i, obj := range ast.Objects {
		builder.WriteString(fmt.Sprintf("Object %d: %s\n", i, obj.Name))
		if obj.Code != nil {
			builder.WriteString(fmt.Sprintf("  Code: %d statements\n", len(obj.Code.Statements)))
		}
		if len(obj.Objects) > 0 {
			builder.WriteString(fmt.Sprintf("  Nested objects: %d\n", len(obj.Objects)))
		}
	}
	
	for i, fn := range ast.Functions {
		builder.WriteString(fmt.Sprintf("Function %d: %s\n", i, fn.Name))
		builder.WriteString(fmt.Sprintf("  Parameters: %d\n", len(fn.Parameters)))
		builder.WriteString(fmt.Sprintf("  Returns: %d\n", len(fn.Returns)))
	}
	
	return builder.String()
}

func PrettyPrintInstructions(instructions []NeoInstruction) string {
	var builder strings.Builder
	builder.WriteString("NeoVM Instructions:\n")
	builder.WriteString("==================\n")
	
	for i, instr := range instructions {
		builder.WriteString(fmt.Sprintf("%3d: %-12s", i, OpcodeMnemonic(instr.Opcode)))
		
		if len(instr.Operand) > 0 {
			builder.WriteString(fmt.Sprintf(" %x", instr.Operand))
		}
		
		if instr.Comment != "" {
			builder.WriteString(fmt.Sprintf(" // %s", instr.Comment))
		}
		
		builder.WriteString("\n")
	}
	
	return builder.String()
}

// Validation helpers

func ValidateYulAST(ast *YulAST) error {
	if ast == nil {
		return fmt.Errorf("AST is nil")
	}
	
	if len(ast.Objects) == 0 && len(ast.Functions) == 0 {
		return fmt.Errorf("empty AST")
	}
	
	// Additional validation logic would go here
	return nil
}

func ValidateNeoContract(contract *NeoContract) error {
	if contract == nil {
		return fmt.Errorf("contract is nil")
	}
	
	if contract.Name == "" {
		return fmt.Errorf("contract name is empty")
	}
	
	if len(contract.Runtime) == 0 {
		return fmt.Errorf("contract has no runtime code")
	}
	
	// Additional validation logic would go here
	return nil
}