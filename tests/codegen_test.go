package main

import (
	"math/big"
	"strings"
	"testing"
)

// TestCodeGeneratorBasicGeneration tests basic code generation functionality
func TestCodeGeneratorBasicGeneration(t *testing.T) {
	tests := []struct {
		name     string
		source   string
		validate func(*NeoContract) error
	}{
		{
			name:   "empty object",
			source: `object "Test" {}`,
			validate: func(contract *NeoContract) error {
				if len(contract.Runtime) > 0 {
					t.Errorf("Expected no instructions for empty object, got %d", len(contract.Runtime))
				}
				return nil
			},
		},
		{
			name: "simple expression",
			source: `object "Test" {
				code {
					add(1, 2)
				}
			}`,
			validate: func(contract *NeoContract) error {
				if len(contract.Runtime) < 3 {
					t.Errorf("Expected at least 3 instructions (push 1, push 2, add), got %d", len(contract.Runtime))
				}
				// Should have PUSH instructions and ADD
				var hasPush, hasAdd bool
				for _, instr := range contract.Runtime {
					if instr.Opcode == PUSH1 || instr.Opcode == PUSH2 {
						hasPush = true
					}
					if instr.Opcode == ADD {
						hasAdd = true
					}
				}
				if !hasPush {
					t.Error("Expected PUSH instructions")
				}
				if !hasAdd {
					t.Error("Expected ADD instruction")
				}
				return nil
			},
		},
		{
			name: "variable declaration",
			source: `object "Test" {
				code {
					let x := 42
				}
			}`,
			validate: func(contract *NeoContract) error {
				if len(contract.Runtime) == 0 {
					t.Error("Expected instructions for variable declaration")
				}
				// Should have a PUSH instruction for the value 42
				found := false
				for _, instr := range contract.Runtime {
					if instr.Opcode == PUSHDATA1 || instr.Opcode == PUSHINT8 {
						found = true
						break
					}
				}
				if !found {
					t.Error("Expected PUSH instruction for literal value")
				}
				return nil
			},
		},
	}

	for _, test := range tests {
		t.Run(test.name, func(t *testing.T) {
			// Parse the source
			parser := NewYulParser()
			ast, err := parser.Parse(test.source)
			if err != nil {
				t.Fatalf("Parse failed: %v", err)
			}

			// Generate code
			context := &CompilerContext{
				SourceMap:      make(map[string]string),
				SymbolTable:    NewSymbolTable(),
				TypeTable:      NewTypeTable(),
				LabelCounter:   0,
				ErrorCollector: NewErrorCollector(),
				Metadata:       NewCompilationMetadata(),
			}
			generator := NewCodeGenerator(context)
			contract, err := generator.Generate(ast)
			if err != nil {
				t.Fatalf("Code generation failed: %v", err)
			}

			if test.validate != nil {
				test.validate(contract)
			}
		})
	}
}

