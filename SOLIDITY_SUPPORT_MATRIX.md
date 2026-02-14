# Solidity 0.8.x Support Matrix

> **Compiler**: neo-solidity v0.9.10
> **Parser**: solang-parser 0.3.5
> **Target**: NeoVM (Neo N3)
> **Audit date**: 2026-02-11

Legend:

- ✅ Fully supported
- ⚠️ Partial support (see notes)
- ❌ Not supported
- 🚫 Intentionally blocked with diagnostic error

---

## A. Types

| Feature                                  | Status | Notes                                                                 |
| ---------------------------------------- | ------ | --------------------------------------------------------------------- |
| `bool`                                   | ✅     | Maps to NeoVM Boolean                                                 |
| `int8`..`int256`                         | ✅     | All widths parsed; NeoVM uses arbitrary-precision BigInteger          |
| `uint8`..`uint256`                       | ✅     | All widths parsed; NeoVM uses arbitrary-precision BigInteger          |
| `address`                                | ✅     | Maps to Neo UInt160 (Hash160, 20 bytes)                               |
| `address payable`                        | ⚠️     | Parsed and canonicalized to `address`; `transfer`/`send` are EVM-only |
| `bytes1`..`bytes32`                      | ✅     | Fixed-length byte arrays via `NeoType::ByteArray { fixed_len }`       |
| `bytes` (dynamic)                        | ✅     | Dynamic byte array                                                    |
| `string`                                 | ✅     | UTF-8 string type                                                     |
| `enum`                                   | ✅     | Converted via `convert_enum`; backed by uint8                         |
| `struct`                                 | ✅     | Full struct support with nested fields; `StructDefinition` converted  |
| `mapping(K => V)`                        | ✅     | Storage mappings with Neo StorageMap; key type validation enforced    |
| `T[]` (dynamic array)                    | ✅     | `new T[](n)` allocation supported                                     |
| `T[N]` (fixed array)                     | ⚠️     | Parsed; `new T[N]` supported when `N` is compile-time constant         |
| `fixed` / `ufixed`                       | ❌     | Not supported (also unsupported in mainline Solidity)                 |
| User-defined value types (`type X is Y`) | ✅     | Transparent type aliases; `wrap`/`unwrap` compile to no-ops           |
| `bytes.concat(...)`                      | ✅     | Chains NeoVM CAT opcodes; zero args produce empty byte array          |
| `string.concat(...)`                     | ✅     | Same implementation as `bytes.concat` via CAT opcode chain            |
| Contract types (e.g., `IERC20`)          | ✅     | Resolved to Neo UInt160 address; interface types tracked              |
| Tuple types                              | ✅     | Represented as NeoVM arrays internally                                |

---

## B. Expressions

| Feature                                       | Status | Notes                                                                   |
| --------------------------------------------- | ------ | ----------------------------------------------------------------------- |
| Arithmetic (`+`, `-`, `*`, `/`, `%`)          | ✅     | Binary ops via `try_lower_expression_binary_ops`                        |
| Comparison (`==`, `!=`, `<`, `>`, `<=`, `>=`) | ✅     | Via `try_lower_expression_comparisons`                                  |
| Logical (`&&`, `\|\|`, `!`)                   | ✅     | Short-circuit evaluation in `logical.rs`                                |
| Bitwise (`&`, `\|`, `^`, `~`, `<<`, `>>`)     | ✅     | Full bitwise support                                                    |
| Unary (`++`, `--`, `-`, `!`)                  | ✅     | Pre/post increment/decrement                                            |
| Ternary (`? :`)                               | ✅     | `ConditionalOperator` lowered with labels                               |
| Assignment (`=`, `+=`, `-=`, etc.)            | ✅     | Compound assignments in `assignments/compound.rs`                       |
| `delete`                                      | ✅     | State vars, mapping entries, locals, array elements, struct fields      |
| Tuple expressions `(a, b, c)`                 | ✅     | `Expression::List` lowered to NeoVM arrays                              |
| Tuple destructuring `(a, b) = f()`            | ⚠️     | Nested destructuring assignment supported; some complex target forms still require intermediate locals |
| Type casting                                  | ✅     | `TypeCastingShowcase.sol` example compiles                              |
| `type(X).min` / `type(X).max`                 | ✅     | Supported for integer types                                             |
| `type(T).name`                                | ✅     | Compile-time string constant for contract/type names                    |
| `type(I).interfaceId`                         | ✅     | Computed from selector XOR of interface methods                         |
| `abi.encode(...)`                             | ⚠️     | Supported in context of `address.call`/`staticcall`; standalone limited |
| `abi.encodePacked(...)`                       | ⚠️     | Same as `abi.encode` — used for Neo contract call encoding              |
| `abi.encodeWithSignature(...)`                | ✅     | Lowered to Neo `System.Contract.Call`                                   |
| `abi.encodeWithSelector(...)`                 | ✅     | Lowered to Neo `System.Contract.Call`                                   |
| `abi.encodeCall(...)`                         | ✅     | Maps to `StdLib.serialize` (same as `abi.encode`)                       |
| `abi.decode(...)`                             | ✅     | Maps to `StdLib.deserialize`; type tuple parsed from second argument    |
| Named function call args `f({x: 1})`          | ✅     | Named args reordered to positional order at IR level                    |

