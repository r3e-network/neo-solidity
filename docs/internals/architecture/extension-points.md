---
title: "Architecture: Extension Points"
description: "Extension Points from Architecture."
---

# Extension Points

[Back to Architecture](/internals/architecture)

### Adding a New Opcode

1. Add the opcode entry to `src/runtime/spec/opcodes.rs` with hex code, name, and gas cost
2. Implement the execution logic in `src/runtime/execution/`
3. Add IR support in `src/ir/ir_types.rs` if the opcode needs direct IR representation
4. Update the code generator in `src/cli/bytecode/` to emit the new opcode

### Adding a New Syscall

1. Add the syscall name to the registry in `src/runtime/spec/syscalls.rs`
2. Add the gas cost entry in `src/runtime/spec/gas.rs`
3. Implement the syscall handler in `src/runtime/execution/`
4. Update `src/codegen.rs` if the syscall needs a new interop ID computation

### Adding a New Native Contract

1. Add the contract hash to `src/runtime/spec/native_contracts.rs`
2. Implement method handlers in the runtime execution engine
3. Update the devpack with Solidity interface files in `devpack/contracts/`

### Adding a New Optimization Pass

1. Create a new file in `src/optimizer/` implementing the pass
2. Add the pass to `OptimizationPasses` in `src/optimizer/types.rs`
3. Wire it into the dispatch logic in `src/optimizer/dispatch.rs`
4. Assign it to the appropriate optimization level in `OptimizationPasses::for_level()`