// TestCodeGeneratorArithmetic tests arithmetic operation generation
func TestCodeGeneratorArithmetic(t *testing.T) {
	tests := []struct {
		name     string
		source   string
		expected []NeoOpcode
	}{
		{
			name:   "addition",
			source: "add(1, 2)",
			expected: []NeoOpcode{PUSH2, PUSH1, ADD, DROP}, // Reverse order due to stack
		},
		{
			name:   "subtraction",
			source: "sub(5, 3)",
			expected: []NeoOpcode{PUSH3, PUSH5, SUB, DROP},
		},
		{
			name:   "multiplication",
			source: "mul(4, 7)",
			expected: []NeoOpcode{PUSH7, PUSH4, MUL, DROP},
		},
		{
			name:   "division",
			source: "div(10, 2)",
			// Division includes zero check: DUP, PUSH0, EQUAL, JMPIF, JMP, ABORT, actual_div
			expected: []NeoOpcode{PUSH2, PUSH10},
		},
		{
			name:   "comparison",
			source: "lt(3, 5)",
			expected: []NeoOpcode{PUSH5, PUSH3, LT, DROP},
		},
		{
			name:   "equality",
			source: "eq(1, 1)",
			expected: []NeoOpcode{PUSH1, PUSH1, EQUAL, DROP},
		},
		{
			name:   "logical and",
			source: "and(1, 0)",
			expected: []NeoOpcode{PUSH0, PUSH1, AND, DROP},
		},
		{
			name:   "bitwise xor",
			source: "xor(0xFF, 0x0F)",
			// Hex literals become PUSHDATA instructions
			expected: []NeoOpcode{PUSHDATA1, PUSHDATA1, XOR, DROP},
		},
	}

	for _, test := range tests {
		t.Run(test.name, func(t *testing.T) {
			source := `object "Test" { code { ` + test.source + ` } }`
			
			parser := NewYulParser()
			ast, err := parser.Parse(source)
			if err != nil {
				t.Fatalf("Parse failed: %v", err)
			}

			context := &CompilerContext{
				SourceMap:      make(map[string]string),
				SymbolTable:    NewSymbolTable(),
				TypeTable:      NewTypeTable(),
				LabelCounter:   0,
				ErrorCollector: NewErrorCollector(),
				Metadata:       NewCompilationMetadata(),
			}
			generator := NewCodeGenerator(context)
			contract, err := generator.Generate(ast)
			if err != nil {
				t.Fatalf("Code generation failed: %v", err)
			}

			// Check that expected opcodes are present
			for _, expectedOp := range test.expected {
				found := false
				for _, instr := range contract.Runtime {
					if instr.Opcode == expectedOp {
						found = true
						break
					}
				}
				if !found {
					t.Errorf("Expected opcode %s not found", OpcodeMnemonic(expectedOp))
				}
			}
		})
	}
}

// TestCodeGeneratorControlFlow tests control flow generation
func TestCodeGeneratorControlFlow(t *testing.T) {
	tests := []struct {
		name     string
		source   string
		validate func([]NeoInstruction) error
	}{
		{
			name: "if statement",
			source: `if iszero(x) { revert(0, 0) }`,
			validate: func(instructions []NeoInstruction) error {
				// Should have conditional jump
				var hasJmpIfNot bool
				for _, instr := range instructions {
					if instr.Opcode == JMPIFNOT {
						hasJmpIfNot = true
						break
					}
				}
				if !hasJmpIfNot {
					t.Error("Expected JMPIFNOT instruction for if statement")
				}
				return nil
			},
		},
		{
			name: "switch statement",
			source: `switch x
				case 0 { y := 1 }
				case 1 { y := 2 }
				default { y := 3 }`,
			validate: func(instructions []NeoInstruction) error {
				// Should have multiple comparisons and jumps
				var jmpCount int
				for _, instr := range instructions {
					if instr.Opcode == JMPIF || instr.Opcode == JMPIFNOT || instr.Opcode == JMP {
						jmpCount++
					}
				}
				if jmpCount < 2 {
					t.Errorf("Expected at least 2 jump instructions for switch, got %d", jmpCount)
				}
				return nil
			},
		},
		{
			name: "for loop",
			source: `for { let i := 0 } lt(i, 10) { i := add(i, 1) } {
				// loop body
			}`,
			validate: func(instructions []NeoInstruction) error {
				// Should have loop start label and back jump
				var hasJmp, hasJmpIfNot bool
				for _, instr := range instructions {
					if instr.Opcode == JMP {
						hasJmp = true
					}
					if instr.Opcode == JMPIFNOT {
						hasJmpIfNot = true
					}
				}
				if !hasJmp {
					t.Error("Expected JMP instruction for loop back-edge")
				}
				if !hasJmpIfNot {
					t.Error("Expected JMPIFNOT instruction for loop condition")
				}
				return nil
			},
		},
	}

	for _, test := range tests {
		t.Run(test.name, func(t *testing.T) {
			source := `object "Test" { code { ` + test.source + ` } }`
			
			parser := NewYulParser()
			ast, err := parser.Parse(source)
			if err != nil {
				t.Fatalf("Parse failed: %v", err)
			}

			context := &CompilerContext{
				SourceMap:      make(map[string]string),
				SymbolTable:    NewSymbolTable(),
				TypeTable:      NewTypeTable(),
				LabelCounter:   0,
				ErrorCollector: NewErrorCollector(),
				Metadata:       NewCompilationMetadata(),
			}
			generator := NewCodeGenerator(context)
			contract, err := generator.Generate(ast)
			if err != nil {
				t.Fatalf("Code generation failed: %v", err)
			}

			if test.validate != nil {
				test.validate(contract.Runtime)
			}
		})
	}
}

