# Standards and Contracts

The devpack provides complete implementations of Neo N3 token standards — NEP-17 (fungible tokens), NEP-11 (non-fungible tokens), and NEP-24 (royalty metadata) — plus reusable lifecycle/callback interfaces for NEP-22/26/27/29/30/31. The compiler auto-detects standard compliance and populates the manifest's `supportedstandards` array.

## Supported Standards

| Standard | Type               | Ethereum Equivalent | Devpack File          |
| -------- | ------------------ | ------------------- | --------------------- |
| NEP-17   | Fungible Token     | ERC-20              | `standards/NEP17.sol` |
| NEP-11   | Non-Fungible Token | ERC-721             | `standards/NEP11.sol` |
| NEP-24   | Royalty Metadata   | ERC-2981            | `standards/NEP24.sol` |
| NEP-22   | Contract Update    | EIP-1967 (partial)  | `standards/NEP22.sol` |
| NEP-26   | NEP-11 Receiver    | ERC-721 Receiver    | `standards/NEP26.sol` |
| NEP-27   | NEP-17 Receiver    | ERC-677/1363-style  | `standards/NEP27.sol` |
| NEP-29   | Deploy Callback    | —                   | `standards/NEP29.sol` |
| NEP-30   | Verify Callback    | —                   | `standards/NEP30.sol` |
| NEP-31   | Destroy Method     | `selfdestruct` (partial) | `standards/NEP31.sol` |

## Ethereum to Neo Standard Mapping

| Ethereum          | Neo                                  | Key Differences                                                                       |
| ----------------- | ------------------------------------ | ------------------------------------------------------------------------------------- |
| ERC-20            | NEP-17                               | 4-parameter transfer, witness auth, no approve/allowance                              |
| ERC-721           | NEP-11                               | `bytes32` token IDs, 3-param transfer, witness auth, required `tokensOf`/`properties` |
| ERC-2981          | NEP-24                               | Multiple royalty recipients, `royaltyToken` parameter                                 |
| EIP-165           | Manifest `supportedstandards`        | Interface detection is manifest-based, no `supportsInterface()`                       |
| EIP-2612 (Permit) | `Runtime.checkWitness()`             | No permit needed — witness model replaces approvals                                   |
| EIP-1967 (Proxy)  | NEP-22/29/31                          | Native update + deploy callback + optional destroy, no proxy pattern                  |

## NEP-17: Fungible Tokens

NEP-17 is Neo's fungible token standard. It replaces the ERC-20 approve/allowance authorization model with cryptographic witness verification, eliminating entire classes of vulnerabilities (infinite approvals, front-running attacks) at the protocol level.

