# Native Contracts

Neo N3 ships eleven native contracts that are deployed at genesis and provide core platform functionality — token management, governance, cryptography, blockchain queries, and more. Unlike user-deployed contracts, native contracts have deterministic script hashes that are identical across all Neo N3 networks (mainnet, testnet, private chains). The `neo-solidity` compiler and devpack expose these contracts through the `NativeCalls` library and `Syscalls` wrappers, lowering calls to either `System.Contract.Call` syscalls or optimized `CALLT` method token instructions.

---

## Overview

| Name               | Script Hash                                  | Purpose                                     | Devpack Wrapper                                                     |
| ------------------ | -------------------------------------------- | ------------------------------------------- | ------------------------------------------------------------------- |
| NEO                | `0xef4073a0f2b305a38ec4050e4d3d28bc40ea63f5` | Governance token, voting, validators        | `NativeCalls.neo*`                                                  |
| GAS                | `0xd2a4cff31913016155e38e474a2c06d08be276cf` | Utility/fee token                           | `NativeCalls.gas*`                                                  |
| ContractManagement | `0xfffdc93764dbaddd97c48f252a53ea4643faa3fd` | Deploy, update, destroy contracts           | `NativeCalls.deployContract` / `updateContract` / `destroyContract` |
| Policy             | `0xcc5e4edd9f5f8dba8bb65734541df7a1c081c67b` | Network policy parameters                   | `NativeCalls.getFeePerByte` / `getExecFeeFactor` / etc.             |
| Oracle             | `0xfe924b7cfe89ddd271abaf7210a80a7e11178758` | Off-chain data requests                     | `NativeCalls.requestOracleData` / `OracleService.sol`               |
| RoleManagement     | `0x49cf4e5378ffcd4dec034fd98a174c5491e395e2` | Node role designation                       | `NativeCalls.designateAsRole` / `getDesignatedByRole`               |
| Ledger             | `0xda65b600f7124ce6c79950c1772a36403104f2be` | Block and transaction queries               | `NativeCalls.currentIndex` / `getBlock` / `getTransaction`          |
| CryptoLib          | `0x726cb6e0cd8628a1350a611384688911ab75f51b` | Cryptographic operations                    | `Syscalls.sha256` / `neoKeccak256` / `verifyWithECDsa`              |
| StdLib             | `0xacce6fd80d44e1796aa0c2c625e9e4e0ce39efc0` | Serialization and encoding utilities        | `Syscalls.serialize` / `base64Encode` / `itoa` / etc.               |
| Notary             | `0xc1e14f19c3e60d0b9244d06dd7ba9b113135ec3b` | Notary service for multi-party transactions | `NativeCalls.notary*`                                               |
| Treasury           | `0x156326f25b1b5d839a4d326aeaa75383c9563ac1` | Treasury management                         | `NativeCalls.treasury*`                                             |

::: info
Script hashes shown above are big-endian hex (the format used by Neo RPC and block explorers). The devpack constants in `NativeCalls.sol` use the same format as Solidity `address` literals.
:::

---

## NEO Token Contract

The NEO token is the governance token of the Neo N3 network. It is indivisible (0 decimals), has a fixed supply of 100,000,000, and is used for voting, validator election, and GAS generation.

### Methods