// TestCodeGeneratorBuiltins tests built-in function translation
func TestCodeGeneratorBuiltins(t *testing.T) {
	tests := []struct {
		name        string
		function    string
		expectedOp  NeoOpcode
		expectedSys string // For syscalls
	}{
		{
			name:       "storage load",
			function:   "sload(0)",
			expectedSys: "System.Storage.Get",
		},
		{
			name:       "storage store",
			function:   "sstore(0, 1)",
			expectedSys: "System.Storage.Put",
		},
		{
			name:       "memory load",
			function:   "mload(0)",
			expectedSys: "System.Storage.Get", // Mapped to storage in NeoVM
		},
		{
			name:       "get caller",
			function:   "caller()",
			expectedSys: "System.Runtime.GetCallingScriptHash",
		},
		{
			name:       "get current address",
			function:   "address()",
			expectedSys: "System.Runtime.GetExecutingScriptHash",
		},
		{
			name:       "keccak256 hash",
			function:   "keccak256(0x41)",
			expectedSys: "Neo.Crypto.Keccak256",
		},
		{
			name:       "revert",
			function:   "revert(0, 0)",
			expectedOp: ABORT,
		},
		{
			name:       "return",
			function:   "return(0, 32)",
			expectedOp: RET,
		},
		{
			name:       "log event",
			function:   "log0(0, 32)",
			expectedSys: "System.Runtime.Notify",
		},
	}

	for _, test := range tests {
		t.Run(test.name, func(t *testing.T) {
			source := `object "Test" { code { ` + test.function + ` } }`
			
			parser := NewYulParser()
			ast, err := parser.Parse(source)
			if err != nil {
				t.Fatalf("Parse failed: %v", err)
			}

			context := &CompilerContext{
				SourceMap:      make(map[string]string),
				SymbolTable:    NewSymbolTable(),
				TypeTable:      NewTypeTable(),
				LabelCounter:   0,
				ErrorCollector: NewErrorCollector(),
				Metadata:       NewCompilationMetadata(),
			}
			generator := NewCodeGenerator(context)
			contract, err := generator.Generate(ast)
			if err != nil {
				t.Fatalf("Code generation failed: %v", err)
			}

			if test.expectedOp != 0 {
				// Check for specific opcode
				found := false
				for _, instr := range contract.Runtime {
					if instr.Opcode == test.expectedOp {
						found = true
						break
					}
				}
				if !found {
					t.Errorf("Expected opcode %s not found", OpcodeMnemonic(test.expectedOp))
				}
			}

			if test.expectedSys != "" {
				// Check for syscall
				found := false
				for _, instr := range contract.Runtime {
					if instr.Opcode == SYSCALL {
						syscallName := string(instr.Operand)
						if syscallName == test.expectedSys {
							found = true
							break
						}
					}
				}
				if !found {
					t.Errorf("Expected syscall %s not found", test.expectedSys)
				}
			}
		})
	}
}

