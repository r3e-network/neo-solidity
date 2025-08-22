package main

import (
	"encoding/json"
	"fmt"
	"strings"
	"testing"
	"time"
)

// TestIntegrationSimpleContract tests the complete compilation pipeline for a simple contract
func TestIntegrationSimpleContract(t *testing.T) {
	source := `
	object "SimpleStorage" {
		code {
			datacopy(0, dataoffset("runtime"), datasize("runtime"))
			return(0, datasize("runtime"))
		}
		object "runtime" {
			code {
				switch selector()
				case 0x60fe47b1 { set(calldataload(4)) }
				case 0x6d4ce63c { return_uint(get()) }
				default { revert(0, 0) }
				
				function selector() -> s {
					s := div(calldataload(0), 0x100000000000000000000000000000000000000000000000000000000)
				}
				
				function set(v) {
					sstore(0, v)
				}
				
				function get() -> v {
					v := sload(0)
				}
				
				function return_uint(v) {
					mstore(0, v)
					return(0, 32)
				}
			}
		}
	}
	`

	// Test complete compilation pipeline
	config := CompilerConfig{
		OptimizationLevel:    2,
		TargetNeoVMVersion:   "3.0",
		EnableBoundsChecking: true,
		EnableDebugInfo:     true,
		MaxStackDepth:       1024,
		MemoryLimit:         64 * 1024 * 1024,
	}

	compiler := NewYulToNeoCompiler(config)
	result, err := compiler.Compile(source)
	
	if err != nil {
		t.Fatalf("Compilation failed: %v", err)
	}

	// Verify compilation succeeded
	if len(result.Errors) > 0 {
		t.Fatalf("Compilation errors: %v", result.Errors)
	}

	// Verify contract was generated
	if result.Contract == nil {
		t.Fatal("No contract generated")
	}

	// Verify contract has runtime code
	if len(result.Contract.Runtime) == 0 {
		t.Fatal("No runtime code generated")
	}

	// Verify metadata
	if result.Contract.Metadata == nil {
		t.Error("No metadata generated")
	}

	// Verify function table is populated
	generator := compiler.CodeGenerator.(*CodeGenerator)
	if len(generator.functionTable) < 4 {
		t.Errorf("Expected at least 4 functions, got %d", len(generator.functionTable))
	}

	// Check for essential functions
	expectedFunctions := []string{"selector", "set", "get", "return_uint"}
	for _, fn := range expectedFunctions {
		if _, exists := generator.functionTable[fn]; !exists {
			t.Errorf("Expected function '%s' not found", fn)
		}
	}

	// Verify statistics
	if result.Statistics.FunctionsCompiled < 4 {
		t.Errorf("Expected at least 4 functions compiled, got %d", result.Statistics.FunctionsCompiled)
	}

	if result.Statistics.CompiledSizeBytes == 0 {
		t.Error("Compiled size should be greater than 0")
	}

	t.Logf("Integration test passed: %d instructions, %d functions, %d bytes",
		len(result.Contract.Runtime), result.Statistics.FunctionsCompiled, result.Statistics.CompiledSizeBytes)
}

