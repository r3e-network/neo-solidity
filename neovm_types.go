package main

import (
	"encoding/hex"
	"fmt"
	"math/big"
)

// NeoVM instruction set and data structures for code generation

// NeoInstruction represents a single NeoVM instruction
type NeoInstruction struct {
	Opcode      NeoOpcode     `json:"opcode"`
	Operand     []byte        `json:"operand,omitempty"`
	Size        int           `json:"size"`
	StackPop    int           `json:"stack_pop"`    // Number of items popped from stack
	StackPush   int           `json:"stack_push"`   // Number of items pushed to stack
	GasCost     int64         `json:"gas_cost"`
	SourceRef   *SourcePosition `json:"source_ref,omitempty"`
	Comment     string        `json:"comment,omitempty"`
}

// NeoOpcode represents NeoVM instruction opcodes
type NeoOpcode byte

// NeoVM instruction set constants
const (
	// Stack operations
	PUSH0    NeoOpcode = 0x10
	PUSH1    NeoOpcode = 0x11
	PUSH2    NeoOpcode = 0x12
	PUSH3    NeoOpcode = 0x13
	PUSH4    NeoOpcode = 0x14
	PUSH5    NeoOpcode = 0x15
	PUSH6    NeoOpcode = 0x16
	PUSH7    NeoOpcode = 0x17
	PUSH8    NeoOpcode = 0x18
	PUSH9    NeoOpcode = 0x19
	PUSH10   NeoOpcode = 0x1A
	PUSH11   NeoOpcode = 0x1B
	PUSH12   NeoOpcode = 0x1C
	PUSH13   NeoOpcode = 0x1D
	PUSH14   NeoOpcode = 0x1E
	PUSH15   NeoOpcode = 0x1F
	PUSH16   NeoOpcode = 0x20

	PUSHINT8   NeoOpcode = 0x00
	PUSHINT16  NeoOpcode = 0x01
	PUSHINT32  NeoOpcode = 0x02
	PUSHINT64  NeoOpcode = 0x03
	PUSHINT128 NeoOpcode = 0x04
	PUSHINT256 NeoOpcode = 0x05

	PUSHDATA1 NeoOpcode = 0x0C
	PUSHDATA2 NeoOpcode = 0x0D
	PUSHDATA4 NeoOpcode = 0x0E

	// Stack manipulation
	DUP   NeoOpcode = 0x21
	SWAP  NeoOpcode = 0x23
	ROT   NeoOpcode = 0x24
	ROLL  NeoOpcode = 0x25
	PICK  NeoOpcode = 0x26
	TUCK  NeoOpcode = 0x27
	DROP  NeoOpcode = 0x28
	NIP   NeoOpcode = 0x29
	XDROP NeoOpcode = 0x2A
	CLEAR NeoOpcode = 0x2B
	DEPTH NeoOpcode = 0x2C

	// Arithmetic
	ADD    NeoOpcode = 0x9F
	SUB    NeoOpcode = 0xA0
	MUL    NeoOpcode = 0xA1
	DIV    NeoOpcode = 0xA2
	MOD    NeoOpcode = 0xA3
	SHL    NeoOpcode = 0xA8
	SHR    NeoOpcode = 0xA9
	NOT    NeoOpcode = 0xAA
	BOOLAND NeoOpcode = 0xAB
	BOOLOR  NeoOpcode = 0xAC
	NUMEQUAL  NeoOpcode = 0xAD
	NUMNOTEQUAL NeoOpcode = 0xAE
	LT     NeoOpcode = 0xAF
	LE     NeoOpcode = 0xB0
	GT     NeoOpcode = 0xB1
	GE     NeoOpcode = 0xB2
	MIN    NeoOpcode = 0xB3
	MAX    NeoOpcode = 0xB4
	WITHIN NeoOpcode = 0xB5

	// Bitwise
	AND NeoOpcode = 0xA4
	OR  NeoOpcode = 0xA5
	XOR NeoOpcode = 0xA6
	EQUAL NeoOpcode = 0x97
	NOTEQUAL NeoOpcode = 0x98

	// Control flow
	NOP     NeoOpcode = 0x21
	JMP     NeoOpcode = 0x22
	JMPIF   NeoOpcode = 0x23
	JMPIFNOT NeoOpcode = 0x24
	JMPEQ   NeoOpcode = 0x25
	JMPNE   NeoOpcode = 0x26
	JMPGT   NeoOpcode = 0x27
	JMPGE   NeoOpcode = 0x28
	JMPLT   NeoOpcode = 0x29
	JMPLE   NeoOpcode = 0x2A
	CALL    NeoOpcode = 0x2B
	CALLA   NeoOpcode = 0x2C
	CALLT   NeoOpcode = 0x2D
	ABORT   NeoOpcode = 0x2E
	ASSERT  NeoOpcode = 0x2F
	THROW   NeoOpcode = 0x3A
	TRY     NeoOpcode = 0x3B
	TRYFINALLY NeoOpcode = 0x3C
	ENDTRY  NeoOpcode = 0x3D
	ENDFINALLY NeoOpcode = 0x3E
	RET     NeoOpcode = 0x40

	// Array and buffer operations
	NEWARRAY  NeoOpcode = 0xC5
	NEWSTRUCT NeoOpcode = 0xC6
	NEWMAP    NeoOpcode = 0xC8
	APPEND    NeoOpcode = 0xC9
	REVERSE   NeoOpcode = 0xCA
	REMOVE    NeoOpcode = 0xCB
	HASKEY    NeoOpcode = 0xCC
	KEYS      NeoOpcode = 0xCD
	VALUES    NeoOpcode = 0xCE
	PICKITEM  NeoOpcode = 0xCF
	SETITEM   NeoOpcode = 0xD0
	SIZE      NeoOpcode = 0xD1

	// Type operations
	ISNULL   NeoOpcode = 0xD8
	ISTYPE   NeoOpcode = 0xD9
	CONVERT  NeoOpcode = 0xDB

	// System calls
	SYSCALL NeoOpcode = 0x41
)

