package main

import (
	"reflect"
	"strings"
	"testing"
)

// TestYulLexerBasicTokenization tests basic token recognition
func TestYulLexerBasicTokenization(t *testing.T) {
	tests := []struct {
		name     string
		source   string
		expected []TokenType
	}{
		{
			name:   "single characters",
			source: "(){},:.",
			expected: []TokenType{
				TokenLeftParen, TokenRightParen, TokenLeftBrace, TokenRightBrace,
				TokenComma, TokenColon, TokenDot, TokenEOF,
			},
		},
		{
			name:     "assignment and arrow",
			source:   ":= ->",
			expected: []TokenType{TokenColonEqual, TokenArrow, TokenEOF},
		},
		{
			name:     "keywords",
			source:   "object code data function let if switch case default for break continue leave true false",
			expected: []TokenType{
				TokenObject, TokenCode, TokenData, TokenFunction, TokenLet,
				TokenIf, TokenSwitch, TokenCase, TokenDefault, TokenFor,
				TokenBreak, TokenContinue, TokenLeave, TokenTrue, TokenFalse, TokenEOF,
			},
		},
		{
			name:     "identifiers",
			source:   "myVar _private var123 CamelCase",
			expected: []TokenType{
				TokenIdentifier, TokenIdentifier, TokenIdentifier, TokenIdentifier, TokenEOF,
			},
		},
		{
			name:     "numbers",
			source:   "0 123 9999999999999999999",
			expected: []TokenType{TokenNumber, TokenNumber, TokenNumber, TokenEOF},
		},
		{
			name:     "hex numbers",
			source:   "0x0 0x123 0xabc 0xABC 0xdeadbeef",
			expected: []TokenType{TokenHex, TokenHex, TokenHex, TokenHex, TokenHex, TokenEOF},
		},
		{
			name:     "strings",
			source:   `"hello" "world with spaces" ""`,
			expected: []TokenType{TokenString, TokenString, TokenString, TokenEOF},
		},
		{
			name:     "arithmetic built-ins",
			source:   "add sub mul div mod",
			expected: []TokenType{
				TokenArithmetic, TokenArithmetic, TokenArithmetic, TokenArithmetic, TokenArithmetic, TokenEOF,
			},
		},
		{
			name:     "memory built-ins",
			source:   "mload mstore calldataload codecopy",
			expected: []TokenType{
				TokenMemory, TokenMemory, TokenMemory, TokenMemory, TokenEOF,
			},
		},
		{
			name:     "storage built-ins",
			source:   "sload sstore",
			expected: []TokenType{TokenStorage, TokenStorage, TokenEOF},
		},
		{
			name:     "environment built-ins",
			source:   "address balance caller origin timestamp",
			expected: []TokenType{
				TokenEnvironment, TokenEnvironment, TokenEnvironment, TokenEnvironment, TokenEnvironment, TokenEOF,
			},
		},
		{
			name:     "control built-ins",
			source:   "call return revert selfdestruct log0",
			expected: []TokenType{
				TokenControl, TokenControl, TokenControl, TokenControl, TokenControl, TokenEOF,
			},
		},
	}

	for _, test := range tests {
		t.Run(test.name, func(t *testing.T) {
			lexer := NewYulLexer()
			err := lexer.Init(test.source)
			if err != nil {
				t.Fatalf("Failed to initialize lexer: %v", err)
			}

			tokens, err := lexer.ScanTokens()
			if err != nil {
				t.Fatalf("Failed to scan tokens: %v", err)
			}

			if len(tokens) != len(test.expected) {
				t.Fatalf("Expected %d tokens, got %d", len(test.expected), len(tokens))
			}

			for i, token := range tokens {
				if token.Type != test.expected[i] {
					t.Errorf("Token %d: expected %s, got %s", i, test.expected[i], token.Type)
				}
			}
		})
	}
}

