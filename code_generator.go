package main

import (
	"fmt"
	"math/big"
	"strings"
)

// CodeGenerator translates normalized Yul IR into NeoVM bytecode
type CodeGenerator struct {
	context          *CompilerContext
	instructions     []NeoInstruction
	labelMap         map[string]int
	pendingLabels    []PendingLabel
	stackTracker     *StackTracker
	functionTable    map[string]*FunctionInfo
	currentFunction  string
	exceptionHandlers []ExceptionHandler
}

// PendingLabel represents a label that needs to be resolved later
type PendingLabel struct {
	Name             string
	InstructionIndex int
	Offset           int
}

// StackTracker maintains stack depth analysis during code generation
type StackTracker struct {
	currentDepth int
	maxDepth     int
	stackMap     map[int]int // instruction index -> stack depth
}

// FunctionInfo contains metadata about compiled functions
type FunctionInfo struct {
	Name         string
	StartOffset  int
	EndOffset    int
	Parameters   int
	Returns      int
	LocalVars    int
	MaxStack     int
}

// NewCodeGenerator creates a new code generator instance
func NewCodeGenerator(context *CompilerContext) *CodeGenerator {
	return &CodeGenerator{
		context:       context,
		instructions:  []NeoInstruction{},
		labelMap:      make(map[string]int),
		pendingLabels: []PendingLabel{},
		stackTracker: &StackTracker{
			currentDepth: 0,
			maxDepth:     0,
			stackMap:     make(map[int]int),
		},
		functionTable:     make(map[string]*FunctionInfo),
		exceptionHandlers: []ExceptionHandler{},
	}
}

// Generate translates a Yul AST into NeoVM bytecode
func (g *CodeGenerator) Generate(ast *YulAST) (*NeoContract, error) {
	contract := &NeoContract{
		Name:        "YulContract",
		Version:     "1.0.0",
		Methods:     []*ContractMethod{},
		Events:      []*ContractEvent{},
		EntryPoints: make(map[string]int),
		Constants:   make(map[string]NeoVMStackItem),
		SourceMap:   make(map[int]SourcePosition),
		Metadata: &ContractMetadata{
			Compiler: CompilerInfo{
				Version: "1.0.0",
				Target:  "NeoVM",
			},
		},
	}

	// Process all objects in the AST
	for _, obj := range ast.Objects {
		err := g.generateObject(obj, contract)
		if err != nil {
			return nil, fmt.Errorf("error generating object %s: %w", obj.Name, err)
		}
	}

	// Resolve pending labels
	err := g.resolveLabels()
	if err != nil {
		return nil, fmt.Errorf("error resolving labels: %w", err)
	}

	// Set final instruction sequences
	contract.Runtime = g.instructions
	contract.EntryPoints = g.labelMap

	return contract, nil
}

// generateObject processes a Yul object (contract or code block)
func (g *CodeGenerator) generateObject(obj *YulObject, contract *NeoContract) error {
	switch obj.Type {
	case ObjectTypeContract, ObjectTypeRuntime:
		if obj.Code != nil {
			return g.generateBlock(obj.Code)
		}
	}

	// Process nested objects
	for _, nestedObj := range obj.Objects {
		err := g.generateObject(nestedObj, contract)
		if err != nil {
			return err
		}
	}

	return nil
}

// generateBlock processes a Yul block of statements
func (g *CodeGenerator) generateBlock(block *YulBlock) error {
	for _, stmt := range block.Statements {
		err := g.generateStatement(stmt)
		if err != nil {
			return fmt.Errorf("error generating statement: %w", err)
		}
	}
	return nil
}

// generateStatement dispatches statement generation based on type
func (g *CodeGenerator) generateStatement(stmt YulStatement) error {
	switch s := stmt.(type) {
	case *YulExpressionStatement:
		return g.generateExpressionStatement(s)
	case *YulVariableDeclaration:
		return g.generateVariableDeclaration(s)
	case *YulAssignment:
		return g.generateAssignment(s)
	case *YulIf:
		return g.generateIf(s)
	case *YulSwitch:
		return g.generateSwitch(s)
	case *YulFor:
		return g.generateFor(s)
	case *YulFunctionDef:
		return g.generateFunctionDef(s)
	case *YulBreak:
		return g.generateBreak(s)
	case *YulContinue:
		return g.generateContinue(s)
	case *YulLeave:
		return g.generateLeave(s)
	default:
		return fmt.Errorf("unsupported statement type: %T", stmt)
	}
}

