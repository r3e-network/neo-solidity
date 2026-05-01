# EIP ↔ NEP Standards Mapping

> Comprehensive mapping between Ethereum Improvement Proposals (EIP) and Neo
> Enhancement Proposals (NEP) as implemented by the `neo-devpack-solidity` compiler.

## Quick Reference

| Ethereum (EIP)    | Neo (NEP) | Status       | Key Differences                                                         |
| ----------------- | --------- | ------------ | ----------------------------------------------------------------------- |
| ERC-20            | NEP-17    | ✅ Full      | 4-param `transfer`, `onNEP17Payment` callback, `Any` data type          |
| ERC-721           | NEP-11    | ✅ Full      | `bytes32` tokenId, `tokensOf`/`properties` required, `onNEP11Payment`   |
| ERC-2981          | NEP-24    | ✅ Full      | Array return for multiple royalty recipients                            |
| ERC-1155          | —         | ⚠️ Partial   | No direct equivalent; use NEP-11 divisible mode                         |
| EIP-165           | Manifest  | 🔄 Different | Neo uses manifest `supportedstandards` instead of `supportsInterface()` |
| EIP-2612 (Permit) | —         | 🔄 Different | Neo uses `Runtime.checkWitness()` instead of off-chain signatures       |
| EIP-1967 (Proxy)  | NEP-22/29/31 | 🔄 Different | Neo upgrades in-place via `update`, uses `_deploy` callback, optional `destroy` |
| ERC-721 Receiver  | NEP-26    | 🔄 Different | Neo uses explicit `onNEP11Payment` callback                             |
| ERC-677 / ERC-1363 style hooks | NEP-27 | 🔄 Different | Neo uses explicit `onNEP17Payment` callback                             |

---

## 1. ERC-20 ↔ NEP-17 (Fungible Tokens)

