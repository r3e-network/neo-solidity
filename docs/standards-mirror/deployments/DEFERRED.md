# Standards Mirror — Deferred Deployment Queue

The recurring agent (`/schedule` routine `Standards Mirror — Add 2 Pairs Every Monday`,
trigger ID `trig_01BJaUX5fVuP9EvGqhtgW7UP`) reads this file each Monday at 09:00 UTC,
pops the **first 2 lines** of the priority list below, and ships a Solidity + Neo C#
pair for each — compiled, deployed to Neo testnet, behavior-verified, and documented.

When this list is empty, the agent opens one final no-op PR marking the queue
complete and self-disables.

## Priority queue

The first 4 are canonical standards we haven't deployed yet. The remaining 11 are
"demo wrapper" entries: their Ethereum forms are extensions / method conventions /
protocol EIPs that don't normally have standalone contracts, but a small Neo
demonstration contract is useful for the mirror's educational purpose.

8 entries previously here were popped and shipped in the v0.19.0 docs round
(ERC-777, ERC-5267, ERC-5114, ERC-5484, ERC-6147, ERC-2470, ERC-2309, ERC-4906).

```
ERC-7540 — Async ERC-4626 vault
ERC-7575 — Multi-asset vault
ERC-7579 — Modular smart account
ERC-4337 — Smart account (NEP-30 verify)
ERC-6492 — Pre-deploy signature verifier
ERC-4494 — PermitNFT (NFT with witness-scope permit demo)
ERC-2612 — PermitToken (ERC-20 with witness-scope permit demo)
ERC-1014 — DeterministicFactory (CREATE2-style deploy demo via ContractManagement.Deploy)
ERC-165 — InterfaceDetectorWrapper (supportsInterface compatibility shim)
ERC-7201 — NamespacedStorageDemo (prefix-storage idiom contract)
EIP-712 — TypedDataVerifier (digest-based off-chain signature verification)
EIP-191 — PersonalSignVerifier (CryptoLib.VerifyWithECDsa demo)
EIP-3198 — FeeAwareContract (Policy.GetFeePerByte introspection)
EIP-1153 — TransientReentrancyGuard (Storage put+delete pattern)
EIP-2098 — CompactSigVerifier (ECDSA secp256r1 signature compactness demo)
```

## How to use

The agent's logic:

1. Read this file.
2. Take the first 2 non-empty lines.
3. Author + compile + deploy + test + document each.
4. Open a single PR `docs(standards-mirror): add ERC-XXX + ERC-YYY pairs`.
5. After merge, open a follow-up PR removing the popped lines from this file.

Manual additions / removals are fine — edit the file directly. The agent does not
sort or re-prioritize; it just pops the head of the list.

## Compile guardrails (must follow)

These came out of the deploy effort that produced PRs #5–#8. Future deploys must
respect them or they fault on real Neo nodes:

- **`uint256(uintN var)` casts in constructor faults at deploy** (neo-solc 0.18,
  unfixed). Use literal pre-computed values.
- **`emit Event(...)` whose `keccak256(signature)` starts with byte `0xDD`** would
  fault before PR #6's compiler fix. The fix shipped, but stick to non-deploy
  paths for emits as a safety margin.
- **`target.call(opaque_bytes)` is unsupported by neo-solc** — it requires
  statically-known method names. Use named-method invocation patterns instead.
- **`msg.sender` at constructor time on Neo is the ManagementContract**, not the
  deploying user. Use a `claimOwner()` / `claimIssuer()` post-deploy pattern in
  Solidity. PR #6's `msg.sender` fix routes to `Transaction.Sender` for the
  ContractManagement caller, but constructor-time identity is still risky.
- **Neo C# `(ByteString)str` for string→ByteString**, never
  `Encoding.UTF8.GetBytes` (nccs rejects).
- **Use `Runtime.Transaction.Sender` at deploy time** in C#, not
  `Runtime.CallingScriptHash`.
- **Add `[Safe]` to every read-only method** so the manifest correctly advertises
  read/write surface.
