use super::*;

pub(crate) fn emit_load_parameter(
    bytecode: &mut Vec<u8>,
    _method: &FunctionMetadata,
    index: usize,
) {
    if index <= 6 {
        bytecode.push(0x78 + index as u8);
    } else {
        bytecode.push(0x7F); // LDARG
        bytecode.push(index as u8);
    }
}

pub(crate) fn emit_load_local(bytecode: &mut Vec<u8>, index: usize) {
    match index {
        0..=6 => bytecode.push(0x68 + index as u8),
        _ => {
            bytecode.push(0x6F); // LDLOC
            bytecode.push(index as u8);
        }
    }
}

pub(crate) fn emit_store_local(bytecode: &mut Vec<u8>, index: usize) {
    match index {
        0..=6 => bytecode.push(0x70 + index as u8),
        _ => {
            bytecode.push(0x77); // STLOC
            bytecode.push(index as u8);
        }
    }
}

// Task #156 — `(a, b) = (b, a)` on function parameters. NeoVM STARG0..6 are
// 0x80..=0x86; generic STARG is 0x87 with a one-byte index operand. Mirrors
// `emit_load_parameter` (0x78..=0x7E / 0x7F) so the store opcode pairs with
// the existing load for the same parameter slot.
pub(crate) fn emit_store_parameter(bytecode: &mut Vec<u8>, index: usize) {
    if index <= 6 {
        bytecode.push(0x80 + index as u8);
    } else {
        bytecode.push(0x87); // STARG
        bytecode.push(index as u8);
    }
}
