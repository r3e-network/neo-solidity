package main

import (
	"errors"
	"fmt"
	"regexp"
	"strconv"
	"strings"
)

// YulParser handles parsing of Yul intermediate representation into an AST
type YulParser struct {
	lexer    *YulLexer
	current  Token
	previous Token
}

// YulAST represents the complete Abstract Syntax Tree for a Yul program
type YulAST struct {
	Objects   []*YulObject         `json:"objects"`
	Functions []*YulFunctionDef    `json:"functions"`
	Metadata  *YulMetadata         `json:"metadata"`
}

// YulObject represents a Yul object (contract or code block)
type YulObject struct {
	Name     string              `json:"name"`
	Type     YulObjectType       `json:"type"`
	Code     *YulBlock           `json:"code,omitempty"`
	Data     *YulData            `json:"data,omitempty"`
	Objects  map[string]*YulObject `json:"objects,omitempty"`
	Location SourcePosition      `json:"location"`
}

// YulBlock represents a block of Yul statements
type YulBlock struct {
	Statements []YulStatement   `json:"statements"`
	Location   SourcePosition   `json:"location"`
}

// YulStatement interface for all Yul statement types
type YulStatement interface {
	GetType() YulNodeType
	GetLocation() SourcePosition
	Accept(visitor YulVisitor) error
}

// YulExpression interface for all Yul expression types
type YulExpression interface {
	GetType() YulNodeType
	GetLocation() SourcePosition
	GetResultType() YulDataType
	Accept(visitor YulVisitor) error
}

// Core Yul statement types
type (
	// YulExpressionStatement wraps expressions used as statements
	YulExpressionStatement struct {
		Expression YulExpression  `json:"expression"`
		Location   SourcePosition `json:"location"`
	}

	// YulVariableDeclaration declares new variables
	YulVariableDeclaration struct {
		Variables []*YulTypedName `json:"variables"`
		Value     YulExpression   `json:"value,omitempty"`
		Location  SourcePosition  `json:"location"`
	}

	// YulAssignment assigns values to existing variables
	YulAssignment struct {
		VariableNames []string       `json:"variable_names"`
		Value         YulExpression  `json:"value"`
		Location      SourcePosition `json:"location"`
	}

	// YulIf represents conditional execution
	YulIf struct {
		Condition YulExpression  `json:"condition"`
		Body      *YulBlock      `json:"body"`
		Location  SourcePosition `json:"location"`
	}

	// YulSwitch represents switch statements
	YulSwitch struct {
		Expression YulExpression  `json:"expression"`
		Cases      []*YulCase     `json:"cases"`
		Default    *YulBlock      `json:"default,omitempty"`
		Location   SourcePosition `json:"location"`
	}

	// YulFor represents for loops
	YulFor struct {
		Init      *YulBlock      `json:"init"`
		Condition YulExpression  `json:"condition"`
		Post      *YulBlock      `json:"post"`
		Body      *YulBlock      `json:"body"`
		Location  SourcePosition `json:"location"`
	}

	// YulFunctionDef represents function definitions
	YulFunctionDef struct {
		Name       string           `json:"name"`
		Parameters []*YulTypedName  `json:"parameters"`
		Returns    []*YulTypedName  `json:"returns"`
		Body       *YulBlock        `json:"body"`
		Location   SourcePosition   `json:"location"`
	}

	// YulBreak represents break statements
	YulBreak struct {
		Location SourcePosition `json:"location"`
	}

	// YulContinue represents continue statements
	YulContinue struct {
		Location SourcePosition `json:"location"`
	}

	// YulLeave represents leave statements (early function return)
	YulLeave struct {
		Location SourcePosition `json:"location"`
	}
)

