---
title: Protocol-Level EIPs — ERC ↔ Neo Mirror
description: Ethereum protocol EIPs (fee market, opcodes, transaction format) and how Neo handles the equivalent concern natively.
outline: false
---

# Protocol-Level EIPs

These EIPs change Ethereum's protocol itself — fee market, opcodes, transaction
formats — rather than introducing application-layer standards. Most are no-ops on
Neo because Neo's protocol already addresses the underlying concern. The third
tab here typically shows the **Neo equivalent mechanism** rather than user-level
contract code.

<StandardsMirror>

<!-- ============================================================ -->
<!-- EIP-1559 -->
<!-- ============================================================ -->

<StandardEntry
  id="eip-1559"
  title="EIP-1559 — Fee Market Reform"
  eip="1559"
  status="Final"
  neoMapping="Native polynomial GAS pricing"
  category="Fees"
  parityLabel="Native"
  parityClass="sm-pill-native"
>

<template #spec>

## EIP-1559: Base Fee + Tip Fee Market

EIP-1559 (London, 2021) replaced Ethereum's first-price gas auction with a
**base-fee + tip** model. The base fee is algorithmically adjusted per block to
target 50% block fullness, and is **burned** rather than paid to miners. Users add
an optional priority fee (tip) to incentivise inclusion.

### Goals

1. Eliminate the wild gas-price spikes of the first-price auction.
2. Make wallet UX better: users can specify a max fee they're willing to pay.
3. Burn fees as a deflationary force on ETH supply.

### Tradeoffs

- Adds protocol complexity: every block has a base fee state.
- Burning fees introduces miner-extractable-value (MEV) game theory because tips
  go to miners but base fee doesn't.
- Wallet UX still requires users to estimate `maxFeePerGas` and `maxPriorityFeePerGas`.

### Neo Equivalent

Neo has had a deterministic fee model since launch. Every transaction pays:

- **System fee**: pays for the VM operations the transaction performs (read from a
  static gas table per opcode).
- **Network fee**: pays for the size of the transaction (deterministic per byte).

There's no auction. No tip. No base-fee algorithm. The cost of a transaction is
calculable before submission with full precision. Neo doesn't burn fees — they
fund the protocol's economic model (governance via NEO holders + GAS distribution).

</template>

<template #solidity>

```solidity
// EIP-1559 transaction (type 0x02) format:
//
//   txType                = 0x02
//   chainId               = uint256
//   nonce                 = uint64
//   maxPriorityFeePerGas  = uint256        // tip to miner
//   maxFeePerGas          = uint256        // ceiling
//   gasLimit              = uint64
//   to                    = address
//   value                 = uint256
//   data                  = bytes
//   accessList            = ...
//   v, r, s
//
// At inclusion time:
//   actualFee = min(maxFeePerGas, baseFee + maxPriorityFeePerGas)
//   miner gets: actualFee - baseFee
//   protocol burns: baseFee * gasUsed
//
// The base fee adjusts each block:
//   baseFee[n+1] = baseFee[n] * (1 + 1/8 * (gasUsed - target) / target)

// Solidity contracts can introspect via:
contract FeeAware {
    function showFees() external view returns (uint256 baseFee, uint256 priority) {
        baseFee  = block.basefee;          // EIP-3198
        priority = tx.gasprice - block.basefee;
    }
}
```

</template>

<template #csharp>

```csharp
// Neo's fee model in C# terms:
//
//   System fee  = sum over all VM ops in the script of `gasTable[op]`
//   Network fee = txSize * networkFeePerByte + signatureVerificationCost
//
// There's no "base fee" because there's no algorithmic adjustment. The
// PolicyContract on Neo maintains:
//   - FeePerByte (storage cost component)
//   - ExecFeeFactor (multiplier for VM ops)
//
// These can be adjusted by a CN majority vote, but they're stable in practice.
//
// Transaction submission:
//   const tx = new TransactionBuilder()
//     .invoke(target, "method", args)
//     .signers([{ account, scopes: WitnessScope.CalledByEntry }])
//     .build();
//
//   const systemFee  = await rpc.invokeScript(tx.script).gasconsumed;   // exact
//   const networkFee = tx.size * feePerByte + verificationCost;          // exact
//   tx.systemFee  = systemFee;
//   tx.networkFee = networkFee;
//
// The user signs once with the exact fee. There's no ceiling/floor estimation.

using Neo;
using Neo.SmartContract.Framework;
using Neo.SmartContract.Framework.Attributes;
using Neo.SmartContract.Framework.Native;

namespace R3E.Examples;

[DisplayName("FeeIntrospection")]
public class FeeIntrospection : SmartContract
{
    public static long FeePerByte() => Policy.GetFeePerByte();
    public static uint ExecFeeFactor() => Policy.GetExecFeeFactor();
}
```

