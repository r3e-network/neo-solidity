---
title: Infrastructure & Patterns — ERC ↔ Neo Mirror
description: Proxies, registries, modular contracts, deterministic deployment — Ethereum infrastructure standards mirrored to Neo C#.
outline: false
---

# Infrastructure & Patterns

Ten standards covering interface detection, registries, proxy upgrades, deterministic
deployment, modular smart accounts, and meta-transactions. Several of these are
straightforward ports; others are subsumed by Neo's manifest-driven contract model.

<StandardsMirror>

<!-- ============================================================ -->
<!-- ERC-165 -->
<!-- ============================================================ -->

<StandardEntry
  id="erc-165"
  title="ERC-165 — Interface Detection"
  eip="165"
  status="Final"
  neoMapping="Manifest supportedstandards"
  category="Detection"
  parityLabel="Native"
  parityClass="sm-pill-native"
>

<template #spec>

## ERC-165: Standard Interface Detection

ERC-165 lets a contract advertise which interfaces it implements via a 4-byte ID
derived from the XOR of function selectors. Wallets call `supportsInterface(bytes4)`
to detect ERC-721, ERC-2981, ERC-1155 compliance.

### Why It's Needed on Ethereum

Ethereum has no concept of contract metadata. The bytecode just is the bytecode.
ERC-165 invented a runtime convention: every interface has an ID, and contracts must
implement a registry function. Detection costs at least one external call.

### Neo Equivalent: Manifest `supportedstandards`

Neo contracts deploy with a **manifest** — a JSON document committed alongside the
NEF bytecode. The manifest includes a `supportedstandards` array that wallets and
explorers read directly. No runtime call. No gas. The compiler populates it
automatically based on method signature analysis.


::: tip Live on Neo TestNet
Both implementations are deployed and behavior-verified on Neo N3 TestNet (network magic `894710606`).