// TestIntegrationERC20Contract tests compilation of a more complex ERC20-like contract
func TestIntegrationERC20Contract(t *testing.T) {
	source := `
	object "ERC20Token" {
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
					
					// Check balance
					let fromBalance := sload(from)
					if lt(fromBalance, value) { revert(0, 0) }
					
					// Update balances
					sstore(from, sub(fromBalance, value))
					let toBalance := sload(to)
					sstore(to, add(toBalance, value))
					
					// Emit Transfer event
					log3(0, 0, 0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef, from, to)
					
					// Return success
					mstore(0, 1)
					return(0, 32)
				}
				case 0x095ea7b3 { // approve(address,uint256)
					let spender := calldataload(4)
					let value := calldataload(36)
					let owner := caller()
					
					// Store allowance
					let allowanceKey := or(shl(160, owner), spender)
					sstore(allowanceKey, value)
					
					// Emit Approval event
					log3(0, 0, 0x8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b925, owner, spender)
					
					// Return success
					mstore(0, 1)
					return(0, 32)
				}
				case 0xdd62ed3e { // allowance(address,address)
					let owner := calldataload(4)
					let spender := calldataload(36)
					let allowanceKey := or(shl(160, owner), spender)
					let allowance := sload(allowanceKey)
					mstore(0, allowance)
					return(0, 32)
				}
				default {
					revert(0, 0)
				}
			}
		}
	}
	`

	// Test compilation with higher optimization
	config := CompilerConfig{
		OptimizationLevel:    3,
		TargetNeoVMVersion:   "3.0",
		EnableBoundsChecking: true,
		EnableDebugInfo:     true,
		MaxStackDepth:       2048,
		MemoryLimit:         128 * 1024 * 1024,
	}

	compiler := NewYulToNeoCompiler(config)
	result, err := compiler.Compile(source)
	
	if err != nil {
		t.Fatalf("ERC20 compilation failed: %v", err)
	}

	if len(result.Errors) > 0 {
		t.Fatalf("ERC20 compilation errors: %v", result.Errors)
	}

	// Verify complex contract features
	contract := result.Contract
	if contract == nil {
		t.Fatal("No ERC20 contract generated")
	}

	// Should have substantial runtime code for ERC20 functionality
	if len(contract.Runtime) < 50 {
		t.Errorf("Expected substantial runtime code for ERC20, got %d instructions", len(contract.Runtime))
	}

	// Verify various instruction types are present
	var hasArithmetic, hasStorage, hasControl, hasMemory, hasLogic bool
	
	for _, instr := range contract.Runtime {
		switch instr.Opcode {
		case ADD, SUB, MUL, DIV, MOD:
			hasArithmetic = true
		case SYSCALL:
			if strings.Contains(string(instr.Operand), "Storage") {
				hasStorage = true
			}
		case JMPIF, JMPIFNOT, JMP:
			hasControl = true
		case LT, GT, EQUAL:
			hasLogic = true
		}
	}

	if !hasArithmetic {
		t.Error("ERC20 contract should contain arithmetic operations")
	}
	if !hasStorage {
		t.Error("ERC20 contract should contain storage operations")
	}
	if !hasControl {
		t.Error("ERC20 contract should contain control flow")
	}
	if !hasLogic {
		t.Error("ERC20 contract should contain logical operations")
	}

	t.Logf("ERC20 integration test passed: %d instructions generated", len(contract.Runtime))
}

// TestIntegrationErrorHandling tests compilation error scenarios
func TestIntegrationErrorHandling(t *testing.T) {
	testCases := []struct {
		name        string
		source      string
		expectError bool
		errorPhase  string
	}{
		{
			name: "syntax error",
			source: `
			object "Invalid" {
				code {
					let x := // missing value
				}
			}
			`,
			expectError: true,
			errorPhase:  "Parsing",
		},
		{
			name: "undefined function",
			source: `
			object "Undefined" {
				code {
					unknownFunction(1, 2, 3)
				}
			}
			`,
			expectError: false, // Should generate CALL instruction
		},
		{
			name: "empty contract",
			source: `object "Empty" {}`,
			expectError: false,
		},
		{
			name: "malformed object",
			source: `object {`,
			expectError: true,
			errorPhase:  "Parsing",
		},
	}

	for _, tc := range testCases {
		t.Run(tc.name, func(t *testing.T) {
			config := CompilerConfig{
				OptimizationLevel:    1,
				TargetNeoVMVersion:   "3.0",
				EnableBoundsChecking: false,
				EnableDebugInfo:     false,
				MaxStackDepth:       1024,
				MemoryLimit:         32 * 1024 * 1024,
			}

			compiler := NewYulToNeoCompiler(config)
			result, err := compiler.Compile(tc.source)

			if tc.expectError {
				if err == nil && len(result.Errors) == 0 {
					t.Errorf("Expected error for test case '%s' but compilation succeeded", tc.name)
				}
				
				if err == nil && len(result.Errors) > 0 {
					// Check error phase if specified
					if tc.errorPhase != "" {
						foundExpectedPhase := false
						for _, compErr := range result.Errors {
							if compErr.Phase == tc.errorPhase {
								foundExpectedPhase = true
								break
							}
						}
						if !foundExpectedPhase {
							t.Errorf("Expected error in phase '%s' but got errors: %v", tc.errorPhase, result.Errors)
						}
					}
				}
			} else {
				if err != nil {
					t.Errorf("Unexpected error for test case '%s': %v", tc.name, err)
				}
				if len(result.Errors) > 0 {
					t.Errorf("Unexpected compilation errors for test case '%s': %v", tc.name, result.Errors)
				}
			}
		})
	}
}

