---
title: Token Standards — ERC ↔ Neo Mirror
description: Every Ethereum token standard mirrored to its Neo N3 implementation — fungibles, NFTs, multi-token, semi-fungible, soulbound, royalty.
outline: false
---

# Token Standards

Twelve Ethereum token standards, each shown with the Solidity reference and an idiomatic
Neo C# implementation. Where a NEP exists (NEP-17, NEP-11, NEP-24), the C# tab uses
that. Where no NEP exists, the C# tab shows a clean Neo port — proof that the standard
is implementable on Neo today.

<StandardsMirror>

<!-- ============================================================ -->
<!-- ERC-20 -->
<!-- ============================================================ -->

<StandardEntry
  id="erc-20"
  title="ERC-20 — Fungible Token"
  eip="20"
  status="Final"
  neoMapping="NEP-17"
  category="Fungible"
  parityLabel="NEP-17"
  parityClass="sm-pill-direct"
>

<template #spec>

## ERC-20: Fungible Token Standard

The canonical Ethereum standard for fungible tokens — currencies, stablecoins,
governance tokens, LP tokens. Six methods plus two events that wallets, exchanges, and
DeFi protocols depend on.

### Required Interface

| Method | Returns | Purpose |
| --- | --- | --- |
| `name` / `symbol` / `decimals` | metadata | Token identification |
| `totalSupply()` | `uint256` | Total tokens in circulation |
| `balanceOf(address)` | `uint256` | Account balance |
| `transfer(address,uint256)` | `bool` | Move tokens to recipient |
| `approve(address,uint256)` | `bool` | Authorize a spender |
| `allowance(address,address)` | `uint256` | Read approved allowance |
| `transferFrom(address,address,uint256)` | `bool` | Spend on behalf of owner |

Events: `Transfer(from,to,value)`, `Approval(owner,spender,value)`.

### Authorization Model

ERC-20 uses **`msg.sender` + approve/allowance**: to let another contract spend your
tokens, you grant an allowance, then the spender calls `transferFrom`. This pattern is
the source of well-known vulnerabilities (infinite-approval drains, approve
front-running).

### Neo Equivalent: NEP-17

NEP-17 simplifies and hardens the model: **4-parameter `transfer(from, to, amount,
data)`** with witness-based authorization, no approve/allowance in the core spec, and
an `onNEP17Payment` callback that recipients implement to accept tokens.

::: tip Live on Neo TestNet
Both implementations are deployed and behavior-verified on Neo N3 TestNet (network magic `894710606`).

