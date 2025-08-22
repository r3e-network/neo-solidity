package main

import (
	"errors"
	"fmt"
	"log"
	"strings"
)

// YulToNeoCompiler is the main compiler struct that orchestrates the compilation
// process from Yul IR to NeoVM bytecode.
type YulToNeoCompiler struct {
	Config          CompilerConfig
	Parser          *YulParser
	Normalizer      *IRNormalizer
	StaticAnalyzer  *StaticAnalyzer
	Optimizer       *OptimizationEngine
	CodeGenerator   *CodeGenerator
	RuntimeManager  *RuntimeManager
}

// CompilerConfig holds configuration options for the compilation process
type CompilerConfig struct {
	OptimizationLevel   int          // 0-3, higher = more aggressive optimization
	TargetNeoVMVersion  string       // Target NeoVM version
	EnableBoundsChecking bool         // Insert runtime bounds checks
	EnableDebugInfo     bool         // Generate debug information
	MaxStackDepth       int          // Maximum allowed stack depth
	MemoryLimit         int64        // Memory usage limit in bytes
	CompilerFlags       []string     // Additional compiler flags
}

// CompilerContext maintains state throughout the compilation process
type CompilerContext struct {
	SourceMap       map[string]string  // Source location mapping
	SymbolTable     *SymbolTable       // Variable and function symbols
	TypeTable       *TypeTable         // Type information
	LabelCounter    int                // Unique label counter
	ErrorCollector  *ErrorCollector    // Compilation error collection
	Metadata        *CompilationMetadata
}

// CompilationResult contains the output of the compilation process
type CompilationResult struct {
	Contract        *NeoContract       // Generated NeoVM contract
	Warnings        []CompilerWarning  // Non-fatal warnings
	Errors          []CompilerError    // Fatal errors
	Statistics      CompilationStats   // Performance statistics
	DebugInfo       *DebugInformation  // Debug symbols and source maps
}

// NewYulToNeoCompiler creates a new compiler instance with the given configuration
func NewYulToNeoCompiler(config CompilerConfig) *YulToNeoCompiler {
	context := &CompilerContext{
		SourceMap:      make(map[string]string),
		SymbolTable:    NewSymbolTable(),
		TypeTable:      NewTypeTable(),
		LabelCounter:   0,
		ErrorCollector: NewErrorCollector(),
		Metadata:       NewCompilationMetadata(),
	}

	return &YulToNeoCompiler{
		Config:         config,
		Parser:         NewYulParser(),
		Normalizer:     NewIRNormalizer(context),
		StaticAnalyzer: NewStaticAnalyzer(context),
		Optimizer:      NewOptimizationEngine(config.OptimizationLevel),
		CodeGenerator:  NewCodeGenerator(context),
		RuntimeManager: NewRuntimeManager(context),
	}
}