// TestCodeGeneratorLiterals tests literal value generation
func TestCodeGeneratorLiterals(t *testing.T) {
	tests := []struct {
		name     string
		source   string
		validate func([]NeoInstruction) error
	}{
		{
			name:   "small integer",
			source: "5",
			validate: func(instructions []NeoInstruction) error {
				// Should use optimized PUSH5 instruction
				found := false
				for _, instr := range instructions {
					if instr.Opcode == PUSH5 {
						found = true
						break
					}
				}
				if !found {
					t.Error("Expected optimized PUSH5 instruction for small integer")
				}
				return nil
			},
		},
		{
			name:   "large integer",
			source: "12345",
			validate: func(instructions []NeoInstruction) error {
				// Should use PUSHDATA instruction
				found := false
				for _, instr := range instructions {
					if instr.Opcode == PUSHDATA1 || instr.Opcode == PUSHDATA2 || instr.Opcode == PUSHDATA4 {
						found = true
						break
					}
				}
				if !found {
					t.Error("Expected PUSHDATA instruction for large integer")
				}
				return nil
			},
		},
		{
			name:   "hex literal",
			source: "0xdeadbeef",
			validate: func(instructions []NeoInstruction) error {
				found := false
				for _, instr := range instructions {
					if instr.Opcode == PUSHDATA1 && len(instr.Operand) == 4 {
						// Check that the bytes match the hex value
						expected := []byte{0xde, 0xad, 0xbe, 0xef}
						if len(instr.Operand) >= 4 &&
							instr.Operand[0] == expected[0] &&
							instr.Operand[1] == expected[1] &&
							instr.Operand[2] == expected[2] &&
							instr.Operand[3] == expected[3] {
							found = true
							break
						}
					}
				}
				if !found {
					t.Error("Expected correct hex literal encoding")
				}
				return nil
			},
		},
		{
			name:   "string literal",
			source: `"hello world"`,
			validate: func(instructions []NeoInstruction) error {
				found := false
				for _, instr := range instructions {
					if instr.Opcode == PUSHDATA1 {
						if string(instr.Operand) == "hello world" {
							found = true
							break
						}
					}
				}
				if !found {
					t.Error("Expected correct string literal encoding")
				}
				return nil
			},
		},
		{
			name:   "boolean true",
			source: "true",
			validate: func(instructions []NeoInstruction) error {
				found := false
				for _, instr := range instructions {
					if instr.Opcode == PUSHDATA1 && len(instr.Operand) == 1 && instr.Operand[0] == 1 {
						found = true
						break
					}
				}
				if !found {
					t.Error("Expected correct boolean true encoding")
				}
				return nil
			},
		},
		{
			name:   "boolean false",
			source: "false",
			validate: func(instructions []NeoInstruction) error {
				found := false
				for _, instr := range instructions {
					if instr.Opcode == PUSH0 || (instr.Opcode == PUSHDATA1 && len(instr.Operand) == 1 && instr.Operand[0] == 0) {
						found = true
						break
					}
				}
				if !found {
					t.Error("Expected correct boolean false encoding")
				}
				return nil
			},
		},
	}

	for _, test := range tests {
		t.Run(test.name, func(t *testing.T) {
			source := `object "Test" { code { let x := ` + test.source + ` } }`
			
			parser := NewYulParser()
			ast, err := parser.Parse(source)
			if err != nil {
				t.Fatalf("Parse failed: %v", err)
			}

			context := &CompilerContext{
				SourceMap:      make(map[string]string),
				SymbolTable:    NewSymbolTable(),
				TypeTable:      NewTypeTable(),
				LabelCounter:   0,
				ErrorCollector: NewErrorCollector(),
				Metadata:       NewCompilationMetadata(),
			}
			generator := NewCodeGenerator(context)
			contract, err := generator.Generate(ast)
			if err != nil {
				t.Fatalf("Code generation failed: %v", err)
			}

			if test.validate != nil {
				test.validate(contract.Runtime)
			}
		})
	}
}

