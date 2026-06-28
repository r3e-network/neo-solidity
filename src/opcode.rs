//! Canonical Neo N3 VM opcode definitions.
//!
//! The variant names follow the canonical Neo N3 opcode spelling
//! (e.g. `JMP_L`, `JMPIF_L`, `NEWARRAY_T`, `LDSFLD0`..`LDSFLD6`,
//! `PUSH0`..`PUSH16`). These deliberately mix digits, underscores,
//! and uppercase letters to match the spec, so we silence the
//! camel-case lint locally instead of mangling the names.
#![allow(non_camel_case_types)]
//!
//! This module is the **single source of truth** for every NeoVM opcode
//! byte value used anywhere in the compiler, runtime simulator, and
//! disassembler. Code that emits or interprets NeoVM bytecode must go
//! through [`OpCode`] rather than hardcoding byte literals — that way
//! renumbering, validation, gas accounting, and disassembler output
//! stay in lockstep with the spec.
//!
//! # Layout
//!
//! Opcodes fall into two categories:
//!
//! * **Fixed** opcodes ([`OpCode::ADD`], [`OpCode::JMP`], …) have a
//!   single, named variant and a single byte value.
//! * **Indexed** opcode families ([`OpCode::PUSH0`]..[`OpCode::PUSH16`],
//!   [`OpCode::LDLOC0`]..[`OpCode::LDLOC6`], [`OpCode::LDLOC`],
//!   [`OpCode::LDSFLD0`].., [`OpCode::LDARG0`].., [`OpCode::STLOC0`]..,
//!   [`OpCode::STSFLD0`].., [`OpCode::STARG0`]..) share a byte
//!   prefix/suffix. Use the [`OpCode::push_small`], [`OpCode::ldloc`],
//!   [`OpCode::stloc`], [`OpCode::ldarg`], [`OpCode::starg`],
//!   [`OpCode::ldsfld`], [`OpCode::stsfld`] constructors instead of
//!   computing `0x10 + n`.
//!
//! # Variants for data-bearing opcodes
//!
//! [`OpCode::PUSHDATA1`] / [`OpCode::PUSHDATA2`] / [`OpCode::PUSHDATA4`]
//! and [`OpCode::PUSHINT8`]..[`OpCode::PUSHINT256`] are all distinct
//! opcodes that the VM dispatches on. [`OpCode::push_data`] and
//! [`OpCode::push_int`] select the right one based on operand size.
//!
//! # Usage
//!
//! ```
//! use neo_devpack_solidity::opcode::OpCode;
//!
//! let mut script = Vec::new();
//! script.push(OpCode::ABORT.byte());
//! script.push(OpCode::PUSH0.byte());
//! assert_eq!(script, vec![0x38, 0x10]);
//! ```

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OpCode {
    // === Constants / Push (0x00–0x20) ===
    /// Push a signed 8-bit integer.
    PUSHINT8 = 0x00,
    /// Push a signed 16-bit integer.
    PUSHINT16 = 0x01,
    /// Push a signed 32-bit integer.
    PUSHINT32 = 0x02,
    /// Push a signed 64-bit integer.
    PUSHINT64 = 0x03,
    /// Push a signed 128-bit integer.
    PUSHINT128 = 0x04,
    /// Push a signed 256-bit integer.
    PUSHINT256 = 0x05,
    /// Push `true`.
    PUSHT = 0x08,
    /// Push `false`.
    PUSHF = 0x09,
    /// Push the current instruction pointer's address.
    PUSHA = 0x0A,
    /// Push `null`.
    PUSHNULL = 0x0B,
    /// Push a `≤ u8::MAX` byte string with a 1-byte length prefix.
    PUSHDATA1 = 0x0C,
    /// Push a `≤ u16::MAX` byte string with a 2-byte length prefix.
    PUSHDATA2 = 0x0D,
    /// Push a `≤ u32::MAX` byte string with a 4-byte length prefix.
    PUSHDATA4 = 0x0E,
    /// Push `-1` as an integer.
    PUSHM1 = 0x0F,
    /// Push the integer `0` (also `PUSH0`).
    PUSH0 = 0x10,
    /// Push the integer `1`.
    PUSH1 = 0x11,
    /// Push the integer `2`.
    PUSH2 = 0x12,
    /// Push the integer `3`.
    PUSH3 = 0x13,
    /// Push the integer `4`.
    PUSH4 = 0x14,
    /// Push the integer `5`.
    PUSH5 = 0x15,
    /// Push the integer `6`.
    PUSH6 = 0x16,
    /// Push the integer `7`.
    PUSH7 = 0x17,
    /// Push the integer `8`.
    PUSH8 = 0x18,
    /// Push the integer `9`.
    PUSH9 = 0x19,
    /// Push the integer `10`.
    PUSH10 = 0x1A,
    /// Push the integer `11`.
    PUSH11 = 0x1B,
    /// Push the integer `12`.
    PUSH12 = 0x1C,
    /// Push the integer `13`.
    PUSH13 = 0x1D,
    /// Push the integer `14`.
    PUSH14 = 0x1E,
    /// Push the integer `15`.
    PUSH15 = 0x1F,
    /// Push the integer `16`.
    PUSH16 = 0x20,

    // === Flow control (0x21–0x41) ===
    /// No operation.
    NOP = 0x21,
    /// Unconditional relative jump (1-byte offset).
    JMP = 0x22,
    /// Unconditional relative jump (4-byte offset).
    JMP_L = 0x23,
    /// Conditional jump if top of stack is truthy (1-byte offset).
    JMPIF = 0x24,
    /// Conditional jump if top of stack is truthy (4-byte offset).
    JMPIF_L = 0x25,
    /// Conditional jump if top of stack is falsy (1-byte offset).
    JMPIFNOT = 0x26,
    /// Conditional jump if top of stack is falsy (4-byte offset).
    JMPIFNOT_L = 0x27,
    /// Jump if the top two stack items are equal (1-byte offset).
    JMPEQ = 0x28,
    /// Jump if the top two stack items are equal (4-byte offset).
    JMPEQ_L = 0x29,
    /// Jump if the top two stack items are not equal (1-byte offset).
    JMPNE = 0x2A,
    /// Jump if the top two stack items are not equal (4-byte offset).
    JMPNE_L = 0x2B,
    /// Jump if the second item is greater than the top (1-byte offset).
    JMPGT = 0x2C,
    /// Jump if the second item is greater than the top (4-byte offset).
    JMPGT_L = 0x2D,
    /// Jump if the second item is greater than or equal to the top (1-byte offset).
    JMPGE = 0x2E,
    /// Jump if the second item is greater than or equal to the top (4-byte offset).
    JMPGE_L = 0x2F,
    /// Jump if the second item is less than the top (1-byte offset).
    JMPLT = 0x30,
    /// Jump if the second item is less than the top (4-byte offset).
    JMPLT_L = 0x31,
    /// Jump if the second item is less than or equal to the top (1-byte offset).
    JMPLE = 0x32,
    /// Jump if the second item is less than or equal to the top (4-byte offset).
    JMPLE_L = 0x33,
    /// Call the address at the top of the stack (1-byte offset).
    CALL = 0x34,
    /// Call the address at the top of the stack (4-byte offset).
    CALL_L = 0x35,
    /// Call the address on top of the stack as a tail call.
    CALLA = 0x36,
    /// Call a typed method by 2-byte index into the current script's method table.
    CALLT = 0x37,
    /// Abort execution.
    ABORT = 0x38,
    /// Assert the top of the stack is truthy, otherwise abort.
    ASSERT = 0x39,
    /// Throw the top of the stack as an exception.
    THROW = 0x3A,
    /// Try/catch (1-byte offset).
    TRY = 0x3B,
    /// Try/catch (4-byte offset).
    TRY_L = 0x3C,
    /// End of a `try` block (1-byte offset).
    ENDTRY = 0x3D,
    /// End of a `try` block (4-byte offset).
    ENDTRY_L = 0x3E,
    /// End of a `finally` block.
    ENDFINALLY = 0x3F,
    /// Return from the current execution context.
    RET = 0x40,
    /// Invoke a system call identified by the next 4 bytes (interop ID).
    SYSCALL = 0x41,

    // === Stack ops (0x43–0x55) ===
    /// Push the current evaluation stack depth.
    DEPTH = 0x43,
    /// Drop the top of the stack.
    DROP = 0x45,
    /// Remove the second-to-top stack item.
    NIP = 0x46,
    /// Drop the stack item at the given 1-byte index.
    XDROP = 0x48,
    /// Clear the evaluation stack.
    CLEAR = 0x49,
    /// Duplicate the top of the stack.
    DUP = 0x4A,
    /// Copy the second-to-top stack item to the top.
    OVER = 0x4B,
    /// Copy the item at the given 1-byte index to the top.
    PICK = 0x4D,
    /// Copy the top of the stack to the position below the second item.
    TUCK = 0x4E,
    /// Swap the top two stack items.
    SWAP = 0x50,
    /// Rotate the top three stack items.
    ROT = 0x51,
    /// Roll the top `n+1` stack items by 1 (1-byte count).
    ROLL = 0x52,
    /// Reverse the order of the top 3 stack items.
    REVERSE3 = 0x53,
    /// Reverse the order of the top 4 stack items.
    REVERSE4 = 0x54,
    /// Reverse the order of the top `n+1` stack items (1-byte count).
    REVERSEN = 0x55,

    // === Slot / local / argument ops (0x56–0x87) ===
    /// Initialize static field slots (1-byte count).
    INITSSLOT = 0x56,
    /// Initialize static-field + local + argument slots (3 1-byte counts).
    INITSLOT = 0x57,
    /// Load static field 0.
    LDSFLD0 = 0x58,
    /// Load static field 1.
    LDSFLD1 = 0x59,
    /// Load static field 2.
    LDSFLD2 = 0x5A,
    /// Load static field 3.
    LDSFLD3 = 0x5B,
    /// Load static field 4.
    LDSFLD4 = 0x5C,
    /// Load static field 5.
    LDSFLD5 = 0x5D,
    /// Load static field 6.
    LDSFLD6 = 0x5E,
    /// Load static field by 1-byte index.
    LDSFLD = 0x5F,
    /// Store into static field 0.
    STSFLD0 = 0x60,
    /// Store into static field 1.
    STSFLD1 = 0x61,
    /// Store into static field 2.
    STSFLD2 = 0x62,
    /// Store into static field 3.
    STSFLD3 = 0x63,
    /// Store into static field 4.
    STSFLD4 = 0x64,
    /// Store into static field 5.
    STSFLD5 = 0x65,
    /// Store into static field 6.
    STSFLD6 = 0x66,
    /// Store into static field by 1-byte index.
    STSFLD = 0x67,
    /// Load local variable 0.
    LDLOC0 = 0x68,
    /// Load local variable 1.
    LDLOC1 = 0x69,
    /// Load local variable 2.
    LDLOC2 = 0x6A,
    /// Load local variable 3.
    LDLOC3 = 0x6B,
    /// Load local variable 4.
    LDLOC4 = 0x6C,
    /// Load local variable 5.
    LDLOC5 = 0x6D,
    /// Load local variable 6.
    LDLOC6 = 0x6E,
    /// Load local variable by 1-byte index.
    LDLOC = 0x6F,
    /// Store into local variable 0.
    STLOC0 = 0x70,
    /// Store into local variable 1.
    STLOC1 = 0x71,
    /// Store into local variable 2.
    STLOC2 = 0x72,
    /// Store into local variable 3.
    STLOC3 = 0x73,
    /// Store into local variable 4.
    STLOC4 = 0x74,
    /// Store into local variable 5.
    STLOC5 = 0x75,
    /// Store into local variable 6.
    STLOC6 = 0x76,
    /// Store into local variable by 1-byte index.
    STLOC = 0x77,
    /// Load function argument 0.
    LDARG0 = 0x78,
    /// Load function argument 1.
    LDARG1 = 0x79,
    /// Load function argument 2.
    LDARG2 = 0x7A,
    /// Load function argument 3.
    LDARG3 = 0x7B,
    /// Load function argument 4.
    LDARG4 = 0x7C,
    /// Load function argument 5.
    LDARG5 = 0x7D,
    /// Load function argument 6.
    LDARG6 = 0x7E,
    /// Load function argument by 1-byte index.
    LDARG = 0x7F,
    /// Store into function argument 0.
    STARG0 = 0x80,
    /// Store into function argument 1.
    STARG1 = 0x81,
    /// Store into function argument 2.
    STARG2 = 0x82,
    /// Store into function argument 3.
    STARG3 = 0x83,
    /// Store into function argument 4.
    STARG4 = 0x84,
    /// Store into function argument 5.
    STARG5 = 0x85,
    /// Store into function argument 6.
    STARG6 = 0x86,
    /// Store into function argument by 1-byte index.
    STARG = 0x87,

    // === Buffer / string (0x88–0x8E) ===
    /// Allocate a new empty buffer.
    NEWBUFFER = 0x88,
    /// Copy `count` bytes from one buffer to another (3 1-byte operands).
    MEMCPY = 0x89,
    /// Concatenate two byte strings.
    CAT = 0x8B,
    /// Substring (2 1-byte operands).
    SUBSTR = 0x8C,
    /// Left-most bytes (1-byte count).
    LEFT = 0x8D,
    /// Right-most bytes (1-byte count).
    RIGHT = 0x8E,

    // === Bitwise / logic (0x90–0x98) ===
    /// Bitwise NOT (one's complement) on the top of the stack.
    INVERT = 0x90,
    /// Bitwise AND of the top two stack items.
    AND = 0x91,
    /// Bitwise OR of the top two stack items.
    OR = 0x92,
    /// Bitwise XOR of the top two stack items.
    XOR = 0x93,
    /// Logical equality of the top two stack items.
    EQUAL = 0x97,
    /// Logical inequality of the top two stack items.
    NOTEQUAL = 0x98,

    // === Numeric (0x99–0xAC) ===
    /// Push `-1`, `0`, or `1` according to the sign of the top of the stack.
    SIGN = 0x99,
    /// Replace the top of the stack with its absolute value.
    ABS = 0x9A,
    /// Negate the top of the stack.
    NEGATE = 0x9B,
    /// Increment the top of the stack by 1.
    INC = 0x9C,
    /// Decrement the top of the stack by 1.
    DEC = 0x9D,
    /// Add the top two stack items.
    ADD = 0x9E,
    /// Subtract the top of the stack from the second item.
    SUB = 0x9F,
    /// Multiply the top two stack items.
    MUL = 0xA0,
    /// Integer divide the second item by the top.
    DIV = 0xA1,
    /// Integer modulo of the second item by the top.
    MOD = 0xA2,
    /// Raise the second item to the power of the top.
    POW = 0xA3,
    /// Integer square root of the top of the stack.
    SQRT = 0xA4,
    /// Modular multiplication (operand order: bottom × top % modulus).
    MODMUL = 0xA5,
    /// Modular exponentiation.
    MODPOW = 0xA6,
    /// Shift the second item left by the top.
    SHL = 0xA8,
    /// Shift the second item right by the top.
    SHR = 0xA9,
    /// Logical NOT of the top of the stack.
    NOT = 0xAA,
    /// Boolean AND of the top two stack items.
    BOOLAND = 0xAB,
    /// Boolean OR of the top two stack items.
    BOOLOR = 0xAC,

    // === Comparisons (0xB1–0xBB) ===
    /// Push `true` if the top of the stack is non-zero.
    NZ = 0xB1,
    /// Numeric equality of the top two stack items.
    NUMEQUAL = 0xB3,
    /// Numeric inequality of the top two stack items.
    NUMNOTEQUAL = 0xB4,
    /// Push `true` if the second item is less than the top.
    LT = 0xB5,
    /// Push `true` if the second item is less than or equal to the top.
    LE = 0xB6,
    /// Push `true` if the second item is greater than the top.
    GT = 0xB7,
    /// Push `true` if the second item is greater than or equal to the top.
    GE = 0xB8,
    /// Push the minimum of the top two stack items.
    MIN = 0xB9,
    /// Push the maximum of the top two stack items.
    MAX = 0xBA,
    /// Push `true` if the third item is within `[min, max)` of the second.
    WITHIN = 0xBB,

    // === Compound / collections (0xBE–0xD4) ===
    /// Pack a map from a 1-byte count of key/value pairs on the stack.
    PACKMAP = 0xBE,
    /// Pack a struct from a 1-byte count of items on the stack.
    PACKSTRUCT = 0xBF,
    /// Pack an array from a 1-byte count of items on the stack.
    PACK = 0xC0,
    /// Unpack an array/struct onto the stack.
    UNPACK = 0xC1,
    /// Push a new empty array.
    NEWARRAY0 = 0xC2,
    /// Push a new array of items already on the stack (1-byte count).
    NEWARRAY = 0xC3,
    /// Push a new array of a specific stack-item type (1-byte type tag).
    NEWARRAY_T = 0xC4,
    /// Push a new empty struct.
    NEWSTRUCT0 = 0xC5,
    /// Push a new struct of items already on the stack (1-byte count).
    NEWSTRUCT = 0xC6,
    /// Push a new empty map.
    NEWMAP = 0xC8,
    /// Push the size of the top-of-stack collection / buffer / string.
    SIZE = 0xCA,
    /// Push `true` if the map has the given key (top).
    HASKEY = 0xCB,
    /// Push all keys of the map on top of the stack as an array.
    KEYS = 0xCC,
    /// Push all values of the map on top of the stack as an array.
    VALUES = 0xCD,
    /// Push the item at the given 1-byte index of the top array/struct/map.
    PICKITEM = 0xCE,
    /// Append the second item to the array on top of the stack.
    APPEND = 0xCF,
    /// Set the value of the map at the given key.
    SETITEM = 0xD0,
    /// Reverse the items of the top array/struct in place.
    REVERSEITEMS = 0xD1,
    /// Remove the entry with the given key from the top map.
    REMOVE = 0xD2,
    /// Clear all items from the top collection.
    CLEARITEMS = 0xD3,
    /// Pop the last item of the top array and push it.
    POPITEM = 0xD4,

    // === Types (0xD8–0xDB) ===
    /// Push `true` if the top of the stack is `null`.
    ISNULL = 0xD8,
    /// Push `true` if the top of the stack has the given 1-byte type tag.
    ISTYPE = 0xD9,
    /// Convert the top of the stack to the given 1-byte type tag.
    CONVERT = 0xDB,

    // === Extensions (0xE0–0xE1) ===
    /// Abort with a 1-byte-prefixed message.
    ABORTMSG = 0xE0,
    /// Assert-with-message; same as `ASSERT` but with a payload.
    ASSERTMSG = 0xE1,
}

