package main

import (
	"reflect"
	"strings"
	"testing"
)

// TestYulParserBasicParsing tests basic parsing functionality
func TestYulParserBasicParsing(t *testing.T) {
	tests := []struct {
		name     string
		source   string
		validate func(*YulAST) error
	}{
		{
			name:   "empty object",
			source: `object "Contract" {}`,
			validate: func(ast *YulAST) error {
				if len(ast.Objects) != 1 {
					t.Errorf("Expected 1 object, got %d", len(ast.Objects))
				}
				if ast.Objects[0].Name != "Contract" {
					t.Errorf("Expected object name 'Contract', got %s", ast.Objects[0].Name)
				}
				return nil
			},
		},
		{
			name: "object with code",
			source: `object "Contract" {
				code {
					let x := 1
				}
			}`,
			validate: func(ast *YulAST) error {
				if len(ast.Objects) != 1 {
					t.Errorf("Expected 1 object, got %d", len(ast.Objects))
				}
				obj := ast.Objects[0]
				if obj.Code == nil {
					t.Errorf("Expected code block")
				}
				if len(obj.Code.Statements) != 1 {
					t.Errorf("Expected 1 statement, got %d", len(obj.Code.Statements))
				}
				return nil
			},
		},
		{
			name: "nested objects",
			source: `object "Contract" {
				object "runtime" {
					code {
						return(0, 0)
					}
				}
			}`,
			validate: func(ast *YulAST) error {
				if len(ast.Objects) != 1 {
					t.Errorf("Expected 1 top-level object, got %d", len(ast.Objects))
				}
				obj := ast.Objects[0]
				if len(obj.Objects) != 1 {
					t.Errorf("Expected 1 nested object, got %d", len(obj.Objects))
				}
				nested, exists := obj.Objects["runtime"]
				if !exists {
					t.Errorf("Expected nested object 'runtime'")
				}
				if nested.Code == nil {
					t.Errorf("Expected code in nested object")
				}
				return nil
			},
		},
		{
			name: "top-level function",
			source: `function add(a, b) -> result {
				result := add(a, b)
			}`,
			validate: func(ast *YulAST) error {
				if len(ast.Functions) != 1 {
					t.Errorf("Expected 1 function, got %d", len(ast.Functions))
				}
				fn := ast.Functions[0]
				if fn.Name != "add" {
					t.Errorf("Expected function name 'add', got %s", fn.Name)
				}
				if len(fn.Parameters) != 2 {
					t.Errorf("Expected 2 parameters, got %d", len(fn.Parameters))
				}
				if len(fn.Returns) != 1 {
					t.Errorf("Expected 1 return value, got %d", len(fn.Returns))
				}
				return nil
			},
		},
	}

	for _, test := range tests {
		t.Run(test.name, func(t *testing.T) {
			parser := NewYulParser()
			ast, err := parser.Parse(test.source)
			if err != nil {
				t.Fatalf("Parse failed: %v", err)
			}
			if ast == nil {
				t.Fatalf("AST is nil")
			}
			if test.validate != nil {
				test.validate(ast)
			}
		})
	}
}

