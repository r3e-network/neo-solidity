# Standards Mirror — Deferred Deployment Queue

> **Queue empty.** All 23 originally-deferred entries shipped across v0.19.0 (8) /
> v0.20.0 (8) / v0.21.0 (7). The recurring weekly agent has been retired.
>
> The mirror catalog now exposes **51 ERC/EIP mapping pages**. The TestNet matrix
> covers **47 deployable standards**. The four catalog entries without a live
> contract pair are EIP-1559 (fee-market auction), EIP-4844 (blob transactions),
> EIP-3074 (superseded by EIP-7702 and covered by witness scopes), and ERC-6909
> (documented as a direct Neo C# port but not part of the checked-in deployment
> snapshot).

## Shipped log

- **v0.19.0** (PR #10): ERC-777, ERC-5267, ERC-5114, ERC-5484, ERC-6147,
  ERC-2470, ERC-2309, ERC-4906
- **v0.20.0** (PR #11): ERC-165, ERC-7201, EIP-1153, EIP-3198, ERC-1014, EIP-191,
  ERC-2612, ERC-4494
- **v0.21.0**: ERC-7540, ERC-7575, ERC-7579, ERC-4337, ERC-6492,
  EIP-712, EIP-2098
- **v0.22.0**: EIP-2718, EIP-2930, EIP-3855, EIP-3860, EIP-6780, EIP-7702

## Compile guardrails (kept for future contract authors)

These came out of the deploy effort that produced PRs #5–#11. Future Neo-Solidity
contracts must respect them or they fault on real Neo nodes:

- **`uint256(uintN var)` casts in constructor fault at deploy** (neo-solc 0.18,
  still open). Use literal pre-computed values.
- **`emit Event(...)` whose `keccak256(signature)` starts with byte `0xDD`** would
  fault before PR #6's compiler fix. The fix shipped, but stick to non-deploy
  paths for emits as a safety margin.
- **`target.call(opaque_bytes)` is unsupported by neo-solc** — it requires
  statically-known method names. Use named-method invocation patterns instead.
- **`msg.sender` at constructor time on Neo is the ManagementContract**, not the
  deploying user. Use a `claimOwner()` / `claimIssuer()` post-deploy pattern in
  Solidity. The deploy runner must invoke the claim in the same deployment flow;
  manual deploys that leave a claimable owner unset allow the first post-deploy
  caller to seize that role. PR #6's `msg.sender` fix routes to
  `Transaction.Sender` for the ContractManagement caller, but constructor-time
  identity is still risky.
- **Neo C# `(ByteString)str` for string→ByteString**, never
  `Encoding.UTF8.GetBytes` (nccs rejects).
- **Use `Runtime.Transaction.Sender` at deploy time** in C#, not
  `Runtime.CallingScriptHash`.
- **Add `[Safe]` to every read-only method** so the manifest correctly advertises
  read/write surface.
