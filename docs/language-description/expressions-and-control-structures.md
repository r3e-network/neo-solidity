# Expressions and Control Structures

## Control Structures

Most of the control structures from JavaScript are available in Solidity:
`if`, `else`, `while`, `do`, `for`, `break`, `continue`, `return`.

All of these are fully supported by the Neo Solidity compiler and lower perfectly to NeoVM JMP logic.

## Function Calls

### Internal Function Calls

Functions of the current contract can be called directly. These map to `CALL` instructions within the NeoVM script.

### External Function Calls

The expressions `this.g(8);` and `c.g(2);` (where `c` is a contract instance) are also valid function calls. They are translated into cross-contract calls using `System.Contract.Call`. 

On Ethereum, you can append options to an external call to forward Ether or set a gas limit: `c.g{value: 10, gas: 10000}(2)`. 
* **NeoVM Mapping:** Because NeoVM handles value transfers strictly via the NEP-17 standard and lacks per-call gas limits, these call options are gracefully ignored. The compiler emits a warning notifying you that the options were dropped, but compilation succeeds.

## Error Handling: Assert, Require, Revert and Exceptions

Solidity uses state-reverting exceptions to handle errors. Such an exception undoes all changes made to the state in the current call (and all its sub-calls) and flags an error to the caller.

### `require` and `assert`

`require(condition)` and `require(condition, "message")` map to NeoVM `ASSERT` and `ASSERTMSG` opcodes. When the condition is false, execution aborts with the provided message.

On EVM, `assert` failures consume all remaining gas and produce a Panic error. On NeoVM, `assert` simply aborts execution — there is no gas penalty distinction between `require` and `assert`.

### `revert`

`revert()` and `revert("message")` map to NeoVM `ABORT` and `ABORTMSG`. Custom error reverts preserve the error name and arguments.

```solidity
error Unauthorized(address caller);

function restricted() public {
    if (msg.sender != owner) {
        revert Unauthorized(msg.sender);
        // NeoVM: ABORTMSG with "Unauthorized" and caller arg
    }
}
```

### `try` / `catch`

`try`/`catch` maps to NeoVM `TRY`/`ENDTRY` structured exception handling. The try block wraps the external call, and catch clauses handle exceptions.

```solidity
try target.riskyCall() returns (uint256 result) {
    // Success path
    processResult(result);
} catch (bytes memory lowLevelData) {
    // Catch-all for other exceptions
    emit CallFailedRaw(lowLevelData);
}
```

## Creating Contracts via `new`

In EVM Solidity, a contract can create other contracts using the `new` keyword. 

* **NeoVM Mapping:** Creating contracts dynamically via `new Contract()` is intentionally blocked. NeoVM handles deployment differently. To deploy a child contract, you must use Neo's `ContractManagement.deploy(nef, manifest, data)` intrinsic directly.

## Arithmetic 

NeoVM uses arbitrary-precision `BigInteger` for all integer arithmetic. This fundamentally changes overflow semantics compared to EVM:

| Behavior                       | EVM (Solidity 0.8.x) | NeoVM                         |
| ------------------------------ | -------------------- | ----------------------------- |
| `uint8(255) + 1`               | Reverts (checked)    | `256` (no overflow)           |
| `uint8(0) - 1`                 | Reverts (checked)    | `-1` (no underflow)           |
| `unchecked { uint8(255) + 1 }` | `0` (wraps)          | `256` (no wrap)               |
| `int256.max + 1`               | Reverts (checked)    | `int256.max + 1` (BigInteger) |