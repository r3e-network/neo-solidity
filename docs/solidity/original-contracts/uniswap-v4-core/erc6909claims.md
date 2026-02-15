# ERC6909Claims (Uniswap V4 Core)

## Audit Snapshot

- Status: ✅ pass
- Source type: `npm`
- Source path: `third_party/famous-contracts/sources/@uniswap/v4-core/src/ERC6909Claims.sol`
- Primary issue: No primary issue recorded.
- Audit corpus size: 98 contracts

## NeoVM Adaptation Status

This upstream contract compiled successfully in the audit run with current `neo-solc`.

Recommended hardening before production deployment:

1. Review generated manifest permissions and remove wildcard entries when possible.
1. Run Neo-Express state-changing tests for your target workflows, not only read-only calls.
1. Validate semantic differences (for example `tx.origin`, payable semantics, callback models) for your integration context.

## Diagnostics

| Severity | Code | Message |
| --- | --- | --- |
| warning | W102 | function 'transfer' has 3 parameters, but NEP-17 requires 4: transfer(from, to, amount, data). The `data` parameter (type Any) is forwarded to the recipient's onNEP17Payment callback. |
| warning | W103 | ERC-20 method(s) [approve, allowance, transferfrom] detected. These are not part of the NEP-17 spec; Neo uses Runtime.checkWitness() for authorization instead of the approve/allowance pattern. You may keep them as extensions, but they will not contribute to NEP-17 standard detection. |
| warning | W106 | function 'supportsInterface' (EIP-165) is unnecessary on Neo N3. Neo uses the manifest 'supportedstandards' array for interface detection, which the compiler populates automatically. |
| warning | W113 | Contract has transfer function but no onNEP17Payment callback. Other contracts cannot send tokens to this contract. |
| warning | W102 | function 'transfer' has 3 parameters, but NEP-17 requires 4: transfer(from, to, amount, data). The `data` parameter (type Any) is forwarded to the recipient's onNEP17Payment callback. |
| warning | W103 | ERC-20 method(s) [approve, allowance, transferfrom] detected. These are not part of the NEP-17 spec; Neo uses Runtime.checkWitness() for authorization instead of the approve/allowance pattern. You may keep them as extensions, but they will not contribute to NEP-17 standard detection. |
| warning | W106 | function 'supportsInterface' (EIP-165) is unnecessary on Neo N3. Neo uses the manifest 'supportedstandards' array for interface detection, which the compiler populates automatically. |
| warning | W113 | Contract has transfer function but no onNEP17Payment callback. Other contracts cannot send tokens to this contract. |

## References

- Global audit report: [Famous Contracts on NeoVM](/solidity/famous-contracts-neo-audit)
- Per-contract index: [Original Famous Contracts](/solidity/original-contracts/)
- Upstream contract path: `third_party/famous-contracts/sources/@uniswap/v4-core/src/ERC6909Claims.sol`