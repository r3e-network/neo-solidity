---
title: Account & Authentication — ERC ↔ Neo Mirror
description: Ethereum signature, ownership, and account-abstraction standards mirrored to Neo's native witness model and idiomatic C# patterns.
outline: false
---

# Account & Authentication

Ten standards covering ownership, off-chain signatures, smart-contract signature
verification, account abstraction, and EOA-as-contract proposals. Most of these are
elaborate workarounds for Ethereum's "EOA can only do ECDSA" limitation; Neo's
witness model handles the same use cases at the protocol level.

<StandardsMirror>

<!-- ============================================================ -->
<!-- ERC-173 -->
<!-- ============================================================ -->

<StandardEntry
  id="erc-173"
  title="ERC-173 — Contract Ownership"
  eip="173"
  status="Final"
  neoMapping="Owner pattern + NEP-22"
  category="Ownership"
  parityLabel="Pattern"
  parityClass="sm-pill-pattern"
>

<template #spec>

## ERC-173: Contract Ownership Standard

ERC-173 standardises the basic admin-ownership pattern: a contract has a single
owner, ownership can be transferred, admin functions check ownership.

### Required Interface

```solidity
function owner() external view returns (address);
function transferOwnership(address newOwner) external;
event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);
```

OpenZeppelin's `Ownable` is the de facto reference; the two-step `Ownable2Step`
variant avoids the "transfer to wrong address, lose contract" footgun.

### Neo Equivalent

Neo doesn't standardise `owner()` because it would be redundant. Every Neo contract:

- Has its `_deploy` callback (NEP-29) where you set the initial owner.
- Has a manifest-defined `update` method (NEP-22) for admin operations.
- Uses `Runtime.CheckWitness(owner)` to gate any admin function.

Most production contracts expose a `getOwner` view as a convention but it's not
required by any standard.

::: tip Live on Neo TestNet
Both implementations deployed and behavior-verified on Neo N3 TestNet.