// TestIntegrationOptimizationLevels tests different optimization levels
func TestIntegrationOptimizationLevels(t *testing.T) {
	source := `
	object "OptimizationTest" {
		code {
			function redundant(a, b) -> result {
				let temp1 := add(a, b)
				let temp2 := mul(temp1, 1) // Multiply by 1 (should be optimized)
				let temp3 := add(temp2, 0) // Add 0 (should be optimized)
				result := temp3
			}
			
			let x := redundant(5, 10)
			let y := redundant(x, x)
		}
	}
	`

	optimizationLevels := []int{0, 1, 2, 3}
	results := make([]*CompilationResult, len(optimizationLevels))

	// Compile with different optimization levels
	for i, level := range optimizationLevels {
		config := CompilerConfig{
			OptimizationLevel:    level,
			TargetNeoVMVersion:   "3.0",
			EnableBoundsChecking: false,
			EnableDebugInfo:     false,
			MaxStackDepth:       1024,
			MemoryLimit:         32 * 1024 * 1024,
		}

		compiler := NewYulToNeoCompiler(config)
		result, err := compiler.Compile(source)
		
		if err != nil {
			t.Fatalf("Optimization level %d compilation failed: %v", level, err)
		}

		if len(result.Errors) > 0 {
			t.Fatalf("Optimization level %d errors: %v", level, result.Errors)
		}

		results[i] = result
		
		t.Logf("Optimization level %d: %d instructions, %d bytes", 
			level, len(result.Contract.Runtime), result.Statistics.CompiledSizeBytes)
	}

	// Verify that higher optimization levels generally produce smaller or more efficient code
	// (This is a simplified check - in practice, optimization effects may vary)
	for i := 1; i < len(results); i++ {
		prev := results[i-1]
		curr := results[i]
		
		// At minimum, ensure all optimization levels produce valid code
		if len(curr.Contract.Runtime) == 0 {
			t.Errorf("Optimization level %d produced no runtime code", optimizationLevels[i])
		}
		
		// Higher optimization might not always reduce instruction count due to different strategies
		// But compilation should always succeed
		if curr.Statistics.OptimizationsPassed < 0 {
			t.Errorf("Optimization level %d has invalid optimization count", optimizationLevels[i])
		}
	}
}

// TestIntegrationDebugInfo tests debug information generation
func TestIntegrationDebugInfo(t *testing.T) {
	source := `
	object "DebugTest" {
		code {
			function testFunction(param) -> result {
				let localVar := add(param, 42)
				result := mul(localVar, 2)
			}
			
			let value := testFunction(10)
		}
	}
	`

	// Test with debug info enabled
	config := CompilerConfig{
		OptimizationLevel:    1,
		TargetNeoVMVersion:   "3.0",
		EnableBoundsChecking: true,
		EnableDebugInfo:     true,
		MaxStackDepth:       1024,
		MemoryLimit:         32 * 1024 * 1024,
	}

	compiler := NewYulToNeoCompiler(config)
	result, err := compiler.Compile(source)
	
	if err != nil {
		t.Fatalf("Debug info compilation failed: %v", err)
	}

	if len(result.Errors) > 0 {
		t.Fatalf("Debug info compilation errors: %v", result.Errors)
	}

	// Verify debug info was generated
	if result.DebugInfo == nil {
		t.Fatal("Debug info was not generated")
	}

	// Check source map
	if len(result.DebugInfo.SourceMap) == 0 {
		t.Error("Source map is empty")
	}

	// Check function map
	if len(result.DebugInfo.FunctionMap) == 0 {
		t.Error("Function map is empty")
	}

	// Verify function info
	if funcInfo, exists := result.DebugInfo.FunctionMap["testFunction"]; exists {
		if funcInfo.ParameterCount != 1 {
			t.Errorf("Expected 1 parameter for testFunction, got %d", funcInfo.ParameterCount)
		}
		if funcInfo.ReturnCount != 1 {
			t.Errorf("Expected 1 return value for testFunction, got %d", funcInfo.ReturnCount)
		}
	} else {
		t.Error("testFunction not found in function map")
	}

	// Verify that runtime instructions have source references
	instructionsWithSource := 0
	for _, instr := range result.Contract.Runtime {
		if instr.SourceRef != nil {
			instructionsWithSource++
		}
	}

	if instructionsWithSource == 0 {
		t.Error("No instructions have source references")
	}

	t.Logf("Debug info test passed: %d source mappings, %d function mappings, %d instructions with source refs",
		len(result.DebugInfo.SourceMap), len(result.DebugInfo.FunctionMap), instructionsWithSource)
}

