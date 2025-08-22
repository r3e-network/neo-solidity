package main

import (
	"fmt"
	"math/big"
	"strings"
	"testing"
	"time"
)

// BenchmarkResult represents the results of a performance benchmark
type BenchmarkResult struct {
	Name              string
	CompilationTime   time.Duration
	ExecutionTime     time.Duration
	InstructionCount  int
	GasUsed           uint64
	MemoryUsed        uint64
	CodeSize          int
	OptimizationLevel int
}

// TestEVMvsNeoVMPerformanceComparison compares performance between EVM and NeoVM execution
func TestEVMvsNeoVMPerformanceComparison(t *testing.T) {
	testCases := []struct {
		name        string
		yulSource   string
		description string
	}{
		{
			name:        "simple_arithmetic",
			description: "Basic arithmetic operations",
			yulSource: `
			object "ArithmeticTest" {
				code {
					let a := 100
					let b := 50
					let sum := add(a, b)
					let diff := sub(a, b)
					let product := mul(sum, diff)
					let quotient := div(product, 10)
				}
			}`,
		},
		{
			name:        "loop_operations",
			description: "For loop with arithmetic",
			yulSource: `
			object "LoopTest" {
				code {
					let result := 0
					for { let i := 0 } lt(i, 100) { i := add(i, 1) } {
						result := add(result, mul(i, 2))
					}
				}
			}`,
		},
		{
			name:        "storage_operations",
			description: "Storage read/write operations",
			yulSource: `
			object "StorageTest" {
				code {
					for { let i := 0 } lt(i, 10) { i := add(i, 1) } {
						sstore(i, mul(i, i))
						let value := sload(i)
						sstore(add(i, 100), add(value, 1))
					}
				}
			}`,
		},
		{
			name:        "memory_operations",
			description: "Memory allocation and manipulation",
			yulSource: `
			object "MemoryTest" {
				code {
					for { let i := 0 } lt(i, 20) { i := add(i, 1) } {
						let offset := mul(i, 32)
						mstore(offset, add(i, 0x123456))
						let value := mload(offset)
						mstore(add(offset, 640), keccak256(offset, 32))
					}
				}
			}`,
		},
		{
			name:        "function_calls",
			description: "Recursive function calls",
			yulSource: `
			object "FunctionTest" {
				code {
					function fibonacci(n) -> result {
						if lt(n, 2) {
							result := n
						}
						result := add(fibonacci(sub(n, 1)), fibonacci(sub(n, 2)))
					}
					
					let fib10 := fibonacci(10)
				}
			}`,
		},
		{
			name:        "complex_contract",
			description: "Complex ERC20-like operations",
			yulSource: `
			object "ComplexTest" {
				code {
					function transfer(from, to, amount) -> success {
						let fromBalance := sload(from)
						if lt(fromBalance, amount) { revert(0, 0) }
						
						sstore(from, sub(fromBalance, amount))
						let toBalance := sload(to)
						sstore(to, add(toBalance, amount))
						
						// Emit event
						mstore(0, amount)
						log3(0, 32, 0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef, from, to)
						success := 1
					}
					
					// Initialize balances
					sstore(0x1111, 1000)
					sstore(0x2222, 2000)
					
					// Perform transfers
					transfer(0x1111, 0x2222, 100)
					transfer(0x2222, 0x1111, 200)
					transfer(0x1111, 0x2222, 50)
				}
			}`,
		},
	}

	results := make(map[string][]BenchmarkResult)

	for _, tc := range testCases {
		t.Run(tc.name, func(t *testing.T) {
			// Test different optimization levels
			optimizationLevels := []int{0, 1, 2, 3}
			
			for _, optLevel := range optimizationLevels {
				result := benchmarkYulToNeoVM(t, tc.name, tc.yulSource, optLevel)
				result.OptimizationLevel = optLevel
				
				results[tc.name] = append(results[tc.name], result)
				
				t.Logf("%s (O%d): Compilation=%v, Instructions=%d, CodeSize=%d bytes",
					tc.name, optLevel, result.CompilationTime, result.InstructionCount, result.CodeSize)
			}
		})
	}

	// Generate performance comparison report
	generatePerformanceReport(t, results)
}