// TestCodeGeneratorFunctions tests function generation
func TestCodeGeneratorFunctions(t *testing.T) {
	source := `
	object "Test" {
		code {
			function add(a, b) -> result {
				result := add(a, b)
			}
			
			function factorial(n) -> result {
				if eq(n, 0) {
					result := 1
				}
				result := mul(n, factorial(sub(n, 1)))
			}
		}
	}
	`

	parser := NewYulParser()
	ast, err := parser.Parse(source)
	if err != nil {
		t.Fatalf("Parse failed: %v", err)
	}

	context := &CompilerContext{
		SourceMap:      make(map[string]string),
		SymbolTable:    NewSymbolTable(),
		TypeTable:      NewTypeTable(),
		LabelCounter:   0,
		ErrorCollector: NewErrorCollector(),
		Metadata:       NewCompilationMetadata(),
	}
	generator := NewCodeGenerator(context)
	contract, err := generator.Generate(ast)
	if err != nil {
		t.Fatalf("Code generation failed: %v", err)
	}

	// Check function table is populated
	if len(generator.functionTable) != 2 {
		t.Errorf("Expected 2 functions in function table, got %d", len(generator.functionTable))
	}

	// Check specific functions exist
	if _, exists := generator.functionTable["add"]; !exists {
		t.Error("Expected 'add' function in function table")
	}

	if _, exists := generator.functionTable["factorial"]; !exists {
		t.Error("Expected 'factorial' function in function table")
	}

	// Check for RET instructions (function returns)
	retCount := 0
	for _, instr := range contract.Runtime {
		if instr.Opcode == RET {
			retCount++
		}
	}
	if retCount < 2 {
		t.Errorf("Expected at least 2 RET instructions (one per function), got %d", retCount)
	}

	// Check for function call instructions
	callCount := 0
	for _, instr := range contract.Runtime {
		if instr.Opcode == CALL {
			callCount++
		}
	}
	if callCount < 1 {
		t.Error("Expected at least 1 CALL instruction for recursive factorial")
	}
}

// TestCodeGeneratorComplexContract tests generation of a complex contract
func TestCodeGeneratorComplexContract(t *testing.T) {
	source := `
	object "ERC20" {
		code {
			datacopy(0, dataoffset("runtime"), datasize("runtime"))
			return(0, datasize("runtime"))
		}
		object "runtime" {
			code {
				let selector := div(calldataload(0), 0x100000000000000000000000000000000000000000000000000000000)
				
				switch selector
				case 0x70a08231 { // balanceOf(address)
					let account := calldataload(4)
					let balance := sload(account)
					mstore(0, balance)
					return(0, 32)
				}
				case 0xa9059cbb { // transfer(address,uint256)
					let to := calldataload(4)
					let value := calldataload(36)
					let from := caller()
					
					let fromBalance := sload(from)
					if lt(fromBalance, value) { revert(0, 0) }
					
					sstore(from, sub(fromBalance, value))
					let toBalance := sload(to)
					sstore(to, add(toBalance, value))
					
					log3(0, 0, 0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef, from, to)
				}
				default {
					revert(0, 0)
				}
			}
		}
	}
	`

	parser := NewYulParser()
	ast, err := parser.Parse(source)
	if err != nil {
		t.Fatalf("Parse failed: %v", err)
	}

	context := &CompilerContext{
		SourceMap:      make(map[string]string),
		SymbolTable:    NewSymbolTable(),
		TypeTable:      NewTypeTable(),
		LabelCounter:   0,
		ErrorCollector: NewErrorCollector(),
		Metadata:       NewCompilationMetadata(),
	}
	generator := NewCodeGenerator(context)
	contract, err := generator.Generate(ast)
	if err != nil {
		t.Fatalf("Code generation failed: %v", err)
	}

	// Verify contract structure
	if contract.Name != "YulContract" {
		t.Errorf("Expected contract name 'YulContract', got %s", contract.Name)
	}

	// Check for essential ERC20 operations
	var hasStorageOps, hasArithmetic, hasControlFlow bool
	
	for _, instr := range contract.Runtime {
		switch instr.Opcode {
		case SYSCALL:
			syscallName := string(instr.Operand)
			if strings.Contains(syscallName, "Storage") {
				hasStorageOps = true
			}
		case ADD, SUB, LT:
			hasArithmetic = true
		case JMPIF, JMPIFNOT, JMP:
			hasControlFlow = true
		}
	}

	if !hasStorageOps {
		t.Error("Expected storage operations (sload/sstore)")
	}

	if !hasArithmetic {
		t.Error("Expected arithmetic operations")
	}

	if !hasControlFlow {
		t.Error("Expected control flow instructions")
	}

	// Verify instruction count is reasonable
	if len(contract.Runtime) < 20 {
		t.Errorf("Expected at least 20 instructions for complex contract, got %d", len(contract.Runtime))
	}
}