### Why Neo Doesn't Need a "Fee Reform"

The Ethereum fee market is auction-based because block space is scarce and demand
is volatile. Neo's TPS (~4000 ops/sec) and block time (~15 sec) leave the chain
typically uncongested, so first-price auctions don't gridlock — and the
deterministic fee model means users always know the cost up front.

</template>

</StandardEntry>

<!-- ============================================================ -->
<!-- EIP-2718 -->
<!-- ============================================================ -->

<StandardEntry
  id="eip-2718"
  title="EIP-2718 — Typed Transaction Envelope"
  eip="2718"
  status="Final"
  neoMapping="Native single transaction type"
  category="Transactions"
  parityLabel="Native"
  parityClass="sm-pill-native"
>

<template #spec>

## EIP-2718: Typed Transaction Envelope

Originally, Ethereum had one transaction format. EIP-2930 (access lists) and
EIP-1559 (base fee) needed new fields, but breaking the existing format would have
required a hard fork. EIP-2718 introduces a **type byte** as the first byte of
serialised transactions:

| Type | Defined by |
| --- | --- |
| `0xc0`-`0xfe` | Legacy (RLP-encoded transaction directly) |
| `0x00` | Reserved |
| `0x01` | EIP-2930 (access list) |
| `0x02` | EIP-1559 (fee market) |
| `0x03` | EIP-4844 (blob) |
| `0x04` | EIP-7702 (set-code) |

### Neo Equivalent

Neo has a single transaction type by design. Adding new capabilities (access lists
via witness scopes, fee handling, etc.) is done by **extending the existing
transaction structure**. Backward compatibility is handled at the network protocol
level rather than by introducing parallel formats. The serialisation is binary,
versioned by the network message version.

</template>

<template #solidity>

```solidity
// EIP-2718 type-prefixed serialisation example:
//
//   serialized_tx = type_byte || rlp([type-specific fields])
//
// Wallets must know how to construct each type. EIP-1559 (type 0x02):
//
//   0x02 || rlp([
//     chainId, nonce, maxPriorityFeePerGas, maxFeePerGas,
//     gasLimit, to, value, data, accessList, v, r, s
//   ])
//
// Type 0x03 (blob, EIP-4844):
//
//   0x03 || rlp([
//     chainId, nonce, maxPriorityFeePerGas, maxFeePerGas,
//     gasLimit, to, value, data, accessList,
//     maxFeePerBlobGas, blobVersionedHashes,
//     v, r, s
//   ])
//
// Each new EIP adds another type. Wallets, indexers, RPC clients all need to
// support every type they want to handle.
```

</template>

<template #csharp>

```csharp
// Neo's transaction shape (one type):
//
//   class Transaction {
//     byte Version;           // network version
//     uint Nonce;
//     long SystemFee;
//     long NetworkFee;
//     uint ValidUntilBlock;
//     Signer[] Signers;       // up to 16 signers, each with scopes
//     TransactionAttribute[] Attributes;  // OracleResponse, HighPriority, etc.
//     byte[] Script;          // VM bytecode to execute
//     Witness[] Witnesses;    // one per Signer, in same order
//   }
//
// New capabilities are added via:
//   - Bumping Version
//   - Adding new TransactionAttribute kinds
//   - Adding new Signer scope flags
//
// All fields are present in every transaction. There's no parallel format —
// older nodes simply reject transactions whose Version they don't support.

using Neo;
using Neo.SmartContract.Framework;
using Neo.SmartContract.Framework.Attributes;
using Neo.SmartContract.Framework.Native;

namespace R3E.Examples;

[DisplayName("TxIntrospection")]
public class TxIntrospection : SmartContract
{
    /// <summary>Read transaction-level fields available to any contract during execution.</summary>
    public static (uint, ulong, ulong, byte) CurrentTx()
    {
        var tx = (Transaction)Runtime.ScriptContainer;
        return (tx.ValidUntilBlock, (ulong)tx.SystemFee, (ulong)tx.NetworkFee, tx.Version);
    }
}
```

</template>

</StandardEntry>

<!-- ============================================================ -->
<!-- EIP-2930 -->
<!-- ============================================================ -->

<StandardEntry
  id="eip-2930"
  title="EIP-2930 — Access Lists"
  eip="2930"
  status="Final"
  neoMapping="Witness scopes (CustomContracts)"
  category="Transactions"
  parityLabel="Native"
  parityClass="sm-pill-native"
>

<template #spec>

## EIP-2930: Access Lists

EIP-2930 adds an optional `accessList` field to transactions: a pre-declared list
of `(address, storageKeys[])` tuples that the transaction will touch. Pre-declared
state access gets a gas discount; non-declared access costs more. The goal: enable
parallel transaction execution by letting nodes know in advance what state each tx
needs.

### Format