---

## C. Statements

| Feature                   | Status | Notes                                                                          |
| ------------------------- | ------ | ------------------------------------------------------------------------------ |
| `if` / `else`             | ✅     | Standard conditional branching                                                 |
| `for` loop                | ✅     | Init, condition, post, body all lowered                                        |
| `while` loop              | ✅     | Condition + body                                                               |
| `do...while` loop         | ✅     | Body + condition                                                               |
| `break`                   | ✅     | Loop break                                                                     |
| `continue`                | ✅     | Loop continue                                                                  |
| `return`                  | ✅     | Single and multi-value returns                                                 |
| `emit Event(...)`         | ✅     | Maps to `Runtime.Notify`; indexed params supported                             |
| `revert(...)`             | ✅     | Maps to NeoVM `ABORT` with message                                             |
| `revert CustomError(...)` | ✅     | Named revert with args; `RevertNamedArgs` also handled                         |
| Variable declaration      | ✅     | Local variable definitions with optional initializer                           |
| Block `{ ... }`           | ✅     | Scoped statement blocks                                                        |
| `unchecked { ... }`       | ✅     | NeoVM uses BigInteger (no overflow); unchecked blocks compile as normal blocks |
| `assembly { ... }`        | 🚫     | Blocked: "inline assembly is not supported — use NativeCalls.sol"              |
| `try` / `catch`           | ✅     | Maps to NeoVM TRY/ENDTRY; single catch clause preferred                        |
| `catch Error(string)`     | ✅     | Named catch with parameter binding                                             |
| `catch Panic(uint256)`    | ⚠️     | Lowered with runtime integer-type guard; values are NeoVM exception payloads, not canonical EVM panic codes |
| `catch (bytes)`           | ✅     | Low-level catch with raw bytes                                                 |

---

## D. Functions

| Feature                          | Status | Notes                                                                            |
| -------------------------------- | ------ | -------------------------------------------------------------------------------- |
| Regular functions                | ✅     | Public, external, internal, private                                              |
| Constructor                      | ✅     | Single constructor; multiple constructors rejected                               |
| `view` / `pure`                  | ✅     | State mutability tracked and enforced at IR level                                |
| `payable`                        | ⚠️     | Parsed; `payable` on non-receive functions warns (Neo has no native gas payment) |
| `returns (T)`                    | ✅     | Single return type                                                               |
| `returns (T1, T2, ...)`          | ✅     | Multi-return via NeoVM arrays                                                    |
| Function overloading             | ⚠️     | Parsed; Neo ABI dispatches by name only — overloads may collide                  |
| `modifier`                       | ✅     | Full modifier expansion with `_` placeholder substitution                        |
| `receive()`                      | ⚠️     | Parsed; diagnostic suggests `onNEP17Payment()` callback                          |
| `fallback()`                     | ⚠️     | Parsed; diagnostic suggests `onNEP17Payment()` callback                          |
| `virtual` / `override`           | ✅     | Inheritance flattening resolves overrides; multi-level chains supported          |
| Function selectors (`.selector`) | ✅     | Computed from canonical parameter types                                          |
| NatSpec comments                 | ✅     | `@notice`, `@dev`, `@param`, `@return` preserved in metadata                     |

---

## E. OOP Features

| Feature              | Status | Notes                                                                                                                   |
| -------------------- | ------ | ----------------------------------------------------------------------------------------------------------------------- |
| Single inheritance   | ✅     | C3 linearization with `flatten_contract_inheritance`                                                                    |
| Multiple inheritance | ✅     | Diamond inheritance detected; constructor arg conflicts reported                                                        |
| `interface`          | ✅     | Interface types tracked; methods validated                                                                              |
| `abstract contract`  | ✅     | Fully validated; unimplemented functions detected; non-abstract contracts get actionable errors                         |
| `library`            | ⚠️     | Builtin devpack libraries (Runtime, Syscalls, etc.) are compiler intrinsics; user-defined libraries partially supported |
| `using X for Y`      | ✅     | Library member-call syntax fully supported; `using X for *` and `using {f,g} for T` included                            |
| `super` keyword      | ✅     | Supported via inheritance flattening with `__super_` method preservation                                                |
| `is` (inheritance)   | ✅     | Inheritance specifiers fully processed                                                                                  |
| Constructor chaining | ✅     | Base constructor arguments resolved from inheritance specifiers                                                         |
| Event inheritance    | ✅     | Interface events collected recursively via `collect_interface_events_recursive`                                         |

