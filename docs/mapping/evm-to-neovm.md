# EVM to NeoVM Mapping

This page documents the semantic mapping between EVM (Ethereum Virtual Machine) concepts and NeoVM (Neo Virtual Machine) equivalents as implemented by the `neo-solidity` compiler. Every Solidity construct that references EVM runtime behavior is mapped to a Neo N3 equivalent — or blocked with a diagnostic when no safe equivalent exists.

Understanding these mappings is essential for writing correct contracts. EVM and NeoVM are fundamentally different execution environments: EVM is a stack-based machine with 256-bit words, gas metering, and account-based state; NeoVM is a stack-based machine with arbitrary-precision integers, GAS fee metering, and UTXO-inspired witness authorization.

---

## Why Mappings Exist

Solidity was designed for EVM. When the `neo-solidity` compiler targets NeoVM, every EVM-specific construct must be translated to a Neo equivalent. These translations fall into four categories:

| Category            | Description                                          | Example                                         |
| ------------------- | ---------------------------------------------------- | ----------------------------------------------- |
| Direct mapping      | Semantically equivalent Neo construct exists         | `msg.sender` → `Runtime.GetCallingScriptHash()` |
| Approximate mapping | Similar construct exists with behavioral differences | `block.difficulty` → `Runtime.getRandom()`      |
| Contextual mapping  | Only valid in specific Neo contexts                  | `msg.value` → `onNEP17Payment` amount parameter |
| Blocked             | No safe equivalent; compiler rejects the construct   | `delegatecall` → compile error                  |

---

## Runtime Value Mapping

### Message Context

| Solidity / EVM | NeoVM / Neo N3                        | Status | Notes                                                                                  |
| -------------- | ------------------------------------- | :----: | -------------------------------------------------------------------------------------- |
| `msg.sender`   | `System.Runtime.GetCallingScriptHash` |   ✅   | Returns the script hash of the calling contract or transaction signer.                 |
| `msg.value`    | `onNEP17Payment` amount parameter     |   ⚠️   | Only valid inside NEP-17/NEP-11 payment callbacks. No equivalent outside that context. |
| `msg.data`     | —                                     |   ❌   | Neo uses typed parameters in the ABI. No raw calldata concept.                         |
| `msg.sig`      | —                                     |   ❌   | Neo dispatches by method name string, not 4-byte selector.                             |

```solidity
// msg.sender → Runtime.GetCallingScriptHash()
function onlyOwner() internal view {
    require(msg.sender == owner, "not owner");
    // On Neo: compares calling script hash to stored owner hash
}

// msg.value → only valid in payment callback
function onNEP17Payment(address from, uint256 amount, bytes memory data) external {
    // msg.value maps to the `amount` parameter here
    require(msg.value >= minDeposit, "too low");
}
```

### Block Context

| Solidity / EVM                          | NeoVM / Neo N3              | Status | Notes                                                                      |
| --------------------------------------- | --------------------------- | :----: | -------------------------------------------------------------------------- |
| `block.timestamp`                       | `System.Runtime.GetTime`    |   ✅   | Returns block time in milliseconds, normalized to seconds by the compiler. |
| `block.number`                          | `Ledger.currentIndex`       |   ✅   | Returns the current block height.                                          |
| `block.chainid`                         | Neo network magic           |   ✅   | Compile-time constant. Mainnet: 860833102, Testnet: 894710606.             |
| `block.coinbase`                        | `address(0)`                |  ✅\*  | Auto-mapped with warning. dBFT consensus has no block miner.               |
| `block.difficulty` / `block.prevrandao` | `Runtime.getRandom()`       |  ✅\*  | Auto-mapped with warning. Different randomness model.                      |
| `block.gaslimit`                        | `Policy.getExecFeeFactor()` |  ✅\*  | Auto-mapped with warning. Different gas accounting model.                  |
| `block.basefee`                         | `Policy.getFeePerByte()`    |  ✅\*  | Auto-mapped with warning. Different fee model.                             |
| `blockhash(n)`                          | `Ledger.getBlockHash(n)`    |  ✅\*  | Auto-mapped with warning. Returns Neo block hash.                          |

\* Auto-mapped features compile successfully but emit a compiler warning because the semantic match is approximate.