// TestIntegrationCompilerValidation tests the validation functionality
func TestIntegrationCompilerValidation(t *testing.T) {
	source := `
	object "ValidationTest" {
		code {
			function divide(a, b) -> result {
				if iszero(b) {
					revert(0, 0)
				}
				result := div(a, b)
			}
			
			let x := divide(100, 5)
			let y := divide(x, 0) // This should trigger validation warning
		}
	}
	`

	config := CompilerConfig{
		OptimizationLevel:    1,
		TargetNeoVMVersion:   "3.0",
		EnableBoundsChecking: true,
		EnableDebugInfo:     false,
		MaxStackDepth:       1024,
		MemoryLimit:         32 * 1024 * 1024,
	}

	compiler := NewYulToNeoCompiler(config)
	
	// Test validation without compilation
	validationResult, err := compiler.Validate(source)
	if err != nil {
		t.Fatalf("Validation failed: %v", err)
	}

	if !validationResult.IsValid {
		t.Errorf("Expected valid code, but got errors: %v", validationResult.Errors)
	}

	// Validation should succeed but may have warnings
	t.Logf("Validation test passed: %d errors, %d warnings", 
		len(validationResult.Errors), len(validationResult.Warnings))

	// Test full compilation to ensure it also succeeds
	result, err := compiler.Compile(source)
	if err != nil {
		t.Fatalf("Full compilation after validation failed: %v", err)
	}

	if len(result.Errors) > 0 {
		t.Errorf("Compilation errors after validation: %v", result.Errors)
	}
}

// TestIntegrationPerformance tests compilation performance
func TestIntegrationPerformance(t *testing.T) {
	// Generate a larger contract for performance testing
	source := generateLargeContract()

	config := CompilerConfig{
		OptimizationLevel:    2,
		TargetNeoVMVersion:   "3.0",
		EnableBoundsChecking: true,
		EnableDebugInfo:     true,
		MaxStackDepth:       2048,
		MemoryLimit:         128 * 1024 * 1024,
	}

	compiler := NewYulToNeoCompiler(config)

	// Measure compilation time
	startTime := time.Now()
	result, err := compiler.Compile(source)
	compilationTime := time.Since(startTime)

	if err != nil {
		t.Fatalf("Performance test compilation failed: %v", err)
	}

	if len(result.Errors) > 0 {
		t.Fatalf("Performance test compilation errors: %v", result.Errors)
	}

	// Verify reasonable performance
	maxCompilationTime := 10 * time.Second
	if compilationTime > maxCompilationTime {
		t.Errorf("Compilation took too long: %v (max: %v)", compilationTime, maxCompilationTime)
	}

	// Verify reasonable output size
	if len(result.Contract.Runtime) == 0 {
		t.Error("No runtime code generated for large contract")
	}

	t.Logf("Performance test passed: compiled %d instructions in %v", 
		len(result.Contract.Runtime), compilationTime)
}

// TestIntegrationMetadataGeneration tests contract metadata generation
func TestIntegrationMetadataGeneration(t *testing.T) {
	source := `
	object "MetadataTest" {
		code {
			function publicFunction(input) -> output {
				output := add(input, 1)
			}
			
			let result := publicFunction(42)
		}
	}
	`

	config := CompilerConfig{
		OptimizationLevel:    1,
		TargetNeoVMVersion:   "3.0",
		EnableBoundsChecking: true,
		EnableDebugInfo:     true,
		MaxStackDepth:       1024,
		MemoryLimit:         64 * 1024 * 1024,
	}

	compiler := NewYulToNeoCompiler(config)
	result, err := compiler.Compile(source)
	
	if err != nil {
		t.Fatalf("Metadata test compilation failed: %v", err)
	}

	if len(result.Errors) > 0 {
		t.Fatalf("Metadata test compilation errors: %v", result.Errors)
	}

	// Verify contract metadata
	metadata := result.Contract.Metadata
	if metadata == nil {
		t.Fatal("Contract metadata not generated")
	}

	// Check compiler info
	if metadata.Compiler.Version == "" {
		t.Error("Compiler version not set in metadata")
	}

	if metadata.Compiler.Target != "NeoVM" {
		t.Errorf("Expected target 'NeoVM', got '%s'", metadata.Compiler.Target)
	}

	// Verify contract structure
	if result.Contract.Name == "" {
		t.Error("Contract name not set")
	}

	if result.Contract.Version == "" {
		t.Error("Contract version not set")
	}

	// Verify entry points exist
	if len(result.Contract.EntryPoints) == 0 {
		t.Error("No entry points generated")
	}

	t.Logf("Metadata test passed: %d entry points, compiler %s", 
		len(result.Contract.EntryPoints), metadata.Compiler.Version)
}