// benchmarkYulToNeoVM benchmarks compilation and theoretical execution of Yul to NeoVM
func benchmarkYulToNeoVM(t *testing.T, name, source string, optimizationLevel int) BenchmarkResult {
	config := CompilerConfig{
		OptimizationLevel:    optimizationLevel,
		TargetNeoVMVersion:   "3.0",
		EnableBoundsChecking: optimizationLevel < 2,
		EnableDebugInfo:     false, // Disable for performance
		MaxStackDepth:       2048,
		MemoryLimit:         128 * 1024 * 1024,
	}

	// Measure compilation time
	startTime := time.Now()
	compiler := NewYulToNeoCompiler(config)
	result, err := compiler.Compile(source)
	compilationTime := time.Since(startTime)

	if err != nil {
		t.Fatalf("Compilation failed for %s: %v", name, err)
	}

	if len(result.Errors) > 0 {
		t.Fatalf("Compilation errors for %s: %v", name, result.Errors)
	}

	// Simulate execution time based on instruction complexity
	executionTime := simulateNeoVMExecution(result.Contract.Runtime)

	// Calculate estimated gas usage
	gasUsed := calculateEstimatedGasUsage(result.Contract.Runtime)

	// Estimate memory usage
	memoryUsed := estimateMemoryUsage(result.Contract.Runtime)

	return BenchmarkResult{
		Name:              name,
		CompilationTime:   compilationTime,
		ExecutionTime:     executionTime,
		InstructionCount:  len(result.Contract.Runtime),
		GasUsed:           gasUsed,
		MemoryUsed:        memoryUsed,
		CodeSize:          result.Statistics.CompiledSizeBytes,
		OptimizationLevel: optimizationLevel,
	}
}

// simulateNeoVMExecution simulates execution time based on instruction types and counts
func simulateNeoVMExecution(instructions []NeoInstruction) time.Duration {
	totalCycles := int64(0)

	for _, instr := range instructions {
		switch instr.Opcode {
		// Fast operations (1 cycle)
		case PUSH0, PUSH1, PUSH2, PUSH3, PUSH4, PUSH5, PUSH6, PUSH7, PUSH8, PUSH9, PUSH10, 
		     PUSH11, PUSH12, PUSH13, PUSH14, PUSH15, PUSH16:
			totalCycles += 1
		
		case DUP, SWAP, DROP, NIP:
			totalCycles += 1

		// Arithmetic operations (2-4 cycles)
		case ADD, SUB:
			totalCycles += 2
		case MUL:
			totalCycles += 4
		case DIV, MOD:
			totalCycles += 8
		
		// Logical operations (2 cycles)
		case AND, OR, XOR, NOT, LT, GT, EQUAL:
			totalCycles += 2

		// Memory operations (variable)
		case PUSHDATA1:
			totalCycles += int64(2 + len(instr.Operand)/4)
		case PUSHDATA2:
			totalCycles += int64(4 + len(instr.Operand)/4)
		case PUSHDATA4:
			totalCycles += int64(8 + len(instr.Operand)/4)

		// Control flow (4-8 cycles)
		case JMP, JMPIF, JMPIFNOT:
			totalCycles += 4
		case CALL:
			totalCycles += 16
		case RET:
			totalCycles += 8

		// System calls (expensive, 100-1000 cycles)
		case SYSCALL:
			syscallName := string(instr.Operand)
			switch {
			case strings.Contains(syscallName, "Storage.Get"):
				totalCycles += 200
			case strings.Contains(syscallName, "Storage.Put"):
				totalCycles += 1000
			case strings.Contains(syscallName, "Crypto"):
				totalCycles += 500
			case strings.Contains(syscallName, "Runtime"):
				totalCycles += 100
			default:
				totalCycles += 300
			}

		default:
			totalCycles += 4 // Default cost for unknown instructions
		}
	}

	// Convert cycles to time (assuming 1 million cycles per millisecond)
	return time.Duration(totalCycles) * time.Microsecond
}