```solidity
// block.timestamp → Runtime.GetTime() (seconds)
function isExpired(uint256 deadline) public view returns (bool) {
    return block.timestamp > deadline;
    // Neo: compares Runtime.GetTime() against deadline
}

// block.number → Ledger.currentIndex
function currentBlock() public view returns (uint256) {
    return block.number;
    // Neo: returns Ledger.currentIndex (block height)
}
```

### Transaction Context

| Solidity / EVM | NeoVM / Neo N3           | Status | Notes                                                                      |
| -------------- | ------------------------ | :----: | -------------------------------------------------------------------------- |
| `tx.origin`    | First signer script hash |   ⚠️   | Maps to the first transaction signer. Warning: authorization anti-pattern. |
| `tx.gasprice`  | `Policy.getFeePerByte()` |  ✅\*  | Auto-mapped with warning. Different fee model.                             |

```solidity
// tx.origin → first signer script hash (use with caution)
// ⚠️ The compiler warns: tx.origin-based auth is an anti-pattern
function unsafeAuth() public view returns (bool) {
    return tx.origin == trustedSigner;
}

// ✅ Prefer Runtime.checkWitness() instead
function safeAuth(address account) public view returns (bool) {
    return Runtime.checkWitness(account);
}
```

### Contract Context

| Solidity / EVM       | NeoVM / Neo N3                          | Status | Notes                                                                 |
| -------------------- | --------------------------------------- | :----: | --------------------------------------------------------------------- |
| `this`               | `System.Runtime.GetExecutingScriptHash` |   ✅   | Returns the script hash of the current contract.                      |
| `gasleft()`          | `System.Runtime.GasLeft`                |   ✅   | Returns remaining GAS for the current invocation.                     |
| `selfdestruct(addr)` | `ContractManagement.destroy()`          |  ✅\*  | Auto-mapped with warning. No refund. Permanent and irreversible.      |
| `address.codehash`   | Contract script hash                    |  ✅\*  | Auto-mapped with warning. Non-contract addresses return `bytes32(0)`. |

```solidity
// this → Runtime.GetExecutingScriptHash()
function getContractAddress() public view returns (address) {
    return address(this);
    // Neo: returns the executing contract's script hash
}

// gasleft() → System.Runtime.GasLeft
function checkGas() public view returns (uint256) {
    return gasleft();
    // Neo: returns remaining GAS in the current invocation
}
```

---

## Cryptography Mapping

| Solidity Builtin           | Neo Mapping                 | Status | Notes                                                                         |
| -------------------------- | --------------------------- | :----: | ----------------------------------------------------------------------------- |
| `keccak256(...)`           | `CryptoLib.keccak256`       |   ✅   | Identical algorithm. Called via native contract.                              |
| `sha256(...)`              | `CryptoLib.sha256`          |   ✅   | Identical algorithm. Called via native contract.                              |
| `ecrecover(hash, v, r, s)` | `CryptoLib.verifyWithECDsa` |   ✅   | Signature verification. Recovery path adapted for Neo's secp256r1/k1 support. |
| `ripemd160(...)`           | `CryptoLib.ripemd160`       |   ✅   | Available via native CryptoLib contract.                                      |

```solidity
// keccak256 → CryptoLib.keccak256
bytes32 hash = keccak256(abi.encodePacked(sender, amount, nonce));
// Neo: calls CryptoLib native contract's keccak256 method

// sha256 → CryptoLib.sha256
bytes32 digest = sha256(data);
// Neo: calls CryptoLib native contract's sha256 method

// ecrecover → CryptoLib.verifyWithECDsa
address signer = ecrecover(hash, v, r, s);
// Neo: uses CryptoLib.verifyWithECDsa with appropriate curve
```

::: info
Neo's CryptoLib native contract supports both secp256k1 (Ethereum-compatible) and secp256r1 (Neo-native) curves. The compiler routes `ecrecover` to the appropriate verification path.
:::

---

## Contract Call Mapping