// NeoVMStackItem represents different types of items on the NeoVM stack
type NeoVMStackItem interface {
	Type() NeoVMType
	ToBytes() []byte
	String() string
	Size() int
}

// NeoVMType represents the type of a stack item
type NeoVMType byte

const (
	AnyType         NeoVMType = 0x00
	PointerType     NeoVMType = 0x10
	BooleanType     NeoVMType = 0x20
	IntegerType     NeoVMType = 0x21
	ByteStringType  NeoVMType = 0x28
	BufferType      NeoVMType = 0x30
	ArrayType       NeoVMType = 0x40
	StructType      NeoVMType = 0x41
	MapType         NeoVMType = 0x48
	InteropType     NeoVMType = 0x60
)

// Concrete NeoVM stack item implementations
type (
	// NeoVMInteger represents arbitrary precision integers
	NeoVMInteger struct {
		Value *big.Int
	}

	// NeoVMByteString represents immutable byte arrays
	NeoVMByteString struct {
		Value []byte
	}

	// NeoVMBuffer represents mutable byte arrays
	NeoVMBuffer struct {
		Value []byte
	}

	// NeoVMBoolean represents boolean values
	NeoVMBoolean struct {
		Value bool
	}

	// NeoVMArray represents heterogeneous arrays
	NeoVMArray struct {
		Items []NeoVMStackItem
	}

	// NeoVMStruct represents structured data (like arrays but different semantics)
	NeoVMStruct struct {
		Items []NeoVMStackItem
	}

	// NeoVMMap represents key-value mappings
	NeoVMMap struct {
		Items map[string]NeoVMStackItem
	}

	// NeoVMPointer represents references to other stack items
	NeoVMPointer struct {
		Target NeoVMStackItem
	}

	// NeoVMInterop represents external system interfaces
	NeoVMInterop struct {
		Interface string
		Methods   map[string]interface{}
	}
)

