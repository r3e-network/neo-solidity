package main

import (
	"errors"
	"fmt"
	"regexp"
	"strconv"
	"strings"
	"unicode"
)

// YulLexer tokenizes Yul source code into tokens for parsing
type YulLexer struct {
	source   string
	tokens   []Token
	start    int
	current  int
	line     int
	column   int
	keywords map[string]TokenType
}

// Token represents a lexical token in Yul source code
type Token struct {
	Type     TokenType     `json:"type"`
	Lexeme   string        `json:"lexeme"`
	Line     int          `json:"line"`
	Column   int          `json:"column"`
	Position TokenPosition `json:"position"`
}

// TokenPosition represents the position of a token in source code
type TokenPosition struct {
	Line   int `json:"line"`
	Column int `json:"column"`
	Offset int `json:"offset"`
	Length int `json:"length"`
}

// TokenType represents different types of tokens in Yul
type TokenType string

const (
	// Single-character tokens
	TokenLeftParen     TokenType = "("
	TokenRightParen    TokenType = ")"
	TokenLeftBrace     TokenType = "{"
	TokenRightBrace    TokenType = "}"
	TokenComma         TokenType = ","
	TokenDot           TokenType = "."
	TokenColon         TokenType = ":"

	// Two-character tokens  
	TokenColonEqual    TokenType = ":="
	TokenArrow         TokenType = "->"

	// Literals
	TokenIdentifier    TokenType = "IDENTIFIER"
	TokenString        TokenType = "STRING"
	TokenNumber        TokenType = "NUMBER"
	TokenHex           TokenType = "HEX"
	TokenTrue          TokenType = "true"
	TokenFalse         TokenType = "false"

	// Keywords
	TokenObject        TokenType = "object"
	TokenCode          TokenType = "code"
	TokenData          TokenType = "data"
	TokenFunction      TokenType = "function"
	TokenLet           TokenType = "let"
	TokenIf            TokenType = "if"
	TokenSwitch        TokenType = "switch"
	TokenCase          TokenType = "case"
	TokenDefault       TokenType = "default"
	TokenFor           TokenType = "for"
	TokenBreak         TokenType = "break"
	TokenContinue      TokenType = "continue"
	TokenLeave         TokenType = "leave"

	// Built-in function categories (for optimization)
	TokenArithmetic    TokenType = "ARITHMETIC"
	TokenComparison    TokenType = "COMPARISON"
	TokenBitwise       TokenType = "BITWISE"
	TokenMemory        TokenType = "MEMORY"
	TokenStorage       TokenType = "STORAGE"
	TokenEnvironment   TokenType = "ENVIRONMENT"
	TokenControl       TokenType = "CONTROL"

	// Special
	TokenEOF           TokenType = "EOF"
	TokenError         TokenType = "ERROR"
	TokenComment       TokenType = "COMMENT"
	TokenWhitespace    TokenType = "WHITESPACE"
)

// Built-in Yul functions organized by category
var (
	arithmeticOps = map[string]bool{
		"add": true, "sub": true, "mul": true, "div": true, "sdiv": true,
		"mod": true, "smod": true, "exp": true, "not": true, "lt": true,
		"gt": true, "slt": true, "sgt": true, "eq": true, "iszero": true,
		"and": true, "or": true, "xor": true, "byte": true, "shl": true,
		"shr": true, "sar": true, "addmod": true, "mulmod": true, "signextend": true,
	}

	memoryOps = map[string]bool{
		"mload": true, "mstore": true, "mstore8": true, "msize": true,
		"calldataload": true, "calldatasize": true, "calldatacopy": true,
		"codecopy": true, "codesize": true, "extcodesize": true, "extcodecopy": true,
		"returndatasize": true, "returndatacopy": true, "mcopy": true,
	}

	storageOps = map[string]bool{
		"sload": true, "sstore": true,
	}

	environmentOps = map[string]bool{
		"address": true, "balance": true, "caller": true, "callvalue": true,
		"origin": true, "gasprice": true, "extcodesize": true, "extcodehash": true,
		"blockhash": true, "coinbase": true, "timestamp": true, "number": true,
		"difficulty": true, "gaslimit": true, "chainid": true, "selfbalance": true,
		"basefee": true, "gas": true, "pc": true, "msize": true, "gas": true,
	}

	controlOps = map[string]bool{
		"call": true, "callcode": true, "delegatecall": true, "staticcall": true,
		"return": true, "revert": true, "selfdestruct": true, "invalid": true,
		"log0": true, "log1": true, "log2": true, "log3": true, "log4": true,
		"create": true, "create2": true, "stop": true,
	}
)