**Spec:** [NEP-17](https://github.com/neo-project/proposals/blob/master/nep-17.mediawiki)

### Method Signature Mapping

| ERC-20 (Ethereum)                                | NEP-17 (Neo)                                                             | Notes                       |
| ------------------------------------------------ | ------------------------------------------------------------------------ | --------------------------- |
| `name() → string`                                | `name() → String`                                                        | Identical                   |
| `symbol() → string`                              | `symbol() → String`                                                      | Identical                   |
| `decimals() → uint8`                             | `decimals() → Integer`                                                   | Identical semantics         |
| `totalSupply() → uint256`                        | `totalSupply() → Integer`                                                | Identical semantics         |
| `balanceOf(address) → uint256`                   | `balanceOf(Hash160) → Integer`                                           | `address` → `Hash160`       |
| `transfer(address to, uint256 amount) → bool`    | `transfer(Hash160 from, Hash160 to, Integer amount, Any data) → Boolean` | **4 params, witness-based** |
| `approve(address, uint256) → bool`               | —                                                                        | Not in NEP-17 spec          |
| `transferFrom(address, address, uint256) → bool` | —                                                                        | Use `transfer` with witness |
| `allowance(address, address) → uint256`          | —                                                                        | Not in NEP-17 spec          |
| —                                                | `onNEP17Payment(Hash160 from, Integer amount, Any data)`                 | **Neo-only callback**       |

### Key Differences

1. **Authorization model**: ERC-20 uses `msg.sender` + allowance pattern. NEP-17 uses
   `Runtime.checkWitness(from)` — the caller proves they control the `from` address.
2. **Transfer signature**: NEP-17 `transfer` takes 4 parameters `(from, to, amount, data)`.
   The `data` parameter (type `Any`) is passed to the recipient's `onNEP17Payment` callback.
3. **No approve/allowance**: NEP-17 does not define an allowance mechanism. Contracts that
   need delegated spending should implement it as an extension (the devpack includes one).
4. **Payment callback**: Contracts receiving NEP-17 tokens **must** implement
   `onNEP17Payment(from, amount, data)` or the transfer reverts.

### Solidity Migration Pattern

```solidity
// ❌ ERC-20 style (will NOT produce NEP-17 compliant manifest)
function transfer(address to, uint256 amount) public returns (bool) { ... }

// ✅ NEP-17 style
function transfer(address from, address to, uint256 amount, Any calldata data)
    public returns (bool)
{
    require(Runtime.checkWitness(from), "unauthorized");
    _transfer(from, to, amount);
    return true;
}

// ✅ NEP-17 payment callback (replaces Solidity receive())
function onNEP17Payment(address from, uint256 amount, bytes memory data) external {
    // Handle incoming token payment
}
```

### Event Mapping

| ERC-20 Event                                              | NEP-17 Event                                         |
| --------------------------------------------------------- | ---------------------------------------------------- |
| `Transfer(address from, address to, uint256 value)`       | `Transfer(Hash160 from, Hash160 to, Integer amount)` |
| `Approval(address owner, address spender, uint256 value)` | — (not in NEP-17 spec)                               |

### Compiler Behavior

- The compiler auto-detects NEP-17 when all 5 required methods are present and `ownerOf` is absent.
- Solidity `receive()` is automatically remapped to `onNEP17Payment` unless an explicit
  `onNEP17Payment` function already exists.
- The manifest `supportedstandards` array will include `"NEP-17"`.

---

## 2. ERC-721 ↔ NEP-11 (Non-Fungible Tokens)

**Spec:** [NEP-11](https://github.com/neo-project/proposals/blob/master/nep-11.mediawiki)

### Method Signature Mapping

| ERC-721 (Ethereum)                                   | NEP-11 (Neo)                                                  | Notes                       |
| ---------------------------------------------------- | ------------------------------------------------------------- | --------------------------- |
| `name() → string`                                    | `name() → String`                                             | Identical                   |
| `symbol() → string`                                  | `symbol() → String`                                           | Identical                   |
| `totalSupply() → uint256`                            | `totalSupply() → Integer`                                     | Identical semantics         |
| `balanceOf(address) → uint256`                       | `balanceOf(Hash160) → Integer`                                | Identical semantics         |
| `ownerOf(uint256 tokenId) → address`                 | `ownerOf(ByteArray tokenId) → Hash160`                        | **`uint256` → `bytes32`**   |
| `transferFrom(address, address, uint256)`            | `transfer(Hash160 to, ByteArray tokenId, Any data) → Boolean` | **3 params, witness-based** |
| `safeTransferFrom(address, address, uint256, bytes)` | `transfer(Hash160 to, ByteArray tokenId, Any data) → Boolean` | Merged into single transfer |
| `approve(address, uint256)`                          | —                                                             | Not in NEP-11 spec          |
| `setApprovalForAll(address, bool)`                   | —                                                             | Not in NEP-11 spec          |
| `getApproved(uint256) → address`                     | —                                                             | Not in NEP-11 spec          |
| `isApprovedForAll(address, address) → bool`          | —                                                             | Not in NEP-11 spec          |
| —                                                    | `decimals() → Integer`                                        | **Required**: returns 0     |
| —                                                    | `tokensOf(Hash160 owner) → Iterator`                          | **Neo-only**: enumerate     |
| —                                                    | `properties(ByteArray tokenId) → Map`                         | **Neo-only**: metadata      |
| —                                                    | `onNEP11Payment(Hash160, Integer, ByteArray, Any)`            | **Neo-only callback**       |

### Key Differences

1. **Token ID type**: ERC-721 uses `uint256`. NEP-11 uses `ByteArray` (mapped from
   Solidity `bytes32`). The compiler maps `bytes32` → Neo `Hash256`/`ByteArray`.
2. **Transfer signature**: NEP-11 `transfer(to, tokenId, data)` takes 3 parameters.
   Authorization is via `Runtime.checkWitness(owner)`, not `msg.sender`.
3. **No approval mechanism**: NEP-11 spec does not define `approve`/`getApproved`.
   Contracts may implement these as extensions (the devpack includes them).
4. **Required `decimals()`**: Must return `0` for indivisible NFTs. NEP-11 also
   supports divisible NFTs where `decimals() > 0`.
5. **Required `tokensOf()`**: Returns an iterator over token IDs owned by an address.
   No ERC-721 equivalent (ERC-721 Enumerable is optional).
6. **Required `properties()`**: Returns a serialized map of token metadata.
   Replaces ERC-721's `tokenURI()` approach.

### Solidity Migration Pattern

```solidity
// ❌ ERC-721 style
function transferFrom(address from, address to, uint256 tokenId) public { ... }

// ✅ NEP-11 style
function transfer(address to, bytes32 tokenId, bytes calldata data)
    public returns (bool)
{
    address tokenOwner = ownerOf(tokenId);
    require(Runtime.checkWitness(tokenOwner), "unauthorized");
    _transfer(tokenOwner, to, tokenId);
    return true;
}

// ✅ NEP-11 required: decimals must return 0 for indivisible NFTs
function decimals() public pure returns (uint8) { return 0; }

// ✅ NEP-11 required: enumerate tokens owned by address
function tokensOf(address owner) public view returns (bytes32[] memory) { ... }

// ✅ NEP-11 required: return token properties as serialized map
function properties(bytes32 tokenId) public view returns (bytes memory) { ... }
```

### Event Mapping

| ERC-721 Event                                                | NEP-11 Event                                                            |
| ------------------------------------------------------------ | ----------------------------------------------------------------------- |
| `Transfer(address from, address to, uint256 tokenId)`        | `Transfer(Hash160 from, Hash160 to, Integer amount, ByteArray tokenId)` |
| `Approval(address owner, address approved, uint256 tokenId)` | — (not in NEP-11 spec)                                                  |
| `ApprovalForAll(address owner, address operator, bool)`      | — (not in NEP-11 spec)                                                  |

> **Note:** NEP-11 Transfer event has 4 parameters. The `amount` field is `1` for
> indivisible NFTs. Contracts may emit Approval events as extensions.

### Compiler Behavior

- Auto-detects NEP-11 when `balanceOf` + `ownerOf` are present, plus at least one of
  `transfer`, `transferFrom`, or `tokensOf`.
- The manifest `supportedstandards` array will include `"NEP-11"`.
- `bytes32` parameters compile to Neo ABI type `Hash256` (functionally equivalent to `ByteArray`).

---

## 3. ERC-2981 ↔ NEP-24 (Royalty Standard)

**Spec:** [NEP-24](https://github.com/neo-project/proposals/blob/master/nep-24.mediawiki)

### Method Signature Mapping

| ERC-2981 (Ethereum)                                                    | NEP-24 (Neo)                                                                      | Notes                   |
| ---------------------------------------------------------------------- | --------------------------------------------------------------------------------- | ----------------------- |
| `royaltyInfo(uint256 tokenId, uint256 salePrice) → (address, uint256)` | `royaltyInfo(ByteArray tokenId, Hash160 royaltyToken, Integer salePrice) → Array` | **Multiple recipients** |
| `supportsInterface(bytes4) → bool`                                     | Manifest `supportedstandards: ["NEP-24"]`                                         | Manifest-based          |

### Key Differences

1. **Multiple recipients**: ERC-2981 returns a single `(receiver, amount)` tuple.
   NEP-24 returns an **array** of `[recipient, amount]` pairs, supporting split royalties.
2. **Royalty token parameter**: NEP-24 adds a `royaltyToken` parameter (Hash160) specifying
   which token the royalty should be paid in (e.g., GAS, a NEP-17 token).
3. **Basis points**: Both use basis points. NEP-24 convention: `10000 = 100%`.

### Compiler Behavior

- Auto-detects NEP-24 when `tokenURI` or `royaltyInfo` method is present.
- The manifest `supportedstandards` array will include `"NEP-24"`.

---

## 4. ERC-1155 (Multi-Token) — No Direct Neo Equivalent

ERC-1155 combines fungible and non-fungible tokens in a single contract. Neo N3 does
not have a dedicated multi-token standard. Migration strategies:

| ERC-1155 Feature        | Neo Approach                                         |
| ----------------------- | ---------------------------------------------------- |
| Fungible token IDs      | Deploy a separate NEP-17 contract per token type     |
| Non-fungible token IDs  | Use NEP-11 with divisible mode (`decimals() > 0`)    |
| `balanceOfBatch`        | Implement as contract extension; not in any NEP spec |
| `safeBatchTransferFrom` | Implement batch logic in a wrapper contract          |
| `uri(uint256 id)`       | Use NEP-11 `properties()` or NEP-24 `tokenURI()`     |

> **Recommendation:** For contracts that mix fungible and non-fungible assets, deploy
> separate NEP-17 and NEP-11 contracts and coordinate them via cross-contract calls
> using `abi.encodeWithSignature()`.

---

## 5. EIP-165 (Interface Detection) → Manifest-Based

Ethereum uses runtime `supportsInterface(bytes4)` calls. Neo uses the **contract
manifest** — a static JSON document deployed alongside the NEF bytecode.

| EIP-165 (Ethereum)                         | Neo Manifest                                         |
| ------------------------------------------ | ---------------------------------------------------- |
| `supportsInterface(0x80ac58cd)` → ERC-721  | `"supportedstandards": ["NEP-11"]`                   |
| `supportsInterface(0x36372b07)` → ERC-20   | `"supportedstandards": ["NEP-17"]`                   |
| `supportsInterface(0x2a55205a)` → ERC-2981 | `"supportedstandards": ["NEP-24"]`                   |
| Runtime query via `staticcall`             | Read manifest via `ContractManagement.getContract()` |

### Compiler Behavior

The `neo-devpack-solidity` compiler **automatically populates** `supportedstandards` based on
method signature analysis. No `supportsInterface()` function is needed.

---

## 6. EIP-2612 (Permit) → `Runtime.checkWitness()`

Ethereum's Permit standard enables gasless approvals via off-chain EIP-712 signatures.
Neo's transaction model makes this unnecessary.

| EIP-2612 (Ethereum)                                | Neo Equivalent                                |
| -------------------------------------------------- | --------------------------------------------- |
| `permit(owner, spender, value, deadline, v, r, s)` | Not needed — use `Runtime.checkWitness()`     |
| Off-chain EIP-712 signature                        | Transaction witness (built into Neo protocol) |
| `nonces(address) → uint256`                        | Not needed — replay protection is native      |
| `DOMAIN_SEPARATOR() → bytes32`                     | Not needed                                    |

### Why Permit Is Unnecessary on Neo

Neo transactions include **witness scopes** that cryptographically prove the caller
controls an address. `Runtime.checkWitness(address)` verifies this at the VM level.
There is no need for off-chain signature schemes or nonce tracking.

---

## 7. Lifecycle and Callback NEPs (NEP-22/26/27/29/30/31)

Neo N3 defines additional contract behavior standards beyond NEP-11/17/24.

| NEP | Required Method | Purpose |
| --- | --------------- | ------- |
| NEP-22 | `update(nefFile, manifest, data)` | Standard contract update method |
| NEP-26 | `onNEP11Payment(from, amount, tokenId, data)` | NEP-11 receiver callback |
| NEP-27 | `onNEP17Payment(from, amount, data)` | NEP-17 receiver callback |
| NEP-29 | `_deploy(data, update)` | Deploy/update lifecycle callback |
| NEP-30 | `verify(...) -> bool` | Witness verification entrypoint |
| NEP-31 | `destroy()` | Standard destroy method |

### Compiler Behavior

- `neo-devpack-solidity` detects and advertises the corresponding NEP when these signatures are present.
- Detection is signature-based; methods with the same name but incompatible arity/return type are reported as near-misses.
- Lifecycle NEPs are additive to token standards (e.g., a contract can advertise both `NEP-17` and `NEP-27`).

---

## 8. Solidity ↔ Neo ABI Type Mapping

| Solidity Type        | Neo ABI Type  | Notes                                     |
| -------------------- | ------------- | ----------------------------------------- |
| `address`            | `Hash160`     | 20-byte account/contract hash             |
| `uint256` / `int256` | `Integer`     | Arbitrary-precision integer on NeoVM      |
| `uint8` … `uint128`  | `Integer`     | All integer widths map to `Integer`       |
| `bool`               | `Boolean`     | Identical semantics                       |
| `string`             | `String`      | UTF-8 encoded                             |
| `bytes`              | `ByteArray`   | Dynamic byte array                        |
| `bytes32`            | `Hash256`     | 32-byte fixed array (used for token IDs)  |
| `bytes4` … `bytes31` | `ByteArray`   | Fixed-size byte arrays                    |
| `address[]`          | `Array`       | Array of Hash160                          |
| `mapping(K => V)`    | Storage       | Compiled to Neo storage prefix operations |
| `struct`             | `Array`/`Map` | Serialized via `StdLib.serialize()`       |
| `Any` (neo-devpack-solidity) | `Any`         | Unconstrained NeoVM StackItem             |

### Neo-Specific Types

The `neo-devpack-solidity` compiler introduces the `Any` type for NEP-17/NEP-11 `data`
parameters. This maps to NeoVM's unconstrained `StackItem`, allowing callers to
pass any serializable value.

```solidity
// The `Any` type is available without import in neo-devpack-solidity
function transfer(address from, address to, uint256 amount, Any calldata data)
    public returns (bool) { ... }
```

---

## 9. Migration Checklist

When porting an Ethereum contract to Neo N3 via `neo-devpack-solidity`:

### ERC-20 → NEP-17

- [ ] Change `transfer(to, amount)` to `transfer(from, to, amount, data)` with 4 params
- [ ] Add `Runtime.checkWitness(from)` instead of relying on `msg.sender`
- [ ] Add `onNEP17Payment(from, amount, data)` callback if contract receives tokens
- [ ] Replace `receive()` / `fallback()` with `onNEP17Payment`
- [ ] Remove `approve`/`transferFrom`/`allowance` from core interface (keep as extensions)
- [ ] Use `Any` type for the `data` parameter

### ERC-721 → NEP-11

- [ ] Change token ID type from `uint256` to `bytes32`
- [ ] Change `transferFrom(from, to, tokenId)` to `transfer(to, tokenId, data)` with 3 params
- [ ] Add `decimals()` returning `0` for indivisible NFTs
- [ ] Add `tokensOf(owner)` returning token ID array/iterator
- [ ] Add `properties(tokenId)` returning serialized metadata
- [ ] Add `onNEP11Payment(from, amount, tokenId, data)` callback
- [ ] Remove `safeTransferFrom` (merged into `transfer`)

### General

- [ ] Replace `{value: ...}` with `NativeCalls.gasTransfer()` / `NativeCalls.neoTransfer()`
- [ ] Replace `receive()` / `fallback()` with `onNEP17Payment()`
- [ ] Use parameterless constructor (Neo deploy constraint)
- [ ] Import devpack via `-I devpack` compiler flag
- [ ] No inline assembly — use library functions
- [ ] No same-argument-count overloads — use distinct function names

---

## 10. Devpack Standard Interfaces

| File                                | Standard | Description                         |
| ----------------------------------- | -------- | ----------------------------------- |
| `devpack/standards/NEP17.sol`       | NEP-17   | Full fungible token with extensions |
| `devpack/standards/NEP11.sol`       | NEP-11   | Full NFT with enumeration           |
| `devpack/standards/NEP24.sol`       | NEP-24   | Royalty standard                    |
| `devpack/standards/NEP22.sol`       | NEP-22   | Update method interface             |
| `devpack/standards/NEP26.sol`       | NEP-26   | NEP-11 receiver callback interface  |
| `devpack/standards/NEP27.sol`       | NEP-27   | NEP-17 receiver callback interface  |
| `devpack/standards/NEP29.sol`       | NEP-29   | Deploy callback interface           |
| `devpack/standards/NEP30.sol`       | NEP-30   | Verify callback interface           |
| `devpack/standards/NEP31.sol`       | NEP-31   | Destroy method interface            |
| `devpack/contracts/NativeCalls.sol` | —        | GAS/NEO native token transfers      |
| `devpack/libraries/Runtime.sol`     | —        | `checkWitness`, `getTime`, etc.     |
| `devpack/libraries/Storage.sol`     | —        | Persistent storage operations       |
| `devpack/libraries/Neo.sol`         | —        | Neo-specific utilities              |