// Implement NeoVMStackItem interface for each type

func (i *NeoVMInteger) Type() NeoVMType    { return IntegerType }
func (i *NeoVMInteger) ToBytes() []byte    { return i.Value.Bytes() }
func (i *NeoVMInteger) String() string     { return i.Value.String() }
func (i *NeoVMInteger) Size() int          { return len(i.Value.Bytes()) }

func (b *NeoVMByteString) Type() NeoVMType { return ByteStringType }
func (b *NeoVMByteString) ToBytes() []byte { return b.Value }
func (b *NeoVMByteString) String() string  { return hex.EncodeToString(b.Value) }
func (b *NeoVMByteString) Size() int       { return len(b.Value) }

func (b *NeoVMBuffer) Type() NeoVMType     { return BufferType }
func (b *NeoVMBuffer) ToBytes() []byte     { return b.Value }
func (b *NeoVMBuffer) String() string      { return hex.EncodeToString(b.Value) }
func (b *NeoVMBuffer) Size() int           { return len(b.Value) }

func (b *NeoVMBoolean) Type() NeoVMType    { return BooleanType }
func (b *NeoVMBoolean) ToBytes() []byte    { 
	if b.Value { 
		return []byte{1} 
	} 
	return []byte{0} 
}
func (b *NeoVMBoolean) String() string     { 
	if b.Value { 
		return "true" 
	} 
	return "false" 
}
func (b *NeoVMBoolean) Size() int          { return 1 }

func (a *NeoVMArray) Type() NeoVMType      { return ArrayType }
func (a *NeoVMArray) ToBytes() []byte      { 
	// Serialize array as concatenated items
	var result []byte
	for _, item := range a.Items {
		result = append(result, item.ToBytes()...)
	}
	return result
}
func (a *NeoVMArray) String() string       { 
	return fmt.Sprintf("Array[%d]", len(a.Items)) 
}
func (a *NeoVMArray) Size() int            {
	size := 4 // Array header
	for _, item := range a.Items {
		size += item.Size()
	}
	return size
}

func (s *NeoVMStruct) Type() NeoVMType     { return StructType }
func (s *NeoVMStruct) ToBytes() []byte     { 
	var result []byte
	for _, item := range s.Items {
		result = append(result, item.ToBytes()...)
	}
	return result
}
func (s *NeoVMStruct) String() string      { 
	return fmt.Sprintf("Struct[%d]", len(s.Items)) 
}
func (s *NeoVMStruct) Size() int           {
	size := 4 // Struct header
	for _, item := range s.Items {
		size += item.Size()
	}
	return size
}

func (m *NeoVMMap) Type() NeoVMType        { return MapType }
func (m *NeoVMMap) ToBytes() []byte        { 
	// Serialize map as key-value pairs
	var result []byte
	for key, value := range m.Items {
		result = append(result, []byte(key)...)
		result = append(result, value.ToBytes()...)
	}
	return result
}
func (m *NeoVMMap) String() string         { 
	return fmt.Sprintf("Map[%d]", len(m.Items)) 
}
func (m *NeoVMMap) Size() int              {
	size := 4 // Map header
	for key, value := range m.Items {
		size += len(key) + value.Size()
	}
	return size
}

func (p *NeoVMPointer) Type() NeoVMType    { return PointerType }
func (p *NeoVMPointer) ToBytes() []byte    { return p.Target.ToBytes() }
func (p *NeoVMPointer) String() string     { return fmt.Sprintf("Pointer -> %s", p.Target.String()) }
func (p *NeoVMPointer) Size() int          { return 8 } // Pointer size

