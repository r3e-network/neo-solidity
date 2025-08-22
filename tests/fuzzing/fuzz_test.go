package main

import (
	"fmt"
	"math/rand"
	"strings"
	"testing"
	"time"
	"unicode"
)

// FuzzResult represents the result of a fuzzing test
type FuzzResult struct {
	Input       string
	Success     bool
	Error       error
	Panicked    bool
	Duration    time.Duration
	Phase       string // Which phase failed: "Lexing", "Parsing", "CodeGen"
}

// TestFuzzLexer tests the lexer with random input
func TestFuzzLexer(t *testing.T) {
	fuzzConfig := FuzzConfig{
		Iterations:   1000,
		MaxInputSize: 1024,
		Timeout:      100 * time.Millisecond,
	}

	generator := NewRandomYulGenerator()
	results := make([]FuzzResult, 0, fuzzConfig.Iterations)

	for i := 0; i < fuzzConfig.Iterations; i++ {
		input := generator.GenerateRandomInput(fuzzConfig.MaxInputSize)
		result := fuzzLexer(input, fuzzConfig.Timeout)
		results = append(results, result)

		if result.Panicked {
			t.Errorf("Iteration %d: Lexer panicked on input: %q", i, truncateString(input, 100))
		}
	}

	analyzeFuzzResults(t, "Lexer", results)
}

// TestFuzzParser tests the parser with random but syntactically valid input
func TestFuzzParser(t *testing.T) {
	fuzzConfig := FuzzConfig{
		Iterations:   500,
		MaxInputSize: 2048,
		Timeout:      200 * time.Millisecond,
	}

	generator := NewRandomYulGenerator()
	results := make([]FuzzResult, 0, fuzzConfig.Iterations)

	for i := 0; i < fuzzConfig.Iterations; i++ {
		// Generate more structured input for parser testing
		input := generator.GenerateStructuredYul()
		result := fuzzParser(input, fuzzConfig.Timeout)
		results = append(results, result)

		if result.Panicked {
			t.Errorf("Iteration %d: Parser panicked on input: %q", i, truncateString(input, 100))
		}
	}

	analyzeFuzzResults(t, "Parser", results)
}

// TestFuzzCompiler tests the complete compilation pipeline
func TestFuzzCompiler(t *testing.T) {
	fuzzConfig := FuzzConfig{
		Iterations:   200,
		MaxInputSize: 4096,
		Timeout:      1 * time.Second,
	}

	generator := NewRandomYulGenerator()
	results := make([]FuzzResult, 0, fuzzConfig.Iterations)

	for i := 0; i < fuzzConfig.Iterations; i++ {
		input := generator.GenerateValidYulContract()
		result := fuzzCompiler(input, fuzzConfig.Timeout)
		results = append(results, result)

		if result.Panicked {
			t.Errorf("Iteration %d: Compiler panicked on input: %q", i, truncateString(input, 200))
		}

		// Log interesting failures for analysis
		if !result.Success && result.Error != nil && !result.Panicked {
			if i%50 == 0 { // Log every 50th failure
				t.Logf("Iteration %d: Expected failure - %s: %v", i, result.Phase, result.Error)
			}
		}
	}

	analyzeFuzzResults(t, "Compiler", results)
}

