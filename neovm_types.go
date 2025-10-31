package main

import (
    "crypto/sha256"
    "encoding/binary"
    "encoding/hex"
    "fmt"
    "math/big"
)

// syscallSig defines stack effects for known syscalls: pops, pushes
var syscallSig = map[string]struct{ pop, push int }{
    // Storage
    "System.Storage.Get":               {pop: 2, push: 1}, // context, key -> value
    "System.Storage.Put":               {pop: 3, push: 0}, // context, key, value
    "System.Storage.Delete":            {pop: 2, push: 0},
    "System.Storage.GetContext":        {pop: 0, push: 1},
    "System.Storage.GetReadOnlyContext":{pop: 0, push: 1},
    // Runtime
    "System.Runtime.GetCallingScriptHash":   {pop: 0, push: 1},
    "System.Runtime.GetEntryScriptHash":     {pop: 0, push: 1},
    "System.Runtime.GetExecutingScriptHash": {pop: 0, push: 1},
    "System.Runtime.GetInvocationCounter":   {pop: 0, push: 1},
    "System.Runtime.Log":                    {pop: 1, push: 0}, // string
    "System.Runtime.Notify":                 {pop: 1, push: 0}, // any[]
    // Crypto
    "Neo.Crypto.Sha256":      {pop: 1, push: 1},
    "Neo.Crypto.RIPEMD160":   {pop: 1, push: 1},
    "Neo.Crypto.Keccak256":   {pop: 1, push: 1},
    // Contract
    "System.Contract.Call":   {pop: 4, push: 1}, // scriptHash, method, flags, params[] -> result
}

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
	// Push / constant instructions
	PUSHINT8   NeoOpcode = 0x00
	PUSHINT16  NeoOpcode = 0x01
	PUSHINT32  NeoOpcode = 0x02
	PUSHINT64  NeoOpcode = 0x03
	PUSHINT128 NeoOpcode = 0x04
	PUSHINT256 NeoOpcode = 0x05
	PUSHA      NeoOpcode = 0x0A
	PUSHNULL   NeoOpcode = 0x0B
	PUSHDATA1  NeoOpcode = 0x0C
	PUSHDATA2  NeoOpcode = 0x0D
	PUSHDATA4  NeoOpcode = 0x0E
	PUSHM1     NeoOpcode = 0x0F
	PUSH0      NeoOpcode = 0x10
	PUSH1      NeoOpcode = 0x11
	PUSH2      NeoOpcode = 0x12
	PUSH3      NeoOpcode = 0x13
	PUSH4      NeoOpcode = 0x14
	PUSH5      NeoOpcode = 0x15
	PUSH6      NeoOpcode = 0x16
	PUSH7      NeoOpcode = 0x17
	PUSH8      NeoOpcode = 0x18
	PUSH9      NeoOpcode = 0x19
	PUSH10     NeoOpcode = 0x1A
	PUSH11     NeoOpcode = 0x1B
	PUSH12     NeoOpcode = 0x1C
	PUSH13     NeoOpcode = 0x1D
	PUSH14     NeoOpcode = 0x1E
	PUSH15     NeoOpcode = 0x1F
	PUSH16     NeoOpcode = 0x20

	// Stack manipulation
	DEPTH NeoOpcode = 0x43
	DROP  NeoOpcode = 0x45
	NIP   NeoOpcode = 0x46
	XDROP NeoOpcode = 0x48
	CLEAR NeoOpcode = 0x49
	DUP   NeoOpcode = 0x4A
	PICK  NeoOpcode = 0x4D
	TUCK  NeoOpcode = 0x4E
	SWAP  NeoOpcode = 0x50
	ROT   NeoOpcode = 0x51
	ROLL  NeoOpcode = 0x52

	// Arithmetic
	ADD         NeoOpcode = 0x95
	SUB         NeoOpcode = 0x96
	MUL         NeoOpcode = 0x97
	DIV         NeoOpcode = 0x98
	MOD         NeoOpcode = 0x99
	SHL         NeoOpcode = 0x9E
	SHR         NeoOpcode = 0x9F
	NOT         NeoOpcode = 0xA0
	BOOLAND     NeoOpcode = 0xA1
	BOOLOR      NeoOpcode = 0xA2
	NUMEQUAL    NeoOpcode = 0xA3
	NUMNOTEQUAL NeoOpcode = 0xA4
	LT          NeoOpcode = 0xA5
	LE          NeoOpcode = 0xA6
	GT          NeoOpcode = 0xA7
	GE          NeoOpcode = 0xA8
	MIN         NeoOpcode = 0xA9
	MAX         NeoOpcode = 0xAA
	WITHIN      NeoOpcode = 0xAB

	// Aliases for higher-level operations
	AND      NeoOpcode = BOOLAND
	OR       NeoOpcode = BOOLOR
	XOR      NeoOpcode = NUMNOTEQUAL // Placeholder until dedicated XOR opcode is exposed
	EQUAL    NeoOpcode = NUMEQUAL
	NOTEQUAL NeoOpcode = NUMNOTEQUAL

	// Control flow
	NOP        NeoOpcode = 0x21
	JMP        NeoOpcode = 0x22
	JMPIF      NeoOpcode = 0x23
	JMPIFNOT   NeoOpcode = 0x24
	CALL       NeoOpcode = 0x2B
	CALLA      NeoOpcode = 0x2C
	CALLT      NeoOpcode = 0x2D
	ABORT      NeoOpcode = 0x2E
	ASSERT     NeoOpcode = 0x2F
	RET        NeoOpcode = 0x40
	SYSCALL    NeoOpcode = 0x41
	THROW      NeoOpcode = 0x3A
	TRY        NeoOpcode = 0x3B
	TRYFINALLY NeoOpcode = 0x3C
	ENDTRY     NeoOpcode = 0x3D
	ENDFINALLY NeoOpcode = 0x3E

	// Array and buffer operations
	PICKITEM  NeoOpcode = 0xC2
	SETITEM   NeoOpcode = 0xC3
	NEWARRAY  NeoOpcode = 0xC5
	NEWSTRUCT NeoOpcode = 0xC8
	NEWMAP    NeoOpcode = 0xC9
	SIZE      NeoOpcode = 0xCA
	HASKEY    NeoOpcode = 0xCB
	KEYS      NeoOpcode = 0xCC
	VALUES    NeoOpcode = 0xCD
	APPEND    NeoOpcode = 0xD0
	REMOVE    NeoOpcode = 0xD3

	// Type operations
	ISNULL   NeoOpcode = 0xD8
	ISTYPE   NeoOpcode = 0xD9
	CONVERT  NeoOpcode = 0xDB

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

	// Optimize for small integers that have dedicated opcodes in Neo N3
	if value.Type() == IntegerType {
		if intVal, ok := value.(*NeoVMInteger); ok {
			if intVal.Value.IsInt64() {
				val := intVal.Value.Int64()
				switch {
				case val == -1:
					return NeoInstruction{
						Opcode:    PUSHM1,
						Operand:   nil,
						Size:      1,
						StackPop:  0,
						StackPush: 1,
						GasCost:   1,
					}
				case val >= 0 && val <= 16:
					op := NeoOpcode(byte(PUSH0) + byte(val))
					return NeoInstruction{
						Opcode:    op,
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
	var (
		operand  []byte
		stackPop int
		gasCost  int64 = 1
	)

	switch op {
	case JMP:
		stackPop = 0
		gasCost = 3
	case JMPIF, JMPIFNOT:
		stackPop = 1
		gasCost = 3
	case CALL, CALLA, CALLT:
		stackPop = 0
		gasCost = 10
	case RET:
		stackPop = 0
		gasCost = 1
	case ABORT:
		stackPop = 0
		gasCost = 1
	}

	switch op {
	case JMP, JMPIF, JMPIFNOT, CALL, CALLA, CALLT, TRY, TRYFINALLY, ENDTRY, ENDFINALLY:
		operand = make([]byte, 4)
		if target != 0 {
			binary.LittleEndian.PutUint32(operand, uint32(target))
		}
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
    // Neo N3 interop ID: first 4 bytes of SHA-256(method), little-endian order
    sum := sha256.Sum256([]byte(method))
    interopID := make([]byte, 4)
    // Copy little-endian 4-byte prefix as-is
    copy(interopID, sum[:4])

    // For clarity, compute the uint32 value (unused but documents intent)
    _ = binary.LittleEndian.Uint32(interopID)

    sig := syscallSig[method]
    return NeoInstruction{
        Opcode:    SYSCALL,
        Operand:   interopID,
        Size:      1 + len(interopID),
        StackPop:  sig.pop,
        StackPush: sig.push,
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
var opcodeNames = map[NeoOpcode]string{
	PUSHINT8:   "PUSHINT8",
	PUSHINT16:  "PUSHINT16",
	PUSHINT32:  "PUSHINT32",
	PUSHINT64:  "PUSHINT64",
	PUSHINT128: "PUSHINT128",
	PUSHINT256: "PUSHINT256",
	PUSHDATA1:  "PUSHDATA1",
	PUSHDATA2:  "PUSHDATA2",
	PUSHDATA4:  "PUSHDATA4",
	PUSHM1:     "PUSHM1",
	PUSH0:      "PUSH0",
	PUSH1:      "PUSH1",
	PUSH2:      "PUSH2",
	PUSH3:      "PUSH3",
	PUSH4:      "PUSH4",
	PUSH5:      "PUSH5",
	PUSH6:      "PUSH6",
	PUSH7:      "PUSH7",
	PUSH8:      "PUSH8",
	PUSH9:      "PUSH9",
	PUSH10:     "PUSH10",
	PUSH11:     "PUSH11",
	PUSH12:     "PUSH12",
	PUSH13:     "PUSH13",
	PUSH14:     "PUSH14",
	PUSH15:     "PUSH15",
	PUSH16:     "PUSH16",
	DEPTH:      "DEPTH",
	DROP:       "DROP",
	NIP:        "NIP",
	XDROP:      "XDROP",
	CLEAR:      "CLEAR",
	DUP:        "DUP",
	PICK:       "PICK",
	TUCK:       "TUCK",
	SWAP:       "SWAP",
	ROT:        "ROT",
	ROLL:       "ROLL",
	ADD:        "ADD",
	SUB:        "SUB",
	MUL:        "MUL",
	DIV:        "DIV",
	MOD:        "MOD",
	SHL:        "SHL",
	SHR:        "SHR",
	NOT:        "NOT",
	BOOLAND:    "BOOLAND",
	BOOLOR:     "BOOLOR",
	NUMEQUAL:   "NUMEQUAL",
	NUMNOTEQUAL:"NUMNOTEQUAL",
	LT:         "LT",
	LE:         "LE",
	GT:         "GT",
	GE:         "GE",
	MIN:        "MIN",
	MAX:        "MAX",
	WITHIN:     "WITHIN",
	JMP:        "JMP",
	JMPIF:      "JMPIF",
	JMPIFNOT:   "JMPIFNOT",
	CALL:       "CALL",
	RET:        "RET",
	SYSCALL:    "SYSCALL",
	NEWARRAY:   "NEWARRAY",
	NEWSTRUCT:  "NEWSTRUCT",
	NEWMAP:     "NEWMAP",
	APPEND:     "APPEND",
	REMOVE:     "REMOVE",
	HASKEY:     "HASKEY",
	KEYS:       "KEYS",
	VALUES:     "VALUES",
	PICKITEM:   "PICKITEM",
	SETITEM:    "SETITEM",
	SIZE:       "SIZE",
	ISNULL:     "ISNULL",
	ISTYPE:     "ISTYPE",
	CONVERT:    "CONVERT",
	ABORT:      "ABORT",
	ASSERT:     "ASSERT",
	THROW:      "THROW",
	TRY:        "TRY",
	ENDTRY:     "ENDTRY",
	ENDFINALLY: "ENDFINALLY",
}

func OpcodeMnemonic(op NeoOpcode) string {
	if name, ok := opcodeNames[op]; ok {
		return name
	}
	return fmt.Sprintf("UNKNOWN(0x%02X)", byte(op))
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