| Method                   | Signature                                 | Return           | Safe | Description                                                   |
| ------------------------ | ----------------------------------------- | ---------------- | :--: | ------------------------------------------------------------- |
| `name`                   | `name()`                                  | `string`         |  ✅  | Returns `"NEO"`.                                              |
| `symbol`                 | `symbol()`                                | `string`         |  ✅  | Returns `"NEO"`.                                              |
| `decimals`               | `decimals()`                              | `uint8`          |  ✅  | Returns `0` (indivisible).                                    |
| `totalSupply`            | `totalSupply()`                           | `uint256`        |  ✅  | Returns `100000000`.                                          |
| `balanceOf`              | `balanceOf(address)`                      | `uint256`        |  ✅  | NEO balance of account.                                       |
| `transfer`               | `transfer(address,address,uint256,bytes)` | `bool`           |  ❌  | Transfer NEO tokens. Requires witness of `from`.              |
| `vote`                   | `vote(address,bytes)`                     | `bool`           |  ❌  | Vote for a validator candidate. Pass `null` pubkey to cancel. |
| `getCandidates`          | `getCandidates()`                         | `NeoCandidate[]` |  ✅  | First 256 registered candidates with vote counts.             |
| `getAllCandidates`       | `getAllCandidates()`                      | `Iterator`       |  ✅  | Iterator over all registered candidates.                      |
| `getCandidateVote`       | `getCandidateVote(bytes)`                 | `int256`         |  ✅  | Vote count for a public key. Returns `-1` if not found.       |
| `registerCandidate`      | `registerCandidate(bytes)`                | `bool`           |  ❌  | Register a public key as validator candidate.                 |
| `unregisterCandidate`    | `unregisterCandidate(bytes)`              | `bool`           |  ❌  | Remove candidate registration.                                |
| `getCommittee`           | `getCommittee()`                          | `bytes[]`        |  ✅  | Current committee member public keys.                         |
| `getNextBlockValidators` | `getNextBlockValidators()`                | `address[]`      |  ✅  | Validators for the next block.                                |
| `getGasPerBlock`         | `getGasPerBlock()`                        | `uint256`        |  ✅  | GAS generated per block.                                      |
| `setGasPerBlock`         | `setGasPerBlock(uint256)`                 | `void`           |  ❌  | Set GAS per block (committee only).                           |
| `getRegisterPrice`       | `getRegisterPrice()`                      | `uint256`        |  ✅  | GAS cost to register as candidate.                            |
| `setRegisterPrice`       | `setRegisterPrice(uint256)`               | `void`           |  ❌  | Set registration price (committee only).                      |
| `getAccountState`        | `getAccountState(address)`                | `AccountState`   |  ✅  | Balance, vote target, and last GAS claim height.              |
| `unclaimedGas`           | `unclaimedGas(address,uint256)`           | `uint256`        |  ✅  | Unclaimed GAS for account at block height.                    |

### Code Example

```solidity
import "devpack/contracts/NativeCalls.sol";

contract Governance {
    function getMyNeoBalance() public view returns (uint256) {
        return NativeCalls.neoBalanceOf(address(this));
    }

    function voteForCandidate(bytes memory publicKey) public {
        require(Runtime.checkWitness(msg.sender), "unauthorized");
        NativeCalls.vote(msg.sender, publicKey);
    }

    function claimableGas(address account) public view returns (uint256) {
        uint256 height = NativeCalls.currentIndex();
        return NativeCalls.unclaimedGas(account, height);
    }
}
```

---

## GAS Token Contract

GAS is the utility token used to pay transaction fees on Neo N3. It has 8 decimals and is generated by holding NEO.

### Methods

| Method        | Signature                                 | Return    | Safe | Description                               |
| ------------- | ----------------------------------------- | --------- | :--: | ----------------------------------------- |
| `name`        | `name()`                                  | `string`  |  ✅  | Returns `"GAS"`.                          |
| `symbol`      | `symbol()`                                | `string`  |  ✅  | Returns `"GAS"`.                          |
| `decimals`    | `decimals()`                              | `uint8`   |  ✅  | Returns `8`.                              |
| `totalSupply` | `totalSupply()`                           | `uint256` |  ✅  | Current GAS supply (increases over time). |
| `balanceOf`   | `balanceOf(address)`                      | `uint256` |  ✅  | GAS balance of account (in 10^-8 units).  |
| `transfer`    | `transfer(address,address,uint256,bytes)` | `bool`    |  ❌  | Transfer GAS. Requires witness of `from`. |

### Code Example

```solidity
import "devpack/contracts/NativeCalls.sol";

contract Treasury {
    function getGasBalance() public view returns (uint256) {
        return NativeCalls.gasBalanceOf(address(this));
    }

    function withdrawGas(address to, uint256 amount) public {
        require(Runtime.checkWitness(msg.sender), "unauthorized");
        bool success = NativeCalls.gasTransfer(address(this), to, amount, "");
        require(success, "transfer failed");
    }
}
```