// TestFuzzWithKnownPatterns tests fuzzing with known problematic patterns
func TestFuzzWithKnownPatterns(t *testing.T) {
	// Known patterns that might cause issues
	problematicPatterns := []string{
		// Empty or minimal cases
		"",
		" ",
		"\n",
		"{}",
		"object {}",
		
		// Deeply nested structures
		strings.Repeat("{", 100) + strings.Repeat("}", 100),
		
		// Very long identifiers
		strings.Repeat("a", 1000),
		
		// Large numbers
		strings.Repeat("9", 500),
		
		// Unicode and special characters
		"🚀💰🔥",
		"ñáméößß",
		"\x00\x01\x02",
		
		// Malformed hex
		"0x" + strings.Repeat("g", 100),
		
		// Extremely long strings
		`"` + strings.Repeat("x", 10000) + `"`,
		
		// Recursive patterns
		"object \"a\" { object \"b\" { object \"c\" { code { object \"d\" {} } } } }",
		
		// Memory exhaustion patterns
		strings.Repeat("let x := add(", 1000) + "1" + strings.Repeat(")", 1000),
	}

	for i, pattern := range problematicPatterns {
		t.Run(fmt.Sprintf("pattern_%d", i), func(t *testing.T) {
			result := fuzzCompiler(pattern, 5*time.Second)
			
			if result.Panicked {
				t.Errorf("Pattern %d caused panic: %q", i, truncateString(pattern, 100))
			}
			
			// These patterns are expected to fail, but shouldn't panic
			if result.Success {
				t.Logf("Pattern %d unexpectedly succeeded: %q", i, truncateString(pattern, 100))
			}
		})
	}
}

// TestFuzzPropertyBasedTesting tests compiler properties using property-based testing
func TestFuzzPropertyBasedTesting(t *testing.T) {
	generator := NewRandomYulGenerator()
	
	properties := []struct {
		name string
		test func(input string) bool
	}{
		{
			name: "deterministic_compilation",
			test: func(input string) bool {
				// Same input should produce same output
				result1 := fuzzCompiler(input, 1*time.Second)
				result2 := fuzzCompiler(input, 1*time.Second)
				
				return (result1.Success == result2.Success) &&
					   (result1.Panicked == result2.Panicked) &&
					   ((result1.Error == nil) == (result2.Error == nil))
			},
		},
		{
			name: "no_crashes_on_valid_syntax",
			test: func(input string) bool {
				// Syntactically valid input should not cause panics
				if isValidYulSyntax(input) {
					result := fuzzCompiler(input, 2*time.Second)
					return !result.Panicked
				}
				return true // Skip invalid syntax
			},
		},
		{
			name: "compilation_timeout_respected",
			test: func(input string) bool {
				// Compilation should respect timeout
				timeout := 500 * time.Millisecond
				start := time.Now()
				fuzzCompiler(input, timeout)
				elapsed := time.Since(start)
				
				// Allow some margin for cleanup
				return elapsed < timeout*2
			},
		},
	}

	iterations := 100
	for _, prop := range properties {
		t.Run(prop.name, func(t *testing.T) {
			failures := 0
			
			for i := 0; i < iterations; i++ {
				input := generator.GenerateValidYulContract()
				if !prop.test(input) {
					failures++
					if failures <= 5 { // Log first few failures
						t.Logf("Property violation on input: %q", truncateString(input, 150))
					}
				}
			}
			
			failureRate := float64(failures) / float64(iterations) * 100
			t.Logf("Property %s: %d/%d failures (%.1f%%)", prop.name, failures, iterations, failureRate)
			
			if failureRate > 10 { // Allow up to 10% failure rate
				t.Errorf("Property %s failed too often: %.1f%%", prop.name, failureRate)
			}
		})
	}
}

// TestFuzzMutationalTesting tests using mutational fuzzing
func TestFuzzMutationalTesting(t *testing.T) {
	// Base valid contracts to mutate
	baseContracts := []string{
		`object "Simple" { code { let x := 1 } }`,
		`object "Complex" { 
			code { 
				function add(a, b) -> result { result := add(a, b) }
				let x := add(1, 2)
			} 
		}`,
		`object "ERC20" {
			code {
				switch div(calldataload(0), 0x100000000000000000000000000000000000000000000000000000000)
				case 0x70a08231 { returnUint(sload(calldataload(4))) }
				default { revert(0, 0) }
				
				function returnUint(value) {
					mstore(0, value)
					return(0, 32)
				}
			}
		}`,
	}

	mutator := NewYulMutator()
	
	for baseIdx, base := range baseContracts {
		t.Run(fmt.Sprintf("base_%d", baseIdx), func(t *testing.T) {
			for i := 0; i < 50; i++ {
				mutated := mutator.Mutate(base)
				result := fuzzCompiler(mutated, 1*time.Second)
				
				if result.Panicked {
					t.Errorf("Mutation %d of base %d caused panic: %q", 
						i, baseIdx, truncateString(mutated, 200))
				}
			}
		})
	}
}