```
accessList = [
  (address_1, [storageKey_1a, storageKey_1b, ...]),
  (address_2, [storageKey_2a, ...]),
  ...
]
```

### Neo Equivalent: Witness Scopes

Neo's `WitnessScope.CustomContracts` already serves the same role for the
authorisation surface — declaring "this signature authorises calls only into these
contracts". Combined with `Signer.AllowedContracts` and `Signer.AllowedGroups`,
the protocol knows ahead of time which contracts a transaction can touch.

For the parallel-execution use case (knowing storage keys ahead of time), Neo's
storage prefix model makes static analysis tractable — the contract bytecode
declares its storage prefixes via `Storage.Get/Put/Delete` call sites.

</template>

<template #solidity>

```solidity
// EIP-2930 type-1 transaction:
//
//   0x01 || rlp([
//     chainId, nonce, gasPrice, gasLimit,
//     to, value, data,
//     accessList,       // [(address, storageKeys[])]
//     v, r, s
//   ])
//
// Application impact: contracts can be written without thinking about access
// lists; they're a transaction-level concern. Protocols that benefit:
// MEV bots that pre-compute the storage they'll touch and pass it as access list
// for a small gas discount and inclusion priority.
```

</template>

<template #csharp>

```csharp
// Neo witness scopes — equivalent of EIP-2930's authorisation surface:
//
//   enum WitnessScope : byte {
//     None             = 0x00,    // signer pays fee but grants no contract authority
//     CalledByEntry    = 0x01,    // grant only to top-level invoke and direct callees
//     CustomContracts  = 0x10,    // grant to listed contracts only
//     CustomGroups     = 0x20,    // grant to listed groups (cert-signed)
//     WitnessRules     = 0x40,    // arbitrary pattern matching
//     Global           = 0x80     // unlimited grant (dangerous; usually rejected)
//   }
//
// Building a transaction with access-list-equivalent scoping:
//
//   const tx = new TransactionBuilder()
//     .invoke(swapRouter, "swap", [tokenIn, tokenOut, amount])
//     .signers([{
//       account: user,
//       scopes:  WitnessScope.CustomContracts,
//       allowedContracts: [tokenIn, tokenOut, swapRouter]
//     }])
//     .build();
//
// The protocol enforces: any Contract.Call from inside the transaction that
// would require user's witness must be to one of allowedContracts, or
// CheckWitness(user) returns false. This is a finer-grained authorisation
// surface than EIP-2930's access list, which is purely a gas-pricing hint.

using Neo;
using Neo.SmartContract.Framework;
using Neo.SmartContract.Framework.Attributes;
using Neo.SmartContract.Framework.Native;
using Neo.SmartContract.Framework.Services;

namespace R3E.Examples;

[DisplayName("WitnessIntrospection")]
public class WitnessIntrospection : SmartContract
{
    public static UInt160[] CurrentSigners()
    {
        var signers = Runtime.CurrentSigners;
        var output  = new UInt160[signers.Length];
        for (int i = 0; i < signers.Length; i++) output[i] = signers[i].Account;
        return output;
    }
}
```

### Why Neo's Version Is Stronger

EIP-2930 access lists are advisory — they're a gas optimisation, not an
authorisation boundary. Witness scopes are **enforced**: a contract that's not in
your allowed list cannot pretend you authorised it.

</template>

</StandardEntry>

<!-- ============================================================ -->
<!-- EIP-3198 -->
<!-- ============================================================ -->

<StandardEntry
  id="eip-3198"
  title="EIP-3198 — BASEFEE Opcode"
  eip="3198"
  status="Final"
  neoMapping="Policy.GetFeePerByte"
  category="Fees"
  parityLabel="Native"
  parityClass="sm-pill-native"
>

<template #spec>

## EIP-3198: BASEFEE Opcode

EIP-1559 added a base fee to blocks. EIP-3198 added the `BASEFEE` opcode (`block.basefee`
in Solidity) so contracts can read the current block's base fee. Used by
fee-aware contracts and on-chain MEV mitigation logic.

### Neo Equivalent

The PolicyContract native is the equivalent: contracts read fee parameters via
`Policy.GetFeePerByte()` and `Policy.GetExecFeeFactor()`. The fee model is
deterministic per-block but doesn't auto-adjust; values change only when committee
nodes vote a change.

</template>

<template #solidity>

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.7;

contract FeeAware {
    /// @notice Estimate cost of a method given the current base fee.
    function estimateCost(uint256 expectedGasUsed) external view returns (uint256 wei_) {
        return expectedGasUsed * block.basefee;
    }

    /// @notice Refuse to operate if base fee is unreasonably high
    /// (e.g. avoid burning user funds during a gas spike).
    modifier reasonableBaseFee() {
        require(block.basefee < 200 gwei, "base fee too high");
        _;
    }
}
```

</template>

<template #csharp>

```csharp
using System.Numerics;
using Neo;
using Neo.SmartContract.Framework;
using Neo.SmartContract.Framework.Attributes;
using Neo.SmartContract.Framework.Native;