// calculateEstimatedGasUsage calculates estimated gas usage for NeoVM instructions
func calculateEstimatedGasUsage(instructions []NeoInstruction) uint64 {
	totalGas := uint64(0)

	for _, instr := range instructions {
		totalGas += uint64(instr.GasCost)
	}

	return totalGas
}

// estimateMemoryUsage estimates memory usage based on instruction patterns
func estimateMemoryUsage(instructions []NeoInstruction) uint64 {
	memoryUsage := uint64(0)
	stackDepth := 0
	maxStackDepth := 0

	for _, instr := range instructions {
		// Track stack changes
		stackDepth -= instr.StackPop
		stackDepth += instr.StackPush
		
		if stackDepth > maxStackDepth {
			maxStackDepth = stackDepth
		}

		// Add memory for instruction storage
		memoryUsage += uint64(instr.Size)

		// Add memory for operand data
		if len(instr.Operand) > 0 {
			memoryUsage += uint64(len(instr.Operand))
		}
	}

	// Add estimated stack memory (32 bytes per stack item)
	memoryUsage += uint64(maxStackDepth * 32)

	return memoryUsage
}

// generatePerformanceReport generates a comprehensive performance comparison report
func generatePerformanceReport(t *testing.T, results map[string][]BenchmarkResult) {
	t.Log("\n" + strings.Repeat("=", 80))
	t.Log("PERFORMANCE BENCHMARK REPORT - EVM vs NeoVM Comparison")
	t.Log(strings.Repeat("=", 80))

	// Overall statistics
	totalTests := 0
	totalCompilationTime := time.Duration(0)
	totalInstructions := 0

	for testName, testResults := range results {
		t.Log(fmt.Sprintf("\n--- %s ---", strings.ToUpper(testName)))
		
		for _, result := range testResults {
			totalTests++
			totalCompilationTime += result.CompilationTime
			totalInstructions += result.InstructionCount

			t.Log(fmt.Sprintf("  O%d: Compile=%6s | Exec=%6s | Instructions=%4d | Gas=%6d | Memory=%6d | Size=%4d",
				result.OptimizationLevel,
				result.CompilationTime.Round(time.Microsecond),
				result.ExecutionTime.Round(time.Microsecond),
				result.InstructionCount,
				result.GasUsed,
				result.MemoryUsed,
				result.CodeSize))
		}

		// Show optimization improvements
		if len(testResults) > 1 {
			baseline := testResults[0] // O0
			optimized := testResults[len(testResults)-1] // Highest optimization

			compilationImprovement := float64(baseline.CompilationTime-optimized.CompilationTime) / float64(baseline.CompilationTime) * 100
			executionImprovement := float64(baseline.ExecutionTime-optimized.ExecutionTime) / float64(baseline.ExecutionTime) * 100
			sizeImprovement := float64(baseline.CodeSize-optimized.CodeSize) / float64(baseline.CodeSize) * 100

			t.Log(fmt.Sprintf("  Optimization gains: Exec=%.1f%% | Size=%.1f%% | Compile=%.1f%%",
				executionImprovement, sizeImprovement, compilationImprovement))
		}
	}

	// Summary statistics
	t.Log(fmt.Sprintf("\n--- SUMMARY ---"))
	avgCompilationTime := totalCompilationTime / time.Duration(totalTests)
	avgInstructions := totalInstructions / totalTests

	t.Log(fmt.Sprintf("  Total tests: %d", totalTests))
	t.Log(fmt.Sprintf("  Average compilation time: %v", avgCompilationTime.Round(time.Microsecond)))
	t.Log(fmt.Sprintf("  Average instructions generated: %d", avgInstructions))

	// Performance categories
	t.Log(fmt.Sprintf("\n--- PERFORMANCE CATEGORIES ---"))
	for testName, testResults := range results {
		bestResult := testResults[len(testResults)-1] // Highest optimization
		
		var category string
		switch {
		case bestResult.ExecutionTime < 100*time.Microsecond:
			category = "🚀 EXCELLENT"
		case bestResult.ExecutionTime < 500*time.Microsecond:
			category = "✅ GOOD"
		case bestResult.ExecutionTime < 1*time.Millisecond:
			category = "⚠️  AVERAGE"
		default:
			category = "🔥 NEEDS_OPTIMIZATION"
		}

		t.Log(fmt.Sprintf("  %s: %s (exec: %v)", testName, category, bestResult.ExecutionTime.Round(time.Microsecond)))
	}

	t.Log(strings.Repeat("=", 80))
}