::: tip
GAS uses 10^8 decimals, not 10^18 like Ether. When porting EVM contracts, adjust your unit constants: `0.1 GAS = 10_000_000` (not `10^17`).
:::

---

## ContractManagement

Manages the lifecycle of all deployed contracts on Neo N3 — deployment, upgrades, destruction, and introspection.

### Methods

| Method                    | Signature                          | Return          | Safe | Description                                               |
| ------------------------- | ---------------------------------- | --------------- | :--: | --------------------------------------------------------- |
| `deploy`                  | `deploy(bytes,bytes)`              | `ContractState` |  ❌  | Deploy a new contract from NEF + manifest.                |
| `deploy`                  | `deploy(bytes,bytes,bytes)`        | `ContractState` |  ❌  | Deploy with initialization data passed to `_deploy`.      |
| `update`                  | `update(bytes,bytes)`              | `void`          |  ❌  | Update calling contract's NEF and/or manifest.            |
| `update`                  | `update(bytes,bytes,bytes)`        | `void`          |  ❌  | Update with migration data passed to `_deploy`.           |
| `destroy`                 | `destroy()`                        | `void`          |  ❌  | Permanently destroy the calling contract and its storage. |
| `getContract`             | `getContract(address)`             | `ContractState` |  ✅  | Get contract state by script hash.                        |
| `getContractById`         | `getContractById(int256)`          | `ContractState` |  ✅  | Get contract state by numeric ID.                         |
| `getMinimumDeploymentFee` | `getMinimumDeploymentFee()`        | `uint256`       |  ✅  | Minimum GAS required to deploy a contract.                |
| `setMinimumDeploymentFee` | `setMinimumDeploymentFee(uint256)` | `void`          |  ❌  | Set minimum deployment fee (committee only).              |
| `hasMethod`               | `hasMethod(address,string,uint8)`  | `bool`          |  ✅  | Check if a contract exposes a specific method.            |
| `listContracts`           | `listContracts()`                  | `Iterator`      |  ✅  | Iterator over all deployed contracts.                     |

### Code Example

```solidity
import "devpack/contracts/NativeCalls.sol";

contract Upgradeable {
    address private _owner;

    constructor() {
        _owner = msg.sender;
    }

    /// @dev Upgrade this contract in-place
    function upgrade(bytes memory newNef, bytes memory newManifest) public {
        require(Runtime.checkWitness(_owner), "not owner");
        NativeCalls.updateContract(newNef, newManifest);
        // After update, the new code executes immediately
    }

    /// @dev Permanently destroy this contract
    function kill() public {
        require(Runtime.checkWitness(_owner), "not owner");
        NativeCalls.destroyContract();
    }

    /// @dev Check if another contract exists
    function contractExists(address target) public view returns (bool) {
        return NativeCalls.hasMethod(target, "name", 0);
    }
}
```

::: warning
`destroy()` is permanent and irreversible. All contract storage is deleted. There is no refund mechanism like EVM's deprecated `selfdestruct`. Always gate destruction behind strict authorization.
:::

---

## Policy Contract

Controls network-wide policy parameters. Most setter methods require committee multi-signature authorization.

### Methods

| Method             | Signature                  | Return    | Safe | Description                                          |
| ------------------ | -------------------------- | --------- | :--: | ---------------------------------------------------- |
| `getFeePerByte`    | `getFeePerByte()`          | `uint256` |  ✅  | Network fee per transaction byte (in GAS fractions). |
| `setFeePerByte`    | `setFeePerByte(uint256)`   | `void`    |  ❌  | Set fee per byte (committee only).                   |
| `getExecFeeFactor` | `getExecFeeFactor()`       | `uint32`  |  ✅  | Execution fee multiplier for opcode costs.           |
| `setExecFeeFactor` | `setExecFeeFactor(uint32)` | `void`    |  ❌  | Set execution fee factor (committee only).           |
| `getStoragePrice`  | `getStoragePrice()`        | `uint256` |  ✅  | GAS cost per byte of contract storage.               |
| `setStoragePrice`  | `setStoragePrice(uint256)` | `void`    |  ❌  | Set storage price (committee only).                  |
| `isBlocked`        | `isBlocked(address)`       | `bool`    |  ✅  | Check if an account is blocked from transacting.     |
| `blockAccount`     | `blockAccount(address)`    | `void`    |  ❌  | Block an account (committee only).                   |
| `unblockAccount`   | `unblockAccount(address)`  | `void`    |  ❌  | Unblock an account (committee only).                 |