namespace R3E.Examples;

[DisplayName("FeeAware")]
public class FeeAware : SmartContract
{
    /// <summary>Network fee per byte from the policy contract.</summary>
    public static long FeePerByte() => Policy.GetFeePerByte();

    /// <summary>Estimate the network fee for a transaction of given size.</summary>
    public static long EstimateNetworkFee(uint txSize)
        => txSize * Policy.GetFeePerByte();

    /// <summary>Multiplier applied to per-op system fees.</summary>
    public static uint ExecFeeFactor() => Policy.GetExecFeeFactor();
}
```

</template>

</StandardEntry>

<!-- ============================================================ -->
<!-- EIP-3855 -->
<!-- ============================================================ -->

<StandardEntry
  id="eip-3855"
  title="EIP-3855 — PUSH0 Opcode"
  eip="3855"
  status="Final"
  neoMapping="Native PUSH0 in NeoVM"
  category="Opcodes"
  parityLabel="Native"
  parityClass="sm-pill-native"
>

<template #spec>

## EIP-3855: PUSH0 Opcode

Before EIP-3855, pushing zero onto the EVM stack required `PUSH1 0x00` (2 bytes,
3 gas). EIP-3855 added `PUSH0` (`0x5f`) — 1 byte, 2 gas. A trivial-looking change
that saves significant bytecode size and gas across the entire ecosystem because
zero is the most-pushed value.

### Status

Activated in the Shanghai hard fork (April 2023). Solidity 0.8.20+ emits PUSH0 by
default for chains that support it.

### Neo Equivalent

NeoVM has had `PUSH0` (opcode `0x10`) since launch, plus `PUSH1` through `PUSH16` as
direct integer constants. There was no need for a retrofit because the design
included it from day one.

</template>

<template #solidity>

```
EVM bytecode comparison:

Pre-EIP-3855:    60 00     PUSH1 0x00
                 [2 bytes, 3 gas]

Post-EIP-3855:   5f         PUSH0
                 [1 byte, 2 gas]

Across an entire compiled contract, this can save dozens of bytes and millions of
gas across thousands of executions. OZ's ERC-20 dropped ~50 bytes after the
PUSH0 introduction.
```

</template>

<template #csharp>

```
NeoVM bytecode (since day one):

  PUSH0 (0x10)    // push integer 0
  PUSH1 (0x11)    // push integer 1
  PUSH2 (0x12)    // push integer 2
  ...
  PUSH16 (0x20)   // push integer 16

  PUSHM1 (0x0F)   // push integer -1

  PUSHA   (0x0A)  // push address
  PUSHNULL (0x0B) // push StackItem.Null

  PUSHDATA1 (0x0C) ...  // push N bytes (N <= 255)

A Solidity-to-NeoVM compiler (like neo-solidity) emits PUSH0 directly when it
needs zero. There's no retrofit story here because the VM was designed with
small-integer constant opcodes from the start.
```

### Compiler Note

The `neo-solidity` compiler maps Solidity zero literals to NeoVM `PUSH0` on every
emit. No version gate, no chain-version conditional — the opcode has always been
available.

</template>

</StandardEntry>

<!-- ============================================================ -->
<!-- EIP-3860 -->
<!-- ============================================================ -->

<StandardEntry
  id="eip-3860"
  title="EIP-3860 — Initcode Size Limit"
  eip="3860"
  status="Final"
  neoMapping="NEF format limits"
  category="Deployment"
  parityLabel="Native"
  parityClass="sm-pill-native"
>

<template #spec>

## EIP-3860: Limit and Meter Initcode

EIP-3860 (Shanghai) caps contract creation `initcode` at **49152 bytes** (`0xc000`)
and meters initcode execution by 2 gas per 32-byte word. Without this cap, a very
large initcode could consume excessive resources during contract deployment.

### Why It Was Needed

Pre-3860, an attacker could submit a transaction creating a contract with a 10MB
initcode, forcing every full node to process the entire blob just to determine
the deploy fails (or succeeds with absurd code). Cheap DoS surface.

### Neo Equivalent

The Neo NEF (Neo Executable Format) has explicit limits baked in:

- **Script size**: max 1 MB per contract (much higher than 48 KB but still bounded).
- **Manifest size**: max 64 KB.
- **Method count, parameter counts, etc.**: all bounded.

Contract deployment cost on Neo scales linearly with NEF size via the network fee
(byte-cost) plus a flat deployment cost from `ContractManagement.GetMinimumDeploymentFee()`.

</template>

<template #solidity>

```
EIP-3860 effects on contract deployment:

  if (initcodeSize > 49152) revert;
  initcodeWords = (initcodeSize + 31) / 32;
  initCodeCost  = 2 * initcodeWords;

  totalGasCost  = baseDeployCost + initCodeCost + executionCost;