// generateExpressionStatement processes expression statements
func (g *CodeGenerator) generateExpressionStatement(stmt *YulExpressionStatement) error {
	if stmt.Expression != nil {
		err := g.generateExpression(stmt.Expression)
		if err != nil {
			return err
		}
		// Pop the result since it's not used
		g.emitInstruction(NewStackInstruction(DROP, 0), stmt.Location)
	}
	return nil
}

// generateVariableDeclaration processes variable declarations
func (g *CodeGenerator) generateVariableDeclaration(stmt *YulVariableDeclaration) error {
	// Generate initial value if provided
	if stmt.Value != nil {
		err := g.generateExpression(stmt.Value)
		if err != nil {
			return err
		}
	} else {
		// Push default zero value for each variable
		for range stmt.Variables {
			g.emitInstruction(NewPushInstruction(CreateNeoVMInteger(0)), stmt.Location)
		}
	}

	// Variables are now on the stack
	// In a more complete implementation, we would track variable locations
	return nil
}

// generateAssignment processes variable assignments
func (g *CodeGenerator) generateAssignment(stmt *YulAssignment) error {
	// Generate the right-hand side value
	err := g.generateExpression(stmt.Value)
	if err != nil {
		return err
	}

	// For multiple assignment targets, duplicate the value
	for i := range stmt.VariableNames {
		if i < len(stmt.VariableNames)-1 {
			g.emitInstruction(NewStackInstruction(DUP, 0), stmt.Location)
		}
	}

	// In a complete implementation, we would store to actual variable locations
	// Maintain values on stack for efficient access
	return nil
}

// generateIf processes conditional statements
func (g *CodeGenerator) generateIf(stmt *YulIf) error {
	// Generate condition
	err := g.generateExpression(stmt.Condition)
	if err != nil {
		return err
	}

	// Jump to end if condition is false
	endLabel := g.createUniqueLabel("if_end")
	g.emitInstruction(NewControlFlowInstruction(JMPIFNOT, 0), stmt.Location)
	g.addPendingLabel(endLabel, len(g.instructions)-1)

	// Generate body
	err = g.generateBlock(stmt.Body)
	if err != nil {
		return err
	}

	// Mark end label
	g.markLabel(endLabel)
	return nil
}

// generateSwitch processes switch statements
func (g *CodeGenerator) generateSwitch(stmt *YulSwitch) error {
	// Generate switch expression
	err := g.generateExpression(stmt.Expression)
	if err != nil {
		return err
	}

	endLabel := g.createUniqueLabel("switch_end")
	
	// Generate case comparisons and jumps
	for _, caseStmt := range stmt.Cases {
		// Duplicate switch value for comparison
		g.emitInstruction(NewStackInstruction(DUP, 0), stmt.Location)
		
		// Push case value
		err = g.generateLiteral(&caseStmt.Value)
		if err != nil {
			return err
		}
		
		// Compare
		g.emitInstruction(NewArithmeticInstruction(EQUAL), stmt.Location)
		
		// Jump to case body if equal
		caseLabel := g.createUniqueLabel("case_" + caseStmt.Value.Value)
		g.emitInstruction(NewControlFlowInstruction(JMPIF, 0), stmt.Location)
		g.addPendingLabel(caseLabel, len(g.instructions)-1)
		
		// Generate case body (will be placed later)
		defer func(c *YulCase, label string) {
			g.markLabel(label)
			g.generateBlock(c.Body)
			g.emitInstruction(NewControlFlowInstruction(JMP, 0), c.Location)
			g.addPendingLabel(endLabel, len(g.instructions)-1)
		}(caseStmt, caseLabel)
	}

	// Generate default case if present
	if stmt.Default != nil {
		err = g.generateBlock(stmt.Default)
		if err != nil {
			return err
		}
	}

	// Clean up switch value from stack
	g.emitInstruction(NewStackInstruction(DROP, 0), stmt.Location)

	// Mark end label
	g.markLabel(endLabel)
	return nil
}

// generateFor processes for loops
func (g *CodeGenerator) generateFor(stmt *YulFor) error {
	// Generate initialization
	err := g.generateBlock(stmt.Init)
	if err != nil {
		return err
	}

	// Loop start label
	loopStart := g.createUniqueLabel("for_start")
	g.markLabel(loopStart)

	// Generate condition
	err = g.generateExpression(stmt.Condition)
	if err != nil {
		return err
	}

	// Jump to end if condition is false
	loopEnd := g.createUniqueLabel("for_end")
	g.emitInstruction(NewControlFlowInstruction(JMPIFNOT, 0), stmt.Location)
	g.addPendingLabel(loopEnd, len(g.instructions)-1)

	// Generate body
	err = g.generateBlock(stmt.Body)
	if err != nil {
		return err
	}

	// Generate post increment
	err = g.generateBlock(stmt.Post)
	if err != nil {
		return err
	}

	// Jump back to start
	g.emitInstruction(NewControlFlowInstruction(JMP, 0), stmt.Location)
	g.addPendingLabel(loopStart, len(g.instructions)-1)

	// Mark end label
	g.markLabel(loopEnd)
	return nil
}

