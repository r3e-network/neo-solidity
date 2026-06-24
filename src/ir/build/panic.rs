use super::*;

/// Task #107 — Emit the canonical EVM `Panic(uint256)` revert envelope for a
/// given panic code and THROW it.
///
/// The envelope shape is:
///   `keccak256("Panic(uint256)")[0..4] || abi.encode(uint256 code)`
///     = `0x4e487b71` (4 bytes) || `<32-byte big-endian code>`
///
/// This matches the shape consumed by `catch Panic(uint code)` in
/// `src/ir/statements/dispatch/try_catch.rs` and by the observe() helper
/// in `tests/fuzz_tests.rs`, so migrated call sites all route through the
/// same canonical path.
///
/// Before Task #103 / #107, sites emitted a raw `PushLiteral(ByteString("Panic: 0xNN")) ; Throw`
/// which the runtime surfaced only as an exception-message string and made
/// `catch Panic(uint code)` fail to bind. This helper replaces that shape
/// everywhere it was used (assert false, div/mod zero, arith over/under-flow,
/// unary negation, enum range cast, abi.decode short buffer, array pop from
/// empty) with the canonical envelope.
///
/// Stack contract: pushes the envelope ByteString then emits `Throw`. No
/// items are popped.
///
/// Emitted instructions:
/// ```text
///   PushLiteral ByteArray([0x4e, 0x48, 0x7b, 0x71])     ; selector
///   PushLiteral ByteArray([0x00 * 31, code])           ; 32-byte BE payload
///   CallBuiltin BytesConcat 2                           ; selector || payload
///   Throw
/// ```
pub(crate) fn emit_panic(code: u8, instructions: &mut Vec<Instruction>) {
    let panic_selector = {
        let mut hasher = Keccak256::new();
        hasher.update(b"Panic(uint256)");
        let digest = hasher.finalize();
        [digest[0], digest[1], digest[2], digest[3]]
    };
    instructions.push(Instruction::PushLiteral(LiteralValue::ByteArray(
        panic_selector.to_vec(),
    )));
    let mut payload = vec![0u8; 32];
    payload[31] = code;
    instructions.push(Instruction::PushLiteral(LiteralValue::ByteArray(payload)));
    instructions.push(Instruction::CallBuiltin {
        builtin: BuiltinCall::BytesConcat,
        arg_count: 2,
    });
    instructions.push(Instruction::Throw);
}