// Compile performs the complete compilation process from Yul source to NeoVM bytecode
func (c *YulToNeoCompiler) Compile(yulSource string) (*CompilationResult, error) {
	log.Printf("Starting Yul to NeoVM compilation process")
	
	result := &CompilationResult{
		Statistics: CompilationStats{},
	}

	// Phase 1: Parse Yul source into AST
	log.Printf("Phase 1: Parsing Yul source")
	ast, err := c.Parser.Parse(yulSource)
	if err != nil {
		result.Errors = append(result.Errors, CompilerError{
			Phase: "Parsing",
			Message: fmt.Sprintf("Parse error: %v", err),
		})
		return result, err
	}

	// Phase 2: Normalize IR to canonical form
	log.Printf("Phase 2: Normalizing IR")
	normalizedAST, err := c.Normalizer.Normalize(ast)
	if err != nil {
		result.Errors = append(result.Errors, CompilerError{
			Phase: "Normalization",
			Message: fmt.Sprintf("Normalization error: %v", err),
		})
		return result, err
	}

	// Phase 3: Static analysis and validation
	log.Printf("Phase 3: Static analysis")
	analysisResult, err := c.StaticAnalyzer.Analyze(normalizedAST)
	if err != nil {
		result.Errors = append(result.Errors, CompilerError{
			Phase: "Static Analysis",
			Message: fmt.Sprintf("Analysis error: %v", err),
		})
		return result, err
	}
	result.Warnings = append(result.Warnings, analysisResult.Warnings...)

	// Phase 4: Optimization passes
	log.Printf("Phase 4: Optimization")
	optimizedAST, err := c.Optimizer.Optimize(normalizedAST)
	if err != nil {
		result.Errors = append(result.Errors, CompilerError{
			Phase: "Optimization",
			Message: fmt.Sprintf("Optimization error: %v", err),
		})
		return result, err
	}

	// Phase 5: Code generation
	log.Printf("Phase 5: Code generation")
	contract, err := c.CodeGenerator.Generate(optimizedAST)
	if err != nil {
		result.Errors = append(result.Errors, CompilerError{
			Phase: "Code Generation",
			Message: fmt.Sprintf("Code generation error: %v", err),
		})
		return result, err
	}

	// Phase 6: Runtime integration and finalization
	log.Printf("Phase 6: Runtime integration")
	finalContract, err := c.RuntimeManager.Finalize(contract)
	if err != nil {
		result.Errors = append(result.Errors, CompilerError{
			Phase: "Runtime Integration",
			Message: fmt.Sprintf("Runtime error: %v", err),
		})
		return result, err
	}

	result.Contract = finalContract
	
	// Generate debug information if requested
	if c.Config.EnableDebugInfo {
		result.DebugInfo = c.generateDebugInfo(ast, finalContract)
	}

	log.Printf("Compilation completed successfully")
	return result, nil
}

// CompileFromFile compiles Yul source from a file
func (c *YulToNeoCompiler) CompileFromFile(filename string) (*CompilationResult, error) {
	// Implementation would read file and call Compile()
	return nil, errors.New("file compilation not implemented yet")
}

// Validate performs validation without full compilation
func (c *YulToNeoCompiler) Validate(yulSource string) (*ValidationResult, error) {
	ast, err := c.Parser.Parse(yulSource)
	if err != nil {
		return nil, err
	}

	normalizedAST, err := c.Normalizer.Normalize(ast)
	if err != nil {
		return nil, err
	}

	analysisResult, err := c.StaticAnalyzer.Analyze(normalizedAST)
	if err != nil {
		return nil, err
	}

	return &ValidationResult{
		IsValid:  len(analysisResult.Errors) == 0,
		Errors:   analysisResult.Errors,
		Warnings: analysisResult.Warnings,
	}, nil
}

// generateDebugInfo creates debug information for the compiled contract
func (c *YulToNeoCompiler) generateDebugInfo(ast *YulAST, contract *NeoContract) *DebugInformation {
	return &DebugInformation{
		SourceMap:        c.buildSourceMap(ast),
		FunctionMap:      c.buildFunctionMap(ast, contract),
		VariableMap:      c.buildVariableMap(ast, contract),
		InstructionMap:   c.buildInstructionMap(ast, contract),
	}
}

func (c *YulToNeoCompiler) buildSourceMap(ast *YulAST) map[int]SourceLocation {
	// Build mapping from bytecode offset to source location
	return make(map[int]SourceLocation)
}

func (c *YulToNeoCompiler) buildFunctionMap(ast *YulAST, contract *NeoContract) map[string]FunctionInfo {
	// Build mapping from function names to bytecode locations
	return make(map[string]FunctionInfo)
}

func (c *YulToNeoCompiler) buildVariableMap(ast *YulAST, contract *NeoContract) map[string]VariableInfo {
	// Build mapping from variable names to stack/storage locations
	return make(map[string]VariableInfo)
}

func (c *YulToNeoCompiler) buildInstructionMap(ast *YulAST, contract *NeoContract) map[int]InstructionInfo {
	// Build mapping from bytecode offset to instruction information
	return make(map[int]InstructionInfo)
}

// Supporting types and structures

type CompilerError struct {
	Phase    string `json:"phase"`
	Message  string `json:"message"`
	Line     int    `json:"line,omitempty"`
	Column   int    `json:"column,omitempty"`
	Severity string `json:"severity"`
}