This was specifically about CREATE / CREATE2 transaction-level deploys.
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
/// Neo deployment fee model — informational. ContractManagement enforces:
///
///   - NEF.Script.Length <= 0x100000 (1 MB)
///   - Manifest serialised length <= 0xffff (64 KB)
///   - Manifest.Name length <= 32 chars
///   - Manifest.SupportedStandards count <= 8
///   - Manifest.Permissions count <= 16
///   - Manifest.Trusts count <= 16
///   - Manifest.Groups count <= 16
///   - Manifest.Abi.Methods count <= 1000
///   - Manifest.Abi.Events count <= 1000
///
/// Cost: minimumDeploymentFee + (NEF.Length + Manifest.Length) * StoragePrice
///
/// Querying the limits at runtime:
/// </summary>
[DisplayName("DeploymentLimits")]
public class DeploymentLimits : SmartContract
{
    public static long MinimumDeploymentFee()
        => ContractManagement.GetMinimumDeploymentFee();
}
```

### What Goes Away

EIP-3860 was a retrofit because Ethereum had no original limit on initcode size.
NEF's structured format with explicit length-prefixed sections forces every limit
to be declared and enforced at parse time — a contract whose NEF would exceed the
limit can't even be serialised, let alone deployed.

</template>

</StandardEntry>

<!-- ============================================================ -->
<!-- EIP-4844 -->
<!-- ============================================================ -->

<StandardEntry
  id="eip-4844"
  title="EIP-4844 — Proto-Danksharding (Blobs)"
  eip="4844"
  status="Final"
  neoMapping="Native sharding via state channels + Oracle"
  category="Scaling"
  parityLabel="Native"
  parityClass="sm-pill-native"
>

<template #spec>

## EIP-4844: Shard Blob Transactions (Proto-Danksharding)

EIP-4844 (Cancun, 2024) introduces **blob-carrying transactions** — type-3 txs
that carry up to 6 "blobs" of 128 KB each. Blobs are stored on the consensus
layer for ~18 days, then pruned. Designed to massively reduce L2 rollup data costs
because rollups can publish their compressed state diffs as cheap blobs instead of
expensive calldata.

### Mechanics

- Blobs are committed to via KZG polynomial commitments.
- On-chain code can verify a KZG commitment but cannot read blob data directly.
- Rollups post blob commitments; rollup verifiers reconstruct off-chain.

### Neo Equivalent

Neo's scalability model is different: rather than blob storage to support L2
rollups, Neo emphasises **on-chain throughput** (~4000 TPS at the protocol level)
plus **state channels** for off-chain settlement when needed. The Oracle native
provides off-chain data ingestion when contracts need to reference data not stored
on-chain.

For a Neo contract that needs to verify large external data, the typical pattern
is to store a hash on-chain and verify Merkle/KZG/etc. proofs in C#. Neo's CryptoLib
exposes the building blocks (`Sha256`, `Ripemd160`, `VerifyWithECDsa`,
`Bls12381*`).

</template>

<template #solidity>

```solidity
// EIP-4844 blob tx (type 0x03) format includes:
//   maxFeePerBlobGas    = uint256
//   blobVersionedHashes = bytes32[]   (KZG commitments)
//
// On-chain code can introspect via:

contract BlobReader {
    /// EIP-7516 BLOBBASEFEE opcode
    function blobBaseFee() external view returns (uint256) {
        // return block.blobbasefee;
        return 0;
    }

    /// EIP-4844 BLOBHASH opcode — read the i-th blob's versioned hash
    function blobHash(uint256 index) external view returns (bytes32 vh) {
        // assembly { vh := blobhash(index) }
    }

    /// Verify a KZG point evaluation against a blob commitment.
    /// Used by rollups to prove specific blob bytes without fetching the whole blob.
    function verifyKzg(
        bytes32 blobCommitment,
        bytes calldata kzgProof,
        bytes32 z,
        bytes32 y
    ) external view returns (bool) {
        // staticcall to 0x0a precompile (EIP-4844 point eval precompile)
        return false;
    }
}
```

</template>

<template #csharp>

```csharp
using System.Numerics;
using Neo;
using Neo.SmartContract.Framework;
using Neo.SmartContract.Framework.Attributes;
using Neo.SmartContract.Framework.Native;
using Neo.SmartContract.Framework.Services;

namespace R3E.Examples;

[DisplayName("LargeDataAttestation")]
[ContractPermission("*", "*")]
public class LargeDataAttestation : SmartContract
{
    private const byte Prefix_Attestation = 0x01;