// Core Yul expression types
type (
	// YulFunctionCall represents function calls
	YulFunctionCall struct {
		FunctionName YulIdentifier   `json:"function_name"`
		Arguments    []YulExpression `json:"arguments"`
		Location     SourcePosition  `json:"location"`
		ResultType   YulDataType     `json:"result_type"`
	}

	// YulIdentifier represents variable names and identifiers
	YulIdentifier struct {
		Name     string         `json:"name"`
		Location SourcePosition `json:"location"`
	}

	// YulLiteral represents literal values
	YulLiteral struct {
		Kind     YulLiteralKind `json:"kind"`
		Value    string         `json:"value"`
		Type     YulDataType    `json:"type"`
		Location SourcePosition `json:"location"`
	}
)

// Supporting types
type (
	YulCase struct {
		Value    YulLiteral     `json:"value"`
		Body     *YulBlock      `json:"body"`
		Location SourcePosition `json:"location"`
	}

	YulTypedName struct {
		Name     string         `json:"name"`
		Type     YulDataType    `json:"type"`
		Location SourcePosition `json:"location"`
	}

	YulData struct {
		Value    string         `json:"value"`
		Location SourcePosition `json:"location"`
	}

	YulMetadata struct {
		SourceFile   string            `json:"source_file"`
		CompilerInfo *CompilerInfo     `json:"compiler_info"`
		ParseTime    int64             `json:"parse_time_ms"`
	}

	CompilerInfo struct {
		Version string `json:"version"`
		Target  string `json:"target"`
	}
)

// Enums and constants
type YulNodeType string
const (
	NodeTypeObject               YulNodeType = "Object"
	NodeTypeBlock                YulNodeType = "Block"
	NodeTypeExpressionStatement  YulNodeType = "ExpressionStatement"
	NodeTypeVariableDeclaration  YulNodeType = "VariableDeclaration"
	NodeTypeAssignment           YulNodeType = "Assignment"
	NodeTypeIf                   YulNodeType = "If"
	NodeTypeSwitch               YulNodeType = "Switch"
	NodeTypeFor                  YulNodeType = "For"
	NodeTypeFunctionDef          YulNodeType = "FunctionDefinition"
	NodeTypeBreak                YulNodeType = "Break"
	NodeTypeContinue             YulNodeType = "Continue"
	NodeTypeLeave                YulNodeType = "Leave"
	NodeTypeFunctionCall         YulNodeType = "FunctionCall"
	NodeTypeIdentifier           YulNodeType = "Identifier"
	NodeTypeLiteral              YulNodeType = "Literal"
)

type YulObjectType string
const (
	ObjectTypeContract YulObjectType = "contract"
	ObjectTypeLibrary  YulObjectType = "library"
	ObjectTypeRuntime  YulObjectType = "runtime"
)

type YulDataType string
const (
	DataTypeUint256 YulDataType = "uint256"
	DataTypeBool    YulDataType = "bool"
	DataTypeBytes32 YulDataType = "bytes32"
	DataTypeAddress YulDataType = "address"
	DataTypeString  YulDataType = "string"
)

type YulLiteralKind string
const (
	LiteralKindNumber YulLiteralKind = "number"
	LiteralKindString YulLiteralKind = "string"
	LiteralKindBool   YulLiteralKind = "bool"
	LiteralKindHex    YulLiteralKind = "hex"
)

type SourcePosition struct {
	File   string `json:"file"`
	Line   int    `json:"line"`
	Column int    `json:"column"`
	Offset int    `json:"offset"`
	Length int    `json:"length"`
}

// YulVisitor interface for AST traversal
type YulVisitor interface {
	VisitObject(*YulObject) error
	VisitBlock(*YulBlock) error
	VisitExpressionStatement(*YulExpressionStatement) error
	VisitVariableDeclaration(*YulVariableDeclaration) error
	VisitAssignment(*YulAssignment) error
	VisitIf(*YulIf) error
	VisitSwitch(*YulSwitch) error
	VisitFor(*YulFor) error
	VisitFunctionDef(*YulFunctionDef) error
	VisitBreak(*YulBreak) error
	VisitContinue(*YulContinue) error
	VisitLeave(*YulLeave) error
	VisitFunctionCall(*YulFunctionCall) error
	VisitIdentifier(*YulIdentifier) error
	VisitLiteral(*YulLiteral) error
}