| Solidity Pattern            | Neo Mapping                        | Status | Notes                                                                     |
| --------------------------- | ---------------------------------- | :----: | ------------------------------------------------------------------------- |
| `address.call(...)`         | `System.Contract.Call`             |   ✅   | Cross-contract call with full call flags.                                 |
| `address.staticcall(...)`   | `System.Contract.Call` (read-only) |   ✅   | Cross-contract call with read-only flag.                                  |
| `address.delegatecall(...)` | —                                  |   🚫   | Blocked. NeoVM has no delegate call mechanism.                            |
| `address.transfer(amount)`  | `GAS.transfer(from,to,amount,data)`|   ✅   | Auto-mapped. Uses executing script hash as `from`; aborts on failure.     |
| `address.send(amount)`      | `GAS.transfer(from,to,amount,data)`|   ✅   | Auto-mapped. Uses executing script hash as `from`; returns bool.           |
| `address.balance`           | `GAS.balanceOf(address)`           |   ✅   | Auto-mapped to GAS token balance query.                                   |
| `address.code`              | Empty bytes / existence probe      |   ⚠️   | `address.code` returns empty bytes; `address.code.length` is 0/1 contract check. |

### Cross-Contract Calls

Solidity's `address.call()` with `abi.encodeWithSignature` is lowered to `System.Contract.Call`. The compiler extracts the target contract hash, method name, and parameters from the encoding.

```solidity
// address.call → System.Contract.Call
(bool success, bytes memory result) = address(target).call(
    abi.encodeWithSignature("transfer(address,uint256)", to, amount)
);
// Neo: System.Contract.Call(targetHash, "transfer", CallFlags.All, [to, amount])

// address.staticcall → System.Contract.Call with read-only flag
(bool ok, bytes memory data) = address(target).staticcall(
    abi.encodeWithSignature("balanceOf(address)", account)
);
// Neo: System.Contract.Call(targetHash, "balanceOf", CallFlags.ReadOnly, [account])
```

### Why delegatecall Is Blocked

EVM's `delegatecall` executes code from another contract in the caller's storage context. NeoVM has no equivalent mechanism — each contract has its own isolated storage context. The proxy/upgrade pattern that relies on `delegatecall` on Ethereum is replaced by Neo's native `ContractManagement.update()` which upgrades contracts in-place.

```solidity
// 🚫 Blocked — no delegate call on Neo
// address(impl).delegatecall(data);

// ✅ Neo alternative: native contract upgrade
// ContractManagement.update(newNef, newManifest, data);
```

### How transfer/send Are Mapped

EVM's `address.transfer()` and `address.send()` forward a fixed gas stipend (2300 gas) with native Ether. Neo does not have native value transfer through address members. Token transfers on Neo use the NEP-17 standard.

```solidity
// ✅ Auto-mapped by neo-solidity:
// payable(recipient).transfer(1 ether);
// payable(recipient).send(1 ether);

// Lowered to GAS.transfer(address(this), recipient, amount, "")
// (transfer aborts on failure; send returns bool)
```

---

## Storage Lowering

### EVM Storage Slots vs Neo Storage Keys

EVM uses 256-bit storage slots addressed by sequential indices. Solidity computes slot positions using keccak256 hashing for mappings and arrays. Neo uses key-value storage accessed through syscalls, with keys being arbitrary byte arrays.

| Concept           | EVM                       | Neo                                               |
| ----------------- | ------------------------- | ------------------------------------------------- |
| Storage model     | 256-bit slots, sequential | Key-value pairs, byte array keys                  |
| Simple variable   | Slot index (0, 1, 2, ...) | `SHA256(variable_name)`                           |
| Mapping key       | `keccak256(key . slot)`   | `SHA256(serialize(key) \|\| slot_hash)`           |
| Nested mapping    | Iterated keccak256        | Iterated SHA256                                   |
| Array length      | Slot `p`                  | `SHA256(variable_name)`                           |
| Array element `i` | `keccak256(p) + i`        | `SHA256(serialize(i) \|\| SHA256(variable_name))` |
| Struct            | Consecutive slots         | `StdLib.serialize()` blob at single key           |

### Deterministic Key Derivation

The compiler derives storage keys using SHA256 hashing. The base slot for each state variable is `SHA256(variable_name)`.

For mappings, keys are derived iteratively:

```
// Simple mapping: mapping(address => uint256) balances;
slot_hash = SHA256("balances")
storage_key = SHA256(serialize(key) || slot_hash)

// Nested mapping: mapping(address => mapping(uint256 => bool)) approvals;
slot_hash   = SHA256("approvals")
level_1     = SHA256(serialize(outer_key) || slot_hash)
level_2     = SHA256(serialize(inner_key) || level_1)
// level_2 is the final storage key
```

### Key Serialization

Keys are serialized to byte arrays before hashing:

