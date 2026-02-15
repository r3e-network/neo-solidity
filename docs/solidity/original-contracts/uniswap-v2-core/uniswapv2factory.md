# UniswapV2Factory (Uniswap V2 Core)

## Audit Snapshot

- Status: ✅ pass
- Source type: `npm`
- Source path: `node_modules/@uniswap/v2-core/contracts/UniswapV2Factory.sol`
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
| error | RAW | [info][NEP-17] NEP-17 `transfer` method has 2 parameter(s), spec expects 4. See STANDARDS_MAPPING.md for details. |
| error | RAW | [info][NEP-17] NEP-17 `transfer` method has 2 parameter(s), spec expects 4. See STANDARDS_MAPPING.md for details. |
| warning | W101 | function 'transfer' has 2 parameters (ERC-20 pattern). NEP-17 requires 4 parameters: transfer(from, to, amount, data). The `from` address is verified via Runtime.checkWitness() and `data` (type Any) is forwarded to the recipient's onNEP17Payment callback. |
| warning | W103 | ERC-20 method(s) [approve, allowance, transferfrom] detected. These are not part of the NEP-17 spec; Neo uses Runtime.checkWitness() for authorization instead of the approve/allowance pattern. You may keep them as extensions, but they will not contribute to NEP-17 standard detection. |
| warning | W108 | ERC-2612 permit pattern detected (7-parameter permit function). Neo N3 uses Runtime.checkWitness() for authorization; off-chain signature permits are not needed. |
| warning | W113 | Contract has transfer function but no onNEP17Payment callback. Other contracts cannot send tokens to this contract. |
| warning | W121 | duplicate constant state variable 'Q112' detected while merging libraries |
| warning | W101 | function 'transfer' has 2 parameters (ERC-20 pattern). NEP-17 requires 4 parameters: transfer(from, to, amount, data). The `from` address is verified via Runtime.checkWitness() and `data` (type Any) is forwarded to the recipient's onNEP17Payment callback. |
| warning | W103 | ERC-20 method(s) [approve, allowance, transferfrom] detected. These are not part of the NEP-17 spec; Neo uses Runtime.checkWitness() for authorization instead of the approve/allowance pattern. You may keep them as extensions, but they will not contribute to NEP-17 standard detection. |
| warning | W108 | ERC-2612 permit pattern detected (7-parameter permit function). Neo N3 uses Runtime.checkWitness() for authorization; off-chain signature permits are not needed. |
| warning | W113 | Contract has transfer function but no onNEP17Payment callback. Other contracts cannot send tokens to this contract. |
| warning | W200 | function 'add' in 'UniswapV2Pair' overrides 'UniswapV2ERC20::add' which is not marked 'virtual' |
| warning | W200 | function 'add' in 'UniswapV2Pair' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sub' in 'UniswapV2Pair' overrides 'UniswapV2ERC20::sub' which is not marked 'virtual' |
| warning | W200 | function 'sub' in 'UniswapV2Pair' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mul' in 'UniswapV2Pair' overrides 'UniswapV2ERC20::mul' which is not marked 'virtual' |
| warning | W200 | function 'mul' in 'UniswapV2Pair' overrides a base function but is not marked 'override' |
| warning | W200 | function 'min' in 'UniswapV2Pair' overrides 'UniswapV2ERC20::min' which is not marked 'virtual' |
| warning | W200 | function 'min' in 'UniswapV2Pair' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sqrt' in 'UniswapV2Pair' overrides 'UniswapV2ERC20::sqrt' which is not marked 'virtual' |
| warning | W200 | function 'sqrt' in 'UniswapV2Pair' overrides a base function but is not marked 'override' |
| warning | W200 | function 'encode' in 'UniswapV2Pair' overrides 'UniswapV2ERC20::encode' which is not marked 'virtual' |
| warning | W200 | function 'encode' in 'UniswapV2Pair' overrides a base function but is not marked 'override' |
| warning | W200 | function 'uqdiv' in 'UniswapV2Pair' overrides 'UniswapV2ERC20::uqdiv' which is not marked 'virtual' |
| warning | W200 | function 'uqdiv' in 'UniswapV2Pair' overrides a base function but is not marked 'override' |
| warning | MANIFEST_WILDCARD_CONTRACT | contract 'UniswapV2Pair' requires wildcard contract manifest permissions (contract='*') due to dynamic contract calls. This is riskier than fixed contract hashes; use --deny-wildcard-contracts to make this a hard error. |

## References

- Global audit report: [Famous Contracts on NeoVM](/solidity/famous-contracts-neo-audit)
- Per-contract index: [Original Famous Contracts](/solidity/original-contracts/)
- Upstream contract path: `node_modules/@uniswap/v2-core/contracts/UniswapV2Factory.sol`