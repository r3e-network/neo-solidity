# Syscalls

NeoVM syscalls are the interface between smart contract bytecode and the Neo N3 blockchain runtime. Each syscall is identified by a 4-byte hash derived from SHA-256 of its name string. The neo-solidity compiler lowers Solidity constructs to these syscalls automatically.

When the NeoVM encounters a `SYSCALL` opcode, it reads the next 4 bytes as the syscall ID, looks up the corresponding handler in the interop service table, and dispatches execution. The compiler handles this translation transparently — you write Solidity, and the correct syscall sequence is emitted in the NEF output.

## Overview

Syscalls are invoked via the `SYSCALL` opcode followed by a 4-byte identifier. The identifier is the first 4 bytes of `SHA256(syscall_name_string)`. For example, `System.Storage.Get` has the ID derived from `SHA256("System.Storage.Get")`.

The compiler maps Solidity patterns to the appropriate syscalls during code generation. State variable reads become `System.Storage.Get`, event emissions become `System.Runtime.Notify`, and cross-contract calls become `System.Contract.Call`.

The devpack provides two levels of access:

- **Direct wrappers** — `Syscalls.sol` exposes every syscall as a typed Solidity function.
- **Ergonomic libraries** — `Storage.sol`, `Runtime.sol`, and `Neo.sol` provide higher-level APIs built on top of the raw syscalls.

```solidity
import "devpack/contracts/Syscalls.sol";

// Direct syscall access
address caller = Syscalls.getCallingScriptHash();

// Or use the ergonomic wrapper
import "devpack/libraries/Runtime.sol";
bool authorized = Runtime.checkWitness(caller);
```

## Syscall Categories

Neo N3 defines 38 syscalls across 5 categories. The compiler and devpack cover all of them.

| Category | Count | Prefix              | Purpose                             |
| -------- | ----: | ------------------- | ----------------------------------- |
| Storage  |    11 | `System.Storage.*`  | Persistent key-value state          |
| Runtime  |    19 | `System.Runtime.*`  | Execution context and notifications |
| Contract |     4 | `System.Contract.*` | Cross-contract calls and accounts   |
| Crypto   |     2 | `System.Crypto.*`   | Signature verification              |
| Iterator |     2 | `System.Iterator.*` | Traversal of storage search results |

::: info
Most cryptographic operations (SHA256, RIPEMD160, keccak256, ECDSA verification, BLS12-381) are exposed through the `CryptoLib` native contract, not through syscalls. The two `System.Crypto.*` syscalls handle only signature verification against the current script container.
:::

## Storage Syscalls

Storage syscalls provide persistent key-value state for contracts. Each contract has its own isolated storage context.

| Syscall Name                        | Gas Cost | Description                       | Devpack Wrapper                        |
| ----------------------------------- | -------: | --------------------------------- | -------------------------------------- |
| `System.Storage.GetContext`         |        1 | Get current storage context       | `Syscalls.getStorageContext()`         |
| `System.Storage.GetReadOnlyContext` |        1 | Get read-only context             | `Syscalls.getReadOnlyStorageContext()` |
| `System.Storage.AsReadOnly`         |        1 | Convert context to read-only      | `Syscalls.storageAsReadOnly(ctx)`      |
| `System.Storage.Get`                |      100 | Read value by key                 | `Syscalls.storageGet(ctx, key)`        |
| `System.Storage.Put`                |    1,000 | Write key-value pair              | `Syscalls.storagePut(ctx, key, val)`   |
| `System.Storage.Delete`             |      100 | Delete key                        | `Syscalls.storageDelete(ctx, key)`     |
| `System.Storage.Find`               |      100 | Find by prefix (returns iterator) | `Syscalls.storageFind(ctx, prefix)`    |
| `System.Storage.Local.Get`          |      100 | Read from local context           | `Syscalls.storageGetLocal(key)`        |
| `System.Storage.Local.Put`          |    1,000 | Write to local context            | `Syscalls.storagePutLocal(key, val)`   |
| `System.Storage.Local.Delete`       |      100 | Delete from local context         | `Syscalls.storageDeleteLocal(key)`     |
| `System.Storage.Local.Find`         |      100 | Find in local context             | `Syscalls.storageFindLocal(prefix)`    |