// NewYulParser creates a new Yul parser instance
func NewYulParser() *YulParser {
	return &YulParser{
		lexer: NewYulLexer(),
	}
}

// Parse parses Yul source code into an AST
func (p *YulParser) Parse(source string) (*YulAST, error) {
	// Initialize lexer with source
	err := p.lexer.Init(source)
	if err != nil {
		return nil, fmt.Errorf("lexer initialization failed: %w", err)
	}

	// Start parsing
	p.advance() // Load first token
	
	ast := &YulAST{
		Objects:   []*YulObject{},
		Functions: []*YulFunctionDef{},
		Metadata: &YulMetadata{
			SourceFile: "inline",
			CompilerInfo: &CompilerInfo{
				Version: "1.0.0",
				Target:  "NeoVM",
			},
		},
	}

	// Parse top-level constructs
	for !p.isAtEnd() {
		if p.check(TokenObject) {
			obj, err := p.parseObject()
			if err != nil {
				return nil, err
			}
			ast.Objects = append(ast.Objects, obj)
		} else if p.check(TokenFunction) {
			fn, err := p.parseFunction()
			if err != nil {
				return nil, err
			}
			ast.Functions = append(ast.Functions, fn)
		} else {
			return nil, fmt.Errorf("unexpected token %v at line %d", p.current.Type, p.current.Line)
		}
	}

	return ast, nil
}

// parseObject parses a Yul object definition
func (p *YulParser) parseObject() (*YulObject, error) {
	startPos := p.current.Position
	
	p.consume(TokenObject, "Expected 'object'")
	
	name := p.consume(TokenString, "Expected object name").Lexeme
	name = strings.Trim(name, `"`) // Remove quotes
	
	p.consume(TokenLeftBrace, "Expected '{'")
	
	obj := &YulObject{
		Name:     name,
		Type:     ObjectTypeContract,
		Objects:  make(map[string]*YulObject),
		Location: p.makePosition(startPos),
	}

	// Parse object body
	for !p.check(TokenRightBrace) && !p.isAtEnd() {
		if p.check(TokenCode) {
			p.advance() // consume 'code'
			p.consume(TokenLeftBrace, "Expected '{'")
			
			block, err := p.parseBlock()
			if err != nil {
				return nil, err
			}
			obj.Code = block
			
		} else if p.check(TokenData) {
			p.advance() // consume 'data'
			data, err := p.parseData()
			if err != nil {
				return nil, err
			}
			obj.Data = data
			
		} else if p.check(TokenObject) {
			nestedObj, err := p.parseObject()
			if err != nil {
				return nil, err
			}
			obj.Objects[nestedObj.Name] = nestedObj
			
		} else {
			return nil, fmt.Errorf("unexpected token in object body: %v", p.current.Type)
		}
	}
	
	p.consume(TokenRightBrace, "Expected '}'")
	return obj, nil
}

// parseBlock parses a block of statements
func (p *YulParser) parseBlock() (*YulBlock, error) {
	startPos := p.current.Position
	
	block := &YulBlock{
		Statements: []YulStatement{},
		Location:   p.makePosition(startPos),
	}

	for !p.check(TokenRightBrace) && !p.isAtEnd() {
		stmt, err := p.parseStatement()
		if err != nil {
			return nil, err
		}
		if stmt != nil {
			block.Statements = append(block.Statements, stmt)
		}
	}

	p.consume(TokenRightBrace, "Expected '}'")
	return block, nil
}