// NewYulLexer creates a new Yul lexer instance
func NewYulLexer() *YulLexer {
	lexer := &YulLexer{
		keywords: make(map[string]TokenType),
		line:     1,
		column:   1,
	}

	// Initialize keywords
	lexer.keywords["object"] = TokenObject
	lexer.keywords["code"] = TokenCode
	lexer.keywords["data"] = TokenData
	lexer.keywords["function"] = TokenFunction
	lexer.keywords["let"] = TokenLet
	lexer.keywords["if"] = TokenIf
	lexer.keywords["switch"] = TokenSwitch
	lexer.keywords["case"] = TokenCase
	lexer.keywords["default"] = TokenDefault
	lexer.keywords["for"] = TokenFor
	lexer.keywords["break"] = TokenBreak
	lexer.keywords["continue"] = TokenContinue
	lexer.keywords["leave"] = TokenLeave
	lexer.keywords["true"] = TokenTrue
	lexer.keywords["false"] = TokenFalse

	return lexer
}

// Init initializes the lexer with source code
func (l *YulLexer) Init(source string) error {
	if len(source) == 0 {
		return errors.New("empty source code")
	}

	l.source = source
	l.tokens = []Token{}
	l.start = 0
	l.current = 0
	l.line = 1
	l.column = 1

	return nil
}

// ScanTokens scans the entire source and returns all tokens
func (l *YulLexer) ScanTokens() ([]Token, error) {
	if l.source == "" {
		return nil, errors.New("lexer not initialized")
	}

	for !l.isAtEnd() {
		l.start = l.current
		err := l.scanToken()
		if err != nil {
			return nil, err
		}
	}

	l.tokens = append(l.tokens, Token{
		Type:   TokenEOF,
		Lexeme: "",
		Line:   l.line,
		Column: l.column,
		Position: TokenPosition{
			Line:   l.line,
			Column: l.column,
			Offset: l.current,
			Length: 0,
		},
	})

	return l.tokens, nil
}

// NextToken returns the next token from the source
func (l *YulLexer) NextToken() Token {
	if l.current >= len(l.tokens) {
		if len(l.tokens) > 0 {
			return l.tokens[len(l.tokens)-1] // Return EOF
		}
		// Generate EOF token
		return Token{
			Type:   TokenEOF,
			Lexeme: "",
			Line:   l.line,
			Column: l.column,
			Position: TokenPosition{
				Line:   l.line,
				Column: l.column,
				Offset: len(l.source),
				Length: 0,
			},
		}
	}

	token := l.tokens[l.current]
	l.current++
	return token
}

// scanToken scans a single token from the source
func (l *YulLexer) scanToken() error {
	c := l.advance()

	switch c {
	case '(':
		l.addToken(TokenLeftParen)
	case ')':
		l.addToken(TokenRightParen)
	case '{':
		l.addToken(TokenLeftBrace)
	case '}':
		l.addToken(TokenRightBrace)
	case ',':
		l.addToken(TokenComma)
	case '.':
		l.addToken(TokenDot)
	case ':':
		if l.match('=') {
			l.addToken(TokenColonEqual)
		} else {
			l.addToken(TokenColon)
		}
	case '-':
		if l.match('>') {
			l.addToken(TokenArrow)
		} else {
			return fmt.Errorf("unexpected character '-' at line %d, column %d", l.line, l.column-1)
		}
	case ' ', '\r', '\t':
		// Ignore whitespace
	case '\n':
		l.line++
		l.column = 0 // Will be incremented in advance()
	case '/':
		if l.match('/') {
			// Line comment
			l.scanLineComment()
		} else if l.match('*') {
			// Block comment
			err := l.scanBlockComment()
			if err != nil {
				return err
			}
		} else {
			return fmt.Errorf("unexpected character '/' at line %d, column %d", l.line, l.column-1)
		}
	case '"':
		err := l.scanString()
		if err != nil {
			return err
		}
	case '0':
		if l.peek() == 'x' || l.peek() == 'X' {
			l.advance() // consume 'x' or 'X'
			err := l.scanHexNumber()
			if err != nil {
				return err
			}
		} else {
			err := l.scanNumber()
			if err != nil {
				return err
			}
		}
	default:
		if l.isDigit(c) {
			err := l.scanNumber()
			if err != nil {
				return err
			}
		} else if l.isAlpha(c) {
			l.scanIdentifier()
		} else {
			return fmt.Errorf("unexpected character '%c' at line %d, column %d", c, l.line, l.column-1)
		}
	}

	return nil
}

