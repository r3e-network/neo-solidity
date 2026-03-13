# Inline Assembly

Solidity defines an assembly language that can be used without Solidity. This assembly language can also be used as "inline assembly" inside Solidity source code. 

::: tip 💡 NeoVM Difference
In Ethereum, inline assembly gives you direct access to the EVM stack and opcodes (e.g., `sload`, `sstore`, `call`). **On NeoVM, EVM inline assembly blocks are not supported** because the opcodes and stack architecture fundamentally differ.
:::

## How Neo Solidity Handles Inline Assembly

When the Neo Solidity compiler encounters an `assembly { ... }` block, it parses the syntax for source compatibility but **safely compiles it to a no-op**. 

Any logic written inside an `assembly` block will be skipped at runtime. The compiler will emit a semantic warning notifying you of this behavior:

```solidity
function doSomething() public {
    uint256 x = 5;

    // ⚠️ Warning: inline assembly block compiled as no-op: NeoVM does not support EVM assembly
    assembly {
        let y := sload(0)
        sstore(1, add(y, x))
    }
}
```

## NeoVM Alternatives

If your contract requires low-level operations, do not use `assembly`. Instead, you must use Neo's native intrinsics and Syscalls directly.

For example, to execute a cross-contract call with dynamic parameters or specific execution flags, use the `Syscalls` library rather than EVM's `call` opcode.

```solidity
import {Syscalls, CallFlags} from "@neo/Syscalls.sol";

function callAnotherContract(address target, string memory method) public {
    // Replaces the need for low-level assembly calls
    Syscalls.contractCall(target, method, CallFlags.All, new any[](0));
}
```

For advanced mathematics, cryptography, or memory manipulation, the Neo Devpack exposes a comprehensive suite of native libraries designed explicitly for the NeoVM environment.