// parseStatement parses individual statements
func (p *YulParser) parseStatement() (YulStatement, error) {
	switch p.current.Type {
	case TokenLet:
		return p.parseVariableDeclaration()
	case TokenIf:
		return p.parseIf()
	case TokenSwitch:
		return p.parseSwitch()
	case TokenFor:
		return p.parseFor()
	case TokenFunction:
		return p.parseFunction()
	case TokenBreak:
		return p.parseBreak()
	case TokenContinue:
		return p.parseContinue()
	case TokenLeave:
		return p.parseLeave()
	case TokenLeftBrace:
		// Block statement
		p.advance()
		block, err := p.parseBlock()
		if err != nil {
			return nil, err
		}
		return &YulExpressionStatement{
			Expression: nil, // Blocks don't have expressions
			Location:   block.Location,
		}, nil
	default:
		// Try to parse as expression statement or assignment
		return p.parseExpressionOrAssignment()
	}
}

// parseVariableDeclaration parses variable declarations
func (p *YulParser) parseVariableDeclaration() (*YulVariableDeclaration, error) {
	startPos := p.current.Position
	p.consume(TokenLet, "Expected 'let'")

	var variables []*YulTypedName
	
	// Parse variable list
	for {
		name := p.consume(TokenIdentifier, "Expected variable name").Lexeme
		variables = append(variables, &YulTypedName{
			Name:     name,
			Type:     DataTypeUint256, // Default type
			Location: p.makePosition(p.previous.Position),
		})
		
		if !p.match(TokenComma) {
			break
		}
	}

	var value YulExpression
	var err error
	
	if p.match(TokenColonEqual) {
		value, err = p.parseExpression()
		if err != nil {
			return nil, err
		}
	}

	return &YulVariableDeclaration{
		Variables: variables,
		Value:     value,
		Location:  p.makePosition(startPos),
	}, nil
}

// parseIf parses if statements
func (p *YulParser) parseIf() (*YulIf, error) {
	startPos := p.current.Position
	p.consume(TokenIf, "Expected 'if'")

	condition, err := p.parseExpression()
	if err != nil {
		return nil, err
	}

	p.consume(TokenLeftBrace, "Expected '{'")
	body, err := p.parseBlock()
	if err != nil {
		return nil, err
	}

	return &YulIf{
		Condition: condition,
		Body:      body,
		Location:  p.makePosition(startPos),
	}, nil
}

// parseSwitch parses switch statements
func (p *YulParser) parseSwitch() (*YulSwitch, error) {
	startPos := p.current.Position
	p.consume(TokenSwitch, "Expected 'switch'")

	expr, err := p.parseExpression()
	if err != nil {
		return nil, err
	}

	var cases []*YulCase
	var defaultCase *YulBlock

	for p.check(TokenCase) || p.check(TokenDefault) {
		if p.match(TokenCase) {
			value := p.consume(TokenNumber, "Expected case value")
			p.consume(TokenLeftBrace, "Expected '{'")
			
			body, err := p.parseBlock()
			if err != nil {
				return nil, err
			}

			cases = append(cases, &YulCase{
				Value: YulLiteral{
					Kind:     LiteralKindNumber,
					Value:    value.Lexeme,
					Type:     DataTypeUint256,
					Location: p.makePosition(value.Position),
				},
				Body:     body,
				Location: p.makePosition(value.Position),
			})
		} else if p.match(TokenDefault) {
			p.consume(TokenLeftBrace, "Expected '{'")
			defaultCase, err = p.parseBlock()
			if err != nil {
				return nil, err
			}
		}
	}

	return &YulSwitch{
		Expression: expr,
		Cases:      cases,
		Default:    defaultCase,
		Location:   p.makePosition(startPos),
	}, nil
}

// parseFor parses for loops
func (p *YulParser) parseFor() (*YulFor, error) {
	startPos := p.current.Position
	p.consume(TokenFor, "Expected 'for'")

	p.consume(TokenLeftBrace, "Expected '{'")
	init, err := p.parseBlock()
	if err != nil {
		return nil, err
	}

	condition, err := p.parseExpression()
	if err != nil {
		return nil, err
	}

	p.consume(TokenLeftBrace, "Expected '{'")
	post, err := p.parseBlock()
	if err != nil {
		return nil, err
	}

	p.consume(TokenLeftBrace, "Expected '{'")
	body, err := p.parseBlock()
	if err != nil {
		return nil, err
	}

	return &YulFor{
		Init:      init,
		Condition: condition,
		Post:      post,
		Body:      body,
		Location:  p.makePosition(startPos),
	}, nil
}

