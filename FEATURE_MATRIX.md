# Feature Matrix

Solidity feature support status for the Neo Solidity Compiler (`neo-solc`).

**Legend**: Supported | Partial | Not Supported

---

## Types

| Solidity Feature        | Status    | Notes                                                                  |
| ----------------------- | --------- | ---------------------------------------------------------------------- |
| `bool`                  | Supported | Maps to NeoVM Boolean                                                  |
| `uint8` - `uint256`     | Supported | All widths; stored as BigInteger on NeoVM                              |
| `int8` - `int256`       | Supported | All widths; signed BigInteger                                          |
| `address`               | Supported | Maps to 20-byte Hash160 (UInt160)                                      |
| `bytes1` - `bytes32`    | Supported | Fixed-size byte arrays                                                 |
| `bytes` (dynamic)       | Supported | Variable-length ByteString                                             |
| `string`                | Supported | UTF-8 ByteString                                                       |
| `arrays` (fixed)        | Supported | Compile-time sized arrays                                              |
| `arrays` (dynamic)      | Supported | Storage-backed with push/pop/length                                    |
| `mapping`               | Supported | Key-value via Neo Storage prefix model                                 |
| `struct`                | Supported | Value types and nested structs                                         |
| `enum`                  | Supported | Compiled to uint8 constants                                            |
| `function` types        | Partial   | Internal function pointers only; external function types not supported |
| `type(...)` expressions | Supported | `.min`, `.max`, `.name`, `.interfaceId` for integer/contract types     |

## Control Flow

| Solidity Feature       | Status    | Notes                                                                                                                     |
| ---------------------- | --------- | ------------------------------------------------------------------------------------------------------------------------- |
| `if` / `else`          | Supported | Standard conditional branching                                                                                            |
| `for` loop             | Supported | Including `for(;;)` infinite loops                                                                                        |
| `while` loop           | Supported | Pre-condition loop                                                                                                        |
| `do...while` loop      | Supported | Post-condition loop                                                                                                       |
| `break`                | Supported | Loop exit                                                                                                                 |
| `continue`             | Supported | Loop skip                                                                                                                 |
| `return`               | Supported | Single and multi-value returns                                                                                            |
| `try` / `catch`        | Supported | Multi-clause dispatch lowered via runtime `ISTYPE` guards; selector-level EVM error distinction is not available on NeoVM |
| `revert`               | Supported | With and without error message                                                                                            |
| `revert` custom errors | Supported | `error MyError(uint256)` syntax                                                                                           |
| `require`              | Supported | With optional message string or `CustomError(...)` (Solidity 0.8.26+)                                                     |
| `assert`               | Supported | Maps to NeoVM ABORT on failure                                                                                            |
| Ternary `? :`          | Supported | Conditional expression                                                                                                    |

## Functions

| Solidity Feature     | Status        | Notes                                                                        |
| -------------------- | ------------- | ---------------------------------------------------------------------------- |
| `public`             | Supported     | Exported in manifest ABI                                                     |
| `external`           | Supported     | Treated as `public` in Neo context                                           |
| `internal`           | Supported     | Intra-contract calls                                                         |
| `private`            | Supported     | Contract-scoped only                                                         |
| `view`               | Supported     | Manifest safe annotation                                                     |
| `pure`               | Supported     | No storage access                                                            |
| `payable`            | Partial       | Accepted with warning; use `onNEP17Payment` for value receipt                |
| `modifiers`          | Supported     | `_` placeholder expansion                                                    |
| Function overloading | Partial       | Different arg counts supported; same arg count rejected (Neo ABI limitation) |
| Named arguments      | Supported     | `f({x: 1, y: 2})` reordered to positional order at compile time              |
| Default parameters   | Not Supported | Not part of Solidity spec                                                    |
| `fallback()`         | Partial       | Compiled with warning; Neo uses `onNEP17Payment` for value receipt           |
| `receive()`          | Partial       | Compiled with warning; Neo uses `onNEP17Payment` for value receipt           |
| `constructor`        | Supported     | Mapped to `_deploy(data, update)`                                            |

## Object-Oriented