### Code Example

```solidity
import "devpack/contracts/NativeCalls.sol";

contract FeeEstimator {
    function estimateStorageCost(uint256 bytesCount) public view returns (uint256) {
        uint256 pricePerByte = NativeCalls.getStoragePrice();
        return pricePerByte * bytesCount;
    }

    function getNetworkFeeParams() public view returns (uint256 feePerByte, uint32 execFactor) {
        feePerByte = NativeCalls.getFeePerByte();
        execFactor = NativeCalls.getExecFeeFactor();
    }

    function requireNotBlocked(address account) internal view {
        require(!NativeCalls.isBlocked(account), "account is blocked");
    }
}
```

---

## Oracle Contract

Provides off-chain data access through a request/callback pattern. Oracle nodes fetch external data and deliver results back to the requesting contract.

### Methods

| Method     | Signature                                     | Return    | Safe | Description                                      |
| ---------- | --------------------------------------------- | --------- | :--: | ------------------------------------------------ |
| `request`  | `request(string,string,string,bytes,uint256)` | `void`    |  ❌  | Submit an oracle data request.                   |
| `getPrice` | `getPrice()`                                  | `uint256` |  ✅  | Base GAS cost per oracle request.                |
| `setPrice` | `setPrice(uint256)`                           | `void`    |  ❌  | Set oracle price (committee only).               |
| `finish`   | `finish()`                                    | `void`    |  ❌  | Complete an oracle response (oracle nodes only). |
| `verify`   | `verify()`                                    | `bool`    |  ✅  | Verify an oracle response transaction.           |

### Callback Pattern

The Oracle native contract uses an asynchronous request/callback model:

1. Your contract calls `Oracle.request(url, filter, callbackMethod, userData, gasForResponse)`.
2. Oracle nodes fetch the URL, apply the JSONPath `filter`, and invoke `callbackMethod` on your contract.
3. The callback receives `(string url, bytes userData, int code, bytes result)`.

```solidity
import "devpack/contracts/NativeCalls.sol";

contract PriceOracle {
    uint256 public lastPrice;

    function requestPrice() public {
        NativeCalls.requestOracleData(
            "https://api.example.com/price",  // URL
            "$.neo.usd",                       // JSONPath filter
            "onPriceResponse",                 // callback method name
            "",                                // user data
            100_000_000                        // 1 GAS for response
        );
    }

    /// @dev Called by the Oracle native contract
    function onPriceResponse(
        string calldata url,
        bytes calldata userData,
        uint256 code,
        bytes calldata result
    ) external {
        require(msg.sender == NativeCalls.ORACLE_CONTRACT, "unauthorized");
        if (code == 0) { // Success
            lastPrice = abi.decode(result, (uint256));
        }
    }
}
```

::: tip
The devpack includes `OracleService.sol` — a convenience wrapper that manages request IDs, stores responses, and forwards callbacks through a fixed `onOracleResponse` method name. This avoids wildcard manifest permissions that a dynamic callback name would require.
:::

::: warning
Oracle requests are not free. Each request costs at least `Oracle.getPrice()` GAS plus the `gasForResponse` budget you specify. The callback method name becomes a manifest permission entry — use a fixed name to avoid wildcard permissions.
:::

---

## RoleManagement

Manages designated node roles in the Neo N3 network. Used by committee members to assign oracle nodes, state validators, and other infrastructure roles.

### Methods

| Method                | Signature                             | Return    | Safe | Description                                             |
| --------------------- | ------------------------------------- | --------- | :--: | ------------------------------------------------------- |
| `designateAsRole`     | `designateAsRole(bytes1,bytes[])`     | `void`    |  ❌  | Assign public keys to a role (committee only).          |
| `getDesignatedByRole` | `getDesignatedByRole(bytes1,uint256)` | `bytes[]` |  ✅  | Get public keys designated for a role at a block index. |