| Key Type                             | Serialization Method                                                                                        |
| ------------------------------------ | ----------------------------------------------------------------------------------------------------------- |
| Integers (`uint256`, `int128`, etc.) | Big-endian byte array, padded to declared bit width. Signed integers sign-extended, unsigned zero-extended. |
| `bool`                               | Single byte: `0x00` (false) or `0x01` (true).                                                               |
| `address`                            | Raw 20 bytes (UInt160).                                                                                     |
| `bytesN` (fixed)                     | Raw bytes, no padding.                                                                                      |
| `bytes` / `string` (dynamic)         | UTF-8 bytes, no length prefix.                                                                              |

### Storage Syscalls

| Operation      | Syscall                     | Stack Shape                      |
| -------------- | --------------------------- | -------------------------------- |
| Read value     | `System.Storage.Get`        | `[context, key] → [value]`       |
| Write value    | `System.Storage.Put`        | `[context, key, value] → []`     |
| Delete value   | `System.Storage.Delete`     | `[context, key] → []`            |
| Get context    | `System.Storage.GetContext` | `[] → [context]`                 |
| Find by prefix | `System.Storage.Find`       | `[context, prefix] → [iterator]` |

```solidity
// Solidity source
mapping(address => uint256) public balances;

function setBalance(address account, uint256 amount) internal {
    balances[account] = amount;
}

// Compiled Neo behavior:
// 1. Compute slot_hash = SHA256("balances")
// 2. Serialize account key (20 bytes, raw)
// 3. Compute storage_key = SHA256(account_bytes || slot_hash)
// 4. System.Storage.GetContext → context
// 5. System.Storage.Put(context, storage_key, amount)
```

---

## Standards Mapping

### ERC-20 to NEP-17 (Fungible Tokens)

| ERC-20 Method                                    | NEP-17 Method                                        | Key Differences                                           |
| ------------------------------------------------ | ---------------------------------------------------- | --------------------------------------------------------- |
| `name() → string`                                | `name() → String`                                    | Identical.                                                |
| `symbol() → string`                              | `symbol() → String`                                  | Identical.                                                |
| `decimals() → uint8`                             | `decimals() → Integer`                               | Identical semantics.                                      |
| `totalSupply() → uint256`                        | `totalSupply() → Integer`                            | Identical semantics.                                      |
| `balanceOf(address) → uint256`                   | `balanceOf(Hash160) → Integer`                       | `address` → `Hash160`.                                    |
| `transfer(address, uint256) → bool`              | `transfer(Hash160, Hash160, Integer, Any) → Boolean` | 4 params: `(from, to, amount, data)`. Witness-based auth. |
| `approve(address, uint256) → bool`               | —                                                    | Not in NEP-17 spec. Use `Runtime.checkWitness()`.         |
| `transferFrom(address, address, uint256) → bool` | —                                                    | Use `transfer` with witness.                              |
| `allowance(address, address) → uint256`          | —                                                    | Not in NEP-17 spec.                                       |
| —                                                | `onNEP17Payment(Hash160, Integer, Any)`              | Neo-only callback for receiving tokens.                   |

```solidity
// ❌ ERC-20 style — will NOT produce NEP-17 compliant manifest
function transfer(address to, uint256 amount) public returns (bool) { ... }

// ✅ NEP-17 style — 4 parameters with witness authorization
function transfer(address from, address to, uint256 amount, Any calldata data)
    public returns (bool)
{
    require(Runtime.checkWitness(from), "unauthorized");
    _transfer(from, to, amount);
    return true;
}

// ✅ NEP-17 payment callback (replaces receive()/fallback())
function onNEP17Payment(address from, uint256 amount, bytes memory data) external {
    // Handle incoming token payment
}
```

The compiler auto-detects NEP-17 when all 5 required methods (`name`, `symbol`, `decimals`, `totalSupply`, `balanceOf`) are present and `ownerOf` is absent. The manifest `supportedstandards` array will include `"NEP-17"`.

### ERC-721 to NEP-11 (Non-Fungible Tokens)