// TestFuzzStructuralTesting tests structural properties
func TestFuzzStructuralTesting(t *testing.T) {
	generator := NewRandomYulGenerator()
	
	for i := 0; i < 100; i++ {
		t.Run(fmt.Sprintf("structural_%d", i), func(t *testing.T) {
			input := generator.GenerateValidYulContract()
			
			// Test that the lexer produces balanced tokens
			lexer := NewYulLexer()
			err := lexer.Init(input)
			if err != nil {
				return // Skip on init error
			}
			
			tokens, err := lexer.ScanTokens()
			if err != nil {
				return // Skip on lexing error
			}
			
			// Check token balance
			braceBalance := 0
			parenBalance := 0
			
			for _, token := range tokens {
				switch token.Type {
				case TokenLeftBrace:
					braceBalance++
				case TokenRightBrace:
					braceBalance--
				case TokenLeftParen:
					parenBalance++
				case TokenRightParen:
					parenBalance--
				}
				
				if braceBalance < 0 || parenBalance < 0 {
					t.Errorf("Negative balance detected in generated input: %q", 
						truncateString(input, 100))
					break
				}
			}
			
			if braceBalance != 0 {
				t.Errorf("Unbalanced braces in generated input: %q", 
					truncateString(input, 100))
			}
			
			if parenBalance != 0 {
				t.Errorf("Unbalanced parentheses in generated input: %q", 
					truncateString(input, 100))
			}
		})
	}
}

// Supporting types and functions

type FuzzConfig struct {
	Iterations   int
	MaxInputSize int
	Timeout      time.Duration
}

// fuzzLexer performs fuzzing on the lexer
func fuzzLexer(input string, timeout time.Duration) FuzzResult {
	result := FuzzResult{
		Input: input,
		Phase: "Lexing",
	}
	
	defer func() {
		if r := recover(); r != nil {
			result.Panicked = true
			result.Error = fmt.Errorf("panic: %v", r)
		}
	}()

	start := time.Now()
	
	lexer := NewYulLexer()
	err := lexer.Init(input)
	if err != nil {
		result.Error = err
		result.Duration = time.Since(start)
		return result
	}

	_, err = lexer.ScanTokens()
	result.Duration = time.Since(start)
	result.Error = err
	result.Success = err == nil

	return result
}

// fuzzParser performs fuzzing on the parser
func fuzzParser(input string, timeout time.Duration) FuzzResult {
	result := FuzzResult{
		Input: input,
		Phase: "Parsing",
	}
	
	defer func() {
		if r := recover(); r != nil {
			result.Panicked = true
			result.Error = fmt.Errorf("panic: %v", r)
		}
	}()

	start := time.Now()
	
	parser := NewYulParser()
	_, err := parser.Parse(input)
	
	result.Duration = time.Since(start)
	result.Error = err
	result.Success = err == nil

	return result
}

// fuzzCompiler performs fuzzing on the complete compiler
func fuzzCompiler(input string, timeout time.Duration) FuzzResult {
	result := FuzzResult{
		Input: input,
		Phase: "Compilation",
	}
	
	defer func() {
		if r := recover(); r != nil {
			result.Panicked = true
			result.Error = fmt.Errorf("panic: %v", r)
		}
	}()

	start := time.Now()
	
	config := CompilerConfig{
		OptimizationLevel:    1,
		TargetNeoVMVersion:   "3.0",
		EnableBoundsChecking: false,
		EnableDebugInfo:     false,
		MaxStackDepth:       1024,
		MemoryLimit:         32 * 1024 * 1024,
	}

	compiler := NewYulToNeoCompiler(config)
	_, err := compiler.Compile(input)
	
	result.Duration = time.Since(start)
	result.Error = err
	result.Success = err == nil

	return result
}

