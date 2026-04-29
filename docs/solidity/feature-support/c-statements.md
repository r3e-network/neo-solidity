---
title: "Solidity Feature Support: C. Statements"
description: "C. Statements from Solidity Feature Support."
---

# C. Statements

[Back to Solidity Feature Support](/solidity/feature-support)

| Feature                   | Status | Notes                                                                                                        |
| ------------------------- | :----: | ------------------------------------------------------------------------------------------------------------ |
| `if` / `else`             |   ✅   | Standard conditional branching.                                                                              |
| `for` loop                |   ✅   | Init, condition, post, body all lowered.                                                                     |
| `while` loop              |   ✅   | Condition + body.                                                                                            |
| `do...while` loop         |   ✅   | Body + condition.                                                                                            |
| `break`                   |   ✅   | Loop break.                                                                                                  |
| `continue`                |   ✅   | Loop continue.                                                                                               |
| `return`                  |   ✅   | Single and multi-value returns.                                                                              |
| `emit Event(...)`         |   ✅   | Maps to `Runtime.Notify`. Indexed params supported.                                                          |
| `revert(...)`             |   ✅   | Maps to NeoVM `ABORT` with message.                                                                          |
| `revert CustomError(...)` |   ✅   | Named revert with args.                                                                                      |
| Variable declaration      |   ✅   | Local variable definitions with optional initializer.                                                        |
| Block `{ ... }`           |   ✅   | Scoped statement blocks.                                                                                     |
| `unchecked { ... }`       |   ✅   | NeoVM uses BigInteger (no overflow). Unchecked blocks compile as normal blocks.                              |
| `assembly { ... }`        |   ⚠️   | Compiled as a no-op (with a warning); use `NativeCalls` for low-level ops.                                   |
| `try` / `catch`           |   ✅   | Maps to NeoVM `TRY`/`ENDTRY`. Single catch clause preferred.                                                 |
| `catch Error(string)`     |   ✅   | Named catch with parameter binding.                                                                          |
| `catch Panic(uint256)`    |   ✅   | Matches the EVM-canonical `keccak256("Panic(uint256)")[..4]` (= `0x4e487b71`) selector on the revert envelope and decodes the 32-byte BE code into the catch binding (Task #103). |
| `catch (bytes)`           |   ✅   | Low-level catch with raw bytes.                                                                              |

### Partial statement details

**`catch Panic(uint256)`** — As of Task #103, the compiler emits the EVM-canonical revert envelope (`selector || abi.encode(code)`) for `assert(false)` (0x01), div/mod by zero (0x12), enum-cast range violations (0x21), empty-array `.pop()` (0x31), and `abi.decode` short-buffer faults (0x41). The `catch Panic(uint code)` dispatcher guards on the 4-byte selector and decodes `code` via `StdLib.abiDecode`, so `code == 0x12` for div-by-zero is reachable with Ethereum-compatible semantics. (Checked-arithmetic `0x11` still uses the legacy ByteString `"Panic: 0x11"` path — see follow-up task.)

**`unchecked { ... }`** — Since NeoVM uses arbitrary-precision BigInteger, integer overflow cannot occur. The `unchecked` block is accepted for source compatibility but has no behavioral effect — all arithmetic is inherently unchecked on NeoVM.

```solidity
// Compiles identically with or without unchecked on NeoVM
unchecked {
    uint256 result = a + b; // No overflow possible — BigInteger
}
```

---
