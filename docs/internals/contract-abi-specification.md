# Contract ABI Specification

The Application Binary Interface (ABI) is the standard way to interact with contracts in the ecosystem. 

::: tip 💡 NeoVM Difference
Ethereum relies on a tightly-packed binary encoding scheme and 4-byte `keccak256` function selectors. **Neo uses a structured JSON Manifest ABI and dispatches methods by string name.** 
:::

## Basic Design

When you compile a contract, Neo Solidity generates a `.manifest.json` file. The `abi` object inside this manifest dictates exactly how clients and other contracts interact with your code.

### Function Selector vs Method Name

In EVM, `transfer(address,uint256)` hashes to the selector `0xa9059cbb`. 

On Neo, the function name itself (`"transfer"`) is the dispatch mechanism. The NeoVM executes a `System.Contract.Call` syscall, providing the target contract hash and the exact string `"transfer"`.

**Overload Collisions:**
Because Neo dispatches by name, overloading functions with the same name but different parameters will cause collisions in the manifest unless handled carefully.

```solidity
// ⚠️ Both produce "transfer" in Neo manifest — collision
function transfer(address to, uint256 amount) public { }
function transfer(address to, uint256 amount, bytes calldata data) public { }

// ✅ Distinct names — no collision
function transfer(address to, uint256 amount) public { }
function transferWithData(address to, uint256 amount, bytes calldata data) public { }
```

## Argument Types

Solidity types are lowered to the standard Neo N3 ABI types:

| Solidity Type         | Neo ABI Type |
| --------------------- | ------------ |
| `address`             | `Hash160`    |
| `uint256` / `int256`  | `Integer`    |
| `bool`                | `Boolean`    |
| `string`              | `String`     |
| `bytes`               | `ByteArray`  |
| `bytes32`             | `Hash256`    |
| `address[]`           | `Array`      |
| `struct`              | `Array`      |

When interacting with the contract via Neo-Express or an SDK, you must pass arguments matching the `Neo ABI Type`, not the raw EVM types.

## The `safe` Flag

Neo's ABI includes a `safe` boolean for every method. This indicates whether a method is read-only.
* `view` and `pure` functions in Solidity are compiled with `"safe": true`.
* All other functions are compiled with `"safe": false`.

Methods marked as safe can be called via RPC test invocations without requiring a transaction signature or incurring GAS fees.