| ERC-721 Method                            | NEP-11 Method                                      | Key Differences                           |
| ----------------------------------------- | -------------------------------------------------- | ----------------------------------------- |
| `ownerOf(uint256) → address`              | `ownerOf(ByteArray) → Hash160`                     | Token ID: `uint256` → `bytes32`.          |
| `transferFrom(address, address, uint256)` | `transfer(Hash160, ByteArray, Any) → Boolean`      | 3 params. Witness-based auth.             |
| `safeTransferFrom(...)`                   | `transfer(...)`                                    | Merged into single transfer.              |
| `approve(address, uint256)`               | —                                                  | Not in NEP-11 spec.                       |
| —                                         | `decimals() → Integer`                             | Required. Returns 0 for indivisible NFTs. |
| —                                         | `tokensOf(Hash160) → Iterator`                     | Required. Enumerate tokens by owner.      |
| —                                         | `properties(ByteArray) → Map`                      | Required. Token metadata.                 |
| —                                         | `onNEP11Payment(Hash160, Integer, ByteArray, Any)` | Neo-only callback.                        |

```solidity
// ❌ ERC-721 style
function transferFrom(address from, address to, uint256 tokenId) public { ... }

// ✅ NEP-11 style — 3 parameters with witness authorization
function transfer(address to, bytes32 tokenId, bytes calldata data)
    public returns (bool)
{
    address tokenOwner = ownerOf(tokenId);
    require(Runtime.checkWitness(tokenOwner), "unauthorized");
    _transfer(tokenOwner, to, tokenId);
    return true;
}

// ✅ NEP-11 required methods
function decimals() public pure returns (uint8) { return 0; }
function tokensOf(address owner) public view returns (bytes32[] memory) { ... }
function properties(bytes32 tokenId) public view returns (bytes memory) { ... }
```

The compiler auto-detects NEP-11 when `balanceOf` + `ownerOf` are present. The manifest `supportedstandards` array will include `"NEP-11"`.

### ERC-2981 to NEP-24 (Royalty Standard)

| ERC-2981 Method                                      | NEP-24 Method                                      | Key Differences                                  |
| ---------------------------------------------------- | -------------------------------------------------- | ------------------------------------------------ |
| `royaltyInfo(uint256, uint256) → (address, uint256)` | `royaltyInfo(ByteArray, Hash160, Integer) → Array` | Multiple recipients. Extra `royaltyToken` param. |
| `supportsInterface(bytes4) → bool`                   | Manifest `supportedstandards: ["NEP-24"]`          | Manifest-based detection.                        |

NEP-24 returns an array of `[recipient, amount]` pairs, supporting split royalties across multiple recipients. It also adds a `royaltyToken` parameter specifying which token the royalty should be paid in.

### EIP-165 to Manifest supportedstandards

| EIP-165 (Ethereum)                         | Neo Manifest                                         |
| ------------------------------------------ | ---------------------------------------------------- |
| `supportsInterface(0x80ac58cd)` → ERC-721  | `"supportedstandards": ["NEP-11"]`                   |
| `supportsInterface(0x36372b07)` → ERC-20   | `"supportedstandards": ["NEP-17"]`                   |
| `supportsInterface(0x2a55205a)` → ERC-2981 | `"supportedstandards": ["NEP-24"]`                   |
| Runtime query via `staticcall`             | Read manifest via `ContractManagement.getContract()` |

The `neo-solidity` compiler automatically populates `supportedstandards` based on method signature analysis. No `supportsInterface()` function is needed.

---

## Authorization Model

### EVM: msg.sender + Allowance Pattern

On Ethereum, authorization is implicit through `msg.sender`. The caller's address is automatically set by the EVM based on who initiated the call. Token standards use the approve/allowance pattern for delegated spending.

```solidity
// EVM authorization pattern
function transfer(address to, uint256 amount) public returns (bool) {
    // msg.sender is automatically the caller
    require(balances[msg.sender] >= amount, "insufficient");
    balances[msg.sender] -= amount;
    balances[to] += amount;
    return true;
}

// EVM delegated spending
function transferFrom(address from, address to, uint256 amount) public returns (bool) {
    require(allowances[from][msg.sender] >= amount, "not approved");
    allowances[from][msg.sender] -= amount;
    balances[from] -= amount;
    balances[to] += amount;
    return true;
}
```

### Neo: CheckWitness + Direct Authorization

On Neo, authorization is explicit through witness verification. The transaction includes cryptographic witnesses (signatures) that prove the caller controls specific addresses. `Runtime.checkWitness(address)` verifies this at the VM level.