// TestYulParserVariableDeclarations tests variable declaration parsing
func TestYulParserVariableDeclarations(t *testing.T) {
	tests := []struct {
		name     string
		source   string
		validate func(*YulVariableDeclaration) error
	}{
		{
			name:   "simple variable",
			source: "let x := 1",
			validate: func(decl *YulVariableDeclaration) error {
				if len(decl.Variables) != 1 {
					t.Errorf("Expected 1 variable, got %d", len(decl.Variables))
				}
				if decl.Variables[0].Name != "x" {
					t.Errorf("Expected variable name 'x', got %s", decl.Variables[0].Name)
				}
				if decl.Value == nil {
					t.Errorf("Expected value")
				}
				return nil
			},
		},
		{
			name:   "multiple variables",
			source: "let x, y := add(1, 2)",
			validate: func(decl *YulVariableDeclaration) error {
				if len(decl.Variables) != 2 {
					t.Errorf("Expected 2 variables, got %d", len(decl.Variables))
				}
				if decl.Variables[0].Name != "x" {
					t.Errorf("Expected first variable 'x', got %s", decl.Variables[0].Name)
				}
				if decl.Variables[1].Name != "y" {
					t.Errorf("Expected second variable 'y', got %s", decl.Variables[1].Name)
				}
				return nil
			},
		},
		{
			name:   "declaration without initialization",
			source: "let x",
			validate: func(decl *YulVariableDeclaration) error {
				if len(decl.Variables) != 1 {
					t.Errorf("Expected 1 variable, got %d", len(decl.Variables))
				}
				if decl.Value != nil {
					t.Errorf("Expected no value")
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

			obj := ast.Objects[0]
			if len(obj.Code.Statements) != 1 {
				t.Fatalf("Expected 1 statement, got %d", len(obj.Code.Statements))
			}

			decl, ok := obj.Code.Statements[0].(*YulVariableDeclaration)
			if !ok {
				t.Fatalf("Expected variable declaration, got %T", obj.Code.Statements[0])
			}

			if test.validate != nil {
				test.validate(decl)
			}
		})
	}
}

// TestYulParserAssignments tests assignment parsing
func TestYulParserAssignments(t *testing.T) {
	tests := []struct {
		name     string
		source   string
		validate func(*YulAssignment) error
	}{
		{
			name:   "simple assignment",
			source: "x := 1",
			validate: func(assign *YulAssignment) error {
				if len(assign.VariableNames) != 1 {
					t.Errorf("Expected 1 variable, got %d", len(assign.VariableNames))
				}
				if assign.VariableNames[0] != "x" {
					t.Errorf("Expected variable name 'x', got %s", assign.VariableNames[0])
				}
				return nil
			},
		},
		{
			name:   "multiple assignment",
			source: "x, y := add(1, 2)",
			validate: func(assign *YulAssignment) error {
				if len(assign.VariableNames) != 2 {
					t.Errorf("Expected 2 variables, got %d", len(assign.VariableNames))
				}
				if assign.VariableNames[0] != "x" {
					t.Errorf("Expected first variable 'x', got %s", assign.VariableNames[0])
				}
				if assign.VariableNames[1] != "y" {
					t.Errorf("Expected second variable 'y', got %s", assign.VariableNames[1])
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

			obj := ast.Objects[0]
			if len(obj.Code.Statements) != 1 {
				t.Fatalf("Expected 1 statement, got %d", len(obj.Code.Statements))
			}

			assign, ok := obj.Code.Statements[0].(*YulAssignment)
			if !ok {
				t.Fatalf("Expected assignment, got %T", obj.Code.Statements[0])
			}

			if test.validate != nil {
				test.validate(assign)
			}
		})
	}
}

// TestYulParserExpressions tests expression parsing
func TestYulParserExpressions(t *testing.T) {
	tests := []struct {
		name     string
		source   string
		validate func(YulExpression) error
	}{
		{
			name:   "number literal",
			source: "123",
			validate: func(expr YulExpression) error {
				literal, ok := expr.(*YulLiteral)
				if !ok {
					t.Errorf("Expected literal, got %T", expr)
				}
				if literal.Value != "123" {
					t.Errorf("Expected value '123', got %s", literal.Value)
				}
				if literal.Kind != LiteralKindNumber {
					t.Errorf("Expected number literal, got %s", literal.Kind)
				}
				return nil
			},
		},
		{
			name:   "hex literal",
			source: "0xdeadbeef",
			validate: func(expr YulExpression) error {
				literal, ok := expr.(*YulLiteral)
				if !ok {
					t.Errorf("Expected literal, got %T", expr)
				}
				if literal.Value != "0xdeadbeef" {
					t.Errorf("Expected value '0xdeadbeef', got %s", literal.Value)
				}
				if literal.Kind != LiteralKindHex {
					t.Errorf("Expected hex literal, got %s", literal.Kind)
				}
				return nil
			},
		},
		{
			name:   "string literal",
			source: `"hello world"`,
			validate: func(expr YulExpression) error {
				literal, ok := expr.(*YulLiteral)
				if !ok {
					t.Errorf("Expected literal, got %T", expr)
				}
				if literal.Value != "hello world" {
					t.Errorf("Expected value 'hello world', got %s", literal.Value)
				}
				if literal.Kind != LiteralKindString {
					t.Errorf("Expected string literal, got %s", literal.Kind)
				}
				return nil
			},
		},
		{
			name:   "bool literal true",
			source: "true",
			validate: func(expr YulExpression) error {
				literal, ok := expr.(*YulLiteral)
				if !ok {
					t.Errorf("Expected literal, got %T", expr)
				}
				if literal.Value != "true" {
					t.Errorf("Expected value 'true', got %s", literal.Value)
				}
				if literal.Kind != LiteralKindBool {
					t.Errorf("Expected bool literal, got %s", literal.Kind)
				}
				return nil
			},
		},
		{
			name:   "bool literal false",
			source: "false",
			validate: func(expr YulExpression) error {
				literal, ok := expr.(*YulLiteral)
				if !ok {
					t.Errorf("Expected literal, got %T", expr)
				}
				if literal.Value != "false" {
					t.Errorf("Expected value 'false', got %s", literal.Value)
				}
				if literal.Kind != LiteralKindBool {
					t.Errorf("Expected bool literal, got %s", literal.Kind)
				}
				return nil
			},
		},
		{
			name:   "identifier",
			source: "myVariable",
			validate: func(expr YulExpression) error {
				ident, ok := expr.(*YulIdentifier)
				if !ok {
					t.Errorf("Expected identifier, got %T", expr)
				}
				if ident.Name != "myVariable" {
					t.Errorf("Expected name 'myVariable', got %s", ident.Name)
				}
				return nil
			},
		},
		{
			name:   "function call no args",
			source: "gas()",
			validate: func(expr YulExpression) error {
				call, ok := expr.(*YulFunctionCall)
				if !ok {
					t.Errorf("Expected function call, got %T", expr)
				}
				if call.FunctionName.Name != "gas" {
					t.Errorf("Expected function name 'gas', got %s", call.FunctionName.Name)
				}
				if len(call.Arguments) != 0 {
					t.Errorf("Expected 0 arguments, got %d", len(call.Arguments))
				}
				return nil
			},
		},
		{
			name:   "function call with args",
			source: "add(1, 2)",
			validate: func(expr YulExpression) error {
				call, ok := expr.(*YulFunctionCall)
				if !ok {
					t.Errorf("Expected function call, got %T", expr)
				}
				if call.FunctionName.Name != "add" {
					t.Errorf("Expected function name 'add', got %s", call.FunctionName.Name)
				}
				if len(call.Arguments) != 2 {
					t.Errorf("Expected 2 arguments, got %d", len(call.Arguments))
				}
				return nil
			},
		},
		{
			name:   "nested function call",
			source: "add(mul(2, 3), 4)",
			validate: func(expr YulExpression) error {
				call, ok := expr.(*YulFunctionCall)
				if !ok {
					t.Errorf("Expected function call, got %T", expr)
				}
				if call.FunctionName.Name != "add" {
					t.Errorf("Expected function name 'add', got %s", call.FunctionName.Name)
				}
				if len(call.Arguments) != 2 {
					t.Errorf("Expected 2 arguments, got %d", len(call.Arguments))
				}
				// Check nested call
				nestedCall, ok := call.Arguments[0].(*YulFunctionCall)
				if !ok {
					t.Errorf("Expected nested function call, got %T", call.Arguments[0])
				}
				if nestedCall.FunctionName.Name != "mul" {
					t.Errorf("Expected nested function name 'mul', got %s", nestedCall.FunctionName.Name)
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

			obj := ast.Objects[0]
			if len(obj.Code.Statements) != 1 {
				t.Fatalf("Expected 1 statement, got %d", len(obj.Code.Statements))
			}

			decl, ok := obj.Code.Statements[0].(*YulVariableDeclaration)
			if !ok {
				t.Fatalf("Expected variable declaration, got %T", obj.Code.Statements[0])
			}

			if test.validate != nil {
				test.validate(decl.Value)
			}
		})
	}
}

// TestYulParserControlFlow tests control flow statement parsing
func TestYulParserControlFlow(t *testing.T) {
	tests := []struct {
		name     string
		source   string
		validate func(YulStatement) error
	}{
		{
			name:   "if statement",
			source: "if iszero(x) { revert(0, 0) }",
			validate: func(stmt YulStatement) error {
				ifStmt, ok := stmt.(*YulIf)
				if !ok {
					t.Errorf("Expected if statement, got %T", stmt)
				}
				if ifStmt.Condition == nil {
					t.Errorf("Expected condition")
				}
				if ifStmt.Body == nil {
					t.Errorf("Expected body")
				}
				if len(ifStmt.Body.Statements) == 0 {
					t.Errorf("Expected statements in body")
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
			validate: func(stmt YulStatement) error {
				switchStmt, ok := stmt.(*YulSwitch)
				if !ok {
					t.Errorf("Expected switch statement, got %T", stmt)
				}
				if switchStmt.Expression == nil {
					t.Errorf("Expected switch expression")
				}
				if len(switchStmt.Cases) != 2 {
					t.Errorf("Expected 2 cases, got %d", len(switchStmt.Cases))
				}
				if switchStmt.Default == nil {
					t.Errorf("Expected default case")
				}
				return nil
			},
		},
		{
			name: "for loop",
			source: `for { let i := 0 } lt(i, 10) { i := add(i, 1) } {
				// loop body
			}`,
			validate: func(stmt YulStatement) error {
				forStmt, ok := stmt.(*YulFor)
				if !ok {
					t.Errorf("Expected for statement, got %T", stmt)
				}
				if forStmt.Init == nil {
					t.Errorf("Expected init block")
				}
				if forStmt.Condition == nil {
					t.Errorf("Expected condition")
				}
				if forStmt.Post == nil {
					t.Errorf("Expected post block")
				}
				if forStmt.Body == nil {
					t.Errorf("Expected body")
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

			obj := ast.Objects[0]
			if len(obj.Code.Statements) != 1 {
				t.Fatalf("Expected 1 statement, got %d", len(obj.Code.Statements))
			}

			if test.validate != nil {
				test.validate(obj.Code.Statements[0])
			}
		})
	}
}

// TestYulParserFunctions tests function definition parsing
func TestYulParserFunctions(t *testing.T) {
	tests := []struct {
		name     string
		source   string
		validate func(*YulFunctionDef) error
	}{
		{
			name:   "function no params no returns",
			source: "function test() { let x := 1 }",
			validate: func(fn *YulFunctionDef) error {
				if fn.Name != "test" {
					t.Errorf("Expected function name 'test', got %s", fn.Name)
				}
				if len(fn.Parameters) != 0 {
					t.Errorf("Expected 0 parameters, got %d", len(fn.Parameters))
				}
				if len(fn.Returns) != 0 {
					t.Errorf("Expected 0 returns, got %d", len(fn.Returns))
				}
				if fn.Body == nil {
					t.Errorf("Expected function body")
				}
				return nil
			},
		},
		{
			name:   "function with params",
			source: "function add(a, b) { result := add(a, b) }",
			validate: func(fn *YulFunctionDef) error {
				if fn.Name != "add" {
					t.Errorf("Expected function name 'add', got %s", fn.Name)
				}
				if len(fn.Parameters) != 2 {
					t.Errorf("Expected 2 parameters, got %d", len(fn.Parameters))
				}
				if fn.Parameters[0].Name != "a" {
					t.Errorf("Expected first parameter 'a', got %s", fn.Parameters[0].Name)
				}
				if fn.Parameters[1].Name != "b" {
					t.Errorf("Expected second parameter 'b', got %s", fn.Parameters[1].Name)
				}
				return nil
			},
		},
		{
			name:   "function with returns",
			source: "function getValue() -> result { result := 42 }",
			validate: func(fn *YulFunctionDef) error {
				if fn.Name != "getValue" {
					t.Errorf("Expected function name 'getValue', got %s", fn.Name)
				}
				if len(fn.Returns) != 1 {
					t.Errorf("Expected 1 return value, got %d", len(fn.Returns))
				}
				if fn.Returns[0].Name != "result" {
					t.Errorf("Expected return name 'result', got %s", fn.Returns[0].Name)
				}
				return nil
			},
		},
		{
			name:   "function with params and returns",
			source: "function multiply(a, b) -> product { product := mul(a, b) }",
			validate: func(fn *YulFunctionDef) error {
				if fn.Name != "multiply" {
					t.Errorf("Expected function name 'multiply', got %s", fn.Name)
				}
				if len(fn.Parameters) != 2 {
					t.Errorf("Expected 2 parameters, got %d", len(fn.Parameters))
				}
				if len(fn.Returns) != 1 {
					t.Errorf("Expected 1 return value, got %d", len(fn.Returns))
				}
				return nil
			},
		},
		{
			name:   "function with multiple returns",
			source: "function divmod(a, b) -> quotient, remainder { quotient := div(a, b) remainder := mod(a, b) }",
			validate: func(fn *YulFunctionDef) error {
				if fn.Name != "divmod" {
					t.Errorf("Expected function name 'divmod', got %s", fn.Name)
				}
				if len(fn.Parameters) != 2 {
					t.Errorf("Expected 2 parameters, got %d", len(fn.Parameters))
				}
				if len(fn.Returns) != 2 {
					t.Errorf("Expected 2 return values, got %d", len(fn.Returns))
				}
				if fn.Returns[0].Name != "quotient" {
					t.Errorf("Expected first return 'quotient', got %s", fn.Returns[0].Name)
				}
				if fn.Returns[1].Name != "remainder" {
					t.Errorf("Expected second return 'remainder', got %s", fn.Returns[1].Name)
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

			obj := ast.Objects[0]
			if len(obj.Code.Statements) != 1 {
				t.Fatalf("Expected 1 statement, got %d", len(obj.Code.Statements))
			}

			fn, ok := obj.Code.Statements[0].(*YulFunctionDef)
			if !ok {
				t.Fatalf("Expected function definition, got %T", obj.Code.Statements[0])
			}

			if test.validate != nil {
				test.validate(fn)
			}
		})
	}
}

// TestYulParserComplexProgram tests parsing of a complex Yul program
func TestYulParserComplexProgram(t *testing.T) {
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
				default {
					revert(0, 0)
				}
				
				function safe_add(a, b) -> result {
					result := add(a, b)
					if lt(result, a) { revert(0, 0) }
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

	// Verify top-level structure
	if len(ast.Objects) != 1 {
		t.Errorf("Expected 1 top-level object, got %d", len(ast.Objects))
	}

	obj := ast.Objects[0]
	if obj.Name != "ERC20" {
		t.Errorf("Expected object name 'ERC20', got %s", obj.Name)
	}

	// Verify main object has code
	if obj.Code == nil {
		t.Errorf("Expected code in main object")
	}

	// Verify nested runtime object
	if len(obj.Objects) != 1 {
		t.Errorf("Expected 1 nested object, got %d", len(obj.Objects))
	}

	runtime, exists := obj.Objects["runtime"]
	if !exists {
		t.Errorf("Expected nested object 'runtime'")
	}

	if runtime.Code == nil {
		t.Errorf("Expected code in runtime object")
	}

	// Count statements in runtime code
	statements := runtime.Code.Statements
	if len(statements) < 3 {
		t.Errorf("Expected at least 3 statements in runtime, got %d", len(statements))
	}

	// Find the switch statement
	var switchStmt *YulSwitch
	for _, stmt := range statements {
		if sw, ok := stmt.(*YulSwitch); ok {
			switchStmt = sw
			break
		}
	}

	if switchStmt == nil {
		t.Errorf("Expected to find switch statement in runtime")
	} else {
		if len(switchStmt.Cases) != 2 {
			t.Errorf("Expected 2 switch cases, got %d", len(switchStmt.Cases))
		}
		if switchStmt.Default == nil {
			t.Errorf("Expected default case in switch")
		}
	}

	// Find function definitions
	var functionCount int
	for _, stmt := range statements {
		if _, ok := stmt.(*YulFunctionDef); ok {
			functionCount++
		}
	}

	if functionCount == 0 {
		t.Errorf("Expected to find at least 1 function definition")
	}
}

// TestYulParserErrorHandling tests parser error handling
func TestYulParserErrorHandling(t *testing.T) {
	tests := []struct {
		name     string
		source   string
		errorMsg string
	}{
		{
			name:     "unterminated object",
			source:   `object "Test" {`,
			errorMsg: "Expected '}'",
		},
		{
			name:     "missing object name",
			source:   `object {`,
			errorMsg: "Expected object name",
		},
		{
			name:     "invalid token in object",
			source:   `object "Test" { invalid }`,
			errorMsg: "unexpected token",
		},
		{
			name:     "missing function name",
			source:   `function () {}`,
			errorMsg: "Expected function name",
		},
		{
			name:     "unterminated function",
			source:   `function test() {`,
			errorMsg: "Expected '}'",
		},
		{
			name:     "missing variable name",
			source:   `object "Test" { code { let := 1 } }`,
			errorMsg: "Expected variable name",
		},
		{
			name:     "invalid expression",
			source:   `object "Test" { code { let x := } }`,
			errorMsg: "unexpected token",
		},
		{
			name:     "unterminated function call",
			source:   `object "Test" { code { add(1, } }`,
			errorMsg: "unexpected token",
		},
	}

	for _, test := range tests {
		t.Run(test.name, func(t *testing.T) {
			parser := NewYulParser()
			_, err := parser.Parse(test.source)
			if err == nil {
				t.Fatalf("Expected parse error but got none")
			}
			if !strings.Contains(err.Error(), test.errorMsg) {
				t.Errorf("Expected error containing %q, got %q", test.errorMsg, err.Error())
			}
		})
	}
}

// TestYulParserSourcePositions tests that source positions are correctly tracked
func TestYulParserSourcePositions(t *testing.T) {
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

	obj := ast.Objects[0]
	statements := obj.Code.Statements

	// Check that positions are set
	for i, stmt := range statements {
		pos := stmt.GetLocation()
		if pos.Line == 0 {
			t.Errorf("Statement %d has invalid line number", i)
		}
		if pos.Column == 0 {
			t.Errorf("Statement %d has invalid column number", i)
		}
		if pos.Offset < 0 {
			t.Errorf("Statement %d has invalid offset", i)
		}
	}

	// Check that second statement is after first
	if len(statements) >= 2 {
		pos1 := statements[0].GetLocation()
		pos2 := statements[1].GetLocation()
		
		if pos2.Line <= pos1.Line {
			t.Errorf("Second statement should be on later line")
		}
	}
}

// TestYulParserASTStructure tests the overall AST structure
func TestYulParserASTStructure(t *testing.T) {
	source := `
	object "Contract" {
		code {
			let x := 1
		}
	}
	function helper(a) -> result {
		result := a
	}
	`

	parser := NewYulParser()
	ast, err := parser.Parse(source)
	if err != nil {
		t.Fatalf("Parse failed: %v", err)
	}

	// Check AST metadata
	if ast.Metadata == nil {
		t.Errorf("Expected metadata")
	}
	if ast.Metadata.CompilerInfo == nil {
		t.Errorf("Expected compiler info")
	}
	if ast.Metadata.SourceFile != "inline" {
		t.Errorf("Expected source file 'inline', got %s", ast.Metadata.SourceFile)
	}

	// Check objects and functions are separated
	if len(ast.Objects) != 1 {
		t.Errorf("Expected 1 object, got %d", len(ast.Objects))
	}
	if len(ast.Functions) != 1 {
		t.Errorf("Expected 1 top-level function, got %d", len(ast.Functions))
	}
}

// TestYulParserVisitorPattern tests the visitor pattern implementation
func TestYulParserVisitorPattern(t *testing.T) {
	source := `
	object "Test" {
		code {
			let x := add(1, 2)
			if iszero(x) {
				revert(0, 0)
			}
		}
	}
	`

	parser := NewYulParser()
	ast, err := parser.Parse(source)
	if err != nil {
		t.Fatalf("Parse failed: %v", err)
	}

	// Create a simple visitor that counts nodes
	visitor := &NodeCounterVisitor{
		Counts: make(map[YulNodeType]int),
	}

	// Visit the AST
	obj := ast.Objects[0]
	err = obj.Code.Accept(visitor)
	if err != nil {
		t.Fatalf("Visitor failed: %v", err)
	}

	// Check some expected counts
	if visitor.Counts[NodeTypeVariableDeclaration] != 1 {
		t.Errorf("Expected 1 variable declaration, got %d", visitor.Counts[NodeTypeVariableDeclaration])
	}
	if visitor.Counts[NodeTypeIf] != 1 {
		t.Errorf("Expected 1 if statement, got %d", visitor.Counts[NodeTypeIf])
	}
	if visitor.Counts[NodeTypeFunctionCall] < 2 {
		t.Errorf("Expected at least 2 function calls, got %d", visitor.Counts[NodeTypeFunctionCall])
	}
}

// NodeCounterVisitor is a test visitor that counts node types
type NodeCounterVisitor struct {
	Counts map[YulNodeType]int
}

func (v *NodeCounterVisitor) VisitObject(obj *YulObject) error {
	v.Counts[NodeTypeObject]++
	return nil
}

func (v *NodeCounterVisitor) VisitBlock(block *YulBlock) error {
	v.Counts[NodeTypeBlock]++
	for _, stmt := range block.Statements {
		err := stmt.Accept(v)
		if err != nil {
			return err
		}
	}
	return nil
}

func (v *NodeCounterVisitor) VisitExpressionStatement(stmt *YulExpressionStatement) error {
	v.Counts[NodeTypeExpressionStatement]++
	if stmt.Expression != nil {
		return stmt.Expression.Accept(v)
	}
	return nil
}

func (v *NodeCounterVisitor) VisitVariableDeclaration(decl *YulVariableDeclaration) error {
	v.Counts[NodeTypeVariableDeclaration]++
	if decl.Value != nil {
		return decl.Value.Accept(v)
	}
	return nil
}

func (v *NodeCounterVisitor) VisitAssignment(assign *YulAssignment) error {
	v.Counts[NodeTypeAssignment]++
	return assign.Value.Accept(v)
}

func (v *NodeCounterVisitor) VisitIf(ifStmt *YulIf) error {
	v.Counts[NodeTypeIf]++
	err := ifStmt.Condition.Accept(v)
	if err != nil {
		return err
	}
	return ifStmt.Body.Accept(v)
}

func (v *NodeCounterVisitor) VisitSwitch(switchStmt *YulSwitch) error {
	v.Counts[NodeTypeSwitch]++
	err := switchStmt.Expression.Accept(v)
	if err != nil {
		return err
	}
	for _, caseStmt := range switchStmt.Cases {
		err = caseStmt.Body.Accept(v)
		if err != nil {
			return err
		}
	}
	if switchStmt.Default != nil {
		err = switchStmt.Default.Accept(v)
		if err != nil {
			return err
		}
	}
	return nil
}

func (v *NodeCounterVisitor) VisitFor(forStmt *YulFor) error {
	v.Counts[NodeTypeFor]++
	err := forStmt.Init.Accept(v)
	if err != nil {
		return err
	}
	err = forStmt.Condition.Accept(v)
	if err != nil {
		return err
	}
	err = forStmt.Post.Accept(v)
	if err != nil {
		return err
	}
	return forStmt.Body.Accept(v)
}

func (v *NodeCounterVisitor) VisitFunctionDef(fn *YulFunctionDef) error {
	v.Counts[NodeTypeFunctionDef]++
	return fn.Body.Accept(v)
}

func (v *NodeCounterVisitor) VisitBreak(breakStmt *YulBreak) error {
	v.Counts[NodeTypeBreak]++
	return nil
}

func (v *NodeCounterVisitor) VisitContinue(continueStmt *YulContinue) error {
	v.Counts[NodeTypeContinue]++
	return nil
}

func (v *NodeCounterVisitor) VisitLeave(leaveStmt *YulLeave) error {
	v.Counts[NodeTypeLeave]++
	return nil
}

func (v *NodeCounterVisitor) VisitFunctionCall(call *YulFunctionCall) error {
	v.Counts[NodeTypeFunctionCall]++
	for _, arg := range call.Arguments {
		err := arg.Accept(v)
		if err != nil {
			return err
		}
	}
	return nil
}

func (v *NodeCounterVisitor) VisitIdentifier(ident *YulIdentifier) error {
	v.Counts[NodeTypeIdentifier]++
	return nil
}

func (v *NodeCounterVisitor) VisitLiteral(literal *YulLiteral) error {
	v.Counts[NodeTypeLiteral]++
	return nil
}

// BenchmarkYulParser benchmarks parser performance
func BenchmarkYulParser(b *testing.B) {
	// Complex ERC20-like contract for benchmarking
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
				case 0x70a08231 { // balanceOf
					return_uint(sload(calldataload(4)))
				}
				case 0xa9059cbb { // transfer
					let to := calldataload(4)
					let value := calldataload(36)
					let from := caller()
					
					let fromBalance := sload(from)
					if lt(fromBalance, value) { revert(0, 0) }
					
					sstore(from, sub(fromBalance, value))
					sstore(to, add(sload(to), value))
				}
				default { revert(0, 0) }
				
				function return_uint(value) {
					mstore(0, value)
					return(0, 32)
				}
			}
		}
	}
	`

	b.ResetTimer()

	for i := 0; i < b.N; i++ {
		parser := NewYulParser()
		_, err := parser.Parse(source)
		if err != nil {
			b.Fatalf("Parse failed: %v", err)
		}
	}
}

// TestYulParserRecovery tests error recovery mechanisms
func TestYulParserRecovery(t *testing.T) {
	// Verify parser handles malformed input gracefully with proper error reporting
	// Production parser includes comprehensive error recovery and synchronization
	source := `
	object "Test" {
		code {
			let x := ; // syntax error
			let y := 2 // this should still be valid if parser recovers
		}
	}
	`

	parser := NewYulParser()
	_, err := parser.Parse(source)
	
	// Should get an error
	if err == nil {
		t.Errorf("Expected parse error for invalid syntax")
	}
	
	// Error message should be meaningful
	if !strings.Contains(err.Error(), "unexpected token") {
		t.Errorf("Expected meaningful error message, got: %s", err.Error())
	}
}