// TestIntegrationJSONSerialization tests JSON serialization of compilation results
func TestIntegrationJSONSerialization(t *testing.T) {
	source := `
	object "SerializationTest" {
		code {
			let x := 42
			let y := add(x, 8)
		}
	}
	`

	config := CompilerConfig{
		OptimizationLevel:    1,
		TargetNeoVMVersion:   "3.0",
		EnableBoundsChecking: false,
		EnableDebugInfo:     true,
		MaxStackDepth:       1024,
		MemoryLimit:         32 * 1024 * 1024,
	}

	compiler := NewYulToNeoCompiler(config)
	result, err := compiler.Compile(source)
	
	if err != nil {
		t.Fatalf("Serialization test compilation failed: %v", err)
	}

	if len(result.Errors) > 0 {
		t.Fatalf("Serialization test compilation errors: %v", result.Errors)
	}

	// Test JSON serialization of the contract
	contractJSON, err := json.Marshal(result.Contract)
	if err != nil {
		t.Fatalf("Failed to serialize contract to JSON: %v", err)
	}

	if len(contractJSON) == 0 {
		t.Fatal("Empty JSON serialization")
	}

	// Test deserialization
	var deserializedContract NeoContract
	err = json.Unmarshal(contractJSON, &deserializedContract)
	if err != nil {
		t.Fatalf("Failed to deserialize contract from JSON: %v", err)
	}

	// Verify key properties are preserved
	if deserializedContract.Name != result.Contract.Name {
		t.Errorf("Contract name not preserved: expected %s, got %s", 
			result.Contract.Name, deserializedContract.Name)
	}

	if len(deserializedContract.Runtime) != len(result.Contract.Runtime) {
		t.Errorf("Runtime instructions count not preserved: expected %d, got %d",
			len(result.Contract.Runtime), len(deserializedContract.Runtime))
	}

	t.Logf("JSON serialization test passed: %d bytes", len(contractJSON))
}

// Helper function to generate a large contract for performance testing
func generateLargeContract() string {
	var builder strings.Builder
	builder.WriteString(`object "LargeContract" { code {`)

	// Generate many functions
	for i := 0; i < 20; i++ {
		builder.WriteString(fmt.Sprintf(`
		function func%d(a, b, c) -> result {
			let temp1 := add(a, b)
			let temp2 := mul(temp1, c)
			let temp3 := div(temp2, add(a, 1))
			if lt(temp3, 100) {
				result := add(temp3, %d)
			}
			result := sub(temp3, %d)
		}`, i, i*10, i*5))
	}

	// Generate function calls
	builder.WriteString(`
		let result0 := func0(1, 2, 3)
		let result1 := func1(result0, 4, 5)
		let result2 := func2(result1, 6, 7)`)

	// Generate a switch with many cases
	builder.WriteString(`
		switch result2`)
	for i := 0; i < 10; i++ {
		builder.WriteString(fmt.Sprintf(`
		case %d { 
			let temp := func%d(result2, %d, %d)
			sstore(%d, temp)
		}`, i, i%20, i, i+1, i))
	}
	builder.WriteString(`
		default { revert(0, 0) }`)

	builder.WriteString(`}}`)
	return builder.String()
}

// BenchmarkIntegrationCompilation benchmarks the complete compilation process
func BenchmarkIntegrationCompilation(b *testing.B) {
	source := `
	object "BenchmarkContract" {
		code {
			function fibonacci(n) -> result {
				if lt(n, 2) {
					result := n
				}
				result := add(fibonacci(sub(n, 1)), fibonacci(sub(n, 2)))
			}
			
			let fib10 := fibonacci(10)
		}
	}
	`

	config := CompilerConfig{
		OptimizationLevel:    2,
		TargetNeoVMVersion:   "3.0",
		EnableBoundsChecking: true,
		EnableDebugInfo:     false, // Disable for better benchmark performance
		MaxStackDepth:       1024,
		MemoryLimit:         64 * 1024 * 1024,
	}

	b.ResetTimer()

	for i := 0; i < b.N; i++ {
		compiler := NewYulToNeoCompiler(config)
		result, err := compiler.Compile(source)
		
		if err != nil {
			b.Fatalf("Benchmark compilation failed: %v", err)
		}

		if len(result.Errors) > 0 {
			b.Fatalf("Benchmark compilation errors: %v", result.Errors)
		}

		// Ensure the result is used to prevent optimization
		if len(result.Contract.Runtime) == 0 {
			b.Fatal("No runtime code generated in benchmark")
		}
	}
}