Local storage (`System.Storage.Local.*`) is contract-private and cannot be read by other contracts even through `System.Storage.GetReadOnlyContext`. Use it for internal bookkeeping that should never be externally visible.

```solidity
import "devpack/contracts/Syscalls.sol";

contract TokenVault {
    // The compiler lowers state variables to Storage.Get/Put automatically.
    // For manual control, use the Syscalls library directly:

    function manualStorageExample(bytes memory key, bytes memory value) internal {
        // Get the current contract's storage context
        Syscalls.StorageContext memory ctx = Syscalls.getStorageContext();

        // Write a key-value pair (costs 1,000 GAS units)
        Syscalls.storagePut(ctx, key, value);

        // Read it back (costs 100 GAS units)
        bytes memory stored = Syscalls.storageGet(ctx, key);

        // Delete when no longer needed (costs 100 GAS units)
        Syscalls.storageDelete(ctx, key);
    }
}
```

::: tip Storage Cost Awareness
`System.Storage.Put` costs 1,000 GAS units — 10x more expensive than `Get` or `Delete` (100 each). Minimize writes by batching updates and avoiding redundant puts. The `Storage.sol` library provides `batchPut()` for multi-key writes, but each individual put still incurs the full syscall cost.
:::

## Runtime Syscalls

Runtime syscalls provide access to execution context, authorization, notifications, and platform metadata.

| Syscall Name                            | Gas Cost | Description                              | Devpack Wrapper                            |
| --------------------------------------- | -------: | ---------------------------------------- | ------------------------------------------ |
| `System.Runtime.GetTrigger`             |        1 | Get execution trigger type               | `Syscalls.getTrigger()`                    |
| `System.Runtime.Platform`               |        1 | Get platform string (`"NEO"`)            | `Syscalls.getPlatform()`                   |
| `System.Runtime.GetNetwork`             |        1 | Get network magic number                 | `Syscalls.getNetwork()`                    |
| `System.Runtime.GetAddressVersion`      |        1 | Get address version byte                 | `Syscalls.getAddressVersion()`             |
| `System.Runtime.GetTime`                |        1 | Get current block timestamp (ms)         | `Syscalls.getTime()`                       |
| `System.Runtime.GetScriptContainer`     |        1 | Get transaction container                | `Syscalls.getScriptContainer()`            |
| `System.Runtime.GetExecutingScriptHash` |        1 | Get current contract hash                | `Syscalls.getExecutingScriptHash()`        |
| `System.Runtime.GetCallingScriptHash`   |        1 | Get caller contract hash                 | `Syscalls.getCallingScriptHash()`          |
| `System.Runtime.GetEntryScriptHash`     |        1 | Get entry point hash                     | `Syscalls.getEntryScriptHash()`            |
| `System.Runtime.LoadScript`             |        1 | Load and execute script                  | `Syscalls.loadScript(script, flags, args)` |
| `System.Runtime.CheckWitness`           |      200 | Verify witness authorization             | `Syscalls.checkWitness(hash)`              |
| `System.Runtime.GetInvocationCounter`   |        1 | Get call count for current contract      | `Syscalls.getInvocationCounter()`          |
| `System.Runtime.GetRandom`              |       50 | Get deterministic random number          | `Syscalls.getCurrentRandom()`              |
| `System.Runtime.Log`                    |        1 | Emit log message                         | `Syscalls.log(message)`                    |
| `System.Runtime.Notify`                 |        1 | Emit notification (event)                | `Syscalls.notify(data)`                    |
| `System.Runtime.GetNotifications`       |        1 | Get notifications from current execution | `Syscalls.getNotifications()`              |
| `System.Runtime.GasLeft`                |        1 | Get remaining GAS                        | `Syscalls.gasLeft()`                       |
| `System.Runtime.BurnGas`                |        1 | Burn specified GAS amount                | `Syscalls.burnGas(amount)`                 |
| `System.Runtime.CurrentSigners`         |        1 | Get transaction signers                  | `Syscalls.getCurrentSigners()`             |