// scanString scans a string literal
func (l *YulLexer) scanString() error {
	for l.peek() != '"' && !l.isAtEnd() {
		if l.peek() == '\n' {
			l.line++
			l.column = 0
		}
		l.advance()
	}

	if l.isAtEnd() {
		return fmt.Errorf("unterminated string at line %d", l.line)
	}

	// Consume closing "
	l.advance()

	// Get string value (without quotes)
	value := l.source[l.start+1 : l.current-1]
	l.addTokenWithLiteral(TokenString, value)

	return nil
}

// scanNumber scans a decimal number
func (l *YulLexer) scanNumber() error {
	for l.isDigit(l.peek()) {
		l.advance()
	}

	// Validate number format
	value := l.source[l.start:l.current]
	_, err := strconv.ParseUint(value, 10, 64)
	if err != nil {
		// Try parsing as big integer (Yul supports arbitrary precision)
		if !l.isValidNumber(value) {
			return fmt.Errorf("invalid number format '%s' at line %d", value, l.line)
		}
	}

	l.addTokenWithLiteral(TokenNumber, value)
	return nil
}

// scanHexNumber scans a hexadecimal number
func (l *YulLexer) scanHexNumber() error {
	if !l.isHexDigit(l.peek()) {
		return fmt.Errorf("invalid hex number at line %d", l.line)
	}

	for l.isHexDigit(l.peek()) {
		l.advance()
	}

	// Get hex value including '0x' prefix
	value := l.source[l.start:l.current]
	
	// Validate hex format
	if !l.isValidHex(value) {
		return fmt.Errorf("invalid hex format '%s' at line %d", value, l.line)
	}

	l.addTokenWithLiteral(TokenHex, value)
	return nil
}

// scanIdentifier scans identifiers and keywords
func (l *YulLexer) scanIdentifier() {
	for l.isAlphaNumeric(l.peek()) {
		l.advance()
	}

	text := l.source[l.start:l.current]
	
	// Check if it's a keyword
	if tokenType, exists := l.keywords[text]; exists {
		l.addToken(tokenType)
		return
	}

	// Check if it's a built-in function and categorize
	if arithmeticOps[text] {
		l.addTokenWithLiteral(TokenArithmetic, text)
	} else if memoryOps[text] {
		l.addTokenWithLiteral(TokenMemory, text)
	} else if storageOps[text] {
		l.addTokenWithLiteral(TokenStorage, text)
	} else if environmentOps[text] {
		l.addTokenWithLiteral(TokenEnvironment, text)
	} else if controlOps[text] {
		l.addTokenWithLiteral(TokenControl, text)
	} else {
		// Regular identifier
		l.addToken(TokenIdentifier)
	}
}

// scanLineComment scans line comments
func (l *YulLexer) scanLineComment() {
	for l.peek() != '\n' && !l.isAtEnd() {
		l.advance()
	}
	// Comments are ignored in this implementation
}

// scanBlockComment scans block comments
func (l *YulLexer) scanBlockComment() error {
	nesting := 1
	
	for nesting > 0 && !l.isAtEnd() {
		if l.peek() == '/' && l.peekNext() == '*' {
			nesting++
			l.advance() // consume '/'
			l.advance() // consume '*'
		} else if l.peek() == '*' && l.peekNext() == '/' {
			nesting--
			l.advance() // consume '*'
			l.advance() // consume '/'
		} else {
			if l.peek() == '\n' {
				l.line++
				l.column = 0
			}
			l.advance()
		}
	}

	if nesting > 0 {
		return fmt.Errorf("unterminated block comment at line %d", l.line)
	}

	return nil
}

// Utility methods
func (l *YulLexer) isAtEnd() bool {
	return l.current >= len(l.source)
}

func (l *YulLexer) advance() byte {
	if l.isAtEnd() {
		return 0
	}
	l.column++
	char := l.source[l.current]
	l.current++
	return char
}

func (l *YulLexer) match(expected byte) bool {
	if l.isAtEnd() {
		return false
	}
	if l.source[l.current] != expected {
		return false
	}
	l.current++
	l.column++
	return true
}

func (l *YulLexer) peek() byte {
	if l.isAtEnd() {
		return 0
	}
	return l.source[l.current]
}

