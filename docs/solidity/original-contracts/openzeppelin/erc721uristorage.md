# ERC721URIStorage (OpenZeppelin)

## Audit Snapshot

- Status: ✅ pass
- Source type: `npm`
- Source path: `third_party/famous-contracts/sources/@openzeppelin/contracts/token/ERC721/extensions/ERC721URIStorage.sol`
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
| error | RAW | [info][NEP-11] NEP-11 `Transfer` event has 3 parameter(s), expected 4. |
| error | RAW | [info][NEP-11] NEP-11 `Transfer` event has 3 parameter(s), expected 4. |
| warning | MANIFEST_WILDCARD_CONTRACT | contract 'Context' requires wildcard contract manifest permissions (contract='*') due to dynamic contract calls. This is riskier than fixed contract hashes; use --deny-wildcard-contracts to make this a hard error. |
| warning | W106 | function 'supportsInterface' (EIP-165) is unnecessary on Neo N3. Neo uses the manifest 'supportedstandards' array for interface detection, which the compiler populates automatically. |
| warning | MANIFEST_WILDCARD_CONTRACT | contract 'ERC165' requires wildcard contract manifest permissions (contract='*') due to dynamic contract calls. This is riskier than fixed contract hashes; use --deny-wildcard-contracts to make this a hard error. |
| warning | W121 | duplicate constant state variable 'GENERIC' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ASSERT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'UNDER_OVERFLOW' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'DIVISION_BY_ZERO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ENUM_CONVERSION_ERROR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'STORAGE_ENCODING_ERROR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'EMPTY_ARRAY_POP' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ARRAY_OUT_OF_BOUNDS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RESOURCE_ERROR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_INTERNAL_FUNCTION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'HEX_DIGITS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ADDRESS_LENGTH' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SPECIAL_CHARS_LOOKUP' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ABS_MIN_INT256' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'GENERIC' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ASSERT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'UNDER_OVERFLOW' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'DIVISION_BY_ZERO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ENUM_CONVERSION_ERROR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'STORAGE_ENCODING_ERROR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'EMPTY_ARRAY_POP' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ARRAY_OUT_OF_BOUNDS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RESOURCE_ERROR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_INTERNAL_FUNCTION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'HEX_DIGITS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ADDRESS_LENGTH' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SPECIAL_CHARS_LOOKUP' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ABS_MIN_INT256' detected while merging libraries |
| warning | W104 | function 'transferFrom' with 3 parameters (ERC-721 pattern) detected. NEP-11 uses transfer(to, tokenId, data) with 3 parameters instead. Authorization is via Runtime.checkWitness(owner), not msg.sender. |
| warning | W106 | function 'supportsInterface' (EIP-165) is unnecessary on Neo N3. Neo uses the manifest 'supportedstandards' array for interface detection, which the compiler populates automatically. |
| warning | W114 | NFT contract (has ownerOf) but missing onNEP11Payment callback. Other contracts cannot send NFTs to this contract. |
| warning | W200 | function 'checkOnERC721Received' in 'ERC165' overrides 'Context::checkOnERC721Received' which is not marked 'virtual' |
| warning | W200 | function 'checkOnERC721Received' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'panic' in 'ERC165' overrides 'Context::panic' which is not marked 'virtual' |
| warning | W200 | function 'panic' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint248' in 'ERC165' overrides 'Context::toUint248' which is not marked 'virtual' |
| warning | W200 | function 'toUint248' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint240' in 'ERC165' overrides 'Context::toUint240' which is not marked 'virtual' |
| warning | W200 | function 'toUint240' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint232' in 'ERC165' overrides 'Context::toUint232' which is not marked 'virtual' |
| warning | W200 | function 'toUint232' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint224' in 'ERC165' overrides 'Context::toUint224' which is not marked 'virtual' |
| warning | W200 | function 'toUint224' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint216' in 'ERC165' overrides 'Context::toUint216' which is not marked 'virtual' |
| warning | W200 | function 'toUint216' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint208' in 'ERC165' overrides 'Context::toUint208' which is not marked 'virtual' |
| warning | W200 | function 'toUint208' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint200' in 'ERC165' overrides 'Context::toUint200' which is not marked 'virtual' |
| warning | W200 | function 'toUint200' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint192' in 'ERC165' overrides 'Context::toUint192' which is not marked 'virtual' |
| warning | W200 | function 'toUint192' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint184' in 'ERC165' overrides 'Context::toUint184' which is not marked 'virtual' |
| warning | W200 | function 'toUint184' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint176' in 'ERC165' overrides 'Context::toUint176' which is not marked 'virtual' |
| warning | W200 | function 'toUint176' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint168' in 'ERC165' overrides 'Context::toUint168' which is not marked 'virtual' |
| warning | W200 | function 'toUint168' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint160' in 'ERC165' overrides 'Context::toUint160' which is not marked 'virtual' |
| warning | W200 | function 'toUint160' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint152' in 'ERC165' overrides 'Context::toUint152' which is not marked 'virtual' |
| warning | W200 | function 'toUint152' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint144' in 'ERC165' overrides 'Context::toUint144' which is not marked 'virtual' |
| warning | W200 | function 'toUint144' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint136' in 'ERC165' overrides 'Context::toUint136' which is not marked 'virtual' |
| warning | W200 | function 'toUint136' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'ERC165' overrides 'Context::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint120' in 'ERC165' overrides 'Context::toUint120' which is not marked 'virtual' |
| warning | W200 | function 'toUint120' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint112' in 'ERC165' overrides 'Context::toUint112' which is not marked 'virtual' |
| warning | W200 | function 'toUint112' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint104' in 'ERC165' overrides 'Context::toUint104' which is not marked 'virtual' |
| warning | W200 | function 'toUint104' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint96' in 'ERC165' overrides 'Context::toUint96' which is not marked 'virtual' |
| warning | W200 | function 'toUint96' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint88' in 'ERC165' overrides 'Context::toUint88' which is not marked 'virtual' |
| warning | W200 | function 'toUint88' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint80' in 'ERC165' overrides 'Context::toUint80' which is not marked 'virtual' |
| warning | W200 | function 'toUint80' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint72' in 'ERC165' overrides 'Context::toUint72' which is not marked 'virtual' |
| warning | W200 | function 'toUint72' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint64' in 'ERC165' overrides 'Context::toUint64' which is not marked 'virtual' |
| warning | W200 | function 'toUint64' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint56' in 'ERC165' overrides 'Context::toUint56' which is not marked 'virtual' |
| warning | W200 | function 'toUint56' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint48' in 'ERC165' overrides 'Context::toUint48' which is not marked 'virtual' |
| warning | W200 | function 'toUint48' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint40' in 'ERC165' overrides 'Context::toUint40' which is not marked 'virtual' |
| warning | W200 | function 'toUint40' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint32' in 'ERC165' overrides 'Context::toUint32' which is not marked 'virtual' |
| warning | W200 | function 'toUint32' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint24' in 'ERC165' overrides 'Context::toUint24' which is not marked 'virtual' |
| warning | W200 | function 'toUint24' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint16' in 'ERC165' overrides 'Context::toUint16' which is not marked 'virtual' |
| warning | W200 | function 'toUint16' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint8' in 'ERC165' overrides 'Context::toUint8' which is not marked 'virtual' |
| warning | W200 | function 'toUint8' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint256' in 'ERC165' overrides 'Context::toUint256' which is not marked 'virtual' |
| warning | W200 | function 'toUint256' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt248' in 'ERC165' overrides 'Context::toInt248' which is not marked 'virtual' |
| warning | W200 | function 'toInt248' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt240' in 'ERC165' overrides 'Context::toInt240' which is not marked 'virtual' |
| warning | W200 | function 'toInt240' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt232' in 'ERC165' overrides 'Context::toInt232' which is not marked 'virtual' |
| warning | W200 | function 'toInt232' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt224' in 'ERC165' overrides 'Context::toInt224' which is not marked 'virtual' |
| warning | W200 | function 'toInt224' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt216' in 'ERC165' overrides 'Context::toInt216' which is not marked 'virtual' |
| warning | W200 | function 'toInt216' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt208' in 'ERC165' overrides 'Context::toInt208' which is not marked 'virtual' |
| warning | W200 | function 'toInt208' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt200' in 'ERC165' overrides 'Context::toInt200' which is not marked 'virtual' |
| warning | W200 | function 'toInt200' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt192' in 'ERC165' overrides 'Context::toInt192' which is not marked 'virtual' |
| warning | W200 | function 'toInt192' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt184' in 'ERC165' overrides 'Context::toInt184' which is not marked 'virtual' |
| warning | W200 | function 'toInt184' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt176' in 'ERC165' overrides 'Context::toInt176' which is not marked 'virtual' |
| warning | W200 | function 'toInt176' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt168' in 'ERC165' overrides 'Context::toInt168' which is not marked 'virtual' |
| warning | W200 | function 'toInt168' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt160' in 'ERC165' overrides 'Context::toInt160' which is not marked 'virtual' |
| warning | W200 | function 'toInt160' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt152' in 'ERC165' overrides 'Context::toInt152' which is not marked 'virtual' |
| warning | W200 | function 'toInt152' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt144' in 'ERC165' overrides 'Context::toInt144' which is not marked 'virtual' |
| warning | W200 | function 'toInt144' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt136' in 'ERC165' overrides 'Context::toInt136' which is not marked 'virtual' |
| warning | W200 | function 'toInt136' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'ERC165' overrides 'Context::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt120' in 'ERC165' overrides 'Context::toInt120' which is not marked 'virtual' |
| warning | W200 | function 'toInt120' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt112' in 'ERC165' overrides 'Context::toInt112' which is not marked 'virtual' |
| warning | W200 | function 'toInt112' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt104' in 'ERC165' overrides 'Context::toInt104' which is not marked 'virtual' |
| warning | W200 | function 'toInt104' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt96' in 'ERC165' overrides 'Context::toInt96' which is not marked 'virtual' |
| warning | W200 | function 'toInt96' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt88' in 'ERC165' overrides 'Context::toInt88' which is not marked 'virtual' |
| warning | W200 | function 'toInt88' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt80' in 'ERC165' overrides 'Context::toInt80' which is not marked 'virtual' |
| warning | W200 | function 'toInt80' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt72' in 'ERC165' overrides 'Context::toInt72' which is not marked 'virtual' |
| warning | W200 | function 'toInt72' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt64' in 'ERC165' overrides 'Context::toInt64' which is not marked 'virtual' |
| warning | W200 | function 'toInt64' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt56' in 'ERC165' overrides 'Context::toInt56' which is not marked 'virtual' |
| warning | W200 | function 'toInt56' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt48' in 'ERC165' overrides 'Context::toInt48' which is not marked 'virtual' |
| warning | W200 | function 'toInt48' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt40' in 'ERC165' overrides 'Context::toInt40' which is not marked 'virtual' |
| warning | W200 | function 'toInt40' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt32' in 'ERC165' overrides 'Context::toInt32' which is not marked 'virtual' |
| warning | W200 | function 'toInt32' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt24' in 'ERC165' overrides 'Context::toInt24' which is not marked 'virtual' |
| warning | W200 | function 'toInt24' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt16' in 'ERC165' overrides 'Context::toInt16' which is not marked 'virtual' |
| warning | W200 | function 'toInt16' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt8' in 'ERC165' overrides 'Context::toInt8' which is not marked 'virtual' |
| warning | W200 | function 'toInt8' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt256' in 'ERC165' overrides 'Context::toInt256' which is not marked 'virtual' |
| warning | W200 | function 'toInt256' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint' in 'ERC165' overrides 'Context::toUint' which is not marked 'virtual' |
| warning | W200 | function 'toUint' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'add512' in 'ERC165' overrides 'Context::add512' which is not marked 'virtual' |
| warning | W200 | function 'add512' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mul512' in 'ERC165' overrides 'Context::mul512' which is not marked 'virtual' |
| warning | W200 | function 'mul512' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryAdd' in 'ERC165' overrides 'Context::tryAdd' which is not marked 'virtual' |
| warning | W200 | function 'tryAdd' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'trySub' in 'ERC165' overrides 'Context::trySub' which is not marked 'virtual' |
| warning | W200 | function 'trySub' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryMul' in 'ERC165' overrides 'Context::tryMul' which is not marked 'virtual' |
| warning | W200 | function 'tryMul' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryDiv' in 'ERC165' overrides 'Context::tryDiv' which is not marked 'virtual' |
| warning | W200 | function 'tryDiv' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryMod' in 'ERC165' overrides 'Context::tryMod' which is not marked 'virtual' |
| warning | W200 | function 'tryMod' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'saturatingAdd' in 'ERC165' overrides 'Context::saturatingAdd' which is not marked 'virtual' |
| warning | W200 | function 'saturatingAdd' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'saturatingSub' in 'ERC165' overrides 'Context::saturatingSub' which is not marked 'virtual' |
| warning | W200 | function 'saturatingSub' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'saturatingMul' in 'ERC165' overrides 'Context::saturatingMul' which is not marked 'virtual' |
| warning | W200 | function 'saturatingMul' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'ternary' in 'ERC165' overrides 'Context::ternary' which is not marked 'virtual' |
| warning | W200 | function 'ternary' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'max' in 'ERC165' overrides 'Context::max' which is not marked 'virtual' |
| warning | W200 | function 'max' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'min' in 'ERC165' overrides 'Context::min' which is not marked 'virtual' |
| warning | W200 | function 'min' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'average' in 'ERC165' overrides 'Context::average' which is not marked 'virtual' |
| warning | W200 | function 'average' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'ceilDiv' in 'ERC165' overrides 'Context::ceilDiv' which is not marked 'virtual' |
| warning | W200 | function 'ceilDiv' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDiv' in 'ERC165' overrides 'Context::mulDiv' which is not marked 'virtual' |
| warning | W200 | function 'mulDiv' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDiv' in 'ERC165' overrides 'Context::mulDiv' which is not marked 'virtual' |
| warning | W200 | function 'mulDiv' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulShr' in 'ERC165' overrides 'Context::mulShr' which is not marked 'virtual' |
| warning | W200 | function 'mulShr' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulShr' in 'ERC165' overrides 'Context::mulShr' which is not marked 'virtual' |
| warning | W200 | function 'mulShr' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'invMod' in 'ERC165' overrides 'Context::invMod' which is not marked 'virtual' |
| warning | W200 | function 'invMod' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'invModPrime' in 'ERC165' overrides 'Context::invModPrime' which is not marked 'virtual' |
| warning | W200 | function 'invModPrime' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'modExp' in 'ERC165' overrides 'Context::modExp' which is not marked 'virtual' |
| warning | W200 | function 'modExp' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryModExp' in 'ERC165' overrides 'Context::tryModExp' which is not marked 'virtual' |
| warning | W200 | function 'tryModExp' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'modExp' in 'ERC165' overrides 'Context::modExp' which is not marked 'virtual' |
| warning | W200 | function 'modExp' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryModExp' in 'ERC165' overrides 'Context::tryModExp' which is not marked 'virtual' |
| warning | W200 | function 'tryModExp' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function '_zeroBytes' in 'ERC165' overrides 'Context::_zeroBytes' which is not marked 'virtual' |
| warning | W200 | function '_zeroBytes' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sqrt' in 'ERC165' overrides 'Context::sqrt' which is not marked 'virtual' |
| warning | W200 | function 'sqrt' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sqrt' in 'ERC165' overrides 'Context::sqrt' which is not marked 'virtual' |
| warning | W200 | function 'sqrt' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log2' in 'ERC165' overrides 'Context::log2' which is not marked 'virtual' |
| warning | W200 | function 'log2' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log2' in 'ERC165' overrides 'Context::log2' which is not marked 'virtual' |
| warning | W200 | function 'log2' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log10' in 'ERC165' overrides 'Context::log10' which is not marked 'virtual' |
| warning | W200 | function 'log10' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log10' in 'ERC165' overrides 'Context::log10' which is not marked 'virtual' |
| warning | W200 | function 'log10' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log256' in 'ERC165' overrides 'Context::log256' which is not marked 'virtual' |
| warning | W200 | function 'log256' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log256' in 'ERC165' overrides 'Context::log256' which is not marked 'virtual' |
| warning | W200 | function 'log256' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'unsignedRoundsUp' in 'ERC165' overrides 'Context::unsignedRoundsUp' which is not marked 'virtual' |
| warning | W200 | function 'unsignedRoundsUp' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'ternary' in 'ERC165' overrides 'Context::ternary' which is not marked 'virtual' |
| warning | W200 | function 'ternary' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'max' in 'ERC165' overrides 'Context::max' which is not marked 'virtual' |
| warning | W200 | function 'max' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'min' in 'ERC165' overrides 'Context::min' which is not marked 'virtual' |
| warning | W200 | function 'min' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'average' in 'ERC165' overrides 'Context::average' which is not marked 'virtual' |
| warning | W200 | function 'average' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'abs' in 'ERC165' overrides 'Context::abs' which is not marked 'virtual' |
| warning | W200 | function 'abs' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toString' in 'ERC165' overrides 'Context::toString' which is not marked 'virtual' |
| warning | W200 | function 'toString' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toStringSigned' in 'ERC165' overrides 'Context::toStringSigned' which is not marked 'virtual' |
| warning | W200 | function 'toStringSigned' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toHexString' in 'ERC165' overrides 'Context::toHexString' which is not marked 'virtual' |
| warning | W200 | function 'toHexString' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toHexString' in 'ERC165' overrides 'Context::toHexString' which is not marked 'virtual' |
| warning | W200 | function 'toHexString' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toHexString' in 'ERC165' overrides 'Context::toHexString' which is not marked 'virtual' |
| warning | W200 | function 'toHexString' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toChecksumHexString' in 'ERC165' overrides 'Context::toChecksumHexString' which is not marked 'virtual' |
| warning | W200 | function 'toChecksumHexString' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'equal' in 'ERC165' overrides 'Context::equal' which is not marked 'virtual' |
| warning | W200 | function 'equal' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseUint' in 'ERC165' overrides 'Context::parseUint' which is not marked 'virtual' |
| warning | W200 | function 'parseUint' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseUint' in 'ERC165' overrides 'Context::parseUint' which is not marked 'virtual' |
| warning | W200 | function 'parseUint' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseUint' in 'ERC165' overrides 'Context::tryParseUint' which is not marked 'virtual' |
| warning | W200 | function 'tryParseUint' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseUint' in 'ERC165' overrides 'Context::tryParseUint' which is not marked 'virtual' |
| warning | W200 | function 'tryParseUint' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function '_tryParseUintUncheckedBounds' in 'ERC165' overrides 'Context::_tryParseUintUncheckedBounds' which is not marked 'virtual' |
| warning | W200 | function '_tryParseUintUncheckedBounds' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseInt' in 'ERC165' overrides 'Context::parseInt' which is not marked 'virtual' |
| warning | W200 | function 'parseInt' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseInt' in 'ERC165' overrides 'Context::parseInt' which is not marked 'virtual' |
| warning | W200 | function 'parseInt' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseInt' in 'ERC165' overrides 'Context::tryParseInt' which is not marked 'virtual' |
| warning | W200 | function 'tryParseInt' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseInt' in 'ERC165' overrides 'Context::tryParseInt' which is not marked 'virtual' |
| warning | W200 | function 'tryParseInt' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function '_tryParseIntUncheckedBounds' in 'ERC165' overrides 'Context::_tryParseIntUncheckedBounds' which is not marked 'virtual' |
| warning | W200 | function '_tryParseIntUncheckedBounds' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseHexUint' in 'ERC165' overrides 'Context::parseHexUint' which is not marked 'virtual' |
| warning | W200 | function 'parseHexUint' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseHexUint' in 'ERC165' overrides 'Context::parseHexUint' which is not marked 'virtual' |
| warning | W200 | function 'parseHexUint' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseHexUint' in 'ERC165' overrides 'Context::tryParseHexUint' which is not marked 'virtual' |
| warning | W200 | function 'tryParseHexUint' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseHexUint' in 'ERC165' overrides 'Context::tryParseHexUint' which is not marked 'virtual' |
| warning | W200 | function 'tryParseHexUint' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function '_tryParseHexUintUncheckedBounds' in 'ERC165' overrides 'Context::_tryParseHexUintUncheckedBounds' which is not marked 'virtual' |
| warning | W200 | function '_tryParseHexUintUncheckedBounds' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseAddress' in 'ERC165' overrides 'Context::parseAddress' which is not marked 'virtual' |
| warning | W200 | function 'parseAddress' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseAddress' in 'ERC165' overrides 'Context::parseAddress' which is not marked 'virtual' |
| warning | W200 | function 'parseAddress' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseAddress' in 'ERC165' overrides 'Context::tryParseAddress' which is not marked 'virtual' |
| warning | W200 | function 'tryParseAddress' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseAddress' in 'ERC165' overrides 'Context::tryParseAddress' which is not marked 'virtual' |
| warning | W200 | function 'tryParseAddress' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function '_tryParseChr' in 'ERC165' overrides 'Context::_tryParseChr' which is not marked 'virtual' |
| warning | W200 | function '_tryParseChr' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'escapeJSON' in 'ERC165' overrides 'Context::escapeJSON' which is not marked 'virtual' |
| warning | W200 | function 'escapeJSON' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function '_unsafeReadBytesOffset' in 'ERC165' overrides 'Context::_unsafeReadBytesOffset' which is not marked 'virtual' |
| warning | W200 | function '_unsafeReadBytesOffset' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'checkOnERC721Received' in 'ERC721' overrides 'ERC165::checkOnERC721Received' which is not marked 'virtual' |
| warning | W200 | function 'checkOnERC721Received' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'panic' in 'ERC721' overrides 'ERC165::panic' which is not marked 'virtual' |
| warning | W200 | function 'panic' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint248' in 'ERC721' overrides 'ERC165::toUint248' which is not marked 'virtual' |
| warning | W200 | function 'toUint248' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint240' in 'ERC721' overrides 'ERC165::toUint240' which is not marked 'virtual' |
| warning | W200 | function 'toUint240' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint232' in 'ERC721' overrides 'ERC165::toUint232' which is not marked 'virtual' |
| warning | W200 | function 'toUint232' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint224' in 'ERC721' overrides 'ERC165::toUint224' which is not marked 'virtual' |
| warning | W200 | function 'toUint224' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint216' in 'ERC721' overrides 'ERC165::toUint216' which is not marked 'virtual' |
| warning | W200 | function 'toUint216' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint208' in 'ERC721' overrides 'ERC165::toUint208' which is not marked 'virtual' |
| warning | W200 | function 'toUint208' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint200' in 'ERC721' overrides 'ERC165::toUint200' which is not marked 'virtual' |
| warning | W200 | function 'toUint200' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint192' in 'ERC721' overrides 'ERC165::toUint192' which is not marked 'virtual' |
| warning | W200 | function 'toUint192' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint184' in 'ERC721' overrides 'ERC165::toUint184' which is not marked 'virtual' |
| warning | W200 | function 'toUint184' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint176' in 'ERC721' overrides 'ERC165::toUint176' which is not marked 'virtual' |
| warning | W200 | function 'toUint176' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint168' in 'ERC721' overrides 'ERC165::toUint168' which is not marked 'virtual' |
| warning | W200 | function 'toUint168' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint160' in 'ERC721' overrides 'ERC165::toUint160' which is not marked 'virtual' |
| warning | W200 | function 'toUint160' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint152' in 'ERC721' overrides 'ERC165::toUint152' which is not marked 'virtual' |
| warning | W200 | function 'toUint152' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint144' in 'ERC721' overrides 'ERC165::toUint144' which is not marked 'virtual' |
| warning | W200 | function 'toUint144' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint136' in 'ERC721' overrides 'ERC165::toUint136' which is not marked 'virtual' |
| warning | W200 | function 'toUint136' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'ERC721' overrides 'ERC165::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint120' in 'ERC721' overrides 'ERC165::toUint120' which is not marked 'virtual' |
| warning | W200 | function 'toUint120' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint112' in 'ERC721' overrides 'ERC165::toUint112' which is not marked 'virtual' |
| warning | W200 | function 'toUint112' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint104' in 'ERC721' overrides 'ERC165::toUint104' which is not marked 'virtual' |
| warning | W200 | function 'toUint104' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint96' in 'ERC721' overrides 'ERC165::toUint96' which is not marked 'virtual' |
| warning | W200 | function 'toUint96' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint88' in 'ERC721' overrides 'ERC165::toUint88' which is not marked 'virtual' |
| warning | W200 | function 'toUint88' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint80' in 'ERC721' overrides 'ERC165::toUint80' which is not marked 'virtual' |
| warning | W200 | function 'toUint80' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint72' in 'ERC721' overrides 'ERC165::toUint72' which is not marked 'virtual' |
| warning | W200 | function 'toUint72' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint64' in 'ERC721' overrides 'ERC165::toUint64' which is not marked 'virtual' |
| warning | W200 | function 'toUint64' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint56' in 'ERC721' overrides 'ERC165::toUint56' which is not marked 'virtual' |
| warning | W200 | function 'toUint56' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint48' in 'ERC721' overrides 'ERC165::toUint48' which is not marked 'virtual' |
| warning | W200 | function 'toUint48' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint40' in 'ERC721' overrides 'ERC165::toUint40' which is not marked 'virtual' |
| warning | W200 | function 'toUint40' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint32' in 'ERC721' overrides 'ERC165::toUint32' which is not marked 'virtual' |
| warning | W200 | function 'toUint32' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint24' in 'ERC721' overrides 'ERC165::toUint24' which is not marked 'virtual' |
| warning | W200 | function 'toUint24' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint16' in 'ERC721' overrides 'ERC165::toUint16' which is not marked 'virtual' |
| warning | W200 | function 'toUint16' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint8' in 'ERC721' overrides 'ERC165::toUint8' which is not marked 'virtual' |
| warning | W200 | function 'toUint8' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint256' in 'ERC721' overrides 'ERC165::toUint256' which is not marked 'virtual' |
| warning | W200 | function 'toUint256' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt248' in 'ERC721' overrides 'ERC165::toInt248' which is not marked 'virtual' |
| warning | W200 | function 'toInt248' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt240' in 'ERC721' overrides 'ERC165::toInt240' which is not marked 'virtual' |
| warning | W200 | function 'toInt240' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt232' in 'ERC721' overrides 'ERC165::toInt232' which is not marked 'virtual' |
| warning | W200 | function 'toInt232' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt224' in 'ERC721' overrides 'ERC165::toInt224' which is not marked 'virtual' |
| warning | W200 | function 'toInt224' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt216' in 'ERC721' overrides 'ERC165::toInt216' which is not marked 'virtual' |
| warning | W200 | function 'toInt216' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt208' in 'ERC721' overrides 'ERC165::toInt208' which is not marked 'virtual' |
| warning | W200 | function 'toInt208' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt200' in 'ERC721' overrides 'ERC165::toInt200' which is not marked 'virtual' |
| warning | W200 | function 'toInt200' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt192' in 'ERC721' overrides 'ERC165::toInt192' which is not marked 'virtual' |
| warning | W200 | function 'toInt192' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt184' in 'ERC721' overrides 'ERC165::toInt184' which is not marked 'virtual' |
| warning | W200 | function 'toInt184' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt176' in 'ERC721' overrides 'ERC165::toInt176' which is not marked 'virtual' |
| warning | W200 | function 'toInt176' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt168' in 'ERC721' overrides 'ERC165::toInt168' which is not marked 'virtual' |
| warning | W200 | function 'toInt168' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt160' in 'ERC721' overrides 'ERC165::toInt160' which is not marked 'virtual' |
| warning | W200 | function 'toInt160' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt152' in 'ERC721' overrides 'ERC165::toInt152' which is not marked 'virtual' |
| warning | W200 | function 'toInt152' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt144' in 'ERC721' overrides 'ERC165::toInt144' which is not marked 'virtual' |
| warning | W200 | function 'toInt144' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt136' in 'ERC721' overrides 'ERC165::toInt136' which is not marked 'virtual' |
| warning | W200 | function 'toInt136' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'ERC721' overrides 'ERC165::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt120' in 'ERC721' overrides 'ERC165::toInt120' which is not marked 'virtual' |
| warning | W200 | function 'toInt120' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt112' in 'ERC721' overrides 'ERC165::toInt112' which is not marked 'virtual' |
| warning | W200 | function 'toInt112' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt104' in 'ERC721' overrides 'ERC165::toInt104' which is not marked 'virtual' |
| warning | W200 | function 'toInt104' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt96' in 'ERC721' overrides 'ERC165::toInt96' which is not marked 'virtual' |
| warning | W200 | function 'toInt96' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt88' in 'ERC721' overrides 'ERC165::toInt88' which is not marked 'virtual' |
| warning | W200 | function 'toInt88' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt80' in 'ERC721' overrides 'ERC165::toInt80' which is not marked 'virtual' |
| warning | W200 | function 'toInt80' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt72' in 'ERC721' overrides 'ERC165::toInt72' which is not marked 'virtual' |
| warning | W200 | function 'toInt72' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt64' in 'ERC721' overrides 'ERC165::toInt64' which is not marked 'virtual' |
| warning | W200 | function 'toInt64' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt56' in 'ERC721' overrides 'ERC165::toInt56' which is not marked 'virtual' |
| warning | W200 | function 'toInt56' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt48' in 'ERC721' overrides 'ERC165::toInt48' which is not marked 'virtual' |
| warning | W200 | function 'toInt48' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt40' in 'ERC721' overrides 'ERC165::toInt40' which is not marked 'virtual' |
| warning | W200 | function 'toInt40' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt32' in 'ERC721' overrides 'ERC165::toInt32' which is not marked 'virtual' |
| warning | W200 | function 'toInt32' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt24' in 'ERC721' overrides 'ERC165::toInt24' which is not marked 'virtual' |
| warning | W200 | function 'toInt24' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt16' in 'ERC721' overrides 'ERC165::toInt16' which is not marked 'virtual' |
| warning | W200 | function 'toInt16' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt8' in 'ERC721' overrides 'ERC165::toInt8' which is not marked 'virtual' |
| warning | W200 | function 'toInt8' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt256' in 'ERC721' overrides 'ERC165::toInt256' which is not marked 'virtual' |
| warning | W200 | function 'toInt256' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint' in 'ERC721' overrides 'ERC165::toUint' which is not marked 'virtual' |
| warning | W200 | function 'toUint' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'add512' in 'ERC721' overrides 'ERC165::add512' which is not marked 'virtual' |
| warning | W200 | function 'add512' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mul512' in 'ERC721' overrides 'ERC165::mul512' which is not marked 'virtual' |
| warning | W200 | function 'mul512' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryAdd' in 'ERC721' overrides 'ERC165::tryAdd' which is not marked 'virtual' |
| warning | W200 | function 'tryAdd' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'trySub' in 'ERC721' overrides 'ERC165::trySub' which is not marked 'virtual' |
| warning | W200 | function 'trySub' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryMul' in 'ERC721' overrides 'ERC165::tryMul' which is not marked 'virtual' |
| warning | W200 | function 'tryMul' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryDiv' in 'ERC721' overrides 'ERC165::tryDiv' which is not marked 'virtual' |
| warning | W200 | function 'tryDiv' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryMod' in 'ERC721' overrides 'ERC165::tryMod' which is not marked 'virtual' |
| warning | W200 | function 'tryMod' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'saturatingAdd' in 'ERC721' overrides 'ERC165::saturatingAdd' which is not marked 'virtual' |
| warning | W200 | function 'saturatingAdd' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'saturatingSub' in 'ERC721' overrides 'ERC165::saturatingSub' which is not marked 'virtual' |
| warning | W200 | function 'saturatingSub' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'saturatingMul' in 'ERC721' overrides 'ERC165::saturatingMul' which is not marked 'virtual' |
| warning | W200 | function 'saturatingMul' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'ternary' in 'ERC721' overrides 'ERC165::ternary' which is not marked 'virtual' |
| warning | W200 | function 'ternary' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'max' in 'ERC721' overrides 'ERC165::max' which is not marked 'virtual' |
| warning | W200 | function 'max' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'min' in 'ERC721' overrides 'ERC165::min' which is not marked 'virtual' |
| warning | W200 | function 'min' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'average' in 'ERC721' overrides 'ERC165::average' which is not marked 'virtual' |
| warning | W200 | function 'average' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'ceilDiv' in 'ERC721' overrides 'ERC165::ceilDiv' which is not marked 'virtual' |
| warning | W200 | function 'ceilDiv' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDiv' in 'ERC721' overrides 'ERC165::mulDiv' which is not marked 'virtual' |
| warning | W200 | function 'mulDiv' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDiv' in 'ERC721' overrides 'ERC165::mulDiv' which is not marked 'virtual' |
| warning | W200 | function 'mulDiv' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulShr' in 'ERC721' overrides 'ERC165::mulShr' which is not marked 'virtual' |
| warning | W200 | function 'mulShr' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulShr' in 'ERC721' overrides 'ERC165::mulShr' which is not marked 'virtual' |
| warning | W200 | function 'mulShr' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'invMod' in 'ERC721' overrides 'ERC165::invMod' which is not marked 'virtual' |
| warning | W200 | function 'invMod' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'invModPrime' in 'ERC721' overrides 'ERC165::invModPrime' which is not marked 'virtual' |
| warning | W200 | function 'invModPrime' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'modExp' in 'ERC721' overrides 'ERC165::modExp' which is not marked 'virtual' |
| warning | W200 | function 'modExp' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryModExp' in 'ERC721' overrides 'ERC165::tryModExp' which is not marked 'virtual' |
| warning | W200 | function 'tryModExp' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'modExp' in 'ERC721' overrides 'ERC165::modExp' which is not marked 'virtual' |
| warning | W200 | function 'modExp' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryModExp' in 'ERC721' overrides 'ERC165::tryModExp' which is not marked 'virtual' |
| warning | W200 | function 'tryModExp' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function '_zeroBytes' in 'ERC721' overrides 'ERC165::_zeroBytes' which is not marked 'virtual' |
| warning | W200 | function '_zeroBytes' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sqrt' in 'ERC721' overrides 'ERC165::sqrt' which is not marked 'virtual' |
| warning | W200 | function 'sqrt' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sqrt' in 'ERC721' overrides 'ERC165::sqrt' which is not marked 'virtual' |
| warning | W200 | function 'sqrt' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log2' in 'ERC721' overrides 'ERC165::log2' which is not marked 'virtual' |
| warning | W200 | function 'log2' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log2' in 'ERC721' overrides 'ERC165::log2' which is not marked 'virtual' |
| warning | W200 | function 'log2' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log10' in 'ERC721' overrides 'ERC165::log10' which is not marked 'virtual' |
| warning | W200 | function 'log10' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log10' in 'ERC721' overrides 'ERC165::log10' which is not marked 'virtual' |
| warning | W200 | function 'log10' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log256' in 'ERC721' overrides 'ERC165::log256' which is not marked 'virtual' |
| warning | W200 | function 'log256' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log256' in 'ERC721' overrides 'ERC165::log256' which is not marked 'virtual' |
| warning | W200 | function 'log256' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'unsignedRoundsUp' in 'ERC721' overrides 'ERC165::unsignedRoundsUp' which is not marked 'virtual' |
| warning | W200 | function 'unsignedRoundsUp' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'ternary' in 'ERC721' overrides 'ERC165::ternary' which is not marked 'virtual' |
| warning | W200 | function 'ternary' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'max' in 'ERC721' overrides 'ERC165::max' which is not marked 'virtual' |
| warning | W200 | function 'max' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'min' in 'ERC721' overrides 'ERC165::min' which is not marked 'virtual' |
| warning | W200 | function 'min' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'average' in 'ERC721' overrides 'ERC165::average' which is not marked 'virtual' |
| warning | W200 | function 'average' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'abs' in 'ERC721' overrides 'ERC165::abs' which is not marked 'virtual' |
| warning | W200 | function 'abs' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toString' in 'ERC721' overrides 'ERC165::toString' which is not marked 'virtual' |
| warning | W200 | function 'toString' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toStringSigned' in 'ERC721' overrides 'ERC165::toStringSigned' which is not marked 'virtual' |
| warning | W200 | function 'toStringSigned' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toHexString' in 'ERC721' overrides 'ERC165::toHexString' which is not marked 'virtual' |
| warning | W200 | function 'toHexString' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toHexString' in 'ERC721' overrides 'ERC165::toHexString' which is not marked 'virtual' |
| warning | W200 | function 'toHexString' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toHexString' in 'ERC721' overrides 'ERC165::toHexString' which is not marked 'virtual' |
| warning | W200 | function 'toHexString' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toChecksumHexString' in 'ERC721' overrides 'ERC165::toChecksumHexString' which is not marked 'virtual' |
| warning | W200 | function 'toChecksumHexString' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'equal' in 'ERC721' overrides 'ERC165::equal' which is not marked 'virtual' |
| warning | W200 | function 'equal' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseUint' in 'ERC721' overrides 'ERC165::parseUint' which is not marked 'virtual' |
| warning | W200 | function 'parseUint' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseUint' in 'ERC721' overrides 'ERC165::parseUint' which is not marked 'virtual' |
| warning | W200 | function 'parseUint' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseUint' in 'ERC721' overrides 'ERC165::tryParseUint' which is not marked 'virtual' |
| warning | W200 | function 'tryParseUint' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseUint' in 'ERC721' overrides 'ERC165::tryParseUint' which is not marked 'virtual' |
| warning | W200 | function 'tryParseUint' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function '_tryParseUintUncheckedBounds' in 'ERC721' overrides 'ERC165::_tryParseUintUncheckedBounds' which is not marked 'virtual' |
| warning | W200 | function '_tryParseUintUncheckedBounds' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseInt' in 'ERC721' overrides 'ERC165::parseInt' which is not marked 'virtual' |
| warning | W200 | function 'parseInt' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseInt' in 'ERC721' overrides 'ERC165::parseInt' which is not marked 'virtual' |
| warning | W200 | function 'parseInt' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseInt' in 'ERC721' overrides 'ERC165::tryParseInt' which is not marked 'virtual' |
| warning | W200 | function 'tryParseInt' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseInt' in 'ERC721' overrides 'ERC165::tryParseInt' which is not marked 'virtual' |
| warning | W200 | function 'tryParseInt' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function '_tryParseIntUncheckedBounds' in 'ERC721' overrides 'ERC165::_tryParseIntUncheckedBounds' which is not marked 'virtual' |
| warning | W200 | function '_tryParseIntUncheckedBounds' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseHexUint' in 'ERC721' overrides 'ERC165::parseHexUint' which is not marked 'virtual' |
| warning | W200 | function 'parseHexUint' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseHexUint' in 'ERC721' overrides 'ERC165::parseHexUint' which is not marked 'virtual' |
| warning | W200 | function 'parseHexUint' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseHexUint' in 'ERC721' overrides 'ERC165::tryParseHexUint' which is not marked 'virtual' |
| warning | W200 | function 'tryParseHexUint' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseHexUint' in 'ERC721' overrides 'ERC165::tryParseHexUint' which is not marked 'virtual' |
| warning | W200 | function 'tryParseHexUint' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function '_tryParseHexUintUncheckedBounds' in 'ERC721' overrides 'ERC165::_tryParseHexUintUncheckedBounds' which is not marked 'virtual' |
| warning | W200 | function '_tryParseHexUintUncheckedBounds' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseAddress' in 'ERC721' overrides 'ERC165::parseAddress' which is not marked 'virtual' |
| warning | W200 | function 'parseAddress' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseAddress' in 'ERC721' overrides 'ERC165::parseAddress' which is not marked 'virtual' |
| warning | W200 | function 'parseAddress' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseAddress' in 'ERC721' overrides 'ERC165::tryParseAddress' which is not marked 'virtual' |
| warning | W200 | function 'tryParseAddress' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseAddress' in 'ERC721' overrides 'ERC165::tryParseAddress' which is not marked 'virtual' |
| warning | W200 | function 'tryParseAddress' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function '_tryParseChr' in 'ERC721' overrides 'ERC165::_tryParseChr' which is not marked 'virtual' |
| warning | W200 | function '_tryParseChr' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'escapeJSON' in 'ERC721' overrides 'ERC165::escapeJSON' which is not marked 'virtual' |
| warning | W200 | function 'escapeJSON' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function '_unsafeReadBytesOffset' in 'ERC721' overrides 'ERC165::_unsafeReadBytesOffset' which is not marked 'virtual' |
| warning | W200 | function '_unsafeReadBytesOffset' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | MANIFEST_WILDCARD_CONTRACT | contract 'ERC721' requires wildcard contract manifest permissions (contract='*') due to dynamic contract calls. This is riskier than fixed contract hashes; use --deny-wildcard-contracts to make this a hard error. |
| warning | W121 | duplicate constant state variable 'GENERIC' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ASSERT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'UNDER_OVERFLOW' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'DIVISION_BY_ZERO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ENUM_CONVERSION_ERROR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'STORAGE_ENCODING_ERROR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'EMPTY_ARRAY_POP' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ARRAY_OUT_OF_BOUNDS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RESOURCE_ERROR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_INTERNAL_FUNCTION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'HEX_DIGITS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ADDRESS_LENGTH' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SPECIAL_CHARS_LOOKUP' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ABS_MIN_INT256' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'GENERIC' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ASSERT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'UNDER_OVERFLOW' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'DIVISION_BY_ZERO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ENUM_CONVERSION_ERROR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'STORAGE_ENCODING_ERROR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'EMPTY_ARRAY_POP' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ARRAY_OUT_OF_BOUNDS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RESOURCE_ERROR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_INTERNAL_FUNCTION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'HEX_DIGITS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ADDRESS_LENGTH' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SPECIAL_CHARS_LOOKUP' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ABS_MIN_INT256' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'GENERIC' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ASSERT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'UNDER_OVERFLOW' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'DIVISION_BY_ZERO' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ENUM_CONVERSION_ERROR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'STORAGE_ENCODING_ERROR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'EMPTY_ARRAY_POP' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ARRAY_OUT_OF_BOUNDS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'RESOURCE_ERROR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'INVALID_INTERNAL_FUNCTION' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'HEX_DIGITS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ADDRESS_LENGTH' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SPECIAL_CHARS_LOOKUP' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ABS_MIN_INT256' detected while merging libraries |
| warning | W104 | function 'transferFrom' with 3 parameters (ERC-721 pattern) detected. NEP-11 uses transfer(to, tokenId, data) with 3 parameters instead. Authorization is via Runtime.checkWitness(owner), not msg.sender. |
| warning | W106 | function 'supportsInterface' (EIP-165) is unnecessary on Neo N3. Neo uses the manifest 'supportedstandards' array for interface detection, which the compiler populates automatically. |
| warning | W114 | NFT contract (has ownerOf) but missing onNEP11Payment callback. Other contracts cannot send NFTs to this contract. |
| warning | W200 | function 'checkOnERC721Received' in 'ERC165' overrides 'Context::checkOnERC721Received' which is not marked 'virtual' |
| warning | W200 | function 'checkOnERC721Received' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'panic' in 'ERC165' overrides 'Context::panic' which is not marked 'virtual' |
| warning | W200 | function 'panic' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint248' in 'ERC165' overrides 'Context::toUint248' which is not marked 'virtual' |
| warning | W200 | function 'toUint248' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint240' in 'ERC165' overrides 'Context::toUint240' which is not marked 'virtual' |
| warning | W200 | function 'toUint240' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint232' in 'ERC165' overrides 'Context::toUint232' which is not marked 'virtual' |
| warning | W200 | function 'toUint232' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint224' in 'ERC165' overrides 'Context::toUint224' which is not marked 'virtual' |
| warning | W200 | function 'toUint224' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint216' in 'ERC165' overrides 'Context::toUint216' which is not marked 'virtual' |
| warning | W200 | function 'toUint216' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint208' in 'ERC165' overrides 'Context::toUint208' which is not marked 'virtual' |
| warning | W200 | function 'toUint208' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint200' in 'ERC165' overrides 'Context::toUint200' which is not marked 'virtual' |
| warning | W200 | function 'toUint200' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint192' in 'ERC165' overrides 'Context::toUint192' which is not marked 'virtual' |
| warning | W200 | function 'toUint192' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint184' in 'ERC165' overrides 'Context::toUint184' which is not marked 'virtual' |
| warning | W200 | function 'toUint184' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint176' in 'ERC165' overrides 'Context::toUint176' which is not marked 'virtual' |
| warning | W200 | function 'toUint176' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint168' in 'ERC165' overrides 'Context::toUint168' which is not marked 'virtual' |
| warning | W200 | function 'toUint168' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint160' in 'ERC165' overrides 'Context::toUint160' which is not marked 'virtual' |
| warning | W200 | function 'toUint160' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint152' in 'ERC165' overrides 'Context::toUint152' which is not marked 'virtual' |
| warning | W200 | function 'toUint152' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint144' in 'ERC165' overrides 'Context::toUint144' which is not marked 'virtual' |
| warning | W200 | function 'toUint144' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint136' in 'ERC165' overrides 'Context::toUint136' which is not marked 'virtual' |
| warning | W200 | function 'toUint136' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'ERC165' overrides 'Context::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint120' in 'ERC165' overrides 'Context::toUint120' which is not marked 'virtual' |
| warning | W200 | function 'toUint120' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint112' in 'ERC165' overrides 'Context::toUint112' which is not marked 'virtual' |
| warning | W200 | function 'toUint112' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint104' in 'ERC165' overrides 'Context::toUint104' which is not marked 'virtual' |
| warning | W200 | function 'toUint104' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint96' in 'ERC165' overrides 'Context::toUint96' which is not marked 'virtual' |
| warning | W200 | function 'toUint96' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint88' in 'ERC165' overrides 'Context::toUint88' which is not marked 'virtual' |
| warning | W200 | function 'toUint88' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint80' in 'ERC165' overrides 'Context::toUint80' which is not marked 'virtual' |
| warning | W200 | function 'toUint80' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint72' in 'ERC165' overrides 'Context::toUint72' which is not marked 'virtual' |
| warning | W200 | function 'toUint72' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint64' in 'ERC165' overrides 'Context::toUint64' which is not marked 'virtual' |
| warning | W200 | function 'toUint64' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint56' in 'ERC165' overrides 'Context::toUint56' which is not marked 'virtual' |
| warning | W200 | function 'toUint56' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint48' in 'ERC165' overrides 'Context::toUint48' which is not marked 'virtual' |
| warning | W200 | function 'toUint48' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint40' in 'ERC165' overrides 'Context::toUint40' which is not marked 'virtual' |
| warning | W200 | function 'toUint40' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint32' in 'ERC165' overrides 'Context::toUint32' which is not marked 'virtual' |
| warning | W200 | function 'toUint32' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint24' in 'ERC165' overrides 'Context::toUint24' which is not marked 'virtual' |
| warning | W200 | function 'toUint24' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint16' in 'ERC165' overrides 'Context::toUint16' which is not marked 'virtual' |
| warning | W200 | function 'toUint16' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint8' in 'ERC165' overrides 'Context::toUint8' which is not marked 'virtual' |
| warning | W200 | function 'toUint8' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint256' in 'ERC165' overrides 'Context::toUint256' which is not marked 'virtual' |
| warning | W200 | function 'toUint256' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt248' in 'ERC165' overrides 'Context::toInt248' which is not marked 'virtual' |
| warning | W200 | function 'toInt248' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt240' in 'ERC165' overrides 'Context::toInt240' which is not marked 'virtual' |
| warning | W200 | function 'toInt240' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt232' in 'ERC165' overrides 'Context::toInt232' which is not marked 'virtual' |
| warning | W200 | function 'toInt232' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt224' in 'ERC165' overrides 'Context::toInt224' which is not marked 'virtual' |
| warning | W200 | function 'toInt224' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt216' in 'ERC165' overrides 'Context::toInt216' which is not marked 'virtual' |
| warning | W200 | function 'toInt216' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt208' in 'ERC165' overrides 'Context::toInt208' which is not marked 'virtual' |
| warning | W200 | function 'toInt208' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt200' in 'ERC165' overrides 'Context::toInt200' which is not marked 'virtual' |
| warning | W200 | function 'toInt200' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt192' in 'ERC165' overrides 'Context::toInt192' which is not marked 'virtual' |
| warning | W200 | function 'toInt192' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt184' in 'ERC165' overrides 'Context::toInt184' which is not marked 'virtual' |
| warning | W200 | function 'toInt184' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt176' in 'ERC165' overrides 'Context::toInt176' which is not marked 'virtual' |
| warning | W200 | function 'toInt176' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt168' in 'ERC165' overrides 'Context::toInt168' which is not marked 'virtual' |
| warning | W200 | function 'toInt168' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt160' in 'ERC165' overrides 'Context::toInt160' which is not marked 'virtual' |
| warning | W200 | function 'toInt160' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt152' in 'ERC165' overrides 'Context::toInt152' which is not marked 'virtual' |
| warning | W200 | function 'toInt152' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt144' in 'ERC165' overrides 'Context::toInt144' which is not marked 'virtual' |
| warning | W200 | function 'toInt144' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt136' in 'ERC165' overrides 'Context::toInt136' which is not marked 'virtual' |
| warning | W200 | function 'toInt136' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'ERC165' overrides 'Context::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt120' in 'ERC165' overrides 'Context::toInt120' which is not marked 'virtual' |
| warning | W200 | function 'toInt120' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt112' in 'ERC165' overrides 'Context::toInt112' which is not marked 'virtual' |
| warning | W200 | function 'toInt112' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt104' in 'ERC165' overrides 'Context::toInt104' which is not marked 'virtual' |
| warning | W200 | function 'toInt104' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt96' in 'ERC165' overrides 'Context::toInt96' which is not marked 'virtual' |
| warning | W200 | function 'toInt96' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt88' in 'ERC165' overrides 'Context::toInt88' which is not marked 'virtual' |
| warning | W200 | function 'toInt88' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt80' in 'ERC165' overrides 'Context::toInt80' which is not marked 'virtual' |
| warning | W200 | function 'toInt80' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt72' in 'ERC165' overrides 'Context::toInt72' which is not marked 'virtual' |
| warning | W200 | function 'toInt72' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt64' in 'ERC165' overrides 'Context::toInt64' which is not marked 'virtual' |
| warning | W200 | function 'toInt64' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt56' in 'ERC165' overrides 'Context::toInt56' which is not marked 'virtual' |
| warning | W200 | function 'toInt56' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt48' in 'ERC165' overrides 'Context::toInt48' which is not marked 'virtual' |
| warning | W200 | function 'toInt48' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt40' in 'ERC165' overrides 'Context::toInt40' which is not marked 'virtual' |
| warning | W200 | function 'toInt40' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt32' in 'ERC165' overrides 'Context::toInt32' which is not marked 'virtual' |
| warning | W200 | function 'toInt32' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt24' in 'ERC165' overrides 'Context::toInt24' which is not marked 'virtual' |
| warning | W200 | function 'toInt24' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt16' in 'ERC165' overrides 'Context::toInt16' which is not marked 'virtual' |
| warning | W200 | function 'toInt16' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt8' in 'ERC165' overrides 'Context::toInt8' which is not marked 'virtual' |
| warning | W200 | function 'toInt8' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt256' in 'ERC165' overrides 'Context::toInt256' which is not marked 'virtual' |
| warning | W200 | function 'toInt256' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint' in 'ERC165' overrides 'Context::toUint' which is not marked 'virtual' |
| warning | W200 | function 'toUint' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'add512' in 'ERC165' overrides 'Context::add512' which is not marked 'virtual' |
| warning | W200 | function 'add512' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mul512' in 'ERC165' overrides 'Context::mul512' which is not marked 'virtual' |
| warning | W200 | function 'mul512' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryAdd' in 'ERC165' overrides 'Context::tryAdd' which is not marked 'virtual' |
| warning | W200 | function 'tryAdd' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'trySub' in 'ERC165' overrides 'Context::trySub' which is not marked 'virtual' |
| warning | W200 | function 'trySub' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryMul' in 'ERC165' overrides 'Context::tryMul' which is not marked 'virtual' |
| warning | W200 | function 'tryMul' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryDiv' in 'ERC165' overrides 'Context::tryDiv' which is not marked 'virtual' |
| warning | W200 | function 'tryDiv' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryMod' in 'ERC165' overrides 'Context::tryMod' which is not marked 'virtual' |
| warning | W200 | function 'tryMod' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'saturatingAdd' in 'ERC165' overrides 'Context::saturatingAdd' which is not marked 'virtual' |
| warning | W200 | function 'saturatingAdd' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'saturatingSub' in 'ERC165' overrides 'Context::saturatingSub' which is not marked 'virtual' |
| warning | W200 | function 'saturatingSub' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'saturatingMul' in 'ERC165' overrides 'Context::saturatingMul' which is not marked 'virtual' |
| warning | W200 | function 'saturatingMul' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'ternary' in 'ERC165' overrides 'Context::ternary' which is not marked 'virtual' |
| warning | W200 | function 'ternary' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'max' in 'ERC165' overrides 'Context::max' which is not marked 'virtual' |
| warning | W200 | function 'max' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'min' in 'ERC165' overrides 'Context::min' which is not marked 'virtual' |
| warning | W200 | function 'min' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'average' in 'ERC165' overrides 'Context::average' which is not marked 'virtual' |
| warning | W200 | function 'average' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'ceilDiv' in 'ERC165' overrides 'Context::ceilDiv' which is not marked 'virtual' |
| warning | W200 | function 'ceilDiv' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDiv' in 'ERC165' overrides 'Context::mulDiv' which is not marked 'virtual' |
| warning | W200 | function 'mulDiv' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDiv' in 'ERC165' overrides 'Context::mulDiv' which is not marked 'virtual' |
| warning | W200 | function 'mulDiv' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulShr' in 'ERC165' overrides 'Context::mulShr' which is not marked 'virtual' |
| warning | W200 | function 'mulShr' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulShr' in 'ERC165' overrides 'Context::mulShr' which is not marked 'virtual' |
| warning | W200 | function 'mulShr' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'invMod' in 'ERC165' overrides 'Context::invMod' which is not marked 'virtual' |
| warning | W200 | function 'invMod' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'invModPrime' in 'ERC165' overrides 'Context::invModPrime' which is not marked 'virtual' |
| warning | W200 | function 'invModPrime' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'modExp' in 'ERC165' overrides 'Context::modExp' which is not marked 'virtual' |
| warning | W200 | function 'modExp' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryModExp' in 'ERC165' overrides 'Context::tryModExp' which is not marked 'virtual' |
| warning | W200 | function 'tryModExp' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'modExp' in 'ERC165' overrides 'Context::modExp' which is not marked 'virtual' |
| warning | W200 | function 'modExp' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryModExp' in 'ERC165' overrides 'Context::tryModExp' which is not marked 'virtual' |
| warning | W200 | function 'tryModExp' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function '_zeroBytes' in 'ERC165' overrides 'Context::_zeroBytes' which is not marked 'virtual' |
| warning | W200 | function '_zeroBytes' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sqrt' in 'ERC165' overrides 'Context::sqrt' which is not marked 'virtual' |
| warning | W200 | function 'sqrt' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sqrt' in 'ERC165' overrides 'Context::sqrt' which is not marked 'virtual' |
| warning | W200 | function 'sqrt' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log2' in 'ERC165' overrides 'Context::log2' which is not marked 'virtual' |
| warning | W200 | function 'log2' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log2' in 'ERC165' overrides 'Context::log2' which is not marked 'virtual' |
| warning | W200 | function 'log2' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log10' in 'ERC165' overrides 'Context::log10' which is not marked 'virtual' |
| warning | W200 | function 'log10' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log10' in 'ERC165' overrides 'Context::log10' which is not marked 'virtual' |
| warning | W200 | function 'log10' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log256' in 'ERC165' overrides 'Context::log256' which is not marked 'virtual' |
| warning | W200 | function 'log256' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log256' in 'ERC165' overrides 'Context::log256' which is not marked 'virtual' |
| warning | W200 | function 'log256' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'unsignedRoundsUp' in 'ERC165' overrides 'Context::unsignedRoundsUp' which is not marked 'virtual' |
| warning | W200 | function 'unsignedRoundsUp' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'ternary' in 'ERC165' overrides 'Context::ternary' which is not marked 'virtual' |
| warning | W200 | function 'ternary' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'max' in 'ERC165' overrides 'Context::max' which is not marked 'virtual' |
| warning | W200 | function 'max' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'min' in 'ERC165' overrides 'Context::min' which is not marked 'virtual' |
| warning | W200 | function 'min' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'average' in 'ERC165' overrides 'Context::average' which is not marked 'virtual' |
| warning | W200 | function 'average' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'abs' in 'ERC165' overrides 'Context::abs' which is not marked 'virtual' |
| warning | W200 | function 'abs' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toString' in 'ERC165' overrides 'Context::toString' which is not marked 'virtual' |
| warning | W200 | function 'toString' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toStringSigned' in 'ERC165' overrides 'Context::toStringSigned' which is not marked 'virtual' |
| warning | W200 | function 'toStringSigned' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toHexString' in 'ERC165' overrides 'Context::toHexString' which is not marked 'virtual' |
| warning | W200 | function 'toHexString' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toHexString' in 'ERC165' overrides 'Context::toHexString' which is not marked 'virtual' |
| warning | W200 | function 'toHexString' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toHexString' in 'ERC165' overrides 'Context::toHexString' which is not marked 'virtual' |
| warning | W200 | function 'toHexString' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toChecksumHexString' in 'ERC165' overrides 'Context::toChecksumHexString' which is not marked 'virtual' |
| warning | W200 | function 'toChecksumHexString' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'equal' in 'ERC165' overrides 'Context::equal' which is not marked 'virtual' |
| warning | W200 | function 'equal' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseUint' in 'ERC165' overrides 'Context::parseUint' which is not marked 'virtual' |
| warning | W200 | function 'parseUint' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseUint' in 'ERC165' overrides 'Context::parseUint' which is not marked 'virtual' |
| warning | W200 | function 'parseUint' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseUint' in 'ERC165' overrides 'Context::tryParseUint' which is not marked 'virtual' |
| warning | W200 | function 'tryParseUint' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseUint' in 'ERC165' overrides 'Context::tryParseUint' which is not marked 'virtual' |
| warning | W200 | function 'tryParseUint' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function '_tryParseUintUncheckedBounds' in 'ERC165' overrides 'Context::_tryParseUintUncheckedBounds' which is not marked 'virtual' |
| warning | W200 | function '_tryParseUintUncheckedBounds' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseInt' in 'ERC165' overrides 'Context::parseInt' which is not marked 'virtual' |
| warning | W200 | function 'parseInt' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseInt' in 'ERC165' overrides 'Context::parseInt' which is not marked 'virtual' |
| warning | W200 | function 'parseInt' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseInt' in 'ERC165' overrides 'Context::tryParseInt' which is not marked 'virtual' |
| warning | W200 | function 'tryParseInt' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseInt' in 'ERC165' overrides 'Context::tryParseInt' which is not marked 'virtual' |
| warning | W200 | function 'tryParseInt' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function '_tryParseIntUncheckedBounds' in 'ERC165' overrides 'Context::_tryParseIntUncheckedBounds' which is not marked 'virtual' |
| warning | W200 | function '_tryParseIntUncheckedBounds' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseHexUint' in 'ERC165' overrides 'Context::parseHexUint' which is not marked 'virtual' |
| warning | W200 | function 'parseHexUint' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseHexUint' in 'ERC165' overrides 'Context::parseHexUint' which is not marked 'virtual' |
| warning | W200 | function 'parseHexUint' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseHexUint' in 'ERC165' overrides 'Context::tryParseHexUint' which is not marked 'virtual' |
| warning | W200 | function 'tryParseHexUint' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseHexUint' in 'ERC165' overrides 'Context::tryParseHexUint' which is not marked 'virtual' |
| warning | W200 | function 'tryParseHexUint' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function '_tryParseHexUintUncheckedBounds' in 'ERC165' overrides 'Context::_tryParseHexUintUncheckedBounds' which is not marked 'virtual' |
| warning | W200 | function '_tryParseHexUintUncheckedBounds' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseAddress' in 'ERC165' overrides 'Context::parseAddress' which is not marked 'virtual' |
| warning | W200 | function 'parseAddress' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseAddress' in 'ERC165' overrides 'Context::parseAddress' which is not marked 'virtual' |
| warning | W200 | function 'parseAddress' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseAddress' in 'ERC165' overrides 'Context::tryParseAddress' which is not marked 'virtual' |
| warning | W200 | function 'tryParseAddress' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseAddress' in 'ERC165' overrides 'Context::tryParseAddress' which is not marked 'virtual' |
| warning | W200 | function 'tryParseAddress' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function '_tryParseChr' in 'ERC165' overrides 'Context::_tryParseChr' which is not marked 'virtual' |
| warning | W200 | function '_tryParseChr' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'escapeJSON' in 'ERC165' overrides 'Context::escapeJSON' which is not marked 'virtual' |
| warning | W200 | function 'escapeJSON' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function '_unsafeReadBytesOffset' in 'ERC165' overrides 'Context::_unsafeReadBytesOffset' which is not marked 'virtual' |
| warning | W200 | function '_unsafeReadBytesOffset' in 'ERC165' overrides a base function but is not marked 'override' |
| warning | W200 | function 'checkOnERC721Received' in 'ERC721' overrides 'ERC165::checkOnERC721Received' which is not marked 'virtual' |
| warning | W200 | function 'checkOnERC721Received' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'panic' in 'ERC721' overrides 'ERC165::panic' which is not marked 'virtual' |
| warning | W200 | function 'panic' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint248' in 'ERC721' overrides 'ERC165::toUint248' which is not marked 'virtual' |
| warning | W200 | function 'toUint248' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint240' in 'ERC721' overrides 'ERC165::toUint240' which is not marked 'virtual' |
| warning | W200 | function 'toUint240' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint232' in 'ERC721' overrides 'ERC165::toUint232' which is not marked 'virtual' |
| warning | W200 | function 'toUint232' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint224' in 'ERC721' overrides 'ERC165::toUint224' which is not marked 'virtual' |
| warning | W200 | function 'toUint224' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint216' in 'ERC721' overrides 'ERC165::toUint216' which is not marked 'virtual' |
| warning | W200 | function 'toUint216' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint208' in 'ERC721' overrides 'ERC165::toUint208' which is not marked 'virtual' |
| warning | W200 | function 'toUint208' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint200' in 'ERC721' overrides 'ERC165::toUint200' which is not marked 'virtual' |
| warning | W200 | function 'toUint200' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint192' in 'ERC721' overrides 'ERC165::toUint192' which is not marked 'virtual' |
| warning | W200 | function 'toUint192' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint184' in 'ERC721' overrides 'ERC165::toUint184' which is not marked 'virtual' |
| warning | W200 | function 'toUint184' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint176' in 'ERC721' overrides 'ERC165::toUint176' which is not marked 'virtual' |
| warning | W200 | function 'toUint176' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint168' in 'ERC721' overrides 'ERC165::toUint168' which is not marked 'virtual' |
| warning | W200 | function 'toUint168' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint160' in 'ERC721' overrides 'ERC165::toUint160' which is not marked 'virtual' |
| warning | W200 | function 'toUint160' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint152' in 'ERC721' overrides 'ERC165::toUint152' which is not marked 'virtual' |
| warning | W200 | function 'toUint152' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint144' in 'ERC721' overrides 'ERC165::toUint144' which is not marked 'virtual' |
| warning | W200 | function 'toUint144' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint136' in 'ERC721' overrides 'ERC165::toUint136' which is not marked 'virtual' |
| warning | W200 | function 'toUint136' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'ERC721' overrides 'ERC165::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint120' in 'ERC721' overrides 'ERC165::toUint120' which is not marked 'virtual' |
| warning | W200 | function 'toUint120' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint112' in 'ERC721' overrides 'ERC165::toUint112' which is not marked 'virtual' |
| warning | W200 | function 'toUint112' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint104' in 'ERC721' overrides 'ERC165::toUint104' which is not marked 'virtual' |
| warning | W200 | function 'toUint104' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint96' in 'ERC721' overrides 'ERC165::toUint96' which is not marked 'virtual' |
| warning | W200 | function 'toUint96' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint88' in 'ERC721' overrides 'ERC165::toUint88' which is not marked 'virtual' |
| warning | W200 | function 'toUint88' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint80' in 'ERC721' overrides 'ERC165::toUint80' which is not marked 'virtual' |
| warning | W200 | function 'toUint80' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint72' in 'ERC721' overrides 'ERC165::toUint72' which is not marked 'virtual' |
| warning | W200 | function 'toUint72' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint64' in 'ERC721' overrides 'ERC165::toUint64' which is not marked 'virtual' |
| warning | W200 | function 'toUint64' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint56' in 'ERC721' overrides 'ERC165::toUint56' which is not marked 'virtual' |
| warning | W200 | function 'toUint56' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint48' in 'ERC721' overrides 'ERC165::toUint48' which is not marked 'virtual' |
| warning | W200 | function 'toUint48' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint40' in 'ERC721' overrides 'ERC165::toUint40' which is not marked 'virtual' |
| warning | W200 | function 'toUint40' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint32' in 'ERC721' overrides 'ERC165::toUint32' which is not marked 'virtual' |
| warning | W200 | function 'toUint32' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint24' in 'ERC721' overrides 'ERC165::toUint24' which is not marked 'virtual' |
| warning | W200 | function 'toUint24' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint16' in 'ERC721' overrides 'ERC165::toUint16' which is not marked 'virtual' |
| warning | W200 | function 'toUint16' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint8' in 'ERC721' overrides 'ERC165::toUint8' which is not marked 'virtual' |
| warning | W200 | function 'toUint8' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint256' in 'ERC721' overrides 'ERC165::toUint256' which is not marked 'virtual' |
| warning | W200 | function 'toUint256' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt248' in 'ERC721' overrides 'ERC165::toInt248' which is not marked 'virtual' |
| warning | W200 | function 'toInt248' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt240' in 'ERC721' overrides 'ERC165::toInt240' which is not marked 'virtual' |
| warning | W200 | function 'toInt240' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt232' in 'ERC721' overrides 'ERC165::toInt232' which is not marked 'virtual' |
| warning | W200 | function 'toInt232' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt224' in 'ERC721' overrides 'ERC165::toInt224' which is not marked 'virtual' |
| warning | W200 | function 'toInt224' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt216' in 'ERC721' overrides 'ERC165::toInt216' which is not marked 'virtual' |
| warning | W200 | function 'toInt216' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt208' in 'ERC721' overrides 'ERC165::toInt208' which is not marked 'virtual' |
| warning | W200 | function 'toInt208' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt200' in 'ERC721' overrides 'ERC165::toInt200' which is not marked 'virtual' |
| warning | W200 | function 'toInt200' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt192' in 'ERC721' overrides 'ERC165::toInt192' which is not marked 'virtual' |
| warning | W200 | function 'toInt192' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt184' in 'ERC721' overrides 'ERC165::toInt184' which is not marked 'virtual' |
| warning | W200 | function 'toInt184' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt176' in 'ERC721' overrides 'ERC165::toInt176' which is not marked 'virtual' |
| warning | W200 | function 'toInt176' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt168' in 'ERC721' overrides 'ERC165::toInt168' which is not marked 'virtual' |
| warning | W200 | function 'toInt168' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt160' in 'ERC721' overrides 'ERC165::toInt160' which is not marked 'virtual' |
| warning | W200 | function 'toInt160' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt152' in 'ERC721' overrides 'ERC165::toInt152' which is not marked 'virtual' |
| warning | W200 | function 'toInt152' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt144' in 'ERC721' overrides 'ERC165::toInt144' which is not marked 'virtual' |
| warning | W200 | function 'toInt144' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt136' in 'ERC721' overrides 'ERC165::toInt136' which is not marked 'virtual' |
| warning | W200 | function 'toInt136' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'ERC721' overrides 'ERC165::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt120' in 'ERC721' overrides 'ERC165::toInt120' which is not marked 'virtual' |
| warning | W200 | function 'toInt120' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt112' in 'ERC721' overrides 'ERC165::toInt112' which is not marked 'virtual' |
| warning | W200 | function 'toInt112' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt104' in 'ERC721' overrides 'ERC165::toInt104' which is not marked 'virtual' |
| warning | W200 | function 'toInt104' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt96' in 'ERC721' overrides 'ERC165::toInt96' which is not marked 'virtual' |
| warning | W200 | function 'toInt96' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt88' in 'ERC721' overrides 'ERC165::toInt88' which is not marked 'virtual' |
| warning | W200 | function 'toInt88' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt80' in 'ERC721' overrides 'ERC165::toInt80' which is not marked 'virtual' |
| warning | W200 | function 'toInt80' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt72' in 'ERC721' overrides 'ERC165::toInt72' which is not marked 'virtual' |
| warning | W200 | function 'toInt72' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt64' in 'ERC721' overrides 'ERC165::toInt64' which is not marked 'virtual' |
| warning | W200 | function 'toInt64' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt56' in 'ERC721' overrides 'ERC165::toInt56' which is not marked 'virtual' |
| warning | W200 | function 'toInt56' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt48' in 'ERC721' overrides 'ERC165::toInt48' which is not marked 'virtual' |
| warning | W200 | function 'toInt48' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt40' in 'ERC721' overrides 'ERC165::toInt40' which is not marked 'virtual' |
| warning | W200 | function 'toInt40' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt32' in 'ERC721' overrides 'ERC165::toInt32' which is not marked 'virtual' |
| warning | W200 | function 'toInt32' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt24' in 'ERC721' overrides 'ERC165::toInt24' which is not marked 'virtual' |
| warning | W200 | function 'toInt24' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt16' in 'ERC721' overrides 'ERC165::toInt16' which is not marked 'virtual' |
| warning | W200 | function 'toInt16' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt8' in 'ERC721' overrides 'ERC165::toInt8' which is not marked 'virtual' |
| warning | W200 | function 'toInt8' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt256' in 'ERC721' overrides 'ERC165::toInt256' which is not marked 'virtual' |
| warning | W200 | function 'toInt256' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint' in 'ERC721' overrides 'ERC165::toUint' which is not marked 'virtual' |
| warning | W200 | function 'toUint' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'add512' in 'ERC721' overrides 'ERC165::add512' which is not marked 'virtual' |
| warning | W200 | function 'add512' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mul512' in 'ERC721' overrides 'ERC165::mul512' which is not marked 'virtual' |
| warning | W200 | function 'mul512' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryAdd' in 'ERC721' overrides 'ERC165::tryAdd' which is not marked 'virtual' |
| warning | W200 | function 'tryAdd' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'trySub' in 'ERC721' overrides 'ERC165::trySub' which is not marked 'virtual' |
| warning | W200 | function 'trySub' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryMul' in 'ERC721' overrides 'ERC165::tryMul' which is not marked 'virtual' |
| warning | W200 | function 'tryMul' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryDiv' in 'ERC721' overrides 'ERC165::tryDiv' which is not marked 'virtual' |
| warning | W200 | function 'tryDiv' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryMod' in 'ERC721' overrides 'ERC165::tryMod' which is not marked 'virtual' |
| warning | W200 | function 'tryMod' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'saturatingAdd' in 'ERC721' overrides 'ERC165::saturatingAdd' which is not marked 'virtual' |
| warning | W200 | function 'saturatingAdd' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'saturatingSub' in 'ERC721' overrides 'ERC165::saturatingSub' which is not marked 'virtual' |
| warning | W200 | function 'saturatingSub' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'saturatingMul' in 'ERC721' overrides 'ERC165::saturatingMul' which is not marked 'virtual' |
| warning | W200 | function 'saturatingMul' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'ternary' in 'ERC721' overrides 'ERC165::ternary' which is not marked 'virtual' |
| warning | W200 | function 'ternary' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'max' in 'ERC721' overrides 'ERC165::max' which is not marked 'virtual' |
| warning | W200 | function 'max' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'min' in 'ERC721' overrides 'ERC165::min' which is not marked 'virtual' |
| warning | W200 | function 'min' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'average' in 'ERC721' overrides 'ERC165::average' which is not marked 'virtual' |
| warning | W200 | function 'average' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'ceilDiv' in 'ERC721' overrides 'ERC165::ceilDiv' which is not marked 'virtual' |
| warning | W200 | function 'ceilDiv' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDiv' in 'ERC721' overrides 'ERC165::mulDiv' which is not marked 'virtual' |
| warning | W200 | function 'mulDiv' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDiv' in 'ERC721' overrides 'ERC165::mulDiv' which is not marked 'virtual' |
| warning | W200 | function 'mulDiv' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulShr' in 'ERC721' overrides 'ERC165::mulShr' which is not marked 'virtual' |
| warning | W200 | function 'mulShr' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulShr' in 'ERC721' overrides 'ERC165::mulShr' which is not marked 'virtual' |
| warning | W200 | function 'mulShr' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'invMod' in 'ERC721' overrides 'ERC165::invMod' which is not marked 'virtual' |
| warning | W200 | function 'invMod' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'invModPrime' in 'ERC721' overrides 'ERC165::invModPrime' which is not marked 'virtual' |
| warning | W200 | function 'invModPrime' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'modExp' in 'ERC721' overrides 'ERC165::modExp' which is not marked 'virtual' |
| warning | W200 | function 'modExp' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryModExp' in 'ERC721' overrides 'ERC165::tryModExp' which is not marked 'virtual' |
| warning | W200 | function 'tryModExp' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'modExp' in 'ERC721' overrides 'ERC165::modExp' which is not marked 'virtual' |
| warning | W200 | function 'modExp' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryModExp' in 'ERC721' overrides 'ERC165::tryModExp' which is not marked 'virtual' |
| warning | W200 | function 'tryModExp' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function '_zeroBytes' in 'ERC721' overrides 'ERC165::_zeroBytes' which is not marked 'virtual' |
| warning | W200 | function '_zeroBytes' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sqrt' in 'ERC721' overrides 'ERC165::sqrt' which is not marked 'virtual' |
| warning | W200 | function 'sqrt' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sqrt' in 'ERC721' overrides 'ERC165::sqrt' which is not marked 'virtual' |
| warning | W200 | function 'sqrt' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log2' in 'ERC721' overrides 'ERC165::log2' which is not marked 'virtual' |
| warning | W200 | function 'log2' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log2' in 'ERC721' overrides 'ERC165::log2' which is not marked 'virtual' |
| warning | W200 | function 'log2' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log10' in 'ERC721' overrides 'ERC165::log10' which is not marked 'virtual' |
| warning | W200 | function 'log10' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log10' in 'ERC721' overrides 'ERC165::log10' which is not marked 'virtual' |
| warning | W200 | function 'log10' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log256' in 'ERC721' overrides 'ERC165::log256' which is not marked 'virtual' |
| warning | W200 | function 'log256' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log256' in 'ERC721' overrides 'ERC165::log256' which is not marked 'virtual' |
| warning | W200 | function 'log256' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'unsignedRoundsUp' in 'ERC721' overrides 'ERC165::unsignedRoundsUp' which is not marked 'virtual' |
| warning | W200 | function 'unsignedRoundsUp' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'ternary' in 'ERC721' overrides 'ERC165::ternary' which is not marked 'virtual' |
| warning | W200 | function 'ternary' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'max' in 'ERC721' overrides 'ERC165::max' which is not marked 'virtual' |
| warning | W200 | function 'max' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'min' in 'ERC721' overrides 'ERC165::min' which is not marked 'virtual' |
| warning | W200 | function 'min' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'average' in 'ERC721' overrides 'ERC165::average' which is not marked 'virtual' |
| warning | W200 | function 'average' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'abs' in 'ERC721' overrides 'ERC165::abs' which is not marked 'virtual' |
| warning | W200 | function 'abs' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toString' in 'ERC721' overrides 'ERC165::toString' which is not marked 'virtual' |
| warning | W200 | function 'toString' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toStringSigned' in 'ERC721' overrides 'ERC165::toStringSigned' which is not marked 'virtual' |
| warning | W200 | function 'toStringSigned' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toHexString' in 'ERC721' overrides 'ERC165::toHexString' which is not marked 'virtual' |
| warning | W200 | function 'toHexString' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toHexString' in 'ERC721' overrides 'ERC165::toHexString' which is not marked 'virtual' |
| warning | W200 | function 'toHexString' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toHexString' in 'ERC721' overrides 'ERC165::toHexString' which is not marked 'virtual' |
| warning | W200 | function 'toHexString' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toChecksumHexString' in 'ERC721' overrides 'ERC165::toChecksumHexString' which is not marked 'virtual' |
| warning | W200 | function 'toChecksumHexString' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'equal' in 'ERC721' overrides 'ERC165::equal' which is not marked 'virtual' |
| warning | W200 | function 'equal' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseUint' in 'ERC721' overrides 'ERC165::parseUint' which is not marked 'virtual' |
| warning | W200 | function 'parseUint' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseUint' in 'ERC721' overrides 'ERC165::parseUint' which is not marked 'virtual' |
| warning | W200 | function 'parseUint' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseUint' in 'ERC721' overrides 'ERC165::tryParseUint' which is not marked 'virtual' |
| warning | W200 | function 'tryParseUint' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseUint' in 'ERC721' overrides 'ERC165::tryParseUint' which is not marked 'virtual' |
| warning | W200 | function 'tryParseUint' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function '_tryParseUintUncheckedBounds' in 'ERC721' overrides 'ERC165::_tryParseUintUncheckedBounds' which is not marked 'virtual' |
| warning | W200 | function '_tryParseUintUncheckedBounds' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseInt' in 'ERC721' overrides 'ERC165::parseInt' which is not marked 'virtual' |
| warning | W200 | function 'parseInt' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseInt' in 'ERC721' overrides 'ERC165::parseInt' which is not marked 'virtual' |
| warning | W200 | function 'parseInt' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseInt' in 'ERC721' overrides 'ERC165::tryParseInt' which is not marked 'virtual' |
| warning | W200 | function 'tryParseInt' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseInt' in 'ERC721' overrides 'ERC165::tryParseInt' which is not marked 'virtual' |
| warning | W200 | function 'tryParseInt' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function '_tryParseIntUncheckedBounds' in 'ERC721' overrides 'ERC165::_tryParseIntUncheckedBounds' which is not marked 'virtual' |
| warning | W200 | function '_tryParseIntUncheckedBounds' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseHexUint' in 'ERC721' overrides 'ERC165::parseHexUint' which is not marked 'virtual' |
| warning | W200 | function 'parseHexUint' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseHexUint' in 'ERC721' overrides 'ERC165::parseHexUint' which is not marked 'virtual' |
| warning | W200 | function 'parseHexUint' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseHexUint' in 'ERC721' overrides 'ERC165::tryParseHexUint' which is not marked 'virtual' |
| warning | W200 | function 'tryParseHexUint' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseHexUint' in 'ERC721' overrides 'ERC165::tryParseHexUint' which is not marked 'virtual' |
| warning | W200 | function 'tryParseHexUint' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function '_tryParseHexUintUncheckedBounds' in 'ERC721' overrides 'ERC165::_tryParseHexUintUncheckedBounds' which is not marked 'virtual' |
| warning | W200 | function '_tryParseHexUintUncheckedBounds' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseAddress' in 'ERC721' overrides 'ERC165::parseAddress' which is not marked 'virtual' |
| warning | W200 | function 'parseAddress' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseAddress' in 'ERC721' overrides 'ERC165::parseAddress' which is not marked 'virtual' |
| warning | W200 | function 'parseAddress' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseAddress' in 'ERC721' overrides 'ERC165::tryParseAddress' which is not marked 'virtual' |
| warning | W200 | function 'tryParseAddress' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseAddress' in 'ERC721' overrides 'ERC165::tryParseAddress' which is not marked 'virtual' |
| warning | W200 | function 'tryParseAddress' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function '_tryParseChr' in 'ERC721' overrides 'ERC165::_tryParseChr' which is not marked 'virtual' |
| warning | W200 | function '_tryParseChr' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'escapeJSON' in 'ERC721' overrides 'ERC165::escapeJSON' which is not marked 'virtual' |
| warning | W200 | function 'escapeJSON' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function '_unsafeReadBytesOffset' in 'ERC721' overrides 'ERC165::_unsafeReadBytesOffset' which is not marked 'virtual' |
| warning | W200 | function '_unsafeReadBytesOffset' in 'ERC721' overrides a base function but is not marked 'override' |
| warning | W200 | function 'checkOnERC721Received' in 'ERC721URIStorage' overrides 'ERC721::checkOnERC721Received' which is not marked 'virtual' |
| warning | W200 | function 'checkOnERC721Received' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'panic' in 'ERC721URIStorage' overrides 'ERC721::panic' which is not marked 'virtual' |
| warning | W200 | function 'panic' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint248' in 'ERC721URIStorage' overrides 'ERC721::toUint248' which is not marked 'virtual' |
| warning | W200 | function 'toUint248' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint240' in 'ERC721URIStorage' overrides 'ERC721::toUint240' which is not marked 'virtual' |
| warning | W200 | function 'toUint240' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint232' in 'ERC721URIStorage' overrides 'ERC721::toUint232' which is not marked 'virtual' |
| warning | W200 | function 'toUint232' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint224' in 'ERC721URIStorage' overrides 'ERC721::toUint224' which is not marked 'virtual' |
| warning | W200 | function 'toUint224' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint216' in 'ERC721URIStorage' overrides 'ERC721::toUint216' which is not marked 'virtual' |
| warning | W200 | function 'toUint216' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint208' in 'ERC721URIStorage' overrides 'ERC721::toUint208' which is not marked 'virtual' |
| warning | W200 | function 'toUint208' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint200' in 'ERC721URIStorage' overrides 'ERC721::toUint200' which is not marked 'virtual' |
| warning | W200 | function 'toUint200' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint192' in 'ERC721URIStorage' overrides 'ERC721::toUint192' which is not marked 'virtual' |
| warning | W200 | function 'toUint192' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint184' in 'ERC721URIStorage' overrides 'ERC721::toUint184' which is not marked 'virtual' |
| warning | W200 | function 'toUint184' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint176' in 'ERC721URIStorage' overrides 'ERC721::toUint176' which is not marked 'virtual' |
| warning | W200 | function 'toUint176' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint168' in 'ERC721URIStorage' overrides 'ERC721::toUint168' which is not marked 'virtual' |
| warning | W200 | function 'toUint168' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint160' in 'ERC721URIStorage' overrides 'ERC721::toUint160' which is not marked 'virtual' |
| warning | W200 | function 'toUint160' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint152' in 'ERC721URIStorage' overrides 'ERC721::toUint152' which is not marked 'virtual' |
| warning | W200 | function 'toUint152' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint144' in 'ERC721URIStorage' overrides 'ERC721::toUint144' which is not marked 'virtual' |
| warning | W200 | function 'toUint144' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint136' in 'ERC721URIStorage' overrides 'ERC721::toUint136' which is not marked 'virtual' |
| warning | W200 | function 'toUint136' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'ERC721URIStorage' overrides 'ERC721::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint120' in 'ERC721URIStorage' overrides 'ERC721::toUint120' which is not marked 'virtual' |
| warning | W200 | function 'toUint120' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint112' in 'ERC721URIStorage' overrides 'ERC721::toUint112' which is not marked 'virtual' |
| warning | W200 | function 'toUint112' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint104' in 'ERC721URIStorage' overrides 'ERC721::toUint104' which is not marked 'virtual' |
| warning | W200 | function 'toUint104' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint96' in 'ERC721URIStorage' overrides 'ERC721::toUint96' which is not marked 'virtual' |
| warning | W200 | function 'toUint96' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint88' in 'ERC721URIStorage' overrides 'ERC721::toUint88' which is not marked 'virtual' |
| warning | W200 | function 'toUint88' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint80' in 'ERC721URIStorage' overrides 'ERC721::toUint80' which is not marked 'virtual' |
| warning | W200 | function 'toUint80' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint72' in 'ERC721URIStorage' overrides 'ERC721::toUint72' which is not marked 'virtual' |
| warning | W200 | function 'toUint72' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint64' in 'ERC721URIStorage' overrides 'ERC721::toUint64' which is not marked 'virtual' |
| warning | W200 | function 'toUint64' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint56' in 'ERC721URIStorage' overrides 'ERC721::toUint56' which is not marked 'virtual' |
| warning | W200 | function 'toUint56' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint48' in 'ERC721URIStorage' overrides 'ERC721::toUint48' which is not marked 'virtual' |
| warning | W200 | function 'toUint48' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint40' in 'ERC721URIStorage' overrides 'ERC721::toUint40' which is not marked 'virtual' |
| warning | W200 | function 'toUint40' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint32' in 'ERC721URIStorage' overrides 'ERC721::toUint32' which is not marked 'virtual' |
| warning | W200 | function 'toUint32' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint24' in 'ERC721URIStorage' overrides 'ERC721::toUint24' which is not marked 'virtual' |
| warning | W200 | function 'toUint24' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint16' in 'ERC721URIStorage' overrides 'ERC721::toUint16' which is not marked 'virtual' |
| warning | W200 | function 'toUint16' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint8' in 'ERC721URIStorage' overrides 'ERC721::toUint8' which is not marked 'virtual' |
| warning | W200 | function 'toUint8' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint256' in 'ERC721URIStorage' overrides 'ERC721::toUint256' which is not marked 'virtual' |
| warning | W200 | function 'toUint256' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt248' in 'ERC721URIStorage' overrides 'ERC721::toInt248' which is not marked 'virtual' |
| warning | W200 | function 'toInt248' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt240' in 'ERC721URIStorage' overrides 'ERC721::toInt240' which is not marked 'virtual' |
| warning | W200 | function 'toInt240' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt232' in 'ERC721URIStorage' overrides 'ERC721::toInt232' which is not marked 'virtual' |
| warning | W200 | function 'toInt232' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt224' in 'ERC721URIStorage' overrides 'ERC721::toInt224' which is not marked 'virtual' |
| warning | W200 | function 'toInt224' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt216' in 'ERC721URIStorage' overrides 'ERC721::toInt216' which is not marked 'virtual' |
| warning | W200 | function 'toInt216' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt208' in 'ERC721URIStorage' overrides 'ERC721::toInt208' which is not marked 'virtual' |
| warning | W200 | function 'toInt208' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt200' in 'ERC721URIStorage' overrides 'ERC721::toInt200' which is not marked 'virtual' |
| warning | W200 | function 'toInt200' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt192' in 'ERC721URIStorage' overrides 'ERC721::toInt192' which is not marked 'virtual' |
| warning | W200 | function 'toInt192' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt184' in 'ERC721URIStorage' overrides 'ERC721::toInt184' which is not marked 'virtual' |
| warning | W200 | function 'toInt184' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt176' in 'ERC721URIStorage' overrides 'ERC721::toInt176' which is not marked 'virtual' |
| warning | W200 | function 'toInt176' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt168' in 'ERC721URIStorage' overrides 'ERC721::toInt168' which is not marked 'virtual' |
| warning | W200 | function 'toInt168' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt160' in 'ERC721URIStorage' overrides 'ERC721::toInt160' which is not marked 'virtual' |
| warning | W200 | function 'toInt160' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt152' in 'ERC721URIStorage' overrides 'ERC721::toInt152' which is not marked 'virtual' |
| warning | W200 | function 'toInt152' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt144' in 'ERC721URIStorage' overrides 'ERC721::toInt144' which is not marked 'virtual' |
| warning | W200 | function 'toInt144' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt136' in 'ERC721URIStorage' overrides 'ERC721::toInt136' which is not marked 'virtual' |
| warning | W200 | function 'toInt136' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'ERC721URIStorage' overrides 'ERC721::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt120' in 'ERC721URIStorage' overrides 'ERC721::toInt120' which is not marked 'virtual' |
| warning | W200 | function 'toInt120' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt112' in 'ERC721URIStorage' overrides 'ERC721::toInt112' which is not marked 'virtual' |
| warning | W200 | function 'toInt112' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt104' in 'ERC721URIStorage' overrides 'ERC721::toInt104' which is not marked 'virtual' |
| warning | W200 | function 'toInt104' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt96' in 'ERC721URIStorage' overrides 'ERC721::toInt96' which is not marked 'virtual' |
| warning | W200 | function 'toInt96' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt88' in 'ERC721URIStorage' overrides 'ERC721::toInt88' which is not marked 'virtual' |
| warning | W200 | function 'toInt88' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt80' in 'ERC721URIStorage' overrides 'ERC721::toInt80' which is not marked 'virtual' |
| warning | W200 | function 'toInt80' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt72' in 'ERC721URIStorage' overrides 'ERC721::toInt72' which is not marked 'virtual' |
| warning | W200 | function 'toInt72' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt64' in 'ERC721URIStorage' overrides 'ERC721::toInt64' which is not marked 'virtual' |
| warning | W200 | function 'toInt64' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt56' in 'ERC721URIStorage' overrides 'ERC721::toInt56' which is not marked 'virtual' |
| warning | W200 | function 'toInt56' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt48' in 'ERC721URIStorage' overrides 'ERC721::toInt48' which is not marked 'virtual' |
| warning | W200 | function 'toInt48' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt40' in 'ERC721URIStorage' overrides 'ERC721::toInt40' which is not marked 'virtual' |
| warning | W200 | function 'toInt40' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt32' in 'ERC721URIStorage' overrides 'ERC721::toInt32' which is not marked 'virtual' |
| warning | W200 | function 'toInt32' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt24' in 'ERC721URIStorage' overrides 'ERC721::toInt24' which is not marked 'virtual' |
| warning | W200 | function 'toInt24' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt16' in 'ERC721URIStorage' overrides 'ERC721::toInt16' which is not marked 'virtual' |
| warning | W200 | function 'toInt16' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt8' in 'ERC721URIStorage' overrides 'ERC721::toInt8' which is not marked 'virtual' |
| warning | W200 | function 'toInt8' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt256' in 'ERC721URIStorage' overrides 'ERC721::toInt256' which is not marked 'virtual' |
| warning | W200 | function 'toInt256' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint' in 'ERC721URIStorage' overrides 'ERC721::toUint' which is not marked 'virtual' |
| warning | W200 | function 'toUint' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'add512' in 'ERC721URIStorage' overrides 'ERC721::add512' which is not marked 'virtual' |
| warning | W200 | function 'add512' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mul512' in 'ERC721URIStorage' overrides 'ERC721::mul512' which is not marked 'virtual' |
| warning | W200 | function 'mul512' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryAdd' in 'ERC721URIStorage' overrides 'ERC721::tryAdd' which is not marked 'virtual' |
| warning | W200 | function 'tryAdd' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'trySub' in 'ERC721URIStorage' overrides 'ERC721::trySub' which is not marked 'virtual' |
| warning | W200 | function 'trySub' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryMul' in 'ERC721URIStorage' overrides 'ERC721::tryMul' which is not marked 'virtual' |
| warning | W200 | function 'tryMul' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryDiv' in 'ERC721URIStorage' overrides 'ERC721::tryDiv' which is not marked 'virtual' |
| warning | W200 | function 'tryDiv' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryMod' in 'ERC721URIStorage' overrides 'ERC721::tryMod' which is not marked 'virtual' |
| warning | W200 | function 'tryMod' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'saturatingAdd' in 'ERC721URIStorage' overrides 'ERC721::saturatingAdd' which is not marked 'virtual' |
| warning | W200 | function 'saturatingAdd' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'saturatingSub' in 'ERC721URIStorage' overrides 'ERC721::saturatingSub' which is not marked 'virtual' |
| warning | W200 | function 'saturatingSub' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'saturatingMul' in 'ERC721URIStorage' overrides 'ERC721::saturatingMul' which is not marked 'virtual' |
| warning | W200 | function 'saturatingMul' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'ternary' in 'ERC721URIStorage' overrides 'ERC721::ternary' which is not marked 'virtual' |
| warning | W200 | function 'ternary' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'max' in 'ERC721URIStorage' overrides 'ERC721::max' which is not marked 'virtual' |
| warning | W200 | function 'max' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'min' in 'ERC721URIStorage' overrides 'ERC721::min' which is not marked 'virtual' |
| warning | W200 | function 'min' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'average' in 'ERC721URIStorage' overrides 'ERC721::average' which is not marked 'virtual' |
| warning | W200 | function 'average' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'ceilDiv' in 'ERC721URIStorage' overrides 'ERC721::ceilDiv' which is not marked 'virtual' |
| warning | W200 | function 'ceilDiv' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDiv' in 'ERC721URIStorage' overrides 'ERC721::mulDiv' which is not marked 'virtual' |
| warning | W200 | function 'mulDiv' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulDiv' in 'ERC721URIStorage' overrides 'ERC721::mulDiv' which is not marked 'virtual' |
| warning | W200 | function 'mulDiv' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulShr' in 'ERC721URIStorage' overrides 'ERC721::mulShr' which is not marked 'virtual' |
| warning | W200 | function 'mulShr' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'mulShr' in 'ERC721URIStorage' overrides 'ERC721::mulShr' which is not marked 'virtual' |
| warning | W200 | function 'mulShr' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'invMod' in 'ERC721URIStorage' overrides 'ERC721::invMod' which is not marked 'virtual' |
| warning | W200 | function 'invMod' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'invModPrime' in 'ERC721URIStorage' overrides 'ERC721::invModPrime' which is not marked 'virtual' |
| warning | W200 | function 'invModPrime' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'modExp' in 'ERC721URIStorage' overrides 'ERC721::modExp' which is not marked 'virtual' |
| warning | W200 | function 'modExp' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryModExp' in 'ERC721URIStorage' overrides 'ERC721::tryModExp' which is not marked 'virtual' |
| warning | W200 | function 'tryModExp' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'modExp' in 'ERC721URIStorage' overrides 'ERC721::modExp' which is not marked 'virtual' |
| warning | W200 | function 'modExp' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryModExp' in 'ERC721URIStorage' overrides 'ERC721::tryModExp' which is not marked 'virtual' |
| warning | W200 | function 'tryModExp' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function '_zeroBytes' in 'ERC721URIStorage' overrides 'ERC721::_zeroBytes' which is not marked 'virtual' |
| warning | W200 | function '_zeroBytes' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sqrt' in 'ERC721URIStorage' overrides 'ERC721::sqrt' which is not marked 'virtual' |
| warning | W200 | function 'sqrt' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'sqrt' in 'ERC721URIStorage' overrides 'ERC721::sqrt' which is not marked 'virtual' |
| warning | W200 | function 'sqrt' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log2' in 'ERC721URIStorage' overrides 'ERC721::log2' which is not marked 'virtual' |
| warning | W200 | function 'log2' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log2' in 'ERC721URIStorage' overrides 'ERC721::log2' which is not marked 'virtual' |
| warning | W200 | function 'log2' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log10' in 'ERC721URIStorage' overrides 'ERC721::log10' which is not marked 'virtual' |
| warning | W200 | function 'log10' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log10' in 'ERC721URIStorage' overrides 'ERC721::log10' which is not marked 'virtual' |
| warning | W200 | function 'log10' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log256' in 'ERC721URIStorage' overrides 'ERC721::log256' which is not marked 'virtual' |
| warning | W200 | function 'log256' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'log256' in 'ERC721URIStorage' overrides 'ERC721::log256' which is not marked 'virtual' |
| warning | W200 | function 'log256' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'unsignedRoundsUp' in 'ERC721URIStorage' overrides 'ERC721::unsignedRoundsUp' which is not marked 'virtual' |
| warning | W200 | function 'unsignedRoundsUp' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'ternary' in 'ERC721URIStorage' overrides 'ERC721::ternary' which is not marked 'virtual' |
| warning | W200 | function 'ternary' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'max' in 'ERC721URIStorage' overrides 'ERC721::max' which is not marked 'virtual' |
| warning | W200 | function 'max' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'min' in 'ERC721URIStorage' overrides 'ERC721::min' which is not marked 'virtual' |
| warning | W200 | function 'min' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'average' in 'ERC721URIStorage' overrides 'ERC721::average' which is not marked 'virtual' |
| warning | W200 | function 'average' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'abs' in 'ERC721URIStorage' overrides 'ERC721::abs' which is not marked 'virtual' |
| warning | W200 | function 'abs' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toString' in 'ERC721URIStorage' overrides 'ERC721::toString' which is not marked 'virtual' |
| warning | W200 | function 'toString' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toStringSigned' in 'ERC721URIStorage' overrides 'ERC721::toStringSigned' which is not marked 'virtual' |
| warning | W200 | function 'toStringSigned' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toHexString' in 'ERC721URIStorage' overrides 'ERC721::toHexString' which is not marked 'virtual' |
| warning | W200 | function 'toHexString' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toHexString' in 'ERC721URIStorage' overrides 'ERC721::toHexString' which is not marked 'virtual' |
| warning | W200 | function 'toHexString' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toHexString' in 'ERC721URIStorage' overrides 'ERC721::toHexString' which is not marked 'virtual' |
| warning | W200 | function 'toHexString' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toChecksumHexString' in 'ERC721URIStorage' overrides 'ERC721::toChecksumHexString' which is not marked 'virtual' |
| warning | W200 | function 'toChecksumHexString' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'equal' in 'ERC721URIStorage' overrides 'ERC721::equal' which is not marked 'virtual' |
| warning | W200 | function 'equal' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseUint' in 'ERC721URIStorage' overrides 'ERC721::parseUint' which is not marked 'virtual' |
| warning | W200 | function 'parseUint' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseUint' in 'ERC721URIStorage' overrides 'ERC721::parseUint' which is not marked 'virtual' |
| warning | W200 | function 'parseUint' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseUint' in 'ERC721URIStorage' overrides 'ERC721::tryParseUint' which is not marked 'virtual' |
| warning | W200 | function 'tryParseUint' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseUint' in 'ERC721URIStorage' overrides 'ERC721::tryParseUint' which is not marked 'virtual' |
| warning | W200 | function 'tryParseUint' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function '_tryParseUintUncheckedBounds' in 'ERC721URIStorage' overrides 'ERC721::_tryParseUintUncheckedBounds' which is not marked 'virtual' |
| warning | W200 | function '_tryParseUintUncheckedBounds' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseInt' in 'ERC721URIStorage' overrides 'ERC721::parseInt' which is not marked 'virtual' |
| warning | W200 | function 'parseInt' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseInt' in 'ERC721URIStorage' overrides 'ERC721::parseInt' which is not marked 'virtual' |
| warning | W200 | function 'parseInt' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseInt' in 'ERC721URIStorage' overrides 'ERC721::tryParseInt' which is not marked 'virtual' |
| warning | W200 | function 'tryParseInt' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseInt' in 'ERC721URIStorage' overrides 'ERC721::tryParseInt' which is not marked 'virtual' |
| warning | W200 | function 'tryParseInt' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function '_tryParseIntUncheckedBounds' in 'ERC721URIStorage' overrides 'ERC721::_tryParseIntUncheckedBounds' which is not marked 'virtual' |
| warning | W200 | function '_tryParseIntUncheckedBounds' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseHexUint' in 'ERC721URIStorage' overrides 'ERC721::parseHexUint' which is not marked 'virtual' |
| warning | W200 | function 'parseHexUint' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseHexUint' in 'ERC721URIStorage' overrides 'ERC721::parseHexUint' which is not marked 'virtual' |
| warning | W200 | function 'parseHexUint' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseHexUint' in 'ERC721URIStorage' overrides 'ERC721::tryParseHexUint' which is not marked 'virtual' |
| warning | W200 | function 'tryParseHexUint' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseHexUint' in 'ERC721URIStorage' overrides 'ERC721::tryParseHexUint' which is not marked 'virtual' |
| warning | W200 | function 'tryParseHexUint' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function '_tryParseHexUintUncheckedBounds' in 'ERC721URIStorage' overrides 'ERC721::_tryParseHexUintUncheckedBounds' which is not marked 'virtual' |
| warning | W200 | function '_tryParseHexUintUncheckedBounds' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseAddress' in 'ERC721URIStorage' overrides 'ERC721::parseAddress' which is not marked 'virtual' |
| warning | W200 | function 'parseAddress' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'parseAddress' in 'ERC721URIStorage' overrides 'ERC721::parseAddress' which is not marked 'virtual' |
| warning | W200 | function 'parseAddress' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseAddress' in 'ERC721URIStorage' overrides 'ERC721::tryParseAddress' which is not marked 'virtual' |
| warning | W200 | function 'tryParseAddress' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'tryParseAddress' in 'ERC721URIStorage' overrides 'ERC721::tryParseAddress' which is not marked 'virtual' |
| warning | W200 | function 'tryParseAddress' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function '_tryParseChr' in 'ERC721URIStorage' overrides 'ERC721::_tryParseChr' which is not marked 'virtual' |
| warning | W200 | function '_tryParseChr' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function 'escapeJSON' in 'ERC721URIStorage' overrides 'ERC721::escapeJSON' which is not marked 'virtual' |
| warning | W200 | function 'escapeJSON' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | W200 | function '_unsafeReadBytesOffset' in 'ERC721URIStorage' overrides 'ERC721::_unsafeReadBytesOffset' which is not marked 'virtual' |
| warning | W200 | function '_unsafeReadBytesOffset' in 'ERC721URIStorage' overrides a base function but is not marked 'override' |
| warning | MANIFEST_WILDCARD_CONTRACT | contract 'ERC721URIStorage' requires wildcard contract manifest permissions (contract='*') due to dynamic contract calls. This is riskier than fixed contract hashes; use --deny-wildcard-contracts to make this a hard error. |

## References

- Global audit report: [Famous Contracts on NeoVM](/solidity/famous-contracts-neo-audit)
- Per-contract index: [Original Famous Contracts](/solidity/original-contracts/)
- Upstream contract path: `third_party/famous-contracts/sources/@openzeppelin/contracts/token/ERC721/extensions/ERC721URIStorage.sol`