    [DisplayName("DataAttested")]
    public static event System.Action<ByteString, ByteString, BigInteger> OnAttested;

    /// <summary>
    /// Pattern: contracts attest to large off-chain data by storing a hash.
    /// Verifiers fetch the data off-chain (IPFS, CDN, side-channel) and verify
    /// the hash matches before accepting.
    /// </summary>
    public static void AttestDataHash(ByteString contentId, ByteString sha256Hash, BigInteger size)
    {
        if (!Runtime.CheckWitness(GetAdmin())) throw new Exception("admin only");
        Storage.Put(Storage.CurrentContext,
            new byte[] { Prefix_Attestation }.Concat(contentId),
            StdLib.Serialize(new object[] { sha256Hash, size, Runtime.Time }));
        OnAttested(contentId, sha256Hash, size);
    }

    /// <summary>Verify that a blob of bytes matches a previously-attested hash.</summary>
    public static bool VerifyData(ByteString contentId, ByteString data)
    {
        var raw = Storage.Get(Storage.CurrentContext,
                              new byte[] { Prefix_Attestation }.Concat(contentId));
        if (raw == null) return false;
        var attest = (object[])StdLib.Deserialize(raw);
        var expected = (ByteString)attest[0];
        return CryptoLib.Sha256(data).Equals(expected);
    }

    /// <summary>
    /// For BLS / KZG style commitments, CryptoLib exposes BLS12-381 ops directly.
    /// Used by zk-rollups, threshold signatures, KZG proofs.
    /// </summary>
    public static bool VerifyBls(ByteString message, ByteString signature, ByteString publicKey)
    {
        var sig = (BLS12_381)signature;
        var pk  = (BLS12_381)publicKey;
        // Application-specific BLS verification flow
        return true;
    }

    private static UInt160 GetAdmin() => (UInt160)"0x0000000000000000000000000000000000000000";
}
```

### Neo's Approach to Scaling

Neo emphasises high base-layer throughput rather than rollup-style data
availability. For DApps that need off-chain compute or state, the canonical
Neo pattern is:

1. Store hashes on-chain.
2. Use the Oracle native for verifiable off-chain data fetches.
3. Use BLS/Merkle/KZG primitives in CryptoLib for cryptographic proofs.

If/when Neo adopts a sharding model, blob-style data availability would slot in
as a new transaction attribute or native contract — the architecture supports
incremental extension.

</template>

</StandardEntry>

<!-- ============================================================ -->
<!-- EIP-1153 -->
<!-- ============================================================ -->

<StandardEntry
  id="eip-1153"
  title="EIP-1153 — Transient Storage"
  eip="1153"
  status="Final"
  neoMapping="Native (Storage with transaction-scoped lifetime)"
  category="Storage"
  parityLabel="Native"
  parityClass="sm-pill-native"
>

<template #spec>

## EIP-1153: Transient Storage Opcodes (TSTORE / TLOAD)

EIP-1153 (Cancun, 2024) introduced two new opcodes — `TSTORE` and `TLOAD` — that
read/write storage that lives only for the duration of a single transaction.
Persistent storage (`SSTORE`) costs 20,000 gas; transient storage costs 100 gas.

### Use Cases

- **Re-entrancy guards**: a `bool` flag set during entry, checked on re-entry,
  cleared at the end of the call. Without TSTORE this requires writing a regular
  storage slot at 20K gas.
- **Inter-call communication**: a contract that needs to pass non-trivial state
  through a callback no longer pays for permanent storage.
- **Cached computation**: expensive views computed once per transaction.

### Neo Equivalent

Neo's storage API supports the same pattern via per-transaction storage scopes.
A contract can use a small, manually-managed transient store: write a value, use
it, delete it before the transaction ends. The cost is similar to permanent
storage at write/delete, but the value is gone after the tx — no permanent state
bloat.

For the most common use case (re-entrancy guards), the standard Neo idiom is
to use a contract-local boolean stored in transient memory implemented through
a deliberate `Put` then `Delete` pattern, OR to use the `Runtime.GetTrigger() ==
TriggerType.Application` check combined with `Runtime.ExecutingScriptHash !=
Runtime.CallingScriptHash` to detect external entry. Neo VM's call semantics
prevent a re-entrancy hazard pattern much more strongly than EVM's, so guards
are needed less often.

</template>

<template #solidity>

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

abstract contract ReentrancyGuardTransient {
    bytes32 private constant LOCKED = keccak256("ReentrancyGuard.LOCKED");

    modifier nonReentrant() {
        require(_locked() == 0, "reentrant");
        _setLocked(1);
        _;
        _setLocked(0);
    }

    function _locked() private view returns (uint256 v) {
        bytes32 slot = LOCKED;
        assembly { v := tload(slot) }
    }

    function _setLocked(uint256 v) private {
        bytes32 slot = LOCKED;
        assembly { tstore(slot, v) }
    }
}

contract MyVault is ReentrancyGuardTransient {
    function withdraw(uint256 amount) external nonReentrant {
        // ... safe to do external calls here ...
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
/// Neo re-entrancy guard. Two patterns:
///
/// (a) Storage-based: same as EIP-1153 conceptually — write a flag, check on
///     re-entry, delete at exit. Storage in Neo is only persistent if you don't
///     delete it; transient effect = put+delete in one tx.
///
/// (b) Caller-based: use Runtime.CallingScriptHash to detect direct vs reentrant
///     calls. Often sufficient because Neo's call model is more restrictive than
///     delegatecall-heavy EVM patterns.
/// </summary>
[DisplayName("ReentrancyGuarded")]
[ContractPermission("*", "*")]
public class ReentrancyGuarded : SmartContract
{
    private static readonly byte[] LockKey = { 0x70 };

    public static void Withdraw(BigInteger amount)
    {
        if (Storage.Get(Storage.CurrentContext, LockKey) != null)
            throw new Exception("reentrant");
        Storage.Put(Storage.CurrentContext, LockKey, 1);

        try
        {
            // ... external calls / asset transfers here ...
        }
        finally
        {
            Storage.Delete(Storage.CurrentContext, LockKey);
        }
    }
}
```

