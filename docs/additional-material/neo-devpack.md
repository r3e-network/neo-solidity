# Devpack Overview

The devpack provides Solidity-facing libraries, standard contracts, and compiler intrinsics for Neo N3 development. It bridges the gap between Solidity syntax and Neo blockchain capabilities, giving developers access to native contracts, syscalls, storage, runtime services, and NEP token standards through familiar Solidity interfaces.

For a reader-focused map from Solidity constructs to syscalls and devpack
wrappers, see [Syscalls and Devpack](/mapping/syscalls-and-devpack).

::: info Compiler Intrinsics
These libraries are primarily compiler intrinsics surfaces — the `neo-solc` compiler recognizes supported members and lowers them directly to NeoVM opcodes and syscalls. Unsupported members produce a compile-time diagnostic listing available intrinsics. You write Solidity; the compiler emits NeoVM bytecode.
:::

## Directory Layout

```
devpack/
├── contracts/               # Syscall/native-call wrappers and framework contracts
│   ├── FrameworkBase.sol    # Base framework with ownership, storage, gas management
│   ├── Framework.sol        # Extended framework (strict-manifest mode)
│   ├── Syscalls.sol         # Complete Neo N3 syscall surface (1,323 lines)
│   ├── NativeCalls.sol      # Native contract wrappers (1,068 lines)
│   ├── OracleService.sol    # Oracle convenience wrapper
│   └── NEP17Rescue.sol      # Emergency native-token recovery for NEP-17 contracts
├── libraries/               # High-level helper libraries
│   ├── Neo.sol              # Blockchain integration (542 lines)
│   ├── Storage.sol          # Advanced storage operations (827 lines)
│   └── Runtime.sol          # Runtime services and utilities (688 lines)
├── standards/               # NEP token standard implementations
│   ├── NEP17.sol            # Fungible token (NEP-17)
│   ├── NEP11.sol            # Non-fungible token (NEP-11, divisible + indivisible)
│   └── NEP24.sol            # Royalty standard (NEP-24)
├── examples/                # Production-oriented usage examples
│   ├── CompleteNEP17Token.sol
│   ├── CompleteNEP11NFT.sol
│   └── VaultPattern.sol
├── DEVPACK_GUIDE.md
├── README.md
└── standards/STANDARDS_MAPPING.md
```

## Usage

### Compiling with the devpack

Pass the devpack root as an include path:

```bash
neo-solc MyContract.sol -I devpack -O2 -o build/MyContract
```

### Importing in Solidity

```solidity
import "contracts/NativeCalls.sol";
import "libraries/Storage.sol";
import "standards/NEP17.sol";
```

All imports resolve relative to the `-I` include path. No package manager or remappings required.

## Core Contracts

### Syscalls.sol

Complete mapping of all Neo N3 syscalls to Solidity functions. This is the lowest-level devpack surface — every other contract and library ultimately delegates to `Syscalls`.

| Category  | Syscalls                                                                    | Examples                                             |
| --------- | --------------------------------------------------------------------------- | ---------------------------------------------------- |
| Storage   | `GetContext`, `Get`, `Put`, `Delete`, `Find`, `AsReadOnly`, `Local.*`       | `storageGet()`, `storagePut()`, `storageFind()`      |
| Runtime   | `CheckWitness`, `GetTime`, `GasLeft`, `Notify`, `Log`, `BurnGas`            | `checkWitness()`, `gasLeft()`, `notify()`            |
| Contract  | `Call`, `GetCallFlags`, `CreateStandardAccount`, `CreateMultisigAccount`    | `contractCall()`, `getCallFlags()`                   |
| Crypto    | `CheckSig`, `CheckMultisig` (via `System.Crypto.*`)                         | `checkSig()`, `checkMultisig()`                      |
| CryptoLib | SHA256, RIPEMD160, Keccak256, VerifyWithECDsa, Ed25519, BLS12-381, Murmur32 | `sha256()`, `verifyWithECDsa()`, `bls12381Pairing()` |
| StdLib    | serialize, deserialize, JSON, base64/58, hex, string ops, itoa/atoi         | `jsonSerialize()`, `base64Encode()`, `stringSplit()` |
| Iterator  | `Next`, `Value`                                                             | `iteratorNext()`, `iteratorValue()`                  |
| Advanced  | `GetRandom`, `GetNetwork`, `GetAddressVersion`, `GetInvocationCounter`      | `getCurrentRandom()`, `getNetwork()`                 |

