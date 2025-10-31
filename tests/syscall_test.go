package main

import (
    "encoding/binary"
    "testing"
)

// Test that NewSyscallInstruction encodes Neo N3 interop ID correctly:
// first 4 bytes of sha256(method) in little-endian order.
func TestNewSyscallInstructionInteropID(t *testing.T) {
    cases := []string{
        "System.Contract.Call",
        "System.Storage.Put",
        "System.Storage.Get",
        "Neo.Crypto.Sha256",
        "Neo.Crypto.Keccak256",
    }

    for _, method := range cases {
        instr := NewSyscallInstruction(method)
        if instr.Opcode != SYSCALL {
            t.Fatalf("expected SYSCALL opcode for %s", method)
        }
        if len(instr.Operand) != 4 {
            t.Fatalf("expected 4-byte interop ID for %s, got %d", method, len(instr.Operand))
        }

        // Compute expected 4-byte LE prefix of sha256(method)
        expected := computeInteropID(method)
        // Verify LE uint32 matches for clarity
        gotLE := binary.LittleEndian.Uint32(instr.Operand)
        expLE := binary.LittleEndian.Uint32(expected)
        if gotLE != expLE {
            t.Fatalf("interop ID mismatch for %s: got 0x%08x, expected 0x%08x", method, gotLE, expLE)
        }
    }
}