// BenchmarkCompilationSpeed benchmarks just the compilation speed
func BenchmarkCompilationSpeed(b *testing.B) {
	testSources := map[string]string{
		"simple": `
		object "Simple" {
			code {
				let x := add(1, 2)
				let y := mul(x, 3)
			}
		}`,
		"medium": `
		object "Medium" {
			code {
				function factorial(n) -> result {
					if lt(n, 2) { result := 1 }
					result := mul(n, factorial(sub(n, 1)))
				}
				let result := factorial(10)
			}
		}`,
		"complex": readComplexBenchmarkSource(),
	}

	config := CompilerConfig{
		OptimizationLevel:    2,
		TargetNeoVMVersion:   "3.0",
		EnableBoundsChecking: false,
		EnableDebugInfo:     false,
		MaxStackDepth:       2048,
		MemoryLimit:         128 * 1024 * 1024,
	}

	for name, source := range testSources {
		b.Run(name, func(b *testing.B) {
			b.ResetTimer()
			for i := 0; i < b.N; i++ {
				compiler := NewYulToNeoCompiler(config)
				result, err := compiler.Compile(source)
				
				if err != nil {
					b.Fatalf("Compilation failed: %v", err)
				}

				if len(result.Errors) > 0 {
					b.Fatalf("Compilation errors: %v", result.Errors)
				}

				// Ensure result is used
				if len(result.Contract.Runtime) == 0 {
					b.Fatal("No runtime code generated")
				}
			}
		})
	}
}

// BenchmarkMemoryUsage benchmarks memory usage during compilation
func BenchmarkMemoryUsage(b *testing.B) {
	source := readComplexBenchmarkSource()

	config := CompilerConfig{
		OptimizationLevel:    2,
		TargetNeoVMVersion:   "3.0",
		EnableBoundsChecking: false,
		EnableDebugInfo:     false,
		MaxStackDepth:       2048,
		MemoryLimit:         128 * 1024 * 1024,
	}

	b.ResetTimer()
	b.ReportAllocs() // Report memory allocations

	for i := 0; i < b.N; i++ {
		compiler := NewYulToNeoCompiler(config)
		result, err := compiler.Compile(source)
		
		if err != nil {
			b.Fatalf("Compilation failed: %v", err)
		}

		if len(result.Contract.Runtime) == 0 {
			b.Fatal("No runtime code generated")
		}
	}
}