impl OpCode {
    /// The raw byte value of this opcode, as it appears in a script.
    ///
    /// `#[repr(u8)]` makes the layout trivially `as u8`-castable, but
    /// we expose this explicit `const fn` so call sites do not need to
    /// write `as u8` everywhere.
    #[inline]
    pub const fn byte(self) -> u8 {
        self as u8
    }

    /// Static, canonical name (e.g. `"PUSH0"`, `"JMP_L"`).
    #[inline]
    pub const fn name(self) -> &'static str {
        match self {
            OpCode::PUSHINT8 => "PUSHINT8",
            OpCode::PUSHINT16 => "PUSHINT16",
            OpCode::PUSHINT32 => "PUSHINT32",
            OpCode::PUSHINT64 => "PUSHINT64",
            OpCode::PUSHINT128 => "PUSHINT128",
            OpCode::PUSHINT256 => "PUSHINT256",
            OpCode::PUSHT => "PUSHT",
            OpCode::PUSHF => "PUSHF",
            OpCode::PUSHA => "PUSHA",
            OpCode::PUSHNULL => "PUSHNULL",
            OpCode::PUSHDATA1 => "PUSHDATA1",
            OpCode::PUSHDATA2 => "PUSHDATA2",
            OpCode::PUSHDATA4 => "PUSHDATA4",
            OpCode::PUSHM1 => "PUSHM1",
            OpCode::PUSH0 => "PUSH0",
            OpCode::PUSH1 => "PUSH1",
            OpCode::PUSH2 => "PUSH2",
            OpCode::PUSH3 => "PUSH3",
            OpCode::PUSH4 => "PUSH4",
            OpCode::PUSH5 => "PUSH5",
            OpCode::PUSH6 => "PUSH6",
            OpCode::PUSH7 => "PUSH7",
            OpCode::PUSH8 => "PUSH8",
            OpCode::PUSH9 => "PUSH9",
            OpCode::PUSH10 => "PUSH10",
            OpCode::PUSH11 => "PUSH11",
            OpCode::PUSH12 => "PUSH12",
            OpCode::PUSH13 => "PUSH13",
            OpCode::PUSH14 => "PUSH14",
            OpCode::PUSH15 => "PUSH15",
            OpCode::PUSH16 => "PUSH16",
            OpCode::NOP => "NOP",
            OpCode::JMP => "JMP",
            OpCode::JMP_L => "JMP_L",
            OpCode::JMPIF => "JMPIF",
            OpCode::JMPIF_L => "JMPIF_L",
            OpCode::JMPIFNOT => "JMPIFNOT",
            OpCode::JMPIFNOT_L => "JMPIFNOT_L",
            OpCode::JMPEQ => "JMPEQ",
            OpCode::JMPEQ_L => "JMPEQ_L",
            OpCode::JMPNE => "JMPNE",
            OpCode::JMPNE_L => "JMPNE_L",
            OpCode::JMPGT => "JMPGT",
            OpCode::JMPGT_L => "JMPGT_L",
            OpCode::JMPGE => "JMPGE",
            OpCode::JMPGE_L => "JMPGE_L",
            OpCode::JMPLT => "JMPLT",
            OpCode::JMPLT_L => "JMPLT_L",
            OpCode::JMPLE => "JMPLE",
            OpCode::JMPLE_L => "JMPLE_L",
            OpCode::CALL => "CALL",
            OpCode::CALL_L => "CALL_L",
            OpCode::CALLA => "CALLA",
            OpCode::CALLT => "CALLT",
            OpCode::ABORT => "ABORT",
            OpCode::ASSERT => "ASSERT",
            OpCode::THROW => "THROW",
            OpCode::TRY => "TRY",
            OpCode::TRY_L => "TRY_L",
            OpCode::ENDTRY => "ENDTRY",
            OpCode::ENDTRY_L => "ENDTRY_L",
            OpCode::ENDFINALLY => "ENDFINALLY",
            OpCode::RET => "RET",
            OpCode::SYSCALL => "SYSCALL",
            OpCode::DEPTH => "DEPTH",
            OpCode::DROP => "DROP",
            OpCode::NIP => "NIP",
            OpCode::XDROP => "XDROP",
            OpCode::CLEAR => "CLEAR",
            OpCode::DUP => "DUP",
            OpCode::OVER => "OVER",
            OpCode::PICK => "PICK",
            OpCode::TUCK => "TUCK",
            OpCode::SWAP => "SWAP",
            OpCode::ROT => "ROT",
            OpCode::ROLL => "ROLL",
            OpCode::REVERSE3 => "REVERSE3",
            OpCode::REVERSE4 => "REVERSE4",
            OpCode::REVERSEN => "REVERSEN",
            OpCode::INITSSLOT => "INITSSLOT",
            OpCode::INITSLOT => "INITSLOT",
            OpCode::LDSFLD0 => "LDSFLD0",
            OpCode::LDSFLD1 => "LDSFLD1",
            OpCode::LDSFLD2 => "LDSFLD2",
            OpCode::LDSFLD3 => "LDSFLD3",
            OpCode::LDSFLD4 => "LDSFLD4",
            OpCode::LDSFLD5 => "LDSFLD5",
            OpCode::LDSFLD6 => "LDSFLD6",
            OpCode::LDSFLD => "LDSFLD",
            OpCode::STSFLD0 => "STSFLD0",
            OpCode::STSFLD1 => "STSFLD1",
            OpCode::STSFLD2 => "STSFLD2",
            OpCode::STSFLD3 => "STSFLD3",
            OpCode::STSFLD4 => "STSFLD4",
            OpCode::STSFLD5 => "STSFLD5",
            OpCode::STSFLD6 => "STSFLD6",
            OpCode::STSFLD => "STSFLD",
            OpCode::LDLOC0 => "LDLOC0",
            OpCode::LDLOC1 => "LDLOC1",
            OpCode::LDLOC2 => "LDLOC2",
            OpCode::LDLOC3 => "LDLOC3",
            OpCode::LDLOC4 => "LDLOC4",
            OpCode::LDLOC5 => "LDLOC5",
            OpCode::LDLOC6 => "LDLOC6",
            OpCode::LDLOC => "LDLOC",
            OpCode::STLOC0 => "STLOC0",
            OpCode::STLOC1 => "STLOC1",
            OpCode::STLOC2 => "STLOC2",
            OpCode::STLOC3 => "STLOC3",
            OpCode::STLOC4 => "STLOC4",
            OpCode::STLOC5 => "STLOC5",
            OpCode::STLOC6 => "STLOC6",
            OpCode::STLOC => "STLOC",
            OpCode::LDARG0 => "LDARG0",
            OpCode::LDARG1 => "LDARG1",
            OpCode::LDARG2 => "LDARG2",
            OpCode::LDARG3 => "LDARG3",
            OpCode::LDARG4 => "LDARG4",
            OpCode::LDARG5 => "LDARG5",
            OpCode::LDARG6 => "LDARG6",
            OpCode::LDARG => "LDARG",
            OpCode::STARG0 => "STARG0",
            OpCode::STARG1 => "STARG1",
            OpCode::STARG2 => "STARG2",
            OpCode::STARG3 => "STARG3",
            OpCode::STARG4 => "STARG4",
            OpCode::STARG5 => "STARG5",
            OpCode::STARG6 => "STARG6",
            OpCode::STARG => "STARG",
            OpCode::NEWBUFFER => "NEWBUFFER",
            OpCode::MEMCPY => "MEMCPY",
            OpCode::CAT => "CAT",
            OpCode::SUBSTR => "SUBSTR",
            OpCode::LEFT => "LEFT",
            OpCode::RIGHT => "RIGHT",
            OpCode::INVERT => "INVERT",
            OpCode::AND => "AND",
            OpCode::OR => "OR",
            OpCode::XOR => "XOR",
            OpCode::EQUAL => "EQUAL",
            OpCode::NOTEQUAL => "NOTEQUAL",
            OpCode::SIGN => "SIGN",
            OpCode::ABS => "ABS",
            OpCode::NEGATE => "NEGATE",
            OpCode::INC => "INC",
            OpCode::DEC => "DEC",
            OpCode::ADD => "ADD",
            OpCode::SUB => "SUB",
            OpCode::MUL => "MUL",
            OpCode::DIV => "DIV",
            OpCode::MOD => "MOD",
            OpCode::POW => "POW",
            OpCode::SQRT => "SQRT",
            OpCode::MODMUL => "MODMUL",
            OpCode::MODPOW => "MODPOW",
            OpCode::SHL => "SHL",
            OpCode::SHR => "SHR",
            OpCode::NOT => "NOT",
            OpCode::BOOLAND => "BOOLAND",
            OpCode::BOOLOR => "BOOLOR",
            OpCode::NZ => "NZ",
            OpCode::NUMEQUAL => "NUMEQUAL",
            OpCode::NUMNOTEQUAL => "NUMNOTEQUAL",
            OpCode::LT => "LT",
            OpCode::LE => "LE",
            OpCode::GT => "GT",
            OpCode::GE => "GE",
            OpCode::MIN => "MIN",
            OpCode::MAX => "MAX",
            OpCode::WITHIN => "WITHIN",
            OpCode::PACKMAP => "PACKMAP",
            OpCode::PACKSTRUCT => "PACKSTRUCT",
            OpCode::PACK => "PACK",
            OpCode::UNPACK => "UNPACK",
            OpCode::NEWARRAY0 => "NEWARRAY0",
            OpCode::NEWARRAY => "NEWARRAY",
            OpCode::NEWARRAY_T => "NEWARRAY_T",
            OpCode::NEWSTRUCT0 => "NEWSTRUCT0",
            OpCode::NEWSTRUCT => "NEWSTRUCT",
            OpCode::NEWMAP => "NEWMAP",
            OpCode::SIZE => "SIZE",
            OpCode::HASKEY => "HASKEY",
            OpCode::KEYS => "KEYS",
            OpCode::VALUES => "VALUES",
            OpCode::PICKITEM => "PICKITEM",
            OpCode::APPEND => "APPEND",
            OpCode::SETITEM => "SETITEM",
            OpCode::REVERSEITEMS => "REVERSEITEMS",
            OpCode::REMOVE => "REMOVE",
            OpCode::CLEARITEMS => "CLEARITEMS",
            OpCode::POPITEM => "POPITEM",
            OpCode::ISNULL => "ISNULL",
            OpCode::ISTYPE => "ISTYPE",
            OpCode::CONVERT => "CONVERT",
            OpCode::ABORTMSG => "ABORTMSG",
            OpCode::ASSERTMSG => "ASSERTMSG",
        }
    }

    /// Default gas cost (matches the canonical Neo N3 spec).
    ///
    /// `0` is returned for opcodes that do not consume gas on their
    /// own (e.g. `RET`); the actual cost depends on context for
    /// opcodes like `CALL` and `SYSCALL` whose pricing is dynamic
    /// — callers that care about exact cost should special-case.
    #[inline]
    pub const fn gas(self) -> u64 {
        match self {
            OpCode::PUSHINT8
            | OpCode::PUSHINT16
            | OpCode::PUSHINT32
            | OpCode::PUSHINT64
            | OpCode::PUSHINT128
            | OpCode::PUSHINT256
            | OpCode::PUSHT
            | OpCode::PUSHF
            | OpCode::PUSHA
            | OpCode::PUSHNULL
            | OpCode::PUSHM1
            | OpCode::PUSH0
            | OpCode::PUSH1
            | OpCode::PUSH2
            | OpCode::PUSH3
            | OpCode::PUSH4
            | OpCode::PUSH5
            | OpCode::PUSH6
            | OpCode::PUSH7
            | OpCode::PUSH8
            | OpCode::PUSH9
            | OpCode::PUSH10
            | OpCode::PUSH11
            | OpCode::PUSH12
            | OpCode::PUSH13
            | OpCode::PUSH14
            | OpCode::PUSH15
            | OpCode::PUSH16 => 1,
            OpCode::PUSHDATA1 | OpCode::PUSHDATA2 | OpCode::PUSHDATA4 => 2,
            OpCode::NOP
            | OpCode::JMP
            | OpCode::JMP_L
            | OpCode::JMPIF
            | OpCode::JMPIF_L
            | OpCode::JMPIFNOT
            | OpCode::JMPIFNOT_L
            | OpCode::JMPEQ
            | OpCode::JMPEQ_L
            | OpCode::JMPNE
            | OpCode::JMPNE_L
            | OpCode::JMPGT
            | OpCode::JMPGT_L
            | OpCode::JMPGE
            | OpCode::JMPGE_L
            | OpCode::JMPLT
            | OpCode::JMPLT_L
            | OpCode::JMPLE
            | OpCode::JMPLE_L => 2,
            OpCode::CALL | OpCode::CALL_L | OpCode::CALLA | OpCode::CALLT => 512,
            OpCode::ABORT | OpCode::ASSERT | OpCode::THROW => 1,
            OpCode::TRY
            | OpCode::TRY_L
            | OpCode::ENDTRY
            | OpCode::ENDTRY_L
            | OpCode::ENDFINALLY => 1,
            OpCode::RET => 0,
            OpCode::SYSCALL => 10,
            OpCode::DEPTH
            | OpCode::DROP
            | OpCode::NIP
            | OpCode::XDROP
            | OpCode::CLEAR
            | OpCode::DUP
            | OpCode::OVER
            | OpCode::PICK
            | OpCode::TUCK
            | OpCode::SWAP
            | OpCode::ROT
            | OpCode::ROLL
            | OpCode::REVERSE3
            | OpCode::REVERSE4
            | OpCode::REVERSEN
            | OpCode::INITSSLOT
            | OpCode::INITSLOT
            | OpCode::LDSFLD0
            | OpCode::LDSFLD1
            | OpCode::LDSFLD2
            | OpCode::LDSFLD3
            | OpCode::LDSFLD4
            | OpCode::LDSFLD5
            | OpCode::LDSFLD6
            | OpCode::LDSFLD
            | OpCode::STSFLD0
            | OpCode::STSFLD1
            | OpCode::STSFLD2
            | OpCode::STSFLD3
            | OpCode::STSFLD4
            | OpCode::STSFLD5
            | OpCode::STSFLD6
            | OpCode::STSFLD
            | OpCode::LDLOC0
            | OpCode::LDLOC1
            | OpCode::LDLOC2
            | OpCode::LDLOC3
            | OpCode::LDLOC4
            | OpCode::LDLOC5
            | OpCode::LDLOC6
            | OpCode::LDLOC
            | OpCode::STLOC0
            | OpCode::STLOC1
            | OpCode::STLOC2
            | OpCode::STLOC3
            | OpCode::STLOC4
            | OpCode::STLOC5
            | OpCode::STLOC6
            | OpCode::STLOC
            | OpCode::LDARG0
            | OpCode::LDARG1
            | OpCode::LDARG2
            | OpCode::LDARG3
            | OpCode::LDARG4
            | OpCode::LDARG5
            | OpCode::LDARG6
            | OpCode::LDARG
            | OpCode::STARG0
            | OpCode::STARG1
            | OpCode::STARG2
            | OpCode::STARG3
            | OpCode::STARG4
            | OpCode::STARG5
            | OpCode::STARG6
            | OpCode::STARG
            | OpCode::ISNULL
            | OpCode::ISTYPE
            | OpCode::CONVERT
            | OpCode::NOT
            | OpCode::BOOLAND
            | OpCode::BOOLOR
            | OpCode::NZ => 2,
            OpCode::NEWBUFFER
            | OpCode::MEMCPY
            | OpCode::CAT
            | OpCode::SUBSTR
            | OpCode::LEFT
            | OpCode::RIGHT
            | OpCode::PACKMAP
            | OpCode::PACKSTRUCT
            | OpCode::PACK
            | OpCode::UNPACK
            | OpCode::NEWARRAY0
            | OpCode::NEWARRAY
            | OpCode::NEWARRAY_T
            | OpCode::NEWSTRUCT0
            | OpCode::NEWSTRUCT
            | OpCode::NEWMAP
            | OpCode::SIZE
            | OpCode::HASKEY
            | OpCode::KEYS
            | OpCode::VALUES
            | OpCode::PICKITEM
            | OpCode::APPEND
            | OpCode::SETITEM
            | OpCode::REVERSEITEMS
            | OpCode::REMOVE
            | OpCode::CLEARITEMS
            | OpCode::POPITEM => 4,
            OpCode::INVERT
            | OpCode::AND
            | OpCode::OR
            | OpCode::XOR
            | OpCode::EQUAL
            | OpCode::NOTEQUAL => 3,
            OpCode::SIGN
            | OpCode::ABS
            | OpCode::NEGATE
            | OpCode::INC
            | OpCode::DEC
            | OpCode::ADD
            | OpCode::SUB
            | OpCode::SHL
            | OpCode::SHR
            | OpCode::NUMEQUAL
            | OpCode::NUMNOTEQUAL
            | OpCode::LT
            | OpCode::LE
            | OpCode::GT
            | OpCode::GE
            | OpCode::MIN
            | OpCode::MAX
            | OpCode::WITHIN => 3,
            OpCode::MUL | OpCode::DIV | OpCode::MOD => 5,
            OpCode::POW | OpCode::SQRT | OpCode::MODMUL | OpCode::MODPOW => match self {
                OpCode::POW => 8,
                OpCode::SQRT => 6,
                _ => 8,
            },
            OpCode::ABORTMSG | OpCode::ASSERTMSG => 1,
        }
    }

    /// `PUSH0`..`PUSH16` for `n` in `0..=16`, or `None` for any other
    /// value. Use this anywhere the codegen computes a small integer
    /// push instead of writing `0x10 + n` by hand.
    #[inline]
    pub const fn push_small(n: u8) -> Option<Self> {
        match n {
            0 => Some(OpCode::PUSH0),
            1 => Some(OpCode::PUSH1),
            2 => Some(OpCode::PUSH2),
            3 => Some(OpCode::PUSH3),
            4 => Some(OpCode::PUSH4),
            5 => Some(OpCode::PUSH5),
            6 => Some(OpCode::PUSH6),
            7 => Some(OpCode::PUSH7),
            8 => Some(OpCode::PUSH8),
            9 => Some(OpCode::PUSH9),
            10 => Some(OpCode::PUSH10),
            11 => Some(OpCode::PUSH11),
            12 => Some(OpCode::PUSH12),
            13 => Some(OpCode::PUSH13),
            14 => Some(OpCode::PUSH14),
            15 => Some(OpCode::PUSH15),
            16 => Some(OpCode::PUSH16),
            _ => None,
        }
    }

    /// `LDLOC0`..`LDLOC6` for `n` in `0..=6`, otherwise the generic
    /// `LDLOC` opcode (which is followed by a 1-byte index operand).
    #[inline]
    pub const fn ldloc(n: u8) -> Self {
        match n {
            0 => OpCode::LDLOC0,
            1 => OpCode::LDLOC1,
            2 => OpCode::LDLOC2,
            3 => OpCode::LDLOC3,
            4 => OpCode::LDLOC4,
            5 => OpCode::LDLOC5,
            6 => OpCode::LDLOC6,
            _ => OpCode::LDLOC,
        }
    }

    /// `STLOC0`..`STLOC6` for `n` in `0..=6`, otherwise the generic
    /// `STLOC` opcode (which is followed by a 1-byte index operand).
    #[inline]
    pub const fn stloc(n: u8) -> Self {
        match n {
            0 => OpCode::STLOC0,
            1 => OpCode::STLOC1,
            2 => OpCode::STLOC2,
            3 => OpCode::STLOC3,
            4 => OpCode::STLOC4,
            5 => OpCode::STLOC5,
            6 => OpCode::STLOC6,
            _ => OpCode::STLOC,
        }
    }

    /// `LDARG0`..`LDARG6` for `n` in `0..=6`, otherwise the generic
    /// `LDARG` opcode (which is followed by a 1-byte index operand).
    #[inline]
    pub const fn ldarg(n: u8) -> Self {
        match n {
            0 => OpCode::LDARG0,
            1 => OpCode::LDARG1,
            2 => OpCode::LDARG2,
            3 => OpCode::LDARG3,
            4 => OpCode::LDARG4,
            5 => OpCode::LDARG5,
            6 => OpCode::LDARG6,
            _ => OpCode::LDARG,
        }
    }

    /// `STARG0`..`STARG6` for `n` in `0..=6`, otherwise the generic
    /// `STARG` opcode (which is followed by a 1-byte index operand).
    #[inline]
    pub const fn starg(n: u8) -> Self {
        match n {
            0 => OpCode::STARG0,
            1 => OpCode::STARG1,
            2 => OpCode::STARG2,
            3 => OpCode::STARG3,
            4 => OpCode::STARG4,
            5 => OpCode::STARG5,
            6 => OpCode::STARG6,
            _ => OpCode::STARG,
        }
    }

    /// `LDSFLD0`..`LDSFLD6` for `n` in `0..=6`, otherwise the generic
    /// `LDSFLD` opcode (which is followed by a 1-byte index operand).
    #[inline]
    pub const fn ldsfld(n: u8) -> Self {
        match n {
            0 => OpCode::LDSFLD0,
            1 => OpCode::LDSFLD1,
            2 => OpCode::LDSFLD2,
            3 => OpCode::LDSFLD3,
            4 => OpCode::LDSFLD4,
            5 => OpCode::LDSFLD5,
            6 => OpCode::LDSFLD6,
            _ => OpCode::LDSFLD,
        }
    }

    /// `STSFLD0`..`STSFLD6` for `n` in `0..=6`, otherwise the generic
    /// `STSFLD` opcode (which is followed by a 1-byte index operand).
    #[inline]
    pub const fn stsfld(n: u8) -> Self {
        match n {
            0 => OpCode::STSFLD0,
            1 => OpCode::STSFLD1,
            2 => OpCode::STSFLD2,
            3 => OpCode::STSFLD3,
            4 => OpCode::STSFLD4,
            5 => OpCode::STSFLD5,
            6 => OpCode::STSFLD6,
            _ => OpCode::STSFLD,
        }
    }

    /// Pick the right `PUSHDATA*` opcode for a payload of `len` bytes.
    ///
    /// - `len ≤ 0xFF`  → [`OpCode::PUSHDATA1`] (1-byte length prefix)
    /// - `len ≤ 0xFFFF` → [`OpCode::PUSHDATA2`] (2-byte length prefix)
    /// - else           → [`OpCode::PUSHDATA4`] (4-byte length prefix)
    #[inline]
    pub const fn push_data(len: usize) -> Self {
        if len <= u8::MAX as usize {
            OpCode::PUSHDATA1
        } else if len <= u16::MAX as usize {
            OpCode::PUSHDATA2
        } else {
            OpCode::PUSHDATA4
        }
    }

    /// Pick the smallest `PUSHINT*` opcode that fits `value` (as
    /// little-endian two's-complement, signed).
    ///
    /// - `0..=0xFF`        → [`OpCode::PUSHINT8`]
    /// - `0..=0xFFFF`      → [`OpCode::PUSHINT16`]
    /// - `0..=0xFFFF_FFFF` → [`OpCode::PUSHINT32`]
    /// - else              → [`OpCode::PUSHINT64`]
    ///
    /// Values that do not fit in `i64` (or that are negative) should
    /// be handled by the caller via [`OpCode::PUSHINT128`] or
    /// [`OpCode::PUSHINT256`].
    #[inline]
    pub const fn push_int(value: u64) -> Self {
        if value <= u8::MAX as u64 {
            OpCode::PUSHINT8
        } else if value <= u16::MAX as u64 {
            OpCode::PUSHINT16
        } else if value <= u32::MAX as u64 {
            OpCode::PUSHINT32
        } else {
            OpCode::PUSHINT64
        }
    }
}