// TestYulLexerStringHandling tests string literal parsing with edge cases
func TestYulLexerStringHandling(t *testing.T) {
	tests := []struct {
		name     string
		source   string
		expected []string
		hasError bool
	}{
		{
			name:     "simple string",
			source:   `"hello"`,
			expected: []string{"hello"},
		},
		{
			name:     "empty string",
			source:   `""`,
			expected: []string{""},
		},
		{
			name:     "string with spaces",
			source:   `"hello world"`,
			expected: []string{"hello world"},
		},
		{
			name:     "string with numbers",
			source:   `"test123"`,
			expected: []string{"test123"},
		},
		{
			name:     "string with special chars",
			source:   `"!@#$%^&*()"`,
			expected: []string{"!@#$%^&*()"},
		},
		{
			name:     "multiple strings",
			source:   `"first" "second" "third"`,
			expected: []string{"first", "second", "third"},
		},
		{
			name:     "multiline string",
			source:   "\"line1\nline2\"",
			expected: []string{"line1\nline2"},
		},
		{
			name:     "unterminated string",
			source:   `"unterminated`,
			hasError: true,
		},
	}

	for _, test := range tests {
		t.Run(test.name, func(t *testing.T) {
			lexer := NewYulLexer()
			err := lexer.Init(test.source)
			if err != nil {
				t.Fatalf("Failed to initialize lexer: %v", err)
			}

			tokens, err := lexer.ScanTokens()
			if test.hasError {
				if err == nil {
					t.Fatalf("Expected error but got none")
				}
				return
			}

			if err != nil {
				t.Fatalf("Unexpected error: %v", err)
			}

			stringTokens := []Token{}
			for _, token := range tokens {
				if token.Type == TokenString {
					stringTokens = append(stringTokens, token)
				}
			}

			if len(stringTokens) != len(test.expected) {
				t.Fatalf("Expected %d string tokens, got %d", len(test.expected), len(stringTokens))
			}

			for i, token := range stringTokens {
				if token.Lexeme != test.expected[i] {
					t.Errorf("String %d: expected %q, got %q", i, test.expected[i], token.Lexeme)
				}
			}
		})
	}
}

// TestYulLexerNumberHandling tests number parsing with various formats
func TestYulLexerNumberHandling(t *testing.T) {
	tests := []struct {
		name     string
		source   string
		expected []struct {
			value string
			ttype TokenType
		}
		hasError bool
	}{
		{
			name:   "decimal numbers",
			source: "0 1 123 999999",
			expected: []struct {
				value string
				ttype TokenType
			}{
				{"0", TokenNumber},
				{"1", TokenNumber},
				{"123", TokenNumber},
				{"999999", TokenNumber},
			},
		},
		{
			name:   "hex numbers",
			source: "0x0 0x1 0x123 0xabc 0xABC 0xDeAdBeEf",
			expected: []struct {
				value string
				ttype TokenType
			}{
				{"0x0", TokenHex},
				{"0x1", TokenHex},
				{"0x123", TokenHex},
				{"0xabc", TokenHex},
				{"0xABC", TokenHex},
				{"0xDeAdBeEf", TokenHex},
			},
		},
		{
			name:   "large numbers",
			source: "115792089237316195423570985008687907853269984665640564039457584007913129639935",
			expected: []struct {
				value string
				ttype TokenType
			}{
				{"115792089237316195423570985008687907853269984665640564039457584007913129639935", TokenNumber},
			},
		},
		{
			name:     "invalid hex",
			source:   "0x",
			hasError: true,
		},
		{
			name:     "invalid hex chars",
			source:   "0xgg",
			hasError: true,
		},
	}

	for _, test := range tests {
		t.Run(test.name, func(t *testing.T) {
			lexer := NewYulLexer()
			err := lexer.Init(test.source)
			if err != nil {
				t.Fatalf("Failed to initialize lexer: %v", err)
			}

			tokens, err := lexer.ScanTokens()
			if test.hasError {
				if err == nil {
					t.Fatalf("Expected error but got none")
				}
				return
			}

			if err != nil {
				t.Fatalf("Unexpected error: %v", err)
			}

			numberTokens := []Token{}
			for _, token := range tokens {
				if token.Type == TokenNumber || token.Type == TokenHex {
					numberTokens = append(numberTokens, token)
				}
			}

			if len(numberTokens) != len(test.expected) {
				t.Fatalf("Expected %d number tokens, got %d", len(test.expected), len(numberTokens))
			}

			for i, token := range numberTokens {
				if token.Lexeme != test.expected[i].value {
					t.Errorf("Number %d: expected %q, got %q", i, test.expected[i].value, token.Lexeme)
				}
				if token.Type != test.expected[i].ttype {
					t.Errorf("Number %d type: expected %s, got %s", i, test.expected[i].ttype, token.Type)
				}
			}
		})
	}
}