type CompilerWarning struct {
	Phase   string `json:"phase"`
	Message string `json:"message"`
	Line    int    `json:"line,omitempty"`
	Column  int    `json:"column,omitempty"`
}

type CompilationStats struct {
	CompilationTimeMs   int64 `json:"compilation_time_ms"`
	OriginalSizeBytes   int   `json:"original_size_bytes"`
	CompiledSizeBytes   int   `json:"compiled_size_bytes"`
	OptimizationsPassed int   `json:"optimizations_passed"`
	FunctionsCompiled   int   `json:"functions_compiled"`
}

type ValidationResult struct {
	IsValid  bool              `json:"is_valid"`
	Errors   []CompilerError   `json:"errors"`
	Warnings []CompilerWarning `json:"warnings"`
}

type DebugInformation struct {
	SourceMap      map[int]SourceLocation    `json:"source_map"`
	FunctionMap    map[string]FunctionInfo   `json:"function_map"`
	VariableMap    map[string]VariableInfo   `json:"variable_map"`
	InstructionMap map[int]InstructionInfo   `json:"instruction_map"`
}

type SourceLocation struct {
	File   string `json:"file"`
	Line   int    `json:"line"`
	Column int    `json:"column"`
	Length int    `json:"length"`
}

type FunctionInfo struct {
	Name           string `json:"name"`
	StartOffset    int    `json:"start_offset"`
	EndOffset      int    `json:"end_offset"`
	ParameterCount int    `json:"parameter_count"`
	ReturnCount    int    `json:"return_count"`
}

type VariableInfo struct {
	Name         string `json:"name"`
	Type         string `json:"type"`
	StorageType  string `json:"storage_type"` // "stack", "storage", "memory"
	Location     int    `json:"location"`
	Size         int    `json:"size"`
}

type InstructionInfo struct {
	Opcode      string         `json:"opcode"`
	Offset      int           `json:"offset"`
	StackBefore int           `json:"stack_before"`
	StackAfter  int           `json:"stack_after"`
	SourceRef   SourceLocation `json:"source_ref"`
}

// CompilationMetadata stores metadata about the compilation process
type CompilationMetadata struct {
	CompilerVersion string            `json:"compiler_version"`
	CompilationTime string            `json:"compilation_time"`
	SourceHash      string            `json:"source_hash"`
	Config          CompilerConfig    `json:"config"`
	Statistics      CompilationStats  `json:"statistics"`
}

func NewCompilationMetadata() *CompilationMetadata {
	return &CompilationMetadata{
		CompilerVersion: "1.0.0",
		Statistics:      CompilationStats{},
	}
}

// Example usage and testing functions

func ExampleCompilation() {
	// Example Yul code
	yulCode := `
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

	// Configure compiler
	config := CompilerConfig{
		OptimizationLevel:    2,
		TargetNeoVMVersion:   "3.0",
		EnableBoundsChecking: true,
		EnableDebugInfo:     true,
		MaxStackDepth:       1024,
		MemoryLimit:         64 * 1024 * 1024, // 64MB
	}

	// Create compiler and compile
	compiler := NewYulToNeoCompiler(config)
	result, err := compiler.Compile(yulCode)
	
	if err != nil {
		log.Printf("Compilation failed: %v", err)
		return
	}

	if len(result.Errors) > 0 {
		log.Printf("Compilation errors:")
		for _, error := range result.Errors {
			log.Printf("  %s: %s", error.Phase, error.Message)
		}
		return
	}

	log.Printf("Compilation successful!")
	log.Printf("Contract size: %d bytes", len(result.Contract.Runtime))
	log.Printf("Functions compiled: %d", result.Statistics.FunctionsCompiled)
	log.Printf("Compilation time: %dms", result.Statistics.CompilationTimeMs)

	if len(result.Warnings) > 0 {
		log.Printf("Warnings:")
		for _, warning := range result.Warnings {
			log.Printf("  %s: %s", warning.Phase, warning.Message)
		}
	}
}

func main() {
	fmt.Println("Yul to NeoVM Compiler v1.0.0")
	fmt.Println("============================")
	ExampleCompilation()
}