impl TryFrom<u8> for OpCode {
    type Error = ();

    /// Convert a raw byte into the canonical [`OpCode`].
    ///
    /// Returns `Err(())` for bytes that are not assigned a variant
    /// (e.g. `0x06`, `0x07`, `0x42`, `0x44`, … — the unused slots in
    /// the NeoVM spec). The runtime simulator and disassembler use
    /// this to detect unknown instructions and report them honestly
    /// rather than silently mapping to a placeholder.
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            0x00 => OpCode::PUSHINT8,
            0x01 => OpCode::PUSHINT16,
            0x02 => OpCode::PUSHINT32,
            0x03 => OpCode::PUSHINT64,
            0x04 => OpCode::PUSHINT128,
            0x05 => OpCode::PUSHINT256,
            0x08 => OpCode::PUSHT,
            0x09 => OpCode::PUSHF,
            0x0A => OpCode::PUSHA,
            0x0B => OpCode::PUSHNULL,
            0x0C => OpCode::PUSHDATA1,
            0x0D => OpCode::PUSHDATA2,
            0x0E => OpCode::PUSHDATA4,
            0x0F => OpCode::PUSHM1,
            0x10 => OpCode::PUSH0,
            0x11 => OpCode::PUSH1,
            0x12 => OpCode::PUSH2,
            0x13 => OpCode::PUSH3,
            0x14 => OpCode::PUSH4,
            0x15 => OpCode::PUSH5,
            0x16 => OpCode::PUSH6,
            0x17 => OpCode::PUSH7,
            0x18 => OpCode::PUSH8,
            0x19 => OpCode::PUSH9,
            0x1A => OpCode::PUSH10,
            0x1B => OpCode::PUSH11,
            0x1C => OpCode::PUSH12,
            0x1D => OpCode::PUSH13,
            0x1E => OpCode::PUSH14,
            0x1F => OpCode::PUSH15,
            0x20 => OpCode::PUSH16,
            0x21 => OpCode::NOP,
            0x22 => OpCode::JMP,
            0x23 => OpCode::JMP_L,
            0x24 => OpCode::JMPIF,
            0x25 => OpCode::JMPIF_L,
            0x26 => OpCode::JMPIFNOT,
            0x27 => OpCode::JMPIFNOT_L,
            0x28 => OpCode::JMPEQ,
            0x29 => OpCode::JMPEQ_L,
            0x2A => OpCode::JMPNE,
            0x2B => OpCode::JMPNE_L,
            0x2C => OpCode::JMPGT,
            0x2D => OpCode::JMPGT_L,
            0x2E => OpCode::JMPGE,
            0x2F => OpCode::JMPGE_L,
            0x30 => OpCode::JMPLT,
            0x31 => OpCode::JMPLT_L,
            0x32 => OpCode::JMPLE,
            0x33 => OpCode::JMPLE_L,
            0x34 => OpCode::CALL,
            0x35 => OpCode::CALL_L,
            0x36 => OpCode::CALLA,
            0x37 => OpCode::CALLT,
            0x38 => OpCode::ABORT,
            0x39 => OpCode::ASSERT,
            0x3A => OpCode::THROW,
            0x3B => OpCode::TRY,
            0x3C => OpCode::TRY_L,
            0x3D => OpCode::ENDTRY,
            0x3E => OpCode::ENDTRY_L,
            0x3F => OpCode::ENDFINALLY,
            0x40 => OpCode::RET,
            0x41 => OpCode::SYSCALL,
            0x43 => OpCode::DEPTH,
            0x45 => OpCode::DROP,
            0x46 => OpCode::NIP,
            0x48 => OpCode::XDROP,
            0x49 => OpCode::CLEAR,
            0x4A => OpCode::DUP,
            0x4B => OpCode::OVER,
            0x4D => OpCode::PICK,
            0x4E => OpCode::TUCK,
            0x50 => OpCode::SWAP,
            0x51 => OpCode::ROT,
            0x52 => OpCode::ROLL,
            0x53 => OpCode::REVERSE3,
            0x54 => OpCode::REVERSE4,
            0x55 => OpCode::REVERSEN,
            0x56 => OpCode::INITSSLOT,
            0x57 => OpCode::INITSLOT,
            0x58 => OpCode::LDSFLD0,
            0x59 => OpCode::LDSFLD1,
            0x5A => OpCode::LDSFLD2,
            0x5B => OpCode::LDSFLD3,
            0x5C => OpCode::LDSFLD4,
            0x5D => OpCode::LDSFLD5,
            0x5E => OpCode::LDSFLD6,
            0x5F => OpCode::LDSFLD,
            0x60 => OpCode::STSFLD0,
            0x61 => OpCode::STSFLD1,
            0x62 => OpCode::STSFLD2,
            0x63 => OpCode::STSFLD3,
            0x64 => OpCode::STSFLD4,
            0x65 => OpCode::STSFLD5,
            0x66 => OpCode::STSFLD6,
            0x67 => OpCode::STSFLD,
            0x68 => OpCode::LDLOC0,
            0x69 => OpCode::LDLOC1,
            0x6A => OpCode::LDLOC2,
            0x6B => OpCode::LDLOC3,
            0x6C => OpCode::LDLOC4,
            0x6D => OpCode::LDLOC5,
            0x6E => OpCode::LDLOC6,
            0x6F => OpCode::LDLOC,
            0x70 => OpCode::STLOC0,
            0x71 => OpCode::STLOC1,
            0x72 => OpCode::STLOC2,
            0x73 => OpCode::STLOC3,
            0x74 => OpCode::STLOC4,
            0x75 => OpCode::STLOC5,
            0x76 => OpCode::STLOC6,
            0x77 => OpCode::STLOC,
            0x78 => OpCode::LDARG0,
            0x79 => OpCode::LDARG1,
            0x7A => OpCode::LDARG2,
            0x7B => OpCode::LDARG3,
            0x7C => OpCode::LDARG4,
            0x7D => OpCode::LDARG5,
            0x7E => OpCode::LDARG6,
            0x7F => OpCode::LDARG,
            0x80 => OpCode::STARG0,
            0x81 => OpCode::STARG1,
            0x82 => OpCode::STARG2,
            0x83 => OpCode::STARG3,
            0x84 => OpCode::STARG4,
            0x85 => OpCode::STARG5,
            0x86 => OpCode::STARG6,
            0x87 => OpCode::STARG,
            0x88 => OpCode::NEWBUFFER,
            0x89 => OpCode::MEMCPY,
            0x8B => OpCode::CAT,
            0x8C => OpCode::SUBSTR,
            0x8D => OpCode::LEFT,
            0x8E => OpCode::RIGHT,
            0x90 => OpCode::INVERT,
            0x91 => OpCode::AND,
            0x92 => OpCode::OR,
            0x93 => OpCode::XOR,
            0x97 => OpCode::EQUAL,
            0x98 => OpCode::NOTEQUAL,
            0x99 => OpCode::SIGN,
            0x9A => OpCode::ABS,
            0x9B => OpCode::NEGATE,
            0x9C => OpCode::INC,
            0x9D => OpCode::DEC,
            0x9E => OpCode::ADD,
            0x9F => OpCode::SUB,
            0xA0 => OpCode::MUL,
            0xA1 => OpCode::DIV,
            0xA2 => OpCode::MOD,
            0xA3 => OpCode::POW,
            0xA4 => OpCode::SQRT,
            0xA5 => OpCode::MODMUL,
            0xA6 => OpCode::MODPOW,
            0xA8 => OpCode::SHL,
            0xA9 => OpCode::SHR,
            0xAA => OpCode::NOT,
            0xAB => OpCode::BOOLAND,
            0xAC => OpCode::BOOLOR,
            0xB1 => OpCode::NZ,
            0xB3 => OpCode::NUMEQUAL,
            0xB4 => OpCode::NUMNOTEQUAL,
            0xB5 => OpCode::LT,
            0xB6 => OpCode::LE,
            0xB7 => OpCode::GT,
            0xB8 => OpCode::GE,
            0xB9 => OpCode::MIN,
            0xBA => OpCode::MAX,
            0xBB => OpCode::WITHIN,
            0xBE => OpCode::PACKMAP,
            0xBF => OpCode::PACKSTRUCT,
            0xC0 => OpCode::PACK,
            0xC1 => OpCode::UNPACK,
            0xC2 => OpCode::NEWARRAY0,
            0xC3 => OpCode::NEWARRAY,
            0xC4 => OpCode::NEWARRAY_T,
            0xC5 => OpCode::NEWSTRUCT0,
            0xC6 => OpCode::NEWSTRUCT,
            0xC8 => OpCode::NEWMAP,
            0xCA => OpCode::SIZE,
            0xCB => OpCode::HASKEY,
            0xCC => OpCode::KEYS,
            0xCD => OpCode::VALUES,
            0xCE => OpCode::PICKITEM,
            0xCF => OpCode::APPEND,
            0xD0 => OpCode::SETITEM,
            0xD1 => OpCode::REVERSEITEMS,
            0xD2 => OpCode::REMOVE,
            0xD3 => OpCode::CLEARITEMS,
            0xD4 => OpCode::POPITEM,
            0xD8 => OpCode::ISNULL,
            0xD9 => OpCode::ISTYPE,
            0xDB => OpCode::CONVERT,
            0xE0 => OpCode::ABORTMSG,
            0xE1 => OpCode::ASSERTMSG,
            _ => return Err(()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn byte_roundtrips_through_try_from() {
        // Every variant must round-trip byte -> variant -> byte.
        let cases = [
            OpCode::PUSHINT8,
            OpCode::PUSHINT256,
            OpCode::PUSHDATA1,
            OpCode::PUSHDATA4,
            OpCode::PUSH0,
            OpCode::PUSH16,
            OpCode::NOP,
            OpCode::JMP_L,
            OpCode::CALL,
            OpCode::SYSCALL,
            OpCode::RET,
            OpCode::DUP,
            OpCode::LDLOC0,
            OpCode::LDLOC6,
            OpCode::LDLOC,
            OpCode::STARG,
            OpCode::ADD,
            OpCode::MODPOW,
            OpCode::ISNULL,
            OpCode::CONVERT,
            OpCode::ABORTMSG,
            OpCode::ASSERTMSG,
        ];
        for op in cases {
            let byte = op.byte();
            let parsed = OpCode::try_from(byte).expect("known opcode");
            assert_eq!(parsed, op, "byte 0x{byte:02X} round-trip mismatch");
        }
    }

    #[test]
    fn try_from_rejects_unassigned_bytes() {
        // Holes in the opcode space: 0x06, 0x07, 0x42, 0x44, 0x47, 0x4C, 0x4F, …
        for byte in [
            0x06u8, 0x07, 0x42, 0x44, 0x47, 0x4C, 0x4F, 0x8A, 0xA7, 0xAD, 0xD5,
        ] {
            assert!(
                OpCode::try_from(byte).is_err(),
                "byte 0x{byte:02X} should be unassigned"
            );
        }
    }

    #[test]
    fn indexed_constructors_match_byte_layout() {
        // PUSHn
        for n in 0u8..=16 {
            assert_eq!(OpCode::push_small(n).unwrap().byte(), 0x10 + n);
        }
        assert!(OpCode::push_small(17).is_none());

        // LDLOCn / STLOCn
        for n in 0u8..=6 {
            assert_eq!(OpCode::ldloc(n).byte(), 0x68 + n);
            assert_eq!(OpCode::stloc(n).byte(), 0x70 + n);
        }
        assert_eq!(OpCode::ldloc(7).byte(), OpCode::LDLOC.byte());
        assert_eq!(OpCode::stloc(255).byte(), OpCode::STLOC.byte());

        // LDARGn / STARGn
        for n in 0u8..=6 {
            assert_eq!(OpCode::ldarg(n).byte(), 0x78 + n);
            assert_eq!(OpCode::starg(n).byte(), 0x80 + n);
        }
        assert_eq!(OpCode::ldarg(7).byte(), OpCode::LDARG.byte());
        assert_eq!(OpCode::starg(7).byte(), OpCode::STARG.byte());

        // LDSFLDn / STSFLDn
        for n in 0u8..=6 {
            assert_eq!(OpCode::ldsfld(n).byte(), 0x58 + n);
            assert_eq!(OpCode::stsfld(n).byte(), 0x60 + n);
        }
        assert_eq!(OpCode::ldsfld(7).byte(), OpCode::LDSFLD.byte());
        assert_eq!(OpCode::stsfld(7).byte(), OpCode::STSFLD.byte());
    }

    #[test]
    fn push_data_picks_narrowest_prefix() {
        assert_eq!(OpCode::push_data(0).byte(), 0x0C);
        assert_eq!(OpCode::push_data(0xFF).byte(), 0x0C);
        assert_eq!(OpCode::push_data(0x100).byte(), 0x0D);
        assert_eq!(OpCode::push_data(0xFFFF).byte(), 0x0D);
        assert_eq!(OpCode::push_data(0x1_0000).byte(), 0x0E);
    }

    #[test]
    fn push_int_picks_smallest_signed_fit() {
        assert_eq!(OpCode::push_int(0).byte(), 0x00);
        assert_eq!(OpCode::push_int(0xFF).byte(), 0x00);
        assert_eq!(OpCode::push_int(0x100).byte(), 0x01);
        assert_eq!(OpCode::push_int(0xFFFF).byte(), 0x01);
        assert_eq!(OpCode::push_int(0x1_0000).byte(), 0x02);
        assert_eq!(OpCode::push_int(0xFFFF_FFFF).byte(), 0x02);
        assert_eq!(OpCode::push_int(0x1_0000_0000).byte(), 0x03);
        assert_eq!(OpCode::push_int(u64::MAX).byte(), 0x03);
    }

    #[test]
    fn name_is_stable_and_nonempty() {
        // Every defined variant must have a stable, non-empty name.
        for byte in 0x00u8..=0xE1 {
            if let Ok(op) = OpCode::try_from(byte) {
                let n = op.name();
                assert!(!n.is_empty(), "byte 0x{byte:02X} has empty name");
                assert!(n.is_ascii(), "byte 0x{byte:02X} name '{n}' must be ASCII");
            }
        }
    }
}