func (i *NeoVMInterop) Type() NeoVMType    { return InteropType }
func (i *NeoVMInterop) ToBytes() []byte    { return []byte(i.Interface) }
func (i *NeoVMInterop) String() string     { return fmt.Sprintf("Interop: %s", i.Interface) }
func (i *NeoVMInterop) Size() int          { return len(i.Interface) + 16 } // Interface name + overhead

// NeoContract represents a compiled NeoVM contract
type NeoContract struct {
	// Contract metadata
	Name        string              `json:"name"`
	Version     string              `json:"version"`
	Author      string              `json:"author,omitempty"`
	Description string              `json:"description,omitempty"`

	// Compiled code sections
	Constructor []NeoInstruction    `json:"constructor"`
	Runtime     []NeoInstruction    `json:"runtime"`
	
	// Contract interface
	Methods     []*ContractMethod   `json:"methods"`
	Events      []*ContractEvent    `json:"events"`
	
	// Runtime information
	EntryPoints map[string]int      `json:"entry_points"`
	Constants   map[string]NeoVMStackItem `json:"constants"`
	Imports     []string            `json:"imports,omitempty"`
	
	// Debug and metadata
	SourceMap   map[int]SourcePosition `json:"source_map,omitempty"`
	Metadata    *ContractMetadata   `json:"metadata"`
}

// ContractMethod describes a contract method
type ContractMethod struct {
	Name       string              `json:"name"`
	Selector   [4]byte             `json:"selector"`
	Parameters []MethodParameter   `json:"parameters"`
	Returns    []MethodParameter   `json:"returns"`
	Offset     int                 `json:"offset"`
	Safe       bool                `json:"safe"`       // Read-only method
	Payable    bool                `json:"payable"`    // Can receive native tokens
}

// ContractEvent describes a contract event
type ContractEvent struct {
	Name       string              `json:"name"`
	Signature  string              `json:"signature"`
	Parameters []EventParameter    `json:"parameters"`
	Anonymous  bool                `json:"anonymous"`
}

// Method and event parameters
type MethodParameter struct {
	Name    string    `json:"name"`
	Type    string    `json:"type"`
	Indexed bool      `json:"indexed,omitempty"` // For events
}

type EventParameter struct {
	Name    string    `json:"name"`
	Type    string    `json:"type"`
	Indexed bool      `json:"indexed"`
}

// ContractMetadata stores additional contract information
type ContractMetadata struct {
	Compiler        CompilerInfo        `json:"compiler"`
	CompilationTime string              `json:"compilation_time"`
	SourceFiles     []string            `json:"source_files"`
	Libraries       []LibraryInfo       `json:"libraries,omitempty"`
	Optimization    OptimizationInfo    `json:"optimization"`
	Security        SecurityInfo        `json:"security"`
}

type LibraryInfo struct {
	Name    string `json:"name"`
	Version string `json:"version"`
	Hash    string `json:"hash"`
}

type OptimizationInfo struct {
	Enabled         bool    `json:"enabled"`
	Level           int     `json:"level"`
	Runs            int     `json:"runs"`
	SizeReduction   float64 `json:"size_reduction_percent"`
	GasOptimization float64 `json:"gas_optimization_percent"`
}

type SecurityInfo struct {
	SafetyChecks    bool     `json:"safety_checks"`
	BoundsChecking  bool     `json:"bounds_checking"`
	OverflowChecks  bool     `json:"overflow_checks"`
	Vulnerabilities []string `json:"vulnerabilities,omitempty"`
	AuditFindings   []string `json:"audit_findings,omitempty"`
}