| Solidity Feature       | Status    | Notes                                                                                                                                                                                                             |
| ---------------------- | --------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Single inheritance     | Supported | Linear C3 linearization                                                                                                                                                                                           |
| Multiple inheritance   | Partial   | Supported with limitations on diamond patterns                                                                                                                                                                    |
| `interface`            | Supported | Abstract method declarations                                                                                                                                                                                      |
| `abstract contract`    | Supported | Cannot be deployed directly                                                                                                                                                                                       |
| `virtual` / `override` | Supported | Method override dispatch                                                                                                                                                                                          |
| `super`                | Supported | C3 linearization with `__super_` method preservation through inheritance                                                                                                                                          |
| `using ... for`        | Supported | Library member-call syntax; internal functions inlined at call site                                                                                                                                               |
| Libraries              | Partial   | User-defined libraries are merged/inlined into consuming contracts; `public` / `external` functions are normalized to internal helpers with warnings, but deployable/linkable library semantics are not available |

## Storage

| Solidity Feature          | Status    | Notes                                                         |
| ------------------------- | --------- | ------------------------------------------------------------- |
| State variables           | Supported | Persisted via Neo Storage syscalls                            |
| `constant`                | Supported | Inlined at compile time                                       |
| `immutable`               | Partial   | Writes outside constructor/deploy initialization are rejected |
| Nested mappings           | Supported | Prefix-based key concatenation                                |
| Dynamic arrays in storage | Supported | Length-prefixed with index keys                               |
| `delete`                  | Supported | Resets to default value / removes storage key                 |

## Events

| Solidity Feature   | Status    | Notes                                                |
| ------------------ | --------- | ---------------------------------------------------- |
| `emit`             | Supported | Maps to `System.Runtime.Notify`                      |
| Indexed parameters | Supported | Up to 3 indexed params per event                     |
| Anonymous events   | Partial   | Compiled but topic[0] omission not guaranteed on Neo |
| Custom event types | Supported | Manifest includes event definitions                  |

## Imports

| Solidity Feature                   | Status    | Notes                                                                                                                                                                                                       |
| ---------------------------------- | --------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Plain import (`import "file.sol"`) | Supported | Resolved via `-I` include paths                                                                                                                                                                             |
| Named import (`import {X} from`)   | Supported | Selective symbol import                                                                                                                                                                                     |
| Aliased import (`import {X as Y}`) | Partial   | Dependency resolution, aliased static symbol calls, contract/interface casts, selector forms, and `abi.encodeCall` function references are supported; broader namespace rewriting remains limited           |
| Wildcard import (`import * as X`)  | Partial   | Dependency resolution plus `X.Symbol.member(...)` static calls, `X.Symbol(addr)` casts, selector forms, and `abi.encodeCall` function references are supported; broader namespace rewriting remains limited |
| Remappings                         | Partial   | `-I` flag provides basic path remapping                                                                                                                                                                     |

## ABI Encoding

| Solidity Feature          | Status    | Notes                                           |
| ------------------------- | --------- | ----------------------------------------------- |
| `abi.encode`              | Supported | Maps to `StdLib.serialize`                      |
| `abi.decode`              | Supported | Maps to `StdLib.deserialize`                    |
| `abi.encodePacked`        | Partial   | Concatenation-based encoding                    |
| `abi.encodeWithSignature` | Supported | Used for low-level call lowering                |
| `abi.encodeWithSelector`  | Supported | Used for low-level call lowering                |
| `abi.encodeCall`          | Supported | Maps to `StdLib.serialize` (same as abi.encode) |

## Neo N3 Specific

| Solidity Feature                 | Status    | Notes                                          |
| -------------------------------- | --------- | ---------------------------------------------- |
| `Runtime.checkWitness`           | Supported | Via devpack `Runtime.sol`                      |
| `Runtime.getTime`                | Supported | Block timestamp                                |
| `Runtime.getInvocationCounter`   | Supported | Call depth counter                             |
| `Runtime.getRandom`              | Supported | On-chain randomness (mock in embedded runtime) |
| `Runtime.getNetwork`             | Supported | Network ID (mainnet/testnet)                   |
| `Runtime.getPlatform`            | Supported | Platform info                                  |
| `Runtime.getAddressVersion`      | Supported | Address version                                |
| `Runtime.getExecutingScriptHash` | Supported | Current contract hash                          |
| `Runtime.getCallingScriptHash`   | Supported | Caller contract hash                           |
| `Runtime.getEntryScriptHash`     | Supported | Entry script hash                              |
| `Storage.get` / `Storage.put`    | Supported | Key-value storage syscalls                     |
| `Storage.find`                   | Supported | Iterator-based prefix search                   |
| `Storage.batchPut/Get/Delete`    | Supported | Batch storage operations                       |
| `Storage.count/Local`            | Supported | Count entries in prefix                        |
| `Storage.findValues/Keys`        | Supported | Find values or keys by prefix                  |
| Native contract calls (GAS, NEO) | Supported | Via `NativeCalls.sol` wrappers                 |
| `NativeCalls.neoName/gasName`    | Supported | Get native token names                         |
| Policy contract                  | Supported | Fee and blocked-account queries                |
| Oracle contract                  | Supported | Request/response pattern with callback         |
| RoleManagement                   | Supported | Designated node queries                        |
| Ledger contract                  | Supported | Block and transaction queries                  |
| ContractManagement               | Supported | Deploy, update, destroy lifecycle              |
| NEP-17 standard                  | Supported | Full token standard via devpack                |
| NEP-11 standard                  | Supported | Full NFT standard via devpack                  |
| `onNEP17Payment`                 | Supported | Token receive callback                         |
| `_deploy(data, update)`          | Supported | Constructor and upgrade entry point            |