### Role Types

| Value  | Role              | Description                  |
| :----: | ----------------- | ---------------------------- |
| `0x04` | StateValidator    | State root validation nodes. |
| `0x08` | Oracle            | Oracle service nodes.        |
| `0x10` | NeoFSAlphabetNode | NeoFS alphabet nodes.        |
| `0x20` | P2PNotary         | P2P notary service nodes.    |

---

## Ledger Contract

Provides read-only access to blockchain data — blocks, transactions, and their metadata.

### Methods

| Method                    | Signature                                  | Return        | Safe | Description                                                         |
| ------------------------- | ------------------------------------------ | ------------- | :--: | ------------------------------------------------------------------- |
| `currentIndex`            | `currentIndex()`                           | `uint256`     |  ✅  | Current block height. Maps from `block.number`.                     |
| `currentHash`             | `currentHash()`                            | `bytes32`     |  ✅  | Hash of the current block.                                          |
| `getBlock`                | `getBlock(uint256)`                        | `Block`       |  ✅  | Get block by index.                                                 |
| `getBlock`                | `getBlock(bytes32)`                        | `Block`       |  ✅  | Get block by hash.                                                  |
| `getTransaction`          | `getTransaction(bytes32)`                  | `Transaction` |  ✅  | Get transaction by hash.                                            |
| `getTransactionHeight`    | `getTransactionHeight(bytes32)`            | `int256`      |  ✅  | Block height containing the transaction. Returns `-1` if not found. |
| `getTransactionFromBlock` | `getTransactionFromBlock(uint256,uint256)` | `Transaction` |  ✅  | Get transaction by block index and tx index.                        |
| `getTransactionSigners`   | `getTransactionSigners(bytes32)`           | `Signer[]`    |  ✅  | Get signers of a transaction.                                       |
| `getTransactionVMState`   | `getTransactionVMState(bytes32)`           | `uint8`       |  ✅  | VM execution state of a transaction (`HALT`, `FAULT`, etc.).        |

::: info
The compiler auto-maps `block.number` to `Ledger.currentIndex()` and `blockhash(n)` to `Ledger.getBlock(n).hash`. See [Units and Globally Available Variables](/language-description/units-and-global-variables) for the full list of auto-mapped block context values.
:::

---

## CryptoLib

Provides cryptographic hash functions, signature verification, and BLS12-381 curve operations. CryptoLib methods are exposed through `Syscalls.sol` rather than `NativeCalls.sol`.

### Methods

| Method                | Signature                                    | Return    | Safe | Description                                     |
| --------------------- | -------------------------------------------- | --------- | :--: | ----------------------------------------------- |
| `sha256`              | `sha256(bytes)`                              | `bytes32` |  ✅  | SHA-256 hash.                                   |
| `ripemd160`           | `ripemd160(bytes)`                           | `bytes20` |  ✅  | RIPEMD-160 hash.                                |
| `keccak256`           | `keccak256(bytes)`                           | `bytes32` |  ✅  | Keccak-256 hash (added at Cockatrice hardfork). |
| `murmur32`            | `murmur32(bytes,uint32)`                     | `bytes4`  |  ✅  | Murmur3 32-bit hash with seed.                  |
| `verifyWithECDsa`     | `verifyWithECDsa(bytes32,bytes,bytes,uint8)` | `bool`    |  ✅  | ECDSA signature verification.                   |
| `bls12381Serialize`   | `bls12381Serialize(bytes)`                   | `bytes`   |  ✅  | Serialize a BLS12-381 point.                    |
| `bls12381Deserialize` | `bls12381Deserialize(bytes)`                 | `bytes`   |  ✅  | Deserialize a BLS12-381 point.                  |
| `bls12381Equal`       | `bls12381Equal(bytes,bytes)`                 | `bool`    |  ✅  | Compare two BLS12-381 points.                   |
| `bls12381Add`         | `bls12381Add(bytes,bytes)`                   | `bytes`   |  ✅  | Add two BLS12-381 points.                       |
| `bls12381Mul`         | `bls12381Mul(bytes,bytes,bool)`              | `bytes`   |  ✅  | Multiply BLS12-381 point by scalar.             |
| `bls12381Pairing`     | `bls12381Pairing(bytes,bytes)`               | `bytes`   |  ✅  | BLS12-381 pairing check.                        |