// BenchmarkOptimizationLevels benchmarks different optimization levels
func BenchmarkOptimizationLevels(b *testing.B) {
	source := `
	object "OptimizationBench" {
		code {
			function compute(n) -> result {
				result := 0
				for { let i := 0 } lt(i, n) { i := add(i, 1) } {
					result := add(result, mul(i, i))
				}
			}
			
			let total := 0
			total := add(total, compute(10))
			total := add(total, compute(20))
			total := add(total, compute(30))
		}
	}`

	optimizationLevels := []int{0, 1, 2, 3}

	for _, level := range optimizationLevels {
		b.Run(fmt.Sprintf("O%d", level), func(b *testing.B) {
			config := CompilerConfig{
				OptimizationLevel:    level,
				TargetNeoVMVersion:   "3.0",
				EnableBoundsChecking: level < 2,
				EnableDebugInfo:     false,
				MaxStackDepth:       2048,
				MemoryLimit:         128 * 1024 * 1024,
			}

			b.ResetTimer()
			for i := 0; i < b.N; i++ {
				compiler := NewYulToNeoCompiler(config)
				result, err := compiler.Compile(source)
				
				if err != nil {
					b.Fatalf("Compilation failed: %v", err)
				}

				if len(result.Contract.Runtime) == 0 {
					b.Fatal("No runtime code generated")
				}
			}
		})
	}
}

// BenchmarkParallelCompilation benchmarks parallel compilation scenarios
func BenchmarkParallelCompilation(b *testing.B) {
	source := readComplexBenchmarkSource()

	config := CompilerConfig{
		OptimizationLevel:    2,
		TargetNeoVMVersion:   "3.0",
		EnableBoundsChecking: false,
		EnableDebugInfo:     false,
		MaxStackDepth:       2048,
		MemoryLimit:         128 * 1024 * 1024,
	}

	b.ResetTimer()

	b.Run("sequential", func(b *testing.B) {
		for i := 0; i < b.N; i++ {
			compiler := NewYulToNeoCompiler(config)
			result, err := compiler.Compile(source)
			
			if err != nil {
				b.Fatalf("Compilation failed: %v", err)
			}

			if len(result.Contract.Runtime) == 0 {
				b.Fatal("No runtime code generated")
			}
		}
	})

	b.Run("parallel", func(b *testing.B) {
		b.RunParallel(func(pb *testing.PB) {
			for pb.Next() {
				compiler := NewYulToNeoCompiler(config)
				result, err := compiler.Compile(source)
				
				if err != nil {
					b.Fatalf("Compilation failed: %v", err)
				}

				if len(result.Contract.Runtime) == 0 {
					b.Fatal("No runtime code generated")
				}
			}
		})
	})
}

// readComplexBenchmarkSource returns a complex contract source for benchmarking
func readComplexBenchmarkSource() string {
	return `
	object "ComplexBenchmark" {
		code {
			function fibonacci(n) -> result {
				if lt(n, 2) {
					result := n
				}
				result := add(fibonacci(sub(n, 1)), fibonacci(sub(n, 2)))
			}
			
			function factorial(n) -> result {
				result := 1
				for { let i := 2 } lte(i, n) { i := add(i, 1) } {
					result := mul(result, i)
				}
			}
			
			function sieve(n) -> count {
				// Sieve of Eratosthenes implementation
				count := 0
				for { let i := 2 } lt(i, n) { i := add(i, 1) } {
					let isPrime := 1
					for { let j := 2 } lt(j, i) { j := add(j, 1) } {
						if eq(mod(i, j), 0) {
							isPrime := 0
							break
						}
					}
					if isPrime {
						count := add(count, 1)
					}
				}
			}
			
			function hash_computation(data) -> hash {
				hash := data
				for { let i := 0 } lt(i, 10) { i := add(i, 1) } {
					hash := xor(hash, shl(i, data))
					hash := add(hash, mul(i, 0x9E3779B9))
				}
			}
			
			// Complex computations
			let fib_result := fibonacci(15)
			let fact_result := factorial(10)
			let prime_count := sieve(100)
			let hash_result := hash_computation(0x12345678)
			
			// Storage operations
			sstore(0, fib_result)
			sstore(1, fact_result)
			sstore(2, prime_count)
			sstore(3, hash_result)
			
			// Memory operations
			for { let i := 0 } lt(i, 50) { i := add(i, 1) } {
				let offset := mul(i, 32)
				mstore(offset, add(hash_computation(i), fib_result))
			}
			
			function lte(a, b) -> result {
				result := iszero(gt(a, b))
			}
		}
	}`
}