## EVM Feature Compatibility

| Solidity Feature                        | Status        | Notes                                                                                                   |
| --------------------------------------- | ------------- | ------------------------------------------------------------------------------------------------------- | --- | ------------------------------------------------ |
| Inline assembly (`assembly {}`)         | Partial       | Compiles as no-op with warning; special handlers for extsload/exttload                                  |
| `delegatecall`                          | Partial       | Compiles as `System.Contract.Call` with warning; isolated storage semantics differ                      |
| `selfdestruct`                          | Supported     | Auto-mapped to `ContractManagement.destroy()` with warning                                              |
| `create` / `create2`                    | Not Supported | Use `ContractManagement.deploy` instead                                                                 |
| `tx.origin`                             | Partial       | Compiles with warning; Neo uses multi-sig witnesses                                                     |
| `tx.hash`                               | Supported     | Auto-mapped to System.Runtime.GetScriptContainer                                                        |
| `block.coinbase`                        | Supported     | Auto-mapped to `address(0)` (dBFT has no miner)                                                         |
| `block.difficulty` / `block.prevrandao` | Supported     | Auto-mapped to `Runtime.getRandom()` with warning                                                       |
| `gasleft()`                             | Supported     | Auto-mapped to `System.Runtime.GasLeft` syscall                                                         |
| `block.gaslimit`                        | Supported     | Auto-mapped to `Policy.getExecFeeFactor()` with warning                                                 |
| `block.basefee`                         | Supported     | Auto-mapped to `Policy.getFeePerByte()` with warning                                                    |
| `tx.gasprice`                           | Supported     | Auto-mapped to `Policy.getFeePerByte()` with warning                                                    |
| `blockhash()`                           | Supported     | Auto-mapped to `Ledger.getBlockHash()` with warning                                                     |
| `block.parenthash`                      | Supported     | Auto-mapped to Ledger.currentHash                                                                       |
| `block.sha3`                            | Supported     | Auto-mapped to Ledger.currentHash (the current block's hash) with warning. Deprecated in Solidity 0.8+. |
| `address.codehash`                      | Supported     | Auto-mapped to contract script hash with warning                                                        |
| `address.balance`                       | Supported     | Auto-mapped to Gas.balanceOf                                                                            |
| `address.call`                          | Supported     | Auto-mapped to System.Contract.Call                                                                     |
| `address.staticcall`                    | Supported     | Auto-mapped to System.Contract.Call (read-only)                                                         |
| `address.transfer`                      | Supported     | Auto-mapped to Gas.transfer                                                                             |
| `address.send`                          | Supported     | Auto-mapped to Gas.transfer                                                                             |
| `msg.value`                             | Partial       | Mapped inside `onNEP17Payment` callback; not available elsewhere                                        |
| `msg.data`                              | Supported     | Approximated as `selector                                                                               |     | abi.encode(current args)` outside onNEP17Payment |
| `msg.sig`                               | Supported     | Compiles to empty bytes4 (method dispatch by name on Neo N3)                                            |
| Yul / inline Yul                        | Not Supported | Compiler accepts Solidity source only                                                                   |
| User-defined value types                | Supported     | `type X is Y` transparent aliases; `wrap`/`unwrap` are no-ops                                           |
| Transient storage (`tstore`/`tload`)    | Not Supported | EIP-1153; no Neo equivalent                                                                             |