// generateFunctionDef processes function definitions
func (g *CodeGenerator) generateFunctionDef(stmt *YulFunctionDef) error {
	// Mark function entry point
	functionLabel := "func_" + stmt.Name
	g.markLabel(functionLabel)
	
	startOffset := len(g.instructions)
	g.currentFunction = stmt.Name

	// Create function info
	funcInfo := &FunctionInfo{
		Name:        stmt.Name,
		StartOffset: startOffset,
		Parameters:  len(stmt.Parameters),
		Returns:     len(stmt.Returns),
		LocalVars:   0, // Will be calculated during generation
		MaxStack:    g.stackTracker.currentDepth,
	}

	// Generate function body
	err := g.generateBlock(stmt.Body)
	if err != nil {
		return err
	}

	// Generate return instruction
	g.emitInstruction(NewControlFlowInstruction(RET, 0), stmt.Location)

	funcInfo.EndOffset = len(g.instructions)
	funcInfo.MaxStack = g.stackTracker.maxDepth
	g.functionTable[stmt.Name] = funcInfo

	g.currentFunction = ""
	return nil
}

// generateExpression dispatches expression generation based on type
func (g *CodeGenerator) generateExpression(expr YulExpression) error {
	switch e := expr.(type) {
	case *YulFunctionCall:
		return g.generateFunctionCall(e)
	case *YulIdentifier:
		return g.generateIdentifier(e)
	case *YulLiteral:
		return g.generateLiteral(e)
	default:
		return fmt.Errorf("unsupported expression type: %T", expr)
	}
}

// generateFunctionCall processes function calls (built-ins and user-defined)
func (g *CodeGenerator) generateFunctionCall(call *YulFunctionCall) error {
	functionName := call.FunctionName.Name

	// Generate arguments (pushed in reverse order for stack convention)
	for i := len(call.Arguments) - 1; i >= 0; i-- {
		err := g.generateExpression(call.Arguments[i])
		if err != nil {
			return err
		}
	}

	// Handle built-in functions
	if g.isBuiltinFunction(functionName) {
		return g.generateBuiltinCall(functionName, len(call.Arguments), call.Location)
	}

	// Handle user-defined function calls
	g.emitInstruction(NewControlFlowInstruction(CALL, 0), call.Location)
	g.addPendingLabel("func_"+functionName, len(g.instructions)-1)

	return nil
}