// TestYulLexerComments tests comment handling
func TestYulLexerComments(t *testing.T) {
	tests := []struct {
		name     string
		source   string
		expected []TokenType
		hasError bool
	}{
		{
			name:     "line comment",
			source:   "let x // this is a comment\nlet y",
			expected: []TokenType{TokenLet, TokenIdentifier, TokenLet, TokenIdentifier, TokenEOF},
		},
		{
			name:     "block comment",
			source:   "let x /* this is a block comment */ let y",
			expected: []TokenType{TokenLet, TokenIdentifier, TokenLet, TokenIdentifier, TokenEOF},
		},
		{
			name:     "nested block comments",
			source:   "let x /* outer /* inner */ outer */ let y",
			expected: []TokenType{TokenLet, TokenIdentifier, TokenLet, TokenIdentifier, TokenEOF},
		},
		{
			name:     "multiline block comment",
			source:   "let x /* line1\nline2\nline3 */ let y",
			expected: []TokenType{TokenLet, TokenIdentifier, TokenLet, TokenIdentifier, TokenEOF},
		},
		{
			name:     "comment at end",
			source:   "let x // comment",
			expected: []TokenType{TokenLet, TokenIdentifier, TokenEOF},
		},
		{
			name:     "unterminated block comment",
			source:   "let x /* unterminated",
			hasError: true,
		},
	}

	for _, test := range tests {
		t.Run(test.name, func(t *testing.T) {
			lexer := NewYulLexer()
			err := lexer.Init(test.source)
			if err != nil {
				t.Fatalf("Failed to initialize lexer: %v", err)
			}

			tokens, err := lexer.ScanTokens()
			if test.hasError {
				if err == nil {
					t.Fatalf("Expected error but got none")
				}
				return
			}

			if err != nil {
				t.Fatalf("Unexpected error: %v", err)
			}

			if len(tokens) != len(test.expected) {
				t.Fatalf("Expected %d tokens, got %d", len(test.expected), len(tokens))
			}

			for i, token := range tokens {
				if token.Type != test.expected[i] {
					t.Errorf("Token %d: expected %s, got %s", i, test.expected[i], token.Type)
				}
			}
		})
	}
}

// TestYulLexerWhitespace tests whitespace handling
func TestYulLexerWhitespace(t *testing.T) {
	tests := []struct {
		name     string
		source   string
		expected []TokenType
	}{
		{
			name:     "spaces",
			source:   "let    x    :=    1",
			expected: []TokenType{TokenLet, TokenIdentifier, TokenColonEqual, TokenNumber, TokenEOF},
		},
		{
			name:     "tabs",
			source:   "let\tx\t:=\t1",
			expected: []TokenType{TokenLet, TokenIdentifier, TokenColonEqual, TokenNumber, TokenEOF},
		},
		{
			name:     "newlines",
			source:   "let\nx\n:=\n1",
			expected: []TokenType{TokenLet, TokenIdentifier, TokenColonEqual, TokenNumber, TokenEOF},
		},
		{
			name:     "mixed whitespace",
			source:   "let \t\n x   \t\n:= \t 1",
			expected: []TokenType{TokenLet, TokenIdentifier, TokenColonEqual, TokenNumber, TokenEOF},
		},
	}

	for _, test := range tests {
		t.Run(test.name, func(t *testing.T) {
			lexer := NewYulLexer()
			err := lexer.Init(test.source)
			if err != nil {
				t.Fatalf("Failed to initialize lexer: %v", err)
			}

			tokens, err := lexer.ScanTokens()
			if err != nil {
				t.Fatalf("Unexpected error: %v", err)
			}

			if len(tokens) != len(test.expected) {
				t.Fatalf("Expected %d tokens, got %d", len(test.expected), len(tokens))
			}

			for i, token := range tokens {
				if token.Type != test.expected[i] {
					t.Errorf("Token %d: expected %s, got %s", i, test.expected[i], token.Type)
				}
			}
		})
	}
}