```solidity
// Neo authorization pattern
function transfer(address from, address to, uint256 amount, Any calldata data)
    public returns (bool)
{
    // Explicit witness check — proves caller controls `from`
    require(Runtime.checkWitness(from), "unauthorized");
    require(balances[from] >= amount, "insufficient");
    balances[from] -= amount;
    balances[to] += amount;
    return true;
}
```

### Migration Guide

| EVM Pattern                      | Neo Equivalent                                               |
| -------------------------------- | ------------------------------------------------------------ |
| `require(msg.sender == owner)`   | `require(Runtime.checkWitness(owner))`                       |
| `require(msg.sender == from)`    | `require(Runtime.checkWitness(from))`                        |
| `approve(spender, amount)`       | Not needed — use `checkWitness` directly                     |
| `transferFrom(from, to, amount)` | `transfer(from, to, amount, data)` with `checkWitness(from)` |
| `allowance[owner][spender]`      | Not needed — witness model replaces allowances               |
| `receive() external payable`     | `onNEP17Payment(from, amount, data)`                         |

::: tip
The approve/allowance pattern is unnecessary on Neo because `Runtime.checkWitness()` directly verifies that the transaction signer controls the `from` address. This eliminates an entire class of approval-related vulnerabilities (infinite approvals, front-running approve).
:::

---

## Gas Model Differences

### EVM Gas vs Neo GAS Fees

| Aspect       | EVM                         | Neo                                 |
| ------------ | --------------------------- | ----------------------------------- |
| Unit         | Gas (abstract unit)         | GAS (native token, 10^8 decimals)   |
| Pricing      | Per-opcode gas table        | Per-opcode + per-syscall fee table  |
| Payment      | `gasPrice * gasUsed` in ETH | System fee + network fee in GAS     |
| Refund       | Unused gas refunded         | Unused GAS not refunded             |
| Limit        | `block.gaslimit` per block  | Per-transaction fee limit           |
| Storage cost | 20,000 gas per SSTORE       | Per-byte storage pricing via Policy |

### How Gas Accounting Differs

On EVM, each opcode has a fixed gas cost, and the total gas consumed determines the transaction fee. On Neo, fees are split into:

1. System fee — covers execution cost (opcode + syscall fees).
2. Network fee — covers transaction size and signature verification.

The `gasleft()` function maps to `System.Runtime.GasLeft`, which returns the remaining GAS budget for the current invocation.

```solidity
// gasleft() → System.Runtime.GasLeft
function checkRemainingGas() public view returns (uint256) {
    return gasleft();
    // Returns remaining GAS in the current invocation (10^8 decimals)
}
```

::: warning
Ether unit literals (`1 ether = 10^18 wei`, `1 gwei = 10^9 wei`) are parsed for source compatibility but emit a warning. Neo GAS uses 10^8 decimals, not 10^18. Adjust your constants:

```solidity
// ⚠️ EVM units — warning emitted
uint256 fee = 0.1 ether; // 10^17 wei — wrong for Neo

// ✅ Neo GAS units
uint256 fee = 10_000_000; // 0.1 GAS (10^7, since GAS has 10^8 decimals)
```

:::

---

## Auto-Mapping Warnings

The compiler auto-maps several EVM globals to approximate Neo equivalents. Each auto-mapping emits a compiler warning to ensure developers understand the semantic differences.

| EVM Global                              | Auto-Mapped To                 | Warning Reason                                                                  |
| --------------------------------------- | ------------------------------ | ------------------------------------------------------------------------------- |
| `block.coinbase`                        | `address(0)`                   | dBFT consensus has no block miner. The value is always zero.                    |
| `block.difficulty` / `block.prevrandao` | `Runtime.getRandom()`          | EVM uses block difficulty/randomness beacon. Neo uses a different PRNG.         |
| `block.gaslimit`                        | `Policy.getExecFeeFactor()`    | EVM block gas limit vs Neo execution fee factor — different concepts.           |
| `block.basefee`                         | `Policy.getFeePerByte()`       | EIP-1559 base fee vs Neo per-byte fee — different fee models.                   |
| `tx.gasprice`                           | `Policy.getFeePerByte()`       | Transaction gas price vs Neo per-byte fee.                                      |
| `blockhash(n)`                          | `Ledger.getBlockHash(n)`       | Semantic match, but returns a Neo block hash (different chain).                 |
| `selfdestruct(addr)`                    | `ContractManagement.destroy()` | No ETH refund. Permanent contract destruction. Storage is deleted.              |
| `address.codehash`                      | Contract script hash           | Returns the contract's script hash. Non-contract addresses return `bytes32(0)`. |