### Cost Comparison

| Op | EVM (post-EIP-1153) | Neo |
| --- | --- | --- |
| Set guard | TSTORE — 100 gas | Put — ~3.5 GAS at default fee factor |
| Check guard | TLOAD — 100 gas | Get — ~1 GAS |
| Clear guard | TSTORE 0 — 100 gas | Delete — ~1 GAS |

Neo's storage costs are higher in absolute terms but the gas-to-value-protected
ratio is similar — a guarded function call costs only marginally more than an
unguarded one in either system.

</template>

</StandardEntry>

<!-- ============================================================ -->
<!-- EIP-6780 -->
<!-- ============================================================ -->

<StandardEntry
  id="eip-6780"
  title="EIP-6780 — SELFDESTRUCT Restriction"
  eip="6780"
  status="Final"
  neoMapping="ContractManagement.Destroy (explicit)"
  category="Lifecycle"
  parityLabel="Native"
  parityClass="sm-pill-native"
>

<template #spec>

## EIP-6780: Reduced SELFDESTRUCT Functionality

`SELFDESTRUCT` originally:
1. Destroyed the contract's bytecode and storage.
2. Forwarded the contract's ETH balance to a designated recipient.
3. Refunded gas to the caller (incentive to clean up unused contracts).

This created multiple footguns: re-deploying via CREATE2 to the same address with
different code (because storage was wiped), unintended data loss in proxy contracts
when the implementation called `selfdestruct`, MEV opportunities around
"refund the gas" mechanics.

EIP-6780 (Cancun) reduced `SELFDESTRUCT` to:
- **If called in the same transaction as the contract was created**: full original
  behavior (destroy + forward funds + refund gas).
- **Otherwise**: only forward the balance. The bytecode and storage stay intact.

In effect, `SELFDESTRUCT` is now a **balance-sweep** opcode for already-deployed
contracts, not a destruction primitive.

### Neo Equivalent

Neo's `ContractManagement.Destroy()` has always been:
- An explicit method call (no opcode).
- Authorized by the contract's witness.
- Removes the contract's storage and manifest.
- Has no automatic fund-forwarding or gas refund.

The "destroy" path is intentionally narrow: contracts must explicitly opt in by
implementing a callable destroy method, witnesses must check, and any token
balances held at the contract address must be transferred out beforehand by
contract logic — there's no implicit sweep.

</template>

<template #solidity>

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

contract OldStyle {
    function destroy(address payable recipient) external {
        // Pre-Cancun: this destroyed the bytecode AND swept ETH.
        // Post-EIP-6780: only sweeps ETH unless this is a same-tx-deploy contract.
        // A surprising change for many existing contracts.
        selfdestruct(recipient);
    }
}