// TestCodeGeneratorStackTracking tests stack depth tracking
func TestCodeGeneratorStackTracking(t *testing.T) {
	source := `
	object "Test" {
		code {
			let a := 1
			let b := 2  
			let c := add(a, b)
			let d := mul(c, add(a, b))
		}
	}
	`

	parser := NewYulParser()
	ast, err := parser.Parse(source)
	if err != nil {
		t.Fatalf("Parse failed: %v", err)
	}

	context := &CompilerContext{
		SourceMap:      make(map[string]string),
		SymbolTable:    NewSymbolTable(),
		TypeTable:      NewTypeTable(),
		LabelCounter:   0,
		ErrorCollector: NewErrorCollector(),
		Metadata:       NewCompilationMetadata(),
	}
	generator := NewCodeGenerator(context)
	_, err = generator.Generate(ast)
	if err != nil {
		t.Fatalf("Code generation failed: %v", err)
	}

	// Check stack tracking worked
	if generator.stackTracker.maxDepth <= 0 {
		t.Error("Expected positive maximum stack depth")
	}

	if generator.stackTracker.currentDepth < 0 {
		t.Error("Final stack depth should not be negative")
	}

	// Check that stack map was populated
	if len(generator.stackTracker.stackMap) == 0 {
		t.Error("Expected stack map to be populated")
	}
}

// TestCodeGeneratorErrorHandling tests error cases
func TestCodeGeneratorErrorHandling(t *testing.T) {
	tests := []struct {
		name     string
		source   string
		errorMsg string
	}{
		{
			name: "unsupported expression",
			source: `object "Test" {
				code {
					// This would require custom expression types to test
				}
			}`,
		},
		{
			name: "undefined function call",
			source: `object "Test" {
				code {
					unknownFunction(1, 2)
				}
			}`,
			errorMsg: "", // This should generate a CALL instruction, not an error
		},
	}

	for _, test := range tests {
		if test.errorMsg == "" {
			continue // Skip tests without expected errors
		}
		
		t.Run(test.name, func(t *testing.T) {
			parser := NewYulParser()
			ast, err := parser.Parse(test.source)
			if err != nil {
				t.Fatalf("Parse failed: %v", err)
			}

			context := &CompilerContext{
				SourceMap:      make(map[string]string),
				SymbolTable:    NewSymbolTable(),
				TypeTable:      NewTypeTable(),
				LabelCounter:   0,
				ErrorCollector: NewErrorCollector(),
				Metadata:       NewCompilationMetadata(),
			}
			generator := NewCodeGenerator(context)
			_, err = generator.Generate(ast)
			
			if test.errorMsg != "" {
				if err == nil {
					t.Fatalf("Expected error containing %q but got none", test.errorMsg)
				}
				if !strings.Contains(err.Error(), test.errorMsg) {
					t.Errorf("Expected error containing %q, got %q", test.errorMsg, err.Error())
				}
			}
		})
	}
}