// TestYulLexerPositioning tests that position information is correctly tracked
func TestYulLexerPositioning(t *testing.T) {
	source := "let x := 1\nlet y := 2"
	lexer := NewYulLexer()
	err := lexer.Init(source)
	if err != nil {
		t.Fatalf("Failed to initialize lexer: %v", err)
	}

	tokens, err := lexer.ScanTokens()
	if err != nil {
		t.Fatalf("Failed to scan tokens: %v", err)
	}

	expectedPositions := []struct {
		line   int
		column int
		offset int
		length int
	}{
		{1, 1, 0, 3},   // let
		{1, 5, 4, 1},   // x
		{1, 7, 6, 2},   // :=
		{1, 10, 9, 1},  // 1
		{2, 1, 11, 3},  // let
		{2, 5, 15, 1},  // y
		{2, 7, 17, 2},  // :=
		{2, 10, 20, 1}, // 2
		{2, 11, 21, 0}, // EOF
	}

	if len(tokens) != len(expectedPositions) {
		t.Fatalf("Expected %d tokens, got %d", len(expectedPositions), len(tokens))
	}

	for i, token := range tokens {
		expected := expectedPositions[i]
		if token.Line != expected.line {
			t.Errorf("Token %d line: expected %d, got %d", i, expected.line, token.Line)
		}
		if token.Position.Column != expected.column {
			t.Errorf("Token %d column: expected %d, got %d", i, expected.column, token.Position.Column)
		}
		if token.Position.Offset != expected.offset {
			t.Errorf("Token %d offset: expected %d, got %d", i, expected.offset, token.Position.Offset)
		}
		if token.Position.Length != expected.length {
			t.Errorf("Token %d length: expected %d, got %d", i, expected.length, token.Position.Length)
		}
	}
}

// TestYulLexerErrorHandling tests various error conditions
func TestYulLexerErrorHandling(t *testing.T) {
	tests := []struct {
		name     string
		source   string
		errorMsg string
	}{
		{
			name:     "unexpected character",
			source:   "@",
			errorMsg: "unexpected character",
		},
		{
			name:     "single dash",
			source:   "-",
			errorMsg: "unexpected character '-'",
		},
		{
			name:     "single slash",
			source:   "/",
			errorMsg: "unexpected character '/'",
		},
		{
			name:     "unterminated string",
			source:   `"hello`,
			errorMsg: "unterminated string",
		},
		{
			name:     "empty source",
			source:   "",
			errorMsg: "empty source code",
		},
	}

	for _, test := range tests {
		t.Run(test.name, func(t *testing.T) {
			lexer := NewYulLexer()
			
			if test.source == "" {
				err := lexer.Init(test.source)
				if err == nil {
					t.Fatalf("Expected error for empty source")
				}
				if !strings.Contains(err.Error(), test.errorMsg) {
					t.Errorf("Expected error containing %q, got %q", test.errorMsg, err.Error())
				}
				return
			}
			
			err := lexer.Init(test.source)
			if err != nil {
				t.Fatalf("Failed to initialize lexer: %v", err)
			}

			_, err = lexer.ScanTokens()
			if err == nil {
				t.Fatalf("Expected error but got none")
			}

			if !strings.Contains(err.Error(), test.errorMsg) {
				t.Errorf("Expected error containing %q, got %q", test.errorMsg, err.Error())
			}
		})
	}
}

