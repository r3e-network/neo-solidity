---
title: "Devpack Overview: Libraries"
description: "Libraries from Devpack Overview."
---

# Libraries

[Back to Devpack Overview](/additional-material/neo-devpack)

## Neo.sol (542 lines)

High-level blockchain integration library. Wraps `Syscalls` and `NativeCalls` into a convenient API:

| Category        | Functions                                                                      |
| --------------- | ------------------------------------------------------------------------------ |
| Block info      | `getCurrentBlock()`, `getBlockByIndex()`, `getBlockHeight()`, `getBlockTime()` |
| Transactions    | `getTransaction()`, `getTransactionHeight()`                                   |
| Account/balance | NEO and GAS balance queries via NativeCalls                                    |
| Cryptographic   | Signature verification, hash functions (delegates to Syscalls CryptoLib)       |
| Contract mgmt   | Call, deploy, query contracts                                                  |
| Network         | Governance, committee, validators                                              |
| Gas             | Fee calculations, gas management utilities                                     |

```solidity
using Neo for *;

(uint256 index, bytes32 hash, uint256 timestamp, bytes32 merkle) = Neo.getCurrentBlock();
uint256 height = Neo.getBlockHeight();
```

## Storage.sol (852 lines)

Advanced storage operations built on top of `Syscalls` storage syscalls:

| Category        | Functions                                                                                     |
| --------------- | --------------------------------------------------------------------------------------------- |
| Context         | `getContext()`, `getReadOnlyContext()`, `asReadOnly()`                                        |
| Basic CRUD      | `put()`, `get()`, `remove()`, `exists()`                                                      |
| Local storage   | `putLocal()`, `getLocal()`, `removeLocal()`                                                   |
| Iterators       | `find()`, `findLocal()`, `findValues()`, `findKeys()`, `findLocalValues()`, `findLocalKeys()` |
| Counting        | `count()` — count entries matching a prefix                                                   |
| Batch ops       | Batch put/remove (max 100 per batch)                                                          |
| Key derivation  | Mapping keys, array keys, nested keys                                                         |
| Typed accessors | `uint256`, `address`, `string`, `bool` typed get/put                                          |
| Secure storage  | Checksum validation                                                                           |
| Expiration      | TTL-based storage entries                                                                     |

```solidity
using Storage for *;

// Basic operations
Storage.put("owner", abi.encode(msg.sender));
bytes memory data = Storage.get("owner");
bool hasKey = Storage.exists("owner");

// Iterator-based prefix scan
bytes[] memory allValues = Storage.findValues("token:");
bytes[] memory allKeys = Storage.findKeys("token:");

// Local storage (contract-private, not visible to other contracts)
Storage.putLocal("internal_state", abi.encode(42));
```

::: info Local vs Global Storage
`put()`/`get()` use the contract's global storage context. `putLocal()`/`getLocal()` use a local context that is private to the contract and cannot be read by other contracts via `System.Storage.GetReadOnlyContext`.
:::

## Runtime.sol (695 lines)

Runtime services and utilities (currently supported as compiler intrinsics):

| Category         | Functions                                                                                              |
| ---------------- | ------------------------------------------------------------------------------------------------------ |
| Events           | `notify()`, `notifyIndexed()`                                                                          |
| Witness          | `checkWitness()`, `requireWitness()`, `checkAnyWitness()`, `checkAllWitnesses()`, `checkMultiSigWitness()` |
| Runtime context  | `gasLeft()`, `getTime()`, `getTrigger()`, `getInvocationCounter()`, `getCurrentSigners()`            |
| Contract context | `getCallFlags()`, `getScriptContainer()`, `loadScript()`, `getExecutingScriptHash()`, `getCallingScriptHash()`, `getEntryScriptHash()` |
| Platform         | `getNetwork()`, `getPlatform()`, `getAddressVersion()`, `getRandom()`                                 |
| Logging          | `log()`, `burnGas()`, `initializeServices()`                                                           |

```solidity
using Runtime for *;

// Witness verification
bool hasWitness = Runtime.checkWitness(msg.sender);
require(hasWitness, "invalid witness");

// Gas management
uint256 remaining = Runtime.gasLeft();
Runtime.burnGas(1000000); // burn 0.01 GAS
```

::: warning Runtime Intrinsic Coverage
`Runtime.sol` includes callback-oriented convenience helpers that are not compiler intrinsics in neo-solidity. Calls to `optimizeGasUsage`, `executeIfGasAvailable`, and `tryWithFallback` are currently rejected at compile time because NeoVM does not support first-class internal function callbacks. Use inline logic (`gasLeft` guards and `try/catch`) instead.
:::