// RandomYulGenerator generates random Yul inputs
type RandomYulGenerator struct {
	rand *rand.Rand
}

func NewRandomYulGenerator() *RandomYulGenerator {
	return &RandomYulGenerator{
		rand: rand.New(rand.NewSource(time.Now().UnixNano())),
	}
}

func (g *RandomYulGenerator) GenerateRandomInput(maxSize int) string {
	size := g.rand.Intn(maxSize + 1)
	result := make([]byte, size)
	
	for i := 0; i < size; i++ {
		result[i] = byte(g.rand.Intn(256))
	}
	
	return string(result)
}

func (g *RandomYulGenerator) GenerateStructuredYul() string {
	patterns := []string{
		"object \"Test\" { code { %s } }",
		"{ %s }",
		"function test() { %s }",
		"let x := %s",
		"if %s { %s }",
		"for { %s } %s { %s } { %s }",
		"switch %s case %s { %s } default { %s }",
	}
	
	pattern := patterns[g.rand.Intn(len(patterns))]
	
	// Fill pattern with random elements
	elements := []string{"1", "x", "add(1, 2)", "true", "false", "0x42"}
	filledPattern := pattern
	
	for strings.Contains(filledPattern, "%s") {
		element := elements[g.rand.Intn(len(elements))]
		filledPattern = strings.Replace(filledPattern, "%s", element, 1)
	}
	
	return filledPattern
}

func (g *RandomYulGenerator) GenerateValidYulContract() string {
	templates := []string{
		`object "Contract%d" {
			code {
				let x := %d
				let y := add(x, %d)
			}
		}`,
		`object "Contract%d" {
			code {
				function test(a) -> result {
					result := add(a, %d)
				}
				let value := test(%d)
			}
		}`,
		`object "Contract%d" {
			code {
				if gt(%d, %d) {
					sstore(0, %d)
				}
			}
		}`,
		`object "Contract%d" {
			code {
				for { let i := 0 } lt(i, %d) { i := add(i, 1) } {
					sstore(i, mul(i, %d))
				}
			}
		}`,
	}
	
	template := templates[g.rand.Intn(len(templates))]
	
	// Fill template with random values
	contractId := g.rand.Intn(10000)
	values := make([]interface{}, 10)
	values[0] = contractId
	
	for i := 1; i < len(values); i++ {
		values[i] = g.rand.Intn(1000)
	}
	
	return fmt.Sprintf(template, values...)
}

// YulMutator performs mutations on Yul code
type YulMutator struct {
	rand *rand.Rand
}

func NewYulMutator() *YulMutator {
	return &YulMutator{
		rand: rand.New(rand.NewSource(time.Now().UnixNano())),
	}
}

func (m *YulMutator) Mutate(input string) string {
	mutations := []func(string) string{
		m.insertRandomChar,
		m.deleteRandomChar,
		m.replaceRandomChar,
		m.duplicateRandomSubstring,
		m.swapRandomChars,
		m.insertRandomToken,
		m.replaceRandomNumber,
	}
	
	// Apply 1-3 mutations
	mutationCount := m.rand.Intn(3) + 1
	result := input
	
	for i := 0; i < mutationCount; i++ {
		mutation := mutations[m.rand.Intn(len(mutations))]
		result = mutation(result)
	}
	
	return result
}

func (m *YulMutator) insertRandomChar(input string) string {
	if len(input) == 0 {
		return string(rune(m.rand.Intn(256)))
	}
	
	pos := m.rand.Intn(len(input) + 1)
	char := rune(m.rand.Intn(256))
	
	return input[:pos] + string(char) + input[pos:]
}