// generateBuiltinCall generates code for built-in Yul functions
func (g *CodeGenerator) generateBuiltinCall(name string, argCount int, location SourcePosition) error {
	switch name {
	// Arithmetic operations
	case "add":
		g.emitInstruction(NewArithmeticInstruction(ADD), location)
	case "sub":
		g.emitInstruction(NewArithmeticInstruction(SUB), location)
	case "mul":
		g.emitInstruction(NewArithmeticInstruction(MUL), location)
	case "div":
		// Add division by zero check
		g.emitDivisionByZeroCheck(location)
		g.emitInstruction(NewArithmeticInstruction(DIV), location)
	case "mod":
		g.emitDivisionByZeroCheck(location)
		g.emitInstruction(NewArithmeticInstruction(MOD), location)
	case "lt":
		g.emitInstruction(NewArithmeticInstruction(LT), location)
	case "gt":
		g.emitInstruction(NewArithmeticInstruction(GT), location)
	case "eq":
		g.emitInstruction(NewArithmeticInstruction(EQUAL), location)
	case "iszero":
		g.emitInstruction(NewPushInstruction(CreateNeoVMInteger(0)), location)
		g.emitInstruction(NewArithmeticInstruction(EQUAL), location)
	case "and":
		g.emitInstruction(NewArithmeticInstruction(AND), location)
	case "or":
		g.emitInstruction(NewArithmeticInstruction(OR), location)
	case "xor":
		g.emitInstruction(NewArithmeticInstruction(XOR), location)
	case "not":
		g.emitInstruction(NewArithmeticInstruction(NOT), location)
	case "shl":
		g.emitInstruction(NewArithmeticInstruction(SHL), location)
	case "shr":
		g.emitInstruction(NewArithmeticInstruction(SHR), location)

	// Memory operations (mapped to NeoVM syscalls)
	case "mload":
		g.emitInstruction(NewSyscallInstruction("System.Storage.Get"), location)
	case "mstore":
		g.emitInstruction(NewSyscallInstruction("System.Storage.Put"), location)
	case "mstore8":
		// Store single byte - convert to full storage operation
		g.emitInstruction(NewSyscallInstruction("System.Storage.Put"), location)
	case "msize":
		// Return current memory size - approximate with storage context
		g.emitInstruction(NewSyscallInstruction("System.Storage.GetContext"), location)
		g.emitInstruction(NewSyscallInstruction("System.Storage.GetReadOnlyContext"), location)

	// Storage operations
	case "sload":
		g.emitInstruction(NewSyscallInstruction("System.Storage.Get"), location)
	case "sstore":
		g.emitInstruction(NewSyscallInstruction("System.Storage.Put"), location)

	// Call data operations
	case "calldataload":
		g.emitInstruction(NewSyscallInstruction("System.Runtime.GetArgument"), location)
	case "calldatasize":
		g.emitInstruction(NewSyscallInstruction("System.Runtime.GetArgumentCount"), location)
	case "calldatacopy":
		// Approximate with multiple argument loads
		g.emitInstruction(NewSyscallInstruction("System.Runtime.GetArgument"), location)

	// Environment operations
	case "caller":
		g.emitInstruction(NewSyscallInstruction("System.Runtime.GetCallingScriptHash"), location)
	case "callvalue":
		g.emitInstruction(NewSyscallInstruction("System.Runtime.GetInvocationCounter"), location)
	case "address":
		g.emitInstruction(NewSyscallInstruction("System.Runtime.GetExecutingScriptHash"), location)
	case "balance":
		g.emitInstruction(NewSyscallInstruction("System.Runtime.GetExecutingScriptHash"), location)
		g.emitInstruction(NewSyscallInstruction("Neo.Native.GAS.balanceOf"), location)

	// Control flow operations
	case "revert":
		g.emitInstruction(NewControlFlowInstruction(ABORT, 0), location)
	case "return":
		// Return data from stack top with proper type conversion
		g.emitInstruction(NewControlFlowInstruction(RET, 0), location)
	case "stop":
		g.emitInstruction(NewControlFlowInstruction(RET, 0), location)

	// Logging operations
	case "log0", "log1", "log2", "log3", "log4":
		g.emitInstruction(NewSyscallInstruction("System.Runtime.Notify"), location)

	// Hashing operations
	case "keccak256":
		g.emitInstruction(NewSyscallInstruction("Neo.Crypto.Keccak256"), location)
	case "sha256":
		g.emitInstruction(NewSyscallInstruction("Neo.Crypto.Sha256"), location)

	default:
		return fmt.Errorf("unsupported built-in function: %s", name)
	}

	return nil
}

// generateIdentifier processes variable references
func (g *CodeGenerator) generateIdentifier(ident *YulIdentifier) error {
	// In a complete implementation, this would load from variable storage
	// Variables managed on stack with spill to storage as needed
	g.emitInstruction(NewStackInstruction(DUP, 0), ident.Location)
	return nil
}

// generateLiteral processes literal values
func (g *CodeGenerator) generateLiteral(lit *YulLiteral) error {
	switch lit.Kind {
	case LiteralKindNumber:
		value := CreateNeoVMInteger(lit.Value)
		g.emitInstruction(NewPushInstruction(value), lit.Location)
	case LiteralKindString:
		value := CreateNeoVMByteString(lit.Value)
		g.emitInstruction(NewPushInstruction(value), lit.Location)
	case LiteralKindBool:
		value := CreateNeoVMBoolean(lit.Value == "true")
		g.emitInstruction(NewPushInstruction(value), lit.Location)
	case LiteralKindHex:
		// Convert hex string to bytes
		hexStr := strings.TrimPrefix(lit.Value, "0x")
		if len(hexStr)%2 != 0 {
			hexStr = "0" + hexStr
		}
		bytes := make([]byte, len(hexStr)/2)
		for i := 0; i < len(hexStr); i += 2 {
			var b byte
			fmt.Sscanf(hexStr[i:i+2], "%02x", &b)
			bytes[i/2] = b
		}
		value := CreateNeoVMByteString(bytes)
		g.emitInstruction(NewPushInstruction(value), lit.Location)
	default:
		return fmt.Errorf("unsupported literal kind: %s", lit.Kind)
	}
	return nil
}

// Helper functions for control flow and optimization