---

## F. Storage & Memory

| Feature                          | Status | Notes                                                                 |
| -------------------------------- | ------ | --------------------------------------------------------------------- |
| State variables                  | ✅     | Mapped to Neo Storage with prefix-based keys                          |
| `constant`                       | ✅     | Compile-time constants inlined                                        |
| `immutable`                      | ✅     | Tracked via `is_immutable` flag; modification blocked at compile time |
| `memory` keyword                 | ✅     | Parsed; NeoVM is stack-based so memory is implicit                    |
| `storage` keyword                | ✅     | Storage references for mappings and state variables                   |
| `calldata` keyword               | ✅     | Parsed; treated as `memory` (correct for NeoVM — no calldata region)  |
| Nested mappings                  | ✅     | `mapping(K1 => mapping(K2 => V))` with composite storage keys         |
| Struct in storage                | ✅     | Serialized/deserialized via `StdLib.serialize`/`StdLib.deserialize`   |
| Array `.push()` / `.pop()`       | ✅     | Storage array operations supported                                    |
| Array `.length`                  | ✅     | Both memory and storage arrays                                        |
| `new bytes(n)` / `new string(n)` | ✅     | Buffer allocation via `NEWBUFFER`                                     |
| `new T[](n)`                     | ✅     | Dynamic array allocation via `NEWARRAY`                               |
| `new Contract(...)`              | 🚫     | Blocked: "use ContractManagement for contract deployment"             |

---

## G. Error Handling

| Feature                                   | Status | Notes                                                     |
| ----------------------------------------- | ------ | --------------------------------------------------------- |
| `require(condition)`                      | ✅     | Maps to NeoVM ASSERT                                      |
| `require(condition, "msg")`               | ✅     | ASSERT with message                                       |
| `require(condition, CustomError(...))`    | ✅     | Error name and arg count preserved in NeoVM THROW message |
| `assert(condition)`                       | ✅     | Maps to NeoVM ASSERT                                      |
| `revert()`                                | ✅     | Maps to NeoVM ABORT                                       |
| `revert("message")`                       | ✅     | ABORT with message                                        |
| `revert CustomError(...)`                 | ✅     | Named revert with arguments                               |
| Custom error definitions (`error X(...)`) | ✅     | Parsed and used in revert statements                      |
| `try` / `catch`                           | ✅     | NeoVM TRY/ENDTRY structured exception handling            |
| `try` with return binding                 | ✅     | `try f() returns (uint r) { ... }` supported              |
| Multiple catch clauses                    | ⚠️     | Lowered with runtime stack-item type guards (`ISTYPE`); selector-level `Error`/`Panic` distinction remains limited |

---

## H. EVM-Specific Features (with Neo Alternatives)

