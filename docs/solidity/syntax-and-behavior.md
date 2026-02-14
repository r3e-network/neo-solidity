# Syntax and Behavior

This page documents the behavioral semantics of Solidity constructs when compiled to NeoVM by the `neo-solidity` compiler. Where EVM and NeoVM diverge, the differences are explained with code examples showing the Solidity source and the resulting Neo behavior.

For the feature-level support matrix, see [Feature Support](/solidity/feature-support). For the complete EVM-to-NeoVM mapping table, see [EVM to NeoVM Mapping](/mapping/evm-to-neovm).

---

## Type System on Neo

### Integer Types

Solidity defines fixed-width integer types from `int8`/`uint8` through `int256`/`uint256`. NeoVM represents all integers as arbitrary-precision BigInteger values.

| Solidity Type        | NeoVM Representation | Behavior                      |
| -------------------- | -------------------- | ----------------------------- |
| `int8` .. `int256`   | BigInteger           | Signed, arbitrary precision   |
| `uint8` .. `uint256` | BigInteger           | Unsigned, arbitrary precision |

The compiler tracks the declared width for type-checking and ABI generation, but at runtime all integers are BigInteger. This means:

- Overflow is impossible. `uint8(255) + 1` produces `256`, not `0`.
- The `unchecked { }` block has no behavioral effect — it exists for source compatibility.
- Solidity 0.8.x checked arithmetic (overflow/underflow revert) does not apply on NeoVM.

```solidity
// On EVM: reverts with overflow in Solidity 0.8.x
// On NeoVM: produces 256 (BigInteger, no overflow)
uint8 x = 255;
uint8 y = x + 1; // y == 256 on NeoVM

// unchecked has no effect on NeoVM — all arithmetic is inherently unchecked
unchecked {
    uint8 z = x + 1; // z == 256, same as above
}
```

::: warning
If your contract logic depends on overflow/underflow behavior (wrapping or reverting), you must add explicit range checks when targeting NeoVM.
:::

### Address Type

The `address` type maps to Neo's UInt160 — a 20-byte script hash identifying an account or contract.

| Solidity          | NeoVM              | Neo ABI Type |
| ----------------- | ------------------ | ------------ |
| `address`         | UInt160 (20 bytes) | `Hash160`    |
| `address payable` | UInt160 (20 bytes) | `Hash160`    |

`address payable` is accepted and canonicalized to `address`. The `.transfer()` and `.send()` members are blocked because Neo does not support direct value transfer through address members — use NEP-17 `transfer()` instead.

```solidity
// Solidity address → Neo Hash160 (20-byte script hash)
address owner = msg.sender;
// msg.sender → Runtime.GetCallingScriptHash()

// ✅ Address comparison works as expected
require(owner == expectedOwner, "wrong owner");

// 🚫 Blocked — no direct value transfer on Neo
// owner.transfer(1 ether);

// ✅ Use NEP-17 transfer instead
// NativeCalls.gasTransfer(address(this), owner, amount, "");
```

### Bytes and BytesN

| Solidity              | NeoVM                    | Neo ABI Type                        |
| --------------------- | ------------------------ | ----------------------------------- |
| `bytes1` .. `bytes32` | ByteArray (fixed length) | `ByteArray` (`bytes32` → `Hash256`) |
| `bytes` (dynamic)     | ByteArray (dynamic)      | `ByteArray`                         |

Fixed-length byte arrays (`bytes1` through `bytes32`) are represented as fixed-length ByteArrays. The special case `bytes32` maps to Neo ABI type `Hash256`, which is commonly used for token IDs in NEP-11.

Dynamic `bytes` maps to a variable-length ByteArray. Concatenation uses the NeoVM `CAT` opcode.

```solidity
// bytes32 → Hash256 in Neo ABI (used for NFT token IDs)
bytes32 tokenId = keccak256(abi.encode(owner, tokenIndex));

// Dynamic bytes concatenation → CAT opcode
bytes memory a = hex"0102";
bytes memory b = hex"0304";
bytes memory c = bytes.concat(a, b); // → 0x01020304
```