```solidity
import "devpack/contracts/Syscalls.sol";

contract Guarded {
    address public owner;

    modifier onlyOwner() {
        // CheckWitness verifies the transaction includes a valid
        // signature for the given address (200 GAS units)
        require(Syscalls.checkWitness(owner), "unauthorized");
        _;
    }

    function sensitiveAction() external onlyOwner {
        // Only executes if the transaction signer controls `owner`
        Syscalls.log("sensitiveAction executed");
    }

    function getExecutionInfo() external view returns (
        address executing,
        address caller,
        uint256 timestamp,
        uint256 remainingGas
    ) {
        executing = Syscalls.getExecutingScriptHash();  // address(this)
        caller = Syscalls.getCallingScriptHash();        // msg.sender
        timestamp = Syscalls.getTime();                  // block.timestamp
        remainingGas = Syscalls.gasLeft();               // gasleft()
    }
}
```

::: warning CheckWitness Gas Cost
`System.Runtime.CheckWitness` costs 200 GAS units — the most expensive runtime syscall. It performs cryptographic signature verification against the transaction's witness list. Avoid calling it in tight loops. Cache the result in a local variable when you need to check the same witness multiple times within a single execution.
:::

## Contract Syscalls

Contract syscalls handle cross-contract invocation and account creation.

| Syscall Name                            | Gas Cost | Description                    | Devpack Wrapper                               |
| --------------------------------------- | -------: | ------------------------------ | --------------------------------------------- |
| `System.Contract.Call`                  |       10 | Cross-contract call            | `Syscalls.contractCall(hash, method, params)` |
| `System.Contract.GetCallFlags`          |       10 | Get current call flags         | `Syscalls.getCallFlags()`                     |
| `System.Contract.CreateStandardAccount` |       10 | Create account from public key | `Syscalls.createStandardAccount(pubkey)`      |
| `System.Contract.CreateMultisigAccount` |       10 | Create multisig account        | `Syscalls.createMultisigAccount(m, pubkeys)`  |

```solidity
import "devpack/contracts/Syscalls.sol";

contract Bridge {
    address constant GAS_CONTRACT = 0xd2a4cff31913016155e38e474a2c06d08be276cf;

    function checkBalance(address account) external returns (bytes memory) {
        // Cross-contract call to the GAS native contract (10 GAS units)
        bytes memory params = abi.encode(account);
        return Syscalls.contractCall(GAS_CONTRACT, "balanceOf", params);
    }

    function createMultisig(uint256 threshold, bytes[] memory pubkeys)
        external
        returns (address)
    {
        // Create a multisig account requiring `threshold` of `pubkeys`
        return Syscalls.createMultisigAccount(threshold, pubkeys);
    }
}
```

### CallFlags

The `System.Contract.GetCallFlags` syscall returns the permission flags for the current execution context. These flags control what operations the called contract is allowed to perform.

| Flag          | Value  | Description                                 |
| ------------- | ------ | ------------------------------------------- |
| `None`        | `0x00` | No permissions                              |
| `ReadStates`  | `0x01` | Can read storage and call other contracts   |
| `WriteStates` | `0x02` | Can write to storage                        |
| `AllowCall`   | `0x04` | Can make cross-contract calls               |
| `AllowNotify` | `0x08` | Can emit notifications                      |
| `All`         | `0x0F` | Full permissions (default for direct calls) |

::: tip
When calling another contract, you can restrict its permissions by passing specific call flags. This is a defense-in-depth mechanism — a called contract with `ReadStates` only cannot modify your storage even if it contains malicious code.
:::

## Crypto Syscalls

Crypto syscalls verify signatures against the current transaction's script container.