| Feature                                 | Status | Neo Alternative                                                             |
| --------------------------------------- | ------ | --------------------------------------------------------------------------- |
| `msg.sender`                            | ✅     | Maps to `Runtime.GetCallingScriptHash()`                                    |
| `msg.value`                             | ⚠️     | Only mapped inside `onNEP17Payment` callback                                |
| `msg.data`                              | ❌     | No equivalent (Neo uses typed parameters)                                   |
| `msg.sig`                               | ❌     | No equivalent                                                               |
| `block.timestamp`                       | ✅     | Maps to `Runtime.GetTime()` (normalized to seconds)                         |
| `block.number`                          | ✅     | Maps to `Ledger.CurrentIndex()`                                             |
| `block.chainid`                         | ✅     | Maps to Neo network magic number                                            |
| `block.coinbase`                        | ✅     | Auto-mapped to `address(0)` with warning (dBFT has no miner)                |
| `block.difficulty` / `block.prevrandao` | ✅     | Auto-mapped to `Runtime.getRandom()` with warning                           |
| `block.gaslimit`                        | ✅     | Auto-mapped to `Policy.getExecFeeFactor()` with warning                     |
| `block.basefee`                         | ✅     | Auto-mapped to `Policy.getFeePerByte()` with warning                        |
| `tx.origin`                             | ⚠️     | Parsed; warning about authorization risks                                   |
| `tx.gasprice`                           | ✅     | Auto-mapped to `Policy.getFeePerByte()` with warning                        |
| `gasleft()`                             | ✅     | Auto-mapped to `System.Runtime.GasLeft` syscall                             |
| `blockhash(n)`                          | ✅     | Auto-mapped to `Ledger.getBlockHash()` with warning                         |
| `keccak256(...)`                        | ✅     | Maps to `CryptoLib.keccak256`                                                |
| `sha256(...)`                           | ✅     | Maps to `CryptoLib.sha256`                                                  |
| `ecrecover(...)`                        | ✅     | Maps to `CryptoLib.verifyWithECDsa`                                         |
| `selfdestruct(addr)`                    | ✅     | Auto-mapped to `ContractManagement.destroy()` with warning                  |
| `address.call(...)`                     | ✅     | Maps to `System.Contract.Call`                                              |
| `address.staticcall(...)`               | ✅     | Maps to `System.Contract.Call` (read-only flag)                             |
| `address.delegatecall(...)`             | 🚫     | Blocked: no delegate call on Neo                                            |
| `address.transfer(amount)`              | 🚫     | Blocked: use NEP-17 `transfer()`                                            |
| `address.send(amount)`                  | 🚫     | Blocked: use NEP-17 `transfer()`                                            |
| `address.balance`                       | 🚫     | Blocked: use `NativeCalls.neoBalanceOf()` / `NativeCalls.gasBalanceOf()`    |
| `address.code`                          | 🚫     | Blocked: use `ContractManagement.getContract()`                             |
| `address.codehash`                      | ✅     | Auto-mapped to contract script hash with warning; non-contract → bytes32(0) |
| Ether units (`wei`, `gwei`, `ether`)    | ⚠️     | Parsed; warning that Neo uses GAS token (10^8 decimals)                     |
| Time units (`seconds`, `minutes`, etc.) | ✅     | Compile-time constants (normalized to seconds)                              |
| `this` keyword                          | ✅     | Maps to `Runtime.GetExecutingScriptHash()`; correct Neo equivalent          |
| `type(X).creationCode`                  | 🚫     | Blocked: no bytecode access on Neo                                          |
| `type(X).runtimeCode`                   | 🚫     | Blocked: no bytecode access on Neo                                          |

---

## I. ERC to NEP Protocol Mapping

| ERC Standard                 | NEP Equivalent                | Status | Notes                                                                         |
| ---------------------------- | ----------------------------- | ------ | ----------------------------------------------------------------------------- |
| ERC-20 (Fungible Token)      | NEP-17                        | ✅     | Auto-detected; `transfer(to,amount)` warns to use 4-param NEP-17 form         |
| ERC-721 (NFT)                | NEP-11                        | ✅     | Auto-detected; `transferFrom` warns to use NEP-11 `transfer(to,tokenId,data)` |
| ERC-20 `approve`/`allowance` | N/A                           | ⚠️     | Warning: not part of NEP-17 spec; Neo uses `Runtime.checkWitness()`           |
| ERC-165 `supportsInterface`  | Manifest `supportedstandards` | ⚠️     | Warning: unnecessary on Neo; manifest-based discovery                         |
| ERC-4626 (Tokenized Vault)   | NEP-17                        | ⚠️     | Vault logic compiles; ERC-20 interactions must use NEP-17 equivalents         |
| `receive()` / `fallback()`   | `onNEP17Payment()`            | ⚠️     | Diagnostic suggests callback pattern                                          |

---

## Summary

| Category            | ✅      | ⚠️     | ❌    | 🚫    |
| ------------------- | ------- | ------ | ----- | ----- |
| A. Types            | 16      | 2      | 1     | 0     |
| B. Expressions      | 18      | 3      | 0     | 0     |
| C. Statements       | 15      | 1      | 0     | 1     |
| D. Functions        | 9       | 4      | 0     | 0     |
| E. OOP Features     | 9       | 1      | 0     | 0     |
| F. Storage & Memory | 12      | 0      | 0     | 1     |
| G. Error Handling   | 9       | 1      | 0     | 0     |
| H. EVM-Specific     | 20      | 3      | 2     | 7     |
| I. ERC-NEP Mapping  | 2       | 4      | 0     | 0     |
| **Total**           | **110** | **19** | **3** | **9** |

**Total features audited: 141**

- ✅ Fully supported: 110 (78%)
- ⚠️ Partial support: 19 (13%)
- ❌ Not supported: 3 (2%)
- 🚫 Intentionally blocked: 9 (6%)