// TestYulLexerComplexProgram tests lexing of a complex Yul program
func TestYulLexerComplexProgram(t *testing.T) {
	source := `
	object "Contract" {
		code {
			datacopy(0, dataoffset("runtime"), datasize("runtime"))
			return(0, datasize("runtime"))
		}
		object "runtime" {
			code {
				let selector := div(calldataload(0), 0x100000000000000000000000000000000000000000000000000000000)
				
				switch selector
				case 0x60fe47b1 { // set(uint256)
					let value := calldataload(4)
					sstore(0, value)
				}
				case 0x6d4ce63c { // get()
					let value := sload(0)
					mstore(0, value)
					return(0, 32)
				}
				default {
					revert(0, 0)
				}
			}
		}
	}
	`

	lexer := NewYulLexer()
	err := lexer.Init(source)
	if err != nil {
		t.Fatalf("Failed to initialize lexer: %v", err)
	}

	tokens, err := lexer.ScanTokens()
	if err != nil {
		t.Fatalf("Failed to scan tokens: %v", err)
	}

	// Verify we got a reasonable number of tokens
	if len(tokens) < 50 {
		t.Errorf("Expected at least 50 tokens for complex program, got %d", len(tokens))
	}

	// Verify structure tokens are present
	structureTokens := []TokenType{
		TokenObject, TokenCode, TokenSwitch, TokenCase, TokenDefault,
		TokenFunction, TokenLet, TokenColonEqual,
	}

	for _, expectedType := range structureTokens {
		found := false
		for _, token := range tokens {
			if token.Type == expectedType {
				found = true
				break
			}
		}
		if !found {
			t.Errorf("Expected to find token type %s in complex program", expectedType)
		}
	}

	// Verify built-in functions are correctly categorized
	builtinTypes := []TokenType{
		TokenMemory, TokenStorage, TokenControl, TokenArithmetic,
	}

	for _, expectedType := range builtinTypes {
		found := false
		for _, token := range tokens {
			if token.Type == expectedType {
				found = true
				break
			}
		}
		if !found {
			t.Errorf("Expected to find built-in type %s in complex program", expectedType)
		}
	}
}

// TestYulLexerPatternMatching tests the pattern matching functionality
func TestYulLexerPatternMatching(t *testing.T) {
	source := `
	function test() -> result {
		let x := 1
		let y := add(x, 2)
		if iszero(y) {
			result := 0
		}
		switch y
		case 0 { result := 1 }
		default { result := 2 }
	}
	`

	lexer := NewYulLexer()
	err := lexer.Init(source)
	if err != nil {
		t.Fatalf("Failed to initialize lexer: %v", err)
	}

	tokens, err := lexer.ScanTokens()
	if err != nil {
		t.Fatalf("Failed to scan tokens: %v", err)
	}

	// Test pattern matching
	patterns := FindPatterns(tokens, CommonPatterns)

	// Should find function definition
	if len(patterns["function_definition"]) == 0 {
		t.Error("Expected to find function definition pattern")
	}

	// Should find variable declarations
	if len(patterns["variable_declaration"]) < 2 {
		t.Error("Expected to find at least 2 variable declaration patterns")
	}

	// Should find conditional block
	if len(patterns["conditional_block"]) == 0 {
		t.Error("Expected to find conditional block pattern")
	}

	// Should find switch statement
	if len(patterns["switch_statement"]) == 0 {
		t.Error("Expected to find switch statement pattern")
	}
}