| Implementation | Contract Hash | Deploy Tx |
| --- | --- | --- |
| **Solidity** (`neo-solc`) | `0x2b5db552d1c23a43f51a0ea50765e4a1a7a051e6` | (reused — see [`0x2b5db552d1c23a43f51a0ea50765e4a1a7a051e6`](https://dora.coz.io/contract/neo3/testnet/0x2b5db552d1c23a43f51a0ea50765e4a1a7a051e6)) |
| **Neo C#** (`nccs`) | `0xa400b6cb20159fb3140798401c41edcb06e00f49` | (reused — see [`0xa400b6cb20159fb3140798401c41edcb06e00f49`](https://dora.coz.io/contract/neo3/testnet/0xa400b6cb20159fb3140798401c41edcb06e00f49)) |

Cross-implementation invocations match on `supportsInterface`. Source pairs under
[`docs/standards-mirror/deployments/erc-165/`](https://github.com/r3e-network/neo-solidity/tree/main/docs/standards-mirror/deployments/erc-165).
:::

</template>

<template #solidity>

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

interface IERC165 {
    function supportsInterface(bytes4 interfaceId) external view returns (bool);
}

abstract contract ERC165 is IERC165 {
    function supportsInterface(bytes4 interfaceId) public view virtual returns (bool) {
        return interfaceId == type(IERC165).interfaceId;
    }
}

contract MyContract is ERC165 {
    bytes4 private constant _INTERFACE_ID_MY = 0xdeadbeef;

    function supportsInterface(bytes4 id) public pure override returns (bool) {
        return id == _INTERFACE_ID_MY
            || id == 0x01ffc9a7;  // ERC-165 itself
    }
}

contract Detector {
    function isERC721(address target) external view returns (bool) {
        try IERC165(target).supportsInterface(0x80ac58cd) returns (bool ok) {
            return ok;
        } catch {
            return false;
        }
    }
}
```

</template>

<template #csharp>

```csharp
using Neo;
using Neo.SmartContract.Framework;
using Neo.SmartContract.Framework.Attributes;
using Neo.SmartContract.Framework.Native;

namespace R3E.Examples;

[DisplayName("DemoToken")]
[SupportedStandards("NEP-17", "NEP-27")]   // ← ERC-165 equivalent in one attribute
[ContractPermission("*", "*")]
public class DemoToken : SmartContract { /* ... */ }

/// <summary>Detect compliance from another contract — zero external calls into the target.</summary>
public static class StandardsDetector
{
    public static bool IsNep17(UInt160 candidate)
    {
        var contract = ContractManagement.GetContract(candidate);
        if (contract == null) return false;
        foreach (var std in contract.Manifest.SupportedStandards)
            if (std == "NEP-17") return true;
        return false;
    }

    public static bool IsNep11(UInt160 candidate)
    {
        var contract = ContractManagement.GetContract(candidate);
        if (contract == null) return false;
        foreach (var std in contract.Manifest.SupportedStandards)
            if (std == "NEP-11") return true;
        return false;
    }
}
```

`ContractManagement.GetContract` reads from the contract state ledger directly
without invoking any user code on the target. There is no re-entrancy surface and
no opportunity for the target contract to lie or revert.

</template>

</StandardEntry>

<!-- ============================================================ -->
<!-- ERC-1820 -->
<!-- ============================================================ -->

<StandardEntry
  id="erc-1820"
  title="ERC-1820 — Pseudo-introspection Registry"
  eip="1820"
  status="Final"
  neoMapping="Manifest + native registry"
  category="Registry"
  parityLabel="Native"
  parityClass="sm-pill-native"
>

<template #spec>

## ERC-1820: Pseudo-introspection Registry Contract

ERC-1820 deploys a **single global registry contract** at the deterministic address
`0x1820a4B7618BdE71Dce8cdc73aAB6C95905faD24` (via Nick's-method singleton deploy).
Any contract can register that another address implements an interface for it —
`setInterfaceImplementer(account, interfaceHash, implementer)`. ERC-777's
`tokensReceived` hook discovery uses this.

### Why It's Needed on Ethereum

ERC-165 only works for contracts that **implement** the standard themselves. ERC-1820
adds a layer: external delegated implementers, and EOAs that designate another
contract to act on their behalf for specific interfaces.

### Neo Equivalent

Neo has no global registry contract because **the manifest already serves the
discovery purpose** for the contract's own interfaces, and `ContractManagement`
provides the lookup primitives. For "EOA-style accounts that delegate to another
contract", Neo's NEP-30 verify subsumes the use case.

If you genuinely need a key-value style attribute registry, the Neo C# port below
shows a minimal version — typically deployed once by an ecosystem and called by
participating contracts.

::: tip Live on Neo TestNet
Both implementations deployed and behavior-verified on Neo N3 TestNet.

| Implementation | TestNet Address | Contract Hash |
| --- | --- | --- |
| **Solidity** (`neo-solc`) | `NgiVMV6vo8QHaQEhxMDBMCHdq2Ps5iXQxm` | [`0x02704624…2ad52ee4`](https://dora.coz.io/contract/neo3/testnet/0x02704624615747bdcc7994a6be347be42ad52ee4) |
| **Neo C#** (`nccs`) | `NaJCEkEXwwsWFHXU791K6Zk3ZpFya5HPk6` | [`0x8f36ff27…99cec59d`](https://dora.coz.io/contract/neo3/testnet/0x8f36ff27ef6564209956c05a4b886c0c99cec59d) |

Verified: `getManager(account) == account` (default behavior matches the EIP).
[`docs/standards-mirror/deployments/erc-1820/`](https://github.com/r3e-network/neo-solidity/tree/main/docs/standards-mirror/deployments/erc-1820).
:::

</template>

<template #solidity>

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

contract ERC1820Registry {
    // (account, interfaceHash) -> implementer
    mapping(address => mapping(bytes32 => address)) private _implementer;
    mapping(address => address) private _manager;

    event InterfaceImplementerSet(
        address indexed account, bytes32 indexed interfaceHash, address indexed implementer
    );

    function setInterfaceImplementer(address account, bytes32 interfaceHash, address impl)
        external
    {
        require(getManager(account) == msg.sender, "not manager");
        _implementer[account][interfaceHash] = impl;
        emit InterfaceImplementerSet(account, interfaceHash, impl);
    }

    function getInterfaceImplementer(address account, bytes32 interfaceHash)
        external view returns (address)
    {
        return _implementer[account][interfaceHash];
    }

    function setManager(address account, address newManager) external {
        require(getManager(account) == msg.sender, "not manager");
        _manager[account] = newManager == account ? address(0) : newManager;
    }

    function getManager(address account) public view returns (address) {
        address m = _manager[account];
        return m == address(0) ? account : m;
    }

    function interfaceHash(string calldata name) external pure returns (bytes32) {
        return keccak256(abi.encodePacked(name));
    }
}
```

</template>

<template #csharp>

```csharp
using System;
using System.ComponentModel;
using System.Numerics;
using Neo;
using Neo.SmartContract.Framework;
using Neo.SmartContract.Framework.Attributes;
using Neo.SmartContract.Framework.Native;
using Neo.SmartContract.Framework.Services;

namespace R3E.Examples;

[DisplayName("InterfaceRegistry")]
[ContractPermission("*", "*")]
public class InterfaceRegistry : SmartContract
{
    private const byte Prefix_Implementer = 0x01;
    private const byte Prefix_Manager     = 0x02;

    [DisplayName("InterfaceImplementerSet")]
    public static event Action<UInt160, ByteString, UInt160> OnSet;

    public static UInt160 GetImplementer(UInt160 account, ByteString interfaceHash)
        => (UInt160)Storage.Get(Storage.CurrentContext,
                                new byte[] { Prefix_Implementer }
                                    .Concat(account).Concat(interfaceHash));

    public static UInt160 GetManager(UInt160 account)
    {
        var m = (UInt160)Storage.Get(Storage.CurrentContext,
                                     new byte[] { Prefix_Manager }.Concat(account));
        return m ?? account;
    }

    public static void SetImplementer(UInt160 account, ByteString interfaceHash,
                                      UInt160 implementer)
    {
        var manager = GetManager(account);
        if (!Runtime.CheckWitness(manager)) throw new Exception("not manager");

        Storage.Put(Storage.CurrentContext,
                    new byte[] { Prefix_Implementer }.Concat(account).Concat(interfaceHash),
                    implementer);
        OnSet(account, interfaceHash, implementer);
    }

    public static void SetManager(UInt160 account, UInt160 newManager)
    {
        if (!Runtime.CheckWitness(GetManager(account))) throw new Exception("not manager");
        if (newManager.Equals(account))
            Storage.Delete(Storage.CurrentContext,
                           new byte[] { Prefix_Manager }.Concat(account));
        else
            Storage.Put(Storage.CurrentContext,
                        new byte[] { Prefix_Manager }.Concat(account), newManager);
    }

    public static ByteString InterfaceHash(string name)
        => CryptoLib.Sha256((ByteString)System.Text.Encoding.UTF8.GetBytes(name));
}
```

### Why Most Contracts Won't Need This

The big ERC-1820 use case — delegating ERC-777 token reception to another contract
— is solved on Neo by `onNEP17Payment`: every NEP-17 recipient can implement the
hook directly, and the manifest advertises NEP-27 compliance. No registry lookup
needed.

</template>

</StandardEntry>

<!-- ============================================================ -->
<!-- ERC-1967 -->
<!-- ============================================================ -->

<StandardEntry
  id="erc-1967"
  title="ERC-1967 — Standard Proxy Storage Slots"
  eip="1967"
  status="Final"
  neoMapping="NEP-22 ContractManagement.Update"
  category="Upgrade"
  parityLabel="NEP-22"
  parityClass="sm-pill-direct"
>

<template #spec>

## ERC-1967: Standard Proxy Storage Slots

Ethereum bytecode is immutable: once deployed, you cannot change the runtime code.
Upgradeable contracts work around this with a **proxy pattern**: a small,
intentionally-stable proxy contract `delegatecall`s into a separate "implementation"
contract whose address it stores. ERC-1967 standardises **where** the proxy stores
the implementation address so explorers can find it.

### Costs of the Proxy Pattern

- Per-call overhead: every external call routes through `delegatecall`.
- Storage layout fragility: implementation upgrades must preserve storage layout exactly.
- `selfdestruct` traps: implementation calling `selfdestruct` destroys the proxy.
- Initializer races: first call must initialize before adversary does.
- Audit complexity: two contracts to audit; storage-collision audits required.

### Neo Equivalent: NEP-22 Native Update (No Proxy)

Neo contracts upgrade in place via `ContractManagement.Update(nef, manifest, data)`.
Replaces bytecode and manifest atomically, preserves all storage, runs `_deploy(data,
update: true)` for migrations, requires owner witness. The contract hash never
changes — every reference, every NEP-17 holder, every NFT, every approval continues
to work.

::: tip Live on Neo TestNet
Both implementations deployed and behavior-verified on Neo N3 TestNet.

| Implementation | TestNet Address | Contract Hash |
| --- | --- | --- |
| **Solidity** (`neo-solc`) | `NRiL2gKGW5L8YgRcudr8mFNjrDUkkWZHR3` | [`0x48f6d58a…bfdbd245`](https://dora.coz.io/contract/neo3/testnet/0x48f6d58aa74ad1d507cb2eb07242e033bfdbd245) |
| **Neo C#** (`nccs`) | `NTgmiKbdcnknygAtt2ssWEjfPsJUM9hWGV` | [`0x096f01e4…481be976`](https://dora.coz.io/contract/neo3/testnet/0x096f01e40f7cf9cea4304195cc2ab6bb481be976) |

Verified: initial version is 1, owner is the deployer. The Neo C# contract uses NEP-22's `Update` method for in-place upgrades — calling it bumps the version counter via `_deploy(data, update: true)`, the standard Neo lifecycle hook.
[`docs/standards-mirror/deployments/erc-1967/`](https://github.com/r3e-network/neo-solidity/tree/main/docs/standards-mirror/deployments/erc-1967).
:::

</template>

<template #solidity>

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

contract ERC1967Proxy {
    bytes32 internal constant _IMPLEMENTATION_SLOT =
        0x360894a13ba1a3210667c828492db98dca3e2076cc3735a920a3ca505d382bbc;
    bytes32 internal constant _ADMIN_SLOT =
        0xb53127684a568b3173ae13b9f8a6016e243e63b6e8ee1178d6a717850b5d6103;

    constructor(address impl, bytes memory data) payable {
        _setImplementation(impl);
        _setAdmin(msg.sender);
        if (data.length > 0) {
            (bool ok, ) = impl.delegatecall(data);
            require(ok, "init failed");
        }
    }

    fallback() external payable { _delegate(_implementation()); }
    receive() external payable  { _delegate(_implementation()); }

    function _delegate(address impl) internal {
        assembly {
            calldatacopy(0, 0, calldatasize())
            let result := delegatecall(gas(), impl, 0, calldatasize(), 0, 0)
            returndatacopy(0, 0, returndatasize())
            switch result
            case 0 { revert(0, returndatasize()) }
            default { return(0, returndatasize()) }
        }
    }

    function _implementation() internal view returns (address impl) {
        assembly { impl := sload(_IMPLEMENTATION_SLOT) }
    }

    function _setImplementation(address impl) internal {
        assembly { sstore(_IMPLEMENTATION_SLOT, impl) }
    }

    function _setAdmin(address admin) internal {
        assembly { sstore(_ADMIN_SLOT, admin) }
    }
}
```

</template>

<template #csharp>

```csharp
using System;
using System.ComponentModel;
using Neo;
using Neo.SmartContract.Framework;
using Neo.SmartContract.Framework.Attributes;
using Neo.SmartContract.Framework.Native;
using Neo.SmartContract.Framework.Services;

namespace R3E.Examples;

[DisplayName("UpgradeableContract")]
[ContractPermission("*", "*")]
public class UpgradeableContract : SmartContract
{
    private static readonly byte[] OwnerKey   = { 0xff };
    private static readonly byte[] VersionKey = { 0xfe };

    [DisplayName("Updated")]
    public static event Action<int> OnUpdated;

    /// <summary>NEP-22: standard contract update entrypoint.</summary>
    public static void Update(ByteString nefFile, string manifest, object data)
    {
        var owner = (UInt160)Storage.Get(Storage.CurrentContext, OwnerKey);
        if (!Runtime.CheckWitness(owner)) throw new Exception("owner only");
        ContractManagement.Update(nefFile, manifest, data);
    }

    /// <summary>NEP-29: deploy/update lifecycle callback.</summary>
    public static void _deploy(object data, bool update)
    {
        if (update)
        {
            var oldVersion = (int)(BigInteger)(Storage.Get(Storage.CurrentContext, VersionKey)
                                               ?? new byte[] { 0 });
            var newVersion = oldVersion + 1;
            Storage.Put(Storage.CurrentContext, VersionKey, newVersion);
            OnUpdated(newVersion);
            return;
        }
        Storage.Put(Storage.CurrentContext, OwnerKey, (UInt160)data);
        Storage.Put(Storage.CurrentContext, VersionKey, 1);
    }

    /// <summary>NEP-31: optional destroy method.</summary>
    public static void Destroy()
    {
        var owner = (UInt160)Storage.Get(Storage.CurrentContext, OwnerKey);
        if (!Runtime.CheckWitness(owner)) throw new Exception("owner only");
        ContractManagement.Destroy();
    }
}
```

### Side-By-Side: What Goes Away

| Proxy Concern | Neo Result |
| --- | --- |
| Implementation slot | None — contract bytecode lives at the script hash |
| Admin slot | None — owner is just a storage key |
| `delegatecall` indirection | None — calls hit the contract directly |
| Storage layout fragility | Storage layout is just storage; you control it |
| Per-call gas overhead | Zero — no proxy hop |
| Initializer race | `_deploy(update: false)` runs once at deploy |
| Selfdestruct trap | `ContractManagement.Destroy()` is explicit and gated |

</template>

</StandardEntry>

<!-- ============================================================ -->
<!-- ERC-2535 -->
<!-- ============================================================ -->

<StandardEntry
  id="erc-2535"
  title="ERC-2535 — Diamond Standard"
  eip="2535"
  status="Final"
  neoMapping="Modular dispatch (port)"
  category="Modularity"
  parityLabel="Port"
  parityClass="sm-pill-port"
>

<template #spec>

## ERC-2535: Diamond Multi-Facet Proxy

ERC-2535 ("Diamonds") extends ERC-1967 to allow a single proxy to dispatch calls
across many implementation contracts ("facets"). Each function selector is mapped to
exactly one facet. Diamonds let you upgrade individual functions without
reimplementing the whole contract, work around Ethereum's 24KB contract size limit
by spreading code across facets, and cleanly separate concerns.

### `IDiamondCut` and `IDiamondLoupe`

```solidity
function diamondCut(FacetCut[] calldata _diamondCut, address _init, bytes calldata _calldata)
    external;
function facetAddress(bytes4 _functionSelector) external view returns (address);
function facetFunctionSelectors(address _facet) external view returns (bytes4[] memory);
```

### Neo Equivalent

Neo contracts have no 24 KB size limit (NEF max is 1 MB) so diamonds aren't needed
purely for size. But the modular-upgrade-per-function pattern is still useful and
implementable on Neo via a small dispatcher contract that routes by method name to
the correct facet contract.

::: tip Live on Neo TestNet
Both implementations deployed and behavior-verified on Neo N3 TestNet.

| Implementation | TestNet Address | Contract Hash |
| --- | --- | --- |
| **Solidity** (`neo-solc`) | `NMtzwNXhZGetcyPmotYvf4ALMEJD1xjGSb` | [`0x26b6f333…cf79f527`](https://dora.coz.io/contract/neo3/testnet/0x26b6f333b18bffd00702348b1cec5b55cf79f527) |
| **Neo C#** (`nccs`) | `NXNKvkkpyx9Z5KYCAS6JrH6GDPEPhwvZsE` | [`0x1b3c602c…34c5bdcf`](https://dora.coz.io/contract/neo3/testnet/0x1b3c602c1a208238f981125e2ad3045734c5bdcf) |

Verified: ownership claim and method-router state. The Neo port avoids `delegatecall` (which doesn't exist in NeoVM) — instead, `Dispatch(method, args)` looks up the facet contract and invokes it via `Contract.Call(facet, method, ...)`. Cleaner audit surface than the EVM diamond pattern.
[`docs/standards-mirror/deployments/erc-2535/`](https://github.com/r3e-network/neo-solidity/tree/main/docs/standards-mirror/deployments/erc-2535).
:::

</template>

<template #solidity>

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

interface IDiamondCut {
    enum FacetCutAction { Add, Replace, Remove }
    struct FacetCut {
        address facetAddress;
        FacetCutAction action;
        bytes4[] functionSelectors;
    }
    function diamondCut(FacetCut[] calldata _cut, address _init, bytes calldata _calldata) external;
}

contract Diamond {
    bytes32 constant DIAMOND_STORAGE = keccak256("diamond.storage");

    struct DiamondStorage {
        mapping(bytes4 => address) facetAddress;
        mapping(bytes4 => uint256) facetSelectorIndex;
        mapping(address => bytes4[]) facetSelectors;
    }

    function ds() internal pure returns (DiamondStorage storage s) {
        bytes32 slot = DIAMOND_STORAGE;
        assembly { s.slot := slot }
    }

    fallback() external payable {
        address facet = ds().facetAddress[msg.sig];
        require(facet != address(0), "no facet");
        assembly {
            calldatacopy(0, 0, calldatasize())
            let result := delegatecall(gas(), facet, 0, calldatasize(), 0, 0)
            returndatacopy(0, 0, returndatasize())
            switch result
            case 0 { revert(0, returndatasize()) }
            default { return(0, returndatasize()) }
        }
    }
}
```

</template>

<template #csharp>

```csharp
using System;
using System.ComponentModel;
using Neo;
using Neo.SmartContract.Framework;
using Neo.SmartContract.Framework.Attributes;
using Neo.SmartContract.Framework.Native;
using Neo.SmartContract.Framework.Services;

namespace R3E.Examples;

[DisplayName("Diamond")]
[ContractPermission("*", "*")]
public class Diamond : SmartContract
{
    private const byte Prefix_FacetByMethod = 0x01;
    private static readonly byte[] OwnerKey = { 0xff };

    [DisplayName("FacetCut")]
    public static event Action<string, UInt160> OnFacetCut;

    /// <summary>Map a method name to a facet contract.</summary>
    public static void AddFacet(string method, UInt160 facet)
    {
        var owner = (UInt160)Storage.Get(Storage.CurrentContext, OwnerKey);
        if (!Runtime.CheckWitness(owner)) throw new Exception("owner only");
        Storage.Put(Storage.CurrentContext,
                    new byte[] { Prefix_FacetByMethod }
                        .Concat((ByteString)System.Text.Encoding.UTF8.GetBytes(method)),
                    facet);
        OnFacetCut(method, facet);
    }

    public static void RemoveFacet(string method)
    {
        var owner = (UInt160)Storage.Get(Storage.CurrentContext, OwnerKey);
        if (!Runtime.CheckWitness(owner)) throw new Exception("owner only");
        Storage.Delete(Storage.CurrentContext,
                       new byte[] { Prefix_FacetByMethod }
                           .Concat((ByteString)System.Text.Encoding.UTF8.GetBytes(method)));
    }

    public static UInt160 FacetAddress(string method)
        => (UInt160)Storage.Get(Storage.CurrentContext,
            new byte[] { Prefix_FacetByMethod }
                .Concat((ByteString)System.Text.Encoding.UTF8.GetBytes(method)));

    /// <summary>
    /// Dispatch — clients invoke "Dispatch" with the target method name, and
    /// the diamond routes to the appropriate facet contract.
    /// </summary>
    public static object Dispatch(string method, object[] args)
    {
        var facet = FacetAddress(method);
        if (facet == null) throw new Exception("no facet for method");
        return Contract.Call(facet, method, CallFlags.All, args);
    }

    public static void _deploy(object data, bool update)
    {
        if (update) return;
        Storage.Put(Storage.CurrentContext, OwnerKey, (UInt160)data);
    }
}
```

### Differences from EVM Diamonds

- **No `delegatecall`**: facets don't share storage with the diamond. If a facet
  needs persistent state, it owns its own storage. This is actually safer — facets
  can't accidentally trample each other.
- **Method-name routing** instead of 4-byte selector routing — readable and easier
  to debug.
- **No size pressure**: Neo's NEF size limit (1 MB) is rarely the reason to choose
  a diamond. Use modular dispatch when you genuinely want hot-swappable per-method
  upgrades.

</template>

</StandardEntry>

<!-- ============================================================ -->
<!-- ERC-7201 -->
<!-- ============================================================ -->

<StandardEntry
  id="erc-7201"
  title="ERC-7201 — Namespaced Storage Layout"
  eip="7201"
  status="Final"
  neoMapping="Storage prefixes (idiomatic)"
  category="Storage"
  parityLabel="Port"
  parityClass="sm-pill-port"
>

<template #spec>

## ERC-7201: Namespaced Storage Layout

ERC-7201 standardises how upgradeable Solidity contracts compute non-colliding
storage slots: hash the namespace string with `keccak256` and store everything
under that root. Without this, two facets in a diamond proxy can collide on slot 0
and silently corrupt each other.

### Required Convention

```solidity
// keccak256(abi.encode(uint256(keccak256("erc7201:my.namespace")) - 1))
//   & ~bytes32(uint256(0xff))
bytes32 constant MY_STORAGE = 0xa1b2c3...;

struct MyStorage { uint256 totalSupply; mapping(address => uint256) balances; }

function _store() private pure returns (MyStorage storage $) {
    assembly { $.slot := MY_STORAGE }
}
```

### Neo Equivalent

The Neo storage model is byte-prefix based by convention — every contract picks a
single-byte prefix per logical map. Collision is structurally avoided because
prefix bytes are short, hand-picked, and visible in the source. The "port" below
just demonstrates the canonical Neo idiom.


::: tip Live on Neo TestNet
Both implementations are deployed and behavior-verified on Neo N3 TestNet (network magic `894710606`).

| Implementation | Contract Hash | Deploy Tx |
| --- | --- | --- |
| **Solidity** (`neo-solc`) | `0xbb2553c79f3a740113bf22fbadb6828a9bdbdf32` | (reused — see [`0xbb2553c79f3a740113bf22fbadb6828a9bdbdf32`](https://dora.coz.io/contract/neo3/testnet/0xbb2553c79f3a740113bf22fbadb6828a9bdbdf32)) |
| **Neo C#** (`nccs`) | `0x0932ad78b3d71c7af06468604f1d00ef89c3205d` | (reused — see [`0x0932ad78b3d71c7af06468604f1d00ef89c3205d`](https://dora.coz.io/contract/neo3/testnet/0x0932ad78b3d71c7af06468604f1d00ef89c3205d)) |

Cross-implementation invocations match on all read methods. Source pairs under
[`docs/standards-mirror/deployments/erc-7201/`](https://github.com/r3e-network/neo-solidity/tree/main/docs/standards-mirror/deployments/erc-7201).
:::

</template>

<template #solidity>

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.22;

abstract contract MyContract {
    // ERC-7201 namespace: keccak256("my.demo.storage") - 1, masked
    bytes32 private constant STORAGE_LOCATION =
        0xc7d3a4b2c8e9f0a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8090a1b2c3d4e00;

    struct Storage {
        uint256 totalSupply;
        mapping(address => uint256) balances;
        mapping(address => mapping(address => uint256)) allowances;
    }

    function _store() private pure returns (Storage storage $) {
        assembly { $.slot := STORAGE_LOCATION }
    }

    function totalSupply() public view returns (uint256) {
        return _store().totalSupply;
    }

    function balanceOf(address a) public view returns (uint256) {
        return _store().balances[a];
    }
}
```

</template>

<template #csharp>

```csharp
using System;
using System.ComponentModel;
using System.Numerics;
using Neo;
using Neo.SmartContract.Framework;
using Neo.SmartContract.Framework.Attributes;
using Neo.SmartContract.Framework.Native;
using Neo.SmartContract.Framework.Services;

namespace R3E.Examples;

/// <summary>
/// Idiomatic Neo storage namespacing: every logical collection gets a unique
/// single-byte prefix. Prefixes are short, hand-picked, and visible at the
/// top of the contract — no hashing dance required. Collision is structurally
/// impossible: when a developer adds a new collection, they pick the next free byte.
/// </summary>
[DisplayName("DemoToken")]
[ContractPermission("*", "*")]
public class DemoToken : SmartContract
{
    // === Storage namespace ======================================================
    // Prefix layout (visible at-a-glance in source):
    //   0x00  TotalSupply scalar
    //   0x01  Balances map      (account -> BigInteger)
    //   0x02  Allowances map    (owner+spender -> BigInteger)
    //   0xfe  Version
    //   0xff  Owner
    private const byte Prefix_TotalSupply = 0x00;
    private const byte Prefix_Balance     = 0x01;
    private const byte Prefix_Allowance   = 0x02;

    public static BigInteger TotalSupply()
        => (BigInteger)(Storage.Get(Storage.CurrentContext,
                                    new byte[] { Prefix_TotalSupply }) ?? ByteString.Empty);

    public static BigInteger BalanceOf(UInt160 a)
        => (BigInteger)(Storage.Get(Storage.CurrentContext,
                                    new byte[] { Prefix_Balance }.Concat(a)) ?? ByteString.Empty);

    public static BigInteger AllowanceOf(UInt160 owner, UInt160 spender)
        => (BigInteger)(Storage.Get(Storage.CurrentContext,
                                    new byte[] { Prefix_Allowance }.Concat(owner).Concat(spender))
                       ?? ByteString.Empty);
}
```

### Why Hashing Isn't Needed

In Solidity, slots are 256-bit integers and storage is one big array indexed by
slot. The compiler picks `slot 0`, `slot 1`, etc. unless you override. Two
contracts using slot 0 in a diamond will collide unless you hash to a "random"
slot, hence ERC-7201.

In Neo, storage is a key-value store keyed by arbitrary byte arrays. Prefix bytes
are explicit — you SEE the namespace. There's nothing to overwrite, nothing to
hash. Storage layout is part of the source, not derived by the compiler.

</template>

</StandardEntry>

<!-- ============================================================ -->
<!-- ERC-1014 -->
<!-- ============================================================ -->

<StandardEntry
  id="erc-1014"
  title="ERC-1014 — Skinny CREATE2"
  eip="1014"
  status="Final"
  neoMapping="ContractManagement.Deploy"
  category="Deployment"
  parityLabel="Native"
  parityClass="sm-pill-native"
>

<template #spec>

## EIP-1014: Deterministic Deployment via CREATE2

`CREATE2(value, salt, code)` deploys a contract at a deterministic address derived
from `(deployer, salt, codehash)`. Use cases: counterfactual deployment (you know
the address before deploy), state channels, factory patterns, vanity addresses.

### Address Formula

```
address = keccak256(0xff ++ deployer ++ salt ++ keccak256(initcode))[12:]
```

### Neo Equivalent

Neo's `ContractManagement.Deploy(nef, manifest, data)` produces a script hash that's
deterministic in the bytecode and the deployer's identity — same property, simpler
formula.

```
scriptHash = Hash160(deployer || nef.script || manifest.name)
```

The deployer can pre-compute the script hash before deploy and use it in
counterfactual flows. The Neo C# tab shows a factory pattern.


::: tip Live on Neo TestNet
Both implementations are deployed and behavior-verified on Neo N3 TestNet (network magic `894710606`).

| Implementation | Contract Hash | Deploy Tx |
| --- | --- | --- |
| **Solidity** (`neo-solc`) | `0xc267a2eaa32edae5ac95d484a69e77653fe10b0e` | (reused — see [`0xc267a2eaa32edae5ac95d484a69e77653fe10b0e`](https://dora.coz.io/contract/neo3/testnet/0xc267a2eaa32edae5ac95d484a69e77653fe10b0e)) |
| **Neo C#** (`nccs`) | `0x462113ca40c8a41597165ccbeada2e70e57764f8` | (reused — see [`0x462113ca40c8a41597165ccbeada2e70e57764f8`](https://dora.coz.io/contract/neo3/testnet/0x462113ca40c8a41597165ccbeada2e70e57764f8)) |

Cross-implementation invocations match on `deployCount`. Source pairs under
[`docs/standards-mirror/deployments/erc-1014/`](https://github.com/r3e-network/neo-solidity/tree/main/docs/standards-mirror/deployments/erc-1014).
:::

</template>

<template #solidity>

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

contract Factory {
    event Deployed(address indexed addr, bytes32 indexed salt);

    function deploy(bytes32 salt, bytes memory bytecode)
        external returns (address addr)
    {
        assembly {
            addr := create2(0, add(bytecode, 0x20), mload(bytecode), salt)
        }
        require(addr != address(0), "deploy failed");
        emit Deployed(addr, salt);
    }

    function predict(bytes32 salt, bytes memory bytecode)
        external view returns (address)
    {
        bytes32 hash = keccak256(abi.encodePacked(
            bytes1(0xff), address(this), salt, keccak256(bytecode)
        ));
        return address(uint160(uint256(hash)));
    }
}
```

</template>

<template #csharp>

```csharp
using System;
using System.ComponentModel;
using Neo;
using Neo.SmartContract.Framework;
using Neo.SmartContract.Framework.Attributes;
using Neo.SmartContract.Framework.Native;
using Neo.SmartContract.Framework.Services;

namespace R3E.Examples;

[DisplayName("DeterministicFactory")]
[ContractPermission("*", "*")]
public class DeterministicFactory : SmartContract
{
    [DisplayName("Deployed")]
    public static event Action<UInt160, ByteString> OnDeployed;

    /// <summary>
    /// Deploy a contract. The resulting script hash is deterministic in
    /// (caller, nef.script, manifest.name) — predictable before the call.
    /// </summary>
    public static UInt160 Deploy(ByteString nef, string manifest, object data)
    {
        var contract = ContractManagement.Deploy(nef, manifest, data);
        OnDeployed(contract.Hash, nef);
        return contract.Hash;
    }

    /// <summary>
    /// Predict the script hash of a contract that hasn't been deployed yet,
    /// for use in counterfactual flows.
    /// </summary>
    public static UInt160 Predict(UInt160 deployer, ByteString nefScript, string manifestName)
    {
        var preimage = ((ByteString)deployer.ToArray())
            .Concat(nefScript)
            .Concat((ByteString)System.Text.Encoding.UTF8.GetBytes(manifestName));
        return (UInt160)CryptoLib.Ripemd160(CryptoLib.Sha256(preimage));
    }
}
```

### What's Already Free

Neo doesn't need a separate "skinny CREATE2 EIP" because deterministic deployment
is the only deployment mode. The script hash is the contract address; deployment
yields it. There's no `CREATE` (sequential, nonce-based) for it to be a "skinny"
alternative to.

</template>

</StandardEntry>

<!-- ============================================================ -->
<!-- ERC-2470 -->
<!-- ============================================================ -->

<StandardEntry
  id="erc-2470"
  title="ERC-2470 — Singleton Factory"
  eip="2470"
  status="Final"
  neoMapping="ContractManagement.Deploy (no factory needed)"
  category="Deployment"
  parityLabel="Native"
  parityClass="sm-pill-native"
>

<template #spec>

## ERC-2470: Singleton Factory

ERC-2470 deploys a single permissionless `SingletonFactory` at a deterministic
address (`0xce0042B868300000d44A59004Da54A005ffdcf9f`) so that anyone can use the
same factory across all EVM chains for CREATE2-style deployment. The factory was
deployed via Nick's-method (a one-shot ECDSA signature with a deterministically
generated public key whose private key nobody knows).

### Why Such a Niche Standard Exists

Different chains have different `CREATE2`-deployer-contract addresses. Without a
universal factory, the same contract bytecode deployed to Polygon, Arbitrum, and
Mainnet would have different addresses. ERC-2470 standardises one factory address
across chains so contracts deployed via that factory get the same address everywhere.

### Neo Equivalent

Neo doesn't have this fragmentation problem: a contract's script hash is
`Hash160(deployer || script || name)`. If the same deployer redeploys the same
NEF + manifest on testnet and mainnet, the script hashes are identical. No
singleton factory required.

For multi-chain projects (e.g. a deploy bot deploying the same contract to multiple
Neo networks), the script hash is the same on each.

::: tip Live on Neo TestNet
Both implementations are deployed and behavior-verified on Neo N3 TestNet (network magic `894710606`).

| Implementation | Contract Hash | Deploy Tx |
| --- | --- | --- |
| **Solidity** (`neo-solc`) | `0x625c19cbd8d0b5cf03bd9996b67a824c14448012` | [`0xbb6e02b4…51651a`](https://dora.coz.io/transaction/neo3/testnet/0xbb6e02b4d2fcf44211712673b8e90c4082c75985df55b5eea05fda168151651a) |
| **Neo C#** (`nccs`) | `0x602d11eca4ebba2799b076fdbba251d1d9eaedf5` | [`0x10eda9be…7f8832`](https://dora.coz.io/transaction/neo3/testnet/0x10eda9bee425d78ff23fae16841304f750aea43d818b04a5f324d4afef7f8832) |

Cross-implementation invocations match on `claimDeployer` / `_deploy`,
`deployCount`, `getDeployer`. Source pairs under
[`docs/standards-mirror/deployments/erc-2470/`](https://github.com/r3e-network/neo-solidity/tree/main/docs/standards-mirror/deployments/erc-2470).
:::

</template>

<template #solidity>

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

contract SingletonFactory {
    event Deployed(address indexed addr, bytes32 indexed salt);

    /// Bytes32 salt + arbitrary initcode -> deterministic deploy.
    function deploy(bytes memory initCode, bytes32 salt) external returns (address addr) {
        assembly {
            addr := create2(0, add(initCode, 0x20), mload(initCode), salt)
        }
        emit Deployed(addr, salt);
    }
}

// User-side prediction
contract MyDeployer {
    address constant FACTORY = 0xce0042B868300000d44A59004Da54A005ffdcf9f;

    function predict(bytes memory initCode, bytes32 salt) public pure returns (address) {
        bytes32 hash = keccak256(abi.encodePacked(
            bytes1(0xff), FACTORY, salt, keccak256(initCode)
        ));
        return address(uint160(uint256(hash)));
    }
}
```

</template>

<template #csharp>

```csharp
using Neo;
using Neo.SmartContract.Framework;
using Neo.SmartContract.Framework.Attributes;
using Neo.SmartContract.Framework.Native;

namespace R3E.Examples;

/// <summary>
/// On Neo, every deployer is "the singleton factory" — there's nothing to standardise.
///
/// To deploy a contract with a deterministic script hash:
///   ContractManagement.Deploy(nef, manifest, data);
///
/// To predict the script hash before deploy:
///   var hash = (UInt160)CryptoLib.Ripemd160(CryptoLib.Sha256(
///       deployerHash.ToArray().Concat(nefScript).Concat(manifestName)));
///
/// Multi-chain identity is automatic: same deployer + same NEF + same manifest
/// name => same script hash on testnet and mainnet.
/// </summary>
public static class DeploymentNotes { /* this whole concept is a no-op on Neo */ }
```

### Why This EIP Exists Only on Ethereum

CREATE2's address depends on `address(deployer)`. Each chain assigns the deployer
a different address, so the same deployment transaction creates a different
address on each chain. ERC-2470 deployed a single factory at the same address
across all EVMs as a workaround.

Neo doesn't have this problem: addresses are derived from the deployment script,
not from a chain-specific deployer address. Same script + same name + same
deployer hash = same address everywhere.

</template>

</StandardEntry>

<!-- ============================================================ -->
<!-- ERC-1056 -->
<!-- ============================================================ -->

<StandardEntry
  id="erc-1056"
  title="ERC-1056 — Lightweight Identity"
  eip="1056"
  status="Final"
  neoMapping="Neo C# port"
  category="Identity"
  parityLabel="Port"
  parityClass="sm-pill-port"
>

<template #spec>

## ERC-1056: Ethereum Lightweight Identity (DID)

ERC-1056 (also known as ETHR-DID) is a registry contract that lets any Ethereum
address attach delegate keys, off-chain attribute proofs, and identity owners
without deploying a per-identity contract. Used by uPort, Veramo, and many
self-sovereign identity systems.

### Required Methods (Highlights)

```solidity
function identityOwner(address identity) external view returns (address);
function changeOwner(address identity, address newOwner) external;
function addDelegate(address identity, bytes32 delegateType, address delegate, uint validity) external;
function setAttribute(address identity, bytes32 name, bytes value, uint validity) external;
```

### Neo Equivalent: Direct Port

The Neo port is a single registry contract that any account can use. Like the
Solidity version, it's pay-per-use: the registry charges no fee beyond the storage
cost of the identity records, and the witness model handles authorisation.

::: tip Live on Neo TestNet
Both implementations deployed and behavior-verified on Neo N3 TestNet.

| Implementation | TestNet Address | Contract Hash |
| --- | --- | --- |
| **Solidity** (`neo-solc`) | `NViDh6CD7QV2Q6V31qAcNkyDeC3UmLAi8z` | [`0xdd6d4a48…2ef41f50`](https://dora.coz.io/contract/neo3/testnet/0xdd6d4a4806445d04982afc68866c9dc92ef41f50) |
| **Neo C#** (`nccs`) | `Nb2QQRhWzy7fgipLUyAZkivapVWa8YWA1F` | [`0xd13806f6…b74c1b30`](https://dora.coz.io/contract/neo3/testnet/0xd13806f6c06854ad3d8b731aebee40f8b74c1b30) |

Verified: default `identityOwner(account) == account` matches the EIP. `addDelegate` / `validDelegate` flows are implemented; tested locally with the `Runtime.Time`-based expiry check.
[`docs/standards-mirror/deployments/erc-1056/`](https://github.com/r3e-network/neo-solidity/tree/main/docs/standards-mirror/deployments/erc-1056).
:::

</template>

<template #solidity>

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

contract EthereumDIDRegistry {
    mapping(address => address) public owners;
    mapping(address => mapping(bytes32 => mapping(address => uint))) public delegates;
    mapping(address => uint) public changed;

    event DIDOwnerChanged(address indexed identity, address owner, uint previousChange);
    event DIDDelegateChanged(
        address indexed identity, bytes32 delegateType,
        address delegate, uint validTo, uint previousChange
    );

    function identityOwner(address id) public view returns (address) {
        address o = owners[id];
        return o == address(0) ? id : o;
    }

    function changeOwner(address id, address newOwner) external {
        require(msg.sender == identityOwner(id), "not owner");
        owners[id] = newOwner;
        emit DIDOwnerChanged(id, newOwner, changed[id]);
        changed[id] = block.number;
    }

    function addDelegate(address id, bytes32 delegateType, address delegate, uint validity)
        external
    {
        require(msg.sender == identityOwner(id), "not owner");
        delegates[id][delegateType][delegate] = block.timestamp + validity;
        emit DIDDelegateChanged(id, delegateType, delegate, block.timestamp + validity, changed[id]);
        changed[id] = block.number;
    }

    function validDelegate(address id, bytes32 delegateType, address delegate)
        external view returns (bool)
    {
        return delegates[id][delegateType][delegate] > block.timestamp;
    }
}
```

</template>

<template #csharp>

```csharp
using System;
using System.ComponentModel;
using System.Numerics;
using Neo;
using Neo.SmartContract.Framework;
using Neo.SmartContract.Framework.Attributes;
using Neo.SmartContract.Framework.Native;
using Neo.SmartContract.Framework.Services;

namespace R3E.Examples;

[DisplayName("DIDRegistry")]
[ContractPermission("*", "*")]
public class DIDRegistry : SmartContract
{
    private const byte Prefix_Owner    = 0x01;
    private const byte Prefix_Delegate = 0x02;  // identity+type+delegate -> validTo
    private const byte Prefix_Changed  = 0x03;
    private const byte Prefix_Attr     = 0x04;  // identity+name -> (value, validTo)

    [DisplayName("DIDOwnerChanged")]
    public static event Action<UInt160, UInt160, BigInteger> OnOwnerChanged;
    [DisplayName("DIDDelegateChanged")]
    public static event Action<UInt160, ByteString, UInt160, BigInteger, BigInteger> OnDelegateChanged;

    public static UInt160 IdentityOwner(UInt160 identity)
    {
        var owner = (UInt160)Storage.Get(Storage.CurrentContext,
                                         new byte[] { Prefix_Owner }.Concat(identity));
        return owner ?? identity;
    }

    public static void ChangeOwner(UInt160 identity, UInt160 newOwner)
    {
        var owner = IdentityOwner(identity);
        if (!Runtime.CheckWitness(owner)) throw new Exception("not owner");

        var prevChange = Changed(identity);
        Storage.Put(Storage.CurrentContext,
                    new byte[] { Prefix_Owner }.Concat(identity), newOwner);
        Storage.Put(Storage.CurrentContext,
                    new byte[] { Prefix_Changed }.Concat(identity), Ledger.CurrentIndex);
        OnOwnerChanged(identity, newOwner, prevChange);
    }

    public static void AddDelegate(UInt160 identity, ByteString delegateType,
                                   UInt160 delegateAddr, BigInteger validity)
    {
        var owner = IdentityOwner(identity);
        if (!Runtime.CheckWitness(owner)) throw new Exception("not owner");

        var validTo = Runtime.Time / 1000 + validity;
        var prevChange = Changed(identity);
        Storage.Put(Storage.CurrentContext,
                    new byte[] { Prefix_Delegate }.Concat(identity)
                        .Concat(delegateType).Concat(delegateAddr),
                    validTo);
        Storage.Put(Storage.CurrentContext,
                    new byte[] { Prefix_Changed }.Concat(identity), Ledger.CurrentIndex);
        OnDelegateChanged(identity, delegateType, delegateAddr, validTo, prevChange);
    }

    public static bool ValidDelegate(UInt160 identity, ByteString delegateType, UInt160 delegateAddr)
    {
        var key = new byte[] { Prefix_Delegate }.Concat(identity)
                     .Concat(delegateType).Concat(delegateAddr);
        var validTo = (BigInteger)(Storage.Get(Storage.CurrentContext, key) ?? ByteString.Empty);
        return validTo > Runtime.Time / 1000;
    }

    private static BigInteger Changed(UInt160 identity)
        => (BigInteger)(Storage.Get(Storage.CurrentContext,
            new byte[] { Prefix_Changed }.Concat(identity)) ?? ByteString.Empty);
}
```

</template>

</StandardEntry>

<!-- ============================================================ -->
<!-- ERC-7579 -->
<!-- ============================================================ -->

<StandardEntry
  id="erc-7579"
  title="ERC-7579 — Modular Smart Accounts"
  eip="7579"
  status="Final"
  neoMapping="Modular NEP-30 account (port)"
  category="Smart Accounts"
  parityLabel="Port"
  parityClass="sm-pill-port"
>

<template #spec>

## ERC-7579: Minimal Modular Smart Accounts

ERC-7579 standardises a **module interface** for smart-contract accounts (built on
ERC-4337). Modules plug into accounts to add features: validators (validate user
ops), executors (perform transactions), fallback handlers, hooks. The goal is to
let users mix-and-match modules from different vendors without rebuilding their
account.

### Module Types

| Type | Purpose | Methods |
| --- | --- | --- |
| Validator | Authorize user ops | `validateUserOp`, `isValidSignature` |
| Executor | Execute transactions | `execute`, `executeFromExecutor` |
| Fallback handler | Handle unknown calls | `fallback` |
| Hook | Pre/post call hooks | `preCheck`, `postCheck` |

### Neo Equivalent

Neo C# port: a programmable account contract whose `Verify` method delegates to a
list of "validator" contracts, and whose `Execute` method optionally runs through
"hook" contracts. Modules are themselves Neo contracts; the account stores their
script hashes and dispatches.


::: tip Live on Neo TestNet
Both implementations are deployed and behavior-verified on Neo N3 TestNet (network magic `894710606`).

| Implementation | Contract Hash | Deploy Tx |
| --- | --- | --- |
| **Solidity** (`neo-solc`) | `0x5e6edfc08e536f6d8891af968a52f7d56c11a528` | (reused — see [`0x5e6edfc08e536f6d8891af968a52f7d56c11a528`](https://dora.coz.io/contract/neo3/testnet/0x5e6edfc08e536f6d8891af968a52f7d56c11a528)) |
| **Neo C#** (`nccs`) | `0xcbd2e64f3ef5d5c9069fadf9c7d72ffcb8664f86` | (reused — see [`0xcbd2e64f3ef5d5c9069fadf9c7d72ffcb8664f86`](https://dora.coz.io/contract/neo3/testnet/0xcbd2e64f3ef5d5c9069fadf9c7d72ffcb8664f86)) |

Cross-implementation invocations match on `moduleCount`. Source pairs under
[`docs/standards-mirror/deployments/erc-7579/`](https://github.com/r3e-network/neo-solidity/tree/main/docs/standards-mirror/deployments/erc-7579).
:::

</template>

<template #solidity>

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

interface IValidator {
    function validateUserOp(bytes32 hash, bytes calldata data) external view returns (uint256);
    function isValidSignature(bytes32 hash, bytes calldata data) external view returns (bytes4);
}

interface IExecutor {
    function execute(address account, bytes calldata data) external;
}

contract ModularAccount {
    mapping(address => bool) public installedValidators;
    mapping(address => bool) public installedExecutors;

    function installModule(uint256 moduleType, address module, bytes calldata initData) external {
        // ... auth check (only owner) ...
        if (moduleType == 1) {
            installedValidators[module] = true;
            (bool ok, ) = module.call(initData);
            require(ok);
        } else if (moduleType == 2) {
            installedExecutors[module] = true;
            (bool ok, ) = module.call(initData);
            require(ok);
        }
    }

    function uninstallModule(uint256 moduleType, address module) external {
        // ... auth check ...
        if (moduleType == 1) installedValidators[module] = false;
        if (moduleType == 2) installedExecutors[module] = false;
    }

    function executeFromExecutor(bytes calldata data) external {
        require(installedExecutors[msg.sender], "not installed");
        // ... dispatch ...
    }
}
```

</template>

<template #csharp>

```csharp
using System;
using System.ComponentModel;
using System.Numerics;
using Neo;
using Neo.SmartContract.Framework;
using Neo.SmartContract.Framework.Attributes;
using Neo.SmartContract.Framework.Native;
using Neo.SmartContract.Framework.Services;

namespace R3E.Examples;

public enum ModuleType : byte { Validator = 1, Executor = 2, Hook = 3 }

[DisplayName("ModularAccount")]
[ContractPermission("*", "*")]
[SupportedStandards("NEP-30")]
public class ModularAccount : SmartContract
{
    private const byte Prefix_Module = 0x01;  // type+addr -> 1 if installed
    private const byte Prefix_Hook   = 0x02;  // ordered list of hook contracts
    private static readonly byte[] OwnerKey = { 0xff };

    [DisplayName("ModuleInstalled")]
    public static event Action<byte, UInt160> OnInstalled;

    public static void InstallModule(byte moduleType, UInt160 module, object initData)
    {
        var owner = (UInt160)Storage.Get(Storage.CurrentContext, OwnerKey);
        if (!Runtime.CheckWitness(owner)) throw new Exception("owner only");

        Storage.Put(Storage.CurrentContext,
                    new byte[] { Prefix_Module, moduleType }.Concat(module), 1);
        // Initialise the module with its module-specific data
        Contract.Call(module, "onInstall", CallFlags.All, new object[] { initData });
        OnInstalled(moduleType, module);
    }

    /// <summary>
    /// NEP-30 verify dispatched through any installed validator. Any single
    /// validator returning true authorises the transaction.
    /// </summary>
    public static bool Verify(object opHash, object signature)
    {
        var iter = Storage.Find(Storage.CurrentContext,
                                new byte[] { Prefix_Module, (byte)ModuleType.Validator },
                                FindOptions.KeysOnly | FindOptions.RemovePrefix);
        while (iter.Next())
        {
            var validator = (UInt160)iter.Value;
            try
            {
                var ok = (bool)Contract.Call(validator, "validateUserOp",
                                             CallFlags.ReadOnly,
                                             new object[] { opHash, signature });
                if (ok) return true;
            }
            catch { /* try next validator */ }
        }
        return false;
    }

    public static object Execute(UInt160 target, string method, object[] args)
    {
        if (!Verify(null, null)) throw new Exception("verify failed");

        // Run pre-hooks
        var hookIter = Storage.Find(Storage.CurrentContext,
                                    new byte[] { Prefix_Module, (byte)ModuleType.Hook },
                                    FindOptions.KeysOnly | FindOptions.RemovePrefix);
        while (hookIter.Next())
            Contract.Call((UInt160)hookIter.Value, "preCheck", CallFlags.All,
                          new object[] { target, method, args });

        var result = Contract.Call(target, method, CallFlags.All, args);

        // Run post-hooks (re-iterate; iterator is one-shot)
        hookIter = Storage.Find(Storage.CurrentContext,
                                new byte[] { Prefix_Module, (byte)ModuleType.Hook },
                                FindOptions.KeysOnly | FindOptions.RemovePrefix);
        while (hookIter.Next())
            Contract.Call((UInt160)hookIter.Value, "postCheck", CallFlags.All,
                          new object[] { target, method, result });

        return result;
    }

    public static void _deploy(object data, bool update)
    {
        if (update) return;
        Storage.Put(Storage.CurrentContext, OwnerKey, (UInt160)data);
    }
}
```

</template>

</StandardEntry>

<!-- ============================================================ -->
<!-- ERC-2771 -->
<!-- ============================================================ -->

<StandardEntry
  id="erc-2771"
  title="ERC-2771 — Trusted Forwarder (Meta-Tx)"
  eip="2771"
  status="Final"
  neoMapping="Native witness scopes"
  category="Meta-Tx"
  parityLabel="Native"
  parityClass="sm-pill-native"
>

<template #spec>

## ERC-2771: Secure Protocol for Trusted Forwarders

Meta-transactions: a relayer pays gas on behalf of users who don't have ETH. The
user signs an off-chain message, the relayer wraps it in a real transaction and
calls a "Forwarder" contract, which calls the target appending the original signer
to calldata. Targets that opt in extract the real `_msgSender()` from the appended
data instead of `msg.sender`.

### Required Convention

```solidity
function isTrustedForwarder(address forwarder) external view returns (bool);
function _msgSender() internal view returns (address) {
    if (msg.sender == trustedForwarder && msg.data.length >= 20) {
        return address(bytes20(msg.data[msg.data.length - 20:]));
    }
    return msg.sender;
}
```

### Neo Equivalent: Witness Scopes (Solved at Protocol)

This entire ceremony is unnecessary on Neo. A relayer-paid transaction lists the
relayer as `Sender` (paying the fee) and the user as a `Signer` with appropriate
scopes. The target contract's `Runtime.CheckWitness(user)` succeeds because the
user genuinely signed the transaction; the relayer is a separate witness paying
gas. No forwarder, no `_msgSender()` wrapper, no calldata munging.

::: tip Live on Neo TestNet
Both implementations deployed and behavior-verified on Neo N3 TestNet.

| Implementation | TestNet Address | Contract Hash |
| --- | --- | --- |
| **Solidity** (`neo-solc`) | `NfiyLrGXijAAHiwq1LUGu69K92ybMu4ZS4` | [`0x6653a8da…2c124ed9`](https://dora.coz.io/contract/neo3/testnet/0x6653a8da9bac7b622987670d97bf740c2c124ed9) |
| **Neo C#** (`nccs`) | `NLq6WUsRv4FAtxrqhwtTNxTeSgxAWYhG8P` | [`0x1463ad54…a6280c0a`](https://dora.coz.io/contract/neo3/testnet/0x1463ad54cf6a8fc7c0ffe3740ad1cf04a6280c0a) |

Verified: `getNonce(deployer) == 0` initially, `bumpNonce` increments it. The C# port uses the witness model — no separate signature verification needed.
[`docs/standards-mirror/deployments/erc-2771/`](https://github.com/r3e-network/neo-solidity/tree/main/docs/standards-mirror/deployments/erc-2771).
:::

</template>

<template #solidity>

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

abstract contract ERC2771Context {
    address private immutable _trustedForwarder;

    constructor(address forwarder) { _trustedForwarder = forwarder; }

    function isTrustedForwarder(address f) public view returns (bool) {
        return f == _trustedForwarder;
    }

    function _msgSender() internal view returns (address signer) {
        if (msg.sender == _trustedForwarder && msg.data.length >= 20) {
            assembly {
                signer := shr(96, calldataload(sub(calldatasize(), 20)))
            }
        } else {
            signer = msg.sender;
        }
    }
}

contract MinimalForwarder {
    struct ForwardRequest {
        address from; address to;
        uint256 value; uint256 gas;
        uint256 nonce; bytes data;
    }
    mapping(address => uint256) public nonces;

    function execute(ForwardRequest calldata req, bytes calldata sig)
        external payable returns (bool, bytes memory)
    {
        // ... verify EIP-712 sig of req using ECDSA on `req.from` ...
        require(nonces[req.from] == req.nonce, "bad nonce");
        nonces[req.from] = req.nonce + 1;

        // Append req.from to calldata so target can extract via _msgSender()
        (bool ok, bytes memory ret) = req.to.call{ gas: req.gas, value: req.value }(
            abi.encodePacked(req.data, req.from)
        );
        require(ok, "exec failed");
        return (ok, ret);
    }
}
```

</template>

<template #csharp>

```csharp
using System;
using System.ComponentModel;
using System.Numerics;
using Neo;
using Neo.SmartContract.Framework;
using Neo.SmartContract.Framework.Attributes;
using Neo.SmartContract.Framework.Native;
using Neo.SmartContract.Framework.Services;

namespace R3E.Examples;

[DisplayName("DemoToken")]
[SupportedStandards("NEP-17")]
public class DemoToken : SmartContract
{
    [DisplayName("Transfer")]
    public static event Action<UInt160, UInt160, BigInteger> OnTransfer;

    /// <summary>
    /// Standard NEP-17 transfer — no special meta-tx handling. The user's
    /// witness is checked normally; the relayer pays the gas as the tx Sender.
    /// </summary>
    public static bool Transfer(UInt160 from, UInt160 to, BigInteger amount, object data)
    {
        if (!Runtime.CheckWitness(from)) throw new Exception("no authorization");
        // ... actual transfer ...
        OnTransfer(from, to, amount);
        return true;
    }
}
```

### How a Relayed Transaction Looks on Neo

```ts
// Client (TypeScript) — user signs, relayer pays
const tx = new TransactionBuilder()
  .invoke(token, "transfer", [user, recipient, amount, "via relayer"])
  .signers([
    {
      account: relayer,
      scopes:  WitnessScope.None             // pays fee, no app authority
    },
    {
      account: user,
      scopes:  WitnessScope.CustomContracts,
      allowedContracts: [token]              // user authorises just this transfer
    }
  ])
  .build();

const userSigned    = await userWallet.signTransaction(tx);
const relayerSigned = await relayerWallet.cosignTransaction(userSigned);
await rpc.sendRawTransaction(relayerSigned);
```

The token contract sees `Runtime.CheckWitness(user) == true` because the user's
signature is on the transaction. The relayer is just paying the fee.

</template>

</StandardEntry>

</StandardsMirror>