### Curve Support

The `verifyWithECDsa` method accepts a `curve` parameter that selects both the elliptic curve and the hash algorithm:

| Value | Constant              | Curve     | Hash       | Use Case                       |
| :---: | --------------------- | --------- | ---------- | ------------------------------ |
| `22`  | `SECP256K1_SHA256`    | secp256k1 | SHA-256    | Bitcoin-compatible signatures  |
| `23`  | `SECP256R1_SHA256`    | secp256r1 | SHA-256    | Neo-native signatures          |
| `122` | `SECP256K1_KECCAK256` | secp256k1 | Keccak-256 | Ethereum-compatible signatures |
| `123` | `SECP256R1_KECCAK256` | secp256r1 | Keccak-256 | Neo + Keccak signatures        |

### Code Example

```solidity
import "devpack/contracts/Syscalls.sol";

contract SignatureVerifier {
    function verifyEthSignature(
        bytes32 messageHash,
        bytes memory publicKey,
        bytes memory signature
    ) public view returns (bool) {
        // secp256k1 + Keccak-256 for Ethereum compatibility
        return Syscalls.verifyWithECDsa(
            messageHash, publicKey, signature,
            Syscalls.SECP256K1_KECCAK256
        );
    }

    function verifyNeoSignature(
        bytes32 messageHash,
        bytes memory publicKey,
        bytes memory signature
    ) public view returns (bool) {
        // secp256r1 + SHA-256 for Neo-native signatures
        return Syscalls.verifyWithECDsa(
            messageHash, publicKey, signature,
            Syscalls.SECP256R1_SHA256
        );
    }

    function hashData(bytes memory data) public view returns (bytes32) {
        return Syscalls.sha256(data);
    }
}
```

::: info
Solidity's built-in `keccak256()` and `sha256()` are automatically lowered to `CryptoLib.keccak256` and `CryptoLib.sha256` native contract calls by the compiler. The `Syscalls.*` wrappers are provided for explicit usage when you want to call through the devpack namespace directly.
:::

---

## StdLib

Provides serialization, encoding, and string manipulation utilities. StdLib methods are exposed through `Syscalls.sol`.

### Methods

| Method            | Signature                               | Return     | Safe | Description                                         |
| ----------------- | --------------------------------------- | ---------- | :--: | --------------------------------------------------- |
| `serialize`       | `serialize(bytes)`                      | `bytes`    |  ✅  | Serialize a NeoVM stack item to bytes.              |
| `deserialize`     | `deserialize(bytes)`                    | `bytes`    |  ✅  | Deserialize bytes back to a stack item.             |
| `jsonSerialize`   | `jsonSerialize(bytes)`                  | `bytes`    |  ✅  | Serialize a stack item to JSON.                     |
| `jsonDeserialize` | `jsonDeserialize(bytes)`                | `bytes`    |  ✅  | Deserialize JSON to a stack item.                   |
| `base64Encode`    | `base64Encode(bytes)`                   | `string`   |  ✅  | Base64 encode.                                      |
| `base64Decode`    | `base64Decode(string)`                  | `bytes`    |  ✅  | Base64 decode.                                      |
| `base58Encode`    | `base58Encode(bytes)`                   | `string`   |  ✅  | Base58 encode.                                      |
| `base58Decode`    | `base58Decode(string)`                  | `bytes`    |  ✅  | Base58 decode.                                      |
| `itoa`            | `itoa(int256,uint8)`                    | `string`   |  ✅  | Integer to string (base 10 or 16).                  |
| `atoi`            | `atoi(string,uint8)`                    | `int256`   |  ✅  | String to integer (base 10 or 16).                  |
| `memoryCompare`   | `memoryCompare(bytes,bytes)`            | `int256`   |  ✅  | Lexicographic byte comparison. Returns -1, 0, or 1. |
| `memorySearch`    | `memorySearch(bytes,bytes,int256,bool)` | `int256`   |  ✅  | Search for a byte pattern. Returns index or -1.     |
| `stringSplit`     | `stringSplit(string,string,bool)`       | `string[]` |  ✅  | Split string by separator.                          |
| `strLen`          | `strLen(string)`                        | `uint256`  |  ✅  | String length in text elements (not bytes).         |