func (g *CodeGenerator) generateBreak(stmt *YulBreak) error {
	// Jump to loop end - would need to track loop context
	g.emitInstruction(NewControlFlowInstruction(JMP, 0), stmt.Location)
	return nil
}

func (g *CodeGenerator) generateContinue(stmt *YulContinue) error {
	// Jump to loop continuation - would need to track loop context
	g.emitInstruction(NewControlFlowInstruction(JMP, 0), stmt.Location)
	return nil
}

func (g *CodeGenerator) generateLeave(stmt *YulLeave) error {
	// Early function return
	g.emitInstruction(NewControlFlowInstruction(RET, 0), stmt.Location)
	return nil
}

// Utility functions

func (g *CodeGenerator) emitInstruction(instr NeoInstruction, location SourcePosition) {
	instr.SourceRef = &location
	g.instructions = append(g.instructions, instr)
	
	// Update stack tracking
	g.stackTracker.stackMap[len(g.instructions)-1] = g.stackTracker.currentDepth
	g.stackTracker.currentDepth -= instr.StackPop
	g.stackTracker.currentDepth += instr.StackPush
	
	if g.stackTracker.currentDepth > g.stackTracker.maxDepth {
		g.stackTracker.maxDepth = g.stackTracker.currentDepth
	}
}

func (g *CodeGenerator) createUniqueLabel(prefix string) string {
	g.context.LabelCounter++
	return fmt.Sprintf("%s_%d", prefix, g.context.LabelCounter)
}

func (g *CodeGenerator) markLabel(name string) {
	g.labelMap[name] = len(g.instructions)
}

func (g *CodeGenerator) addPendingLabel(name string, instrIndex int) {
	g.pendingLabels = append(g.pendingLabels, PendingLabel{
		Name:             name,
		InstructionIndex: instrIndex,
		Offset:           4, // NeoVM uses 4-byte addresses
	})
}

func (g *CodeGenerator) resolveLabels() error {
	for _, pending := range g.pendingLabels {
		if targetOffset, exists := g.labelMap[pending.Name]; exists {
			// Update instruction operand with target address
			instr := &g.instructions[pending.InstructionIndex]
			if len(instr.Operand) >= 4 {
				instr.Operand[0] = byte(targetOffset)
				instr.Operand[1] = byte(targetOffset >> 8)
				instr.Operand[2] = byte(targetOffset >> 16)
				instr.Operand[3] = byte(targetOffset >> 24)
			}
		} else {
			return fmt.Errorf("undefined label: %s", pending.Name)
		}
	}
	return nil
}

func (g *CodeGenerator) isBuiltinFunction(name string) bool {
	builtins := []string{
		"add", "sub", "mul", "div", "mod", "exp",
		"lt", "gt", "eq", "iszero", "and", "or", "xor", "not",
		"shl", "shr", "sar", "byte", "sload", "sstore",
		"mload", "mstore", "mstore8", "msize",
		"calldataload", "calldatasize", "calldatacopy",
		"caller", "callvalue", "address", "balance",
		"revert", "return", "stop", "keccak256", "sha256",
		"log0", "log1", "log2", "log3", "log4",
	}
	
	for _, builtin := range builtins {
		if name == builtin {
			return true
		}
	}
	return false
}

func (g *CodeGenerator) emitDivisionByZeroCheck(location SourcePosition) {
	// Duplicate divisor for check
	g.emitInstruction(NewStackInstruction(DUP, 0), location)
	
	// Check if zero
	g.emitInstruction(NewPushInstruction(CreateNeoVMInteger(0)), location)
	g.emitInstruction(NewArithmeticInstruction(EQUAL), location)
	
	// Jump to error if zero
	errorLabel := g.createUniqueLabel("div_zero_error")
	g.emitInstruction(NewControlFlowInstruction(JMPIF, 0), location)
	g.addPendingLabel(errorLabel, len(g.instructions)-1)
	
	// Continue normally
	continueLabel := g.createUniqueLabel("div_continue")
	g.emitInstruction(NewControlFlowInstruction(JMP, 0), location)
	g.addPendingLabel(continueLabel, len(g.instructions)-1)
	
	// Error handling
	g.markLabel(errorLabel)
	g.emitInstruction(NewControlFlowInstruction(ABORT, 0), location)
	
	// Continue point
	g.markLabel(continueLabel)
}

// GetCompilationStats returns compilation statistics
func (g *CodeGenerator) GetCompilationStats() CompilationStats {
	return CompilationStats{
		CompiledSizeBytes: len(g.instructions) * 4, // Approximate
		FunctionsCompiled: len(g.functionTable),
	}
}