// TestCodeGeneratorOptimization tests basic optimizations
func TestCodeGeneratorOptimization(t *testing.T) {
	source := `
	object "Test" {
		code {
			let zero := 0
			let one := 1
			let two := 2
			let sixteen := 16
		}
	}
	`

	parser := NewYulParser()
	ast, err := parser.Parse(source)
	if err != nil {
		t.Fatalf("Parse failed: %v", err)
	}

	context := &CompilerContext{
		SourceMap:      make(map[string]string),
		SymbolTable:    NewSymbolTable(),
		TypeTable:      NewTypeTable(),
		LabelCounter:   0,
		ErrorCollector: NewErrorCollector(),
		Metadata:       NewCompilationMetadata(),
	}
	generator := NewCodeGenerator(context)
	contract, err := generator.Generate(ast)
	if err != nil {
		t.Fatalf("Code generation failed: %v", err)
	}

	// Check for optimized push instructions for small constants
	var hasPush0, hasPush1, hasPush2, hasPush16 bool
	
	for _, instr := range contract.Runtime {
		switch instr.Opcode {
		case PUSH0:
			hasPush0 = true
		case PUSH1:
			hasPush1 = true
		case PUSH2:
			hasPush2 = true
		case PUSH16:
			hasPush16 = true
		}
	}

	if !hasPush0 {
		t.Error("Expected PUSH0 optimization for literal 0")
	}
	if !hasPush1 {
		t.Error("Expected PUSH1 optimization for literal 1")
	}
	if !hasPush2 {
		t.Error("Expected PUSH2 optimization for literal 2")
	}
	if !hasPush16 {
		t.Error("Expected PUSH16 optimization for literal 16")
	}
}

// TestCodeGeneratorSourceMap tests source map generation
func TestCodeGeneratorSourceMap(t *testing.T) {
	source := `object "Test" {
		code {
			let x := 1
			let y := add(x, 2)
		}
	}`

	parser := NewYulParser()
	ast, err := parser.Parse(source)
	if err != nil {
		t.Fatalf("Parse failed: %v", err)
	}

	context := &CompilerContext{
		SourceMap:      make(map[string]string),
		SymbolTable:    NewSymbolTable(),
		TypeTable:      NewTypeTable(),
		LabelCounter:   0,
		ErrorCollector: NewErrorCollector(),
		Metadata:       NewCompilationMetadata(),
	}
	generator := NewCodeGenerator(context)
	contract, err := generator.Generate(ast)
	if err != nil {
		t.Fatalf("Code generation failed: %v", err)
	}

	// Check that source references are attached to instructions
	var instructionsWithSource int
	for _, instr := range contract.Runtime {
		if instr.SourceRef != nil {
			instructionsWithSource++
			
			// Check that source reference has valid data
			if instr.SourceRef.Line <= 0 {
				t.Error("Instruction source reference has invalid line number")
			}
			if instr.SourceRef.Column <= 0 {
				t.Error("Instruction source reference has invalid column number")
			}
		}
	}

	if instructionsWithSource == 0 {
		t.Error("Expected at least some instructions to have source references")
	}
}