### Code Example

```solidity
import "devpack/contracts/Syscalls.sol";

contract DataEncoder {
    function encodeForStorage(uint256 value) public view returns (bytes memory) {
        return Syscalls.serialize(abi.encode(value));
    }

    function toBase64(bytes memory data) public view returns (string memory) {
        return Syscalls.base64Encode(data);
    }

    function numberToHexString(int256 value) public view returns (string memory) {
        return Syscalls.itoa(value, 16);
    }

    function splitCsv(string memory csv) public view returns (string[] memory) {
        return Syscalls.stringSplit(csv, ",", true);
    }
}
```

---

## Method Token Optimization (CALLT)

By default, native contract calls are emitted as `System.Contract.Call` syscalls with the target contract hash and method name pushed onto the stack. The `--callt` compiler flag enables an optimization that replaces these with `CALLT` instructions referencing a method token table embedded in the NEF binary.

```bash
neo-solc contract.sol --callt -O3 -o build/contract
```

### How It Works

Without `--callt`:

```
PUSHDATA1 <20-byte contract hash>
PUSHDATA1 <method name string>
PUSHINT8 <call flags>
SYSCALL System.Contract.Call
```

With `--callt`:

```
CALLT <2-byte token index>
```

The NEF header contains a method token table that maps each token index to a `(contractHash, method, parametersCount, hasReturnValue, callFlags)` tuple. The Neo VM resolves the token at load time.

### Benefits

- Smaller bytecode — eliminates inline contract hash and method name strings per call site
- Lower GAS cost — fewer opcodes executed per native call
- Explicit manifest permissions — each method token generates a precise permission entry

::: info
CALLT is a pure optimization. It does not change contract behavior. Contracts compiled with and without `--callt` are functionally identical. Use `--callt` in production builds for best efficiency.
:::

---

## Permission Model

Every native contract call generates a permission entry in the compiled manifest's `permissions` array. The compiler infers these permissions from the IR and emits explicit `contract + methods` entries.

### Fixed vs Wildcard Permissions

| Call Pattern                     | Manifest Permission                                     |
| -------------------------------- | ------------------------------------------------------- |
| `NativeCalls.neoBalanceOf(addr)` | `{ "contract": "0xef40...", "methods": ["balanceOf"] }` |
| `NativeCalls.gasTransfer(...)`   | `{ "contract": "0xd2a4...", "methods": ["transfer"] }`  |
| `Syscalls.sha256(data)`          | `{ "contract": "0x726c...", "methods": ["sha256"] }`    |
| Dynamic contract call            | `{ "contract": "*", "methods": "*" }`                   |

Using the devpack's fixed wrappers (`NativeCalls.*`, `Syscalls.*`) produces precise permission entries. Dynamic calls through `Syscalls.contractCall()` with runtime-computed targets or method names may force wildcard permissions.

### Hardening

Reject wildcard permissions in production builds:

```bash
neo-solc contract.sol \
  --callt \
  --deny-wildcard-contracts \
  --deny-wildcard-methods \
  -o build/contract
```

::: warning
Wildcard permissions (`"contract": "*"` or `"methods": "*"`) allow the contract to call any contract or method on the network. Always audit the generated manifest before deployment. Use `--deny-wildcard-contracts --deny-wildcard-methods` to make the compiler reject any code path that would require wildcards.
:::

---

## See Also

- [Syscalls](/internals/syscalls) — NeoVM syscall surface reference
- [Manifest Spec](/internals/contract-metadata) — manifest structure and permission model
- [Devpack Overview](/additional-material/neo-devpack) — devpack library layout and usage
- [Language Description](/language-description/types) — how EVM globals map to native contract calls
- [Compile Workflow](/compiler/analysing-the-compiler-output) — compiler flags including `--callt`