**Spec:** [NEP-17 Proposal](https://github.com/neo-project/proposals/blob/master/nep-17.mediawiki)

### Required Interface

| Method        | Parameters                                           | Return    | Safe | Description             |
| ------------- | ---------------------------------------------------- | --------- | ---- | ----------------------- |
| `symbol`      | —                                                    | `String`  | Yes  | Token symbol            |
| `decimals`    | —                                                    | `Integer` | Yes  | Decimal places (max 18) |
| `totalSupply` | —                                                    | `Integer` | Yes  | Total supply            |
| `balanceOf`   | `Hash160 account`                                    | `Integer` | Yes  | Account balance         |
| `transfer`    | `Hash160 from, Hash160 to, Integer amount, Any data` | `Boolean` | No   | Transfer tokens         |

Required event:

```
Transfer(Hash160 from, Hash160 to, Integer amount)
```

::: info The `Any` Type
The `data` parameter uses the Neo-Solidity `Any` type, which maps to NeoVM's unconstrained `StackItem`. This allows callers to pass any serializable value to the recipient's `onNEP17Payment` callback. In the devpack, `Any` is defined as `type Any is bytes;`.
:::

### Authorization Model

The most significant difference between ERC-20 and NEP-17 is the authorization model:

|                            | ERC-20 (Ethereum)                              | NEP-17 (Neo)                          |
| -------------------------- | ---------------------------------------------- | ------------------------------------- |
| **Auth mechanism**         | `msg.sender` + approve/allowance               | `Runtime.checkWitness(from)`          |
| **Delegated transfers**    | `approve()` → `transferFrom()`                 | Not needed — witness proves ownership |
| **Approval storage**       | On-chain `mapping(owner => spender => amount)` | None                                  |
| **Infinite approval risk** | Yes — common vulnerability                     | Eliminated by design                  |
| **Front-running attacks**  | Possible on `approve()`                        | Not applicable                        |

Neo transactions include **witness scopes** that cryptographically prove the caller controls an address. `Runtime.checkWitness(from)` verifies this at the VM level — no off-chain signatures, no nonce tracking, no approval state.

```solidity
// ERC-20: relies on msg.sender identity
function transfer(address to, uint256 amount) public returns (bool) {
    require(msg.sender == from); // implicit via msg.sender
    // ...
}

// NEP-17: cryptographic witness verification
function transfer(address from, address to, uint256 amount, Any calldata data)
    public returns (bool)
{
    require(Runtime.checkWitness(from), "NEP17: unauthorized");
    // ...
}
```

::: warning No approve/allowance
NEP-17 does not define `approve()`, `transferFrom()`, or `allowance()`. The devpack's `NEP17.sol` includes them as optional ERC-20 compatibility extensions, but they are **not** part of the NEP-17 spec and do not contribute to standard detection.
:::

### Payment Callbacks

When tokens are transferred to a contract, the receiving contract's `onNEP17Payment` callback is invoked automatically:

```solidity
interface INEP17Receiver {
    function onNEP17Payment(address from, uint256 amount, Any calldata data) external;
}
```

|                      | Solidity (EVM)             | Neo-Solidity (NeoVM)                  |
| -------------------- | -------------------------- | ------------------------------------- |
| **Callback**         | `receive()` / `fallback()` | `onNEP17Payment(from, amount, data)`  |
| **Trigger**          | ETH sent to contract       | NEP-17 tokens transferred to contract |
| **Failure behavior** | Transfer reverts           | Transfer reverts                      |
| **Data passing**     | `msg.data`                 | Explicit `data` parameter             |

::: tip
Any contract that receives NEP-17 tokens **must** implement `onNEP17Payment`. If the recipient is a contract and does not implement this callback, the transfer reverts with `NEP17InvalidReceiver`.
:::

### Implementation Example

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import "standards/NEP17.sol";
import "libraries/Runtime.sol";

/// @title MyToken — A production NEP-17 fungible token
contract MyToken is NEP17 {
    using Runtime for *;

    address private _admin;

    constructor()
        NEP17(
            "My Token",     // name
            "MYT",          // symbol
            8,              // decimals
            100_000_000,    // initial supply (1 MYT = 1e8 units)
            0               // max supply (0 = unlimited)
        )
    {
        _admin = msg.sender;
    }

    /// @dev NEP-17 4-parameter transfer with witness authorization
    function transfer(
        address from,
        address to,
        uint256 amount,
        Any calldata data
    ) public override returns (bool) {
        require(Runtime.checkWitness(from), "MyToken: unauthorized");
        return super.transfer(from, to, amount, data);
    }

    /// @dev Callback for receiving NEP-17 tokens
    function onNEP17Payment(address from, uint256 amount, Any calldata data) external {
        // Accept all incoming token payments
        // Add custom logic here (e.g., token swap, staking deposit)
    }

    /// @dev Mint new tokens — restricted to admin with witness check
    function mintTokens(address to, uint256 amount) external {
        require(Runtime.checkWitness(_admin), "MyToken: not admin");
        mint(to, amount);
    }
}
```

::: info Manifest Output
Compiling this contract produces `supportedstandards: ["NEP-17"]` in the manifest. The compiler detects all 5 required methods (`symbol`, `decimals`, `totalSupply`, `balanceOf`, `transfer`) and confirms `ownerOf` is absent (distinguishing it from NEP-11).
:::

### ERC-20 Migration Checklist

1. Replace 2-param `transfer(to, amount)` with 4-param `transfer(from, to, amount, data)`
2. Replace `require(msg.sender == ...)` with `require(Runtime.checkWitness(...))`
3. Remove `approve()`, `transferFrom()`, `allowance()` — not needed in NEP-17 spec
4. Add `onNEP17Payment(from, amount, data)` callback for receiving tokens
5. Replace `receive()` / `fallback()` with `onNEP17Payment`
6. Use the `Any` type for the `data` parameter
7. Verify manifest shows `supportedstandards: ["NEP-17"]`

## NEP-11: Non-Fungible Tokens

NEP-11 is Neo's non-fungible token standard. Compared to ERC-721, it requires `tokensOf()` and `properties()` methods, uses `bytes32` token IDs, and supports both indivisible and divisible (fractional ownership) NFTs in a single standard.

**Spec:** [NEP-11 Proposal](https://github.com/neo-project/proposals/blob/master/nep-11.mediawiki)

### Required Interface

| Method        | Parameters                                | Return     | Safe | Description                             |
| ------------- | ----------------------------------------- | ---------- | ---- | --------------------------------------- |
| `symbol`      | —                                         | `String`   | Yes  | Token symbol                            |
| `decimals`    | —                                         | `Integer`  | Yes  | `0` for indivisible, `>0` for divisible |
| `totalSupply` | —                                         | `Integer`  | Yes  | Total minted tokens                     |
| `balanceOf`   | `Hash160 owner`                           | `Integer`  | Yes  | Token count for owner                   |
| `ownerOf`     | `ByteArray tokenId`                       | `Hash160`  | Yes  | Token owner                             |
| `transfer`    | `Hash160 to, ByteArray tokenId, Any data` | `Boolean`  | No   | Transfer NFT                            |
| `tokensOf`    | `Hash160 owner`                           | `Iterator` | Yes  | Enumerate owner's tokens                |
| `properties`  | `ByteArray tokenId`                       | `Map`      | Yes  | Token metadata                          |

Required event:

```
Transfer(Hash160 from, Hash160 to, Integer amount, ByteArray tokenId)
```

::: tip NEP-11 Transfer has 4 parameters
Unlike NEP-17's 3-parameter Transfer event, NEP-11 includes the `tokenId` as a fourth parameter. The `amount` field is `1` for indivisible NFTs.
:::

### Divisible vs Indivisible

NEP-11 supports two modes in a single standard:

|                    | Indivisible                   | Divisible                                   |
| ------------------ | ----------------------------- | ------------------------------------------- |
| `decimals()`       | Returns `0`                   | Returns `> 0`                               |
| `ownerOf(tokenId)` | Single `Hash160`              | Iterator of `Hash160` (multiple owners)     |
| `transfer`         | `transfer(to, tokenId, data)` | `transfer(from, to, amount, tokenId, data)` |
| Use case           | Unique collectibles, art      | Fractional real estate, shared assets       |

The devpack provides the `INEP11Divisible` interface for divisible NFTs:

```solidity
interface INEP11Divisible is INEP11 {
    function balanceOf(address owner, bytes32 tokenId) external view returns (uint256);
    function transfer(address from, address to, uint256 amount, bytes32 tokenId, bytes calldata data)
        external returns (bool);
    function ownersOf(bytes32 tokenId) external view returns (address[] memory);
}
```

### Token IDs

|                  | ERC-721 (Ethereum)         | NEP-11 (Neo)                                  |
| ---------------- | -------------------------- | --------------------------------------------- |
| **Type**         | `uint256`                  | `ByteArray` (mapped to `bytes32` in Solidity) |
| **Neo ABI type** | —                          | `Hash256` / `ByteArray`                       |
| **Generation**   | Sequential counter or hash | Sequential counter, hash, or arbitrary bytes  |

```solidity
// ERC-721: uint256 token IDs
function ownerOf(uint256 tokenId) public view returns (address) { ... }

// NEP-11: bytes32 token IDs
function ownerOf(bytes32 tokenId) public view returns (address) { ... }
```

### Implementation Example

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import "standards/NEP11.sol";
import "libraries/Runtime.sol";

/// @title MyNFT — A production NEP-11 non-fungible token
contract MyNFT is NEP11 {
    using Runtime for *;

    constructor()
        NEP11(
            "My NFT Collection",  // name
            "MNFT",               // symbol
            0,                    // decimals (0 = indivisible)
            "https://api.example.com/nft/",  // base URI
            10000,                // max supply
            false                 // not divisible
        )
    {}

    /// @dev Mint a new NFT with metadata properties
    function mintNFT(
        address to,
        bytes32 tokenId,
        bytes memory props
    ) external onlyMinter {
        require(Runtime.checkWitness(msg.sender), "MyNFT: unauthorized");
        mint(to, tokenId, props);
    }

    /// @dev NEP-11 3-parameter transfer with witness authorization
    function transfer(
        address to,
        bytes32 tokenId,
        bytes calldata data
    ) public override returns (bool) {
        address tokenOwner = ownerOf(tokenId);
        require(Runtime.checkWitness(tokenOwner), "MyNFT: unauthorized");
        return super.transfer(to, tokenId, data);
    }

    /// @dev Required: enumerate tokens owned by address
    function tokensOf(address owner) public view override returns (bytes32[] memory) {
        return super.tokensOf(owner);
    }

    /// @dev Required: return token metadata
    function properties(bytes32 tokenId) public view override returns (bytes memory) {
        return super.properties(tokenId);
    }

    /// @dev Callback for receiving NEP-11 tokens
    function onNEP11Payment(
        address from,
        uint256 amount,
        bytes32 tokenId,
        bytes calldata data
    ) external {
        // Handle incoming NFT
    }
}
```

### ERC-721 Migration Checklist

1. Replace `uint256 tokenId` with `bytes32`
2. Replace `transferFrom(from, to, tokenId)` with `transfer(to, tokenId, data)` (3 params)
3. Remove `approve()`, `setApprovalForAll()`, `getApproved()` — use witness model
4. Add required `tokensOf(owner)` method returning token ID array
5. Add required `properties(tokenId)` method returning serialized metadata
6. Add `decimals()` returning `0` for indivisible NFTs
7. Add `onNEP11Payment(from, amount, tokenId, data)` callback for receiving NFTs
8. Verify manifest shows `supportedstandards: ["NEP-11"]`

## NEP-24: Royalty Standard

NEP-24 specifies a `royaltyInfo` method for NEP-11 NFTs so marketplaces can fetch royalty recipients and amounts for secondary sales. Unlike ERC-2981 which returns a single recipient, NEP-24 supports multiple royalty recipients in a single response.

**Spec:** [NEP-24 Proposal](https://github.com/neo-project/proposals/blob/master/nep-24.mediawiki)

### Interface

| Method        | Parameters                                                   | Return                      | Description                    |
| ------------- | ------------------------------------------------------------ | --------------------------- | ------------------------------ |
| `royaltyInfo` | `ByteArray tokenId, Hash160 royaltyToken, Integer salePrice` | `Array<[Hash160, Integer]>` | Royalty recipients and amounts |

```solidity
interface INEP24Royalty {
    struct RoyaltyInfo {
        address royaltyRecipient;
        uint256 royaltyAmount;
    }

    function royaltyInfo(bytes32 tokenId, address royaltyToken, uint256 salePrice)
        external view returns (RoyaltyInfo[] memory);
}
```

### Key Differences from ERC-2981

|                         | ERC-2981 (Ethereum)             | NEP-24 (Neo)                              |
| ----------------------- | ------------------------------- | ----------------------------------------- |
| **Return type**         | Single `(address, uint256)`     | Array of `[recipient, amount]` pairs      |
| **Multiple recipients** | No — single recipient only      | Yes — split royalties natively            |
| **Royalty token**       | Implied (same as sale token)    | Explicit `royaltyToken` parameter         |
| **Basis points**        | `10000 = 100%`                  | `10000 = 100%`                            |
| **Interface detection** | `supportsInterface(0x2a55205a)` | Manifest `supportedstandards: ["NEP-24"]` |

::: info Royalty Token Parameter
The `royaltyToken` parameter specifies which token royalties should be paid in (e.g., GAS, a specific NEP-17 token). This is reserved for future use in the devpack's minimal implementation but enables token-specific royalty rules.
:::

### Implementation Example

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import "standards/NEP11.sol";
import "standards/NEP24.sol";
import "libraries/Runtime.sol";

/// @title RoyaltyNFT — NEP-11 NFT with NEP-24 royalties
contract RoyaltyNFT is NEP11, NEP24Royalty {

    constructor()
        NEP11("Royalty NFT", "RNFT", 0, "", 10000, false)
    {
        // Set default royalty: 5% (500 basis points) to deployer
        _setDefaultRoyalty(msg.sender, 500);
    }

    /// @dev Mint with per-token royalty override
    function mintWithRoyalty(
        address to,
        bytes32 tokenId,
        bytes memory props,
        address royaltyRecipient,
        uint96 royaltyBps
    ) external onlyMinter {
        require(Runtime.checkWitness(msg.sender), "unauthorized");
        mint(to, tokenId, props);

        // Override default royalty for this specific token
        if (royaltyRecipient != address(0) && royaltyBps > 0) {
            _setTokenRoyalty(tokenId, royaltyRecipient, royaltyBps);
        }
    }

    /// @dev Query royalty info — returns array of [recipient, amount] pairs
    /// For a sale of 100 GAS with 5% royalty: returns [(creator, 5 GAS)]
    function royaltyInfo(bytes32 tokenId, address royaltyToken, uint256 salePrice)
        public view override returns (RoyaltyInfo[] memory)
    {
        return super.royaltyInfo(tokenId, royaltyToken, salePrice);
    }
}
```

The `NEP24Royalty` abstract contract provides:

| Internal Method                             | Description                                       |
| ------------------------------------------- | ------------------------------------------------- |
| `_setDefaultRoyalty(recipient, bps)`        | Set fallback royalty for all tokens               |
| `_setTokenRoyalty(tokenId, recipient, bps)` | Override royalty for a specific token             |
| `_clearTokenRoyalty(tokenId)`               | Remove per-token override, fall back to default   |
| `_getRoyaltyRule(tokenId)`                  | Get effective royalty rule (per-token or default) |

::: warning Basis Points Validation
The devpack enforces `bps <= 10000` and `recipient != address(0)`. Setting invalid values reverts with `NEP24InvalidRoyaltyBps` or `NEP24InvalidRoyaltyRecipient`.
:::

## Standards Auto-Detection

The `neo-solc` compiler analyzes your contract's public methods and events to automatically detect which NEP standards it implements. Detection results populate the manifest's `supportedstandards` array.

### NEP-17 Detection

All 5 methods must be present as public/external functions:

```
symbol, decimals, totalSupply, balanceOf, transfer
```

Additional rules:

- `ownerOf` must **not** be present (its presence signals NEP-11 instead)
- A `Transfer` event with 3 parameters is expected
- The `transfer` method should have 4 parameters `(from, to, amount, data)`
- **Near-miss warning**: if 3+ of 5 methods are present but not all, the compiler emits a warning listing the missing methods

### NEP-11 Detection

Core requirement:

- `balanceOf` **and** `ownerOf` must both be present

Plus at least one of:

- `transfer`
- `transferFrom`
- `tokensOf`

Additional checks:

- A `Transfer` event with 4 parameters is expected
- The `transfer` method should have 3 parameters `(to, tokenId, data)`
- **Near-miss warnings**: `ownerOf` without a transfer mechanism, or `ownerOf` + transfer without `balanceOf`

### NEP-24 Detection

Either of these methods triggers detection:

- `tokenUri`
- `royaltyInfo`

### Additional NEP Detection

The compiler also auto-detects these contract-lifecycle and callback standards:

- `NEP-22`: `update(nefFile, manifest, data)`
- `NEP-26`: `onNEP11Payment(from, amount, tokenId, data)`
- `NEP-27`: `onNEP17Payment(from, amount, data)`
- `NEP-29`: `_deploy(data, update)`
- `NEP-30`: `verify(...) -> bool`
- `NEP-31`: `destroy()`

::: tip Checking Detection Results
After compilation, inspect the manifest to verify detected standards:

```bash
neo-solc MyToken.sol -I devpack -O2 -o build/MyToken
cat build/MyToken/MyToken.manifest.json | jq '.supportedstandards'
# Expected: ["NEP-17"]
```

:::

## Interface Detection: EIP-165 vs Manifest

Ethereum and Neo take fundamentally different approaches to interface detection:

|                  | EVM (EIP-165)                                 | Neo (Manifest)                     |
| ---------------- | --------------------------------------------- | ---------------------------------- |
| **Mechanism**    | Runtime query via `supportsInterface(bytes4)` | Static manifest JSON               |
| **Query method** | `staticcall` to contract                      | `ContractManagement.getContract()` |
| **Gas cost**     | Per-query gas cost                            | Free (manifest is metadata)        |
| **Maintenance**  | Manual `supportsInterface()` implementation   | Compiler auto-populates            |

No `supportsInterface()` function is needed on Neo. The compiler automatically populates the `supportedstandards` array based on method signature analysis.

| EIP-165 Selector | ERC Standard | Neo Manifest Entry |
| ---------------- | ------------ | ------------------ |
| `0x80ac58cd`     | ERC-721      | `"NEP-11"`         |
| `0x36372b07`     | ERC-20       | `"NEP-17"`         |
| `0x2a55205a`     | ERC-2981     | `"NEP-24"`         |

::: warning supportsInterface() Is Unnecessary
If your contract includes `supportsInterface(bytes4)`, the compiler emits warning `W106`:

```
warning[W106]: function 'supportsInterface' (EIP-165) is unnecessary on Neo N3.
  Neo uses the manifest 'supportedstandards' array for interface detection,
  which the compiler populates automatically.
  = suggestion: Remove — Neo N3 uses manifest-based interface discovery
```

:::

## Compiler Diagnostics for Standards

The compiler emits actionable diagnostics when it detects Ethereum patterns that need migration. These help catch common mistakes during ERC → NEP porting:

| Code   | Pattern Detected                               | Suggestion                                       |
| ------ | ---------------------------------------------- | ------------------------------------------------ |
| `W101` | `transfer(to, amount)` — 2-param ERC-20        | Add `from` and `data` params for NEP-17          |
| `W102` | `transfer(from, to, amount)` — 3-param partial | Add `data` parameter for NEP-17                  |
| `W103` | `approve`/`allowance`/`transferFrom` present   | Not in NEP-17 spec; keep as extensions or remove |
| `W104` | `transferFrom(from, to, tokenId)` — ERC-721    | Replace with `transfer(to, tokenId, data)`       |
| `W105` | `receive()` / `fallback()` present             | Replace with `onNEP17Payment` callback           |
| `W106` | `supportsInterface(bytes4)` present            | Remove — manifest handles interface detection    |
| `W107` | ERC-1155 multi-token pattern                   | Split into separate NEP-17 and NEP-11 contracts  |
| `W108` | ERC-2612 permit (7-param)                      | Use `Runtime.checkWitness()` instead             |

## Testing Standards Compliance

After compiling, verify your contract meets the target standard:

1. **Check manifest** — confirm `supportedstandards` contains the expected entries:

    ```bash
    cat build/MyToken/MyToken.manifest.json | jq '.supportedstandards'
    ```

2. **Verify method signatures** — ensure all required methods are present with correct parameter counts. The compiler warns on near-misses.

3. **Test transfer with witness** — invoke `transfer` with a valid witness scope and verify `Runtime.checkWitness()` passes:

    ```bash
    neo-express invoke MyToken transfer \
      --account sender -- sender recipient 1000 null
    ```

4. **Test payment callbacks** — transfer tokens to a contract and verify `onNEP17Payment` or `onNEP11Payment` is called.

5. **Integration testing** — use `neo-express` for end-to-end testing on a local private network:
    ```bash
    neo-express create
    neo-express run
    neo-express contract deploy build/MyToken/MyToken.nef
    neo-express invoke MyToken symbol
    ```

::: tip Compiler Warnings Are Your Friend
Address all `W1xx` warnings before deploying. They indicate patterns that may compile but won't produce a standards-compliant manifest or may behave unexpectedly on Neo N3.
:::

## See Also

- [Devpack Overview](/additional-material/neo-devpack) — Full devpack directory layout, libraries, and compiler intrinsics
- [Language Description](/language-description/types) — Solidity/EVM patterns mapped to Neo equivalents
- [Manifest Spec](/internals/contract-metadata) — Neo N3 manifest structure and permissions
- [Native Contracts](/internals/native-contracts) — GAS, NEO, ContractManagement API reference
- [CLI Reference](/compiler/using-the-compiler) — `neo-solc` compiler flags and options