| Syscall Name                  | Gas Cost | Description                | Devpack Wrapper                         |
| ----------------------------- | -------: | -------------------------- | --------------------------------------- |
| `System.Crypto.CheckSig`      |    1,000 | Verify single signature    | `Syscalls.checkSig(pubkey, sig)`        |
| `System.Crypto.CheckMultisig` |    1,000 | Verify multiple signatures | `Syscalls.checkMultisig(pubkeys, sigs)` |

These syscalls verify signatures against the hash of the current script container (transaction). For general-purpose signature verification with arbitrary messages, use the `CryptoLib` native contract via `Syscalls.verifyWithECDsa()`.

```solidity
import "devpack/contracts/Syscalls.sol";

contract Verifier {
    // Verify a single signature against the current transaction
    function verifySigner(bytes memory publicKey, bytes memory signature)
        external view returns (bool)
    {
        return Syscalls.checkSig(publicKey, signature);
    }

    // For arbitrary message verification, use CryptoLib (native contract)
    function verifyMessage(
        bytes32 messageHash,
        bytes memory publicKey,
        bytes memory signature
    ) external view returns (bool) {
        return Syscalls.verifyWithECDsa(
            messageHash, publicKey, signature,
            Syscalls.SECP256K1_SHA256  // or SECP256R1_SHA256
        );
    }
}
```

::: info
Most cryptographic operations in Neo N3 are handled by the `CryptoLib` native contract (SHA256, RIPEMD160, keccak256, ECDSA, Ed25519, BLS12-381, Murmur32), which is invoked via `System.Contract.Call` rather than dedicated syscalls. The two `System.Crypto.*` syscalls exist specifically for transaction-level signature verification used in consensus and verification triggers.
:::

## Iterator Syscalls

Iterator syscalls traverse results returned by `System.Storage.Find`.

| Syscall Name            | Gas Cost | Description                | Devpack Wrapper                |
| ----------------------- | -------: | -------------------------- | ------------------------------ |
| `System.Iterator.Next`  |        1 | Advance iterator           | `Syscalls.iteratorNext(iter)`  |
| `System.Iterator.Value` |        1 | Get current iterator value | `Syscalls.iteratorValue(iter)` |

Iterators are the only way to enumerate storage keys by prefix on Neo N3. They are returned by `System.Storage.Find` and consumed with the `Next`/`Value` pair.

```solidity
import "devpack/contracts/Syscalls.sol";

contract Registry {
    bytes constant PREFIX_USER = "user:";

    function listUsers() external view returns (bytes[] memory) {
        // Find all storage keys starting with "user:" (100 GAS units)
        Syscalls.StorageContext memory ctx = Syscalls.getReadOnlyStorageContext();
        Syscalls.Iterator memory iter = Syscalls.storageFind(ctx, PREFIX_USER);

        // Iterate through results (1 GAS unit per Next + 1 per Value)
        bytes[] memory temp = new bytes[](100);
        uint256 count = 0;

        while (Syscalls.iteratorNext(iter) && count < 100) {
            temp[count] = Syscalls.iteratorValue(iter);
            count++;
        }

        // Trim to actual size
        bytes[] memory result = new bytes[](count);
        for (uint256 i = 0; i < count; i++) {
            result[i] = temp[i];
        }
        return result;
    }
}
```

::: warning
Iterators cannot be passed across contract boundaries or stored persistently. They are valid only within the execution context that created them. Attempting to use an iterator from a different invocation will fail.
:::

## Gas Cost Reference

All 38 syscalls sorted by gas cost, grouped into cost tiers.

### Tier 1 — 1 GAS unit

Metadata reads, logging, and iterator traversal. Cheap and safe to call frequently.