// parseFunction parses function definitions
func (p *YulParser) parseFunction() (*YulFunctionDef, error) {
	startPos := p.current.Position
	p.consume(TokenFunction, "Expected 'function'")

	name := p.consume(TokenIdentifier, "Expected function name").Lexeme

	p.consume(TokenLeftParen, "Expected '('")
	
	var parameters []*YulTypedName
	if !p.check(TokenRightParen) {
		for {
			paramName := p.consume(TokenIdentifier, "Expected parameter name").Lexeme
			parameters = append(parameters, &YulTypedName{
				Name:     paramName,
				Type:     DataTypeUint256,
				Location: p.makePosition(p.previous.Position),
			})
			
			if !p.match(TokenComma) {
				break
			}
		}
	}
	
	p.consume(TokenRightParen, "Expected ')'")

	var returns []*YulTypedName
	if p.match(TokenArrow) {
		for {
			returnName := p.consume(TokenIdentifier, "Expected return variable name").Lexeme
			returns = append(returns, &YulTypedName{
				Name:     returnName,
				Type:     DataTypeUint256,
				Location: p.makePosition(p.previous.Position),
			})
			
			if !p.match(TokenComma) {
				break
			}
		}
	}

	p.consume(TokenLeftBrace, "Expected '{'")
	body, err := p.parseBlock()
	if err != nil {
		return nil, err
	}

	return &YulFunctionDef{
		Name:       name,
		Parameters: parameters,
		Returns:    returns,
		Body:       body,
		Location:   p.makePosition(startPos),
	}, nil
}

// parseExpression parses expressions
func (p *YulParser) parseExpression() (YulExpression, error) {
	return p.parseCall()
}

// parseCall parses function calls
func (p *YulParser) parseCall() (YulExpression, error) {
	if p.check(TokenIdentifier) {
		startPos := p.current.Position
		name := p.advance().Lexeme
		
		if p.match(TokenLeftParen) {
			// Function call
			var args []YulExpression
			
			if !p.check(TokenRightParen) {
				for {
					arg, err := p.parseExpression()
					if err != nil {
						return nil, err
					}
					args = append(args, arg)
					
					if !p.match(TokenComma) {
						break
					}
				}
			}
			
			p.consume(TokenRightParen, "Expected ')'")
			
			return &YulFunctionCall{
				FunctionName: YulIdentifier{
					Name:     name,
					Location: p.makePosition(startPos),
				},
				Arguments:  args,
				Location:   p.makePosition(startPos),
				ResultType: DataTypeUint256,
			}, nil
		} else {
			// Identifier
			return &YulIdentifier{
				Name:     name,
				Location: p.makePosition(startPos),
			}, nil
		}
	}
	
	return p.parsePrimary()
}

// parsePrimary parses primary expressions (literals)
func (p *YulParser) parsePrimary() (YulExpression, error) {
	startPos := p.current.Position
	
	switch p.current.Type {
	case TokenNumber:
		value := p.advance().Lexeme
		return &YulLiteral{
			Kind:     LiteralKindNumber,
			Value:    value,
			Type:     DataTypeUint256,
			Location: p.makePosition(startPos),
		}, nil
		
	case TokenString:
		value := p.advance().Lexeme
		return &YulLiteral{
			Kind:     LiteralKindString,
			Value:    value,
			Type:     DataTypeString,
			Location: p.makePosition(startPos),
		}, nil
		
	case TokenTrue:
		p.advance()
		return &YulLiteral{
			Kind:     LiteralKindBool,
			Value:    "true",
			Type:     DataTypeBool,
			Location: p.makePosition(startPos),
		}, nil
		
	case TokenFalse:
		p.advance()
		return &YulLiteral{
			Kind:     LiteralKindBool,
			Value:    "false",
			Type:     DataTypeBool,
			Location: p.makePosition(startPos),
		}, nil
		
	case TokenHex:
		value := p.advance().Lexeme
		return &YulLiteral{
			Kind:     LiteralKindHex,
			Value:    value,
			Type:     DataTypeBytes32,
			Location: p.makePosition(startPos),
		}, nil
	}

	return nil, fmt.Errorf("unexpected token: %v", p.current.Type)
}