// NeoVMExecutionEngine represents the execution environment
type NeoVMExecutionEngine struct {
	// Stack state
	EvaluationStack []NeoVMStackItem    `json:"evaluation_stack"`
	AltStack        []NeoVMStackItem    `json:"alt_stack"`
	
	// Execution state
	InstructionPointer int               `json:"instruction_pointer"`
	Instructions      []NeoInstruction   `json:"instructions"`
	
	// Contract state
	StaticFields      map[int]NeoVMStackItem `json:"static_fields"`
	LocalVariables    []NeoVMStackItem       `json:"local_variables"`
	
	// Execution limits
	GasLimit          int64             `json:"gas_limit"`
	GasConsumed       int64             `json:"gas_consumed"`
	StackLimit        int               `json:"stack_limit"`
	
	// Exception handling
	ExceptionHandlers []ExceptionHandler `json:"exception_handlers"`
	
	// Interop services
	InteropServices   map[string]func([]NeoVMStackItem) (NeoVMStackItem, error) `json:"-"`
}

// ExceptionHandler represents an exception handling frame
type ExceptionHandler struct {
	TryOffset     int    `json:"try_offset"`
	CatchOffset   int    `json:"catch_offset"`
	FinallyOffset int    `json:"finally_offset"`
	EndOffset     int    `json:"end_offset"`
	StackDepth    int    `json:"stack_depth"`
}

// Instruction creation helpers
func NewPushInstruction(value NeoVMStackItem) NeoInstruction {
	data := value.ToBytes()
	opcode := PUSHDATA1
	
	// Optimize for small integers
	if value.Type() == IntegerType {
		if intVal, ok := value.(*NeoVMInteger); ok {
			if intVal.Value.IsInt64() {
				val := intVal.Value.Int64()
				if val >= 0 && val <= 16 {
					return NeoInstruction{
						Opcode:    NeoOpcode(0x10 + val), // PUSH0-PUSH16
						Operand:   nil,
						Size:      1,
						StackPop:  0,
						StackPush: 1,
						GasCost:   1,
					}
				}
			}
		}
	}
	
	// Determine appropriate PUSH instruction based on data size
	if len(data) <= 255 {
		opcode = PUSHDATA1
	} else if len(data) <= 65535 {
		opcode = PUSHDATA2
	} else {
		opcode = PUSHDATA4
	}
	
	return NeoInstruction{
		Opcode:    opcode,
		Operand:   data,
		Size:      1 + len(data) + getSizeByteCount(opcode),
		StackPop:  0,
		StackPush: 1,
		GasCost:   int64(1 + len(data)/32), // Base cost + data cost
	}
}

func NewArithmeticInstruction(op NeoOpcode) NeoInstruction {
	var stackPop, stackPush int
	var gasCost int64
	
	switch op {
	case ADD, SUB, MUL, DIV, MOD, AND, OR, XOR:
		stackPop, stackPush = 2, 1
		gasCost = 8
	case NOT, BOOLAND, BOOLOR:
		stackPop, stackPush = 1, 1
		gasCost = 4
	case SHL, SHR:
		stackPop, stackPush = 2, 1
		gasCost = 8
	case LT, LE, GT, GE, EQUAL, NOTEQUAL, NUMEQUAL, NUMNOTEQUAL:
		stackPop, stackPush = 2, 1
		gasCost = 8
	case MIN, MAX:
		stackPop, stackPush = 2, 1
		gasCost = 8
	case WITHIN:
		stackPop, stackPush = 3, 1
		gasCost = 16
	default:
		stackPop, stackPush = 0, 0
		gasCost = 1
	}
	
	return NeoInstruction{
		Opcode:    op,
		Operand:   nil,
		Size:      1,
		StackPop:  stackPop,
		StackPush: stackPush,
		GasCost:   gasCost,
	}
}