### String Type

Strings are UTF-8 encoded ByteStrings on NeoVM. The Neo ABI type is `String`.

```solidity
// string → UTF-8 ByteString on NeoVM
string memory name = "MyToken";

// string.concat → CAT opcode chain
string memory full = string.concat("Hello, ", name);
```

### Bool Type

`bool` maps directly to NeoVM Boolean. No behavioral differences.

### Enum Type

Enums are backed by `uint8` and converted to integer constants at compile time.

```solidity
enum Status { Pending, Active, Closed }
// Status.Pending == 0, Status.Active == 1, Status.Closed == 2

Status s = Status.Active;
// Stored as integer 1 in NeoVM
```

### Struct Type

Structs are compound types. When stored in state, they are serialized via `StdLib.serialize()` and deserialized via `StdLib.deserialize()`. In memory, they are represented as NeoVM arrays with positional field access.

```solidity
struct Proposal {
    address creator;
    uint256 votes;
    bool executed;
}

// In storage: serialized as a single blob via StdLib.serialize
mapping(uint256 => Proposal) public proposals;

// In memory: NeoVM array [creator, votes, executed]
Proposal memory p = Proposal(msg.sender, 0, false);
```

### Mapping Type

Mappings compile to Neo StorageMap operations with deterministic key derivation. See the [Storage Model](#storage-model) section below for the full key encoding scheme.

```solidity
mapping(address => uint256) public balances;

// Read: balances[addr]
// → System.Storage.Get(context, SHA256(serialize(addr) || slot_hash))

// Write: balances[addr] = 100
// → System.Storage.Put(context, SHA256(serialize(addr) || slot_hash), 100)
```

### Array Types

| Pattern         | NeoVM                     | Notes                             |
| --------------- | ------------------------- | --------------------------------- |
| `T[]` (dynamic) | `NEWARRAY`                | Runtime-sized allocation          |
| `T[N]` (fixed)  | `NEWARRAY`                | `N` must be compile-time constant |
| `.push(x)`      | Storage array append      | Storage arrays only               |
| `.pop()`        | Storage array remove last | Storage arrays only               |
| `.length`       | Array size                | Both memory and storage           |

### Tuple Types

Tuples are represented as NeoVM arrays. Multi-return functions pack return values into an array, and tuple destructuring unpacks them.

```solidity
// Multi-return → NeoVM array
function getValues() public pure returns (uint256, bool) {
    return (42, true);
}

// Tuple destructuring → array element access
(uint256 val, bool flag) = getValues();
```

---

## Storage Model

### State Variable Layout

State variables are stored in Neo Storage using prefix-based keys. Each state variable gets a deterministic storage key derived from its name.

| Variable Type                          | Storage Key                 | Value Encoding            |
| -------------------------------------- | --------------------------- | ------------------------- |
| Simple types (`uint256`, `bool`, etc.) | `SHA256(variable_name)`     | Native NeoVM encoding     |
| `constant`                             | Not stored                  | Inlined at compile time   |
| `immutable`                            | Not stored                  | Inlined at compile time   |
| `mapping(K => V)`                      | Derived per-key (see below) | Native NeoVM encoding     |
| `struct`                               | `SHA256(variable_name)`     | `StdLib.serialize()` blob |
| `T[]` (storage array)                  | Prefix-based                | Length + element keys     |

### Mapping Key Derivation

Mapping storage keys use iterative hashing to produce deterministic, collision-free keys:

```
slot_hash = SHA256(variable_name)
final_key = SHA256(serialize(key) || slot_hash)
```

For nested mappings, the process iterates from outermost to innermost key:

```
// mapping(address => mapping(uint256 => bool)) approvals;
slot_hash   = SHA256("approvals")
level_1_key = SHA256(serialize(outer_key) || slot_hash)
level_2_key = SHA256(serialize(inner_key) || level_1_key)
// level_2_key is the final storage key
```

Key serialization rules:

| Key Type             | Serialization                                                                                                       |
| -------------------- | ------------------------------------------------------------------------------------------------------------------- |
| Integers             | Big-endian byte array, padded to declared bit width. Signed integers are sign-extended; unsigned are zero-extended. |
| Booleans             | Single byte: `0x00` or `0x01`.                                                                                      |
| `address` / `bytesN` | Raw bytes (no padding).                                                                                             |
| `bytes` / `string`   | UTF-8 bytes (no length prefix).                                                                                     |

```solidity
mapping(address => mapping(uint256 => uint256)) public allowances;

// allowances[alice][tokenId] = 100;
// Storage key derivation:
//   slot    = SHA256("allowances")
//   level1  = SHA256(serialize(alice) || slot)
//   level2  = SHA256(serialize(tokenId) || level1)
//   Storage.Put(context, level2, 100)
```

### Struct Storage Layout

Structs stored in state variables are serialized as a single blob using `StdLib.serialize()`. The entire struct is read and written atomically.

```solidity
struct Config {
    address admin;
    uint256 fee;
    bool paused;
}

Config public config;

// Reading config → StdLib.deserialize(Storage.Get(context, key))
// Writing config → Storage.Put(context, key, StdLib.serialize(config))
```

::: tip
Struct storage is atomic — reading one field requires deserializing the entire struct. For frequently accessed individual fields, consider using separate state variables instead.
:::

### Array Storage Layout

Storage arrays use a length key and per-element keys:

```
length_key = SHA256(variable_name)
element_key(i) = SHA256(serialize(i) || SHA256(variable_name))
```

The `.push()` operation increments the length and writes the new element. The `.pop()` operation decrements the length and deletes the last element key.

### Storage Optimization

NeoVM storage operations are syscall-based and carry gas costs. Optimization strategies:

1. Batch reads/writes where possible — each `Storage.Get`/`Storage.Put` is a separate syscall.
2. Use `constant` and `immutable` for values that do not change — they are inlined and never touch storage.
3. Prefer separate state variables over structs when only individual fields are accessed frequently.
4. Minimize nested mapping depth — each nesting level adds a SHA256 hash operation.

---

## Event Model

### Solidity Events to Neo Notifications

Solidity `emit` statements compile to `Runtime.Notify` syscalls. Event parameters become the notification state array.

| Solidity Concept      | Neo Equivalent                                     |
| --------------------- | -------------------------------------------------- |
| `event Transfer(...)` | Notification with event name                       |
| `emit Transfer(...)`  | `Runtime.Notify("Transfer", state_array)`          |
| `indexed` parameter   | Included in notification state (no separate topic) |

```solidity
event Transfer(address indexed from, address indexed to, uint256 amount);

function _transfer(address from, address to, uint256 amount) internal {
    // ...
    emit Transfer(from, to, amount);
    // Compiles to: Runtime.Notify("Transfer", [from, to, amount])
}
```

### Indexed Parameters

On EVM, `indexed` parameters are stored as separate log topics for efficient filtering. On Neo, all parameters (indexed or not) are included in the notification state array. The `indexed` keyword is accepted for source compatibility but does not create separate topics.

::: info
Neo event filtering is done by event name (the notification's first argument), not by indexed parameter values. Off-chain indexers can filter on parameter values after receiving the notification.
:::

### Event Inheritance

Events defined in interfaces and base contracts are collected recursively. A contract inheriting from multiple interfaces will include all inherited events in its notification surface.

```solidity
interface IERC20Events {
    event Transfer(address indexed from, address indexed to, uint256 value);
    event Approval(address indexed owner, address indexed spender, uint256 value);
}

contract MyToken is IERC20Events {
    // Both Transfer and Approval are available for emit
}
```

---

## Error Handling

### require()

`require(condition)` and `require(condition, "message")` map to NeoVM `ASSERT` and `ASSERTMSG` opcodes. When the condition is false, execution aborts with the provided message.

```solidity
require(balance >= amount, "insufficient balance");
// NeoVM: ASSERTMSG "insufficient balance"

require(balance >= amount, InsufficientBalance(balance, amount));
// NeoVM: THROW with error name and args preserved
```

### assert()

`assert(condition)` maps to NeoVM `ASSERT`. On EVM, `assert` failures consume all remaining gas and produce a Panic error. On NeoVM, `assert` simply aborts execution — there is no gas penalty distinction between `require` and `assert`.

### revert()

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

### try/catch

`try`/`catch` maps to NeoVM `TRY`/`ENDTRY` structured exception handling. The try block wraps the external call, and catch clauses handle exceptions.

```solidity
try target.riskyCall() returns (uint256 result) {
    // Success path
    processResult(result);
} catch Error(string memory reason) {
    // String exception — routed by ISTYPE guard
    emit CallFailed(reason);
} catch (bytes memory lowLevelData) {
    // Catch-all for other exceptions
    emit CallFailedRaw(lowLevelData);
}
```

::: warning
NeoVM exceptions are untyped. Multiple catch clauses use `ISTYPE` runtime guards to route by stack item type. The EVM distinction between `Error(string)` and `Panic(uint256)` is approximated but not exact. For maximum reliability, use a single `catch (bytes memory)` clause.
:::

---

## Constructor Behavior

### Solidity Constructor to Neo Deploy

The Solidity constructor maps to the `_deploy(data, update)` entry point in the Neo contract. This method is called automatically by `ContractManagement` when the contract is deployed or updated.

| Solidity               | Neo                                          |
| ---------------------- | -------------------------------------------- |
| `constructor(...)`     | `_deploy(object data, bool update)`          |
| Constructor parameters | Passed via deployment `data`                 |
| Constructor body       | Executed in `_deploy` when `update == false` |

```solidity
// Solidity source
contract MyToken {
    string public name;
    address public owner;

    constructor(string memory _name) {
        name = _name;
        owner = msg.sender;
    }
}

// Compiled Neo behavior:
// _deploy(data, update) is generated
// When update == false: constructor body executes
// When update == true: constructor body is skipped (contract update)
```

### Constructor Constraints

- Only a single constructor is allowed. Multiple constructor definitions are rejected.
- Constructor parameters are passed through the deployment `data` parameter and deserialized at runtime.
- The constructor runs once during initial deployment. During contract updates (`ContractManagement.update()`), the `_deploy` method is called with `update == true`, and the constructor body is skipped.

### Initialization Patterns

```solidity
// Pattern 1: Simple initialization in constructor
constructor() {
    owner = msg.sender;
    paused = false;
}

// Pattern 2: Parameterized initialization
constructor(string memory _name, uint256 _supply) {
    name = _name;
    totalSupply = _supply;
    balances[msg.sender] = _supply;
}
```

---

## Inheritance

### C3 Linearization

Multiple inheritance is resolved using C3 linearization, the same algorithm used by Solidity on EVM. The compiler flattens the inheritance hierarchy into a single contract with all inherited state variables, functions, and events.

```solidity
contract A {
    function foo() public virtual returns (string memory) { return "A"; }
}

contract B is A {
    function foo() public virtual override returns (string memory) { return "B"; }
}

contract C is A {
    function foo() public virtual override returns (string memory) { return "C"; }
}

// C3 linearization: D → B → C → A
contract D is B, C {
    function foo() public override(B, C) returns (string memory) { return "D"; }
}
```

### Virtual and Override Resolution

The `virtual` and `override` keywords work as in standard Solidity. The compiler resolves the final implementation through inheritance flattening. The `__super_` prefix is used internally to preserve parent implementations for `super` calls.

```solidity
contract Base {
    function greet() public virtual returns (string memory) {
        return "Hello from Base";
    }
}

contract Child is Base {
    function greet() public override returns (string memory) {
        string memory base = super.greet();
        // super.greet() calls the __super_ preserved method
        return string.concat(base, " and Child");
    }
}
```

### Interface Implementation

Interfaces define method signatures that implementing contracts must provide. Interface types are tracked and validated at compile time. At the ABI level, interface methods become manifest entries.

```solidity
interface IVault {
    function deposit(uint256 amount) external;
    function withdraw(uint256 amount) external;
    function balance() external view returns (uint256);
}

contract Vault is IVault {
    // All three methods must be implemented
    function deposit(uint256 amount) external override { /* ... */ }
    function withdraw(uint256 amount) external override { /* ... */ }
    function balance() external view override returns (uint256) { /* ... */ }
}
```

### Abstract Contracts

Abstract contracts can have unimplemented functions. The compiler validates that non-abstract contracts implement all inherited abstract methods and provides actionable error messages listing missing implementations.

---

## Visibility and Neo ABI

Solidity visibility modifiers determine whether a function appears in the Neo manifest ABI.

| Visibility | In Neo ABI? | Callable Externally? | Notes                                                    |
| ---------- | ----------- | -------------------- | -------------------------------------------------------- |
| `public`   | Yes         | Yes                  | Generates a manifest ABI method entry.                   |
| `external` | Yes         | Yes                  | Generates a manifest ABI method entry.                   |
| `internal` | No          | No                   | Only callable within the contract and derived contracts. |
| `private`  | No          | No                   | Only callable within the defining contract.              |

### State Mutability and Safe Flag

| Mutability | Neo Manifest  | Behavior                                                  |
| ---------- | ------------- | --------------------------------------------------------- |
| `view`     | `safe: true`  | Read-only. Cannot modify state.                           |
| `pure`     | `safe: true`  | No state access. Computed from inputs only.               |
| (default)  | `safe: false` | May modify state.                                         |
| `payable`  | `safe: false` | Accepted with warning (Neo has no native value transfer). |

```solidity
// public view → ABI method with safe: true
function balanceOf(address account) public view returns (uint256) {
    return balances[account];
}

// public (default) → ABI method with safe: false
function transfer(address to, uint256 amount) public returns (bool) {
    // modifies state
    balances[msg.sender] -= amount;
    balances[to] += amount;
    return true;
}

// internal → NOT in ABI
function _mint(address to, uint256 amount) internal {
    // only callable from within the contract
}
```

---

## Arithmetic

### Overflow Behavior

NeoVM uses arbitrary-precision BigInteger for all integer arithmetic. This fundamentally changes overflow semantics compared to EVM:

| Behavior                       | EVM (Solidity 0.8.x) | NeoVM                         |
| ------------------------------ | -------------------- | ----------------------------- |
| `uint8(255) + 1`               | Reverts (checked)    | `256` (no overflow)           |
| `uint8(0) - 1`                 | Reverts (checked)    | `-1` (no underflow)           |
| `unchecked { uint8(255) + 1 }` | `0` (wraps)          | `256` (no wrap)               |
| `int256.max + 1`               | Reverts (checked)    | `int256.max + 1` (BigInteger) |

::: warning
If your contract relies on overflow/underflow behavior for correctness (e.g., wrapping counters, fixed-point math), you must add explicit modular arithmetic or range checks.
:::

```solidity
// EVM-safe pattern that needs adaptation for NeoVM
function incrementCounter() public {
    // On EVM: wraps from 255 to 0 in unchecked block
    // On NeoVM: increments to 256 — no wrapping
    unchecked { counter++; }

    // ✅ Add explicit range check for NeoVM if wrapping is needed
    counter = (counter + 1) % 256;
}
```

### Division and Modulo

Integer division truncates toward zero on both EVM and NeoVM. Modulo follows the same semantics.

```solidity
int256 a = -7;
int256 b = 2;
int256 result = a / b;  // -3 (truncates toward zero)
int256 mod = a % b;     // -1
```

### Shift Operations

Bitwise shift operations (`<<`, `>>`) work on BigInteger values. Right shift on signed integers is arithmetic (sign-extending).

---

## Data Location Keywords

Solidity's data location keywords (`memory`, `storage`, `calldata`) are parsed for source compatibility but have different runtime semantics on NeoVM.

| Keyword    | EVM Meaning                     | NeoVM Behavior                                     |
| ---------- | ------------------------------- | -------------------------------------------------- |
| `memory`   | Heap-allocated, function-scoped | Stack-based. Implicit.                             |
| `storage`  | Persistent contract storage     | Maps to Neo Storage syscalls.                      |
| `calldata` | Read-only input data region     | Treated as `memory` (no calldata region on NeoVM). |

```solidity
// All three compile — calldata is treated as memory on NeoVM
function process(
    bytes memory memData,
    bytes calldata cdData,    // treated as memory
    uint256[] storage sData   // maps to Neo Storage
) internal { /* ... */ }
```

---

## ABI Encoding

### abi.encode / abi.decode

On Neo, `abi.encode` maps to `StdLib.serialize` and `abi.decode` maps to `StdLib.deserialize`. These produce Neo-native serialization, not Ethereum ABI encoding.

| Function                       | EVM Behavior            | Neo Behavior                      |
| ------------------------------ | ----------------------- | --------------------------------- |
| `abi.encode(...)`              | Ethereum ABI encoding   | `StdLib.serialize` (Neo native)   |
| `abi.encodePacked(...)`        | Tight packing           | Same as `abi.encode` on Neo       |
| `abi.decode(...)`              | Ethereum ABI decoding   | `StdLib.deserialize` (Neo native) |
| `abi.encodeWithSignature(...)` | Selector + ABI encoding | `System.Contract.Call` dispatch   |
| `abi.encodeWithSelector(...)`  | Selector + ABI encoding | `System.Contract.Call` dispatch   |

### Cross-Contract Calls

`abi.encodeWithSignature` and `abi.encodeWithSelector` are lowered to `System.Contract.Call` invocations. The function signature is used to determine the target method name.

```solidity
// Cross-contract call on Neo
// Compiles to: System.Contract.Call(targetHash, "transfer", CallFlags.All, [to, amount])
(bool success, ) = address(target).call(
    abi.encodeWithSignature("transfer(address,uint256)", to, amount)
);
```

---

## Function Dispatch

### Neo ABI Name-Based Dispatch

EVM dispatches function calls by 4-byte selector (keccak256 hash of the signature). Neo dispatches by method name string declared in the manifest ABI.

This has practical implications:

1. Function overloading with the same name but different parameters will collide.
2. Method names in the manifest must be unique.
3. The compiler warns when overloads produce duplicate ABI names.

```solidity
// ⚠️ Both produce "transfer" in Neo manifest — collision
function transfer(address to, uint256 amount) public { }
function transfer(address to, uint256 amount, bytes calldata data) public { }

// ✅ Distinct names — no collision
function transfer(address to, uint256 amount) public { }
function transferWithData(address to, uint256 amount, bytes calldata data) public { }
```

---

## Recommended Migration Patterns

### Authorization

```solidity
// ❌ EVM pattern — relies on msg.sender implicitly
function withdraw(uint256 amount) public {
    require(msg.sender == owner, "not owner");
    // ...
}

// ✅ Neo-idiomatic — explicit witness check
function withdraw(uint256 amount) public {
    require(Runtime.checkWitness(owner), "not authorized");
    // ...
}
```

### Token Transfers

```solidity
// ❌ EVM pattern — direct value transfer
function deposit() public payable {
    balances[msg.sender] += msg.value;
}

// ✅ Neo pattern — NEP-17 callback
function onNEP17Payment(address from, uint256 amount, bytes memory data) external {
    require(msg.sender == address(gasToken), "only GAS accepted");
    balances[from] += amount;
}
```

### Contract Deployment

```solidity
// 🚫 Blocked — new Contract() not supported
// MyChild child = new MyChild();

// ✅ Use ContractManagement
// ContractManagement.deploy(nef, manifest, data);
```

### Strict Compilation

Always compile with strict flags for production:

```bash
neo-solc MyContract.sol \
  --deny-wildcard-contracts \
  --deny-wildcard-methods \
  -o build/MyContract
```

---

## Further Reading

- [Feature Support](/solidity/feature-support) — complete feature matrix with status icons
- [EVM to NeoVM Mapping](/mapping/evm-to-neovm) — runtime value and opcode mapping
- [Standards Mapping](/devpack/standards) — ERC to NEP migration guides
- [Runtime Spec](/reference/runtime) — embedded runtime specification
- [Parity and Limitations](/reference/parity-limitations) — known fidelity gaps