// parseExpressionOrAssignment parses expression statements or assignments
func (p *YulParser) parseExpressionOrAssignment() (YulStatement, error) {
	startPos := p.current.Position
	
	// Try to parse as assignment first
	if p.check(TokenIdentifier) {
		checkpoint := p.current
		names := []string{p.advance().Lexeme}
		
		// Check for multiple assignment targets
		for p.match(TokenComma) {
			names = append(names, p.consume(TokenIdentifier, "Expected identifier").Lexeme)
		}
		
		if p.match(TokenColonEqual) {
			// This is an assignment
			value, err := p.parseExpression()
			if err != nil {
				return nil, err
			}
			
			return &YulAssignment{
				VariableNames: names,
				Value:         value,
				Location:      p.makePosition(startPos),
			}, nil
		} else {
			// Backtrack and parse as expression
			p.current = checkpoint
		}
	}

	// Parse as expression statement
	expr, err := p.parseExpression()
	if err != nil {
		return nil, err
	}

	return &YulExpressionStatement{
		Expression: expr,
		Location:   p.makePosition(startPos),
	}, nil
}

// Helper parsing methods
func (p *YulParser) parseBreak() (*YulBreak, error) {
	pos := p.current.Position
	p.consume(TokenBreak, "Expected 'break'")
	return &YulBreak{Location: p.makePosition(pos)}, nil
}

func (p *YulParser) parseContinue() (*YulContinue, error) {
	pos := p.current.Position
	p.consume(TokenContinue, "Expected 'continue'")
	return &YulContinue{Location: p.makePosition(pos)}, nil
}

func (p *YulParser) parseLeave() (*YulLeave, error) {
	pos := p.current.Position
	p.consume(TokenLeave, "Expected 'leave'")
	return &YulLeave{Location: p.makePosition(pos)}, nil
}

func (p *YulParser) parseData() (*YulData, error) {
	pos := p.current.Position
	value := p.consume(TokenString, "Expected data value").Lexeme
	return &YulData{
		Value:    value,
		Location: p.makePosition(pos),
	}, nil
}

// Utility methods
func (p *YulParser) advance() Token {
	if !p.isAtEnd() {
		p.previous = p.current
		p.current = p.lexer.NextToken()
	}
	return p.previous
}

func (p *YulParser) check(tokenType TokenType) bool {
	if p.isAtEnd() {
		return false
	}
	return p.current.Type == tokenType
}

func (p *YulParser) match(tokenType TokenType) bool {
	if p.check(tokenType) {
		p.advance()
		return true
	}
	return false
}

func (p *YulParser) consume(tokenType TokenType, message string) Token {
	if p.check(tokenType) {
		return p.advance()
	}
	
	panic(fmt.Sprintf("%s. Got %v at line %d", message, p.current.Type, p.current.Line))
}

func (p *YulParser) isAtEnd() bool {
	return p.current.Type == TokenEOF
}

func (p *YulParser) makePosition(pos TokenPosition) SourcePosition {
	return SourcePosition{
		File:   "inline",
		Line:   pos.Line,
		Column: pos.Column,
		Offset: pos.Offset,
		Length: pos.Length,
	}
}