| Syscall Name                            | Category |
| --------------------------------------- | -------- |
| `System.Storage.GetContext`             | Storage  |
| `System.Storage.GetReadOnlyContext`     | Storage  |
| `System.Storage.AsReadOnly`             | Storage  |
| `System.Runtime.GetTrigger`             | Runtime  |
| `System.Runtime.Platform`               | Runtime  |
| `System.Runtime.GetNetwork`             | Runtime  |
| `System.Runtime.GetAddressVersion`      | Runtime  |
| `System.Runtime.GetTime`                | Runtime  |
| `System.Runtime.GetScriptContainer`     | Runtime  |
| `System.Runtime.GetExecutingScriptHash` | Runtime  |
| `System.Runtime.GetCallingScriptHash`   | Runtime  |
| `System.Runtime.GetEntryScriptHash`     | Runtime  |
| `System.Runtime.LoadScript`             | Runtime  |
| `System.Runtime.GetInvocationCounter`   | Runtime  |
| `System.Runtime.Log`                    | Runtime  |
| `System.Runtime.Notify`                 | Runtime  |
| `System.Runtime.GetNotifications`       | Runtime  |
| `System.Runtime.GasLeft`                | Runtime  |
| `System.Runtime.BurnGas`                | Runtime  |
| `System.Runtime.CurrentSigners`         | Runtime  |
| `System.Iterator.Next`                  | Iterator |
| `System.Iterator.Value`                 | Iterator |

### Tier 2 — 10 GAS units

Contract interaction syscalls.

| Syscall Name                            | Category |
| --------------------------------------- | -------- |
| `System.Contract.Call`                  | Contract |
| `System.Contract.GetCallFlags`          | Contract |
| `System.Contract.CreateStandardAccount` | Contract |
| `System.Contract.CreateMultisigAccount` | Contract |

### Tier 3 — 50 GAS units

| Syscall Name               | Category |
| -------------------------- | -------- |
| `System.Runtime.GetRandom` | Runtime  |

### Tier 4 — 100 GAS units

Storage read, delete, and search operations.

| Syscall Name                  | Category |
| ----------------------------- | -------- |
| `System.Storage.Get`          | Storage  |
| `System.Storage.Delete`       | Storage  |
| `System.Storage.Find`         | Storage  |
| `System.Storage.Local.Get`    | Storage  |
| `System.Storage.Local.Delete` | Storage  |
| `System.Storage.Local.Find`   | Storage  |

### Tier 5 — 200 GAS units

| Syscall Name                  | Category |
| ----------------------------- | -------- |
| `System.Runtime.CheckWitness` | Runtime  |

### Tier 6 — 1,000 GAS units

Storage writes and cryptographic verification. The most expensive operations.

| Syscall Name                  | Category |
| ----------------------------- | -------- |
| `System.Storage.Put`          | Storage  |
| `System.Storage.Local.Put`    | Storage  |
| `System.Crypto.CheckSig`      | Crypto   |
| `System.Crypto.CheckMultisig` | Crypto   |

::: info Gas Accuracy
The neo-solidity embedded runtime tracks syscall gas costs at approximately 85% accuracy compared to Neo N3 mainnet. The primary gaps are in storage pricing (which depends on the dynamic `Policy.getStoragePrice()` value on mainnet) and opcode-level metering. Always perform final gas estimation against a Neo N3 testnet node before mainnet deployment.
:::

## Solidity to Syscall Mapping

The compiler automatically translates standard Solidity constructs to the corresponding NeoVM syscalls. No manual syscall invocation is needed for these patterns.

| Solidity Construct                    | Syscall / Neo Call                       | Notes                                       |
| ------------------------------------- | ---------------------------------------- | ------------------------------------------- |
| `msg.sender`                          | `System.Runtime.GetCallingScriptHash`    | Returns caller's script hash (UInt160)      |
| `address(this)`                       | `System.Runtime.GetExecutingScriptHash`  | Returns current contract's script hash      |
| `block.timestamp`                     | `System.Runtime.GetTime`                 | Milliseconds, normalized to seconds         |
| `block.number`                        | `Ledger.currentIndex` (native call)      | Not a syscall — uses `System.Contract.Call` |
| `gasleft()`                           | `System.Runtime.GasLeft`                 | Remaining GAS in current invocation         |
| State variable read                   | `System.Storage.Get`                     | Key derived from variable name via SHA256   |
| State variable write                  | `System.Storage.Put`                     | Key derived from variable name via SHA256   |
| `mapping[key]` read                   | `System.Storage.Get`                     | Key = `SHA256(serialize(key) \|\| slot)`    |
| `mapping[key]` write                  | `System.Storage.Put`                     | Same derived key                            |
| `emit Event(...)`                     | `System.Runtime.Notify`                  | Event name + args as notification           |
| `require(Runtime.checkWitness(addr))` | `System.Runtime.CheckWitness`            | Witness verification (200 GAS)              |
| `address(target).call(...)`           | `System.Contract.Call`                   | Cross-contract invocation (10 GAS)          |
| `address(target).staticcall(...)`     | `System.Contract.Call` (read-only flags) | Read-only cross-contract call               |
| `keccak256(...)`                      | `CryptoLib.keccak256` (native call)      | Via `System.Contract.Call`, not a syscall   |
| `sha256(...)`                         | `CryptoLib.sha256` (native call)         | Via `System.Contract.Call`, not a syscall   |