| Implementation | TestNet Address | Contract Hash |
| --- | --- | --- |
| **Solidity** (`neo-solc`) | `NgmPfqiGc6weStAM5PYBgC4SYgTGUZzPVh` | [`0x19977aea…1156bbe4`](https://dora.coz.io/contract/neo3/testnet/0x19977aea6f158de3844f3261988b17381156bbe4) |
| **Neo C#** (`nccs`) | `NU3yPrTayUJRB16Lu8RjFX8ERJ7pMYvxup` | [`0xce89aec2…11824459`](https://dora.coz.io/contract/neo3/testnet/0xce89aec2e79b121ec264231be49cd96111824459) |

Verified: `getOwner == deployer`, `claimOwnership` (Solidity), `_deploy`-time owner init (C#).
[`docs/standards-mirror/deployments/erc-173/`](https://github.com/r3e-network/neo-solidity/tree/main/docs/standards-mirror/deployments/erc-173).
:::

</template>

<template #solidity>

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

abstract contract Ownable2Step {
    address private _owner;
    address private _pendingOwner;

    event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);

    constructor(address initialOwner) { _owner = initialOwner; }

    modifier onlyOwner() {
        require(msg.sender == _owner, "Ownable: caller is not the owner");
        _;
    }

    function owner() public view returns (address) { return _owner; }
    function pendingOwner() public view returns (address) { return _pendingOwner; }

    function transferOwnership(address newOwner) public onlyOwner {
        _pendingOwner = newOwner;
    }

    function acceptOwnership() public {
        require(msg.sender == _pendingOwner, "not pending");
        address old = _owner;
        _owner = _pendingOwner;
        delete _pendingOwner;
        emit OwnershipTransferred(old, _owner);
    }

    function renounceOwnership() public onlyOwner {
        address old = _owner;
        _owner = address(0);
        emit OwnershipTransferred(old, address(0));
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

[DisplayName("OwnableContract")]
[ContractPermission("*", "*")]
public class OwnableContract : SmartContract
{
    private static readonly byte[] OwnerKey        = { 0xff, 0x01 };
    private static readonly byte[] PendingOwnerKey = { 0xff, 0x02 };

    [DisplayName("OwnershipTransferred")]
    public static event Action<UInt160, UInt160> OnOwnershipTransferred;

    public static UInt160 GetOwner()
        => (UInt160)Storage.Get(Storage.CurrentContext, OwnerKey);

    public static UInt160 PendingOwner()
        => (UInt160)Storage.Get(Storage.CurrentContext, PendingOwnerKey);

    public static void TransferOwnership(UInt160 newOwner)
    {
        if (!Runtime.CheckWitness(GetOwner())) throw new Exception("owner only");
        if (!newOwner.IsValid)                 throw new Exception("invalid newOwner");
        Storage.Put(Storage.CurrentContext, PendingOwnerKey, newOwner);
    }

    public static void AcceptOwnership()
    {
        var pending = PendingOwner();
        if (pending == null || !Runtime.CheckWitness(pending))
            throw new Exception("not pending owner");
        var prev = GetOwner();
        Storage.Put(Storage.CurrentContext, OwnerKey, pending);
        Storage.Delete(Storage.CurrentContext, PendingOwnerKey);
        OnOwnershipTransferred(prev, pending);
    }

    /// <summary>NEP-22: standard update method, replaces ERC-1967 proxy upgrade.</summary>
    public static void Update(ByteString nefFile, string manifest, object data)
    {
        if (!Runtime.CheckWitness(GetOwner())) throw new Exception("owner only");
        ContractManagement.Update(nefFile, manifest, data);
    }

    public static void _deploy(object data, bool update)
    {
        if (update) return;
        var initialOwner = (UInt160)data;
        Storage.Put(Storage.CurrentContext, OwnerKey, initialOwner);
        OnOwnershipTransferred(UInt160.Zero, initialOwner);
    }
}
```

</template>

</StandardEntry>

<!-- ============================================================ -->
<!-- ERC-1271 -->
<!-- ============================================================ -->

<StandardEntry
  id="erc-1271"
  title="ERC-1271 — Smart Contract Signatures"
  eip="1271"
  status="Final"
  neoMapping="Native witness model"
  category="Signatures"
  parityLabel="Native"
  parityClass="sm-pill-native"
>

<template #spec>

## ERC-1271: Smart Contract Signature Verification

ERC-1271 lets smart contracts "sign" messages — useful for multi-sigs, smart
accounts, DAOs that need to authorize off-chain operations like marketplace
listings or governance votes. The contract implements `isValidSignature(hash,
signature)` and returns the magic value `0x1626ba7e` on success.

### Why It Exists on Ethereum

`ecrecover` only verifies EOA signatures. Without ERC-1271, a smart contract cannot
participate in any flow that uses off-chain signatures (NFT marketplaces, DAO snapshot
votes, meta-transactions).

### Neo Equivalent

Neo doesn't differentiate "EOA signs" vs "contract signs". Every Neo address is the
script hash of a verification script — single-sig CHECKSIG, multi-sig CHECKMULTISIG,
or arbitrary contract via NEP-30 `verify`. The protocol invokes the right verifier
automatically. Off-chain signature verification uses `CryptoLib.VerifyWithECDsa`.

::: tip Live on Neo TestNet
Both implementations deployed and behavior-verified on Neo N3 TestNet.

| Implementation | TestNet Address | Contract Hash |
| --- | --- | --- |
| **Solidity** (`neo-solc`) | `Nh2dZYCdvA6KfgeJ78712Twq5tZVRANtdk` | [`0x88eec008…e92b9de7`](https://dora.coz.io/contract/neo3/testnet/0x88eec008aaeb09d10ce68f93f6d98efbe92b9de7) |
| **Neo C#** (`nccs`) | `NXq82dqPfYsB4gYxpurbn2sft8tT9v4NL2` | [`0x88079ecd…a8cfb682`](https://dora.coz.io/contract/neo3/testnet/0x88079ecdd4af98cf932c25c80c0bb218a8cfb682) |

Verified: `ownerCount`/`threshold` start at 0 pre-setup; multi-sig setup pattern. The C# version exposes `Verify()` for native NEP-30 multi-sig witnessing.
[`docs/standards-mirror/deployments/erc-1271/`](https://github.com/r3e-network/neo-solidity/tree/main/docs/standards-mirror/deployments/erc-1271).
:::

</template>

<template #solidity>

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

interface IERC1271 {
    function isValidSignature(bytes32 hash, bytes memory signature)
        external view returns (bytes4 magicValue);
}

contract MultiSigWallet is IERC1271 {
    address[] public owners;
    uint256   public threshold;
    bytes4 private constant MAGIC = 0x1626ba7e;

    function isValidSignature(bytes32 hash, bytes memory signatures)
        external view returns (bytes4)
    {
        require(signatures.length >= threshold * 65, "too few sigs");
        address[] memory seen = new address[](threshold);

        for (uint i; i < threshold; ++i) {
            (uint8 v, bytes32 r, bytes32 s) = _parse(signatures, i);
            address signer = ecrecover(hash, v, r, s);
            require(_isOwner(signer), "not owner");
            for (uint j; j < i; ++j) require(seen[j] != signer, "dup signer");
            seen[i] = signer;
        }
        return MAGIC;
    }

    function _parse(bytes memory sigs, uint i)
        internal pure returns (uint8 v, bytes32 r, bytes32 s)
    {
        assembly {
            r := mload(add(sigs, add(0x20, mul(i, 65))))
            s := mload(add(sigs, add(0x40, mul(i, 65))))
            v := byte(0, mload(add(sigs, add(0x60, mul(i, 65)))))
        }
    }

    function _isOwner(address a) internal view returns (bool) {
        for (uint i; i < owners.length; ++i)
            if (owners[i] == a) return true;
        return false;
    }
}

// Marketplace verifying a smart-contract signature:
contract Marketplace {
    bytes4 private constant ERC1271_MAGIC = 0x1626ba7e;

    function _verifyListing(address signer, bytes32 hash, bytes memory sig)
        internal view returns (bool)
    {
        if (signer.code.length == 0) {
            // EOA path — ecrecover boilerplate elided
            return false;
        }
        try IERC1271(signer).isValidSignature(hash, sig) returns (bytes4 m) {
            return m == ERC1271_MAGIC;
        } catch { return false; }
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
using Neo.SmartContract.Framework.Services;

namespace R3E.Examples;

/// <summary>
/// On-chain authorization — same primitive for any account type.
/// </summary>
public static class AuthorizeAny
{
    public static bool Authorize(UInt160 account)
        => Runtime.CheckWitness(account);   // works for EOA, multi-sig, or contract account
}

/// <summary>
/// Off-chain signature verification — what marketplaces use to verify listings
/// signed by a wallet but submitted on-chain by another party.
/// </summary>
public static class ListingVerifier
{
    public static bool VerifySingleSig(
        UInt160 expectedAccount, ByteString message,
        ByteString signature, ECPoint pubKey)
    {
        var script = Contract.CreateStandardAccount(pubKey);
        if (!script.Equals(expectedAccount)) return false;
        return CryptoLib.VerifyWithECDsa(message, pubKey, signature, NamedCurveHash.secp256r1SHA256);
    }

    public static bool VerifyMultiSig(
        UInt160 expectedAccount, ByteString message,
        ECPoint[] pubKeys, ByteString[] signatures, int threshold)
    {
        var script = Contract.CreateMultisigAccount(threshold, pubKeys);
        if (!script.Equals(expectedAccount)) return false;

        int valid = 0;
        foreach (var sig in signatures)
            foreach (var pk in pubKeys)
                if (CryptoLib.VerifyWithECDsa(message, pk, sig, NamedCurveHash.secp256r1SHA256)) {
                    valid++; break;
                }
        return valid >= threshold;
    }
}
```

### Why ERC-1271 Doesn't Exist on Neo

| Concern | ERC-1271 | Neo |
| --- | --- | --- |
| EOA vs contract dispatch | Caller checks `code.length` and branches | Single primitive: account hash + verification script |
| Magic value | Contract must return `0x1626ba7e` | Not applicable |
| Marketplace integration | Implement both branches | One verification path |
| Failure modes | Inconsistent (revert vs return wrong magic) | Verification scripts succeed or fail |

</template>

</StandardEntry>

<!-- ============================================================ -->
<!-- ERC-2612 -->
<!-- ============================================================ -->

<StandardEntry
  id="erc-2612"
  title="ERC-2612 — Permit (Gasless Approval)"
  eip="2612"
  status="Final"
  neoMapping="Native witness scopes"
  category="Signatures"
  parityLabel="Native"
  parityClass="sm-pill-native"
>

<template #spec>

## ERC-2612: Gasless Token Approval

ERC-2612 patches a structural problem with ERC-20: every approval costs a separate
transaction, which means users must spend gas before they can use a DeFi protocol.
Permit lets users sign an EIP-712 typed message off-chain; the protocol submits it
on-chain alongside the actual swap.

### Required Interface

```solidity
function permit(address owner, address spender, uint256 value,
                uint256 deadline, uint8 v, bytes32 r, bytes32 s) external;
function nonces(address owner) external view returns (uint256);
function DOMAIN_SEPARATOR() external view returns (bytes32);
```

### Known Footguns

Phishing replay (sign infinite permit), signature malleability (pre-EIP-2098),
nonce griefing (front-runner submits before you), domain separator collisions
across forks.

### Neo Equivalent: Witness Scopes

Neo solves the same problem at the protocol level. Every Neo transaction includes
`Signers` with **witness scopes** that limit which contracts may rely on the
authorisation. The dApp builds a transaction with `WitnessScope.CustomContracts`
authorising the swap router and the token contract — the user signs the whole tx,
and `Runtime.CheckWitness(from)` succeeds inside the token's `Transfer`. No
permit method to write.

</template>

<template #solidity>

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

abstract contract ERC20Permit {
    bytes32 private constant _PERMIT_TYPEHASH = keccak256(
      "Permit(address owner,address spender,uint256 value,uint256 nonce,uint256 deadline)"
    );
    mapping(address => uint256) public nonces;
    bytes32 private immutable _DOMAIN_SEPARATOR;
    bytes32 private immutable _HASHED_NAME;

    constructor(string memory tokenName) {
        _HASHED_NAME = keccak256(bytes(tokenName));
        _DOMAIN_SEPARATOR = keccak256(abi.encode(
            keccak256("EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)"),
            _HASHED_NAME, keccak256("1"), block.chainid, address(this)
        ));
    }

    function permit(
        address owner, address spender, uint256 value,
        uint256 deadline, uint8 v, bytes32 r, bytes32 s
    ) public virtual {
        require(block.timestamp <= deadline, "Permit: expired");
        bytes32 structHash = keccak256(abi.encode(
            _PERMIT_TYPEHASH, owner, spender, value, nonces[owner]++, deadline
        ));
        bytes32 hash = keccak256(abi.encodePacked("\x19\x01", _DOMAIN_SEPARATOR, structHash));
        address recovered = ecrecover(hash, v, r, s);
        require(recovered != address(0) && recovered == owner, "Permit: invalid signature");
        _approve(owner, spender, value);
    }

    function _approve(address owner, address spender, uint256 value) internal virtual;
}
```

</template>

<template #csharp>

```csharp
// Neo: nothing to write at the contract level beyond standard NEP-17 transfer.
// The dApp client builds a single transaction with the appropriate witness scope:

// const tx = new TransactionBuilder()
//   .invoke(swapRouter, "swap", [tokenIn, tokenOut, amount])
//   .signers([{
//     account: userAddress,
//     scopes: WitnessScope.CustomContracts,
//     allowedContracts: [tokenIn, swapRouter]
//   }])
//   .build();
// const signed = await wallet.signTransaction(tx);
// await rpc.sendRawTransaction(signed);

using System;
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

    public static bool Transfer(UInt160 from, UInt160 to, BigInteger amount, object data)
    {
        if (!Runtime.CheckWitness(from)) throw new Exception("no authorization");
        // ... actual transfer ...
        OnTransfer(from, to, amount);
        return true;
    }
}
```

### Comparison

| Concern | ERC-2612 | Neo Witness Scopes |
| --- | --- | --- |
| Phishing infinite approval | High risk | Scopes auto-expire with the transaction |
| Signature malleability | Implementation-dependent | Handled at protocol level |
| Cross-chain replay | Domain separator must be perfect | Chain-id is part of the tx hash |
| Nonce griefing | Real attack vector | Signatures are tx-bound — no nonces |
| Code surface | ~60 lines per token | 1 line: `CheckWitness(from)` |

</template>

</StandardEntry>

<!-- ============================================================ -->
<!-- ERC-4337 -->
<!-- ============================================================ -->

<StandardEntry
  id="erc-4337"
  title="ERC-4337 — Account Abstraction"
  eip="4337"
  status="Final"
  neoMapping="Native NEP-30 verify"
  category="Smart Accounts"
  parityLabel="Native"
  parityClass="sm-pill-native"
>

<template #spec>

## ERC-4337: Account Abstraction

ERC-4337 introduces account abstraction to Ethereum without changing the protocol.
Users submit `UserOperation` objects to a Bundler, which packages them into a single
transaction sent to a global EntryPoint contract. Smart-contract wallets implement
`validateUserOp` to authorize their own operations.

### Components

| Component | Role |
| --- | --- |
| `UserOperation` | Pseudo-transaction signed by the smart-account owner |
| Bundler | Off-chain actor; collects UserOps, sends to EntryPoint |
| EntryPoint contract | Singleton on-chain validator/dispatcher |
| Smart Account | Contract implementing `validateUserOp` |
| Paymaster | Optional contract that sponsors gas |

### Neo Equivalent: Native From Day One

Neo accounts ARE smart contracts by default. Standard wallet = single-sig
verification script. Multi-sig = N-of-M CHECKMULTISIG (no contract deploy needed).
Programmable account = NEP-30 `verify` method on a contract — the protocol invokes
it during transaction validation. **No bundler. No EntryPoint. No paymaster
ceremony. No parallel mempool.**

</template>

<template #solidity>

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

struct UserOperation {
    address sender;
    uint256 nonce;
    bytes   initCode;
    bytes   callData;
    uint256 callGasLimit;
    uint256 verificationGasLimit;
    uint256 preVerificationGas;
    uint256 maxFeePerGas;
    uint256 maxPriorityFeePerGas;
    bytes   paymasterAndData;
    bytes   signature;
}

contract SimpleAccount {
    address public immutable entryPoint;
    address public           owner;
    uint256 public           nonce;

    constructor(address ep, address owner_) {
        entryPoint = ep;
        owner      = owner_;
    }

    modifier onlyEntryPoint() {
        require(msg.sender == entryPoint, "not entrypoint");
        _;
    }

    function validateUserOp(
        UserOperation calldata userOp,
        bytes32 userOpHash,
        uint256 missingAccountFunds
    ) external onlyEntryPoint returns (uint256 validationData) {
        require(userOp.nonce == nonce, "bad nonce");
        nonce++;

        bytes32 hash = keccak256(abi.encodePacked(
            "\x19Ethereum Signed Message:\n32", userOpHash));
        require(_recover(hash, userOp.signature) == owner, "bad sig");

        if (missingAccountFunds > 0) {
            (bool ok, ) = msg.sender.call{ value: missingAccountFunds }("");
            require(ok);
        }
        return 0;
    }

    function execute(address target, uint256 value, bytes calldata data)
        external onlyEntryPoint
    {
        (bool ok, bytes memory ret) = target.call{ value: value }(data);
        require(ok, string(ret));
    }

    function _recover(bytes32, bytes memory) internal pure returns (address) {
        return address(0);
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

[DisplayName("ProgrammableAccount")]
[ContractPermission("*", "*")]
[SupportedStandards("NEP-30")]
public class ProgrammableAccount : SmartContract
{
    private static readonly byte[] PrimaryKey   = { 0x01 };
    private static readonly byte[] RecoveryKey  = { 0x02 };
    private static readonly byte[] DailyLimit   = { 0x03 };
    private static readonly byte[] SpentToday   = { 0x04 };
    private static readonly byte[] SpentEpoch   = { 0x05 };

    /// <summary>
    /// NEP-30 verify: invoked during transaction validation for any tx where
    /// this account is a Signer. Returns true if authorised.
    /// </summary>
    public static bool Verify(BigInteger maxSpend)
    {
        var primary = (UInt160)Storage.Get(Storage.CurrentContext, PrimaryKey);
        if (CheckSignerIs(primary))
        {
            EnforceDailyLimit(maxSpend);
            return true;
        }

        // 2-of-N recovery path
        var recoverySet = (UInt160[])StdLib.Deserialize(
            Storage.Get(Storage.CurrentContext, RecoveryKey));
        int approvals = 0;
        foreach (var r in recoverySet)
            if (CheckSignerIs(r)) approvals++;
        return approvals >= 2;
    }

    private static bool CheckSignerIs(UInt160 expected)
    {
        foreach (var s in Runtime.CurrentSigners())
            if (s.Account.Equals(expected)) return true;
        return false;
    }

    private static void EnforceDailyLimit(BigInteger spend)
    {
        var limit = (BigInteger)(Storage.Get(Storage.CurrentContext, DailyLimit) ?? ByteString.Empty);
        if (limit == 0) return;

        var nowEpoch  = Runtime.Time / 86_400_000;
        var lastEpoch = (BigInteger)(Storage.Get(Storage.CurrentContext, SpentEpoch) ?? ByteString.Empty);
        var spent     = nowEpoch == lastEpoch
            ? (BigInteger)(Storage.Get(Storage.CurrentContext, SpentToday) ?? ByteString.Empty)
            : 0;
        if (spent + spend > limit) throw new Exception("daily spend limit exceeded");

        Storage.Put(Storage.CurrentContext, SpentEpoch, nowEpoch);
        Storage.Put(Storage.CurrentContext, SpentToday, spent + spend);
    }

    public static void RotatePrimary(UInt160 newOwner)
    {
        var recoverySet = (UInt160[])StdLib.Deserialize(
            Storage.Get(Storage.CurrentContext, RecoveryKey));
        int approvals = 0;
        foreach (var r in recoverySet)
            if (Runtime.CheckWitness(r)) approvals++;
        if (approvals < 2) throw new Exception("need 2-of-N recovery approval");
        Storage.Put(Storage.CurrentContext, PrimaryKey, newOwner);
    }

    public static void _deploy(object data, bool update)
    {
        if (update) return;
        var args = (object[])data;
        Storage.Put(Storage.CurrentContext, PrimaryKey, (UInt160)args[0]);
        Storage.Put(Storage.CurrentContext, RecoveryKey, StdLib.Serialize((UInt160[])args[1]));
        Storage.Put(Storage.CurrentContext, DailyLimit, (BigInteger)args[2]);
    }
}
```

### Capability Comparison

| Feature | ERC-4337 | Neo NEP-30 |
| --- | --- | --- |
| Multi-sig | Custom contract | Built-in CHECKMULTISIG (zero contract) |
| Spending limits | Custom contract | NEP-30 `verify` reads storage |
| Social recovery | Custom contract | `verify` checks recovery witnesses |
| Session keys | Custom contract | `verify` checks session-key sig + expiry |
| Gas sponsorship | Paymaster contract | Tx `Sender` ≠ `Signers` — any signer pays |
| Off-chain infrastructure | Bundler + mempool | None — standard mempool |
| Per-tx overhead | EntryPoint dispatch + delegatecall | Direct call into `verify` |

</template>

</StandardEntry>

<!-- ============================================================ -->
<!-- EIP-712 -->
<!-- ============================================================ -->

<StandardEntry
  id="eip-712"
  title="EIP-712 — Typed Structured Data Signing"
  eip="712"
  status="Final"
  neoMapping="Native witness model"
  category="Signatures"
  parityLabel="Native"
  parityClass="sm-pill-native"
>

<template #spec>

## EIP-712: Typed Structured Data Hashing and Signing

Before EIP-712, off-chain signing showed users a hex blob that even technical users
couldn't verify. EIP-712 introduced **typed structured data**: signing requests
carry a typed schema that wallets render as human-readable fields ("Approve 100 USDC
for Uniswap until 2026-01-01").

### Mechanics

1. Define a Solidity-like type schema: `struct Permit { address owner; ...; uint256 deadline; }`.
2. Compute `domainSeparator` from `(name, version, chainId, verifyingContract)`.
3. Compute `structHash = keccak256(typeHash || encoded fields)`.
4. The digest is `keccak256(0x1901 || domainSeparator || structHash)`.
5. Wallet signs the digest; `ecrecover` verifies on-chain.

### Why Neo Doesn't Need It

Neo wallets sign **the actual transaction** by default, and Neo transactions are
already structured: Signers, Witnesses, Script. Wallets render the script's invocation
parameters (which contract, which method, which args) — humans see exactly what
they're authorising.

For off-chain signed messages (e.g. marketplace listings), Neo uses a similar
"sign hashed message" primitive but without the EIP-712 schema layer because the
message format is application-specific and already documented in the dApp.

</template>

<template #solidity>

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

contract EIP712Verifier {
    bytes32 public DOMAIN_SEPARATOR;

    struct Order {
        address maker;
        address asset;
        uint256 amount;
        uint256 nonce;
        uint256 deadline;
    }

    bytes32 private constant ORDER_TYPEHASH = keccak256(
        "Order(address maker,address asset,uint256 amount,uint256 nonce,uint256 deadline)"
    );

    constructor() {
        DOMAIN_SEPARATOR = keccak256(abi.encode(
            keccak256("EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)"),
            keccak256("MyDApp"),
            keccak256("1"),
            block.chainid,
            address(this)
        ));
    }

    function hashOrder(Order memory o) public pure returns (bytes32) {
        return keccak256(abi.encode(
            ORDER_TYPEHASH, o.maker, o.asset, o.amount, o.nonce, o.deadline
        ));
    }

    function digest(Order memory o) public view returns (bytes32) {
        return keccak256(abi.encodePacked("\x19\x01", DOMAIN_SEPARATOR, hashOrder(o)));
    }

    function executeSigned(Order memory o, uint8 v, bytes32 r, bytes32 s) external {
        require(block.timestamp <= o.deadline, "expired");
        address signer = ecrecover(digest(o), v, r, s);
        require(signer == o.maker, "bad signer");
        // ... execute order ...
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
/// Neo equivalent — verify an off-chain signed order. The "digest" is just
/// the serialised order; Neo wallets sign the bytes directly. No schema
/// hashing dance because there's no need to disambiguate domains:
/// the contract hash itself is the domain separator.
/// </summary>
[DisplayName("OrderBook")]
[ContractPermission("*", "*")]
public class OrderBook : SmartContract
{
    public struct Order
    {
        public UInt160    Maker;
        public UInt160    Asset;
        public BigInteger Amount;
        public BigInteger Nonce;
        public BigInteger Deadline;
    }

    public static ByteString HashOrder(Order o)
    {
        // Domain separation: contract hash is part of the message.
        var encoded = StdLib.Serialize(new object[] {
            Runtime.ExecutingScriptHash,
            (BigInteger)Runtime.GetNetwork(),
            o.Maker, o.Asset, o.Amount, o.Nonce, o.Deadline
        });
        return CryptoLib.Sha256(encoded);
    }

    public static void ExecuteSigned(Order o, ECPoint pubKey, ByteString signature)
    {
        if (Runtime.Time / 1000 > o.Deadline) throw new Exception("expired");

        // Verify signature
        var digest = HashOrder(o);
        if (!CryptoLib.VerifyWithECDsa(digest, pubKey, signature, NamedCurveHash.secp256r1SHA256))
            throw new Exception("bad signature");

        // Confirm pubKey corresponds to maker
        var script = Contract.CreateStandardAccount(pubKey);
        if (!script.Equals(o.Maker)) throw new Exception("pubKey doesn't match maker");

        // Replay protection — track seen nonces
        var nonceKey = new byte[] { 0x01 }.Concat(o.Maker)
                                          .Concat((ByteString)o.Nonce.ToByteArray());
        if (Storage.Get(Storage.CurrentContext, nonceKey) != null)
            throw new Exception("nonce already used");
        Storage.Put(Storage.CurrentContext, nonceKey, 1);

        // ... execute order ...
    }
}
```

### What's Different

- **Domain separator** is implicit: the contract's script hash + Neo network ID
  uniquely identify the verifying contract; no separate constant.
- **No type hash**: Neo serialises with `StdLib.Serialize`, which is canonical and
  collision-free for the structures it accepts.
- **Same replay protection** via per-maker nonces (when needed). For one-shot
  authorisations, transaction-level uniqueness is sufficient.

</template>

</StandardEntry>

<!-- ============================================================ -->
<!-- EIP-191 -->
<!-- ============================================================ -->

<StandardEntry
  id="eip-191"
  title="EIP-191 — Signed Data Standard"
  eip="191"
  status="Final"
  neoMapping="Native witness model"
  category="Signatures"
  parityLabel="Native"
  parityClass="sm-pill-native"
>

<template #spec>

## EIP-191: Signed Data Standard

EIP-191 introduces the `\x19` prefix that disambiguates signed messages from
transactions: a wallet signing `personal_sign` of an arbitrary string actually signs
`keccak256("\x19Ethereum Signed Message:\n" + len + message)`. This prevents a
malicious dApp from tricking a user into signing a transaction by presenting it as
a "message".

### Versions

- `0x00` — data with intended validator (a specific contract address).
- `0x01` — structured data (extended by EIP-712).
- `0x45` (`E`) — `personal_sign` (the canonical "Sign in with Ethereum" prefix).

### Neo Equivalent

Neo wallets distinguish between signing transactions and signing arbitrary messages
at the wallet level — there's no need for an in-band byte prefix because the
signing API exposes them as separate calls (`signTransaction` vs `signMessage`).
The Neo C# tab shows on-chain verification of a `signMessage`-produced signature.

</template>

<template #solidity>

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

library SignatureChecker {
    /// EIP-191 v0x45 — personal_sign style.
    function recover(bytes32 messageHash, bytes memory sig)
        internal pure returns (address)
    {
        bytes32 ethHash = keccak256(abi.encodePacked(
            "\x19Ethereum Signed Message:\n32", messageHash
        ));
        if (sig.length != 65) return address(0);
        bytes32 r; bytes32 s; uint8 v;
        assembly {
            r := mload(add(sig, 32))
            s := mload(add(sig, 64))
            v := byte(0, mload(add(sig, 96)))
        }
        return ecrecover(ethHash, v, r, s);
    }
}

contract LoginVerifier {
    using SignatureChecker for bytes32;

    function login(string calldata message, bytes calldata signature)
        external view returns (address)
    {
        bytes32 hash = keccak256(bytes(message));
        return hash.recover(signature);
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

[DisplayName("LoginVerifier")]
[ContractPermission("*", "*")]
public class LoginVerifier : SmartContract
{
    /// <summary>
    /// Verify a wallet-signed message. The wallet's signMessage API hashes the
    /// message internally before signing, exactly like personal_sign on Ethereum,
    /// but without the in-band prefix because the Wallet API distinguishes
    /// "sign transaction" and "sign message" as separate methods.
    /// </summary>
    public static UInt160 Login(string message, ECPoint pubKey, ByteString signature)
    {
        var messageBytes = (ByteString)System.Text.Encoding.UTF8.GetBytes(message);
        var digest = CryptoLib.Sha256(messageBytes);

        if (!CryptoLib.VerifyWithECDsa(digest, pubKey, signature, NamedCurveHash.secp256r1SHA256))
            throw new Exception("invalid signature");

        // Derive the Neo address (script hash) from the public key.
        return Contract.CreateStandardAccount(pubKey);
    }
}
```

### Why the Prefix Doesn't Apply

On Ethereum, signing requests reach the wallet as a single `eth_sign` /
`personal_sign` RPC call with no type information; the prefix prevents an attacker
from constructing a message whose bytes match a valid transaction format.

On Neo, the wallet API (NeoLine, O3, OneGate, NeonWallet) has typed methods —
`signTransaction(tx)` and `signMessage(message)` are different RPC calls that wallets
display differently. The prefix becomes redundant.

</template>

</StandardEntry>

<!-- ============================================================ -->
<!-- ERC-5267 -->
<!-- ============================================================ -->

<StandardEntry
  id="erc-5267"
  title="ERC-5267 — EIP-712 Domain Retrieval"
  eip="5267"
  status="Final"
  neoMapping="Native"
  category="Signatures"
  parityLabel="Native"
  parityClass="sm-pill-native"
>

<template #spec>

## ERC-5267: EIP-712 Domain Retrieval

A meta-standard: contracts that use EIP-712 should expose their domain via a single
view method so that wallets and indexers can construct typed signing requests
without hard-coding the domain.

### Required Method

```solidity
function eip712Domain() external view returns (
    bytes1 fields,
    string memory name,
    string memory version,
    uint256 chainId,
    address verifyingContract,
    bytes32 salt,
    uint256[] memory extensions
);
```

### Neo Equivalent

A Neo contract's domain is fully derivable from its manifest and script hash —
wallets and clients fetch it via `ContractManagement.GetContract(...)` and the
network ID via `Runtime.GetNetwork()`. The C# port just exposes a convenience
method that returns the same metadata, but it's optional.

::: tip Live on Neo TestNet
Both implementations are deployed and behavior-verified on Neo N3 TestNet (network magic `894710606`).

| Implementation | Contract Hash | Deploy Tx |
| --- | --- | --- |
| **Solidity** (`neo-solc`) | `0x1dd8a39225d515a4621c5214f336c78f4b19bb6c` | [`0x5994e152…d7add3a8`](https://dora.coz.io/transaction/neo3/testnet/0x5994e152d86ca8753118ba7f27159fa7cd5bc66079d08a28f796baabd7add3a8) |
| **Neo C#** (`nccs`) | `0xdcfa06612bfa8614e4d197bc8206b68320cd9877` | [`0x028edfdf…3d17508`](https://dora.coz.io/transaction/neo3/testnet/0x028edfdf025d5388fec1255f3e6e9aa34f6bb2f6b3e495fc68d8932663d17508) |

Cross-implementation `getName` and `getVersion` calls return identical
`MyDApp` / `1`. Source pairs under
[`docs/standards-mirror/deployments/erc-5267/`](https://github.com/r3e-network/neo-solidity/tree/main/docs/standards-mirror/deployments/erc-5267).
:::

</template>

<template #solidity>

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

contract DomainExposer {
    string  public constant NAME    = "MyDApp";
    string  public constant VERSION = "2";

    event EIP712DomainChanged();

    function eip712Domain() external view returns (
        bytes1   fields,
        string memory name,
        string memory version,
        uint256  chainId,
        address  verifyingContract,
        bytes32  salt,
        uint256[] memory extensions
    ) {
        fields            = bytes1(0x0f);   // name | version | chainId | verifyingContract
        name              = NAME;
        version           = VERSION;
        chainId           = block.chainid;
        verifyingContract = address(this);
        salt              = bytes32(0);
        extensions        = new uint256[](0);
    }
}
```

</template>

<template #csharp>

```csharp
using System.ComponentModel;
using System.Numerics;
using Neo;
using Neo.SmartContract.Framework;
using Neo.SmartContract.Framework.Attributes;
using Neo.SmartContract.Framework.Native;
using Neo.SmartContract.Framework.Services;

namespace R3E.Examples;

[DisplayName("DomainExposer")]
[ContractPermission("*", "*")]
public class DomainExposer : SmartContract
{
    /// <summary>
    /// Convenience method for clients that prefer an explicit domain query over
    /// reading the manifest + network ID. The manifest already contains
    /// name and contract hash; this just bundles them.
    /// </summary>
    public static (string, BigInteger, UInt160) Domain()
    {
        var contract = ContractManagement.GetContract(Runtime.ExecutingScriptHash);
        return (contract.Manifest.Name, Runtime.GetNetwork(), Runtime.ExecutingScriptHash);
    }
}
```

### Why Neo Has Less Ceremony

Wallet clients on Neo construct domain-bound signing payloads from the contract
manifest and the network ID — both authoritative and immutable per-contract. There's
no per-deploy "name + version" string to keep in sync because the manifest already
carries `Name`, the script hash already identifies the deploy, and the network ID
distinguishes mainnet/testnet.

</template>

</StandardEntry>

<!-- ============================================================ -->
<!-- ERC-6492 -->
<!-- ============================================================ -->

<StandardEntry
  id="erc-6492"
  title="ERC-6492 — Signatures for Pre-deployed Contracts"
  eip="6492"
  status="Final"
  neoMapping="Native (no counterfactual deploy)"
  category="Signatures"
  parityLabel="Native"
  parityClass="sm-pill-native"
>

<template #spec>

## ERC-6492: Signatures for Pre-deployed Smart Accounts

ERC-6492 solves a niche but painful problem with smart accounts: a user can have a
"counterfactual" smart account address (deterministic from CREATE2) that they
haven't actually deployed yet. They want to sign a marketplace listing or DAO vote
**before** spending gas to deploy. ERC-6492 wraps signatures with `(factory,
factoryCalldata, signature)` so a verifier can check the signature using the
yet-to-be-deployed account's eventual code.

### Mechanics

A 6492-wrapped signature appended with `0x6492649264926492649264926492649264926492649264926492649264926492` magic suffix tells the verifier:
1. If the contract isn't deployed yet, deploy it (via `factory.factoryCalldata`).
2. Then validate via ERC-1271's `isValidSignature`.
3. If still invalid, treat as standard ECDSA signature.

### Neo Equivalent

Neo doesn't have counterfactual deploys: a contract address is the script hash, and
you must deploy bytecode before you can be signed-for. But a Neo user can sign a
listing *as their EOA* (which always exists at its public-key-derived hash), then
upgrade to a smart-contract account later via NEP-30 `verify`. The C# tab shows
how a verifier handles both cases without the wrapping ceremony.

</template>

<template #solidity>

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

interface IERC1271 {
    function isValidSignature(bytes32, bytes memory) external view returns (bytes4);
}

library SignatureValidator {
    bytes32 constant ERC6492_MAGIC =
        0x6492649264926492649264926492649264926492649264926492649264926492;

    function isValidSig(address signer, bytes32 hash, bytes memory sig)
        internal view returns (bool)
    {
        // Strip & detect ERC-6492 wrapping
        bool is6492 = sig.length >= 32
            && _readBytes32(sig, sig.length - 32) == ERC6492_MAGIC;

        if (is6492) {
            // Decode (factory, factoryCalldata, innerSig) from sig[:-32]
            (address factory, bytes memory call, bytes memory inner) =
                abi.decode(_strip(sig, 32), (address, bytes, bytes));

            if (signer.code.length == 0) {
                (bool ok, ) = factory.call(call);
                require(ok, "deploy failed");
            }
            sig = inner;
        }

        if (signer.code.length > 0) {
            try IERC1271(signer).isValidSignature(hash, sig) returns (bytes4 m) {
                return m == 0x1626ba7e;
            } catch { return false; }
        }
        // Fall through to ecrecover...
        return false;
    }

    function _strip(bytes memory b, uint n) internal pure returns (bytes memory) {
        bytes memory out = new bytes(b.length - n);
        for (uint i; i < b.length - n; ++i) out[i] = b[i];
        return out;
    }

    function _readBytes32(bytes memory b, uint offset) internal pure returns (bytes32 r) {
        assembly { r := mload(add(b, add(32, offset))) }
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

[DisplayName("SigVerifier")]
[ContractPermission("*", "*")]
public class SigVerifier : SmartContract
{
    /// <summary>
    /// Verify a signature for an account that may be either:
    ///   (a) a single-sig EOA-style address (script hash of CHECKSIG script), or
    ///   (b) a deployed contract account (NEP-30 verify or arbitrary script).
    ///
    /// On Neo, the signer's address is the script hash of its verification
    /// script. For (a), the script is canonical and we can verify via
    /// CryptoLib.VerifyWithECDsa. For (b), we delegate to the deployed
    /// contract's verify method.
    /// </summary>
    public static bool IsValid(UInt160 signer, ByteString digest,
                               ECPoint pubKey, ByteString signature)
    {
        // Case (a): EOA-style — pubKey-derived script matches signer hash.
        var derived = (UInt160)CryptoLib.Ripemd160(CryptoLib.Sha256(
            Helper.CreateSignatureCheckScript(pubKey)));
        if (derived.Equals(signer))
            return CryptoLib.VerifyWithECDsa(digest, pubKey, signature, NamedCurveHash.secp256r1SHA256);

        // Case (b): deployed contract — call its verify method.
        var contract = ContractManagement.GetContract(signer);
        if (contract != null)
        {
            try
            {
                var ok = (bool)Contract.Call(signer, "verify", CallFlags.ReadOnly,
                                             new object[] { digest, signature });
                return ok;
            }
            catch { return false; }
        }
        return false;
    }
}
```

### Why "Counterfactual" Isn't an Issue on Neo

On Neo, an account address is one of three things:
- A single-sig EOA — exists by virtue of its public key, no deploy needed.
- A multi-sig — also derivable from `(threshold, pubkeys)`, no deploy needed.
- A deployed smart contract — has a script hash from its NEF + manifest.

Cases 1 and 2 are "always deployed" implicitly — the signature can be verified
without any prior on-chain action. Case 3 requires the contract to exist for the
NEP-30 path. Users who want a smart-contract account migrate from case 1 to case 3
when they're ready, with no signature-replay concern because the address itself
changes.

</template>

</StandardEntry>

<!-- ============================================================ -->
<!-- EIP-7702 -->
<!-- ============================================================ -->

<StandardEntry
  id="eip-7702"
  title="EIP-7702 — Set Code for EOAs"
  eip="7702"
  status="Final (Pectra)"
  neoMapping="Native (every account is a contract)"
  category="Smart Accounts"
  parityLabel="Native"
  parityClass="sm-pill-native"
>

<template #spec>

## EIP-7702: Set Code for Externally-Owned Accounts

EIP-7702 (Pectra hard fork, 2025) lets an EOA temporarily delegate its execution to
contract code via a signed authorisation list in a transaction. For the duration of
the transaction, calls to the EOA's address run the delegated contract's bytecode
— the EOA effectively becomes a smart account, on demand.

This is Ethereum's third attempt at account abstraction, after EIP-2938 (failed)
and ERC-4337 (parallel mempool). 7702 finally reconciles the two by amending the
EVM itself.

### Authorisation Tuple

```
(chainId, address, nonce, y_parity, r, s)
```

Signed off-chain by the EOA, included in a type-4 transaction's `authorisationList`.

### Neo Equivalent

EIP-7702 is Ethereum catching up to what Neo always had: every Neo account is a
contract from the protocol's perspective. To make an account "smart", you deploy a
contract whose script hash matches your wallet address. There's no mode-switching
because there's no mode to switch from — verification scripts run on every
transaction signing the account.

The Neo C# tab shows what an EIP-7702-style "delegated execution" looks like in
Neo terms: a programmable account contract.

</template>

<template #solidity>

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

// EIP-7702 type-4 transaction format (simplified):
//   txType            = 0x04
//   chainId           = uint256
//   nonce             = uint64
//   maxPriorityFee    = uint256
//   maxFeePerGas      = uint256
//   gasLimit          = uint64
//   to                = address
//   value             = uint256
//   data              = bytes
//   accessList        = ...
//   authorizationList = (chainId, address, nonce, y_parity, r, s)[]
//
// For each authorization where signer == EOA: that EOA's code is set to
// 0xef0100 || address (a "delegation indicator") for the duration of the tx.
//
// Application-side: the delegated code is a normal Solidity contract that
// implements whatever the EOA wants — batched calls, multi-sig, session keys.

contract DelegatedAccount {
    event Executed(address indexed target, uint256 value, bytes data);

    function execute(address target, uint256 value, bytes calldata data) external {
        // EOA delegated to this code — `address(this)` is the EOA's address.
        // Authorization: caller is the EOA itself (set-code semantics).
        require(msg.sender == address(this), "not self");
        (bool ok, ) = target.call{ value: value }(data);
        require(ok, "exec failed");
        emit Executed(target, value, data);
    }

    function executeBatch(address[] calldata targets, uint256[] calldata values, bytes[] calldata datas)
        external
    {
        require(msg.sender == address(this), "not self");
        for (uint i; i < targets.length; ++i) {
            (bool ok, ) = targets[i].call{ value: values[i] }(datas[i]);
            require(ok, "exec failed");
        }
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

[DisplayName("DelegatedAccount")]
[ContractPermission("*", "*")]
[SupportedStandards("NEP-30")]
public class DelegatedAccount : SmartContract
{
    private static readonly byte[] OwnerKey = { 0x01 };

    /// <summary>NEP-30 verify — invoked when this account is a tx signer.</summary>
    public static bool Verify()
    {
        var owner = (UInt160)Storage.Get(Storage.CurrentContext, OwnerKey);
        return Runtime.CheckWitness(owner);
    }

    /// <summary>Execute a single delegated call with this account's authority.</summary>
    public static object Execute(UInt160 target, string method, CallFlags flags, object[] args)
    {
        var owner = (UInt160)Storage.Get(Storage.CurrentContext, OwnerKey);
        if (!Runtime.CheckWitness(owner)) throw new Exception("owner only");
        return Contract.Call(target, method, flags, args);
    }

    /// <summary>Batch execution — one transaction, many calls, atomic.</summary>
    public static object[] ExecuteBatch(
        UInt160[] targets, string[] methods, CallFlags[] flags, object[][] args)
    {
        var owner = (UInt160)Storage.Get(Storage.CurrentContext, OwnerKey);
        if (!Runtime.CheckWitness(owner)) throw new Exception("owner only");
        if (targets.Length != methods.Length || methods.Length != flags.Length
            || flags.Length != args.Length)
            throw new Exception("length mismatch");

        var results = new object[targets.Length];
        for (int i = 0; i < targets.Length; i++)
            results[i] = Contract.Call(targets[i], methods[i], flags[i], args[i]);
        return results;
    }

    public static void _deploy(object data, bool update)
    {
        if (update) return;
        Storage.Put(Storage.CurrentContext, OwnerKey, (UInt160)data);
    }
}
```

### Side-by-Side: How They Differ

| Aspect | EIP-7702 | Neo |
| --- | --- | --- |
| Delegation duration | One transaction | Permanent (rotates via deploy/update) |
| Authorisation cost | Sign auth tuple + tx | Standard transaction signing |
| Code storage | EOA gets a 23-byte delegation indicator | Account is the contract |
| Mode-switch hazard | Mid-tx code can be swapped | None — code is deployed |
| Auditor mental model | EOA *and* contract simultaneously | One model: account = script hash |

EIP-7702 is a backward-compatibility patch on top of an EOA-first model. Neo never
had EOAs in the EVM sense, so it doesn't need a patch.

</template>

</StandardEntry>

<!-- ============================================================ -->
<!-- EIP-3074 -->
<!-- ============================================================ -->

<StandardEntry
  id="eip-3074"
  title="EIP-3074 — AUTH and AUTHCALL"
  eip="3074"
  status="Stagnant (superseded by 7702)"
  neoMapping="Native (witness scopes)"
  category="Smart Accounts"
  parityLabel="Native"
  parityClass="sm-pill-native"
>

<template #spec>

## EIP-3074: AUTH and AUTHCALL Opcodes

EIP-3074 proposed two new EVM opcodes — `AUTH` (verify a signature setting an
"authorized" register) and `AUTHCALL` (perform a call as the authorised account).
Together they let an "invoker" contract perform actions on behalf of an EOA after
verifying an off-chain signature.

EIP-3074 was eventually superseded by EIP-7702 (which has cleaner semantics) but
the use cases are identical: meta-transactions, batch operations, gasless UX.

### Neo Equivalent

The use case is exactly what witness scopes do already. A user signs a transaction
with `WitnessScope.CustomContracts` listing the invoker contract; the invoker calls
`Contract.Call(token, "transfer", ...)` and `Runtime.CheckWitness(user)` succeeds
inside the token because the user's witness is in scope.

</template>

<template #solidity>

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

// EIP-3074 (never deployed) sketch:
//
// AUTH opcode:
//   stack: [commit, y_parity, r, s] -> [signer]
//   if ecrecover(keccak("\x03" || chainid || invoker || commit), v, r, s) is valid:
//     set authorized = signer
//
// AUTHCALL opcode: like CALL but msg.sender becomes `authorized`.

contract Invoker {
    /// Off-chain user signs (chainid, invoker, commit) where `commit` is a hash
    /// of the action they're authorising. The invoker submits this on-chain.
    function relay(
        bytes32 commit,
        uint8 v, bytes32 r, bytes32 s,
        address target, uint256 value, bytes calldata data
    ) external {
        // assembly { auth commit v r s }    // sets authorized register
        // assembly { authcall target value data }   // call as authorized

        // Pre-7702 implementations had to roll their own — typically with an
        // "AccountFactory" that deploys a per-user proxy.
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
/// Neo equivalent of EIP-3074: a "relayer" contract that performs operations
/// authorised by the user's transaction-level signature. No new opcodes needed —
/// witness scopes do the job.
/// </summary>
[DisplayName("Relayer")]
[ContractPermission("*", "*")]
public class Relayer : SmartContract
{
    public static object Execute(UInt160 user, UInt160 target,
                                 string method, object[] args)
    {
        // Witness check: the user must have signed this transaction with a
        // scope that includes the target contract.
        if (!Runtime.CheckWitness(user)) throw new Exception("user must sign");

        // Now call target on the user's behalf.
        // CheckWitness(user) inside `target` will succeed because the user's
        // witness is in scope.
        return Contract.Call(target, method, CallFlags.All, args);
    }

    public static object[] ExecuteBatch(
        UInt160 user, UInt160[] targets, string[] methods, object[][] args)
    {
        if (!Runtime.CheckWitness(user)) throw new Exception("user must sign");
        if (targets.Length != methods.Length || methods.Length != args.Length)
            throw new Exception("length mismatch");

        var results = new object[targets.Length];
        for (int i = 0; i < targets.Length; i++)
            results[i] = Contract.Call(targets[i], methods[i], CallFlags.All, args[i]);
        return results;
    }
}
```

### What Witness Scopes Buy You

When the user builds the transaction:

```ts
new TransactionBuilder()
  .invoke(relayer, "executeBatch", [user, targets, methods, args])
  .signers([{
    account: user,
    scopes: WitnessScope.CustomContracts,
    allowedContracts: [relayer, ...targets]
  }])
  .build()
```

The user is explicitly authorising `relayer` to act on their behalf, but only
within calls into `relayer` and the listed targets. There's no concept of a
persistent "authorized register" that could leak across calls — scopes are
transaction-bound.

</template>

</StandardEntry>

</StandardsMirror>
