# TimelockController (OpenZeppelin)

## Audit Snapshot

- Status: ✅ pass
- Source type: `npm`
- Source path: `node_modules/@openzeppelin/contracts/governance/TimelockController.sol`
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
| warning | W106 | function 'supportsInterface' (EIP-165) is unnecessary on Neo N3. Neo uses the manifest 'supportedstandards' array for interface detection, which the compiler populates automatically. |
| warning | W106 | function 'supportsInterface' (EIP-165) is unnecessary on Neo N3. Neo uses the manifest 'supportedstandards' array for interface detection, which the compiler populates automatically. |
| warning | W200 | function 'sendValue' in 'ERC165' overrides 'Context::sendValue' which is not marked 'virtual' |
| warning | W200 | function 'sendValue' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCall' in 'ERC165' overrides 'Context::functionCall' which is not marked 'virtual' |
| warning | W200 | function 'functionCall' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCallWithValue' in 'ERC165' overrides 'Context::functionCallWithValue' which is not marked 'virtual' |
| warning | W200 | function 'functionCallWithValue' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionStaticCall' in 'ERC165' overrides 'Context::functionStaticCall' which is not marked 'virtual' |
| warning | W200 | function 'functionStaticCall' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionDelegateCall' in 'ERC165' overrides 'Context::functionDelegateCall' which is not marked 'virtual' |
| warning | W200 | function 'functionDelegateCall' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResultFromTarget' in 'ERC165' overrides 'Context::verifyCallResultFromTarget' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResultFromTarget' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResult' in 'ERC165' overrides 'Context::verifyCallResult' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResult' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function '_revert' in 'ERC165' overrides 'Context::_revert' which is not marked 'virtual' |
| warning | W200 | function '_revert' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sendValue' in 'AccessControl' overrides 'ERC165::sendValue' which is not marked 'virtual' |
| warning | W200 | function 'sendValue' in 'AccessControl' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCall' in 'AccessControl' overrides 'ERC165::functionCall' which is not marked 'virtual' |
| warning | W200 | function 'functionCall' in 'AccessControl' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCallWithValue' in 'AccessControl' overrides 'ERC165::functionCallWithValue' which is not marked 'virtual' |
| warning | W200 | function 'functionCallWithValue' in 'AccessControl' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionStaticCall' in 'AccessControl' overrides 'ERC165::functionStaticCall' which is not marked 'virtual' |
| warning | W200 | function 'functionStaticCall' in 'AccessControl' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionDelegateCall' in 'AccessControl' overrides 'ERC165::functionDelegateCall' which is not marked 'virtual' |
| warning | W200 | function 'functionDelegateCall' in 'AccessControl' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResultFromTarget' in 'AccessControl' overrides 'ERC165::verifyCallResultFromTarget' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResultFromTarget' in 'AccessControl' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResult' in 'AccessControl' overrides 'ERC165::verifyCallResult' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResult' in 'AccessControl' overrides a base function but is not marked 'override' |
| warning | W200 | function '_revert' in 'AccessControl' overrides 'ERC165::_revert' which is not marked 'virtual' |
| warning | W200 | function '_revert' in 'AccessControl' overrides a base function but is not marked 'override' |
| warning | W106 | function 'supportsInterface' (EIP-165) is unnecessary on Neo N3. Neo uses the manifest 'supportedstandards' array for interface detection, which the compiler populates automatically. |
| warning | W200 | function 'sendValue' in 'ERC1155Holder' overrides 'ERC165::sendValue' which is not marked 'virtual' |
| warning | W200 | function 'sendValue' in 'ERC1155Holder' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCall' in 'ERC1155Holder' overrides 'ERC165::functionCall' which is not marked 'virtual' |
| warning | W200 | function 'functionCall' in 'ERC1155Holder' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCallWithValue' in 'ERC1155Holder' overrides 'ERC165::functionCallWithValue' which is not marked 'virtual' |
| warning | W200 | function 'functionCallWithValue' in 'ERC1155Holder' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionStaticCall' in 'ERC1155Holder' overrides 'ERC165::functionStaticCall' which is not marked 'virtual' |
| warning | W200 | function 'functionStaticCall' in 'ERC1155Holder' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionDelegateCall' in 'ERC1155Holder' overrides 'ERC165::functionDelegateCall' which is not marked 'virtual' |
| warning | W200 | function 'functionDelegateCall' in 'ERC1155Holder' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResultFromTarget' in 'ERC1155Holder' overrides 'ERC165::verifyCallResultFromTarget' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResultFromTarget' in 'ERC1155Holder' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResult' in 'ERC1155Holder' overrides 'ERC165::verifyCallResult' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResult' in 'ERC1155Holder' overrides a base function but is not marked 'override' |
| warning | W200 | function '_revert' in 'ERC1155Holder' overrides 'ERC165::_revert' which is not marked 'virtual' |
| warning | W200 | function '_revert' in 'ERC1155Holder' overrides a base function but is not marked 'override' |
| warning | W111 | function 'onNEP17Payment' is marked `payable`, but Neo N3 has no native coin transfer; the modifier is accepted for compatibility but has no effect. Use onNEP17Payment(address, uint256, bytes) to handle incoming NEP-17 token payments. |
| warning | W111 | function 'execute' is marked `payable`, but Neo N3 has no native coin transfer; the modifier is accepted for compatibility but has no effect. Use onNEP17Payment(address, uint256, bytes) to handle incoming NEP-17 token payments. |
| warning | W111 | function 'executeBatch' is marked `payable`, but Neo N3 has no native coin transfer; the modifier is accepted for compatibility but has no effect. Use onNEP17Payment(address, uint256, bytes) to handle incoming NEP-17 token payments. |
| warning | W106 | function 'supportsInterface' (EIP-165) is unnecessary on Neo N3. Neo uses the manifest 'supportedstandards' array for interface detection, which the compiler populates automatically. |
| warning | W116 | function 'execute' has payable modifier which has no effect on Neo N3. Use onNEP17Payment callback to receive token payments. |
| warning | W116 | function 'executeBatch' has payable modifier which has no effect on Neo N3. Use onNEP17Payment callback to receive token payments. |
| warning | W200 | function 'sendValue' in 'ERC165' overrides 'Context::sendValue' which is not marked 'virtual' |
| warning | W200 | function 'sendValue' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCall' in 'ERC165' overrides 'Context::functionCall' which is not marked 'virtual' |
| warning | W200 | function 'functionCall' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCallWithValue' in 'ERC165' overrides 'Context::functionCallWithValue' which is not marked 'virtual' |
| warning | W200 | function 'functionCallWithValue' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionStaticCall' in 'ERC165' overrides 'Context::functionStaticCall' which is not marked 'virtual' |
| warning | W200 | function 'functionStaticCall' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionDelegateCall' in 'ERC165' overrides 'Context::functionDelegateCall' which is not marked 'virtual' |
| warning | W200 | function 'functionDelegateCall' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResultFromTarget' in 'ERC165' overrides 'Context::verifyCallResultFromTarget' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResultFromTarget' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResult' in 'ERC165' overrides 'Context::verifyCallResult' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResult' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function '_revert' in 'ERC165' overrides 'Context::_revert' which is not marked 'virtual' |
| warning | W200 | function '_revert' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sendValue' in 'AccessControl' overrides 'ERC165::sendValue' which is not marked 'virtual' |
| warning | W200 | function 'sendValue' in 'AccessControl' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCall' in 'AccessControl' overrides 'ERC165::functionCall' which is not marked 'virtual' |
| warning | W200 | function 'functionCall' in 'AccessControl' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCallWithValue' in 'AccessControl' overrides 'ERC165::functionCallWithValue' which is not marked 'virtual' |
| warning | W200 | function 'functionCallWithValue' in 'AccessControl' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionStaticCall' in 'AccessControl' overrides 'ERC165::functionStaticCall' which is not marked 'virtual' |
| warning | W200 | function 'functionStaticCall' in 'AccessControl' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionDelegateCall' in 'AccessControl' overrides 'ERC165::functionDelegateCall' which is not marked 'virtual' |
| warning | W200 | function 'functionDelegateCall' in 'AccessControl' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResultFromTarget' in 'AccessControl' overrides 'ERC165::verifyCallResultFromTarget' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResultFromTarget' in 'AccessControl' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResult' in 'AccessControl' overrides 'ERC165::verifyCallResult' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResult' in 'AccessControl' overrides a base function but is not marked 'override' |
| warning | W200 | function '_revert' in 'AccessControl' overrides 'ERC165::_revert' which is not marked 'virtual' |
| warning | W200 | function '_revert' in 'AccessControl' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sendValue' in 'ERC721Holder' overrides 'AccessControl::sendValue' which is not marked 'virtual' |
| warning | W200 | function 'sendValue' in 'ERC721Holder' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCall' in 'ERC721Holder' overrides 'AccessControl::functionCall' which is not marked 'virtual' |
| warning | W200 | function 'functionCall' in 'ERC721Holder' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCallWithValue' in 'ERC721Holder' overrides 'AccessControl::functionCallWithValue' which is not marked 'virtual' |
| warning | W200 | function 'functionCallWithValue' in 'ERC721Holder' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionStaticCall' in 'ERC721Holder' overrides 'AccessControl::functionStaticCall' which is not marked 'virtual' |
| warning | W200 | function 'functionStaticCall' in 'ERC721Holder' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionDelegateCall' in 'ERC721Holder' overrides 'AccessControl::functionDelegateCall' which is not marked 'virtual' |
| warning | W200 | function 'functionDelegateCall' in 'ERC721Holder' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResultFromTarget' in 'ERC721Holder' overrides 'AccessControl::verifyCallResultFromTarget' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResultFromTarget' in 'ERC721Holder' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResult' in 'ERC721Holder' overrides 'AccessControl::verifyCallResult' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResult' in 'ERC721Holder' overrides a base function but is not marked 'override' |
| warning | W200 | function '_revert' in 'ERC721Holder' overrides 'AccessControl::_revert' which is not marked 'virtual' |
| warning | W200 | function '_revert' in 'ERC721Holder' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sendValue' in 'ERC1155Holder' overrides 'ERC721Holder::sendValue' which is not marked 'virtual' |
| warning | W200 | function 'sendValue' in 'ERC1155Holder' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCall' in 'ERC1155Holder' overrides 'ERC721Holder::functionCall' which is not marked 'virtual' |
| warning | W200 | function 'functionCall' in 'ERC1155Holder' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCallWithValue' in 'ERC1155Holder' overrides 'ERC721Holder::functionCallWithValue' which is not marked 'virtual' |
| warning | W200 | function 'functionCallWithValue' in 'ERC1155Holder' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionStaticCall' in 'ERC1155Holder' overrides 'ERC721Holder::functionStaticCall' which is not marked 'virtual' |
| warning | W200 | function 'functionStaticCall' in 'ERC1155Holder' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionDelegateCall' in 'ERC1155Holder' overrides 'ERC721Holder::functionDelegateCall' which is not marked 'virtual' |
| warning | W200 | function 'functionDelegateCall' in 'ERC1155Holder' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResultFromTarget' in 'ERC1155Holder' overrides 'ERC721Holder::verifyCallResultFromTarget' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResultFromTarget' in 'ERC1155Holder' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResult' in 'ERC1155Holder' overrides 'ERC721Holder::verifyCallResult' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResult' in 'ERC1155Holder' overrides a base function but is not marked 'override' |
| warning | W200 | function '_revert' in 'ERC1155Holder' overrides 'ERC721Holder::_revert' which is not marked 'virtual' |
| warning | W200 | function '_revert' in 'ERC1155Holder' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sendValue' in 'TimelockController' overrides 'ERC1155Holder::sendValue' which is not marked 'virtual' |
| warning | W200 | function 'sendValue' in 'TimelockController' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCall' in 'TimelockController' overrides 'ERC1155Holder::functionCall' which is not marked 'virtual' |
| warning | W200 | function 'functionCall' in 'TimelockController' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionCallWithValue' in 'TimelockController' overrides 'ERC1155Holder::functionCallWithValue' which is not marked 'virtual' |
| warning | W200 | function 'functionCallWithValue' in 'TimelockController' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionStaticCall' in 'TimelockController' overrides 'ERC1155Holder::functionStaticCall' which is not marked 'virtual' |
| warning | W200 | function 'functionStaticCall' in 'TimelockController' overrides a base function but is not marked 'override' |
| warning | W200 | function 'functionDelegateCall' in 'TimelockController' overrides 'ERC1155Holder::functionDelegateCall' which is not marked 'virtual' |
| warning | W200 | function 'functionDelegateCall' in 'TimelockController' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResultFromTarget' in 'TimelockController' overrides 'ERC1155Holder::verifyCallResultFromTarget' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResultFromTarget' in 'TimelockController' overrides a base function but is not marked 'override' |
| warning | W200 | function 'verifyCallResult' in 'TimelockController' overrides 'ERC1155Holder::verifyCallResult' which is not marked 'virtual' |
| warning | W200 | function 'verifyCallResult' in 'TimelockController' overrides a base function but is not marked 'override' |
| warning | W200 | function '_revert' in 'TimelockController' overrides 'ERC1155Holder::_revert' which is not marked 'virtual' |
| warning | W200 | function '_revert' in 'TimelockController' overrides a base function but is not marked 'override' |

## References

- Global audit report: [Famous Contracts on NeoVM](/solidity/famous-contracts-neo-audit)
- Per-contract index: [Original Famous Contracts](/solidity/original-contracts/)
- Upstream contract path: `node_modules/@openzeppelin/contracts/governance/TimelockController.sol`