func (m *YulMutator) deleteRandomChar(input string) string {
	if len(input) == 0 {
		return input
	}
	
	pos := m.rand.Intn(len(input))
	return input[:pos] + input[pos+1:]
}

func (m *YulMutator) replaceRandomChar(input string) string {
	if len(input) == 0 {
		return input
	}
	
	pos := m.rand.Intn(len(input))
	char := rune(m.rand.Intn(256))
	
	return input[:pos] + string(char) + input[pos+1:]
}

func (m *YulMutator) duplicateRandomSubstring(input string) string {
	if len(input) == 0 {
		return input
	}
	
	start := m.rand.Intn(len(input))
	end := start + m.rand.Intn(len(input)-start) + 1
	if end > len(input) {
		end = len(input)
	}
	
	substring := input[start:end]
	insertPos := m.rand.Intn(len(input) + 1)
	
	return input[:insertPos] + substring + input[insertPos:]
}

func (m *YulMutator) swapRandomChars(input string) string {
	if len(input) < 2 {
		return input
	}
	
	pos1 := m.rand.Intn(len(input))
	pos2 := m.rand.Intn(len(input))
	
	if pos1 == pos2 {
		return input
	}
	
	if pos1 > pos2 {
		pos1, pos2 = pos2, pos1
	}
	
	runes := []rune(input)
	runes[pos1], runes[pos2] = runes[pos2], runes[pos1]
	
	return string(runes)
}

func (m *YulMutator) insertRandomToken(input string) string {
	tokens := []string{"let", "function", "if", "for", "switch", "case", "default", 
		"add", "sub", "mul", "div", "sload", "sstore", "mload", "mstore", "revert"}
	
	token := tokens[m.rand.Intn(len(tokens))]
	pos := m.rand.Intn(len(input) + 1)
	
	return input[:pos] + " " + token + " " + input[pos:]
}

func (m *YulMutator) replaceRandomNumber(input string) string {
	// Find numbers and replace with random values
	result := ""
	i := 0
	
	for i < len(input) {
		if unicode.IsDigit(rune(input[i])) {
			// Skip the number
			for i < len(input) && (unicode.IsDigit(rune(input[i])) || input[i] == 'x' || 
				(input[i] >= 'a' && input[i] <= 'f') || (input[i] >= 'A' && input[i] <= 'F')) {
				i++
			}
			// Insert random number
			result += fmt.Sprintf("%d", m.rand.Intn(10000))
		} else {
			result += string(input[i])
			i++
		}
	}
	
	return result
}

// Helper functions

func analyzeFuzzResults(t *testing.T, component string, results []FuzzResult) {
	totalTests := len(results)
	successes := 0
	panics := 0
	totalDuration := time.Duration(0)
	
	for _, result := range results {
		if result.Success {
			successes++
		}
		if result.Panicked {
			panics++
		}
		totalDuration += result.Duration
	}
	
	successRate := float64(successes) / float64(totalTests) * 100
	panicRate := float64(panics) / float64(totalTests) * 100
	avgDuration := totalDuration / time.Duration(totalTests)
	
	t.Logf("%s Fuzz Results:", component)
	t.Logf("  Total tests: %d", totalTests)
	t.Logf("  Success rate: %.1f%% (%d/%d)", successRate, successes, totalTests)
	t.Logf("  Panic rate: %.1f%% (%d/%d)", panicRate, panics, totalTests)
	t.Logf("  Average duration: %v", avgDuration.Round(time.Microsecond))
	
	if panicRate > 1.0 { // More than 1% panics is concerning
		t.Errorf("%s has high panic rate: %.1f%%", component, panicRate)
	}
}

func isValidYulSyntax(input string) bool {
	// Simple heuristic to check if input might be valid Yul
	return strings.Contains(input, "object") || strings.Contains(input, "code") || 
		   strings.Contains(input, "function") || strings.Contains(input, "let")
}

func truncateString(s string, maxLen int) string {
	if len(s) <= maxLen {
		return s
	}
	return s[:maxLen] + "..."
}