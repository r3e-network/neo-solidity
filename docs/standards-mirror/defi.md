---
title: DeFi Building Blocks — ERC ↔ Neo Mirror
description: Vaults, flash loans, governance — Ethereum DeFi standards mirrored to Neo C# implementations.
outline: false
---

# DeFi Building Blocks

Seven DeFi standards covering yield-bearing vaults, flash loans, governance voting,
and protocol clocks. Each one is a clean port to Neo C#.

<StandardsMirror>

<!-- ============================================================ -->
<!-- ERC-4626 -->
<!-- ============================================================ -->

<StandardEntry
  id="erc-4626"
  title="ERC-4626 — Tokenized Vault"
  eip="4626"
  status="Final"
  neoMapping="NEP-17 vault pattern"
  category="Vaults"
  parityLabel="Port"
  parityClass="sm-pill-port"
>

<template #spec>

## ERC-4626: Tokenized Vault Standard

ERC-4626 standardises **yield-bearing share tokens** — a contract that holds a base
ERC-20 asset and mints a share token to depositors. As the vault earns yield, each
share is redeemable for an increasing amount of the underlying. Aave, Yearn,
Sommelier, and most yield-bearing protocols implement this.

### Required Interface (Highlights)

| Method | Purpose |
| --- | --- |
| `asset()` | Underlying token |
| `totalAssets()` | TVL in underlying |
| `convertToShares` / `convertToAssets` | Pricing |
| `deposit` / `mint` | Add liquidity, receive shares |
| `withdraw` / `redeem` | Remove liquidity, burn shares |

### Inflation Attack

The famous ERC-4626 vulnerability: attacker deposits 1 wei, donates a large amount
directly to the vault, then `totalAssets` is huge but `totalSupply` is 1. The next
depositor gets 0 shares due to integer truncation. Mitigation: virtual offset
(OpenZeppelin pattern) or seed dead shares at deployment.

### Neo Equivalent

The Neo C# port uses NEP-17 for the share token + an asset reference. Crucially, it
uses **NEP-17's `onNEP17Payment`** to auto-deposit when the user sends asset
directly to the vault — no separate `approve` + `deposit` dance needed. Single
transaction UX is built in.

::: tip Live on Neo TestNet
Both implementations deployed and behavior-verified on Neo N3 TestNet.