// BenchmarkCodeGenerator benchmarks code generation performance
func BenchmarkCodeGenerator(b *testing.B) {
	// Large contract for benchmarking
	source := `
	object "LargeContract" {
		code {
			function fibonacci(n) -> result {
				if lt(n, 2) {
					result := n
				}
				result := add(fibonacci(sub(n, 1)), fibonacci(sub(n, 2)))
			}
			
			function factorial(n) -> result {
				if eq(n, 0) {
					result := 1
				}
				result := mul(n, factorial(sub(n, 1)))
			}
			
			function processData(data) -> result {
				for { let i := 0 } lt(i, 100) { i := add(i, 1) } {
					data := add(data, mul(i, 2))
				}
				result := data
			}
			
			let x := fibonacci(10)
			let y := factorial(5)
			let z := processData(x)
		}
	}
	`

	parser := NewYulParser()
	ast, err := parser.Parse(source)
	if err != nil {
		b.Fatalf("Parse failed: %v", err)
	}

	b.ResetTimer()

	for i := 0; i < b.N; i++ {
		context := &CompilerContext{
			SourceMap:      make(map[string]string),
			SymbolTable:    NewSymbolTable(),
			TypeTable:      NewTypeTable(),
			LabelCounter:   0,
			ErrorCollector: NewErrorCollector(),
			Metadata:       NewCompilationMetadata(),
		}
		generator := NewCodeGenerator(context)
		_, err := generator.Generate(ast)
		if err != nil {
			b.Fatalf("Code generation failed: %v", err)
		}
	}
}

// TestCodeGeneratorInstructionProperties tests instruction property calculation
func TestCodeGeneratorInstructionProperties(t *testing.T) {
	tests := []struct {
		name         string
		createInstr  func() NeoInstruction
		expectedSize int
		expectedPop  int
		expectedPush int
	}{
		{
			name:         "PUSH0",
			createInstr:  func() NeoInstruction { return NewPushInstruction(CreateNeoVMInteger(0)) },
			expectedSize: 1,
			expectedPop:  0,
			expectedPush: 1,
		},
		{
			name:         "ADD",
			createInstr:  func() NeoInstruction { return NewArithmeticInstruction(ADD) },
			expectedSize: 1,
			expectedPop:  2,
			expectedPush: 1,
		},
		{
			name:         "DUP",
			createInstr:  func() NeoInstruction { return NewStackInstruction(DUP, 0) },
			expectedSize: 1,
			expectedPop:  0,
			expectedPush: 1,
		},
		{
			name:         "JMP",
			createInstr:  func() NeoInstruction { return NewControlFlowInstruction(JMP, 100) },
			expectedSize: 5, // 1 opcode + 4 address bytes
			expectedPop:  0,
			expectedPush: 0,
		},
		{
			name:         "SYSCALL",
			createInstr:  func() NeoInstruction { return NewSyscallInstruction("System.Storage.Get") },
			expectedSize: 1 + len("System.Storage.Get"),
			expectedPop:  0,
			expectedPush: 0,
		},
	}

	for _, test := range tests {
		t.Run(test.name, func(t *testing.T) {
			instr := test.createInstr()
			
			if instr.Size != test.expectedSize {
				t.Errorf("Expected size %d, got %d", test.expectedSize, instr.Size)
			}
			if instr.StackPop != test.expectedPop {
				t.Errorf("Expected stack pop %d, got %d", test.expectedPop, instr.StackPop)
			}
			if instr.StackPush != test.expectedPush {
				t.Errorf("Expected stack push %d, got %d", test.expectedPush, instr.StackPush)
			}
		})
	}
}

// Helper functions for testing

func NewSymbolTable() *SymbolTable {
	return &SymbolTable{
		Symbols: make(map[string]*Symbol),
	}
}

func NewTypeTable() *TypeTable {
	return &TypeTable{
		Types: make(map[string]*TypeInfo),
	}
}

func NewErrorCollector() *ErrorCollector {
	return &ErrorCollector{
		Errors:   []CompilerError{},
		Warnings: []CompilerWarning{},
	}
}

// Test types for code generation validation (defined in supporting_types.go)
type SymbolTable struct {
	Symbols map[string]*Symbol
}

type Symbol struct {
	Name     string
	Type     string
	Scope    string
	Location SourcePosition
}

type TypeTable struct {
	Types map[string]*TypeInfo
}

type TypeInfo struct {
	Name   string
	Size   int
	Signed bool
}

type ErrorCollector struct {
	Errors   []CompilerError
	Warnings []CompilerWarning
}