// Implement YulStatement interface methods
func (s *YulExpressionStatement) GetType() YulNodeType   { return NodeTypeExpressionStatement }
func (s *YulExpressionStatement) GetLocation() SourcePosition { return s.Location }
func (s *YulExpressionStatement) Accept(visitor YulVisitor) error { return visitor.VisitExpressionStatement(s) }

func (s *YulVariableDeclaration) GetType() YulNodeType   { return NodeTypeVariableDeclaration }
func (s *YulVariableDeclaration) GetLocation() SourcePosition { return s.Location }
func (s *YulVariableDeclaration) Accept(visitor YulVisitor) error { return visitor.VisitVariableDeclaration(s) }

func (s *YulAssignment) GetType() YulNodeType   { return NodeTypeAssignment }
func (s *YulAssignment) GetLocation() SourcePosition { return s.Location }
func (s *YulAssignment) Accept(visitor YulVisitor) error { return visitor.VisitAssignment(s) }

func (s *YulIf) GetType() YulNodeType   { return NodeTypeIf }
func (s *YulIf) GetLocation() SourcePosition { return s.Location }
func (s *YulIf) Accept(visitor YulVisitor) error { return visitor.VisitIf(s) }

func (s *YulSwitch) GetType() YulNodeType   { return NodeTypeSwitch }
func (s *YulSwitch) GetLocation() SourcePosition { return s.Location }
func (s *YulSwitch) Accept(visitor YulVisitor) error { return visitor.VisitSwitch(s) }

func (s *YulFor) GetType() YulNodeType   { return NodeTypeFor }
func (s *YulFor) GetLocation() SourcePosition { return s.Location }
func (s *YulFor) Accept(visitor YulVisitor) error { return visitor.VisitFor(s) }

func (s *YulFunctionDef) GetType() YulNodeType   { return NodeTypeFunctionDef }
func (s *YulFunctionDef) GetLocation() SourcePosition { return s.Location }
func (s *YulFunctionDef) Accept(visitor YulVisitor) error { return visitor.VisitFunctionDef(s) }

func (s *YulBreak) GetType() YulNodeType   { return NodeTypeBreak }
func (s *YulBreak) GetLocation() SourcePosition { return s.Location }
func (s *YulBreak) Accept(visitor YulVisitor) error { return visitor.VisitBreak(s) }

func (s *YulContinue) GetType() YulNodeType   { return NodeTypeContinue }
func (s *YulContinue) GetLocation() SourcePosition { return s.Location }
func (s *YulContinue) Accept(visitor YulVisitor) error { return visitor.VisitContinue(s) }

func (s *YulLeave) GetType() YulNodeType   { return NodeTypeLeave }
func (s *YulLeave) GetLocation() SourcePosition { return s.Location }
func (s *YulLeave) Accept(visitor YulVisitor) error { return visitor.VisitLeave(s) }

// Implement YulExpression interface methods
func (e *YulFunctionCall) GetType() YulNodeType      { return NodeTypeFunctionCall }
func (e *YulFunctionCall) GetLocation() SourcePosition    { return e.Location }
func (e *YulFunctionCall) GetResultType() YulDataType     { return e.ResultType }
func (e *YulFunctionCall) Accept(visitor YulVisitor) error { return visitor.VisitFunctionCall(e) }

func (e *YulIdentifier) GetType() YulNodeType      { return NodeTypeIdentifier }
func (e *YulIdentifier) GetLocation() SourcePosition    { return e.Location }
func (e *YulIdentifier) GetResultType() YulDataType     { return DataTypeUint256 }
func (e *YulIdentifier) Accept(visitor YulVisitor) error { return visitor.VisitIdentifier(e) }

func (e *YulLiteral) GetType() YulNodeType      { return NodeTypeLiteral }
func (e *YulLiteral) GetLocation() SourcePosition    { return e.Location }
func (e *YulLiteral) GetResultType() YulDataType     { return e.Type }
func (e *YulLiteral) Accept(visitor YulVisitor) error { return visitor.VisitLiteral(e) }