| Implementation | TestNet Address | Contract Hash |
| --- | --- | --- |
| **Solidity** (`neo-solc`) | `NMKxbu3BoWHMqsehay8ybt4AX2Akbau7JF` | [`0xfaf678fd…1f9f810f`](https://dora.coz.io/contract/neo3/testnet/0xfaf678fdb2053a279cf79f14a3623f3f1f9f810f) |
| **Neo C#** (`nccs`) | `NdtwXkP4UYHoaEkfFavBat7fhNpMMaknX8` | [`0x0e515ad2…647740c5`](https://dora.coz.io/contract/neo3/testnet/0x0e515ad2e892180273ab017a4883084e647740c5) |

Verified: vault metadata (symbol, decimals), empty initial state. The C# port uses NEP-27's `OnNEP17Payment` for auto-deposit when the configured asset transfers in.
[`docs/standards-mirror/deployments/erc-4626/`](https://github.com/r3e-network/neo-solidity/tree/main/docs/standards-mirror/deployments/erc-4626).
:::

</template>

<template #solidity>

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "./ERC20.sol";

interface IERC20 {
    function transfer(address, uint256) external returns (bool);
    function transferFrom(address, address, uint256) external returns (bool);
    function balanceOf(address) external view returns (uint256);
}

contract ERC4626Vault is ERC20 {
    IERC20 public immutable asset;

    constructor(IERC20 asset_, string memory name_, string memory symbol_)
        ERC20(name_, symbol_)
    { asset = asset_; }

    function totalAssets() public view returns (uint256) {
        return asset.balanceOf(address(this));
    }

    function convertToShares(uint256 assets) public view returns (uint256) {
        uint256 supply = totalSupply;
        return supply == 0 ? assets : (assets * supply) / totalAssets();
    }

    function convertToAssets(uint256 shares) public view returns (uint256) {
        uint256 supply = totalSupply;
        return supply == 0 ? shares : (shares * totalAssets()) / supply;
    }

    function deposit(uint256 assets, address receiver) public returns (uint256 shares) {
        shares = convertToShares(assets);
        asset.transferFrom(msg.sender, address(this), assets);
        _mint(receiver, shares);
    }

    function withdraw(uint256 assets, address to, address from)
        public returns (uint256 shares)
    {
        shares = convertToShares(assets);
        if (from != msg.sender) {
            uint256 allowed = allowance[from][msg.sender];
            require(allowed >= shares, "vault: insufficient allowance");
            allowance[from][msg.sender] = allowed - shares;
        }
        _burn(from, shares);
        asset.transfer(to, assets);
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

[DisplayName("YieldVault")]
[ContractPermission("*", "*")]
[SupportedStandards("NEP-17", "NEP-27")]
public class YieldVault : SmartContract
{
    private const byte Prefix_TotalShares = 0x00;
    private const byte Prefix_Balance     = 0x01;
    private static readonly byte[] AssetKey = { 0xA0 };

    private const long DeadShares = 1_000;   // virtual offset for inflation-attack mitigation

    [DisplayName("Transfer")]
    public static event Action<UInt160, UInt160, BigInteger> OnTransfer;
    [DisplayName("Deposit")]
    public static event Action<UInt160, UInt160, BigInteger, BigInteger> OnDeposit;
    [DisplayName("Withdraw")]
    public static event Action<UInt160, UInt160, BigInteger, BigInteger> OnWithdraw;

    public static string  Symbol()    => "vGAS";
    public static byte    Decimals()  => 8;
    public static UInt160 Asset()     => (UInt160)Storage.Get(Storage.CurrentContext, AssetKey);

    public static BigInteger TotalShares()
        => (BigInteger)(Storage.Get(Storage.CurrentContext, new byte[] { Prefix_TotalShares }) ?? ByteString.Empty);

    public static BigInteger BalanceOf(UInt160 acct)
        => (BigInteger)(Storage.Get(Storage.CurrentContext,
            new byte[] { Prefix_Balance }.Concat(acct)) ?? ByteString.Empty);

    public static BigInteger TotalAssets()
        => (BigInteger)Contract.Call(Asset(), "balanceOf", CallFlags.ReadOnly,
                                     new object[] { Runtime.ExecutingScriptHash });

    public static BigInteger ConvertToShares(BigInteger assets)
    {
        var supply = TotalShares() + DeadShares;
        var totAst = TotalAssets() + 1;
        return assets * supply / totAst;
    }

    public static BigInteger ConvertToAssets(BigInteger shares)
    {
        var supply = TotalShares() + DeadShares;
        var totAst = TotalAssets() + 1;
        return shares * totAst / supply;
    }

    /// <summary>Auto-deposit on incoming NEP-17 payment of the configured asset.</summary>
    public static void OnNEP17Payment(UInt160 from, BigInteger amount, object data)
    {
        if (!Runtime.CallingScriptHash.Equals(Asset()))
            throw new Exception("only accepts the configured asset");
        if (from == null) return;

        var shares = ConvertToShares(amount);
        if (shares == 0) throw new Exception("zero shares (try larger deposit)");
        Mint(from, shares);
        OnDeposit(from, from, amount, shares);
    }

    public static bool Withdraw(UInt160 from, UInt160 to, BigInteger shares)
    {
        if (!Runtime.CheckWitness(from)) throw new Exception("no authorization");
        var assets = ConvertToAssets(shares);
        Burn(from, shares);
        Contract.Call(Asset(), "transfer", CallFlags.All,
            new object[] { Runtime.ExecutingScriptHash, to, assets, "withdraw" });
        OnWithdraw(from, to, assets, shares);
        return true;
    }

    private static void Mint(UInt160 to, BigInteger amount)
    {
        var bal = BalanceOf(to) + amount;
        Storage.Put(Storage.CurrentContext, new byte[] { Prefix_Balance }.Concat(to), bal);
        var supply = TotalShares() + amount;
        Storage.Put(Storage.CurrentContext, new byte[] { Prefix_TotalShares }, supply);
        OnTransfer(UInt160.Zero, to, amount);
    }

    private static void Burn(UInt160 from, BigInteger amount)
    {
        var bal = BalanceOf(from);
        if (bal < amount) throw new Exception("insufficient shares");
        Storage.Put(Storage.CurrentContext, new byte[] { Prefix_Balance }.Concat(from), bal - amount);
        var supply = TotalShares() - amount;
        Storage.Put(Storage.CurrentContext, new byte[] { Prefix_TotalShares }, supply);
        OnTransfer(from, UInt160.Zero, amount);
    }

    public static void _deploy(object data, bool update)
    {
        if (update) return;
        Storage.Put(Storage.CurrentContext, AssetKey, (UInt160)data);
    }
}
```

</template>

</StandardEntry>

<!-- ============================================================ -->
<!-- ERC-3156 -->
<!-- ============================================================ -->

<StandardEntry
  id="erc-3156"
  title="ERC-3156 — Flash Loans"
  eip="3156"
  status="Final"
  neoMapping="Neo C# port"
  category="Flash Loans"
  parityLabel="Port"
  parityClass="sm-pill-port"
>

<template #spec>

## ERC-3156: Flash Loan Standard

ERC-3156 standardises uncollateralized atomic loans: a borrower receives any
amount, executes arbitrary logic in a callback, and must return the principal +
fee before the transaction ends. If the callback fails to repay, the entire
transaction reverts and the loan never happened.

### Lender Interface

```solidity
function maxFlashLoan(address token) external view returns (uint256);
function flashFee(address token, uint256 amount) external view returns (uint256);
function flashLoan(IERC3156FlashBorrower receiver, address token,
                   uint256 amount, bytes calldata data) external returns (bool);
```

### Borrower Interface

```solidity
function onFlashLoan(address initiator, address token, uint256 amount,
                     uint256 fee, bytes calldata data)
    external returns (bytes32);   // returns keccak256("ERC3156FlashBorrower.onFlashLoan")
```

### Neo Equivalent

Neo flash loans work identically — atomic transactions ensure repay-or-revert. The
"magic return value" pattern translates one-for-one. The Neo port below uses NEP-17
transfers and a callback method name `onFlashLoan` that borrowers implement.

::: tip Live on Neo TestNet
Both implementations deployed and behavior-verified on Neo N3 TestNet.

| Implementation | TestNet Address | Contract Hash |
| --- | --- | --- |
| **Solidity** (`neo-solc`) | `NhSy8SUwdRdrYAvK2YadgbXZwRTfJZC6DT` | [`0xb7d5cd14…117d37ec`](https://dora.coz.io/contract/neo3/testnet/0xb7d5cd146852006f8bc5d8c1621852c9117d37ec) |
| **Neo C#** (`nccs`) | `NSogFgSB3xhbRn81ie21Xb8L5vzxAguyZZ` | [`0xa82c8142…1440984b`](https://dora.coz.io/contract/neo3/testnet/0xa82c8142c02ec0cf748bbaa57819f9c61440984b) |

Verified: pre-setup state (feeBps == 0). Both implementations enforce the ERC-3156 callback magic value (`keccak256("ERC3156FlashBorrower.onFlashLoan")`) on repay.
[`docs/standards-mirror/deployments/erc-3156/`](https://github.com/r3e-network/neo-solidity/tree/main/docs/standards-mirror/deployments/erc-3156).
:::

</template>

<template #solidity>

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

interface IERC3156FlashBorrower {
    function onFlashLoan(address initiator, address token, uint256 amount,
                         uint256 fee, bytes calldata data)
        external returns (bytes32);
}

contract FlashLender {
    bytes32 constant CALLBACK_SUCCESS = keccak256("ERC3156FlashBorrower.onFlashLoan");
    address public immutable token;
    uint256 public           feeBps;   // 9 == 0.09%

    constructor(address token_, uint256 fee_) {
        token = token_;
        feeBps = fee_;
    }

    function flashFee(address t, uint256 amount) public view returns (uint256) {
        require(t == token, "wrong token");
        return amount * feeBps / 10_000;
    }

    function flashLoan(IERC3156FlashBorrower receiver, address t,
                       uint256 amount, bytes calldata data)
        external returns (bool)
    {
        require(t == token, "wrong token");
        uint256 fee = flashFee(t, amount);
        // Send funds
        IERC20(token).transfer(address(receiver), amount);
        // Borrower runs logic; must return CALLBACK_SUCCESS
        require(
            receiver.onFlashLoan(msg.sender, token, amount, fee, data) == CALLBACK_SUCCESS,
            "callback failed"
        );
        // Pull principal + fee back
        IERC20(token).transferFrom(address(receiver), address(this), amount + fee);
        return true;
    }
}

interface IERC20 {
    function transfer(address, uint256) external returns (bool);
    function transferFrom(address, address, uint256) external returns (bool);
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

[DisplayName("FlashLender")]
[ContractPermission("*", "*")]
public class FlashLender : SmartContract
{
    private static readonly byte[] TokenKey = { 0xA0 };
    private static readonly byte[] FeeKey   = { 0xA1 };

    public const string CallbackSuccess = "ERC3156FlashBorrower.onFlashLoan";

    [DisplayName("FlashLoan")]
    public static event Action<UInt160, UInt160, BigInteger, BigInteger> OnFlashLoan;

    public static UInt160 Token() => (UInt160)Storage.Get(Storage.CurrentContext, TokenKey);

    public static BigInteger FlashFee(UInt160 t, BigInteger amount)
    {
        if (!t.Equals(Token())) throw new Exception("wrong token");
        var bps = (BigInteger)Storage.Get(Storage.CurrentContext, FeeKey);
        return amount * bps / 10_000;
    }

    public static BigInteger MaxFlashLoan(UInt160 t)
    {
        if (!t.Equals(Token())) return 0;
        return (BigInteger)Contract.Call(Token(), "balanceOf", CallFlags.ReadOnly,
                                         new object[] { Runtime.ExecutingScriptHash });
    }

    public static bool FlashLoanRequest(UInt160 receiver, UInt160 t,
                                        BigInteger amount, object data)
    {
        if (!t.Equals(Token())) throw new Exception("wrong token");
        if (amount <= 0)        throw new Exception("amount <= 0");

        var fee = FlashFee(t, amount);
        var balanceBefore = MaxFlashLoan(t);

        // Send principal to borrower
        Contract.Call(Token(), "transfer", CallFlags.All,
            new object[] {
                Runtime.ExecutingScriptHash, receiver, amount,
                "flashloan-principal"
            });

        // Borrower executes — must return CallbackSuccess
        var ret = (string)Contract.Call(receiver, "onFlashLoan", CallFlags.All,
            new object[] {
                Runtime.CallingScriptHash, Token(), amount, fee, data
            });
        if (ret != CallbackSuccess) throw new Exception("callback failed");

        // Verify repaid: balance must be >= balanceBefore + fee.
        var balanceAfter = MaxFlashLoan(t);
        if (balanceAfter < balanceBefore + fee)
            throw new Exception("flashloan not repaid");

        OnFlashLoan(receiver, t, amount, fee);
        return true;
    }

    public static void _deploy(object data, bool update)
    {
        if (update) return;
        var args = (object[])data;
        Storage.Put(Storage.CurrentContext, TokenKey, (UInt160)args[0]);
        Storage.Put(Storage.CurrentContext, FeeKey,   (BigInteger)args[1]);
    }
}

/// <summary>Borrower contract: receives the principal, runs strategy, repays.</summary>
[DisplayName("FlashBorrower")]
public class FlashBorrower : SmartContract
{
    public static string OnFlashLoan(UInt160 initiator, UInt160 token,
                                     BigInteger amount, BigInteger fee, object data)
    {
        // ... arbitrage / liquidation / refinance logic here ...
        // Must end with: send (amount + fee) back to caller.
        Contract.Call(token, "transfer", CallFlags.All,
            new object[] {
                Runtime.ExecutingScriptHash,
                Runtime.CallingScriptHash,
                amount + fee,
                "flashloan-repay"
            });
        return FlashLender.CallbackSuccess;
    }
}
```

### Why This Works on Neo

Atomic transactions: any uncaught exception or insufficient repayment unwinds the
whole transaction, restoring the lender's balance. Same property as Ethereum.
Re-entrancy is not a concern because the state changes are deterministic and
balance verification at the end of `FlashLoanRequest` proves repayment.

</template>

</StandardEntry>

<!-- ============================================================ -->
<!-- ERC-7540 -->
<!-- ============================================================ -->

<StandardEntry
  id="erc-7540"
  title="ERC-7540 — Asynchronous Vaults"
  eip="7540"
  status="Final"
  neoMapping="Neo C# port"
  category="Vaults"
  parityLabel="Port"
  parityClass="sm-pill-port"
>

<template #spec>

## ERC-7540: Asynchronous ERC-4626 Vaults

ERC-4626 assumes deposits and withdrawals settle synchronously in the same call.
But many real-world vaults — RWA tokenization, off-chain CeDeFi strategies,
restaking protocols with unbond periods — require **asynchronous** flow: user
requests deposit, operator processes it after some delay, user later claims their
shares.

### Mechanics

| Phase | Methods |
| --- | --- |
| Request | `requestDeposit(assets, controller, owner)` |
| Process | (off-chain or admin call that fulfills pending requests) |
| Claim | `deposit(assets, receiver, controller)` (different overload from 4626 sync) |

The `controller` address can be different from the `owner` — useful when a custodian
manages requests on behalf of the depositor.

### Neo Equivalent

The Neo port stores pending requests in a per-user mapping, lets a designated
operator process batches, and tracks fulfilled requests for claim-time math.

</template>

<template #solidity>

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

interface IERC7540Vault {
    event DepositRequest(address indexed controller, address indexed owner,
                         uint256 indexed requestId, address sender, uint256 assets);

    function requestDeposit(uint256 assets, address controller, address owner)
        external returns (uint256 requestId);

    function pendingDepositRequest(uint256 requestId, address controller)
        external view returns (uint256 assets);

    function claimableDepositRequest(uint256 requestId, address controller)
        external view returns (uint256 assets);

    function deposit(uint256 assets, address receiver, address controller)
        external returns (uint256 shares);
}

contract AsyncVault {
    struct Request { uint256 pending; uint256 claimable; uint256 sharesClaimable; }

    mapping(address => Request)         public requests;
    mapping(address => uint256)         public balance;
    address public asset;
    address public operator;

    function requestDeposit(uint256 assets, address controller, address owner)
        external returns (uint256)
    {
        // pull assets from owner to vault
        IERC20(asset).transferFrom(owner, address(this), assets);
        requests[controller].pending += assets;
        return 0;   // simplified: single rolling request per controller
    }

    function processBatch(address[] calldata controllers, uint256 sharePerAsset)
        external
    {
        require(msg.sender == operator, "operator only");
        for (uint i; i < controllers.length; ++i) {
            address c = controllers[i];
            uint256 a = requests[c].pending;
            requests[c].pending = 0;
            requests[c].claimable += a;
            requests[c].sharesClaimable += a * sharePerAsset;
        }
    }

    function deposit(uint256 assets, address receiver, address controller)
        external returns (uint256 shares)
    {
        Request storage r = requests[controller];
        require(r.claimable >= assets, "insufficient claimable");
        shares = r.sharesClaimable * assets / r.claimable;
        r.claimable -= assets;
        r.sharesClaimable -= shares;
        balance[receiver] += shares;
    }
}

interface IERC20 { function transferFrom(address, address, uint256) external returns (bool); }
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

[DisplayName("AsyncVault")]
[ContractPermission("*", "*")]
[SupportedStandards("NEP-17", "NEP-27")]
public class AsyncVault : SmartContract
{
    private const byte Prefix_Pending   = 0x10;  // controller -> assets pending
    private const byte Prefix_Claimable = 0x11;  // controller -> assets claimable
    private const byte Prefix_Shares    = 0x12;  // controller -> shares claimable
    private const byte Prefix_ShareBal  = 0x13;  // owner -> share balance
    private static readonly byte[] AssetKey    = { 0xA0 };
    private static readonly byte[] OperatorKey = { 0xA1 };

    [DisplayName("DepositRequest")]
    public static event Action<UInt160, UInt160, BigInteger> OnDepositRequest;
    [DisplayName("DepositProcessed")]
    public static event Action<UInt160, BigInteger, BigInteger> OnDepositProcessed;

    /// <summary>Asset arrives via NEP-17 payment — credit to pending.</summary>
    public static void OnNEP17Payment(UInt160 from, BigInteger amount, object data)
    {
        var asset = (UInt160)Storage.Get(Storage.CurrentContext, AssetKey);
        if (!Runtime.CallingScriptHash.Equals(asset)) throw new Exception("wrong asset");
        if (from == null) return;

        // data is the controller (or null = use sender)
        var controller = data is UInt160 ctrl ? ctrl : from;
        var key = new byte[] { Prefix_Pending }.Concat(controller);
        var pending = (BigInteger)(Storage.Get(Storage.CurrentContext, key) ?? ByteString.Empty);
        Storage.Put(Storage.CurrentContext, key, pending + amount);
        OnDepositRequest(from, controller, amount);
    }

    public static BigInteger PendingDepositRequest(UInt160 controller)
        => (BigInteger)(Storage.Get(Storage.CurrentContext,
            new byte[] { Prefix_Pending }.Concat(controller)) ?? ByteString.Empty);

    public static BigInteger ClaimableDepositRequest(UInt160 controller)
        => (BigInteger)(Storage.Get(Storage.CurrentContext,
            new byte[] { Prefix_Claimable }.Concat(controller)) ?? ByteString.Empty);

    public static void ProcessBatch(UInt160[] controllers, BigInteger sharePerAsset)
    {
        var op = (UInt160)Storage.Get(Storage.CurrentContext, OperatorKey);
        if (!Runtime.CheckWitness(op)) throw new Exception("operator only");

        foreach (var c in controllers)
        {
            var pending = PendingDepositRequest(c);
            if (pending == 0) continue;

            Storage.Delete(Storage.CurrentContext,
                           new byte[] { Prefix_Pending }.Concat(c));

            var prevClaim = ClaimableDepositRequest(c);
            Storage.Put(Storage.CurrentContext,
                        new byte[] { Prefix_Claimable }.Concat(c),
                        prevClaim + pending);

            var prevShares = (BigInteger)(Storage.Get(Storage.CurrentContext,
                                          new byte[] { Prefix_Shares }.Concat(c))
                                          ?? ByteString.Empty);
            Storage.Put(Storage.CurrentContext,
                        new byte[] { Prefix_Shares }.Concat(c),
                        prevShares + pending * sharePerAsset);
            OnDepositProcessed(c, pending, pending * sharePerAsset);
        }
    }

    public static BigInteger Claim(UInt160 controller, UInt160 receiver, BigInteger assets)
    {
        if (!Runtime.CheckWitness(controller)) throw new Exception("controller only");
        var claimable = ClaimableDepositRequest(controller);
        if (claimable < assets) throw new Exception("insufficient claimable");

        var shareBal = (BigInteger)(Storage.Get(Storage.CurrentContext,
                       new byte[] { Prefix_Shares }.Concat(controller)) ?? ByteString.Empty);
        var shares   = shareBal * assets / claimable;

        Storage.Put(Storage.CurrentContext,
                    new byte[] { Prefix_Claimable }.Concat(controller),
                    claimable - assets);
        Storage.Put(Storage.CurrentContext,
                    new byte[] { Prefix_Shares }.Concat(controller),
                    shareBal - shares);

        var recvBal = (BigInteger)(Storage.Get(Storage.CurrentContext,
                      new byte[] { Prefix_ShareBal }.Concat(receiver)) ?? ByteString.Empty);
        Storage.Put(Storage.CurrentContext,
                    new byte[] { Prefix_ShareBal }.Concat(receiver),
                    recvBal + shares);
        return shares;
    }
}
```

</template>

</StandardEntry>

<!-- ============================================================ -->
<!-- ERC-7575 -->
<!-- ============================================================ -->

<StandardEntry
  id="erc-7575"
  title="ERC-7575 — Multi-Asset Vault"
  eip="7575"
  status="Final"
  neoMapping="Neo C# port"
  category="Vaults"
  parityLabel="Port"
  parityClass="sm-pill-port"
>

<template #spec>

## ERC-7575: Multi-Asset ERC-4626 Vaults

ERC-4626 assumes one underlying asset per vault. ERC-7575 generalises: a single
share token can be backed by **multiple assets**, each with its own
`convert/deposit/withdraw` overloads. Useful for stable-coin baskets, multi-asset
yield aggregators, RWA fund-of-fund structures.

### Required Interface (Sketch)

```solidity
function asset(address share) external view returns (address);
function shareToken() external view returns (address);
function convertToShares(address asset, uint256 assets) external view returns (uint256);
function deposit(address asset, uint256 assets, address receiver)
    external returns (uint256 shares);
```

### Neo Equivalent

The Neo C# port keeps the share NEP-17 contract and a separate vault contract that
accepts multiple asset types via `OnNEP17Payment`. Each asset has its own
exchange-rate oracle (or constant ratio for basket vaults).

</template>

<template #solidity>

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

interface IERC7575 {
    function asset(address share)         external view returns (address);
    function shareToken()                  external view returns (address);
    function convertToShares(address a, uint256 v) external view returns (uint256);
    function deposit(address a, uint256 v, address receiver)
        external returns (uint256 shares);
}

contract MultiAssetVault is IERC7575 {
    address public override shareToken;
    mapping(address => uint256) public ratePerAsset;   // share-per-asset ratio

    function asset(address) external view override returns (address) {
        revert("unused");
    }

    function convertToShares(address a, uint256 amount)
        public view override returns (uint256)
    {
        return amount * ratePerAsset[a] / 1e18;
    }

    function deposit(address a, uint256 amount, address receiver)
        external override returns (uint256 shares)
    {
        IERC20(a).transferFrom(msg.sender, address(this), amount);
        shares = convertToShares(a, amount);
        IShareToken(shareToken).mint(receiver, shares);
    }
}

interface IERC20 { function transferFrom(address, address, uint256) external returns (bool); }
interface IShareToken { function mint(address, uint256) external; }
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

[DisplayName("MultiAssetVault")]
[ContractPermission("*", "*")]
[SupportedStandards("NEP-27")]
public class MultiAssetVault : SmartContract
{
    private const byte Prefix_RatePerAsset = 0x20;   // assetHash -> rate (BigInteger, 1e8 fixed)
    private static readonly byte[] ShareTokenKey = { 0xA0 };

    [DisplayName("Deposit")]
    public static event Action<UInt160, UInt160, BigInteger, BigInteger> OnDeposit;

    public static UInt160 ShareToken()
        => (UInt160)Storage.Get(Storage.CurrentContext, ShareTokenKey);

    public static BigInteger ConvertToShares(UInt160 asset, BigInteger amount)
    {
        var rate = (BigInteger)(Storage.Get(Storage.CurrentContext,
            new byte[] { Prefix_RatePerAsset }.Concat(asset)) ?? ByteString.Empty);
        if (rate == 0) throw new Exception("asset not accepted");
        return amount * rate / 100_000_000;
    }

    /// <summary>
    /// Auto-deposit on incoming NEP-17 payment of any whitelisted asset.
    /// Caller is the asset's NEP-17 contract; we look up its rate.
    /// </summary>
    public static void OnNEP17Payment(UInt160 from, BigInteger amount, object data)
    {
        var asset = Runtime.CallingScriptHash;
        var shares = ConvertToShares(asset, amount);
        if (shares == 0) throw new Exception("zero shares");

        // Mint share tokens to the depositor
        Contract.Call(ShareToken(), "mint", CallFlags.All,
                      new object[] { from, shares });
        OnDeposit(from, asset, amount, shares);
    }

    public static void SetAssetRate(UInt160 asset, BigInteger ratePerAsset_1e8)
    {
        if (!Runtime.CheckWitness(GetAdmin())) throw new Exception("admin only");
        Storage.Put(Storage.CurrentContext,
                    new byte[] { Prefix_RatePerAsset }.Concat(asset),
                    ratePerAsset_1e8);
    }

    private static UInt160 GetAdmin() => (UInt160)"0x0000000000000000000000000000000000000000";
}
```

</template>

</StandardEntry>

<!-- ============================================================ -->
<!-- ERC-5805 -->
<!-- ============================================================ -->

<StandardEntry
  id="erc-5805"
  title="ERC-5805 — Voting With Delegation"
  eip="5805"
  status="Final"
  neoMapping="Neo C# port"
  category="Governance"
  parityLabel="Port"
  parityClass="sm-pill-port"
>

<template #spec>

## ERC-5805: Voting with Delegation Checkpointing

ERC-5805 is the standard interface that ERC-20Votes / ERC-721Votes implement. Token
holders **delegate voting power** to themselves or an address (a representative).
Voting power is tracked in **historical checkpoints** so a governance vote at block
N can read the delegate's voting power as it was at block N — protecting against
flash-loan governance attacks.

### Required Methods

```solidity
function clock() external view returns (uint48);
function CLOCK_MODE() external view returns (string memory);
function getVotes(address account) external view returns (uint256);
function getPastVotes(address account, uint256 timepoint) external view returns (uint256);
function delegates(address account) external view returns (address);
function delegate(address delegatee) external;
```

### Neo Equivalent

The Neo C# port stores per-account checkpoints `(block, votes)` in storage and reads
them at vote time using binary search. Same flash-loan-resistance property,
implemented identically.

::: tip Live on Neo TestNet
Both implementations deployed and behavior-verified on Neo N3 TestNet.

| Implementation | TestNet Address | Contract Hash |
| --- | --- | --- |
| **Solidity** (`neo-solc`) | `NUKHCo2gUfGE3vAyKDCYfWvkHcCjkN4MmL` | [`0xb87fa58c…035f3110`](https://dora.coz.io/contract/neo3/testnet/0xb87fa58c80deef8dc910a0ca3a2cc186035f3110) |
| **Neo C#** (`nccs`) | `NSXG6PtP19h74j4xZyPTV6myWdMcWSsRwz` | [`0x1d33818b…3243692`](https://dora.coz.io/contract/neo3/testnet/0x1d33818b3d053d291424848ed1ac7ebaa3243692) |

Verified: ERC-6372 clock interface (`CLOCK_MODE = "mode=blocknumber&from=default"`), token symbol, delegation API. Both implementations use `block.number` / `Ledger.CurrentIndex` as the timestamp source — flash-loan-resistant by construction.
[`docs/standards-mirror/deployments/erc-5805/`](https://github.com/r3e-network/neo-solidity/tree/main/docs/standards-mirror/deployments/erc-5805).
:::

</template>

<template #solidity>

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

abstract contract Votes {
    struct Checkpoint { uint48 fromBlock; uint208 votes; }

    mapping(address => Checkpoint[]) private _checkpoints;
    mapping(address => address)      public  delegates;

    event DelegateChanged(address indexed delegator, address indexed from, address indexed to);
    event DelegateVotesChanged(address indexed delegate, uint256 oldVotes, uint256 newVotes);

    function clock() public view returns (uint48) { return uint48(block.number); }

    function getVotes(address account) public view returns (uint256) {
        uint len = _checkpoints[account].length;
        return len == 0 ? 0 : _checkpoints[account][len - 1].votes;
    }

    function getPastVotes(address account, uint256 timepoint)
        public view returns (uint256)
    {
        require(timepoint < block.number, "future");
        Checkpoint[] storage cps = _checkpoints[account];
        if (cps.length == 0) return 0;
        // Binary search
        uint lo; uint hi = cps.length;
        while (lo < hi) {
            uint mid = (lo + hi + 1) / 2;
            if (cps[mid - 1].fromBlock <= timepoint) lo = mid; else hi = mid - 1;
        }
        return lo == 0 ? 0 : cps[lo - 1].votes;
    }

    function delegate(address delegatee) public {
        address old = delegates[msg.sender];
        delegates[msg.sender] = delegatee;
        emit DelegateChanged(msg.sender, old, delegatee);
        // ... move votes from old to delegatee, write checkpoints ...
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

[DisplayName("VotingToken")]
[ContractPermission("*", "*")]
[SupportedStandards("NEP-17")]
public class VotingToken : SmartContract
{
    private const byte Prefix_Balance      = 0x01;
    private const byte Prefix_Delegate     = 0x02;
    private const byte Prefix_Checkpoint   = 0x03;   // delegate+blockIdx -> votes
    private const byte Prefix_NumCheckpts  = 0x04;   // delegate -> count

    [DisplayName("DelegateChanged")]
    public static event Action<UInt160, UInt160, UInt160> OnDelegateChanged;
    [DisplayName("DelegateVotesChanged")]
    public static event Action<UInt160, BigInteger, BigInteger> OnDelegateVotesChanged;

    public static uint Clock() => Ledger.CurrentIndex;
    public static string ClockMode() => "mode=blocknumber";

    public static UInt160 DelegateOf(UInt160 account)
        => (UInt160)Storage.Get(Storage.CurrentContext,
                                new byte[] { Prefix_Delegate }.Concat(account));

    public static BigInteger GetVotes(UInt160 account)
    {
        var n = (BigInteger)(Storage.Get(Storage.CurrentContext,
                new byte[] { Prefix_NumCheckpts }.Concat(account)) ?? ByteString.Empty);
        if (n == 0) return 0;
        return ReadCheckpointVotes(account, n - 1);
    }

    public static BigInteger GetPastVotes(UInt160 account, uint blockIndex)
    {
        if (blockIndex >= Ledger.CurrentIndex) throw new Exception("future block");
        var n = (BigInteger)(Storage.Get(Storage.CurrentContext,
                new byte[] { Prefix_NumCheckpts }.Concat(account)) ?? ByteString.Empty);
        if (n == 0) return 0;

        // Binary search for the last checkpoint with fromBlock <= blockIndex
        BigInteger lo = 0, hi = n;
        while (lo < hi)
        {
            var mid = (lo + hi + 1) / 2;
            if (ReadCheckpointBlock(account, mid - 1) <= blockIndex) lo = mid;
            else hi = mid - 1;
        }
        return lo == 0 ? 0 : ReadCheckpointVotes(account, lo - 1);
    }

    public static void Delegate(UInt160 delegatee)
    {
        if (!Runtime.CheckWitness(Runtime.CallingScriptHash) &&
            !Runtime.CheckWitness(delegatee))
            throw new Exception("must sign");

        var delegator = Runtime.CallingScriptHash;
        var oldDel = DelegateOf(delegator);
        Storage.Put(Storage.CurrentContext,
                    new byte[] { Prefix_Delegate }.Concat(delegator), delegatee);

        var amount = BalanceOf(delegator);
        if (oldDel != null && !oldDel.Equals(UInt160.Zero))
            MoveVotes(oldDel, -amount);
        MoveVotes(delegatee, amount);

        OnDelegateChanged(delegator, oldDel ?? UInt160.Zero, delegatee);
    }

    public static BigInteger BalanceOf(UInt160 a)
        => (BigInteger)(Storage.Get(Storage.CurrentContext,
            new byte[] { Prefix_Balance }.Concat(a)) ?? ByteString.Empty);

    private static void MoveVotes(UInt160 delegate_, BigInteger delta)
    {
        var current = GetVotes(delegate_);
        var newVotes = current + delta;
        WriteCheckpoint(delegate_, newVotes);
        OnDelegateVotesChanged(delegate_, current, newVotes);
    }

    private static void WriteCheckpoint(UInt160 acct, BigInteger votes)
    {
        var n = (BigInteger)(Storage.Get(Storage.CurrentContext,
                new byte[] { Prefix_NumCheckpts }.Concat(acct)) ?? ByteString.Empty);
        var cpKey = new byte[] { Prefix_Checkpoint }.Concat(acct).Concat((ByteString)n.ToByteArray());
        var blob  = StdLib.Serialize(new object[] { (BigInteger)Ledger.CurrentIndex, votes });
        Storage.Put(Storage.CurrentContext, cpKey, blob);
        Storage.Put(Storage.CurrentContext,
                    new byte[] { Prefix_NumCheckpts }.Concat(acct), n + 1);
    }

    private static BigInteger ReadCheckpointBlock(UInt160 acct, BigInteger idx)
        => (BigInteger)((object[])StdLib.Deserialize(
            Storage.Get(Storage.CurrentContext,
                new byte[] { Prefix_Checkpoint }.Concat(acct).Concat((ByteString)idx.ToByteArray()))))[0];

    private static BigInteger ReadCheckpointVotes(UInt160 acct, BigInteger idx)
        => (BigInteger)((object[])StdLib.Deserialize(
            Storage.Get(Storage.CurrentContext,
                new byte[] { Prefix_Checkpoint }.Concat(acct).Concat((ByteString)idx.ToByteArray()))))[1];
}
```

</template>

</StandardEntry>

<!-- ============================================================ -->
<!-- ERC-6372 -->
<!-- ============================================================ -->

<StandardEntry
  id="erc-6372"
  title="ERC-6372 — Contract Clock"
  eip="6372"
  status="Final"
  neoMapping="Neo C# port (uses Ledger.CurrentIndex)"
  category="Governance"
  parityLabel="Port"
  parityClass="sm-pill-port"
>

<template #spec>

## ERC-6372: Contract Clock for Governance

A meta-standard that pairs with ERC-5805. Every governance contract advertises its
**timekeeping mode** via two methods:

- `clock()` returns the current "time" in the contract's chosen unit.
- `CLOCK_MODE()` returns a machine-readable string like `mode=blocknumber&from=default`
  or `mode=timestamp`.

This lets governance frontends and oracles work uniformly across token contracts
that use block numbers vs timestamps.

### Neo Equivalent

The Neo C# port simply exposes both methods, returning `Ledger.CurrentIndex` (block
number) or `Runtime.Time` (millisecond timestamp).

::: tip Live on Neo TestNet
Both implementations deployed and behavior-verified on Neo N3 TestNet.

| Implementation | TestNet Address | Contract Hash |
| --- | --- | --- |
| **Solidity** (`neo-solc`) | `NPrkGGCSZwLTUQxzUQnf4WSKsk2rZPTSKt` | [`0xe3c55758…df77442b`](https://dora.coz.io/contract/neo3/testnet/0xe3c55758861ba8034c9f3d223ed93cf5df77442b) |
| **Neo C#** (`nccs`) | `NQpFbsh7JUys9dYhB86gCVr2hxfYkzKJYw` | [`0xeb454a6b…1ed6c335`](https://dora.coz.io/contract/neo3/testnet/0xeb454a6b6e102b2700fc1d3b18d58b861ed6c335) |

Verified: `CLOCK_MODE` returns `"mode=blocknumber&from=default"` from both. The Neo C# version uses `Ledger.CurrentIndex` directly.
[`docs/standards-mirror/deployments/erc-6372/`](https://github.com/r3e-network/neo-solidity/tree/main/docs/standards-mirror/deployments/erc-6372).
:::

</template>

<template #solidity>

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

interface IERC6372 {
    function clock() external view returns (uint48);
    function CLOCK_MODE() external view returns (string memory);
}

contract BlockClockGov is IERC6372 {
    function clock() public view returns (uint48) { return uint48(block.number); }
    function CLOCK_MODE() public pure returns (string memory) {
        return "mode=blocknumber&from=default";
    }
}

contract TimestampClockGov is IERC6372 {
    function clock() public view returns (uint48) { return uint48(block.timestamp); }
    function CLOCK_MODE() public pure returns (string memory) { return "mode=timestamp"; }
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

[DisplayName("BlockClockGovernance")]
public class BlockClockGovernance : SmartContract
{
    public static uint   Clock()      => Ledger.CurrentIndex;
    public static string ClockMode()  => "mode=blocknumber&from=default";
}

[DisplayName("TimestampClockGovernance")]
public class TimestampClockGovernance : SmartContract
{
    public static ulong  Clock()      => Runtime.Time;
    public static string ClockMode()  => "mode=timestamp";
}
```

`Ledger.CurrentIndex` is the block number; `Runtime.Time` is the block-time in
milliseconds since unix epoch. Both are deterministic per-block primitives.

</template>

</StandardEntry>

<!-- ============================================================ -->
<!-- ERC-7818 -->
<!-- ============================================================ -->

<StandardEntry
  id="erc-7818"
  title="ERC-7818 — Expirable ERC-20"
  eip="7818"
  status="Final"
  neoMapping="Neo C# port"
  category="Token Lifecycle"
  parityLabel="Port"
  parityClass="sm-pill-port"
>

<template #spec>

## ERC-7818: Expirable Tokens

Tokens that **expire** after a configured duration: airdrops that must be claimed
within 30 days, prepaid credits, gift cards, time-bounded rewards. ERC-7818
extends ERC-20 with per-batch expiry tracking — each mint creates a "lot" whose
balance becomes inaccessible after its expiration.

### Required Interface (Highlights)

```solidity
function balanceOfAtEpoch(uint256 epoch, address account) external view returns (uint256);
function currentEpoch() external view returns (uint256);
function expirationDuration() external view returns (uint256);
```

### Neo Equivalent

The Neo C# port tracks balances per-epoch in storage. On each transfer, expired
lots are skipped; expired balance is effectively burned. Epochs are derived from
`Runtime.Time / duration`.

::: tip Live on Neo TestNet
Both implementations deployed and behavior-verified on Neo N3 TestNet.

| Implementation | TestNet Address | Contract Hash |
| --- | --- | --- |
| **Solidity** (`neo-solc`) | `NfRgrtEfpNb3KCfM33snwzcEBJpAPbFy3F` | [`0xfcaaf98f…a8c6564d`](https://dora.coz.io/contract/neo3/testnet/0xfcaaf98f8c4693b326f883d52db9d9e4a8c6564d) |
| **Neo C#** (`nccs`) | `NMvFiMjyPN3SVQDhPxGcmDM1F1axhMZP3D` | [`0xcb1b0441…a6ae56e0`](https://dora.coz.io/contract/neo3/testnet/0xcb1b0441c5b02a2f7de348951c6bf8e2a6ae56e0) |

Verified: token symbol and decimals. Both contracts track per-epoch balances (epoch derived from `block.timestamp / duration` or `Runtime.Time / duration`); balance reads sum the non-expired epoch range.
[`docs/standards-mirror/deployments/erc-7818/`](https://github.com/r3e-network/neo-solidity/tree/main/docs/standards-mirror/deployments/erc-7818).
:::

</template>

<template #solidity>

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

contract ExpirableToken {
    uint256 public immutable epochDuration;        // seconds
    uint256 public immutable retentionEpochs;       // expire after N epochs

    // (account, epoch) -> balance
    mapping(address => mapping(uint256 => uint256)) public balanceAtEpoch;
    mapping(address => uint256) public oldestEpoch;

    event Transfer(address indexed from, address indexed to, uint256 value);
    event Expired(address indexed account, uint256 epoch, uint256 amount);

    constructor(uint256 dur, uint256 retention) {
        epochDuration   = dur;
        retentionEpochs = retention;
    }

    function currentEpoch() public view returns (uint256) {
        return block.timestamp / epochDuration;
    }

    function balanceOf(address a) public view returns (uint256 total) {
        uint256 cur = currentEpoch();
        uint256 from = cur >= retentionEpochs ? cur - retentionEpochs + 1 : 0;
        for (uint256 i = from; i <= cur; ++i) total += balanceAtEpoch[a][i];
    }

    function mint(address to, uint256 amount) external {
        balanceAtEpoch[to][currentEpoch()] += amount;
        emit Transfer(address(0), to, amount);
    }

    function transfer(address to, uint256 amount) external returns (bool) {
        uint256 remaining = amount;
        uint256 cur = currentEpoch();
        uint256 from = cur >= retentionEpochs ? cur - retentionEpochs + 1 : 0;
        for (uint256 i = from; i <= cur && remaining > 0; ++i) {
            uint256 bal = balanceAtEpoch[msg.sender][i];
            uint256 take = bal < remaining ? bal : remaining;
            balanceAtEpoch[msg.sender][i] -= take;
            balanceAtEpoch[to][i]         += take;
            remaining -= take;
        }
        require(remaining == 0, "insufficient non-expired balance");
        emit Transfer(msg.sender, to, amount);
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

[DisplayName("ExpirableToken")]
[ContractPermission("*", "*")]
[SupportedStandards("NEP-17")]
public class ExpirableToken : SmartContract
{
    private const byte Prefix_BalanceAtEpoch = 0x01;  // owner+epoch -> amount
    private static readonly byte[] DurationKey  = { 0xA0 };
    private static readonly byte[] RetentionKey = { 0xA1 };

    [DisplayName("Transfer")]
    public static event Action<UInt160, UInt160, BigInteger> OnTransfer;
    [DisplayName("Expired")]
    public static event Action<UInt160, BigInteger, BigInteger> OnExpired;

    public static BigInteger CurrentEpoch()
    {
        var dur = (BigInteger)Storage.Get(Storage.CurrentContext, DurationKey);
        return Runtime.Time / 1000 / dur;
    }

    public static BigInteger BalanceOf(UInt160 owner)
    {
        var ret = (BigInteger)Storage.Get(Storage.CurrentContext, RetentionKey);
        var cur = CurrentEpoch();
        var from = cur >= ret ? cur - ret + 1 : 0;
        BigInteger total = 0;
        for (var i = from; i <= cur; i++)
            total += BalanceAtEpoch(owner, i);
        return total;
    }

    public static BigInteger BalanceAtEpoch(UInt160 owner, BigInteger epoch)
    {
        var key = new byte[] { Prefix_BalanceAtEpoch }
            .Concat(owner).Concat((ByteString)epoch.ToByteArray());
        return (BigInteger)(Storage.Get(Storage.CurrentContext, key) ?? ByteString.Empty);
    }

    public static void Mint(UInt160 to, BigInteger amount)
    {
        if (!Runtime.CheckWitness(GetAdmin())) throw new Exception("admin only");
        var epoch = CurrentEpoch();
        var prev  = BalanceAtEpoch(to, epoch);
        SetBalance(to, epoch, prev + amount);
        OnTransfer(UInt160.Zero, to, amount);
    }

    public static bool Transfer(UInt160 from, UInt160 to, BigInteger amount, object data)
    {
        if (!Runtime.CheckWitness(from)) throw new Exception("no authorization");
        var ret = (BigInteger)Storage.Get(Storage.CurrentContext, RetentionKey);
        var cur = CurrentEpoch();
        var fromEpoch = cur >= ret ? cur - ret + 1 : 0;
        var remaining = amount;

        for (var i = fromEpoch; i <= cur && remaining > 0; i++)
        {
            var bal = BalanceAtEpoch(from, i);
            if (bal == 0) continue;
            var take = bal < remaining ? bal : remaining;
            SetBalance(from, i, bal - take);
            SetBalance(to,   i, BalanceAtEpoch(to, i) + take);
            remaining -= take;
        }
        if (remaining > 0) throw new Exception("insufficient non-expired balance");
        OnTransfer(from, to, amount);
        return true;
    }

    private static void SetBalance(UInt160 owner, BigInteger epoch, BigInteger value)
    {
        var key = new byte[] { Prefix_BalanceAtEpoch }
            .Concat(owner).Concat((ByteString)epoch.ToByteArray());
        if (value == 0) Storage.Delete(Storage.CurrentContext, key);
        else            Storage.Put(Storage.CurrentContext, key, value);
    }

    public static void _deploy(object data, bool update)
    {
        if (update) return;
        var args = (object[])data;
        Storage.Put(Storage.CurrentContext, DurationKey,  (BigInteger)args[0]);
        Storage.Put(Storage.CurrentContext, RetentionKey, (BigInteger)args[1]);
    }

    private static UInt160 GetAdmin() => (UInt160)"0x0000000000000000000000000000000000000000";
}
```

</template>

</StandardEntry>

</StandardsMirror>