// TestYulLexerAnalysis tests the lexical analysis functionality
func TestYulLexerAnalysis(t *testing.T) {
	source := `
	function factorial(n) -> result {
		result := 1
		for { let i := 2 } lt(i, add(n, 1)) { i := add(i, 1) } {
			result := mul(result, i)
		}
	}
	`

	lexer := NewYulLexer()
	err := lexer.Init(source)
	if err != nil {
		t.Fatalf("Failed to initialize lexer: %v", err)
	}

	tokens, err := lexer.ScanTokens()
	if err != nil {
		t.Fatalf("Failed to scan tokens: %v", err)
	}

	analysis := AnalyzeTokens(tokens)

	// Verify basic counts
	if analysis.Functions != 1 {
		t.Errorf("Expected 1 function, got %d", analysis.Functions)
	}

	if analysis.Variables < 3 {
		t.Errorf("Expected at least 3 variables, got %d", analysis.Variables)
	}

	if analysis.BuiltinCalls < 3 {
		t.Errorf("Expected at least 3 builtin calls, got %d", analysis.BuiltinCalls)
	}

	if analysis.TotalTokens < 20 {
		t.Errorf("Expected at least 20 total tokens, got %d", analysis.TotalTokens)
	}

	// Verify complexity is reasonable
	if analysis.Complexity <= 0 {
		t.Errorf("Expected positive complexity score, got %f", analysis.Complexity)
	}
}

// TestYulLexerValidation tests the token stream validation
func TestYulLexerValidation(t *testing.T) {
	tests := []struct {
		name     string
		source   string
		hasError bool
		errorMsg string
	}{
		{
			name:   "valid program",
			source: "{ let x := 1 }",
		},
		{
			name:     "unmatched opening brace",
			source:   "{ let x := 1",
			hasError: true,
			errorMsg: "unmatched braces",
		},
		{
			name:     "unmatched closing brace",
			source:   "let x := 1 }",
			hasError: true,
			errorMsg: "unmatched '}'",
		},
		{
			name:     "unmatched opening paren",
			source:   "add(1, 2",
			hasError: true,
			errorMsg: "unmatched parentheses",
		},
		{
			name:     "unmatched closing paren",
			source:   "add 1, 2)",
			hasError: true,
			errorMsg: "unmatched ')'",
		},
	}

	for _, test := range tests {
		t.Run(test.name, func(t *testing.T) {
			lexer := NewYulLexer()
			err := lexer.Init(test.source)
			if err != nil {
				t.Fatalf("Failed to initialize lexer: %v", err)
			}

			tokens, err := lexer.ScanTokens()
			if err != nil {
				t.Fatalf("Failed to scan tokens: %v", err)
			}

			err = ValidateTokenStream(tokens)
			if test.hasError {
				if err == nil {
					t.Fatalf("Expected validation error but got none")
				}
				if !strings.Contains(err.Error(), test.errorMsg) {
					t.Errorf("Expected error containing %q, got %q", test.errorMsg, err.Error())
				}
			} else {
				if err != nil {
					t.Errorf("Unexpected validation error: %v", err)
				}
			}
		})
	}
}

// BenchmarkYulLexer benchmarks lexer performance
func BenchmarkYulLexer(b *testing.B) {
	// Large Yul program for benchmarking
	source := strings.Repeat(`
	object "Contract" {
		code {
			datacopy(0, dataoffset("runtime"), datasize("runtime"))
			return(0, datasize("runtime"))
		}
		object "runtime" {
			code {
				let selector := div(calldataload(0), 0x100000000000000000000000000000000000000000000000000000000)
				
				switch selector
				case 0x60fe47b1 {
					let value := calldataload(4)
					sstore(0, value)
				}
				case 0x6d4ce63c {
					let value := sload(0)
					mstore(0, value)
					return(0, 32)
				}
				default {
					revert(0, 0)
				}
			}
		}
	}
	`, 10) // Repeat 10 times for a larger program

	b.ResetTimer()

	for i := 0; i < b.N; i++ {
		lexer := NewYulLexer()
		err := lexer.Init(source)
		if err != nil {
			b.Fatalf("Failed to initialize lexer: %v", err)
		}

		_, err = lexer.ScanTokens()
		if err != nil {
			b.Fatalf("Failed to scan tokens: %v", err)
		}
	}
}