```solidity
// What you write:
contract Example {
    mapping(address => uint256) public balances;
    event Transfer(address indexed from, address indexed to, uint256 amount);

    function transfer(address to, uint256 amount) external {
        require(Runtime.checkWitness(msg.sender), "unauthorized");
        balances[msg.sender] -= amount;
        balances[to] += amount;
        emit Transfer(msg.sender, to, amount);
    }
}

// What the compiler emits (pseudocode):
// 1. System.Runtime.CheckWitness(callingScriptHash)  — 200 GAS
// 2. System.Storage.GetContext()                      — 1 GAS
// 3. System.Storage.Get(ctx, derived_key_sender)      — 100 GAS
// 4. System.Storage.Put(ctx, derived_key_sender, new) — 1,000 GAS
// 5. System.Storage.Get(ctx, derived_key_to)          — 100 GAS
// 6. System.Storage.Put(ctx, derived_key_to, new)     — 1,000 GAS
// 7. System.Runtime.Notify("Transfer", [from, to, amount]) — 1 GAS
// Total: ~2,402 GAS units (syscalls only, excludes opcode costs)
```

## Devpack Wrapper Reference

The devpack provides syscall access at two abstraction levels.

### Low-Level: `Syscalls.sol`

Located at `devpack/contracts/Syscalls.sol`. Provides 1:1 typed wrappers for every Neo N3 syscall, plus convenience functions for native contract calls (CryptoLib, StdLib, Ledger, Policy, Oracle, RoleManagement).

Key features:

- All 38 syscalls exposed as `internal` Solidity functions
- Data structures: `Block`, `Transaction`, `Signer`, `StorageContext`, `Iterator`, `Notification`
- Constants: trigger types, witness scopes, witness conditions, named curve hashes
- Native contract script hash constants for all 7 core native contracts

### High-Level: Ergonomic Libraries

| Library   | File                            | Built On                                   |
| --------- | ------------------------------- | ------------------------------------------ |
| `Storage` | `devpack/libraries/Storage.sol` | `System.Storage.*` syscalls                |
| `Runtime` | `devpack/libraries/Runtime.sol` | `System.Runtime.*` syscalls                |
| `Neo`     | `devpack/libraries/Neo.sol`     | Multiple syscall categories + native calls |

The `Storage` library adds batch operations, typed accessors (`putUint256`, `getAddress`, `putBool`), iterator helpers (`findKeys`, `findValues`, `count`), key derivation for mappings and arrays, and storage patterns like expiration and checksummed writes.

```solidity
// Low-level: direct syscall
Syscalls.StorageContext memory ctx = Syscalls.getStorageContext();
Syscalls.storagePut(ctx, "key", abi.encode(42));

// High-level: ergonomic wrapper
Storage.putUint256("key", 42);
uint256 value = Storage.getUint256("key");
```

## See Also

- [Native Contracts](/internals/native-contracts) — NEO, GAS, ContractManagement, CryptoLib, and other native contract references
- [Runtime Specification](/internals/runtime-specification) — embedded runtime behavior and fidelity notes
- [Language Description](/language-description/types) — complete Solidity construct translation reference
- [Devpack Overview](/additional-material/neo-devpack) — library layout and usage guide