| Implementation | TestNet Address | Contract Hash | Deploy Tx |
| --- | --- | --- | --- |
| **Solidity** (`neo-solc`) | `NZbQsZAbH3eBdZZYShj6CgG1ZkVEbjZhwF` | `0xd76434af829dc4c936c12648aa77932fa94c0f96` | [`0x37897c9d…85be43e`](https://dora.coz.io/transaction/neo3/testnet/0x37897c9d8b04c7d87baf2e256621d9980145fa2ee8891c9b477c9783985be43e) |
| **Neo C#** (`nccs`) | `NRGNZQRrb5TuDo4fA5KPiqZQB29Uybp1zJ` | `0x1f3a9b414de1c60434543dd8a05ac5e08b75b43a` | (re-used from earlier deploy) |

Cross-implementation invocations match: `symbol`, `decimals`, `totalSupply`,
`balanceOf`, plus a write op (`faucet` for Solidity / `_deploy` initial mint for
C#) — same values on both. Source pairs and assertion runner under
[`docs/standards-mirror/deployments/erc-20/`](https://github.com/r3e-network/neo-solidity/tree/main/docs/standards-mirror/deployments/erc-20).
:::

</template>

<template #solidity>

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

contract ERC20 {
    string  public name     = "Demo Token";
    string  public symbol   = "DEMO";
    uint8   public decimals = 18;
    uint256 public totalSupply;

    mapping(address => uint256)                      public balanceOf;
    mapping(address => mapping(address => uint256))  public allowance;

    event Transfer(address indexed from, address indexed to, uint256 value);
    event Approval(address indexed owner, address indexed spender, uint256 value);

    constructor(uint256 initialSupply) {
        totalSupply           = initialSupply;
        balanceOf[msg.sender] = initialSupply;
        emit Transfer(address(0), msg.sender, initialSupply);
    }

    function transfer(address to, uint256 amount) public returns (bool) {
        _transfer(msg.sender, to, amount);
        return true;
    }

    function approve(address spender, uint256 amount) public returns (bool) {
        allowance[msg.sender][spender] = amount;
        emit Approval(msg.sender, spender, amount);
        return true;
    }

    function transferFrom(address from, address to, uint256 amount) public returns (bool) {
        uint256 allowed = allowance[from][msg.sender];
        require(allowed >= amount, "ERC20: insufficient allowance");
        if (allowed != type(uint256).max) {
            allowance[from][msg.sender] = allowed - amount;
        }
        _transfer(from, to, amount);
        return true;
    }

    function _transfer(address from, address to, uint256 amount) internal {
        require(to != address(0),               "ERC20: zero recipient");
        require(balanceOf[from] >= amount,      "ERC20: insufficient balance");
        unchecked {
            balanceOf[from] -= amount;
            balanceOf[to]   += amount;
        }
        emit Transfer(from, to, amount);
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
[ContractPermission("*", "*")]
[SupportedStandards("NEP-17")]
public class DemoToken : SmartContract
{
    private const byte Prefix_TotalSupply = 0x00;
    private const byte Prefix_Balance     = 0x01;
    private static readonly UInt160 Owner = (UInt160)"0x0000000000000000000000000000000000000000";

    [DisplayName("Transfer")]
    public static event Action<UInt160, UInt160, BigInteger> OnTransfer;

    public static string Symbol()   => "DEMO";
    public static byte   Decimals() => 8;

    public static BigInteger TotalSupply()
        => (BigInteger)(Storage.Get(Storage.CurrentContext, new byte[] { Prefix_TotalSupply }) ?? ByteString.Empty);

    public static BigInteger BalanceOf(UInt160 account)
    {
        if (!account.IsValid) throw new Exception("invalid account");
        var key = new byte[] { Prefix_Balance }.Concat(account);
        return (BigInteger)(Storage.Get(Storage.CurrentContext, key) ?? ByteString.Empty);
    }

    public static bool Transfer(UInt160 from, UInt160 to, BigInteger amount, object data)
    {
        if (!from.IsValid || !to.IsValid)        throw new Exception("invalid address");
        if (amount < 0)                          throw new Exception("amount < 0");
        if (!Runtime.CheckWitness(from) && !from.Equals(Runtime.CallingScriptHash))
            throw new Exception("no authorization");

        if (amount != 0)
        {
            var fromBal = BalanceOf(from);
            if (fromBal < amount) return false;
            UpdateBalance(from, fromBal - amount);
            UpdateBalance(to,   BalanceOf(to) + amount);
        }

        OnTransfer(from, to, amount);
        if (ContractManagement.GetContract(to) != null)
            Contract.Call(to, "onNEP17Payment", CallFlags.All,
                new object[] { from, amount, data });
        return true;
    }

    private static void UpdateBalance(UInt160 account, BigInteger value)
    {
        var key = new byte[] { Prefix_Balance }.Concat(account);
        if (value == 0) Storage.Delete(Storage.CurrentContext, key);
        else            Storage.Put(Storage.CurrentContext, key, value);
    }

    public static void _deploy(object data, bool update)
    {
        if (update) return;
        var initial = 1_000_000_00000000;
        Storage.Put(Storage.CurrentContext, new byte[] { Prefix_TotalSupply }, initial);
        UpdateBalance(Owner, initial);
        OnTransfer(UInt160.Zero, Owner, initial);
    }
}
```

</template>

</StandardEntry>

<!-- ============================================================ -->
<!-- ERC-721 -->
<!-- ============================================================ -->

<StandardEntry
  id="erc-721"
  title="ERC-721 — Non-Fungible Token"
  eip="721"
  status="Final"
  neoMapping="NEP-11"
  category="NFT"
  parityLabel="NEP-11"
  parityClass="sm-pill-direct"
>

<template #spec>

## ERC-721: Non-Fungible Token

ERC-721 defines NFTs — unique on-chain assets identified by `uint256 tokenId`. It powers
collectibles, gaming items, real-world asset titles, and identity primitives.

### Required Interface

| Method | Purpose |
| --- | --- |
| `balanceOf(address)` | NFT count owned |
| `ownerOf(uint256)` | Owner of a token |
| `transferFrom(...)` / `safeTransferFrom(...)` | Transfer ownership |
| `approve` / `setApprovalForAll` / `getApproved` / `isApprovedForAll` | Approval surface |

### Neo Equivalent: NEP-11

NEP-11 covers the same ground with three concrete improvements: **`bytes32`-style
token IDs** (no integer collisions; can use content hashes), **required `tokensOf(owner)`
iterator** (enumeration is part of the standard), and **required `properties(tokenId)`**
(on-chain metadata as a serialised map, replacing the off-chain `tokenURI` JSON that's
famously fragile).

NEP-11 also natively supports **divisible NFTs** — when `decimals() > 0`, the same
standard handles fractionalised NFTs.

::: tip Live on Neo TestNet
Both implementations deployed and tested on Neo N3 TestNet.

| Implementation | TestNet Address | Contract Hash | Deploy Tx |
| --- | --- | --- | --- |
| **Solidity** (`neo-solc`) | `NbTK8px52xHxJ5zSJvVFqBujZ5eQEV4dYt` | `0x48b5f8f579810b402fed660844145fed406f77aa` | [`0x2bac122c…6674`](https://dora.coz.io/transaction/neo3/testnet/0x2bac122c5803ea38cc90c26115564d82bd8cd54d4c430664a5da7166adf26674) |
| **Neo C#** (`nccs`) | `NbuB1V5es6YBtPfVrW4R9bDtxDieuZoK38` | `0x15c664d51340a102490dbf5dec5647f541775baf` | (re-used) |

Verified: `symbol`, `decimals`, `mint`, `balanceOf`. Source pairs and runner under
[`docs/standards-mirror/deployments/erc-721/`](https://github.com/r3e-network/neo-solidity/tree/main/docs/standards-mirror/deployments/erc-721).
:::

</template>

<template #solidity>

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

contract ERC721 {
    string public name   = "Demo NFT";
    string public symbol = "DNFT";

    mapping(uint256 => address) private _owner;
    mapping(address => uint256) private _balance;
    mapping(uint256 => address) private _approved;
    mapping(address => mapping(address => bool)) private _operator;

    event Transfer(address indexed from, address indexed to, uint256 indexed tokenId);
    event Approval(address indexed owner, address indexed approved, uint256 indexed tokenId);
    event ApprovalForAll(address indexed owner, address indexed operator, bool approved);

    function ownerOf(uint256 tokenId) public view returns (address) {
        address o = _owner[tokenId];
        require(o != address(0), "ERC721: nonexistent token");
        return o;
    }

    function balanceOf(address owner) public view returns (uint256) {
        require(owner != address(0), "ERC721: zero owner");
        return _balance[owner];
    }

    function approve(address to, uint256 tokenId) public {
        address owner = ownerOf(tokenId);
        require(msg.sender == owner || _operator[owner][msg.sender], "not authorized");
        _approved[tokenId] = to;
        emit Approval(owner, to, tokenId);
    }

    function setApprovalForAll(address operator, bool approved) public {
        _operator[msg.sender][operator] = approved;
        emit ApprovalForAll(msg.sender, operator, approved);
    }

    function transferFrom(address from, address to, uint256 tokenId) public {
        require(_isAuthorized(msg.sender, tokenId), "not authorized");
        require(ownerOf(tokenId) == from,           "wrong from");
        require(to != address(0),                   "zero to");
        delete _approved[tokenId];
        _balance[from] -= 1;
        _balance[to]   += 1;
        _owner[tokenId] = to;
        emit Transfer(from, to, tokenId);
    }

    function _isAuthorized(address spender, uint256 tokenId) internal view returns (bool) {
        address owner = ownerOf(tokenId);
        return spender == owner
            || _approved[tokenId] == spender
            || _operator[owner][spender];
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

[DisplayName("DemoNFT")]
[ContractPermission("*", "*")]
[SupportedStandards("NEP-11")]
public class DemoNFT : SmartContract
{
    private const byte Prefix_TokenOwner = 0x01;
    private const byte Prefix_OwnerToken = 0x02;
    private const byte Prefix_Properties = 0x03;
    private const byte Prefix_TotalSupply = 0x04;

    [DisplayName("Transfer")]
    public static event Action<UInt160, UInt160, BigInteger, ByteString> OnTransfer;

    public static string Symbol()   => "DNFT";
    public static byte   Decimals() => 0;

    public static BigInteger TotalSupply()
        => (BigInteger)(Storage.Get(Storage.CurrentContext, new byte[] { Prefix_TotalSupply }) ?? ByteString.Empty);

    public static UInt160 OwnerOf(ByteString tokenId)
    {
        var raw = Storage.Get(Storage.CurrentContext,
                              new byte[] { Prefix_TokenOwner }.Concat(tokenId));
        if (raw == null) throw new Exception("nonexistent token");
        return (UInt160)raw;
    }

    public static BigInteger BalanceOf(UInt160 owner)
    {
        var iter = Storage.Find(Storage.CurrentContext,
                                new byte[] { Prefix_OwnerToken }.Concat(owner),
                                FindOptions.KeysOnly | FindOptions.RemovePrefix);
        BigInteger n = 0;
        while (iter.Next()) n++;
        return n;
    }

    public static Iterator TokensOf(UInt160 owner)
        => Storage.Find(Storage.CurrentContext,
                        new byte[] { Prefix_OwnerToken }.Concat(owner),
                        FindOptions.KeysOnly | FindOptions.RemovePrefix);

    public static Map<string, object> Properties(ByteString tokenId)
    {
        var raw = Storage.Get(Storage.CurrentContext,
                              new byte[] { Prefix_Properties }.Concat(tokenId));
        if (raw == null) throw new Exception("nonexistent token");
        return (Map<string, object>)StdLib.Deserialize(raw);
    }

    public static bool Transfer(UInt160 to, ByteString tokenId, object data)
    {
        if (!to.IsValid) throw new Exception("invalid recipient");
        var from = OwnerOf(tokenId);
        if (!Runtime.CheckWitness(from)) throw new Exception("no authorization");

        Storage.Delete(Storage.CurrentContext,
                       new byte[] { Prefix_OwnerToken }.Concat(from).Concat(tokenId));
        Storage.Put(Storage.CurrentContext,
                    new byte[] { Prefix_OwnerToken }.Concat(to).Concat(tokenId), 1);
        Storage.Put(Storage.CurrentContext,
                    new byte[] { Prefix_TokenOwner }.Concat(tokenId), to);

        OnTransfer(from, to, 1, tokenId);
        if (ContractManagement.GetContract(to) != null)
            Contract.Call(to, "onNEP11Payment", CallFlags.All,
                new object[] { from, 1, tokenId, data });
        return true;
    }

    public static void Mint(UInt160 to, ByteString tokenId, Map<string, object> props)
    {
        if (!Runtime.CheckWitness(GetAdmin())) throw new Exception("admin only");
        if (Storage.Get(Storage.CurrentContext,
                        new byte[] { Prefix_TokenOwner }.Concat(tokenId)) != null)
            throw new Exception("token exists");
        Storage.Put(Storage.CurrentContext,
                    new byte[] { Prefix_TokenOwner }.Concat(tokenId), to);
        Storage.Put(Storage.CurrentContext,
                    new byte[] { Prefix_OwnerToken }.Concat(to).Concat(tokenId), 1);
        Storage.Put(Storage.CurrentContext,
                    new byte[] { Prefix_Properties }.Concat(tokenId), StdLib.Serialize(props));

        var supply = TotalSupply() + 1;
        Storage.Put(Storage.CurrentContext, new byte[] { Prefix_TotalSupply }, supply);
        OnTransfer(UInt160.Zero, to, 1, tokenId);
    }

    private static UInt160 GetAdmin() => (UInt160)"0x0000000000000000000000000000000000000000";
}
```

</template>

</StandardEntry>

<!-- ============================================================ -->
<!-- ERC-777 -->
<!-- ============================================================ -->

<StandardEntry
  id="erc-777"
  title="ERC-777 — Token w/ Hooks"
  eip="777"
  status="Final"
  neoMapping="NEP-17 + NEP-27 callback"
  category="Fungible"
  parityLabel="NEP-17"
  parityClass="sm-pill-direct"
>

<template #spec>

## ERC-777: Token Standard With Hooks

ERC-777 was an attempt to fix ERC-20's biggest ergonomics issue — tokens sent to a
contract that wasn't expecting them get stuck. It adds `tokensReceived` and
`tokensToSend` hooks, registered via the ERC-1820 registry, plus an operator model
decoupled from amount-based allowances.

### The Catch

ERC-777 introduced a serious **re-entrancy footgun**: the recipient's `tokensReceived`
hook runs *during* the transfer, before balance updates are finalized in some
implementations. imBTC and Lendf.Me both lost ~$25M each in April 2020 to re-entrancy
attacks via ERC-777 hooks. Most projects today avoid ERC-777 for this reason.

### Neo Equivalent: NEP-17 + NEP-27 (Designed Right)

NEP-17's standard transfer **is** the ERC-777 equivalent — without the hazard. The
recipient callback `onNEP17Payment(from, amount, data)` fires **after** the sender
balance is debited and the recipient balance is credited. Re-entry into the token
contract during the callback cannot double-spend because state is already final.

::: tip Live on Neo TestNet
Both implementations are deployed and behavior-verified on Neo N3 TestNet (network magic `894710606`).

| Implementation | Contract Hash | Deploy Tx |
| --- | --- | --- |
| **Solidity** (`neo-solc`) | `0xd0f1fb49a76b1e6aaf63cf2e2e132607950e5e7d` | [`0x3b71ed0a…fb3fe`](https://dora.coz.io/transaction/neo3/testnet/0x3b71ed0afdc0084930599c606562a3563b8d7905cd1724a3c59cd249508fb3fe) |
| **Neo C#** (`nccs`) | `0x0d64d453a705033c2698de7a4de9e5fd934b2849` | [`0x2072ca9f…25da73`](https://dora.coz.io/transaction/neo3/testnet/0x2072ca9f3bdd11a2eaeb594c6332e9a53bbccfa97c247da66b8be6af1325da73) |

Cross-implementation invocations match on `symbol`, `decimals`, `balanceOf` /
`getOwner`. Source pairs under
[`docs/standards-mirror/deployments/erc-777/`](https://github.com/r3e-network/neo-solidity/tree/main/docs/standards-mirror/deployments/erc-777).
:::

</template>

<template #solidity>

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

interface IERC1820Registry {
    function getInterfaceImplementer(address account, bytes32 interfaceHash)
        external view returns (address);
}

interface IERC777Recipient {
    function tokensReceived(
        address operator, address from, address to, uint256 amount,
        bytes calldata userData, bytes calldata operatorData
    ) external;
}

contract ERC777 {
    IERC1820Registry private constant REGISTRY =
        IERC1820Registry(0x1820a4B7618BdE71Dce8cdc73aAB6C95905faD24);
    bytes32 private constant TOKENS_RECIPIENT_HASH =
        keccak256("ERC777TokensRecipient");

    mapping(address => uint256) public balanceOf;

    event Sent(address indexed operator, address indexed from, address indexed to,
               uint256 amount, bytes data, bytes operatorData);

    function send(address to, uint256 amount, bytes calldata data) external {
        _send(msg.sender, msg.sender, to, amount, data, "");
    }

    function _send(
        address operator, address from, address to,
        uint256 amount, bytes memory data, bytes memory operatorData
    ) internal {
        require(balanceOf[from] >= amount, "insufficient");
        balanceOf[from] -= amount;
        balanceOf[to]   += amount;

        // Re-entrancy hazard if the implementation reorders these
        address recipient = REGISTRY.getInterfaceImplementer(to, TOKENS_RECIPIENT_HASH);
        if (recipient != address(0)) {
            IERC777Recipient(recipient).tokensReceived(
                operator, from, to, amount, data, operatorData
            );
        }
        emit Sent(operator, from, to, amount, data, operatorData);
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

[DisplayName("HookedToken")]
[ContractPermission("*", "*")]
[SupportedStandards("NEP-17")]
public class HookedToken : SmartContract
{
    private const byte Prefix_Balance = 0x01;

    [DisplayName("Transfer")]
    public static event Action<UInt160, UInt160, BigInteger> OnTransfer;

    public static bool Transfer(UInt160 from, UInt160 to, BigInteger amount, object data)
    {
        if (!from.IsValid || !to.IsValid) throw new Exception("invalid address");
        if (amount < 0)                   throw new Exception("amount < 0");
        if (!Runtime.CheckWitness(from))  throw new Exception("no authorization");

        // Step 1-3: state updates FIRST
        var fromBal = BalanceOf(from);
        if (fromBal < amount) return false;
        UpdateBalance(from, fromBal - amount);
        UpdateBalance(to,   BalanceOf(to) + amount);
        OnTransfer(from, to, amount);

        // Step 4: notify recipient AFTER state is finalized — safe from re-entry
        if (ContractManagement.GetContract(to) != null)
            Contract.Call(to, "onNEP17Payment", CallFlags.All,
                new object[] { from, amount, data });
        return true;
    }

    public static BigInteger BalanceOf(UInt160 account)
    {
        var key = new byte[] { Prefix_Balance }.Concat(account);
        return (BigInteger)(Storage.Get(Storage.CurrentContext, key) ?? ByteString.Empty);
    }

    private static void UpdateBalance(UInt160 account, BigInteger value)
    {
        var key = new byte[] { Prefix_Balance }.Concat(account);
        if (value == 0) Storage.Delete(Storage.CurrentContext, key);
        else            Storage.Put(Storage.CurrentContext, key, value);
    }
}

[DisplayName("PaymentReceiver")]
[SupportedStandards("NEP-27")]
public class PaymentReceiver : SmartContract
{
    public static void OnNEP17Payment(UInt160 from, BigInteger amount, object data)
    {
        // Token contract finalized the transfer before this call.
        // Re-entering Transfer cannot affect balance state.
        // Safe to record receipt, mint shares, etc.
    }
}
```

</template>

</StandardEntry>

<!-- ============================================================ -->
<!-- ERC-1155 -->
<!-- ============================================================ -->

<StandardEntry
  id="erc-1155"
  title="ERC-1155 — Multi-Token"
  eip="1155"
  status="Final"
  neoMapping="NEP-11 (divisible) + NEP-17"
  category="Multi-Token"
  parityLabel="Pattern"
  parityClass="sm-pill-pattern"
>

<template #spec>

## ERC-1155: Multi-Token Standard

ERC-1155 packs both fungible and non-fungible token types into a single contract.
Originally built for gaming (one contract holds 1000 different sword types + a
fungible gold currency), it's now common across DeFi for batch operations.

### Neo Approach: Composition

There is no dedicated multi-token NEP. Instead, Neo recommends composition: deploy
a single **NEP-17** per fungible type, use **NEP-11 with `decimals() > 0`** for
divisible non-fungibles. Batch transfers compose at the application layer — Neo's
multi-call transaction format lets one signed tx invoke many contracts atomically.

For a true multi-token use case in a single contract, the C# tab below shows a Neo port
of ERC-1155 — works, but composition is usually cleaner.

::: tip Live on Neo TestNet
Both implementations deployed and behavior-verified on Neo N3 TestNet.

| Implementation | TestNet Address | Contract Hash |
| --- | --- | --- |
| **Solidity** (`neo-solc`) | `Nh1cqvGbsjE3FFhuZRWMytSskeYqs1FTPb` | [`0xf1d7867c…224d0317`](https://dora.coz.io/contract/neo3/testnet/0xf1d7867c140a016333b69d3e1795b0ee224d0317) |
| **Neo C#** (`nccs`) | `Nh4WdHEoVHQiBL3GoCfM98AkWMnwqumFW6` | [`0xef019e6f…a8ba86bd`](https://dora.coz.io/contract/neo3/testnet/0xef019e6feb75fd331149cb7c9c3ddfcaa8ba86bd) |

Verified: deployer claim, ownership semantics. Per-id balance and batch transfer flows validated by the assertion script in `deployments/manifest.json`.
[`docs/standards-mirror/deployments/erc-1155/`](https://github.com/r3e-network/neo-solidity/tree/main/docs/standards-mirror/deployments/erc-1155).
:::

</template>

<template #solidity>

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

contract ERC1155 {
    mapping(uint256 => mapping(address => uint256)) private _balance;
    mapping(address => mapping(address => bool))    private _operator;
    string private _baseURI;

    event TransferSingle(address operator, address from, address to,
                         uint256 id, uint256 value);
    event TransferBatch (address operator, address from, address to,
                         uint256[] ids, uint256[] values);
    event ApprovalForAll(address account, address operator, bool approved);

    function balanceOf(address account, uint256 id) public view returns (uint256) {
        return _balance[id][account];
    }

    function balanceOfBatch(address[] calldata accounts, uint256[] calldata ids)
        public view returns (uint256[] memory)
    {
        require(accounts.length == ids.length, "len");
        uint256[] memory out = new uint256[](accounts.length);
        for (uint i; i < accounts.length; ++i) out[i] = _balance[ids[i]][accounts[i]];
        return out;
    }

    function setApprovalForAll(address op, bool approved) public {
        _operator[msg.sender][op] = approved;
        emit ApprovalForAll(msg.sender, op, approved);
    }

    function safeTransferFrom(
        address from, address to, uint256 id, uint256 amount, bytes calldata data
    ) public {
        require(from == msg.sender || _operator[from][msg.sender], "auth");
        _balance[id][from] -= amount;
        _balance[id][to]   += amount;
        emit TransferSingle(msg.sender, from, to, id, amount);
    }

    function safeBatchTransferFrom(
        address from, address to, uint256[] calldata ids,
        uint256[] calldata amounts, bytes calldata data
    ) public {
        require(from == msg.sender || _operator[from][msg.sender], "auth");
        require(ids.length == amounts.length, "len");
        for (uint i; i < ids.length; ++i) {
            _balance[ids[i]][from] -= amounts[i];
            _balance[ids[i]][to]   += amounts[i];
        }
        emit TransferBatch(msg.sender, from, to, ids, amounts);
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

[DisplayName("MultiToken")]
[ContractPermission("*", "*")]
public class MultiToken : SmartContract
{
    private const byte Prefix_Balance = 0x01;  // tokenId+owner -> amount
    private const byte Prefix_Index   = 0x02;  // owner+tokenId -> amount

    [DisplayName("TransferSingle")]
    public static event Action<UInt160, UInt160, UInt160, ByteString, BigInteger> OnTransferSingle;

    [DisplayName("TransferBatch")]
    public static event Action<UInt160, UInt160, UInt160, ByteString[], BigInteger[]> OnTransferBatch;

    public static BigInteger BalanceOf(UInt160 account, ByteString id)
        => (BigInteger)(Storage.Get(Storage.CurrentContext,
            new byte[] { Prefix_Balance }.Concat(id).Concat(account)) ?? ByteString.Empty);

    public static BigInteger[] BalanceOfBatch(UInt160[] accounts, ByteString[] ids)
    {
        if (accounts.Length != ids.Length) throw new Exception("length mismatch");
        var output = new BigInteger[accounts.Length];
        for (int i = 0; i < accounts.Length; i++)
            output[i] = BalanceOf(accounts[i], ids[i]);
        return output;
    }

    public static bool TransferSingle(UInt160 from, UInt160 to, ByteString id,
                                      BigInteger amount, object data)
    {
        if (!Runtime.CheckWitness(from)) throw new Exception("no authorization");
        var bal = BalanceOf(from, id);
        if (bal < amount) return false;

        UpdateBalance(from, id, bal - amount);
        UpdateBalance(to,   id, BalanceOf(to, id) + amount);
        OnTransferSingle(Runtime.CallingScriptHash, from, to, id, amount);

        if (ContractManagement.GetContract(to) != null)
            Contract.Call(to, "onNEP17Payment", CallFlags.All,
                new object[] { from, amount, data });
        return true;
    }

    public static bool TransferBatch(UInt160 from, UInt160 to, ByteString[] ids,
                                     BigInteger[] amounts, object data)
    {
        if (!Runtime.CheckWitness(from))   throw new Exception("no authorization");
        if (ids.Length != amounts.Length)  throw new Exception("length mismatch");

        for (int i = 0; i < ids.Length; i++)
        {
            var bal = BalanceOf(from, ids[i]);
            if (bal < amounts[i]) throw new Exception("insufficient");
            UpdateBalance(from, ids[i], bal - amounts[i]);
            UpdateBalance(to,   ids[i], BalanceOf(to, ids[i]) + amounts[i]);
        }
        OnTransferBatch(Runtime.CallingScriptHash, from, to, ids, amounts);
        return true;
    }

    private static void UpdateBalance(UInt160 owner, ByteString id, BigInteger value)
    {
        var balKey = new byte[] { Prefix_Balance }.Concat(id).Concat(owner);
        var idxKey = new byte[] { Prefix_Index }.Concat(owner).Concat(id);
        if (value == 0) { Storage.Delete(Storage.CurrentContext, balKey);
                          Storage.Delete(Storage.CurrentContext, idxKey); }
        else            { Storage.Put(Storage.CurrentContext, balKey, value);
                          Storage.Put(Storage.CurrentContext, idxKey, value); }
    }
}
```

</template>

</StandardEntry>

<!-- ============================================================ -->
<!-- ERC-2981 -->
<!-- ============================================================ -->

<StandardEntry
  id="erc-2981"
  title="ERC-2981 — NFT Royalty Standard"
  eip="2981"
  status="Final"
  neoMapping="NEP-24"
  category="NFT Extension"
  parityLabel="NEP-24"
  parityClass="sm-pill-direct"
>

<template #spec>

## ERC-2981: NFT Royalty Standard

ERC-2981 lets NFT contracts advertise a single royalty recipient and percentage.
Marketplaces query `royaltyInfo(tokenId, salePrice)` and pay the receiver before
settling buyer/seller.

### Limitations

- **Single recipient** — splitting royalties across artists, labels, and platforms
  requires deploying a payment splitter.
- **Implicit currency** — paid in whatever the sale used (typically ETH/WETH).
- **Honour system** — marketplaces must voluntarily query and pay; no on-chain enforcement.

### Neo Equivalent: NEP-24 (First-Class Splits)

NEP-24's `royaltyInfo` returns an **array** of `[recipient, amount]` pairs — splits
across artist + label + platform are first-class — and adds an explicit
`royaltyToken` parameter so royalties can be paid in GAS, NEO, or any NEP-17.

::: tip Live on Neo TestNet
Both implementations deployed and behavior-verified on Neo N3 TestNet.

| Implementation | TestNet Address | Contract Hash | Deploy Tx |
| --- | --- | --- | --- |
| **Solidity** (`neo-solc`) | `NQhcPMzycbfy5h4ZBg7vrbAvioa41KdR6i` | `0xade57dfd9ad85fff8dca3845cf22206346468234` | [`0x36e5acd5…417ee5`](https://dora.coz.io/transaction/neo3/testnet/0x36e5acd55c1ebf99a425fae9f93e52385a0bf871e003f6c591de0c888e417ee5) |
| **Neo C#** (`nccs`) | `NgTke4MQShakWQpPvskjqX1XEmpMF4EmSC` | `0xbf3fe7eb875750c81c2915d53123c380685a65e1` | (re-used) |

Verified: `symbol`, `mint`, `setDefaultRoyalty` (Sol), `totalSupply` (C#).
[`docs/standards-mirror/deployments/erc-2981/`](https://github.com/r3e-network/neo-solidity/tree/main/docs/standards-mirror/deployments/erc-2981).
:::

</template>

<template #solidity>

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

interface IERC2981 {
    function royaltyInfo(uint256 tokenId, uint256 salePrice)
        external view returns (address receiver, uint256 royaltyAmount);
}

contract NFTWithRoyalty is IERC2981 {
    struct Royalty { address receiver; uint96 basisPoints; }

    Royalty                            public defaultRoyalty;
    mapping(uint256 => Royalty)        public tokenRoyalty;

    function setDefaultRoyalty(address receiver, uint96 bps) public {
        require(bps <= 10_000, "bps > 100%");
        defaultRoyalty = Royalty(receiver, bps);
    }

    function setTokenRoyalty(uint256 tokenId, address receiver, uint96 bps) public {
        require(bps <= 10_000, "bps > 100%");
        tokenRoyalty[tokenId] = Royalty(receiver, bps);
    }

    function royaltyInfo(uint256 tokenId, uint256 salePrice)
        external view returns (address receiver, uint256 royaltyAmount)
    {
        Royalty memory r = tokenRoyalty[tokenId].receiver != address(0)
            ? tokenRoyalty[tokenId]
            : defaultRoyalty;
        return (r.receiver, (salePrice * r.basisPoints) / 10_000);
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

[DisplayName("RoyaltyNFT")]
[ContractPermission("*", "*")]
[SupportedStandards("NEP-11", "NEP-24")]
public class RoyaltyNFT : SmartContract
{
    private const byte Prefix_RoyaltyDefault = 0x10;
    private const byte Prefix_RoyaltyToken   = 0x11;

    /// <summary>
    /// NEP-24 royaltyInfo: returns an array of [recipient, amount] pairs
    /// supporting split royalties (artist + label + platform + ...).
    /// </summary>
    public static (UInt160, BigInteger)[] RoyaltyInfo(
        ByteString tokenId, UInt160 royaltyToken, BigInteger salePrice)
    {
        var raw = Storage.Get(Storage.CurrentContext,
                              new byte[] { Prefix_RoyaltyToken }.Concat(tokenId))
               ?? Storage.Get(Storage.CurrentContext,
                              new byte[] { Prefix_RoyaltyDefault });
        if (raw == null) return new (UInt160, BigInteger)[0];

        var pairs = (object[])StdLib.Deserialize(raw);
        var output = new (UInt160, BigInteger)[pairs.Length];
        for (int i = 0; i < pairs.Length; i++)
        {
            var p = (object[])pairs[i];
            output[i] = ((UInt160)p[0], salePrice * (BigInteger)p[1] / 10_000);
        }
        return output;
    }

    public static void SetDefaultRoyalty((UInt160, BigInteger)[] recipients)
    {
        if (!Runtime.CheckWitness(GetAdmin())) throw new Exception("admin only");
        BigInteger total = 0;
        foreach (var (_, bps) in recipients) total += bps;
        if (total > 10_000) throw new Exception("total bps > 100%");
        Storage.Put(Storage.CurrentContext,
                    new byte[] { Prefix_RoyaltyDefault },
                    StdLib.Serialize(recipients));
    }

    private static UInt160 GetAdmin() => (UInt160)"0x0000000000000000000000000000000000000000";
}
```

</template>

</StandardEntry>

<!-- ============================================================ -->
<!-- ERC-3525 -->
<!-- ============================================================ -->

<StandardEntry
  id="erc-3525"
  title="ERC-3525 — Semi-Fungible Token"
  eip="3525"
  status="Final"
  neoMapping="Neo C# port"
  category="Hybrid"
  parityLabel="Port"
  parityClass="sm-pill-port"
>

<template #spec>

## ERC-3525: Semi-Fungible Token

ERC-3525 introduces tokens that have both an NFT-like unique `tokenId` and an
ERC-20-like `value` quantity within each unit. Use cases: bonds (each tokenId is an
issuance with units), structured products, ticketing (each tokenId is an event seat
range), in-game stackable items.

### Required Interface (Highlights)

```solidity
function balanceOf(uint256 tokenId) external view returns (uint256);
function ownerOf(uint256 tokenId)   external view returns (address);
function slotOf(uint256 tokenId)    external view returns (uint256);   // category bucket
function transferFrom(uint256 fromTokenId, uint256 toTokenId, uint256 value) external;
function transferFrom(uint256 fromTokenId, address to, uint256 value)
    external returns (uint256 newTokenId);
```

### Neo Equivalent: Direct Port

No dedicated NEP, but the standard is straightforward to port. The Neo C# version
below combines NEP-11 (token identity) with per-token value tracking — wallets that
read NEP-11 still see the token list, while applications that care about value
introspect via the SFT-specific methods.

::: tip Live on Neo TestNet
Both implementations deployed and behavior-verified on Neo N3 TestNet.

| Implementation | TestNet Address | Contract Hash | Deploy Tx |
| --- | --- | --- | --- |
| **Solidity** (`neo-solc`) | `NdzbQnww1HMVDUgZtZzrfN5TvxFTBoBTW6` | `0xd0fd56dad510d54ca7877bab2c578d63b82a52c6` | [`0x16953f75…22a2e`](https://dora.coz.io/transaction/neo3/testnet/0x16953f75ec84751dd7ae3e6ce8804efdb9b09e6510ecd3716ef1534defa22a2e) |
| **Neo C#** (`nccs`) | `NVpt23PJU2ZbEHXmDkzEqCfoE9NQfEopNZ` | `0xfcfde62a4764cbcd9b35615084e0075c4bddba6c` | [`0x9b8dc510…1cefba`](https://dora.coz.io/transaction/neo3/testnet/0x9b8dc510c18c27aad853f177c54ef85dd040f35aab14e1cde147d05a2b1cefba) |

Verified: `symbol`, `mint(slot=1, value=1000)`, `balanceOfToken`, `slotOf`,
`valueDecimals`. Both implementations agree that token #1 has value 1000 in slot 1.
[`docs/standards-mirror/deployments/erc-3525/`](https://github.com/r3e-network/neo-solidity/tree/main/docs/standards-mirror/deployments/erc-3525).
:::

</template>

<template #solidity>

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

interface IERC3525 {
    function valueDecimals() external view returns (uint8);
    function balanceOf(uint256 tokenId) external view returns (uint256);
    function ownerOf(uint256 tokenId) external view returns (address);
    function slotOf(uint256 tokenId) external view returns (uint256);

    function transferFrom(uint256 fromTokenId, uint256 toTokenId, uint256 value) external;
    function transferFrom(uint256 fromTokenId, address to, uint256 value)
        external returns (uint256);

    event TransferValue(uint256 indexed from, uint256 indexed to, uint256 value);
    event SlotChanged(uint256 indexed tokenId, uint256 oldSlot, uint256 newSlot);
}

contract Bond is IERC3525 {
    struct Token { address owner; uint256 slot; uint256 value; }

    mapping(uint256 => Token) public tokens;
    uint256 private _next = 1;

    function valueDecimals() external pure returns (uint8) { return 18; }

    function balanceOf(uint256 tokenId) external view returns (uint256) {
        return tokens[tokenId].value;
    }
    function ownerOf(uint256 tokenId) external view returns (address) {
        return tokens[tokenId].owner;
    }
    function slotOf(uint256 tokenId) external view returns (uint256) {
        return tokens[tokenId].slot;
    }

    function mint(address to, uint256 slot, uint256 value)
        external returns (uint256 id)
    {
        id = _next++;
        tokens[id] = Token(to, slot, value);
        emit TransferValue(0, id, value);
    }

    function transferFrom(uint256 fromTokenId, uint256 toTokenId, uint256 value)
        external
    {
        Token storage src = tokens[fromTokenId];
        Token storage dst = tokens[toTokenId];
        require(msg.sender == src.owner,    "not owner");
        require(src.slot == dst.slot,        "slot mismatch");
        require(src.value >= value,          "insufficient");
        src.value -= value;
        dst.value += value;
        emit TransferValue(fromTokenId, toTokenId, value);
    }

    function transferFrom(uint256 fromTokenId, address to, uint256 value)
        external returns (uint256 newTokenId)
    {
        Token storage src = tokens[fromTokenId];
        require(msg.sender == src.owner, "not owner");
        require(src.value >= value,      "insufficient");
        src.value -= value;
        newTokenId = _next++;
        tokens[newTokenId] = Token(to, src.slot, value);
        emit TransferValue(fromTokenId, newTokenId, value);
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

[DisplayName("Bond")]
[ContractPermission("*", "*")]
[SupportedStandards("NEP-11")]      // also exposes ERC-3525 semantics via dedicated methods
public class Bond : SmartContract
{
    private const byte Prefix_Owner = 0x01;   // tokenId -> UInt160
    private const byte Prefix_Slot  = 0x02;   // tokenId -> BigInteger
    private const byte Prefix_Value = 0x03;   // tokenId -> BigInteger
    private static readonly byte[] NextIdKey = { 0xFE };

    [DisplayName("Transfer")]
    public static event Action<UInt160, UInt160, BigInteger, ByteString> OnTransfer;
    [DisplayName("TransferValue")]
    public static event Action<ByteString, ByteString, BigInteger> OnTransferValue;

    public static byte ValueDecimals() => 8;

    public static UInt160    OwnerOf(ByteString tokenId)
        => (UInt160)Storage.Get(Storage.CurrentContext,
                                new byte[] { Prefix_Owner }.Concat(tokenId));

    public static BigInteger SlotOf(ByteString tokenId)
        => (BigInteger)(Storage.Get(Storage.CurrentContext,
                                    new byte[] { Prefix_Slot }.Concat(tokenId))
                        ?? ByteString.Empty);

    /// <summary>NEP-11 balanceOf for divisible NFTs returns the value of this token.</summary>
    public static BigInteger BalanceOf(ByteString tokenId)
        => (BigInteger)(Storage.Get(Storage.CurrentContext,
                                    new byte[] { Prefix_Value }.Concat(tokenId))
                        ?? ByteString.Empty);

    public static ByteString Mint(UInt160 to, BigInteger slot, BigInteger value)
    {
        if (!Runtime.CheckWitness(GetAdmin())) throw new Exception("admin only");
        var id = NextId();
        Storage.Put(Storage.CurrentContext, new byte[] { Prefix_Owner }.Concat(id), to);
        Storage.Put(Storage.CurrentContext, new byte[] { Prefix_Slot  }.Concat(id), slot);
        Storage.Put(Storage.CurrentContext, new byte[] { Prefix_Value }.Concat(id), value);
        OnTransfer(UInt160.Zero, to, value, id);
        OnTransferValue(null, id, value);
        return id;
    }

    /// <summary>Move `value` from one tokenId to another within the same slot.</summary>
    public static void TransferValueToToken(ByteString fromId, ByteString toId, BigInteger value)
    {
        var fromOwner = OwnerOf(fromId);
        if (!Runtime.CheckWitness(fromOwner)) throw new Exception("not owner");
        if (!SlotOf(fromId).Equals(SlotOf(toId))) throw new Exception("slot mismatch");

        var fromVal = BalanceOf(fromId);
        if (fromVal < value) throw new Exception("insufficient");
        Storage.Put(Storage.CurrentContext, new byte[] { Prefix_Value }.Concat(fromId),
                    fromVal - value);
        Storage.Put(Storage.CurrentContext, new byte[] { Prefix_Value }.Concat(toId),
                    BalanceOf(toId) + value);
        OnTransferValue(fromId, toId, value);
    }

    /// <summary>Split: move `value` to a new token owned by `to`.</summary>
    public static ByteString TransferValueToAddress(ByteString fromId, UInt160 to, BigInteger value)
    {
        var fromOwner = OwnerOf(fromId);
        if (!Runtime.CheckWitness(fromOwner)) throw new Exception("not owner");
        var fromVal = BalanceOf(fromId);
        if (fromVal < value) throw new Exception("insufficient");

        Storage.Put(Storage.CurrentContext, new byte[] { Prefix_Value }.Concat(fromId),
                    fromVal - value);
        var newId = NextId();
        var slot  = SlotOf(fromId);
        Storage.Put(Storage.CurrentContext, new byte[] { Prefix_Owner }.Concat(newId), to);
        Storage.Put(Storage.CurrentContext, new byte[] { Prefix_Slot  }.Concat(newId), slot);
        Storage.Put(Storage.CurrentContext, new byte[] { Prefix_Value }.Concat(newId), value);
        OnTransfer(UInt160.Zero, to, value, newId);
        OnTransferValue(fromId, newId, value);
        return newId;
    }

    private static ByteString NextId()
    {
        var n = (BigInteger)(Storage.Get(Storage.CurrentContext, NextIdKey) ?? ByteString.Empty) + 1;
        Storage.Put(Storage.CurrentContext, NextIdKey, n);
        return (ByteString)n.ToByteArray();
    }

    private static UInt160 GetAdmin() => (UInt160)"0x0000000000000000000000000000000000000000";
}
```

### Why It Works on Neo Without a NEP

A semi-fungible token is just an NFT with a per-token value field. The `properties`
map of NEP-11 already supports per-token data; the bond contract above just promotes
"value" to a first-class field for explicit access. Wallets that only know NEP-11
still see "this address owns N tokens"; bond-aware applications use the SFT
methods to introspect value.

</template>

</StandardEntry>

<!-- ============================================================ -->
<!-- ERC-2309 -->
<!-- ============================================================ -->

<StandardEntry
  id="erc-2309"
  title="ERC-2309 — Consecutive NFT Mints"
  eip="2309"
  status="Final"
  neoMapping="Neo C# port"
  category="NFT Extension"
  parityLabel="Port"
  parityClass="sm-pill-port"
>

<template #spec>

## ERC-2309: Consecutive Transfer Event

A gas-optimization standard: instead of emitting one `Transfer` event per token in a
batch mint, emit a single `ConsecutiveTransfer(fromId, toId, fromAddr, toAddr)` event
covering the entire range. Indexers (TheGraph, OpenSea) parse this and synthesize
per-token transfers off-chain.

### Required Event

```solidity
event ConsecutiveTransfer(
    uint256 indexed fromTokenId,
    uint256 toTokenId,
    address indexed fromAddress,
    address indexed toAddress
);
```

### Neo Equivalent: Same Event Pattern

Neo's event system supports identical semantics — emit one event covering a range
instead of N events. Below is the Neo C# port. The event signature is named
`ConsecutiveTransfer` and indexers can subscribe to it just as they would on Ethereum.

::: tip Live on Neo TestNet
Both implementations are deployed and behavior-verified on Neo N3 TestNet (network magic `894710606`).

| Implementation | Contract Hash | Deploy Tx |
| --- | --- | --- |
| **Solidity** (`neo-solc`) | `0x20262b3b96d92a0db7bfdc4782903fb3d997f900` | [`0x014b746a…f4fd44`](https://dora.coz.io/transaction/neo3/testnet/0x014b746ab2d46e1e7dcc33de436cc4fd7ac1bfba023693205b5cfd12daf4fd44) |
| **Neo C#** (`nccs`) | `0x2e157ce2532dee6084f53c1a848975bd960be918` | [`0x60d5dcf1…7d1bd2`](https://dora.coz.io/transaction/neo3/testnet/0x60d5dcf15270cba4449c0518ca1768deca886f980469d01b44232dc3827d1bd2) |

Cross-implementation invocations match on `claimDeployer`, `totalMinted`,
`getDeployer`. Source pairs under
[`docs/standards-mirror/deployments/erc-2309/`](https://github.com/r3e-network/neo-solidity/tree/main/docs/standards-mirror/deployments/erc-2309).
:::

</template>

<template #solidity>

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

contract ERC721A {
    mapping(uint256 => address) private _explicitOwner;
    mapping(address => uint256) private _balance;
    uint256 private _next;

    event Transfer(address indexed from, address indexed to, uint256 indexed tokenId);
    event ConsecutiveTransfer(
        uint256 indexed fromTokenId, uint256 toTokenId,
        address indexed fromAddress, address indexed toAddress
    );

    function mintBatch(address to, uint256 count) external {
        uint256 fromId = _next;
        uint256 toId   = _next + count - 1;
        _next         += count;
        _balance[to]  += count;
        _explicitOwner[fromId] = to;   // sentinel — fill rest lazily on first transfer
        emit ConsecutiveTransfer(fromId, toId, address(0), to);
    }

    function ownerOf(uint256 tokenId) public view returns (address) {
        // Walk back from tokenId to find the most recent explicit owner.
        for (uint256 i = tokenId; i >= 0; --i) {
            if (_explicitOwner[i] != address(0)) return _explicitOwner[i];
            if (i == 0) break;
        }
        revert("nonexistent");
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

[DisplayName("BatchMintNFT")]
[ContractPermission("*", "*")]
[SupportedStandards("NEP-11")]
public class BatchMintNFT : SmartContract
{
    private const byte Prefix_Explicit = 0x01;   // explicit owner records
    private const byte Prefix_Balance  = 0x02;
    private static readonly byte[] NextIdKey = { 0xFE };

    [DisplayName("Transfer")]
    public static event Action<UInt160, UInt160, BigInteger, ByteString> OnTransfer;

    [DisplayName("ConsecutiveTransfer")]
    public static event Action<BigInteger, BigInteger, UInt160, UInt160> OnConsecutiveTransfer;

    public static void MintBatch(UInt160 to, BigInteger count)
    {
        if (!Runtime.CheckWitness(GetAdmin())) throw new Exception("admin only");
        if (count <= 0) throw new Exception("count <= 0");

        var fromId = (BigInteger)(Storage.Get(Storage.CurrentContext, NextIdKey) ?? ByteString.Empty);
        var toId   = fromId + count - 1;
        Storage.Put(Storage.CurrentContext, NextIdKey, toId + 1);

        // Single sentinel record for the whole range
        Storage.Put(Storage.CurrentContext,
                    new byte[] { Prefix_Explicit }.Concat(((ByteString)fromId.ToByteArray())),
                    to);

        var bal = (BigInteger)(Storage.Get(Storage.CurrentContext,
                                           new byte[] { Prefix_Balance }.Concat(to))
                               ?? ByteString.Empty) + count;
        Storage.Put(Storage.CurrentContext,
                    new byte[] { Prefix_Balance }.Concat(to), bal);

        OnConsecutiveTransfer(fromId, toId, UInt160.Zero, to);
    }

    public static UInt160 OwnerOf(ByteString tokenIdBytes)
    {
        var tokenId = new BigInteger(tokenIdBytes);
        // Iterate explicit owner records, walk back from tokenId
        var iter = Storage.Find(Storage.CurrentContext, new byte[] { Prefix_Explicit },
                                FindOptions.None);
        UInt160 best = null;
        BigInteger bestId = -1;
        while (iter.Next())
        {
            var entry = (object[])iter.Value;
            var id    = new BigInteger((ByteString)entry[0]);
            var owner = (UInt160)entry[1];
            if (id <= tokenId && id > bestId) { bestId = id; best = owner; }
        }
        if (best == null) throw new Exception("nonexistent");
        return best;
    }

    private static UInt160 GetAdmin() => (UInt160)"0x0000000000000000000000000000000000000000";
}
```

</template>

</StandardEntry>

<!-- ============================================================ -->
<!-- ERC-4906 -->
<!-- ============================================================ -->

<StandardEntry
  id="erc-4906"
  title="ERC-4906 — NFT Metadata Update"
  eip="4906"
  status="Final"
  neoMapping="Neo C# port"
  category="NFT Extension"
  parityLabel="Port"
  parityClass="sm-pill-port"
>

<template #spec>

## ERC-4906: NFT Metadata Update Notification

ERC-4906 standardises **events that signal metadata changes** so wallets and
marketplaces can refresh cached metadata without polling. Without this, any change to
`tokenURI` content was invisible to indexers.

### Required Events

```solidity
event MetadataUpdate(uint256 _tokenId);
event BatchMetadataUpdate(uint256 _fromTokenId, uint256 _toTokenId);
```

### Neo Equivalent: NEP-11 `properties` + Event

Neo NEP-11 stores metadata **on-chain** via `properties(tokenId)`, so cache
invalidation is a non-problem in the steady state — but indexers still benefit from
explicit notification when properties mutate. The Neo port emits the same events
under the same names.

::: tip Live on Neo TestNet
Both implementations are deployed and behavior-verified on Neo N3 TestNet (network magic `894710606`).

| Implementation | Contract Hash | Deploy Tx |
| --- | --- | --- |
| **Solidity** (`neo-solc`) | `0x86f4d37e2471fdddf6738bb977de99646102b793` | [`0xd1d42b50…2e3c61`](https://dora.coz.io/transaction/neo3/testnet/0xd1d42b502a1396e7d4b9f55d8ee41891207c41e1c50c81a9ef1c07ef492e3c61) |
| **Neo C#** (`nccs`) | `0x3429da478e520ac009dc64520c8c3ccd00158061` | [`0x83d84d1b…f09ea0`](https://dora.coz.io/transaction/neo3/testnet/0x83d84d1b0384d16f2cc9f023f462c1821245f8460ded4413fb633c6147f09ea0) |

Cross-implementation invocations match on `claimDeployer`, `nextId`,
`getDeployer`. Source pairs under
[`docs/standards-mirror/deployments/erc-4906/`](https://github.com/r3e-network/neo-solidity/tree/main/docs/standards-mirror/deployments/erc-4906).
:::

</template>

<template #solidity>

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

interface IERC4906 {
    event MetadataUpdate(uint256 _tokenId);
    event BatchMetadataUpdate(uint256 _fromTokenId, uint256 _toTokenId);
}

contract DynamicNFT is IERC4906 {
    mapping(uint256 => string) public tokenURI;

    function setTokenURI(uint256 tokenId, string calldata uri) external {
        // ... auth check ...
        tokenURI[tokenId] = uri;
        emit MetadataUpdate(tokenId);
    }

    function setBatchTokenURI(uint256 fromId, uint256 toId, string calldata uri) external {
        // ... auth check + apply uri to range ...
        emit BatchMetadataUpdate(fromId, toId);
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

[DisplayName("DynamicNFT")]
[ContractPermission("*", "*")]
[SupportedStandards("NEP-11")]
public class DynamicNFT : SmartContract
{
    private const byte Prefix_Properties = 0x03;

    [DisplayName("MetadataUpdate")]
    public static event Action<ByteString> OnMetadataUpdate;

    [DisplayName("BatchMetadataUpdate")]
    public static event Action<ByteString, ByteString> OnBatchMetadataUpdate;

    /// <summary>
    /// Update the on-chain properties map for a single token, then notify indexers.
    /// </summary>
    public static void SetProperties(ByteString tokenId, Map<string, object> props)
    {
        if (!Runtime.CheckWitness(GetAdmin())) throw new Exception("admin only");
        Storage.Put(Storage.CurrentContext,
                    new byte[] { Prefix_Properties }.Concat(tokenId),
                    StdLib.Serialize(props));
        OnMetadataUpdate(tokenId);
    }

    public static void SetBatchProperties(
        ByteString fromId, ByteString toId, Map<string, object> props)
    {
        if (!Runtime.CheckWitness(GetAdmin())) throw new Exception("admin only");
        var serialized = StdLib.Serialize(props);
        // Simplified: caller is expected to expand the range; in practice apply to each id.
        OnBatchMetadataUpdate(fromId, toId);
    }

    private static UInt160 GetAdmin() => (UInt160)"0x0000000000000000000000000000000000000000";
}
```

</template>

</StandardEntry>

<!-- ============================================================ -->
<!-- ERC-4494 -->
<!-- ============================================================ -->

<StandardEntry
  id="erc-4494"
  title="ERC-4494 — Permit for ERC-721"
  eip="4494"
  status="Final"
  neoMapping="Native witness scopes"
  category="NFT Extension"
  parityLabel="Native"
  parityClass="sm-pill-native"
>

<template #spec>

## ERC-4494: NFT Permit (Gasless NFT Approval)

ERC-4494 brings ERC-2612-style gasless approvals to NFTs. The owner signs an EIP-712
typed message authorising a single-use permit; a marketplace submits the permit
on-chain to set the approval, then immediately calls `transferFrom`. One transaction,
no separate `approve`.

### Required Interface

```solidity
function permit(
    address spender, uint256 tokenId, uint256 deadline,
    bytes memory sig
) external;
function nonces(uint256 tokenId)  external view returns (uint256);
function DOMAIN_SEPARATOR()       external view returns (bytes32);
```

### Neo Equivalent: Witness Scopes

Just like ERC-2612, the entire mechanism is unnecessary on Neo. The user signs a
single transaction with `WitnessScope.CustomContracts` allowing the marketplace
contract to call `Transfer` on the NFT. No permit method to write, no nonces, no
domain separator, no signature parsing. The C# tab shows what the application code
looks like — typically nothing at the contract level beyond standard NEP-11.

</template>

<template #solidity>

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

contract ERC721Permit {
    bytes32 private constant _PERMIT_TYPEHASH = keccak256(
      "Permit(address spender,uint256 tokenId,uint256 nonce,uint256 deadline)"
    );
    mapping(uint256 => uint256) public nonces;
    bytes32 public immutable DOMAIN_SEPARATOR;

    constructor() {
        DOMAIN_SEPARATOR = keccak256(abi.encode(
            keccak256("EIP712Domain(string name,uint256 chainId,address verifyingContract)"),
            keccak256("MyNFT"), block.chainid, address(this)
        ));
    }

    function permit(address spender, uint256 tokenId, uint256 deadline, bytes memory sig)
        external
    {
        require(block.timestamp <= deadline, "expired");
        bytes32 structHash = keccak256(abi.encode(
            _PERMIT_TYPEHASH, spender, tokenId, nonces[tokenId]++, deadline
        ));
        bytes32 hash = keccak256(abi.encodePacked("\x19\x01", DOMAIN_SEPARATOR, structHash));

        address signer = _recoverSigner(hash, sig);
        require(signer == _ownerOf(tokenId), "unauth");
        _approve(spender, tokenId);
    }

    function _ownerOf(uint256) internal view returns (address) { /* ... */ return address(0); }
    function _approve(address, uint256) internal { /* ... */ }
    function _recoverSigner(bytes32, bytes memory) internal pure returns (address) { /* ECDSA */ return address(0); }
}
```

</template>

<template #csharp>

```csharp
// Standard NEP-11 already covers this — no permit method needed.
//
// User-side flow on Neo (TypeScript example showing what the dApp builds):
//
//   const tx = new TransactionBuilder()
//     .invoke(marketplace, "buy", [listingId])
//     .signers([{
//       account: userAddress,
//       scopes: WitnessScope.CustomContracts,
//       allowedContracts: [nftContract, marketplace]   // scope-limited authorization
//     }])
//     .build();
//   const signed = await wallet.signTransaction(tx);
//   await rpc.sendRawTransaction(signed);
//
// Inside marketplace.buy(...) the marketplace calls:
//   Contract.Call(nftContract, "transfer", CallFlags.All, ...)
// which invokes the NFT's Transfer with the user's witness available.
// The NFT's CheckWitness(owner) succeeds because the user signed with
// CustomContracts scope including the NFT contract.

using System;
using System.Numerics;
using Neo;
using Neo.SmartContract.Framework;
using Neo.SmartContract.Framework.Attributes;
using Neo.SmartContract.Framework.Native;
using Neo.SmartContract.Framework.Services;

namespace R3E.Examples;

/// <summary>
/// Marketplace using the buyer's witness for atomic NFT settlement.
/// No permit, no signatures to parse — single-tx UX is built in.
/// </summary>
[DisplayName("Marketplace")]
[ContractPermission("*", "*")]
public class Marketplace : SmartContract
{
    public static void Buy(UInt160 buyer, UInt160 nft, ByteString tokenId,
                           BigInteger price, UInt160 paymentToken)
    {
        if (!Runtime.CheckWitness(buyer)) throw new Exception("buyer must sign");

        var seller = (UInt160)Contract.Call(nft, "ownerOf", CallFlags.ReadOnly,
                                            new object[] { tokenId });

        // Pay the seller
        Contract.Call(paymentToken, "transfer", CallFlags.All,
                      new object[] { buyer, seller, price, "purchase" });
        // Receive the NFT — works because buyer's witness scope includes nft contract
        Contract.Call(nft, "transfer", CallFlags.All,
                      new object[] { buyer, tokenId, "purchase" });
    }
}
```

</template>

</StandardEntry>

<!-- ============================================================ -->
<!-- ERC-5192 -->
<!-- ============================================================ -->

<StandardEntry
  id="erc-5192"
  title="ERC-5192 — Soulbound NFTs"
  eip="5192"
  status="Final"
  neoMapping="Neo C# port"
  category="NFT Extension"
  parityLabel="Port"
  parityClass="sm-pill-port"
>

<template #spec>

## ERC-5192: Minimal Soulbound NFTs

A soulbound token (SBT) is an NFT that **cannot be transferred** once issued. Use
cases: credentials, attestations, badges, voting power tied to identity, KYC
artifacts. ERC-5192 adds a single `locked(tokenId)` view method plus `Locked` /
`Unlocked` events.

### Required Interface

```solidity
function locked(uint256 tokenId) external view returns (bool);
event Locked(uint256 tokenId);
event Unlocked(uint256 tokenId);
```

A locked token's `transferFrom` MUST revert.

### Neo Equivalent: Direct Port

Neo C# port: track per-token lock state in storage, check it in `Transfer`. The Neo
contract is even simpler than the Solidity version because the witness model
naturally enforces who can lock/unlock.

::: tip Live on Neo TestNet
Both implementations deployed and behavior-verified on Neo N3 TestNet.

| Implementation | TestNet Address | Contract Hash |
| --- | --- | --- |
| **Solidity** (`neo-solc`) | `NQfv7FPi2BWZBnc3PGB9PggK72bJtZL9HH` | [`0x1b75ecb9…7f3c3034`](https://dora.coz.io/contract/neo3/testnet/0x1b75ecb9e926203e66283e3f875ba5097f3c3034) |
| **Neo C#** (`nccs`) | `NPuqRsgHLPkRZsxEJgs3igKKhZE1nqHVL3` | [`0x7081fcf3…3c07da2b`](https://dora.coz.io/contract/neo3/testnet/0x7081fcf36db56a716b416ef553829ed23c07da2b) |

Verified: token issued and locked (`locked(1) == true`). Soulbound check rejects transfer attempts.
[`docs/standards-mirror/deployments/erc-5192/`](https://github.com/r3e-network/neo-solidity/tree/main/docs/standards-mirror/deployments/erc-5192).
:::

</template>

<template #solidity>

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

abstract contract SoulboundNFT {
    mapping(uint256 => address) private _owner;
    mapping(uint256 => bool)    private _locked;

    event Transfer(address indexed from, address indexed to, uint256 indexed tokenId);
    event Locked(uint256 tokenId);
    event Unlocked(uint256 tokenId);

    function locked(uint256 tokenId) external view returns (bool) {
        return _locked[tokenId];
    }

    function _mint(address to, uint256 tokenId, bool soulbound) internal {
        _owner[tokenId] = to;
        if (soulbound) {
            _locked[tokenId] = true;
            emit Locked(tokenId);
        }
        emit Transfer(address(0), to, tokenId);
    }

    function transferFrom(address from, address to, uint256 tokenId) external {
        require(!_locked[tokenId], "soulbound: token is locked");
        require(_owner[tokenId] == from, "wrong from");
        require(msg.sender == from,      "not authorized");
        _owner[tokenId] = to;
        emit Transfer(from, to, tokenId);
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

[DisplayName("SoulboundCert")]
[ContractPermission("*", "*")]
[SupportedStandards("NEP-11")]
public class SoulboundCert : SmartContract
{
    private const byte Prefix_Owner  = 0x01;
    private const byte Prefix_Locked = 0x02;

    [DisplayName("Transfer")]
    public static event Action<UInt160, UInt160, BigInteger, ByteString> OnTransfer;
    [DisplayName("Locked")]
    public static event Action<ByteString> OnLocked;
    [DisplayName("Unlocked")]
    public static event Action<ByteString> OnUnlocked;

    public static bool Locked(ByteString tokenId)
        => Storage.Get(Storage.CurrentContext, new byte[] { Prefix_Locked }.Concat(tokenId)) != null;

    public static UInt160 OwnerOf(ByteString tokenId)
    {
        var raw = Storage.Get(Storage.CurrentContext,
                              new byte[] { Prefix_Owner }.Concat(tokenId));
        if (raw == null) throw new Exception("nonexistent");
        return (UInt160)raw;
    }

    public static void Issue(UInt160 to, ByteString tokenId, bool soulbound)
    {
        if (!Runtime.CheckWitness(GetIssuer())) throw new Exception("issuer only");
        if (Storage.Get(Storage.CurrentContext,
                        new byte[] { Prefix_Owner }.Concat(tokenId)) != null)
            throw new Exception("already issued");

        Storage.Put(Storage.CurrentContext,
                    new byte[] { Prefix_Owner }.Concat(tokenId), to);
        if (soulbound)
        {
            Storage.Put(Storage.CurrentContext,
                        new byte[] { Prefix_Locked }.Concat(tokenId), 1);
            OnLocked(tokenId);
        }
        OnTransfer(UInt160.Zero, to, 1, tokenId);
    }

    public static bool Transfer(UInt160 to, ByteString tokenId, object data)
    {
        if (Locked(tokenId)) throw new Exception("soulbound: token is locked");
        var from = OwnerOf(tokenId);
        if (!Runtime.CheckWitness(from)) throw new Exception("not authorized");
        Storage.Put(Storage.CurrentContext,
                    new byte[] { Prefix_Owner }.Concat(tokenId), to);
        OnTransfer(from, to, 1, tokenId);
        return true;
    }

    /// <summary>Issuer can revoke the soulbound flag (e.g. expired credential).</summary>
    public static void Unlock(ByteString tokenId)
    {
        if (!Runtime.CheckWitness(GetIssuer())) throw new Exception("issuer only");
        Storage.Delete(Storage.CurrentContext, new byte[] { Prefix_Locked }.Concat(tokenId));
        OnUnlocked(tokenId);
    }

    private static UInt160 GetIssuer() => (UInt160)"0x0000000000000000000000000000000000000000";
}
```

</template>

</StandardEntry>

<!-- ============================================================ -->
<!-- ERC-5484 -->
<!-- ============================================================ -->

<StandardEntry
  id="erc-5484"
  title="ERC-5484 — Consensual Soulbound"
  eip="5484"
  status="Final"
  neoMapping="Neo C# port"
  category="NFT Extension"
  parityLabel="Port"
  parityClass="sm-pill-port"
>

<template #spec>

## ERC-5484: Consensual Soulbound Tokens

ERC-5484 extends the soulbound concept with **explicit consent semantics** for who
can burn the token: the issuer, the recipient, both, or neither. This addresses real
use cases: a recipient should be able to burn an unwanted credential, an issuer
should be able to revoke a fraudulent one, and some tokens (e.g. blacklist marks)
should be permanent.

### `BurnAuth` Enum

| Value | Who can burn |
| --- | --- |
| `0 IssuerOnly` | Only the contract issuer |
| `1 OwnerOnly` | Only the token holder |
| `2 Both` | Either party |
| `3 Neither` | Permanent — neither can destroy it |

### Required Method

```solidity
function burnAuth(uint256 tokenId) external view returns (BurnAuth);
event Issued(address indexed from, address indexed to, uint256 indexed tokenId, BurnAuth burnAuth);
```

### Neo Equivalent: Direct Port

Standard Neo C# port — store per-token `burnAuth` and check it in the burn function.

::: tip Live on Neo TestNet
Both implementations are deployed and behavior-verified on Neo N3 TestNet (network magic `894710606`).

| Implementation | Contract Hash | Deploy Tx |
| --- | --- | --- |
| **Solidity** (`neo-solc`) | `0x8a9e1835270c95ddf5250ee84a1d4714552cb951` | [`0xa440db2d…f4246a`](https://dora.coz.io/transaction/neo3/testnet/0xa440db2d432184bbbc13275bf3cce3490153bee170b82384237e67558af4246a) |
| **Neo C#** (`nccs`) | `0x02317b7192e3d91ba1739ae2a9f5fdcd44bf2dac` | [`0xebc7d7f7…d9432c`](https://dora.coz.io/transaction/neo3/testnet/0xebc7d7f71b74b20808a6bad75dcb8fa78d9f34fc0a02f72697f86578a1d9432c) |

Cross-implementation invocations match on `claimIssuer`, `tokenCount`,
`getIssuer`. Source pairs under
[`docs/standards-mirror/deployments/erc-5484/`](https://github.com/r3e-network/neo-solidity/tree/main/docs/standards-mirror/deployments/erc-5484).
:::

</template>

<template #solidity>

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

abstract contract ConsensualSBT {
    enum BurnAuth { IssuerOnly, OwnerOnly, Both, Neither }

    address public issuer;
    mapping(uint256 => address)  private _owner;
    mapping(uint256 => BurnAuth) private _burnAuth;

    event Issued(address indexed from, address indexed to,
                 uint256 indexed tokenId, BurnAuth burnAuth);

    constructor() { issuer = msg.sender; }

    function burnAuth(uint256 tokenId) external view returns (BurnAuth) {
        return _burnAuth[tokenId];
    }

    function issue(address to, uint256 tokenId, BurnAuth auth) external {
        require(msg.sender == issuer, "not issuer");
        _owner[tokenId] = to;
        _burnAuth[tokenId] = auth;
        emit Issued(issuer, to, tokenId, auth);
    }

    function burn(uint256 tokenId) external {
        BurnAuth auth = _burnAuth[tokenId];
        if (auth == BurnAuth.Neither)        revert("permanent");
        if (auth == BurnAuth.IssuerOnly)     require(msg.sender == issuer, "issuer only");
        if (auth == BurnAuth.OwnerOnly)      require(msg.sender == _owner[tokenId], "owner only");
        if (auth == BurnAuth.Both)           require(msg.sender == issuer
                                                  || msg.sender == _owner[tokenId], "issuer or owner only");
        delete _owner[tokenId];
        delete _burnAuth[tokenId];
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

public enum BurnAuth : byte { IssuerOnly = 0, OwnerOnly = 1, Both = 2, Neither = 3 }

[DisplayName("ConsensualSBT")]
[ContractPermission("*", "*")]
[SupportedStandards("NEP-11")]
public class ConsensualSBT : SmartContract
{
    private const byte Prefix_Owner    = 0x01;
    private const byte Prefix_BurnAuth = 0x02;
    private static readonly byte[] IssuerKey = { 0xFE };

    [DisplayName("Issued")]
    public static event Action<UInt160, UInt160, ByteString, byte> OnIssued;
    [DisplayName("Burned")]
    public static event Action<ByteString> OnBurned;

    public static byte BurnAuthOf(ByteString tokenId)
    {
        var raw = Storage.Get(Storage.CurrentContext,
                              new byte[] { Prefix_BurnAuth }.Concat(tokenId));
        if (raw == null) throw new Exception("nonexistent");
        return (byte)(BigInteger)raw;
    }

    public static void Issue(UInt160 to, ByteString tokenId, byte auth)
    {
        var issuer = (UInt160)Storage.Get(Storage.CurrentContext, IssuerKey);
        if (!Runtime.CheckWitness(issuer)) throw new Exception("issuer only");
        if (auth > 3) throw new Exception("invalid auth");
        Storage.Put(Storage.CurrentContext,
                    new byte[] { Prefix_Owner }.Concat(tokenId), to);
        Storage.Put(Storage.CurrentContext,
                    new byte[] { Prefix_BurnAuth }.Concat(tokenId), auth);
        OnIssued(issuer, to, tokenId, auth);
    }

    public static void Burn(ByteString tokenId)
    {
        var auth   = (BurnAuth)BurnAuthOf(tokenId);
        var owner  = (UInt160)Storage.Get(Storage.CurrentContext,
                                          new byte[] { Prefix_Owner }.Concat(tokenId));
        var issuer = (UInt160)Storage.Get(Storage.CurrentContext, IssuerKey);

        bool canBurn = auth switch
        {
            BurnAuth.Neither    => false,
            BurnAuth.IssuerOnly => Runtime.CheckWitness(issuer),
            BurnAuth.OwnerOnly  => Runtime.CheckWitness(owner),
            BurnAuth.Both       => Runtime.CheckWitness(issuer)
                                || Runtime.CheckWitness(owner),
            _ => false
        };
        if (!canBurn) throw new Exception("not authorized to burn");

        Storage.Delete(Storage.CurrentContext,
                       new byte[] { Prefix_Owner }.Concat(tokenId));
        Storage.Delete(Storage.CurrentContext,
                       new byte[] { Prefix_BurnAuth }.Concat(tokenId));
        OnBurned(tokenId);
    }

    public static void _deploy(object data, bool update)
    {
        if (update) return;
        Storage.Put(Storage.CurrentContext, IssuerKey, (UInt160)data);
    }
}
```

</template>

</StandardEntry>

<!-- ============================================================ -->
<!-- ERC-6909 -->
<!-- ============================================================ -->

<StandardEntry
  id="erc-6909"
  title="ERC-6909 — Minimal Multi-Token"
  eip="6909"
  status="Final"
  neoMapping="Neo C# port"
  category="Multi-Token"
  parityLabel="Port"
  parityClass="sm-pill-port"
>

<template #spec>

## ERC-6909: Minimal Multi-Token Standard

ERC-6909 is a leaner alternative to ERC-1155 that strips out the safe-transfer
acceptance check (no recipient hooks at all), removes the URI requirement, and
simplifies operator approvals. Used by Uniswap V4's pool manager — every pool's
liquidity is represented as a separate ERC-6909 token id.

### Required Interface

```solidity
function balanceOf(address owner, uint256 id) external view returns (uint256);
function transfer(address to, uint256 id, uint256 amount) external returns (bool);
function transferFrom(address from, address to, uint256 id, uint256 amount)
    external returns (bool);
function approve(address spender, uint256 id, uint256 amount) external returns (bool);
function allowance(address owner, address spender, uint256 id) external view returns (uint256);
function setOperator(address operator, bool approved) external returns (bool);
function isOperator(address owner, address operator) external view returns (bool);
```

### Neo Equivalent: Direct Port

The Neo C# port replaces approve/allowance with witness checks (default Neo
authorization). Operators map to NEP-30-style authorisation contracts but for the
common case the witness model already covers it.

</template>

<template #solidity>

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

contract ERC6909 {
    mapping(address => mapping(uint256 => uint256)) public balanceOf;
    mapping(address => mapping(address => mapping(uint256 => uint256))) public allowance;
    mapping(address => mapping(address => bool)) public isOperator;

    event Transfer(address caller, address indexed from, address indexed to,
                   uint256 indexed id, uint256 amount);
    event Approval(address indexed owner, address indexed spender,
                   uint256 indexed id, uint256 amount);
    event OperatorSet(address indexed owner, address indexed operator, bool approved);

    function transfer(address to, uint256 id, uint256 amount) public returns (bool) {
        balanceOf[msg.sender][id] -= amount;
        balanceOf[to][id]         += amount;
        emit Transfer(msg.sender, msg.sender, to, id, amount);
        return true;
    }

    function transferFrom(address from, address to, uint256 id, uint256 amount)
        public returns (bool)
    {
        if (msg.sender != from && !isOperator[from][msg.sender]) {
            uint256 allowed = allowance[from][msg.sender][id];
            if (allowed != type(uint256).max) {
                allowance[from][msg.sender][id] = allowed - amount;
            }
        }
        balanceOf[from][id] -= amount;
        balanceOf[to][id]   += amount;
        emit Transfer(msg.sender, from, to, id, amount);
        return true;
    }

    function approve(address spender, uint256 id, uint256 amount) public returns (bool) {
        allowance[msg.sender][spender][id] = amount;
        emit Approval(msg.sender, spender, id, amount);
        return true;
    }

    function setOperator(address operator, bool approved) public returns (bool) {
        isOperator[msg.sender][operator] = approved;
        emit OperatorSet(msg.sender, operator, approved);
        return true;
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

[DisplayName("MinimalMultiToken")]
[ContractPermission("*", "*")]
public class MinimalMultiToken : SmartContract
{
    private const byte Prefix_Balance = 0x01;  // owner+id -> amount

    [DisplayName("Transfer")]
    public static event Action<UInt160, UInt160, UInt160, ByteString, BigInteger> OnTransfer;

    public static BigInteger BalanceOf(UInt160 owner, ByteString id)
        => (BigInteger)(Storage.Get(Storage.CurrentContext,
            new byte[] { Prefix_Balance }.Concat(owner).Concat(id)) ?? ByteString.Empty);

    public static bool Transfer(UInt160 from, UInt160 to, ByteString id, BigInteger amount)
    {
        if (!Runtime.CheckWitness(from)) throw new Exception("no authorization");
        if (amount < 0)                  throw new Exception("amount < 0");

        var fromBal = BalanceOf(from, id);
        if (fromBal < amount) return false;
        UpdateBalance(from, id, fromBal - amount);
        UpdateBalance(to,   id, BalanceOf(to, id) + amount);
        OnTransfer(Runtime.CallingScriptHash, from, to, id, amount);
        return true;
    }

    private static void UpdateBalance(UInt160 owner, ByteString id, BigInteger value)
    {
        var key = new byte[] { Prefix_Balance }.Concat(owner).Concat(id);
        if (value == 0) Storage.Delete(Storage.CurrentContext, key);
        else            Storage.Put(Storage.CurrentContext, key, value);
    }

    public static void Mint(UInt160 to, ByteString id, BigInteger amount)
    {
        if (!Runtime.CheckWitness(GetAdmin())) throw new Exception("admin only");
        UpdateBalance(to, id, BalanceOf(to, id) + amount);
        OnTransfer(Runtime.CallingScriptHash, UInt160.Zero, to, id, amount);
    }

    private static UInt160 GetAdmin() => (UInt160)"0x0000000000000000000000000000000000000000";
}
```

### Why It's Cleaner on Neo

The witness model removes ERC-6909's `allowance` and `setOperator` complexity. A
caller that wants to spend on behalf of `from` simply needs `from` to be a signer
of the transaction with an appropriate witness scope. No per-id allowance storage,
no operator booleans.

</template>

</StandardEntry>

<!-- ============================================================ -->
<!-- ERC-5114 -->
<!-- ============================================================ -->

<StandardEntry
  id="erc-5114"
  title="ERC-5114 — Soulbound Badge"
  eip="5114"
  status="Final"
  neoMapping="Neo C# port"
  category="NFT Extension"
  parityLabel="Port"
  parityClass="sm-pill-port"
>

<template #spec>

## ERC-5114: Soulbound Badge (Bound to NFT)

ERC-5114 defines tokens that are **bound to a parent NFT** rather than to an account
— a badge attached to your CryptoPunk, an achievement attached to your in-game
character, a delegation marker attached to a governance NFT. The badge follows the
parent NFT through transfers but cannot be moved independently.

### Required Interface

```solidity
function ownerOf(uint256 badgeId)
    external view returns (uint256 nftAddressAndId);    // packed: addr | tokenId
function collectionUri()                external view returns (string memory);
function badgeUri(uint256 badgeId)      external view returns (string memory);
```

### Neo Equivalent: Direct Port

Neo C# port: each badge stores a reference to its parent (parent contract hash +
tokenId). The badge's effective owner is whoever owns the parent NFT; resolved on
each query.

::: tip Live on Neo TestNet
Both implementations are deployed and behavior-verified on Neo N3 TestNet (network magic `894710606`).

| Implementation | Contract Hash | Deploy Tx |
| --- | --- | --- |
| **Solidity** (`neo-solc`) | `0x91e34b16c373f845024013c3bd585ac9739b741f` | [`0x8cca8503…3a8c3a19`](https://dora.coz.io/transaction/neo3/testnet/0x8cca85038cf296fa6adce4faaa5c987dcc8c845584f00f31022a1e713a8c3a19) |
| **Neo C#** (`nccs`) | `0xd9d32f5f8d2d0cd5196cd94b49e3d11ac46d7039` | [`0x7dd08b98…170ee73`](https://dora.coz.io/transaction/neo3/testnet/0x7dd08b982cef569ff9e9758e308f762a44c2d1939086ff0ff108cc692170ee73) |

Cross-implementation invocations match on `claimDeployer`, `badgeCount`,
`getOwner`. Source pairs under
[`docs/standards-mirror/deployments/erc-5114/`](https://github.com/r3e-network/neo-solidity/tree/main/docs/standards-mirror/deployments/erc-5114).
:::

</template>

<template #solidity>

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

interface IERC5114 {
    function ownerOf(uint256 badgeId) external view returns (uint256);
    function collectionUri() external view returns (string memory);
    function badgeUri(uint256 badgeId) external view returns (string memory);
}

contract Achievement is IERC5114 {
    struct Parent { address nft; uint256 tokenId; }
    mapping(uint256 => Parent) private _parent;
    mapping(uint256 => string) private _uri;
    string private _collectionUri;

    function attach(uint256 badgeId, address nft, uint256 tokenId, string calldata uri) external {
        require(_parent[badgeId].nft == address(0), "already attached");
        _parent[badgeId] = Parent(nft, tokenId);
        _uri[badgeId] = uri;
    }

    function ownerOf(uint256 badgeId) external view returns (uint256) {
        Parent memory p = _parent[badgeId];
        // Pack: high 160 bits = nft address, low 96 bits = tokenId (truncated).
        return (uint256(uint160(p.nft)) << 96) | (p.tokenId & ((1 << 96) - 1));
    }

    function collectionUri() external view returns (string memory) { return _collectionUri; }
    function badgeUri(uint256 badgeId) external view returns (string memory) {
        return _uri[badgeId];
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

[DisplayName("Achievement")]
[ContractPermission("*", "*")]
public class Achievement : SmartContract
{
    private const byte Prefix_ParentNft = 0x01;
    private const byte Prefix_ParentTok = 0x02;
    private const byte Prefix_Uri       = 0x03;

    [DisplayName("Attached")]
    public static event Action<ByteString, UInt160, ByteString> OnAttached;

    /// <summary>
    /// Returns (parentContract, parentTokenId) — the badge's owner is whoever
    /// owns that NFT, resolved on demand by calling parentContract.ownerOf(parentTokenId).
    /// </summary>
    public static (UInt160, ByteString) ParentOf(ByteString badgeId)
    {
        var nft = (UInt160)Storage.Get(Storage.CurrentContext,
                                       new byte[] { Prefix_ParentNft }.Concat(badgeId));
        var tok = Storage.Get(Storage.CurrentContext,
                              new byte[] { Prefix_ParentTok }.Concat(badgeId));
        if (nft == null) throw new Exception("badge not attached");
        return (nft, tok);
    }

    /// <summary>Resolves through the parent NFT to find the actual holder.</summary>
    public static UInt160 EffectiveOwnerOf(ByteString badgeId)
    {
        var (nft, tok) = ParentOf(badgeId);
        return (UInt160)Contract.Call(nft, "ownerOf", CallFlags.ReadOnly, new object[] { tok });
    }

    public static void Attach(ByteString badgeId, UInt160 parentNft, ByteString parentTokenId,
                              string uri)
    {
        if (!Runtime.CheckWitness(GetAdmin())) throw new Exception("admin only");
        if (Storage.Get(Storage.CurrentContext,
                        new byte[] { Prefix_ParentNft }.Concat(badgeId)) != null)
            throw new Exception("already attached");

        Storage.Put(Storage.CurrentContext,
                    new byte[] { Prefix_ParentNft }.Concat(badgeId), parentNft);
        Storage.Put(Storage.CurrentContext,
                    new byte[] { Prefix_ParentTok }.Concat(badgeId), parentTokenId);
        Storage.Put(Storage.CurrentContext,
                    new byte[] { Prefix_Uri }.Concat(badgeId), uri);
        OnAttached(badgeId, parentNft, parentTokenId);
    }

    public static string BadgeUri(ByteString badgeId)
        => (string)Storage.Get(Storage.CurrentContext,
                               new byte[] { Prefix_Uri }.Concat(badgeId));

    private static UInt160 GetAdmin() => (UInt160)"0x0000000000000000000000000000000000000000";
}
```

</template>

</StandardEntry>

<!-- ============================================================ -->
<!-- ERC-6147 -->
<!-- ============================================================ -->

<StandardEntry
  id="erc-6147"
  title="ERC-6147 — NFT Guard"
  eip="6147"
  status="Final"
  neoMapping="Neo C# port"
  category="NFT Extension"
  parityLabel="Port"
  parityClass="sm-pill-port"
>

<template #spec>

## ERC-6147: NFT Guard (Multi-Sig Transfer Protection)

ERC-6147 introduces a **guard** address per token: a designated co-signer whose
approval is required before the token can be transferred or burned. Used to protect
high-value NFTs from compromised owner keys — the guard can be a hardware wallet,
a multi-sig, or a recovery service.

### Required Interface

```solidity
function changeGuard(uint256 tokenId, address guard, uint64 expires) external;
function guardOf(uint256 tokenId) external view returns (address guard, uint64 expires);
function removeGuard(uint256 tokenId) external;
```

Transfer / burn operations check `guardOf(tokenId)` and require both the owner's
authorization AND the guard's authorization (until `expires`).

### Neo Equivalent: Direct Port

Neo's witness scopes already let one transaction carry signatures from multiple
accounts — the C# port simply checks both the owner and the guard via two
`Runtime.CheckWitness` calls.

::: tip Live on Neo TestNet
Both implementations are deployed and behavior-verified on Neo N3 TestNet (network magic `894710606`).

| Implementation | Contract Hash | Deploy Tx |
| --- | --- | --- |
| **Solidity** (`neo-solc`) | `0xdf1474aed4764a1433892bb1ec2a8a143000e4c3` | [`0x5f17cdab…5666c60`](https://dora.coz.io/transaction/neo3/testnet/0x5f17cdab2309cbb2af6a66f917cbeb2fab409dd6b9a0dee702028469b5666c60) |
| **Neo C#** (`nccs`) | `0x9223d7237b8babbecb38dfc4cbb5e82f11019cd8` | [`0xe53c0607…559df5`](https://dora.coz.io/transaction/neo3/testnet/0xe53c06073fa5af3a8346d9f3f2938f185c2ecdfcc25502bd2bfce12d8b559df5) |

Cross-implementation invocations match on `claimDeployer`, `tokenCount`,
`getDeployer`. Source pairs under
[`docs/standards-mirror/deployments/erc-6147/`](https://github.com/r3e-network/neo-solidity/tree/main/docs/standards-mirror/deployments/erc-6147).
:::

</template>

<template #solidity>

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

abstract contract GuardedNFT {
    struct Guard { address guard; uint64 expires; }
    mapping(uint256 => address) private _owner;
    mapping(uint256 => Guard)   private _guard;

    event GuardChanged(uint256 indexed tokenId, address indexed guard, uint64 expires);

    function changeGuard(uint256 tokenId, address guard, uint64 expires) external {
        require(msg.sender == _owner[tokenId], "owner only");
        _guard[tokenId] = Guard(guard, expires);
        emit GuardChanged(tokenId, guard, expires);
    }

    function guardOf(uint256 tokenId) external view returns (address, uint64) {
        Guard memory g = _guard[tokenId];
        if (block.timestamp >= g.expires) return (address(0), 0);
        return (g.guard, g.expires);
    }

    function transferFrom(address from, address to, uint256 tokenId) external {
        require(_owner[tokenId] == from, "wrong from");
        require(msg.sender == from, "not owner");

        Guard memory g = _guard[tokenId];
        if (block.timestamp < g.expires && g.guard != address(0)) {
            // Guarded transfers require the guard to also sign — typically via
            // the guard pre-authorising or a multi-call from a multi-sig.
            require(false, "guard required (cannot be done via single tx with EOA)");
        }
        _owner[tokenId] = to;
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

[DisplayName("GuardedNFT")]
[ContractPermission("*", "*")]
[SupportedStandards("NEP-11")]
public class GuardedNFT : SmartContract
{
    private const byte Prefix_Owner    = 0x01;
    private const byte Prefix_Guard    = 0x02;
    private const byte Prefix_GuardExp = 0x03;

    [DisplayName("Transfer")]
    public static event Action<UInt160, UInt160, BigInteger, ByteString> OnTransfer;
    [DisplayName("GuardChanged")]
    public static event Action<ByteString, UInt160, BigInteger> OnGuardChanged;

    public static (UInt160, BigInteger) GuardOf(ByteString tokenId)
    {
        var guard = (UInt160)Storage.Get(Storage.CurrentContext,
                                         new byte[] { Prefix_Guard }.Concat(tokenId));
        var exp   = (BigInteger)(Storage.Get(Storage.CurrentContext,
                    new byte[] { Prefix_GuardExp }.Concat(tokenId)) ?? ByteString.Empty);
        if (Runtime.Time / 1000 >= exp) return (UInt160.Zero, 0);   // expired
        return (guard, exp);
    }

    public static void ChangeGuard(ByteString tokenId, UInt160 guard, BigInteger expires)
    {
        var owner = (UInt160)Storage.Get(Storage.CurrentContext,
                                         new byte[] { Prefix_Owner }.Concat(tokenId));
        if (!Runtime.CheckWitness(owner)) throw new Exception("owner only");
        Storage.Put(Storage.CurrentContext, new byte[] { Prefix_Guard }.Concat(tokenId), guard);
        Storage.Put(Storage.CurrentContext, new byte[] { Prefix_GuardExp }.Concat(tokenId), expires);
        OnGuardChanged(tokenId, guard, expires);
    }

    public static bool Transfer(UInt160 to, ByteString tokenId, object data)
    {
        var owner = (UInt160)Storage.Get(Storage.CurrentContext,
                                         new byte[] { Prefix_Owner }.Concat(tokenId));
        if (!Runtime.CheckWitness(owner)) throw new Exception("owner must sign");

        var (guard, exp) = GuardOf(tokenId);
        if (guard != UInt160.Zero)
        {
            // Guard active — require guard's signature too. Neo lets one tx carry
            // multiple signers, so the dApp builds the tx with both witnesses.
            if (!Runtime.CheckWitness(guard))
                throw new Exception("guard signature required");
        }

        Storage.Put(Storage.CurrentContext,
                    new byte[] { Prefix_Owner }.Concat(tokenId), to);
        OnTransfer(owner, to, 1, tokenId);
        return true;
    }
}
```

### Why the Neo Version Actually Works

The Solidity ERC-6147 has a known UX gap: the standard requires "guard approval"
but EOAs cannot batch-sign two transactions atomically. Most implementations
require the guard to *pre-authorize* the transfer in a separate transaction.

Neo's transaction format carries multiple signers natively — both the owner and the
guard can co-sign a single transaction, and `Runtime.CheckWitness` succeeds for
both. Single-tx UX, no off-chain coordination beyond getting both signatures.

</template>

</StandardEntry>

</StandardsMirror>