// TestYulLexerEdgeCases tests various edge cases and boundary conditions
func TestYulLexerEdgeCases(t *testing.T) {
	tests := []struct {
		name     string
		source   string
		expected int // Expected token count (excluding EOF)
		hasError bool
	}{
		{
			name:     "single token",
			source:   "let",
			expected: 1,
		},
		{
			name:     "single character",
			source:   "{",
			expected: 1,
		},
		{
			name:     "only whitespace",
			source:   "   \t\n  ",
			expected: 0,
		},
		{
			name:     "only comments",
			source:   "// comment\n/* block comment */",
			expected: 0,
		},
		{
			name:     "maximum identifier length",
			source:   strings.Repeat("a", 1000),
			expected: 1,
		},
		{
			name:     "maximum string length",
			source:   `"` + strings.Repeat("x", 1000) + `"`,
			expected: 1,
		},
		{
			name:     "unicode in comments",
			source:   "let x // comment with unicode: 🚀 αβγ 中文",
			expected: 2,
		},
		{
			name:     "all keywords together",
			source:   "object code data function let if switch case default for break continue leave true false",
			expected: 14,
		},
		{
			name:     "nested operators",
			source:   "::= ->->",
			expected: 4, // : := -> ->
		},
	}

	for _, test := range tests {
		t.Run(test.name, func(t *testing.T) {
			lexer := NewYulLexer()
			err := lexer.Init(test.source)
			if err != nil {
				t.Fatalf("Failed to initialize lexer: %v", err)
			}

			tokens, err := lexer.ScanTokens()
			if test.hasError {
				if err == nil {
					t.Fatalf("Expected error but got none")
				}
				return
			}

			if err != nil {
				t.Fatalf("Unexpected error: %v", err)
			}

			// Count non-EOF tokens
			nonEofTokens := 0
			for _, token := range tokens {
				if token.Type != TokenEOF {
					nonEofTokens++
				}
			}

			if nonEofTokens != test.expected {
				t.Errorf("Expected %d non-EOF tokens, got %d", test.expected, nonEofTokens)
			}
		})
	}
}

// TestYulLexerRealWorldExample tests lexing of a real ERC20-like contract
func TestYulLexerRealWorldExample(t *testing.T) {
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
					
					// Check balance
					let fromBalance := sload(from)
					if lt(fromBalance, value) { revert(0, 0) }
					
					// Update balances
					sstore(from, sub(fromBalance, value))
					let toBalance := sload(to)
					sstore(to, add(toBalance, value))
					
					// Emit Transfer event
					log3(0, 0, 0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef, from, to)
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
				}
				default {
					revert(0, 0)
				}
			}
		}
	}
	`

	lexer := NewYulLexer()
	err := lexer.Init(source)
	if err != nil {
		t.Fatalf("Failed to initialize lexer: %v", err)
	}

	tokens, err := lexer.ScanTokens()
	if err != nil {
		t.Fatalf("Failed to scan tokens: %v", err)
	}

	// Perform analysis
	analysis := AnalyzeTokens(tokens)

	// Verify the analysis makes sense for an ERC20 contract
	if analysis.Functions < 3 {
		t.Errorf("Expected at least 3 logical functions in ERC20, got %d variables (let statements)", analysis.Variables)
	}

	if analysis.Variables < 10 {
		t.Errorf("Expected at least 10 variables in ERC20, got %d", analysis.Variables)
	}

	if analysis.BuiltinCalls < 15 {
		t.Errorf("Expected at least 15 builtin calls in ERC20, got %d", analysis.BuiltinCalls)
	}

	// Check for specific ERC20-related patterns
	patterns := FindPatterns(tokens, CommonPatterns)
	
	if len(patterns["switch_statement"]) == 0 {
		t.Error("Expected to find switch statement (function selector) in ERC20")
	}

	if len(patterns["conditional_block"]) == 0 {
		t.Error("Expected to find conditional blocks in ERC20")
	}

	// Verify proper token stream validation
	err = ValidateTokenStream(tokens)
	if err != nil {
		t.Errorf("ERC20 token stream validation failed: %v", err)
	}
}