Syscalls.sol also defines the core data structures used throughout the devpack:

| Struct                | Description                                                                                                           |
| --------------------- | --------------------------------------------------------------------------------------------------------------------- |
| `Block`               | Trimmed block: hash, version, previousHash, merkleRoot, timestamp, nonce, index, primaryIndex, nextConsensus, txCount |
| `Transaction`         | Base transaction: hash, version, nonce, sender, systemFee, networkFee, validUntilBlock, script                        |
| `Witness`             | invocationScript + verificationScript                                                                                 |
| `Signer`              | account, scopes, allowedContracts, allowedGroups, rules                                                               |
| `WitnessRule`         | action (deny/allow) + condition bytes                                                                                 |
| `StorageContext`      | Contract storage context (id + isReadOnly)                                                                            |
| `Iterator`            | NeoVM iterator handle (id, hasNext, currentKey, currentValue)                                                         |
| `Notification`        | scriptHash, eventName, state array                                                                                    |
| `ContractStateNative` | id, updateCounter, hash, nef, manifest                                                                                |

::: tip
Prefer `NativeCalls` wrappers over raw `Syscalls.contractCall()` — they generate specific manifest permissions instead of wildcards.
:::

### NativeCalls.sol

Direct wrappers for all Neo N3 native contracts. Each function calls through `Syscalls.contractCall()` with a deterministic contract hash constant, enabling the compiler to infer exact manifest permissions.

| Native Contract    | Hash Constant         | Key Functions                                                                                         |
| ------------------ | --------------------- | ----------------------------------------------------------------------------------------------------- |
| NEO Token          | `NEO_CONTRACT`        | `neoBalanceOf()`, `neoTransfer()`, `vote()`, `getCandidates()`, `getAccountState()`, `unclaimedGas()` |
| GAS Token          | `GAS_CONTRACT`        | `gasBalanceOf()`, `gasTransfer()`, `gasTotalSupply()`                                                 |
| ContractManagement | `CONTRACT_MANAGEMENT` | `deployContract()`, `updateContract()`, `destroyContract()`, `getContract()`, `hasMethod()`           |
| Policy             | `POLICY_CONTRACT`     | `getFeePerByte()`, `getExecFeeFactor()`, `getStoragePrice()`, `blockAccount()`, `isBlocked()`         |
| Oracle             | `ORACLE_CONTRACT`     | `requestOracleData()`, `getOraclePrice()`, `setOraclePrice()`                                         |
| RoleManagement     | `ROLE_MANAGEMENT`     | `designateAsRole()`, `getDesignatedByRole()`                                                          |
| Notary             | `NOTARY_CONTRACT`     | `notaryVerify()`, `notaryBalanceOf()`, `notaryLockDepositUntil()`, `notaryWithdraw()`                 |
| Treasury           | `TREASURY_CONTRACT`   | `treasuryVerify()`, `treasuryOnNEP17Payment()`, `treasuryOnNEP11Payment()`                            |
| Ledger             | `LEDGER_CONTRACT`     | `currentIndex()`, `currentHash()`, `getBlock()`, `getTransaction()`                                   |

Reference constants for CryptoLib (`CRYPTO_LIB`) and StdLib (`STD_LIB`) are also defined, but their methods are exposed through `Syscalls.sol` directly (e.g., `Syscalls.sha256()`, `Syscalls.base64Encode()`).

```solidity
// Transfer GAS — generates permission: {"contract":"0xd2a4...","methods":["transfer"]}
bool ok = NativeCalls.gasTransfer(from, to, amount, "");

// Query NEO governance
NativeCalls.NeoCandidate[] memory candidates = NativeCalls.getCandidates();
uint256 unclaimed = NativeCalls.unclaimedGas(account, NativeCalls.currentIndex());
```