func (l *YulLexer) peekNext() byte {
	if l.current+1 >= len(l.source) {
		return 0
	}
	return l.source[l.current+1]
}

func (l *YulLexer) isDigit(c byte) bool {
	return c >= '0' && c <= '9'
}

func (l *YulLexer) isAlpha(c byte) bool {
	return (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_'
}

func (l *YulLexer) isAlphaNumeric(c byte) bool {
	return l.isAlpha(c) || l.isDigit(c)
}

func (l *YulLexer) isHexDigit(c byte) bool {
	return l.isDigit(c) || (c >= 'a' && c <= 'f') || (c >= 'A' && c <= 'F')
}

func (l *YulLexer) isValidNumber(s string) bool {
	if len(s) == 0 {
		return false
	}
	for _, c := range s {
		if !unicode.IsDigit(c) {
			return false
		}
	}
	return true
}

func (l *YulLexer) isValidHex(s string) bool {
	if len(s) < 3 || s[0:2] != "0x" {
		return false
	}
	for _, c := range s[2:] {
		if !unicode.IsDigit(c) && !(c >= 'a' && c <= 'f') && !(c >= 'A' && c <= 'F') {
			return false
		}
	}
	return true
}

func (l *YulLexer) addToken(tokenType TokenType) {
	l.addTokenWithLiteral(tokenType, "")
}

func (l *YulLexer) addTokenWithLiteral(tokenType TokenType, literal string) {
	text := l.source[l.start:l.current]
	if literal == "" {
		literal = text
	}

	token := Token{
		Type:   tokenType,
		Lexeme: literal,
		Line:   l.line,
		Column: l.column - (l.current - l.start),
		Position: TokenPosition{
			Line:   l.line,
			Column: l.column - (l.current - l.start),
			Offset: l.start,
			Length: l.current - l.start,
		},
	}

	l.tokens = append(l.tokens, token)
}

// TokenInfo provides metadata about token types
type TokenInfo struct {
	Type        TokenType `json:"type"`
	Description string    `json:"description"`
	Category    string    `json:"category"`
	Precedence  int       `json:"precedence"`
}

// GetTokenInfo returns information about a token type
func GetTokenInfo(tokenType TokenType) TokenInfo {
	switch tokenType {
	case TokenLeftParen:
		return TokenInfo{tokenType, "Left parenthesis", "Punctuation", 0}
	case TokenRightParen:
		return TokenInfo{tokenType, "Right parenthesis", "Punctuation", 0}
	case TokenLeftBrace:
		return TokenInfo{tokenType, "Left brace", "Punctuation", 0}
	case TokenRightBrace:
		return TokenInfo{tokenType, "Right brace", "Punctuation", 0}
	case TokenColonEqual:
		return TokenInfo{tokenType, "Assignment operator", "Operator", 1}
	case TokenArrow:
		return TokenInfo{tokenType, "Function return arrow", "Operator", 0}
	case TokenIdentifier:
		return TokenInfo{tokenType, "Identifier", "Literal", 0}
	case TokenString:
		return TokenInfo{tokenType, "String literal", "Literal", 0}
	case TokenNumber:
		return TokenInfo{tokenType, "Number literal", "Literal", 0}
	case TokenHex:
		return TokenInfo{tokenType, "Hexadecimal literal", "Literal", 0}
	case TokenArithmetic:
		return TokenInfo{tokenType, "Arithmetic built-in", "Built-in", 10}
	case TokenMemory:
		return TokenInfo{tokenType, "Memory built-in", "Built-in", 10}
	case TokenStorage:
		return TokenInfo{tokenType, "Storage built-in", "Built-in", 10}
	case TokenEnvironment:
		return TokenInfo{tokenType, "Environment built-in", "Built-in", 10}
	case TokenControl:
		return TokenInfo{tokenType, "Control flow built-in", "Built-in", 10}
	default:
		return TokenInfo{tokenType, "Unknown token", "Unknown", 0}
	}
}

// LexicalAnalysis provides detailed lexical analysis of Yul source
type LexicalAnalysis struct {
	TokenCount     map[TokenType]int `json:"token_count"`
	TotalTokens    int               `json:"total_tokens"`
	Lines          int               `json:"lines"`
	Functions      int               `json:"functions"`
	Variables      int               `json:"variables"`
	BuiltinCalls   int               `json:"builtin_calls"`
	Complexity     float64           `json:"complexity"`
}

// AnalyzeTokens performs lexical analysis on a token stream
func AnalyzeTokens(tokens []Token) LexicalAnalysis {
	analysis := LexicalAnalysis{
		TokenCount: make(map[TokenType]int),
	}

	maxLine := 0
	for _, token := range tokens {
		analysis.TokenCount[token.Type]++
		analysis.TotalTokens++
		
		if token.Line > maxLine {
			maxLine = token.Line
		}

		switch token.Type {
		case TokenFunction:
			analysis.Functions++
		case TokenLet:
			analysis.Variables++
		case TokenArithmetic, TokenMemory, TokenStorage, TokenEnvironment, TokenControl:
			analysis.BuiltinCalls++
		}
	}

	analysis.Lines = maxLine
	
	// Calculate complexity score based on control structures and function calls
	analysis.Complexity = float64(analysis.TokenCount[TokenIf]+
		analysis.TokenCount[TokenFor]+
		analysis.TokenCount[TokenSwitch]+
		analysis.Functions) / float64(analysis.TotalTokens) * 100

	return analysis
}

// PrettyPrintTokens formats tokens for debugging
func PrettyPrintTokens(tokens []Token) string {
	var builder strings.Builder
	builder.WriteString("Tokens:\n")
	builder.WriteString("-------\n")
	
	for i, token := range tokens {
		builder.WriteString(fmt.Sprintf("%3d: %-15s %-20s @%d:%d\n", 
			i, token.Type, fmt.Sprintf("'%s'", token.Lexeme), token.Line, token.Column))
	}
	
	return builder.String()
}

// ValidateTokenStream performs basic validation on a token stream
func ValidateTokenStream(tokens []Token) error {
	if len(tokens) == 0 {
		return errors.New("empty token stream")
	}

	// Check for balanced braces and parentheses
	braceDepth := 0
	parenDepth := 0

	for _, token := range tokens {
		switch token.Type {
		case TokenLeftBrace:
			braceDepth++
		case TokenRightBrace:
			braceDepth--
			if braceDepth < 0 {
				return fmt.Errorf("unmatched '}' at line %d", token.Line)
			}
		case TokenLeftParen:
			parenDepth++
		case TokenRightParen:
			parenDepth--
			if parenDepth < 0 {
				return fmt.Errorf("unmatched ')' at line %d", token.Line)
			}
		}
	}

	if braceDepth != 0 {
		return fmt.Errorf("unmatched braces: %d unclosed '{'", braceDepth)
	}

	if parenDepth != 0 {
		return fmt.Errorf("unmatched parentheses: %d unclosed '('", parenDepth)
	}

	// Check that EOF is the last token
	if tokens[len(tokens)-1].Type != TokenEOF {
		return errors.New("token stream does not end with EOF")
	}

	return nil
}

// TokenPattern represents a pattern for advanced token matching
type TokenPattern struct {
	Pattern     []TokenType `json:"pattern"`
	Name        string      `json:"name"`
	Description string      `json:"description"`
}

// Common Yul patterns
var CommonPatterns = []TokenPattern{
	{
		Pattern:     []TokenType{TokenLet, TokenIdentifier, TokenColonEqual},
		Name:        "variable_declaration",
		Description: "Variable declaration pattern",
	},
	{
		Pattern:     []TokenType{TokenFunction, TokenIdentifier, TokenLeftParen},
		Name:        "function_definition",
		Description: "Function definition pattern",
	},
	{
		Pattern:     []TokenType{TokenIf, TokenIdentifier, TokenLeftBrace},
		Name:        "conditional_block",
		Description: "Conditional execution pattern",
	},
	{
		Pattern:     []TokenType{TokenSwitch, TokenIdentifier, TokenCase},
		Name:        "switch_statement",
		Description: "Switch statement pattern",
	},
}

// FindPatterns finds occurrences of common patterns in token stream
func FindPatterns(tokens []Token, patterns []TokenPattern) map[string][]int {
	matches := make(map[string][]int)
	
	for _, pattern := range patterns {
		matches[pattern.Name] = []int{}
		
		for i := 0; i <= len(tokens)-len(pattern.Pattern); i++ {
			match := true
			for j, expectedType := range pattern.Pattern {
				if tokens[i+j].Type != expectedType {
					match = false
					break
				}
			}
			if match {
				matches[pattern.Name] = append(matches[pattern.Name], i)
			}
		}
	}
	
	return matches
}