contract Sweeper {
    /// New idiomatic pattern: explicit balance transfer instead of selfdestruct.
    function sweep(address payable recipient) external {
        recipient.transfer(address(this).balance);
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

[DisplayName("DestroyableContract")]
[ContractPermission("*", "*")]
public class DestroyableContract : SmartContract
{
    private static readonly byte[] OwnerKey = { 0xff };

    [DisplayName("Destroyed")]
    public static event Action<UInt160> OnDestroyed;

    /// <summary>
    /// NEP-31 destroy. Explicit, witness-gated, removes storage and manifest.
    /// No automatic fund forwarding — handle that separately.
    /// </summary>
    public static void Destroy()
    {
        var owner = (UInt160)Storage.Get(Storage.CurrentContext, OwnerKey);
        if (!Runtime.CheckWitness(owner)) throw new Exception("owner only");

        // Sweep any token balances held at this contract before destruction
        // (user must list which tokens to sweep — there's no implicit sweep).
        // Example for GAS:
        var gasBal = (BigInteger)Contract.Call(GAS.Hash, "balanceOf", CallFlags.ReadOnly,
                                               new object[] { Runtime.ExecutingScriptHash });
        if (gasBal > 0)
            Contract.Call(GAS.Hash, "transfer", CallFlags.All,
                          new object[] { Runtime.ExecutingScriptHash, owner, gasBal, "destroy-sweep" });

        OnDestroyed(Runtime.ExecutingScriptHash);
        ContractManagement.Destroy();
    }
}
```

### What's Different

| Aspect | EVM `selfdestruct` | Neo `ContractManagement.Destroy` |
| --- | --- | --- |
| Trigger | Opcode in middle of execution | Explicit method call |
| Auth | Implicit (whoever holds caller authority) | Explicit `Runtime.CheckWitness` check |
| Bytecode removal | Conditional (post-6780) | Always |
| Fund handling | Implicit ETH transfer | Application code transfers out before destroy |
| Gas refund | Was a thing pre-EIP-3529 | Never existed |
| Same-tx redeploy | Possible via CREATE2 | Not applicable — script hash is content-derived |

</template>

</StandardEntry>

<!-- ============================================================ -->
<!-- EIP-2098 -->
<!-- ============================================================ -->

<StandardEntry
  id="eip-2098"
  title="EIP-2098 — Compact Signatures"
  eip="2098"
  status="Final"
  neoMapping="Native (Neo signatures already compact)"
  category="Signatures"
  parityLabel="Native"
  parityClass="sm-pill-native"
>

<template #spec>

## EIP-2098: Compact Signature Representation

ECDSA signatures on the secp256k1 curve consist of `(r, s, v)` where `r` and `s`
are 32-byte field elements and `v` is the recovery byte (1 bit, but stored as a
full byte for alignment reasons → 65 bytes total).

EIP-2098 packs `v` into the high bit of `s`: since secp256k1 enforces low-`s`
canonicality (EIP-2 / EIP-2098 itself), the high bit of `s` is always 0, so it
can carry the recovery bit. Result: 64-byte signatures instead of 65.

### Why It Matters

- Smart contract storage and calldata cost reduction.
- Off-chain protocols that bundle many signatures (multi-sigs, DAOs) save 1.5%.
- Cleaner compatibility with EVM contracts that take `bytes32` slot-aligned signatures.

### Neo Equivalent

Neo signatures are 64 bytes natively. The verification curve is **secp256r1**
(NIST P-256, ECDSA), and the signature format is `(r, s)` directly — no recovery
byte because Neo verification doesn't need to recover the public key from the
signature (the public key is in the witness script).

</template>

<template #solidity>

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

library Compact2098 {
    /// Decode a 64-byte EIP-2098 signature into (v, r, s).
    function decode(bytes32 r, bytes32 vs)
        internal pure returns (uint8 v, bytes32 r_, bytes32 s)
    {
        v = uint8((uint256(vs) >> 255) + 27);
        s = bytes32(uint256(vs) & ((1 << 255) - 1));
        r_ = r;
    }

    /// Recover the signer of a hash given a 2098 compact signature.
    function recover(bytes32 hash, bytes32 r, bytes32 vs)
        internal pure returns (address)
    {
        (uint8 v, , bytes32 s) = decode(r, vs);
        return ecrecover(hash, v, r, s);
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
/// Neo signatures are 64 bytes from the start — there's no compaction to apply.
///
///   signature = r (32 bytes) || s (32 bytes)
///
/// Verification uses CryptoLib.VerifyWithECDsa which takes the raw 64-byte
/// signature plus a public key (because Neo doesn't do recovery). The public
/// key is part of the verification script — the protocol already knows it
/// when verifying transaction witnesses.
/// </summary>
[DisplayName("SigVerify")]
public class SigVerify : SmartContract
{
    public static bool Verify(ByteString message, ECPoint pubKey, ByteString signature)
    {
        // signature.Length must be 64. CryptoLib enforces this.
        return CryptoLib.VerifyWithECDsa(message, pubKey, signature, NamedCurve.secp256r1);
    }
}
```

### Curve Note

Neo verifies on the **NIST P-256 (secp256r1)** curve, which is implemented in
hardware on most modern phones (Apple Secure Enclave, Android StrongBox), unlike
Ethereum's secp256k1 which doesn't have widespread hardware support. This means
Neo signatures can be produced inside hardware security modules out of the box —
useful for biometric authentication and secure-element wallets.

</template>

</StandardEntry>

</StandardsMirror>
