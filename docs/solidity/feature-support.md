# Solidity Feature Support

The `neo-solidity` compiler parses Solidity 0.8.x source code and lowers it to NeoVM bytecode targeting Neo N3. "Feature support" describes how each Solidity language construct maps to NeoVM semantics — whether it compiles unchanged, compiles with behavioral differences, is rejected outright, or is intentionally blocked because no safe Neo equivalent exists.

This page is the human-readable companion to the canonical machine-audited matrix at [`docs/SOLIDITY_SUPPORT_MATRIX.md`](https://github.com/r3e-network/neo-solidity/blob/main/docs/SOLIDITY_SUPPORT_MATRIX.md).

For protocol-level migration status on well-known production contracts, start with [Original Famous Contracts (Per Contract)](/solidity/original-contracts/) and use [Famous Contracts Audit](/solidity/famous-contracts-neo-audit) for the aggregate report.

## Summary

| Metric                 | Count | Percentage |
| ---------------------- | ----: | ---------: |
| Total audited features |   142 |       100% |
| Fully supported        |   114 |        80% |
| Partial support        |    23 |        16% |
| Not supported          |     1 |         1% |
| Intentionally blocked  |     4 |         3% |

Status icons used throughout this page:

- ✅ Fully supported — compiles and behaves as expected on NeoVM.
- ⚠️ Partial support — compiles, but with behavioral differences or limitations documented below.
- ❌ Not supported — the compiler does not implement this feature.
- 🚫 Intentionally blocked — the compiler emits a diagnostic error because no safe Neo equivalent exists.

---

## A. Types

| Feature                   | Status | Notes                                                                                                                         |
| ------------------------- | :----: | ----------------------------------------------------------------------------------------------------------------------------- |
| `bool`                    |   ✅   | Maps to NeoVM Boolean.                                                                                                        |
| `int8` .. `int256`        |   ✅   | All widths parsed. NeoVM uses arbitrary-precision BigInteger internally.                                                      |
| `uint8` .. `uint256`      |   ✅   | All widths parsed. NeoVM uses arbitrary-precision BigInteger internally.                                                      |
| `address`                 |   ✅   | Maps to Neo UInt160 (Hash160, 20 bytes).                                                                                      |
| `address payable`         |   ⚠️   | Parsed and canonicalized to `address`. `.transfer()` / `.send()` auto-map to GAS transfer semantics (not EVM-attached value). |
| `bytes1` .. `bytes32`     |   ✅   | Fixed-length byte arrays via `NeoType::ByteArray { fixed_len }`.                                                              |
| `bytes` (dynamic)         |   ✅   | Dynamic byte array.                                                                                                           |
| `string`                  |   ✅   | UTF-8 string type.                                                                                                            |
| `enum`                    |   ✅   | Backed by `uint8`. Converted via `convert_enum`.                                                                              |
| `struct`                  |   ✅   | Full struct support with nested fields. Serialized via `StdLib.serialize`/`StdLib.deserialize` for storage.                   |
| `mapping(K => V)`         |   ✅   | Storage mappings with Neo StorageMap. Key type validation enforced.                                                           |
| `T[]` (dynamic array)     |   ✅   | `new T[](n)` allocation supported via `NEWARRAY`.                                                                             |
| `T[N]` (fixed array)      |   ⚠️   | Parsed. `new T[N]` supported when `N` is a compile-time constant.                                                             |
| `fixed` / `ufixed`        |   ❌   | Not supported. Also unsupported in mainline Solidity compilers.                                                               |
| User-defined value types  |   ✅   | `type X is Y` creates transparent aliases. `wrap`/`unwrap` compile to no-ops.                                                 |
| `bytes.concat(...)`       |   ✅   | Chains NeoVM `CAT` opcodes. Zero args produce an empty byte array.                                                            |
| `string.concat(...)`      |   ✅   | Same implementation as `bytes.concat` via `CAT` opcode chain.                                                                 |
| Contract types (`IERC20`) |   ✅   | Resolved to Neo UInt160 address. Interface types tracked.                                                                     |
| Tuple types               |   ✅   | Represented as NeoVM arrays internally.                                                                                       |

### Partial type details

**`address payable`** — The type is accepted and treated identically to `address`. On Neo, `.transfer(amount)` / `.send(amount)` compile via GAS NEP-17 transfer lowering (`transfer(from,to,amount,data)`), so behavior is close but not identical to EVM attached-value calls.

**`T[N]` (fixed array)** — Fixed-size arrays compile when the size `N` is a compile-time constant. Runtime-computed sizes require dynamic arrays (`T[]`).

```solidity
// ✅ Compiles — size is a compile-time constant
uint256[10] memory arr;

// ✅ Compiles — dynamic array with runtime size
uint256[] memory arr = new uint256[](n);
```

---

## B. Expressions

| Feature                                       | Status | Notes                                                                                                    |
| --------------------------------------------- | :----: | -------------------------------------------------------------------------------------------------------- | --- | ------------------ |
| Arithmetic (`+`, `-`, `*`, `/`, `%`)          |   ✅   | Binary ops via `try_lower_expression_binary_ops`.                                                        |
| Comparison (`==`, `!=`, `<`, `>`, `<=`, `>=`) |   ✅   | Via `try_lower_expression_comparisons`.                                                                  |
| Logical (`&&`, `\|\|`, `!`)                   |   ✅   | Short-circuit evaluation.                                                                                |
| Bitwise (`&`, `\|`, `^`, `~`, `<<`, `>>`)     |   ✅   | Full bitwise support.                                                                                    |
| Unary (`++`, `--`, `-`, `!`)                  |   ✅   | Pre/post increment/decrement.                                                                            |
| Ternary (`? :`)                               |   ✅   | `ConditionalOperator` lowered with labels.                                                               |
| Assignment (`=`, `+=`, `-=`, etc.)            |   ✅   | Compound assignments in `assignments/compound.rs`.                                                       |
| `delete`                                      |   ✅   | State vars, mapping entries, locals, array elements, struct fields.                                      |
| Tuple expressions `(a, b, c)`                 |   ✅   | Lowered to NeoVM arrays.                                                                                 |
| Tuple destructuring `(a, b) = f()`            |   ⚠️   | Supported. Some complex nested target forms may require intermediate locals.                             |
| Type casting                                  |   ✅   | Explicit casts between compatible types.                                                                 |
| `type(X).min` / `type(X).max`                 |   ✅   | Supported for integer types.                                                                             |
| `type(T).name`                                |   ✅   | Compile-time string constant.                                                                            |
| `type(I).interfaceId`                         |   ✅   | Computed from selector XOR of interface methods.                                                         |
| `abi.encode(...)`                             |   ⚠️   | Supported in context of `address.call`/`staticcall`. Standalone use is limited.                          |
| `abi.encodePacked(...)`                       |   ⚠️   | Same as `abi.encode` — used for Neo contract call encoding.                                              |
| `abi.encodeWithSignature(...)`                |   ⚠️   | Low-level call payloads rewrite to Neo contract calls; standalone use approximates calldata as `selector |     | abi.encode(args)`. |
| `abi.encodeWithSelector(...)`                 |   ⚠️   | Low-level call payloads rewrite to Neo contract calls; standalone use approximates calldata as `selector |     | abi.encode(args)`. |
| `abi.encodeCall(...)`                         |   ✅   | Maps to `StdLib.serialize`.                                                                              |
| `abi.decode(...)`                             |   ✅   | Maps to `StdLib.deserialize`. Type tuple parsed from second argument.                                    |
| Named function call args `f({x: 1})`          |   ✅   | Named args reordered to positional order at IR level.                                                    |

### Partial expression details

**Tuple destructuring** — Standard patterns like `(uint a, uint b) = getValues()` work. Deeply nested destructuring targets (e.g., destructuring into struct members or nested tuples in a single statement) may require the compiler to introduce intermediate locals.

**`abi.encode` / `abi.encodePacked` / `abi.encodeWithSignature` / `abi.encodeWithSelector`** — These functions are primarily designed for cross-contract call encoding on Neo, not for producing Ethereum-ABI-compatible byte sequences. When used as arguments to `address.call()` or `address.staticcall()`, the compiler rewrites them into Neo contract-call lowering. Standalone use now returns a Neo-side approximation: `abi.encode*` emits `StdLib.serialize(...)`, while `encodeWithSignature` / `encodeWithSelector` emit `selector || abi.encode(args)`. These byte sequences are useful on Neo, but not guaranteed to be EVM-identical raw calldata.

```solidity
// ✅ Works — encoding for cross-contract call
address(target).call(abi.encodeWithSignature("transfer(address,uint256)", to, amount));

// ⚠️ Limited — standalone encoding may differ from EVM ABI
bytes memory encoded = abi.encode(a, b, c);
bytes memory payload = abi.encodeWithSignature("transfer(address,uint256)", to, amount);
```

---

## C. Statements

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
| `catch Panic(uint256)`    |   ⚠️   | Lowered with runtime integer-type guard. Values are NeoVM exception payloads, not canonical EVM panic codes. |
| `catch (bytes)`           |   ✅   | Low-level catch with raw bytes.                                                                              |

### Partial statement details

**`catch Panic(uint256)`** — NeoVM exceptions do not carry EVM-style panic codes (0x01 for assert, 0x11 for overflow, etc.). The catch clause binds the NeoVM exception payload as an integer, but the numeric values will not match Ethereum panic code semantics. Use `catch (bytes memory reason)` for maximum portability.

**`unchecked { ... }`** — Since NeoVM uses arbitrary-precision BigInteger, integer overflow cannot occur. The `unchecked` block is accepted for source compatibility but has no behavioral effect — all arithmetic is inherently unchecked on NeoVM.

```solidity
// Compiles identically with or without unchecked on NeoVM
unchecked {
    uint256 result = a + b; // No overflow possible — BigInteger
}
```

---

## D. Functions

| Feature                          | Status | Notes                                                                                                                   |
| -------------------------------- | :----: | ----------------------------------------------------------------------------------------------------------------------- |
| Regular functions                |   ✅   | `public`, `external`, `internal`, `private` visibility.                                                                 |
| Constructor                      |   ✅   | Single constructor. Multiple constructors rejected.                                                                     |
| `view` / `pure`                  |   ✅   | State mutability tracked and enforced at IR level.                                                                      |
| `payable`                        |   ⚠️   | Parsed. `payable` on non-receive functions warns — Neo has no native gas payment in function calls.                     |
| `returns (T)`                    |   ✅   | Single return type.                                                                                                     |
| `returns (T1, T2, ...)`          |   ✅   | Multi-return via NeoVM arrays.                                                                                          |
| Function overloading             |   ⚠️   | Supported with Neo overload mangling. One canonical ABI name is kept; other overloads use generated `neo_name` entries. |
| `modifier`                       |   ✅   | Full modifier expansion with `_` placeholder substitution.                                                              |
| `receive()`                      |   ⚠️   | Parsed. Diagnostic suggests using `onNEP17Payment()` callback instead.                                                  |
| `fallback()`                     |   ⚠️   | Parsed. Diagnostic suggests using `onNEP17Payment()` callback instead.                                                  |
| `virtual` / `override`           |   ✅   | Inheritance flattening resolves overrides. Multi-level chains supported.                                                |
| Function selectors (`.selector`) |   ✅   | Computed from canonical parameter types.                                                                                |
| NatSpec comments                 |   ✅   | `@notice`, `@dev`, `@param`, `@return` preserved in metadata.                                                           |

### Partial function details

**`payable`** — Neo does not attach native value to function calls the way EVM does with `msg.value`. The `payable` modifier is accepted for source compatibility, but a warning is emitted on non-receive functions. Token payments on Neo are handled through NEP-17/NEP-11 callbacks.

**Function overloading** — The compiler supports overloaded functions by assigning Neo-visible mangled names (`neo_name`) to overloaded variants. One canonical ABI name is preserved, and other overloads are exported under generated names like `foo(uint256)` or `foo(address)`. The limitation is not compilation, but downstream invocation: Neo callers must use the generated Neo method names when targeting a non-primary overload.

```solidity
// ⚠️ Overload collision in Neo ABI — both produce "transfer" in manifest
function transfer(address to, uint256 amount) public { ... }
function transfer(address to, uint256 amount, bytes calldata data) public { ... }

// ✅ Use distinct names instead
function transfer(address to, uint256 amount) public { ... }
function transferWithData(address to, uint256 amount, bytes calldata data) public { ... }
```

**`receive()` / `fallback()`** — These EVM constructs handle incoming Ether. On Neo, token receipts are handled by explicit callbacks. The compiler emits a diagnostic suggesting you implement `onNEP17Payment(address from, uint256 amount, bytes memory data)` instead.

---

## E. OOP Features

| Feature              | Status | Notes                                                                                                                  |
| -------------------- | :----: | ---------------------------------------------------------------------------------------------------------------------- |
| Single inheritance   |   ✅   | C3 linearization with `flatten_contract_inheritance`.                                                                  |
| Multiple inheritance |   ✅   | Diamond inheritance detected. Constructor arg conflicts reported.                                                      |
| `interface`          |   ✅   | Interface types tracked. Methods validated.                                                                            |
| `abstract contract`  |   ✅   | Unimplemented functions detected. Non-abstract contracts get actionable errors.                                        |
| `library`            |   ⚠️   | Builtin devpack libraries are compiler intrinsics. User-defined libraries are merged/inlined into consuming contracts. |
| `using X for Y`      |   ✅   | Library member-call syntax fully supported. `using X for *` and `using {f,g} for T` included.                          |
| `super` keyword      |   ✅   | Supported via inheritance flattening with `__super_` method preservation.                                              |
| `is` (inheritance)   |   ✅   | Inheritance specifiers fully processed.                                                                                |
| Constructor chaining |   ✅   | Base constructor arguments resolved from inheritance specifiers.                                                       |
| Event inheritance    |   ✅   | Interface events collected recursively via `collect_interface_events_recursive`.                                       |

### Partial OOP details

**`library`** — The devpack ships built-in libraries (`Runtime`, `Storage`, `Syscalls`, etc.) that are compiler intrinsics — they lower directly to syscalls and native contract calls. User-defined libraries are merged into the consuming contract. `internal` calls work directly, and `public` / `external` library functions are accepted but normalized to internal helpers with warnings. Libraries still cannot maintain mutable state or behave like separately deployed/linkable EVM libraries.

```solidity
// ✅ Works — using devpack intrinsic library
import "devpack/libraries/Runtime.sol";
require(Runtime.checkWitness(sender), "unauthorized");

// ✅ Works — user-defined library with internal functions
library MathLib {
    function add(uint256 a, uint256 b) internal pure returns (uint256) {
        return a + b;
    }
}

using MathLib for uint256;
uint256 result = x.add(y);
```

---

## F. Storage and Memory

| Feature                          | Status | Notes                                                                  |
| -------------------------------- | :----: | ---------------------------------------------------------------------- |
| State variables                  |   ✅   | Mapped to Neo Storage with prefix-based keys.                          |
| `constant`                       |   ✅   | Compile-time constants inlined.                                        |
| `immutable`                      |   ✅   | Tracked via `is_immutable` flag. Modification blocked at compile time. |
| `memory` keyword                 |   ✅   | Parsed. NeoVM is stack-based so memory is implicit.                    |
| `storage` keyword                |   ✅   | Storage references for mappings and state variables.                   |
| `calldata` keyword               |   ✅   | Parsed. Treated as `memory` — NeoVM has no calldata region.            |
| Nested mappings                  |   ✅   | `mapping(K1 => mapping(K2 => V))` with composite storage keys.         |
| Struct in storage                |   ✅   | Serialized/deserialized via `StdLib.serialize`/`StdLib.deserialize`.   |
| Array `.push()` / `.pop()`       |   ✅   | Storage array operations supported.                                    |
| Array `.length`                  |   ✅   | Both memory and storage arrays.                                        |
| `new bytes(n)` / `new string(n)` |   ✅   | Buffer allocation via `NEWBUFFER`.                                     |
| `new T[](n)`                     |   ✅   | Dynamic array allocation via `NEWARRAY`.                               |
| `new Contract(...)`              |   🚫   | Blocked: "use ContractManagement for contract deployment".             |

### Storage key derivation

State variables are stored in Neo Storage using deterministic key derivation. For simple state variables, the key is derived from the variable name. For mappings, the key is computed as:

```
SHA256(key_bytes || slot_hash)
```

Where `slot_hash` is `SHA256(variable_name)`. Nested mappings iterate this process for each key level. See [Types](/language-description/types) for the full storage lowering specification.

---

## G. Error Handling

| Feature                                | Status | Notes                                                                                                               |
| -------------------------------------- | :----: | ------------------------------------------------------------------------------------------------------------------- |
| `require(condition)`                   |   ✅   | Maps to NeoVM `ASSERT`.                                                                                             |
| `require(condition, "msg")`            |   ✅   | `ASSERT` with message.                                                                                              |
| `require(condition, CustomError(...))` |   ✅   | Error name and arg count preserved in NeoVM `THROW` message.                                                        |
| `assert(condition)`                    |   ✅   | Maps to NeoVM `ASSERT`.                                                                                             |
| `revert()`                             |   ✅   | Maps to NeoVM `ABORT`.                                                                                              |
| `revert("message")`                    |   ✅   | `ABORT` with message.                                                                                               |
| `revert CustomError(...)`              |   ✅   | Named revert with arguments.                                                                                        |
| Custom error definitions               |   ✅   | `error X(...)` parsed and used in revert statements.                                                                |
| `try` / `catch`                        |   ✅   | NeoVM `TRY`/`ENDTRY` structured exception handling.                                                                 |
| `try` with return binding              |   ✅   | `try f() returns (uint r) { ... }` supported.                                                                       |
| Multiple catch clauses                 |   ⚠️   | Lowered with runtime stack-item type guards (`ISTYPE`). Selector-level `Error`/`Panic` distinction remains limited. |

### Partial error handling details

**Multiple catch clauses** — NeoVM exceptions are untyped. When multiple catch clauses are present (`catch Error(string)`, `catch Panic(uint256)`, `catch (bytes)`), the compiler inserts `ISTYPE` guards to route exceptions by stack item type. This provides reasonable dispatch but does not replicate EVM's distinct error/panic channels exactly.

```solidity
// ✅ Recommended — single catch clause
try target.someFunction() returns (uint256 result) {
    // success
} catch (bytes memory reason) {
    // handle any failure
}

// ⚠️ Works but with caveats — multiple catch clauses
try target.someFunction() returns (uint256 result) {
    // success
} catch Error(string memory reason) {
    // string exceptions routed here
} catch (bytes memory lowLevelData) {
    // everything else
}
```

---

## H. EVM-Specific Features

These features reference EVM runtime concepts. The compiler maps them to Neo equivalents where possible, blocks them where no safe equivalent exists, and emits warnings for approximate mappings.

| Feature                                 | Status | Neo Mapping                                                                                                          |
| --------------------------------------- | :----: | -------------------------------------------------------------------------------------------------------------------- | --- | ------------------------------------------------- |
| `msg.sender`                            |   ✅   | `Runtime.GetCallingScriptHash()`                                                                                     |
| `msg.value`                             |   ⚠️   | Only mapped inside `onNEP17Payment` callback.                                                                        |
| `msg.data`                              |   ⚠️   | Approximated as `selector                                                                                            |     | abi.encode(current args)` outside onNEP17Payment. |
| `msg.sig`                               |   ⚠️   | Approximated as the current function selector with warning; internal-call propagation still differs from EVM.        |
| `block.timestamp`                       |   ✅   | `Runtime.GetTime()` (normalized to seconds).                                                                         |
| `block.number`                          |   ✅   | `Ledger.CurrentIndex()`.                                                                                             |
| `block.chainid`                         |   ✅   | Neo network magic number.                                                                                            |
| `block.coinbase`                        |   ✅   | Auto-mapped to `address(0)` with warning (dBFT has no miner).                                                        |
| `block.difficulty` / `block.prevrandao` |   ✅   | Auto-mapped to `Runtime.getRandom()` with warning.                                                                   |
| `block.gaslimit`                        |   ✅   | Auto-mapped to `Policy.getExecFeeFactor()` with warning.                                                             |
| `block.basefee`                         |   ✅   | Auto-mapped to `Policy.getFeePerByte()` with warning.                                                                |
| `tx.origin`                             |   ⚠️   | Parsed. Warning about authorization risks. Maps to first signer script hash.                                         |
| `tx.gasprice`                           |   ✅   | Auto-mapped to `Policy.getFeePerByte()` with warning.                                                                |
| `gasleft()`                             |   ✅   | `System.Runtime.GasLeft` syscall.                                                                                    |
| `blockhash(n)`                          |   ✅   | Auto-mapped to `Ledger.getBlockHash()` with warning.                                                                 |
| `block.sha3`                            |   ⚠️   | Auto-mapped to `Ledger.currentHash`. Deprecated in Solidity 0.8+.                                                    |
| `keccak256(...)`                        |   ✅   | `CryptoLib.keccak256`.                                                                                               |
| `sha256(...)`                           |   ✅   | `CryptoLib.sha256`.                                                                                                  |
| `ecrecover(...)`                        |   ✅   | `CryptoLib.verifyWithECDsa`.                                                                                         |
| `selfdestruct(addr)`                    |   ✅   | Auto-mapped to `ContractManagement.destroy()` with warning.                                                          |
| `address.call(...)`                     |   ✅   | `System.Contract.Call`.                                                                                              |
| `address.staticcall(...)`               |   ✅   | `System.Contract.Call` with read-only flag.                                                                          |
| `address.delegatecall(...)`             |   ⚠️   | Emits warning; compiled as `System.Contract.Call` with different storage semantics.                                  |
| `address.transfer(amount)`              |   ✅   | Auto-mapped to `GAS.transfer(from,to,amount,data)`; aborts on transfer failure.                                      |
| `address.send(amount)`                  |   ✅   | Auto-mapped to `GAS.transfer(from,to,amount,data)`; returns bool.                                                    |
| `address.balance`                       |   ✅   | Auto-mapped to `GAS.balanceOf(address)`.                                                                             |
| `address.code`                          |   ⚠️   | Returns Neo contract script bytes via `ContractManagement.getContract()`; non-contract addresses return empty bytes. |
| `address.codehash`                      |   ✅   | Auto-mapped to contract script hash with warning. Non-contract returns `bytes32(0)`.                                 |
| Ether units (`wei`, `gwei`, `ether`)    |   ⚠️   | Parsed. Warning that Neo uses GAS token (10^8 decimals).                                                             |
| Time units (`seconds`, `minutes`, etc.) |   ✅   | Compile-time constants normalized to seconds.                                                                        |
| `this` keyword                          |   ✅   | `Runtime.GetExecutingScriptHash()`.                                                                                  |
| `type(X).creationCode`                  |   🚫   | Blocked: no bytecode access on Neo.                                                                                  |
| `type(X).runtimeCode`                   |   🚫   | Blocked: no bytecode access on Neo.                                                                                  |

### Partial EVM feature details

**`msg.value`** — Neo does not attach native value to contract calls. The `msg.value` expression is only meaningful inside `onNEP17Payment()` callbacks, where it maps to the `amount` parameter. Outside that context, the compiler emits a warning (W111) and returns `0` at runtime.

```solidity
// ✅ Works — msg.value inside payment callback
function onNEP17Payment(address from, uint256 amount, bytes memory data) external {
    require(msg.value >= minDeposit, "insufficient deposit");
    // msg.value maps to the `amount` parameter
}

// ⚠️ Warning — msg.value outside payment context returns 0
function deposit() public payable {
    balances[msg.sender] += msg.value; // msg.value is always 0 on Neo
}
```

**`tx.origin`** — Maps to the first signer's script hash in the Neo transaction. The compiler emits a warning because `tx.origin`-based authorization is considered an anti-pattern on both EVM and Neo. Use `msg.sender` (which maps to `Runtime.GetCallingScriptHash()`) or `Runtime.checkWitness()` instead.

**Ether units** — The literal multipliers (`1 ether = 10^18`, `1 gwei = 10^9`, etc.) are parsed for source compatibility, but a warning is emitted because Neo GAS uses 10^8 decimals, not 10^18. Adjust your constants accordingly.

**`address.code`** — On Neo this now lowers to the contract script bytes fetched through `ContractManagement.getContract()`. This is closer to EVM runtime bytecode access than the old empty-byte placeholder, but it is still Neo contract script rather than EVM bytecode. Non-contract addresses return empty bytes. `address.code.length` remains the fast contract-existence approximation (0 for non-contract, 1 for contract).

**`address.delegatecall(...)`** — On NeoVM, each contract has isolated storage. `delegatecall` on EVM executes callee code in the caller's storage context, which has no Neo equivalent. The compiler emits a warning and compiles the call as a regular `System.Contract.Call`, which uses the callee's own storage context instead of the caller's. This is a semantic difference that can cause security issues in contracts relying on delegatecall for upgrade patterns or library calls. Use `address.call()` or `address.staticcall()` instead, or redesign to use explicit library calls or `ContractManagement.update()`.

```solidity
// ⚠️ Works but with different semantics than EVM
(bool success, bytes memory data) = target.delegatecall(abi.encodeWithSignature("foo()"));
// Compiles as System.Contract.Call with warning; storage context differs from EVM
```

### Auto-mapping warnings

Several EVM globals are auto-mapped to approximate Neo equivalents. The compiler emits warnings for each to ensure developers understand the semantic differences:

| EVM Global                              | Auto-Mapped To                 | Why It Warns                                              |
| --------------------------------------- | ------------------------------ | --------------------------------------------------------- |
| `block.coinbase`                        | `address(0)`                   | dBFT consensus has no block miner.                        |
| `block.difficulty` / `block.prevrandao` | `Runtime.getRandom()`          | Different randomness model.                               |
| `block.gaslimit`                        | `Policy.getExecFeeFactor()`    | Different gas accounting.                                 |
| `block.basefee`                         | `Policy.getFeePerByte()`       | Different fee model.                                      |
| `tx.gasprice`                           | `Policy.getFeePerByte()`       | Different fee model.                                      |
| `blockhash(n)`                          | `Ledger.getBlockHash()`        | Semantic match but different chain.                       |
| `block.sha3`                            | `Ledger.currentHash`           | Deprecated in Solidity 0.8+; Neo uses current block hash. |
| `selfdestruct(addr)`                    | `ContractManagement.destroy()` | No refund mechanism. Permanent.                           |
| `address.codehash`                      | Contract script hash           | Non-contract addresses return `bytes32(0)`.               |

---

## I. ERC to NEP Protocol Mapping

| Ethereum Standard            | Neo Standard                  | Status | Notes                                                                          |
| ---------------------------- | ----------------------------- | :----: | ------------------------------------------------------------------------------ |
| ERC-20 (Fungible Token)      | NEP-17                        |   ✅   | Auto-detected. `transfer(to,amount)` warns to use 4-param NEP-17 form.         |
| ERC-721 (NFT)                | NEP-11                        |   ✅   | Auto-detected. `transferFrom` warns to use NEP-11 `transfer(to,tokenId,data)`. |
| ERC-20 `approve`/`allowance` | N/A                           |   ⚠️   | Warning: not part of NEP-17 spec. Neo uses `Runtime.checkWitness()`.           |
| ERC-165 `supportsInterface`  | Manifest `supportedstandards` |   ⚠️   | Warning: unnecessary on Neo. Manifest-based discovery.                         |
| ERC-4626 (Tokenized Vault)   | NEP-17                        |   ⚠️   | Vault logic compiles. ERC-20 interactions must use NEP-17 equivalents.         |
| ERC-2981 (Royalty)           | NEP-24                        |   ✅   | Auto-detected. Multiple royalty recipients supported.                          |
| `receive()` / `fallback()`   | `onNEP17Payment()`            |   ⚠️   | Diagnostic suggests callback pattern.                                          |

For detailed standard migration guides, see the [Standards Mapping](/additional-material/neo-standards).

---

## Category Summary

| Category            |      ✅ |     ⚠️ |    ❌ |    🚫 |
| ------------------- | ------: | -----: | ----: | ----: |
| A. Types            |      16 |      2 |     1 |     0 |
| B. Expressions      |      18 |      3 |     0 |     0 |
| C. Statements       |      15 |      2 |     0 |     0 |
| D. Functions        |       9 |      4 |     0 |     0 |
| E. OOP Features     |       9 |      1 |     0 |     0 |
| F. Storage & Memory |      12 |      0 |     0 |     1 |
| G. Error Handling   |       9 |      1 |     0 |     0 |
| H. EVM-Specific     |      23 |      6 |     0 |     3 |
| I. ERC-NEP Mapping  |       3 |      4 |     0 |     0 |
| **Total**           | **114** | **23** | **1** | **4** |

---

## Building with Safe Defaults

Always compile with strict flags in production to avoid unintended wildcard permissions:

```bash
neo-solc MyContract.sol \
  --deny-wildcard-contracts \
  --deny-wildcard-methods \
  -o build/MyContract
```

## Further Reading

- [Language Description](/language-description/types) — detailed behavioral semantics on NeoVM
- [Runtime Spec](/internals/runtime-specification) — embedded runtime specification
- [Parity and Limitations](/internals/parity-and-limitations) — known fidelity gaps
- [Standards Mapping](/additional-material/neo-standards) — ERC to NEP migration guides