### How to Handle Auto-Mapping Warnings

1. Review each warning to understand the semantic difference.
2. If the approximate mapping is acceptable for your use case, the warning can be acknowledged.
3. If exact EVM semantics are required, refactor the code to use Neo-native alternatives.

```solidity
// ⚠️ Warning: block.coinbase auto-mapped to address(0)
// If you need the block producer, this value is meaningless on Neo
address miner = block.coinbase; // Always address(0) on Neo

// ⚠️ Warning: block.difficulty auto-mapped to Runtime.getRandom()
// If you need randomness, this works but uses a different PRNG
uint256 random = block.prevrandao;

// ✅ Better: use Neo-native randomness directly
// uint256 random = Runtime.getRandom();
```

---

## Manifest Impact

Cross-contract calls and native contract invocations are reflected in the Neo manifest's `permissions` array. The compiler infers permissions from the IR and emits explicit `contract + methods` entries where possible.

### Permission Inference

| Call Pattern                                            | Manifest Permission                                    |
| ------------------------------------------------------- | ------------------------------------------------------ |
| `NativeCalls.gasTransfer(...)`                          | `{ "contract": "0xd2a4...", "methods": ["transfer"] }` |
| `address(known).call(abi.encodeWithSignature("foo()"))` | `{ "contract": "0x...", "methods": ["foo"] }`          |
| Dynamic target / dynamic method                         | `{ "contract": "*", "methods": "*" }` (wildcard)       |

### Hardening Permissions

Dynamic call sites can force wildcard permissions, which is a security risk. Use strict compiler flags to reject wildcards:

```bash
neo-solc contract.sol \
  --deny-wildcard-contracts \
  --deny-wildcard-methods \
  -o build/contract
```

::: warning
Wildcard permissions (`"contract": "*"` or `"methods": "*"`) allow the contract to call any contract or method. Always audit the generated manifest before deployment and use `--deny-wildcard-contracts --deny-wildcard-methods` in production builds.
:::

---

## Complete Type Mapping

| Solidity Type         | Neo ABI Type    | NeoVM Representation | Notes                                  |
| --------------------- | --------------- | -------------------- | -------------------------------------- |
| `address`             | `Hash160`       | UInt160 (20 bytes)   | Account or contract script hash.       |
| `uint256` / `int256`  | `Integer`       | BigInteger           | Arbitrary precision.                   |
| `uint8` .. `uint128`  | `Integer`       | BigInteger           | All widths map to `Integer`.           |
| `bool`                | `Boolean`       | Boolean              | Identical semantics.                   |
| `string`              | `String`        | UTF-8 ByteString     | UTF-8 encoded.                         |
| `bytes`               | `ByteArray`     | ByteArray            | Dynamic byte array.                    |
| `bytes32`             | `Hash256`       | ByteArray (32 bytes) | Used for token IDs.                    |
| `bytes4` .. `bytes31` | `ByteArray`     | ByteArray (fixed)    | Fixed-size byte arrays.                |
| `address[]`           | `Array`         | NeoVM Array          | Array of Hash160.                      |
| `mapping(K => V)`     | Storage         | Neo Storage          | Compiled to storage prefix operations. |
| `struct`              | `Array` / `Map` | NeoVM Array          | Serialized via `StdLib.serialize()`.   |
| `enum`                | `Integer`       | BigInteger           | Backed by uint8.                       |
| `Any` (neo-solidity)  | `Any`           | StackItem            | Unconstrained NeoVM StackItem.         |

---

## Further Reading

- [Feature Support](/solidity/feature-support) — complete 142-feature support matrix
- [Syntax and Behavior](/solidity/syntax-and-behavior) — detailed behavioral semantics on NeoVM
- [Standards Mapping](/devpack/standards) — detailed ERC to NEP migration guides with checklists
- [Native Contracts](/neovm/native-contracts) — Neo native contract reference
- [Syscalls](/neovm/syscalls) — NeoVM syscall surface
- [Runtime Spec](/reference/runtime) — embedded runtime specification
- [Parity and Limitations](/reference/parity-limitations) — known fidelity gaps