func NewControlFlowInstruction(op NeoOpcode, target int) NeoInstruction {
	var operand []byte
	var stackPop int
	var gasCost int64
	
	switch op {
	case JMP:
		stackPop = 0
		gasCost = 2
	case JMPIF, JMPIFNOT:
		stackPop = 1
		gasCost = 2
	case CALL, CALLA:
		stackPop = 0
		gasCost = 512
	case RET:
		stackPop = 0
		gasCost = 0
	default:
		stackPop = 0
		gasCost = 1
	}
	
	// Encode target address (4 bytes for NeoVM)
	if target != 0 {
		operand = make([]byte, 4)
		operand[0] = byte(target)
		operand[1] = byte(target >> 8)
		operand[2] = byte(target >> 16)
		operand[3] = byte(target >> 24)
	}
	
	return NeoInstruction{
		Opcode:    op,
		Operand:   operand,
		Size:      1 + len(operand),
		StackPop:  stackPop,
		StackPush: 0,
		GasCost:   gasCost,
	}
}

func NewStackInstruction(op NeoOpcode, depth int) NeoInstruction {
	var operand []byte
	var stackPop, stackPush int
	var gasCost int64
	
	switch op {
	case DUP:
		stackPop, stackPush = 0, 1
		gasCost = 2
	case SWAP:
		stackPop, stackPush = 2, 2
		gasCost = 2
	case ROT, ROLL:
		stackPop, stackPush = 3, 3
		gasCost = 2
	case PICK:
		stackPop, stackPush = 0, 1
		gasCost = 2
	case TUCK:
		stackPop, stackPush = 2, 3
		gasCost = 2
	case DROP, NIP:
		stackPop, stackPush = 1, 0
		gasCost = 2
	case XDROP:
		stackPop, stackPush = 2, 0
		gasCost = 16
	case CLEAR:
		stackPop, stackPush = 0, 0 // Clears entire stack
		gasCost = 16
	case DEPTH:
		stackPop, stackPush = 0, 1
		gasCost = 2
	default:
		stackPop, stackPush = 0, 0
		gasCost = 1
	}
	
	// Some instructions need depth parameter
	if depth > 0 {
		operand = []byte{byte(depth)}
	}
	
	return NeoInstruction{
		Opcode:    op,
		Operand:   operand,
		Size:      1 + len(operand),
		StackPop:  stackPop,
		StackPush: stackPush,
		GasCost:   gasCost,
	}
}

func NewSyscallInstruction(method string) NeoInstruction {
	methodBytes := []byte(method)
	
	return NeoInstruction{
		Opcode:    SYSCALL,
		Operand:   methodBytes,
		Size:      1 + len(methodBytes),
		StackPop:  0, // Variable based on syscall
		StackPush: 0, // Variable based on syscall  
		GasCost:   1024, // Base syscall cost
		Comment:   fmt.Sprintf("SYSCALL %s", method),
	}
}

// Helper functions
func getSizeByteCount(opcode NeoOpcode) int {
	switch opcode {
	case PUSHDATA1:
		return 1
	case PUSHDATA2:
		return 2
	case PUSHDATA4:
		return 4
	default:
		return 0
	}
}

