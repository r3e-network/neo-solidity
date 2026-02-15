# Multicall (OpenZeppelin)

## Audit Snapshot

- Status: ✅ pass
- Source type: `npm`
- Source path: `third_party/famous-contracts/sources/@openzeppelin/contracts/utils/Multicall.sol`
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
| warning | W200 | function 'sendValue' in 'Multicall' overrides 'Context::sendValue' which is not marked 'virtual' |
| warning | W200 | function 'sendValue' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCall' in 'Multicall' overrides 'Context::functionCall' which is not marked 'virtual' |
| warning | W200 | function 'functionCall' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCallWithValue' in 'Multicall' overrides 'Context::functionCallWithValue' which is not marked 'virtual' |
| warning | W200 | function 'functionCallWithValue' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionStaticCall' in 'Multicall' overrides 'Context::functionStaticCall' which is not marked 'virtual' |
| warning | W200 | function 'functionStaticCall' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionDelegateCall' in 'Multicall' overrides 'Context::functionDelegateCall' which is not marked 'virtual' |
| warning | W200 | function 'functionDelegateCall' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResultFromTarget' in 'Multicall' overrides 'Context::verifyCallResultFromTarget' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResultFromTarget' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResult' in 'Multicall' overrides 'Context::verifyCallResult' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResult' in 'Multicall' overrides a base function but is not marked 'override' |
| warning | W200 | function '_revert' in 'Multicall' overrides 'Context::_revert' which is not marked 'virtual' |
| warning | W200 | function '_revert' in 'Multicall' overrides a base function but is not marked 'override' |

## References

- Global audit report: [Famous Contracts on NeoVM](/solidity/famous-contracts-neo-audit)
- Per-contract index: [Original Famous Contracts](/solidity/original-contracts/)
- Upstream contract path: `third_party/famous-contracts/sources/@openzeppelin/contracts/utils/Multicall.sol`