### FrameworkBase.sol

Base contract providing common infrastructure for Neo N3 Solidity contracts:

- **Ownership** — `transferOwnership()`, `renounceOwnership()`, `onlyOwner` modifier
- **Initialization** — `whenInitialized` modifier, version tracking
- **Witness-based access control** — `withWitness` modifier (calls `Runtime.checkWitness`)
- **Gas limit guards** — `withGasLimit(minGas)` modifier (calls `Runtime.gasLeft`)
- **Contract lifecycle** — `upgradeContract()` with NEF + manifest update
- **Events** — `FrameworkInitialized`, `OwnershipTransferred`, `ContractUpgraded`, `EmergencyStop`

```solidity
contract MyToken is FrameworkBase {
    function adminAction() public onlyOwner withWitness withGasLimit(20000000) {
        // Only owner, witness-verified, with at least 0.2 GAS remaining
    }
}
```

### Framework.sol

Extended framework that inherits `FrameworkBase`. Previously exposed a fully dynamic `callContract(address, string, bytes)` surface, but this forced wildcard permissions in the manifest (`{"contract":"*","methods":"*"}`).

::: warning Strict-Manifest Mode
`Framework.callContract()` is intentionally disabled — it reverts at runtime. Use explicit `NativeCalls.*` or `Syscalls.contractCall(KNOWN_HASH, ...)` wrappers instead. If truly dynamic dispatch is unavoidable, compile with `--manifest-permissions` to supply explicit overrides.
:::

### OracleService.sol

Convenience wrapper around the Neo N3 Oracle native contract:

- `request(url, filter, userData, gasForResponse)` — issue an oracle data request, returns a `requestId`
- `oracleCallback(url, userData, code, result)` — receives the native Oracle callback (restricted to `onlyOracleNative`)
- Forwards responses to the original requester via `IOracleServiceReceiver.onOracleResponse()`
- Uses a fixed callback method name to avoid wildcard manifest permissions
- Tracks request metadata (requester, URL, filter, timestamps, completion status)

```solidity
contract MyOracle is IOracleServiceReceiver {
    OracleService private _oracle;

    function fetchPrice(string calldata url) external {
        _oracle.request(url, "$.price", "", 100000000);
    }

    function onOracleResponse(
        uint256 requestId, uint256 code, bytes calldata result, bytes calldata userData
    ) external override {
        // Handle oracle response
    }
}
```

### NEP17Rescue.sol

Optional extension for NEP-17 tokens that enables emergency recovery of accidentally sent native tokens (NEO and GAS only). In strict-manifest mode, rescuing arbitrary NEP-17 tokens is not supported because it would require wildcard contract permissions.

```solidity
contract MyToken is NEP17Rescue {
    // Inherits emergencyTokenRecovery(token, to, amount, data)
    // Restricted to NEO_CONTRACT and GAS_CONTRACT targets
}
```

## Libraries

### Neo.sol (542 lines)

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

### Storage.sol (827 lines)

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

### Runtime.sol (688 lines)

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

## Token Standards

### NEP17.sol — Fungible Tokens

Complete NEP-17 implementation with ERC-20 compatibility:

```solidity
interface INEP17 {
    function symbol() external view returns (string memory);
    function decimals() external view returns (uint8);
    function totalSupply() external view returns (uint256);
    function balanceOf(address account) external view returns (uint256);
    function transfer(address from, address to, uint256 amount, Any calldata data) external returns (bool);
}
```

Key features:

- `onNEP17Payment` callback support via `INEP17Receiver`
- `Any` type for the `data` parameter (maps to NeoVM's unconstrained StackItem)
- Mint, burn, pause, and allowance management
- Inherits `FrameworkBase` for ownership and witness control

### NEP11.sol — Non-Fungible Tokens

Complete NEP-11 implementation supporting both indivisible and divisible NFTs:

```solidity
interface INEP11 {
    function symbol() external view returns (string memory);
    function decimals() external view returns (uint8);
    function totalSupply() external view returns (uint256);
    function balanceOf(address owner) external view returns (uint256);
    function tokensOf(address owner) external view returns (bytes32[] memory);
    function ownerOf(bytes32 tokenId) external view returns (address);
    function transfer(address to, bytes32 tokenId, bytes calldata data) external returns (bool);
    function properties(bytes32 tokenId) external view returns (bytes memory);
}

interface INEP11Divisible is INEP11 {
    function balanceOf(address owner, bytes32 tokenId) external view returns (uint256);
    function transfer(address from, address to, uint256 amount, bytes32 tokenId, bytes calldata data) external returns (bool);
    function ownersOf(bytes32 tokenId) external view returns (address[] memory);
}
```

### NEP24.sol — Royalty Standard

Minimal NEP-24 royalty mixin for NEP-11 NFTs:

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

- Per-token and default royalty rules (basis points: 10000 = 100%)
- `_setDefaultRoyalty()`, `_setTokenRoyalty()`, `_clearTokenRoyalty()`
- Returns empty array when no royalty is configured

## Compiler Intrinsics

The following devpack surfaces are recognized as compiler intrinsics by `neo-solc`:

| Intrinsic Surface                      | Lowered To                                |
| -------------------------------------- | ----------------------------------------- |
| `Runtime.checkWitness()`               | `System.Runtime.CheckWitness`             |
| `Runtime.gasLeft()`                    | `System.Runtime.GasLeft`                  |
| `Runtime.notify()`                     | `System.Runtime.Notify`                   |
| `Storage.put()` / `get()` / `remove()` | `System.Storage.Put` / `Get` / `Delete`   |
| `Storage.find()`                       | `System.Storage.Find`                     |
| `Syscalls.sha256()`                    | `CryptoLib.sha256` (native call)          |
| `Syscalls.verifyWithECDsa()`           | `CryptoLib.verifyWithECDsa` (native call) |
| `Syscalls.neoKeccak256()`              | `CryptoLib.keccak256` (native call)       |
| `Syscalls.contractCall()`              | `System.Contract.Call`                    |
| `NativeCalls.gasTransfer()`            | `System.Contract.Call` → GAS `transfer`   |
| `abi.encode()` / `abi.decode()`        | NeoVM stack manipulation                  |
| Solidity `keccak256()`                 | `CryptoLib.keccak256` (native call)       |

When you call a member that the compiler does not recognize as an intrinsic, compilation fails with a diagnostic:

```
error: unsupported intrinsic 'Runtime.unsupportedMethod'
  --> MyContract.sol:12:5
   |
12 |     Runtime.unsupportedMethod();
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: available Runtime intrinsics: checkWitness, gasLeft, notify, log, ...
```

## Permission-Conscious Development

Neo N3 manifests declare which contracts and methods your contract is allowed to call. The compiler infers these permissions from your code. Using fixed-target wrappers produces minimal permissions:

```
NativeCalls.gasTransfer(...)
  → manifest permission: {"contract":"0xd2a4cff3...","methods":["transfer"]}

Syscalls.contractCall(dynamicTarget, dynamicMethod, ...)
  → manifest permission: {"contract":"*","methods":"*"}
```

::: warning Wildcard Permissions
Wildcard permissions (`"*"`) are a security anti-pattern. They allow your contract to call any contract and any method, which increases the attack surface. Always prefer fixed-target wrappers.
:::

Compile with strict flags to reject wildcard permissions at build time:

```bash
neo-solc MyContract.sol -I devpack \
  --deny-wildcard-contracts \
  --deny-wildcard-methods \
  -o build/MyContract
```

If truly dynamic dispatch is unavoidable, supply explicit permission overrides:

```bash
neo-solc MyContract.sol -I devpack \
  --manifest-permissions '{"contract":"0xabcd...","methods":["specificMethod"]}' \
  -o build/MyContract
```

## Building Custom Contracts

Minimal contract using devpack features:

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import "contracts/NativeCalls.sol";
import "libraries/Storage.sol";
import "libraries/Runtime.sol";
import "contracts/FrameworkBase.sol";

contract Vault is FrameworkBase {
    event Deposit(address indexed from, uint256 amount);
    event Withdraw(address indexed to, uint256 amount);

    function deposit() external withWitness {
        uint256 balance = NativeCalls.gasBalanceOf(address(this));
        Storage.put("balance", abi.encode(balance));
        emit Deposit(msg.sender, balance);
    }

    function withdraw(address to, uint256 amount) external onlyOwner withWitness {
        Runtime.requireWitness(to);
        bool ok = NativeCalls.gasTransfer(address(this), to, amount, "");
        require(ok, "Vault: transfer failed");

        uint256 remaining = NativeCalls.gasBalanceOf(address(this));
        Storage.put("balance", abi.encode(remaining));
        emit Withdraw(to, amount);
    }

    function getBalance() external view returns (uint256) {
        bytes memory data = Storage.get("balance");
        if (data.length == 0) return 0;
        return abi.decode(data, (uint256));
    }
}
```

Compile and deploy:

```bash
# Compile with strict permissions
neo-solc Vault.sol -I devpack -O2 \
  --deny-wildcard-contracts \
  --deny-wildcard-methods \
  -o build/Vault

# Inspect generated manifest permissions
cat build/Vault/Vault.manifest.json | jq '.permissions'

# Deploy to testnet
neo-solc deploy build/Vault --network testnet
```

## Native Contract Hash Reference

All native contract hashes are deterministic and identical across all Neo N3 networks:

| Contract           | Hash                                         | Devpack Constant                  |
| ------------------ | -------------------------------------------- | --------------------------------- |
| NeoToken           | `0xef4073a0f2b305a38ec4050e4d3d28bc40ea63f5` | `NativeCalls.NEO_CONTRACT`        |
| GasToken           | `0xd2a4cff31913016155e38e474a2c06d08be276cf` | `NativeCalls.GAS_CONTRACT`        |
| ContractManagement | `0xfffdc93764dbaddd97c48f252a53ea4643faa3fd` | `NativeCalls.CONTRACT_MANAGEMENT` |
| PolicyContract     | `0xcc5e4edd9f5f8dba8bb65734541df7a1c081c67b` | `NativeCalls.POLICY_CONTRACT`     |
| OracleContract     | `0xfe924b7cfe89ddd271abaf7210a80a7e11178758` | `NativeCalls.ORACLE_CONTRACT`     |
| RoleManagement     | `0x49cf4e5378ffcd4dec034fd98a174c5491e395e2` | `NativeCalls.ROLE_MANAGEMENT`     |
| Notary             | `0xc1e14f19c3e60d0b9244d06dd7ba9b113135ec3b` | `NativeCalls.NOTARY_CONTRACT`     |
| Treasury           | `0x156326f25b1b5d839a4d326aeaa75383c9563ac1` | `NativeCalls.TREASURY_CONTRACT`   |
| LedgerContract     | `0xda65b600f7124ce6c79950c1772a36403104f2be` | `NativeCalls.LEDGER_CONTRACT`     |
| CryptoLib          | `0x726cb6e0cd8628a1350a611384688911ab75f51b` | `NativeCalls.CRYPTO_LIB`          |
| StdLib             | `0xacce6fd80d44e1796aa0c2c625e9e4e0ce39efc0` | `NativeCalls.STD_LIB`             |

## See Also

- [Standards and Contracts](/additional-material/neo-standards) — NEP-17, NEP-11, NEP-24 deep dive
- [Syscalls Reference](/internals/syscalls) — Complete syscall listing
- [Native Contracts](/internals/native-contracts) — Native contract API reference
- [CLI Reference](/compiler/using-the-compiler) — `neo-solc` compiler flags and options
- [Language Description](/language-description/types) — Solidity/EVM patterns mapped to Neo equivalents