// OpcodeMnemonic returns the string representation of an opcode
func OpcodeMnemonic(op NeoOpcode) string {
	switch op {
	case PUSH0: return "PUSH0"
	case PUSH1: return "PUSH1"
	case PUSH2: return "PUSH2"
	case PUSH3: return "PUSH3"
	case PUSH4: return "PUSH4"
	case PUSH5: return "PUSH5"
	case PUSH6: return "PUSH6"
	case PUSH7: return "PUSH7"
	case PUSH8: return "PUSH8"
	case PUSH9: return "PUSH9"
	case PUSH10: return "PUSH10"
	case PUSH11: return "PUSH11"
	case PUSH12: return "PUSH12"
	case PUSH13: return "PUSH13"
	case PUSH14: return "PUSH14"
	case PUSH15: return "PUSH15"
	case PUSH16: return "PUSH16"
	case PUSHDATA1: return "PUSHDATA1"
	case PUSHDATA2: return "PUSHDATA2"
	case PUSHDATA4: return "PUSHDATA4"
	case DUP: return "DUP"
	case SWAP: return "SWAP"
	case ROT: return "ROT"
	case ROLL: return "ROLL"
	case PICK: return "PICK"
	case TUCK: return "TUCK"
	case DROP: return "DROP"
	case NIP: return "NIP"
	case XDROP: return "XDROP"
	case CLEAR: return "CLEAR"
	case DEPTH: return "DEPTH"
	case ADD: return "ADD"
	case SUB: return "SUB"
	case MUL: return "MUL"
	case DIV: return "DIV"
	case MOD: return "MOD"
	case SHL: return "SHL"
	case SHR: return "SHR"
	case NOT: return "NOT"
	case BOOLAND: return "BOOLAND"
	case BOOLOR: return "BOOLOR"
	case NUMEQUAL: return "NUMEQUAL"
	case NUMNOTEQUAL: return "NUMNOTEQUAL"
	case LT: return "LT"
	case LE: return "LE"
	case GT: return "GT"
	case GE: return "GE"
	case MIN: return "MIN"
	case MAX: return "MAX"
	case WITHIN: return "WITHIN"
	case AND: return "AND"
	case OR: return "OR"
	case XOR: return "XOR"
	case EQUAL: return "EQUAL"
	case NOTEQUAL: return "NOTEQUAL"
	case JMP: return "JMP"
	case JMPIF: return "JMPIF"
	case JMPIFNOT: return "JMPIFNOT"
	case CALL: return "CALL"
	case RET: return "RET"
	case SYSCALL: return "SYSCALL"
	case NEWARRAY: return "NEWARRAY"
	case NEWSTRUCT: return "NEWSTRUCT"
	case NEWMAP: return "NEWMAP"
	case APPEND: return "APPEND"
	case REVERSE: return "REVERSE"
	case REMOVE: return "REMOVE"
	case HASKEY: return "HASKEY"
	case KEYS: return "KEYS"
	case VALUES: return "VALUES"
	case PICKITEM: return "PICKITEM"
	case SETITEM: return "SETITEM"
	case SIZE: return "SIZE"
	case ISNULL: return "ISNULL"
	case ISTYPE: return "ISTYPE"
	case CONVERT: return "CONVERT"
	case ABORT: return "ABORT"
	case ASSERT: return "ASSERT"
	case THROW: return "THROW"
	case TRY: return "TRY"
	case ENDTRY: return "ENDTRY"
	case ENDFINALLY: return "ENDFINALLY"
	default:
		return fmt.Sprintf("UNKNOWN(0x%02X)", byte(op))
	}
}

// CreateNeoVMInteger creates a NeoVM integer from various input types
func CreateNeoVMInteger(value interface{}) *NeoVMInteger {
	switch v := value.(type) {
	case int:
		return &NeoVMInteger{Value: big.NewInt(int64(v))}
	case int64:
		return &NeoVMInteger{Value: big.NewInt(v)}
	case *big.Int:
		return &NeoVMInteger{Value: v}
	case string:
		val, _ := new(big.Int).SetString(v, 10)
		return &NeoVMInteger{Value: val}
	default:
		return &NeoVMInteger{Value: big.NewInt(0)}
	}
}

// CreateNeoVMByteString creates a NeoVM byte string from bytes or string
func CreateNeoVMByteString(value interface{}) *NeoVMByteString {
	switch v := value.(type) {
	case []byte:
		return &NeoVMByteString{Value: v}
	case string:
		return &NeoVMByteString{Value: []byte(v)}
	default:
		return &NeoVMByteString{Value: []byte{}}
	}
}

// CreateNeoVMBoolean creates a NeoVM boolean
func CreateNeoVMBoolean(value bool) *NeoVMBoolean {
	return &NeoVMBoolean{Value: value}
}

// CreateNeoVMArray creates a NeoVM array from a slice of items
func CreateNeoVMArray(items []NeoVMStackItem) *NeoVMArray {
	return &NeoVMArray{Items: items}
}