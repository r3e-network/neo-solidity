# FunctionsCoordinator_v1_3_0 (Chainlink)

## Audit Snapshot

- Status: ✅ pass
- Source type: `npm`
- Source path: `third_party/famous-contracts/sources/@chainlink/contracts/src/v0.8/functions/v1_3_0/FunctionsCoordinator.sol`
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
| warning | RAW | warning: block.basefee auto-mapped to Policy.getFeePerByte() on Neo N3. Neo uses a fixed fee structure, not EIP-1559 base fees. |
| warning | RAW | warning: block.basefee auto-mapped to Policy.getFeePerByte() on Neo N3. Neo uses a fixed fee structure, not EIP-1559 base fees. |
| warning | RAW | warning: tx.gasprice auto-mapped to Policy.getFeePerByte() on Neo N3. Neo fees are determined by script size and syscall costs. |
| warning | RAW | warning: tx.origin has different semantics on Neo N3. Neo uses multi-signature witnesses instead of a single origin. Consider using msg.sender or Runtime.CheckWitness() instead. |
| warning | RAW | warning: tx.gasprice auto-mapped to Policy.getFeePerByte() on Neo N3. Neo fees are determined by script size and syscall costs. |
| warning | RAW | warning: tx.gasprice auto-mapped to Policy.getFeePerByte() on Neo N3. Neo fees are determined by script size and syscall costs. |
| warning | RAW | warning: tx.gasprice auto-mapped to Policy.getFeePerByte() on Neo N3. Neo fees are determined by script size and syscall costs. |
| warning | RAW | warning: tx.origin has different semantics on Neo N3. Neo uses multi-signature witnesses instead of a single origin. Consider using msg.sender or Runtime.CheckWitness() instead. |
| warning | RAW | warning: tx.gasprice auto-mapped to Policy.getFeePerByte() on Neo N3. Neo fees are determined by script size and syscall costs. |
| warning | RAW | warning: tx.gasprice auto-mapped to Policy.getFeePerByte() on Neo N3. Neo fees are determined by script size and syscall costs. |
| warning | RAW | warning: tx.origin has different semantics on Neo N3. Neo uses multi-signature witnesses instead of a single origin. Consider using msg.sender or Runtime.CheckWitness() instead. |
| warning | W121 | duplicate constant state variable 'L2_TO_L1_MESSAGE_PASSER' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'L2_CROSS_DOMAIN_MESSENGER' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'L2_STANDARD_BRIDGE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'L2_ERC721_BRIDGE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SEQUENCER_FEE_WALLET' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OPTIMISM_MINTABLE_ERC20_FACTORY' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OPTIMISM_MINTABLE_ERC721_FACTORY' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'L1_BLOCK_ATTRIBUTES' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'GAS_PRICE_ORACLE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'L1_MESSAGE_SENDER' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'DEPLOYER_WHITELIST' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LEGACY_ERC20_ETH' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'L1_BLOCK_NUMBER' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LEGACY_MESSAGE_PASSER' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'PROXY_ADMIN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BASE_FEE_VAULT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'L1_FEE_VAULT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'GOVERNANCE_TOKEN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SCHEMA_REGISTRY' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'EAS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ARBGAS_ADDR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ARBGAS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ARB_MAINNET_CHAIN_ID' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ARB_GOERLI_TESTNET_CHAIN_ID' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ARB_SEPOLIA_TESTNET_CHAIN_ID' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'L1_FEE_DATA_PADDING' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OVM_GASPRICEORACLE_ADDR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OVM_GASPRICEORACLE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OP_MAINNET_CHAIN_ID' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OP_GOERLI_CHAIN_ID' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OP_SEPOLIA_CHAIN_ID' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BASE_MAINNET_CHAIN_ID' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BASE_GOERLI_CHAIN_ID' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BASE_SEPOLIA_CHAIN_ID' detected while merging libraries |
| warning | VALIDATION_WARNING | abstract contract 'FunctionsBilling' has 3 unimplemented function(s): [_getTransmitters, _onlyOwner, _owner] |
| warning | W200 | function 'toUint248' in 'FunctionsBilling' overrides 'Routable::toUint248' which is not marked 'virtual' |
| warning | W200 | function 'toUint248' in 'FunctionsBilling' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint240' in 'FunctionsBilling' overrides 'Routable::toUint240' which is not marked 'virtual' |
| warning | W200 | function 'toUint240' in 'FunctionsBilling' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint232' in 'FunctionsBilling' overrides 'Routable::toUint232' which is not marked 'virtual' |
| warning | W200 | function 'toUint232' in 'FunctionsBilling' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint224' in 'FunctionsBilling' overrides 'Routable::toUint224' which is not marked 'virtual' |
| warning | W200 | function 'toUint224' in 'FunctionsBilling' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint216' in 'FunctionsBilling' overrides 'Routable::toUint216' which is not marked 'virtual' |
| warning | W200 | function 'toUint216' in 'FunctionsBilling' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint208' in 'FunctionsBilling' overrides 'Routable::toUint208' which is not marked 'virtual' |
| warning | W200 | function 'toUint208' in 'FunctionsBilling' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint200' in 'FunctionsBilling' overrides 'Routable::toUint200' which is not marked 'virtual' |
| warning | W200 | function 'toUint200' in 'FunctionsBilling' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint192' in 'FunctionsBilling' overrides 'Routable::toUint192' which is not marked 'virtual' |
| warning | W200 | function 'toUint192' in 'FunctionsBilling' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint184' in 'FunctionsBilling' overrides 'Routable::toUint184' which is not marked 'virtual' |
| warning | W200 | function 'toUint184' in 'FunctionsBilling' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint176' in 'FunctionsBilling' overrides 'Routable::toUint176' which is not marked 'virtual' |
| warning | W200 | function 'toUint176' in 'FunctionsBilling' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint168' in 'FunctionsBilling' overrides 'Routable::toUint168' which is not marked 'virtual' |
| warning | W200 | function 'toUint168' in 'FunctionsBilling' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint160' in 'FunctionsBilling' overrides 'Routable::toUint160' which is not marked 'virtual' |
| warning | W200 | function 'toUint160' in 'FunctionsBilling' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint152' in 'FunctionsBilling' overrides 'Routable::toUint152' which is not marked 'virtual' |
| warning | W200 | function 'toUint152' in 'FunctionsBilling' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint144' in 'FunctionsBilling' overrides 'Routable::toUint144' which is not marked 'virtual' |
| warning | W200 | function 'toUint144' in 'FunctionsBilling' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint136' in 'FunctionsBilling' overrides 'Routable::toUint136' which is not marked 'virtual' |
| warning | W200 | function 'toUint136' in 'FunctionsBilling' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'FunctionsBilling' overrides 'Routable::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'FunctionsBilling' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint120' in 'FunctionsBilling' overrides 'Routable::toUint120' which is not marked 'virtual' |
| warning | W200 | function 'toUint120' in 'FunctionsBilling' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint112' in 'FunctionsBilling' overrides 'Routable::toUint112' which is not marked 'virtual' |
| warning | W200 | function 'toUint112' in 'FunctionsBilling' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint104' in 'FunctionsBilling' overrides 'Routable::toUint104' which is not marked 'virtual' |
| warning | W200 | function 'toUint104' in 'FunctionsBilling' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint96' in 'FunctionsBilling' overrides 'Routable::toUint96' which is not marked 'virtual' |
| warning | W200 | function 'toUint96' in 'FunctionsBilling' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint88' in 'FunctionsBilling' overrides 'Routable::toUint88' which is not marked 'virtual' |
| warning | W200 | function 'toUint88' in 'FunctionsBilling' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint80' in 'FunctionsBilling' overrides 'Routable::toUint80' which is not marked 'virtual' |
| warning | W200 | function 'toUint80' in 'FunctionsBilling' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint72' in 'FunctionsBilling' overrides 'Routable::toUint72' which is not marked 'virtual' |
| warning | W200 | function 'toUint72' in 'FunctionsBilling' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint64' in 'FunctionsBilling' overrides 'Routable::toUint64' which is not marked 'virtual' |
| warning | W200 | function 'toUint64' in 'FunctionsBilling' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint56' in 'FunctionsBilling' overrides 'Routable::toUint56' which is not marked 'virtual' |
| warning | W200 | function 'toUint56' in 'FunctionsBilling' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint48' in 'FunctionsBilling' overrides 'Routable::toUint48' which is not marked 'virtual' |
| warning | W200 | function 'toUint48' in 'FunctionsBilling' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint40' in 'FunctionsBilling' overrides 'Routable::toUint40' which is not marked 'virtual' |
| warning | W200 | function 'toUint40' in 'FunctionsBilling' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint32' in 'FunctionsBilling' overrides 'Routable::toUint32' which is not marked 'virtual' |
| warning | W200 | function 'toUint32' in 'FunctionsBilling' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint24' in 'FunctionsBilling' overrides 'Routable::toUint24' which is not marked 'virtual' |
| warning | W200 | function 'toUint24' in 'FunctionsBilling' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint16' in 'FunctionsBilling' overrides 'Routable::toUint16' which is not marked 'virtual' |
| warning | W200 | function 'toUint16' in 'FunctionsBilling' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint8' in 'FunctionsBilling' overrides 'Routable::toUint8' which is not marked 'virtual' |
| warning | W200 | function 'toUint8' in 'FunctionsBilling' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint256' in 'FunctionsBilling' overrides 'Routable::toUint256' which is not marked 'virtual' |
| warning | W200 | function 'toUint256' in 'FunctionsBilling' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt248' in 'FunctionsBilling' overrides 'Routable::toInt248' which is not marked 'virtual' |
| warning | W200 | function 'toInt248' in 'FunctionsBilling' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt240' in 'FunctionsBilling' overrides 'Routable::toInt240' which is not marked 'virtual' |
| warning | W200 | function 'toInt240' in 'FunctionsBilling' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt232' in 'FunctionsBilling' overrides 'Routable::toInt232' which is not marked 'virtual' |
| warning | W200 | function 'toInt232' in 'FunctionsBilling' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt224' in 'FunctionsBilling' overrides 'Routable::toInt224' which is not marked 'virtual' |
| warning | W200 | function 'toInt224' in 'FunctionsBilling' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt216' in 'FunctionsBilling' overrides 'Routable::toInt216' which is not marked 'virtual' |
| warning | W200 | function 'toInt216' in 'FunctionsBilling' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt208' in 'FunctionsBilling' overrides 'Routable::toInt208' which is not marked 'virtual' |
| warning | W200 | function 'toInt208' in 'FunctionsBilling' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt200' in 'FunctionsBilling' overrides 'Routable::toInt200' which is not marked 'virtual' |
| warning | W200 | function 'toInt200' in 'FunctionsBilling' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt192' in 'FunctionsBilling' overrides 'Routable::toInt192' which is not marked 'virtual' |
| warning | W200 | function 'toInt192' in 'FunctionsBilling' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt184' in 'FunctionsBilling' overrides 'Routable::toInt184' which is not marked 'virtual' |
| warning | W200 | function 'toInt184' in 'FunctionsBilling' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt176' in 'FunctionsBilling' overrides 'Routable::toInt176' which is not marked 'virtual' |
| warning | W200 | function 'toInt176' in 'FunctionsBilling' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt168' in 'FunctionsBilling' overrides 'Routable::toInt168' which is not marked 'virtual' |
| warning | W200 | function 'toInt168' in 'FunctionsBilling' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt160' in 'FunctionsBilling' overrides 'Routable::toInt160' which is not marked 'virtual' |
| warning | W200 | function 'toInt160' in 'FunctionsBilling' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt152' in 'FunctionsBilling' overrides 'Routable::toInt152' which is not marked 'virtual' |
| warning | W200 | function 'toInt152' in 'FunctionsBilling' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt144' in 'FunctionsBilling' overrides 'Routable::toInt144' which is not marked 'virtual' |
| warning | W200 | function 'toInt144' in 'FunctionsBilling' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt136' in 'FunctionsBilling' overrides 'Routable::toInt136' which is not marked 'virtual' |
| warning | W200 | function 'toInt136' in 'FunctionsBilling' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'FunctionsBilling' overrides 'Routable::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'FunctionsBilling' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt120' in 'FunctionsBilling' overrides 'Routable::toInt120' which is not marked 'virtual' |
| warning | W200 | function 'toInt120' in 'FunctionsBilling' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt112' in 'FunctionsBilling' overrides 'Routable::toInt112' which is not marked 'virtual' |
| warning | W200 | function 'toInt112' in 'FunctionsBilling' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt104' in 'FunctionsBilling' overrides 'Routable::toInt104' which is not marked 'virtual' |
| warning | W200 | function 'toInt104' in 'FunctionsBilling' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt96' in 'FunctionsBilling' overrides 'Routable::toInt96' which is not marked 'virtual' |
| warning | W200 | function 'toInt96' in 'FunctionsBilling' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt88' in 'FunctionsBilling' overrides 'Routable::toInt88' which is not marked 'virtual' |
| warning | W200 | function 'toInt88' in 'FunctionsBilling' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt80' in 'FunctionsBilling' overrides 'Routable::toInt80' which is not marked 'virtual' |
| warning | W200 | function 'toInt80' in 'FunctionsBilling' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt72' in 'FunctionsBilling' overrides 'Routable::toInt72' which is not marked 'virtual' |
| warning | W200 | function 'toInt72' in 'FunctionsBilling' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt64' in 'FunctionsBilling' overrides 'Routable::toInt64' which is not marked 'virtual' |
| warning | W200 | function 'toInt64' in 'FunctionsBilling' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt56' in 'FunctionsBilling' overrides 'Routable::toInt56' which is not marked 'virtual' |
| warning | W200 | function 'toInt56' in 'FunctionsBilling' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt48' in 'FunctionsBilling' overrides 'Routable::toInt48' which is not marked 'virtual' |
| warning | W200 | function 'toInt48' in 'FunctionsBilling' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt40' in 'FunctionsBilling' overrides 'Routable::toInt40' which is not marked 'virtual' |
| warning | W200 | function 'toInt40' in 'FunctionsBilling' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt32' in 'FunctionsBilling' overrides 'Routable::toInt32' which is not marked 'virtual' |
| warning | W200 | function 'toInt32' in 'FunctionsBilling' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt24' in 'FunctionsBilling' overrides 'Routable::toInt24' which is not marked 'virtual' |
| warning | W200 | function 'toInt24' in 'FunctionsBilling' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt16' in 'FunctionsBilling' overrides 'Routable::toInt16' which is not marked 'virtual' |
| warning | W200 | function 'toInt16' in 'FunctionsBilling' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt8' in 'FunctionsBilling' overrides 'Routable::toInt8' which is not marked 'virtual' |
| warning | W200 | function 'toInt8' in 'FunctionsBilling' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt256' in 'FunctionsBilling' overrides 'Routable::toInt256' which is not marked 'virtual' |
| warning | W200 | function 'toInt256' in 'FunctionsBilling' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getCurrentTxL1GasFees' in 'FunctionsBilling' overrides 'Routable::_getCurrentTxL1GasFees' which is not marked 'virtual' |
| warning | W200 | function '_getCurrentTxL1GasFees' in 'FunctionsBilling' overrides a base function but is not marked 'override' |
| warning | W200 | function '_isArbitrumChainId' in 'FunctionsBilling' overrides 'Routable::_isArbitrumChainId' which is not marked 'virtual' |
| warning | W200 | function '_isArbitrumChainId' in 'FunctionsBilling' overrides a base function but is not marked 'override' |
| warning | W200 | function '_isOptimismChainId' in 'FunctionsBilling' overrides 'Routable::_isOptimismChainId' which is not marked 'virtual' |
| warning | W200 | function '_isOptimismChainId' in 'FunctionsBilling' overrides a base function but is not marked 'override' |
| warning | MANIFEST_WILDCARD_CONTRACT | contract 'FunctionsBilling' requires wildcard contract manifest permissions (contract='*') due to dynamic contract calls. This is riskier than fixed contract hashes; use --deny-wildcard-contracts to make this a hard error. |
| warning | W121 | duplicate constant state variable 'L2_TO_L1_MESSAGE_PASSER' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'L2_CROSS_DOMAIN_MESSENGER' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'L2_STANDARD_BRIDGE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'L2_ERC721_BRIDGE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SEQUENCER_FEE_WALLET' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OPTIMISM_MINTABLE_ERC20_FACTORY' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OPTIMISM_MINTABLE_ERC721_FACTORY' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'L1_BLOCK_ATTRIBUTES' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'GAS_PRICE_ORACLE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'L1_MESSAGE_SENDER' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'DEPLOYER_WHITELIST' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LEGACY_ERC20_ETH' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'L1_BLOCK_NUMBER' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LEGACY_MESSAGE_PASSER' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'PROXY_ADMIN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BASE_FEE_VAULT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'L1_FEE_VAULT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'GOVERNANCE_TOKEN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SCHEMA_REGISTRY' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'EAS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ARBGAS_ADDR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ARBGAS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ARB_MAINNET_CHAIN_ID' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ARB_GOERLI_TESTNET_CHAIN_ID' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ARB_SEPOLIA_TESTNET_CHAIN_ID' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'L1_FEE_DATA_PADDING' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OVM_GASPRICEORACLE_ADDR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OVM_GASPRICEORACLE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OP_MAINNET_CHAIN_ID' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OP_GOERLI_CHAIN_ID' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OP_SEPOLIA_CHAIN_ID' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BASE_MAINNET_CHAIN_ID' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BASE_GOERLI_CHAIN_ID' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BASE_SEPOLIA_CHAIN_ID' detected while merging libraries |
| warning | W200 | function 'toUint248' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint248' which is not marked 'virtual' |
| warning | W200 | function 'toUint248' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint240' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint240' which is not marked 'virtual' |
| warning | W200 | function 'toUint240' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint232' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint232' which is not marked 'virtual' |
| warning | W200 | function 'toUint232' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint224' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint224' which is not marked 'virtual' |
| warning | W200 | function 'toUint224' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint216' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint216' which is not marked 'virtual' |
| warning | W200 | function 'toUint216' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint208' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint208' which is not marked 'virtual' |
| warning | W200 | function 'toUint208' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint200' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint200' which is not marked 'virtual' |
| warning | W200 | function 'toUint200' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint192' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint192' which is not marked 'virtual' |
| warning | W200 | function 'toUint192' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint184' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint184' which is not marked 'virtual' |
| warning | W200 | function 'toUint184' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint176' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint176' which is not marked 'virtual' |
| warning | W200 | function 'toUint176' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint168' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint168' which is not marked 'virtual' |
| warning | W200 | function 'toUint168' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint160' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint160' which is not marked 'virtual' |
| warning | W200 | function 'toUint160' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint152' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint152' which is not marked 'virtual' |
| warning | W200 | function 'toUint152' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint144' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint144' which is not marked 'virtual' |
| warning | W200 | function 'toUint144' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint136' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint136' which is not marked 'virtual' |
| warning | W200 | function 'toUint136' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint120' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint120' which is not marked 'virtual' |
| warning | W200 | function 'toUint120' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint112' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint112' which is not marked 'virtual' |
| warning | W200 | function 'toUint112' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint104' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint104' which is not marked 'virtual' |
| warning | W200 | function 'toUint104' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint96' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint96' which is not marked 'virtual' |
| warning | W200 | function 'toUint96' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint88' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint88' which is not marked 'virtual' |
| warning | W200 | function 'toUint88' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint80' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint80' which is not marked 'virtual' |
| warning | W200 | function 'toUint80' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint72' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint72' which is not marked 'virtual' |
| warning | W200 | function 'toUint72' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint64' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint64' which is not marked 'virtual' |
| warning | W200 | function 'toUint64' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint56' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint56' which is not marked 'virtual' |
| warning | W200 | function 'toUint56' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint48' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint48' which is not marked 'virtual' |
| warning | W200 | function 'toUint48' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint40' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint40' which is not marked 'virtual' |
| warning | W200 | function 'toUint40' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint32' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint32' which is not marked 'virtual' |
| warning | W200 | function 'toUint32' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint24' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint24' which is not marked 'virtual' |
| warning | W200 | function 'toUint24' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint16' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint16' which is not marked 'virtual' |
| warning | W200 | function 'toUint16' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint8' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint8' which is not marked 'virtual' |
| warning | W200 | function 'toUint8' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint256' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint256' which is not marked 'virtual' |
| warning | W200 | function 'toUint256' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt248' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt248' which is not marked 'virtual' |
| warning | W200 | function 'toInt248' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt240' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt240' which is not marked 'virtual' |
| warning | W200 | function 'toInt240' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt232' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt232' which is not marked 'virtual' |
| warning | W200 | function 'toInt232' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt224' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt224' which is not marked 'virtual' |
| warning | W200 | function 'toInt224' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt216' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt216' which is not marked 'virtual' |
| warning | W200 | function 'toInt216' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt208' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt208' which is not marked 'virtual' |
| warning | W200 | function 'toInt208' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt200' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt200' which is not marked 'virtual' |
| warning | W200 | function 'toInt200' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt192' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt192' which is not marked 'virtual' |
| warning | W200 | function 'toInt192' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt184' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt184' which is not marked 'virtual' |
| warning | W200 | function 'toInt184' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt176' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt176' which is not marked 'virtual' |
| warning | W200 | function 'toInt176' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt168' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt168' which is not marked 'virtual' |
| warning | W200 | function 'toInt168' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt160' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt160' which is not marked 'virtual' |
| warning | W200 | function 'toInt160' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt152' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt152' which is not marked 'virtual' |
| warning | W200 | function 'toInt152' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt144' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt144' which is not marked 'virtual' |
| warning | W200 | function 'toInt144' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt136' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt136' which is not marked 'virtual' |
| warning | W200 | function 'toInt136' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt120' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt120' which is not marked 'virtual' |
| warning | W200 | function 'toInt120' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt112' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt112' which is not marked 'virtual' |
| warning | W200 | function 'toInt112' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt104' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt104' which is not marked 'virtual' |
| warning | W200 | function 'toInt104' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt96' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt96' which is not marked 'virtual' |
| warning | W200 | function 'toInt96' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt88' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt88' which is not marked 'virtual' |
| warning | W200 | function 'toInt88' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt80' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt80' which is not marked 'virtual' |
| warning | W200 | function 'toInt80' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt72' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt72' which is not marked 'virtual' |
| warning | W200 | function 'toInt72' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt64' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt64' which is not marked 'virtual' |
| warning | W200 | function 'toInt64' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt56' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt56' which is not marked 'virtual' |
| warning | W200 | function 'toInt56' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt48' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt48' which is not marked 'virtual' |
| warning | W200 | function 'toInt48' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt40' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt40' which is not marked 'virtual' |
| warning | W200 | function 'toInt40' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt32' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt32' which is not marked 'virtual' |
| warning | W200 | function 'toInt32' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt24' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt24' which is not marked 'virtual' |
| warning | W200 | function 'toInt24' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt16' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt16' which is not marked 'virtual' |
| warning | W200 | function 'toInt16' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt8' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt8' which is not marked 'virtual' |
| warning | W200 | function 'toInt8' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt256' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt256' which is not marked 'virtual' |
| warning | W200 | function 'toInt256' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getCurrentTxL1GasFees' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::_getCurrentTxL1GasFees' which is not marked 'virtual' |
| warning | W200 | function '_getCurrentTxL1GasFees' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function '_isArbitrumChainId' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::_isArbitrumChainId' which is not marked 'virtual' |
| warning | W200 | function '_isArbitrumChainId' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function '_isOptimismChainId' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::_isOptimismChainId' which is not marked 'virtual' |
| warning | W200 | function '_isOptimismChainId' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | VALIDATION_WARNING | abstract contract 'OCR2Abstract' has 4 unimplemented function(s): [setConfig, latestConfigDetails, latestConfigDigestAndEpoch, transmit] |
| warning | W121 | duplicate constant state variable 'L2_TO_L1_MESSAGE_PASSER' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'L2_CROSS_DOMAIN_MESSENGER' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'L2_STANDARD_BRIDGE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'L2_ERC721_BRIDGE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SEQUENCER_FEE_WALLET' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OPTIMISM_MINTABLE_ERC20_FACTORY' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OPTIMISM_MINTABLE_ERC721_FACTORY' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'L1_BLOCK_ATTRIBUTES' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'GAS_PRICE_ORACLE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'L1_MESSAGE_SENDER' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'DEPLOYER_WHITELIST' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LEGACY_ERC20_ETH' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'L1_BLOCK_NUMBER' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LEGACY_MESSAGE_PASSER' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'PROXY_ADMIN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BASE_FEE_VAULT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'L1_FEE_VAULT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'GOVERNANCE_TOKEN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SCHEMA_REGISTRY' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'EAS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ARBGAS_ADDR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ARBGAS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ARB_MAINNET_CHAIN_ID' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ARB_GOERLI_TESTNET_CHAIN_ID' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ARB_SEPOLIA_TESTNET_CHAIN_ID' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'L1_FEE_DATA_PADDING' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OVM_GASPRICEORACLE_ADDR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OVM_GASPRICEORACLE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OP_MAINNET_CHAIN_ID' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OP_GOERLI_CHAIN_ID' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OP_SEPOLIA_CHAIN_ID' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BASE_MAINNET_CHAIN_ID' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BASE_GOERLI_CHAIN_ID' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BASE_SEPOLIA_CHAIN_ID' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'L2_TO_L1_MESSAGE_PASSER' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'L2_CROSS_DOMAIN_MESSENGER' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'L2_STANDARD_BRIDGE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'L2_ERC721_BRIDGE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SEQUENCER_FEE_WALLET' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OPTIMISM_MINTABLE_ERC20_FACTORY' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OPTIMISM_MINTABLE_ERC721_FACTORY' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'L1_BLOCK_ATTRIBUTES' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'GAS_PRICE_ORACLE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'L1_MESSAGE_SENDER' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'DEPLOYER_WHITELIST' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LEGACY_ERC20_ETH' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'L1_BLOCK_NUMBER' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LEGACY_MESSAGE_PASSER' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'PROXY_ADMIN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BASE_FEE_VAULT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'L1_FEE_VAULT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'GOVERNANCE_TOKEN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SCHEMA_REGISTRY' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'EAS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ARBGAS_ADDR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ARBGAS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ARB_MAINNET_CHAIN_ID' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ARB_GOERLI_TESTNET_CHAIN_ID' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ARB_SEPOLIA_TESTNET_CHAIN_ID' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'L1_FEE_DATA_PADDING' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OVM_GASPRICEORACLE_ADDR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OVM_GASPRICEORACLE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OP_MAINNET_CHAIN_ID' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OP_GOERLI_CHAIN_ID' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OP_SEPOLIA_CHAIN_ID' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BASE_MAINNET_CHAIN_ID' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BASE_GOERLI_CHAIN_ID' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BASE_SEPOLIA_CHAIN_ID' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'L2_TO_L1_MESSAGE_PASSER' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'L2_CROSS_DOMAIN_MESSENGER' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'L2_STANDARD_BRIDGE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'L2_ERC721_BRIDGE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SEQUENCER_FEE_WALLET' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OPTIMISM_MINTABLE_ERC20_FACTORY' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OPTIMISM_MINTABLE_ERC721_FACTORY' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'L1_BLOCK_ATTRIBUTES' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'GAS_PRICE_ORACLE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'L1_MESSAGE_SENDER' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'DEPLOYER_WHITELIST' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LEGACY_ERC20_ETH' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'L1_BLOCK_NUMBER' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LEGACY_MESSAGE_PASSER' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'PROXY_ADMIN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BASE_FEE_VAULT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'L1_FEE_VAULT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'GOVERNANCE_TOKEN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SCHEMA_REGISTRY' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'EAS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ARBGAS_ADDR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ARBGAS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ARB_MAINNET_CHAIN_ID' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ARB_GOERLI_TESTNET_CHAIN_ID' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ARB_SEPOLIA_TESTNET_CHAIN_ID' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'L1_FEE_DATA_PADDING' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OVM_GASPRICEORACLE_ADDR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OVM_GASPRICEORACLE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OP_MAINNET_CHAIN_ID' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OP_GOERLI_CHAIN_ID' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OP_SEPOLIA_CHAIN_ID' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BASE_MAINNET_CHAIN_ID' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BASE_GOERLI_CHAIN_ID' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BASE_SEPOLIA_CHAIN_ID' detected while merging libraries |
| warning | VALIDATION_WARNING | abstract contract 'OCR2Base' has 3 unimplemented function(s): [_beforeSetConfig, _report, _beforeTransmit] |
| warning | W200 | function 'toUint248' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint248' which is not marked 'virtual' |
| warning | W200 | function 'toUint248' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint240' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint240' which is not marked 'virtual' |
| warning | W200 | function 'toUint240' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint232' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint232' which is not marked 'virtual' |
| warning | W200 | function 'toUint232' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint224' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint224' which is not marked 'virtual' |
| warning | W200 | function 'toUint224' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint216' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint216' which is not marked 'virtual' |
| warning | W200 | function 'toUint216' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint208' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint208' which is not marked 'virtual' |
| warning | W200 | function 'toUint208' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint200' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint200' which is not marked 'virtual' |
| warning | W200 | function 'toUint200' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint192' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint192' which is not marked 'virtual' |
| warning | W200 | function 'toUint192' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint184' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint184' which is not marked 'virtual' |
| warning | W200 | function 'toUint184' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint176' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint176' which is not marked 'virtual' |
| warning | W200 | function 'toUint176' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint168' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint168' which is not marked 'virtual' |
| warning | W200 | function 'toUint168' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint160' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint160' which is not marked 'virtual' |
| warning | W200 | function 'toUint160' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint152' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint152' which is not marked 'virtual' |
| warning | W200 | function 'toUint152' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint144' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint144' which is not marked 'virtual' |
| warning | W200 | function 'toUint144' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint136' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint136' which is not marked 'virtual' |
| warning | W200 | function 'toUint136' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint120' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint120' which is not marked 'virtual' |
| warning | W200 | function 'toUint120' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint112' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint112' which is not marked 'virtual' |
| warning | W200 | function 'toUint112' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint104' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint104' which is not marked 'virtual' |
| warning | W200 | function 'toUint104' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint96' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint96' which is not marked 'virtual' |
| warning | W200 | function 'toUint96' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint88' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint88' which is not marked 'virtual' |
| warning | W200 | function 'toUint88' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint80' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint80' which is not marked 'virtual' |
| warning | W200 | function 'toUint80' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint72' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint72' which is not marked 'virtual' |
| warning | W200 | function 'toUint72' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint64' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint64' which is not marked 'virtual' |
| warning | W200 | function 'toUint64' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint56' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint56' which is not marked 'virtual' |
| warning | W200 | function 'toUint56' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint48' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint48' which is not marked 'virtual' |
| warning | W200 | function 'toUint48' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint40' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint40' which is not marked 'virtual' |
| warning | W200 | function 'toUint40' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint32' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint32' which is not marked 'virtual' |
| warning | W200 | function 'toUint32' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint24' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint24' which is not marked 'virtual' |
| warning | W200 | function 'toUint24' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint16' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint16' which is not marked 'virtual' |
| warning | W200 | function 'toUint16' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint8' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint8' which is not marked 'virtual' |
| warning | W200 | function 'toUint8' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint256' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint256' which is not marked 'virtual' |
| warning | W200 | function 'toUint256' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt248' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt248' which is not marked 'virtual' |
| warning | W200 | function 'toInt248' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt240' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt240' which is not marked 'virtual' |
| warning | W200 | function 'toInt240' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt232' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt232' which is not marked 'virtual' |
| warning | W200 | function 'toInt232' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt224' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt224' which is not marked 'virtual' |
| warning | W200 | function 'toInt224' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt216' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt216' which is not marked 'virtual' |
| warning | W200 | function 'toInt216' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt208' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt208' which is not marked 'virtual' |
| warning | W200 | function 'toInt208' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt200' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt200' which is not marked 'virtual' |
| warning | W200 | function 'toInt200' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt192' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt192' which is not marked 'virtual' |
| warning | W200 | function 'toInt192' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt184' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt184' which is not marked 'virtual' |
| warning | W200 | function 'toInt184' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt176' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt176' which is not marked 'virtual' |
| warning | W200 | function 'toInt176' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt168' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt168' which is not marked 'virtual' |
| warning | W200 | function 'toInt168' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt160' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt160' which is not marked 'virtual' |
| warning | W200 | function 'toInt160' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt152' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt152' which is not marked 'virtual' |
| warning | W200 | function 'toInt152' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt144' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt144' which is not marked 'virtual' |
| warning | W200 | function 'toInt144' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt136' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt136' which is not marked 'virtual' |
| warning | W200 | function 'toInt136' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt120' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt120' which is not marked 'virtual' |
| warning | W200 | function 'toInt120' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt112' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt112' which is not marked 'virtual' |
| warning | W200 | function 'toInt112' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt104' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt104' which is not marked 'virtual' |
| warning | W200 | function 'toInt104' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt96' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt96' which is not marked 'virtual' |
| warning | W200 | function 'toInt96' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt88' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt88' which is not marked 'virtual' |
| warning | W200 | function 'toInt88' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt80' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt80' which is not marked 'virtual' |
| warning | W200 | function 'toInt80' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt72' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt72' which is not marked 'virtual' |
| warning | W200 | function 'toInt72' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt64' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt64' which is not marked 'virtual' |
| warning | W200 | function 'toInt64' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt56' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt56' which is not marked 'virtual' |
| warning | W200 | function 'toInt56' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt48' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt48' which is not marked 'virtual' |
| warning | W200 | function 'toInt48' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt40' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt40' which is not marked 'virtual' |
| warning | W200 | function 'toInt40' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt32' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt32' which is not marked 'virtual' |
| warning | W200 | function 'toInt32' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt24' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt24' which is not marked 'virtual' |
| warning | W200 | function 'toInt24' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt16' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt16' which is not marked 'virtual' |
| warning | W200 | function 'toInt16' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt8' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt8' which is not marked 'virtual' |
| warning | W200 | function 'toInt8' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt256' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt256' which is not marked 'virtual' |
| warning | W200 | function 'toInt256' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getCurrentTxL1GasFees' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::_getCurrentTxL1GasFees' which is not marked 'virtual' |
| warning | W200 | function '_getCurrentTxL1GasFees' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function '_isArbitrumChainId' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::_isArbitrumChainId' which is not marked 'virtual' |
| warning | W200 | function '_isArbitrumChainId' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function '_isOptimismChainId' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::_isOptimismChainId' which is not marked 'virtual' |
| warning | W200 | function '_isOptimismChainId' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint248' in 'OCR2Abstract' overrides 'ConfirmedOwner::toUint248' which is not marked 'virtual' |
| warning | W200 | function 'toUint248' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint240' in 'OCR2Abstract' overrides 'ConfirmedOwner::toUint240' which is not marked 'virtual' |
| warning | W200 | function 'toUint240' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint232' in 'OCR2Abstract' overrides 'ConfirmedOwner::toUint232' which is not marked 'virtual' |
| warning | W200 | function 'toUint232' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint224' in 'OCR2Abstract' overrides 'ConfirmedOwner::toUint224' which is not marked 'virtual' |
| warning | W200 | function 'toUint224' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint216' in 'OCR2Abstract' overrides 'ConfirmedOwner::toUint216' which is not marked 'virtual' |
| warning | W200 | function 'toUint216' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint208' in 'OCR2Abstract' overrides 'ConfirmedOwner::toUint208' which is not marked 'virtual' |
| warning | W200 | function 'toUint208' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint200' in 'OCR2Abstract' overrides 'ConfirmedOwner::toUint200' which is not marked 'virtual' |
| warning | W200 | function 'toUint200' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint192' in 'OCR2Abstract' overrides 'ConfirmedOwner::toUint192' which is not marked 'virtual' |
| warning | W200 | function 'toUint192' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint184' in 'OCR2Abstract' overrides 'ConfirmedOwner::toUint184' which is not marked 'virtual' |
| warning | W200 | function 'toUint184' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint176' in 'OCR2Abstract' overrides 'ConfirmedOwner::toUint176' which is not marked 'virtual' |
| warning | W200 | function 'toUint176' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint168' in 'OCR2Abstract' overrides 'ConfirmedOwner::toUint168' which is not marked 'virtual' |
| warning | W200 | function 'toUint168' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint160' in 'OCR2Abstract' overrides 'ConfirmedOwner::toUint160' which is not marked 'virtual' |
| warning | W200 | function 'toUint160' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint152' in 'OCR2Abstract' overrides 'ConfirmedOwner::toUint152' which is not marked 'virtual' |
| warning | W200 | function 'toUint152' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint144' in 'OCR2Abstract' overrides 'ConfirmedOwner::toUint144' which is not marked 'virtual' |
| warning | W200 | function 'toUint144' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint136' in 'OCR2Abstract' overrides 'ConfirmedOwner::toUint136' which is not marked 'virtual' |
| warning | W200 | function 'toUint136' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'OCR2Abstract' overrides 'ConfirmedOwner::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint120' in 'OCR2Abstract' overrides 'ConfirmedOwner::toUint120' which is not marked 'virtual' |
| warning | W200 | function 'toUint120' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint112' in 'OCR2Abstract' overrides 'ConfirmedOwner::toUint112' which is not marked 'virtual' |
| warning | W200 | function 'toUint112' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint104' in 'OCR2Abstract' overrides 'ConfirmedOwner::toUint104' which is not marked 'virtual' |
| warning | W200 | function 'toUint104' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint96' in 'OCR2Abstract' overrides 'ConfirmedOwner::toUint96' which is not marked 'virtual' |
| warning | W200 | function 'toUint96' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint88' in 'OCR2Abstract' overrides 'ConfirmedOwner::toUint88' which is not marked 'virtual' |
| warning | W200 | function 'toUint88' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint80' in 'OCR2Abstract' overrides 'ConfirmedOwner::toUint80' which is not marked 'virtual' |
| warning | W200 | function 'toUint80' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint72' in 'OCR2Abstract' overrides 'ConfirmedOwner::toUint72' which is not marked 'virtual' |
| warning | W200 | function 'toUint72' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint64' in 'OCR2Abstract' overrides 'ConfirmedOwner::toUint64' which is not marked 'virtual' |
| warning | W200 | function 'toUint64' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint56' in 'OCR2Abstract' overrides 'ConfirmedOwner::toUint56' which is not marked 'virtual' |
| warning | W200 | function 'toUint56' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint48' in 'OCR2Abstract' overrides 'ConfirmedOwner::toUint48' which is not marked 'virtual' |
| warning | W200 | function 'toUint48' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint40' in 'OCR2Abstract' overrides 'ConfirmedOwner::toUint40' which is not marked 'virtual' |
| warning | W200 | function 'toUint40' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint32' in 'OCR2Abstract' overrides 'ConfirmedOwner::toUint32' which is not marked 'virtual' |
| warning | W200 | function 'toUint32' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint24' in 'OCR2Abstract' overrides 'ConfirmedOwner::toUint24' which is not marked 'virtual' |
| warning | W200 | function 'toUint24' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint16' in 'OCR2Abstract' overrides 'ConfirmedOwner::toUint16' which is not marked 'virtual' |
| warning | W200 | function 'toUint16' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint8' in 'OCR2Abstract' overrides 'ConfirmedOwner::toUint8' which is not marked 'virtual' |
| warning | W200 | function 'toUint8' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint256' in 'OCR2Abstract' overrides 'ConfirmedOwner::toUint256' which is not marked 'virtual' |
| warning | W200 | function 'toUint256' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt248' in 'OCR2Abstract' overrides 'ConfirmedOwner::toInt248' which is not marked 'virtual' |
| warning | W200 | function 'toInt248' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt240' in 'OCR2Abstract' overrides 'ConfirmedOwner::toInt240' which is not marked 'virtual' |
| warning | W200 | function 'toInt240' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt232' in 'OCR2Abstract' overrides 'ConfirmedOwner::toInt232' which is not marked 'virtual' |
| warning | W200 | function 'toInt232' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt224' in 'OCR2Abstract' overrides 'ConfirmedOwner::toInt224' which is not marked 'virtual' |
| warning | W200 | function 'toInt224' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt216' in 'OCR2Abstract' overrides 'ConfirmedOwner::toInt216' which is not marked 'virtual' |
| warning | W200 | function 'toInt216' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt208' in 'OCR2Abstract' overrides 'ConfirmedOwner::toInt208' which is not marked 'virtual' |
| warning | W200 | function 'toInt208' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt200' in 'OCR2Abstract' overrides 'ConfirmedOwner::toInt200' which is not marked 'virtual' |
| warning | W200 | function 'toInt200' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt192' in 'OCR2Abstract' overrides 'ConfirmedOwner::toInt192' which is not marked 'virtual' |
| warning | W200 | function 'toInt192' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt184' in 'OCR2Abstract' overrides 'ConfirmedOwner::toInt184' which is not marked 'virtual' |
| warning | W200 | function 'toInt184' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt176' in 'OCR2Abstract' overrides 'ConfirmedOwner::toInt176' which is not marked 'virtual' |
| warning | W200 | function 'toInt176' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt168' in 'OCR2Abstract' overrides 'ConfirmedOwner::toInt168' which is not marked 'virtual' |
| warning | W200 | function 'toInt168' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt160' in 'OCR2Abstract' overrides 'ConfirmedOwner::toInt160' which is not marked 'virtual' |
| warning | W200 | function 'toInt160' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt152' in 'OCR2Abstract' overrides 'ConfirmedOwner::toInt152' which is not marked 'virtual' |
| warning | W200 | function 'toInt152' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt144' in 'OCR2Abstract' overrides 'ConfirmedOwner::toInt144' which is not marked 'virtual' |
| warning | W200 | function 'toInt144' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt136' in 'OCR2Abstract' overrides 'ConfirmedOwner::toInt136' which is not marked 'virtual' |
| warning | W200 | function 'toInt136' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'OCR2Abstract' overrides 'ConfirmedOwner::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt120' in 'OCR2Abstract' overrides 'ConfirmedOwner::toInt120' which is not marked 'virtual' |
| warning | W200 | function 'toInt120' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt112' in 'OCR2Abstract' overrides 'ConfirmedOwner::toInt112' which is not marked 'virtual' |
| warning | W200 | function 'toInt112' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt104' in 'OCR2Abstract' overrides 'ConfirmedOwner::toInt104' which is not marked 'virtual' |
| warning | W200 | function 'toInt104' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt96' in 'OCR2Abstract' overrides 'ConfirmedOwner::toInt96' which is not marked 'virtual' |
| warning | W200 | function 'toInt96' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt88' in 'OCR2Abstract' overrides 'ConfirmedOwner::toInt88' which is not marked 'virtual' |
| warning | W200 | function 'toInt88' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt80' in 'OCR2Abstract' overrides 'ConfirmedOwner::toInt80' which is not marked 'virtual' |
| warning | W200 | function 'toInt80' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt72' in 'OCR2Abstract' overrides 'ConfirmedOwner::toInt72' which is not marked 'virtual' |
| warning | W200 | function 'toInt72' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt64' in 'OCR2Abstract' overrides 'ConfirmedOwner::toInt64' which is not marked 'virtual' |
| warning | W200 | function 'toInt64' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt56' in 'OCR2Abstract' overrides 'ConfirmedOwner::toInt56' which is not marked 'virtual' |
| warning | W200 | function 'toInt56' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt48' in 'OCR2Abstract' overrides 'ConfirmedOwner::toInt48' which is not marked 'virtual' |
| warning | W200 | function 'toInt48' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt40' in 'OCR2Abstract' overrides 'ConfirmedOwner::toInt40' which is not marked 'virtual' |
| warning | W200 | function 'toInt40' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt32' in 'OCR2Abstract' overrides 'ConfirmedOwner::toInt32' which is not marked 'virtual' |
| warning | W200 | function 'toInt32' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt24' in 'OCR2Abstract' overrides 'ConfirmedOwner::toInt24' which is not marked 'virtual' |
| warning | W200 | function 'toInt24' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt16' in 'OCR2Abstract' overrides 'ConfirmedOwner::toInt16' which is not marked 'virtual' |
| warning | W200 | function 'toInt16' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt8' in 'OCR2Abstract' overrides 'ConfirmedOwner::toInt8' which is not marked 'virtual' |
| warning | W200 | function 'toInt8' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt256' in 'OCR2Abstract' overrides 'ConfirmedOwner::toInt256' which is not marked 'virtual' |
| warning | W200 | function 'toInt256' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getCurrentTxL1GasFees' in 'OCR2Abstract' overrides 'ConfirmedOwner::_getCurrentTxL1GasFees' which is not marked 'virtual' |
| warning | W200 | function '_getCurrentTxL1GasFees' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function '_isArbitrumChainId' in 'OCR2Abstract' overrides 'ConfirmedOwner::_isArbitrumChainId' which is not marked 'virtual' |
| warning | W200 | function '_isArbitrumChainId' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function '_isOptimismChainId' in 'OCR2Abstract' overrides 'ConfirmedOwner::_isOptimismChainId' which is not marked 'virtual' |
| warning | W200 | function '_isOptimismChainId' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint248' in 'OCR2Base' overrides 'OCR2Abstract::toUint248' which is not marked 'virtual' |
| warning | W200 | function 'toUint248' in 'OCR2Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint240' in 'OCR2Base' overrides 'OCR2Abstract::toUint240' which is not marked 'virtual' |
| warning | W200 | function 'toUint240' in 'OCR2Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint232' in 'OCR2Base' overrides 'OCR2Abstract::toUint232' which is not marked 'virtual' |
| warning | W200 | function 'toUint232' in 'OCR2Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint224' in 'OCR2Base' overrides 'OCR2Abstract::toUint224' which is not marked 'virtual' |
| warning | W200 | function 'toUint224' in 'OCR2Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint216' in 'OCR2Base' overrides 'OCR2Abstract::toUint216' which is not marked 'virtual' |
| warning | W200 | function 'toUint216' in 'OCR2Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint208' in 'OCR2Base' overrides 'OCR2Abstract::toUint208' which is not marked 'virtual' |
| warning | W200 | function 'toUint208' in 'OCR2Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint200' in 'OCR2Base' overrides 'OCR2Abstract::toUint200' which is not marked 'virtual' |
| warning | W200 | function 'toUint200' in 'OCR2Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint192' in 'OCR2Base' overrides 'OCR2Abstract::toUint192' which is not marked 'virtual' |
| warning | W200 | function 'toUint192' in 'OCR2Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint184' in 'OCR2Base' overrides 'OCR2Abstract::toUint184' which is not marked 'virtual' |
| warning | W200 | function 'toUint184' in 'OCR2Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint176' in 'OCR2Base' overrides 'OCR2Abstract::toUint176' which is not marked 'virtual' |
| warning | W200 | function 'toUint176' in 'OCR2Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint168' in 'OCR2Base' overrides 'OCR2Abstract::toUint168' which is not marked 'virtual' |
| warning | W200 | function 'toUint168' in 'OCR2Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint160' in 'OCR2Base' overrides 'OCR2Abstract::toUint160' which is not marked 'virtual' |
| warning | W200 | function 'toUint160' in 'OCR2Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint152' in 'OCR2Base' overrides 'OCR2Abstract::toUint152' which is not marked 'virtual' |
| warning | W200 | function 'toUint152' in 'OCR2Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint144' in 'OCR2Base' overrides 'OCR2Abstract::toUint144' which is not marked 'virtual' |
| warning | W200 | function 'toUint144' in 'OCR2Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint136' in 'OCR2Base' overrides 'OCR2Abstract::toUint136' which is not marked 'virtual' |
| warning | W200 | function 'toUint136' in 'OCR2Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'OCR2Base' overrides 'OCR2Abstract::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'OCR2Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint120' in 'OCR2Base' overrides 'OCR2Abstract::toUint120' which is not marked 'virtual' |
| warning | W200 | function 'toUint120' in 'OCR2Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint112' in 'OCR2Base' overrides 'OCR2Abstract::toUint112' which is not marked 'virtual' |
| warning | W200 | function 'toUint112' in 'OCR2Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint104' in 'OCR2Base' overrides 'OCR2Abstract::toUint104' which is not marked 'virtual' |
| warning | W200 | function 'toUint104' in 'OCR2Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint96' in 'OCR2Base' overrides 'OCR2Abstract::toUint96' which is not marked 'virtual' |
| warning | W200 | function 'toUint96' in 'OCR2Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint88' in 'OCR2Base' overrides 'OCR2Abstract::toUint88' which is not marked 'virtual' |
| warning | W200 | function 'toUint88' in 'OCR2Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint80' in 'OCR2Base' overrides 'OCR2Abstract::toUint80' which is not marked 'virtual' |
| warning | W200 | function 'toUint80' in 'OCR2Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint72' in 'OCR2Base' overrides 'OCR2Abstract::toUint72' which is not marked 'virtual' |
| warning | W200 | function 'toUint72' in 'OCR2Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint64' in 'OCR2Base' overrides 'OCR2Abstract::toUint64' which is not marked 'virtual' |
| warning | W200 | function 'toUint64' in 'OCR2Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint56' in 'OCR2Base' overrides 'OCR2Abstract::toUint56' which is not marked 'virtual' |
| warning | W200 | function 'toUint56' in 'OCR2Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint48' in 'OCR2Base' overrides 'OCR2Abstract::toUint48' which is not marked 'virtual' |
| warning | W200 | function 'toUint48' in 'OCR2Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint40' in 'OCR2Base' overrides 'OCR2Abstract::toUint40' which is not marked 'virtual' |
| warning | W200 | function 'toUint40' in 'OCR2Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint32' in 'OCR2Base' overrides 'OCR2Abstract::toUint32' which is not marked 'virtual' |
| warning | W200 | function 'toUint32' in 'OCR2Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint24' in 'OCR2Base' overrides 'OCR2Abstract::toUint24' which is not marked 'virtual' |
| warning | W200 | function 'toUint24' in 'OCR2Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint16' in 'OCR2Base' overrides 'OCR2Abstract::toUint16' which is not marked 'virtual' |
| warning | W200 | function 'toUint16' in 'OCR2Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint8' in 'OCR2Base' overrides 'OCR2Abstract::toUint8' which is not marked 'virtual' |
| warning | W200 | function 'toUint8' in 'OCR2Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint256' in 'OCR2Base' overrides 'OCR2Abstract::toUint256' which is not marked 'virtual' |
| warning | W200 | function 'toUint256' in 'OCR2Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt248' in 'OCR2Base' overrides 'OCR2Abstract::toInt248' which is not marked 'virtual' |
| warning | W200 | function 'toInt248' in 'OCR2Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt240' in 'OCR2Base' overrides 'OCR2Abstract::toInt240' which is not marked 'virtual' |
| warning | W200 | function 'toInt240' in 'OCR2Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt232' in 'OCR2Base' overrides 'OCR2Abstract::toInt232' which is not marked 'virtual' |
| warning | W200 | function 'toInt232' in 'OCR2Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt224' in 'OCR2Base' overrides 'OCR2Abstract::toInt224' which is not marked 'virtual' |
| warning | W200 | function 'toInt224' in 'OCR2Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt216' in 'OCR2Base' overrides 'OCR2Abstract::toInt216' which is not marked 'virtual' |
| warning | W200 | function 'toInt216' in 'OCR2Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt208' in 'OCR2Base' overrides 'OCR2Abstract::toInt208' which is not marked 'virtual' |
| warning | W200 | function 'toInt208' in 'OCR2Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt200' in 'OCR2Base' overrides 'OCR2Abstract::toInt200' which is not marked 'virtual' |
| warning | W200 | function 'toInt200' in 'OCR2Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt192' in 'OCR2Base' overrides 'OCR2Abstract::toInt192' which is not marked 'virtual' |
| warning | W200 | function 'toInt192' in 'OCR2Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt184' in 'OCR2Base' overrides 'OCR2Abstract::toInt184' which is not marked 'virtual' |
| warning | W200 | function 'toInt184' in 'OCR2Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt176' in 'OCR2Base' overrides 'OCR2Abstract::toInt176' which is not marked 'virtual' |
| warning | W200 | function 'toInt176' in 'OCR2Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt168' in 'OCR2Base' overrides 'OCR2Abstract::toInt168' which is not marked 'virtual' |
| warning | W200 | function 'toInt168' in 'OCR2Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt160' in 'OCR2Base' overrides 'OCR2Abstract::toInt160' which is not marked 'virtual' |
| warning | W200 | function 'toInt160' in 'OCR2Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt152' in 'OCR2Base' overrides 'OCR2Abstract::toInt152' which is not marked 'virtual' |
| warning | W200 | function 'toInt152' in 'OCR2Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt144' in 'OCR2Base' overrides 'OCR2Abstract::toInt144' which is not marked 'virtual' |
| warning | W200 | function 'toInt144' in 'OCR2Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt136' in 'OCR2Base' overrides 'OCR2Abstract::toInt136' which is not marked 'virtual' |
| warning | W200 | function 'toInt136' in 'OCR2Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'OCR2Base' overrides 'OCR2Abstract::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'OCR2Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt120' in 'OCR2Base' overrides 'OCR2Abstract::toInt120' which is not marked 'virtual' |
| warning | W200 | function 'toInt120' in 'OCR2Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt112' in 'OCR2Base' overrides 'OCR2Abstract::toInt112' which is not marked 'virtual' |
| warning | W200 | function 'toInt112' in 'OCR2Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt104' in 'OCR2Base' overrides 'OCR2Abstract::toInt104' which is not marked 'virtual' |
| warning | W200 | function 'toInt104' in 'OCR2Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt96' in 'OCR2Base' overrides 'OCR2Abstract::toInt96' which is not marked 'virtual' |
| warning | W200 | function 'toInt96' in 'OCR2Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt88' in 'OCR2Base' overrides 'OCR2Abstract::toInt88' which is not marked 'virtual' |
| warning | W200 | function 'toInt88' in 'OCR2Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt80' in 'OCR2Base' overrides 'OCR2Abstract::toInt80' which is not marked 'virtual' |
| warning | W200 | function 'toInt80' in 'OCR2Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt72' in 'OCR2Base' overrides 'OCR2Abstract::toInt72' which is not marked 'virtual' |
| warning | W200 | function 'toInt72' in 'OCR2Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt64' in 'OCR2Base' overrides 'OCR2Abstract::toInt64' which is not marked 'virtual' |
| warning | W200 | function 'toInt64' in 'OCR2Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt56' in 'OCR2Base' overrides 'OCR2Abstract::toInt56' which is not marked 'virtual' |
| warning | W200 | function 'toInt56' in 'OCR2Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt48' in 'OCR2Base' overrides 'OCR2Abstract::toInt48' which is not marked 'virtual' |
| warning | W200 | function 'toInt48' in 'OCR2Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt40' in 'OCR2Base' overrides 'OCR2Abstract::toInt40' which is not marked 'virtual' |
| warning | W200 | function 'toInt40' in 'OCR2Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt32' in 'OCR2Base' overrides 'OCR2Abstract::toInt32' which is not marked 'virtual' |
| warning | W200 | function 'toInt32' in 'OCR2Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt24' in 'OCR2Base' overrides 'OCR2Abstract::toInt24' which is not marked 'virtual' |
| warning | W200 | function 'toInt24' in 'OCR2Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt16' in 'OCR2Base' overrides 'OCR2Abstract::toInt16' which is not marked 'virtual' |
| warning | W200 | function 'toInt16' in 'OCR2Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt8' in 'OCR2Base' overrides 'OCR2Abstract::toInt8' which is not marked 'virtual' |
| warning | W200 | function 'toInt8' in 'OCR2Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt256' in 'OCR2Base' overrides 'OCR2Abstract::toInt256' which is not marked 'virtual' |
| warning | W200 | function 'toInt256' in 'OCR2Base' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getCurrentTxL1GasFees' in 'OCR2Base' overrides 'OCR2Abstract::_getCurrentTxL1GasFees' which is not marked 'virtual' |
| warning | W200 | function '_getCurrentTxL1GasFees' in 'OCR2Base' overrides a base function but is not marked 'override' |
| warning | W200 | function '_isArbitrumChainId' in 'OCR2Base' overrides 'OCR2Abstract::_isArbitrumChainId' which is not marked 'virtual' |
| warning | W200 | function '_isArbitrumChainId' in 'OCR2Base' overrides a base function but is not marked 'override' |
| warning | W200 | function '_isOptimismChainId' in 'OCR2Base' overrides 'OCR2Abstract::_isOptimismChainId' which is not marked 'virtual' |
| warning | W200 | function '_isOptimismChainId' in 'OCR2Base' overrides a base function but is not marked 'override' |
| warning | W121 | duplicate constant state variable 'L2_TO_L1_MESSAGE_PASSER' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'L2_CROSS_DOMAIN_MESSENGER' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'L2_STANDARD_BRIDGE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'L2_ERC721_BRIDGE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SEQUENCER_FEE_WALLET' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OPTIMISM_MINTABLE_ERC20_FACTORY' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OPTIMISM_MINTABLE_ERC721_FACTORY' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'L1_BLOCK_ATTRIBUTES' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'GAS_PRICE_ORACLE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'L1_MESSAGE_SENDER' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'DEPLOYER_WHITELIST' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LEGACY_ERC20_ETH' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'L1_BLOCK_NUMBER' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LEGACY_MESSAGE_PASSER' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'PROXY_ADMIN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BASE_FEE_VAULT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'L1_FEE_VAULT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'GOVERNANCE_TOKEN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SCHEMA_REGISTRY' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'EAS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ARBGAS_ADDR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ARBGAS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ARB_MAINNET_CHAIN_ID' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ARB_GOERLI_TESTNET_CHAIN_ID' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ARB_SEPOLIA_TESTNET_CHAIN_ID' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'L1_FEE_DATA_PADDING' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OVM_GASPRICEORACLE_ADDR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OVM_GASPRICEORACLE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OP_MAINNET_CHAIN_ID' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OP_GOERLI_CHAIN_ID' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OP_SEPOLIA_CHAIN_ID' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BASE_MAINNET_CHAIN_ID' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BASE_GOERLI_CHAIN_ID' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BASE_SEPOLIA_CHAIN_ID' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'L2_TO_L1_MESSAGE_PASSER' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'L2_CROSS_DOMAIN_MESSENGER' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'L2_STANDARD_BRIDGE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'L2_ERC721_BRIDGE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SEQUENCER_FEE_WALLET' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OPTIMISM_MINTABLE_ERC20_FACTORY' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OPTIMISM_MINTABLE_ERC721_FACTORY' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'L1_BLOCK_ATTRIBUTES' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'GAS_PRICE_ORACLE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'L1_MESSAGE_SENDER' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'DEPLOYER_WHITELIST' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LEGACY_ERC20_ETH' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'L1_BLOCK_NUMBER' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LEGACY_MESSAGE_PASSER' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'PROXY_ADMIN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BASE_FEE_VAULT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'L1_FEE_VAULT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'GOVERNANCE_TOKEN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SCHEMA_REGISTRY' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'EAS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ARBGAS_ADDR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ARBGAS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ARB_MAINNET_CHAIN_ID' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ARB_GOERLI_TESTNET_CHAIN_ID' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ARB_SEPOLIA_TESTNET_CHAIN_ID' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'L1_FEE_DATA_PADDING' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OVM_GASPRICEORACLE_ADDR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OVM_GASPRICEORACLE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OP_MAINNET_CHAIN_ID' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OP_GOERLI_CHAIN_ID' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OP_SEPOLIA_CHAIN_ID' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BASE_MAINNET_CHAIN_ID' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BASE_GOERLI_CHAIN_ID' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BASE_SEPOLIA_CHAIN_ID' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'L2_TO_L1_MESSAGE_PASSER' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'L2_CROSS_DOMAIN_MESSENGER' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'L2_STANDARD_BRIDGE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'L2_ERC721_BRIDGE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SEQUENCER_FEE_WALLET' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OPTIMISM_MINTABLE_ERC20_FACTORY' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OPTIMISM_MINTABLE_ERC721_FACTORY' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'L1_BLOCK_ATTRIBUTES' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'GAS_PRICE_ORACLE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'L1_MESSAGE_SENDER' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'DEPLOYER_WHITELIST' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LEGACY_ERC20_ETH' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'L1_BLOCK_NUMBER' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LEGACY_MESSAGE_PASSER' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'PROXY_ADMIN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BASE_FEE_VAULT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'L1_FEE_VAULT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'GOVERNANCE_TOKEN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SCHEMA_REGISTRY' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'EAS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ARBGAS_ADDR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ARBGAS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ARB_MAINNET_CHAIN_ID' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ARB_GOERLI_TESTNET_CHAIN_ID' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ARB_SEPOLIA_TESTNET_CHAIN_ID' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'L1_FEE_DATA_PADDING' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OVM_GASPRICEORACLE_ADDR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OVM_GASPRICEORACLE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OP_MAINNET_CHAIN_ID' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OP_GOERLI_CHAIN_ID' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OP_SEPOLIA_CHAIN_ID' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BASE_MAINNET_CHAIN_ID' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BASE_GOERLI_CHAIN_ID' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BASE_SEPOLIA_CHAIN_ID' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'L2_TO_L1_MESSAGE_PASSER' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'L2_CROSS_DOMAIN_MESSENGER' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'L2_STANDARD_BRIDGE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'L2_ERC721_BRIDGE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SEQUENCER_FEE_WALLET' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OPTIMISM_MINTABLE_ERC20_FACTORY' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OPTIMISM_MINTABLE_ERC721_FACTORY' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'L1_BLOCK_ATTRIBUTES' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'GAS_PRICE_ORACLE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'L1_MESSAGE_SENDER' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'DEPLOYER_WHITELIST' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LEGACY_ERC20_ETH' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'L1_BLOCK_NUMBER' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LEGACY_MESSAGE_PASSER' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'PROXY_ADMIN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BASE_FEE_VAULT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'L1_FEE_VAULT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'GOVERNANCE_TOKEN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SCHEMA_REGISTRY' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'EAS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ARBGAS_ADDR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ARBGAS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ARB_MAINNET_CHAIN_ID' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ARB_GOERLI_TESTNET_CHAIN_ID' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ARB_SEPOLIA_TESTNET_CHAIN_ID' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'L1_FEE_DATA_PADDING' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OVM_GASPRICEORACLE_ADDR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OVM_GASPRICEORACLE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OP_MAINNET_CHAIN_ID' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OP_GOERLI_CHAIN_ID' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OP_SEPOLIA_CHAIN_ID' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BASE_MAINNET_CHAIN_ID' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BASE_GOERLI_CHAIN_ID' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BASE_SEPOLIA_CHAIN_ID' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'L2_TO_L1_MESSAGE_PASSER' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'L2_CROSS_DOMAIN_MESSENGER' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'L2_STANDARD_BRIDGE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'L2_ERC721_BRIDGE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SEQUENCER_FEE_WALLET' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OPTIMISM_MINTABLE_ERC20_FACTORY' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OPTIMISM_MINTABLE_ERC721_FACTORY' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'L1_BLOCK_ATTRIBUTES' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'GAS_PRICE_ORACLE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'L1_MESSAGE_SENDER' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'DEPLOYER_WHITELIST' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LEGACY_ERC20_ETH' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'L1_BLOCK_NUMBER' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LEGACY_MESSAGE_PASSER' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'PROXY_ADMIN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BASE_FEE_VAULT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'L1_FEE_VAULT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'GOVERNANCE_TOKEN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SCHEMA_REGISTRY' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'EAS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ARBGAS_ADDR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ARBGAS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ARB_MAINNET_CHAIN_ID' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ARB_GOERLI_TESTNET_CHAIN_ID' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ARB_SEPOLIA_TESTNET_CHAIN_ID' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'L1_FEE_DATA_PADDING' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OVM_GASPRICEORACLE_ADDR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OVM_GASPRICEORACLE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OP_MAINNET_CHAIN_ID' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OP_GOERLI_CHAIN_ID' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OP_SEPOLIA_CHAIN_ID' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BASE_MAINNET_CHAIN_ID' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BASE_GOERLI_CHAIN_ID' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BASE_SEPOLIA_CHAIN_ID' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'L2_TO_L1_MESSAGE_PASSER' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'L2_CROSS_DOMAIN_MESSENGER' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'L2_STANDARD_BRIDGE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'L2_ERC721_BRIDGE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SEQUENCER_FEE_WALLET' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OPTIMISM_MINTABLE_ERC20_FACTORY' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OPTIMISM_MINTABLE_ERC721_FACTORY' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'L1_BLOCK_ATTRIBUTES' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'GAS_PRICE_ORACLE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'L1_MESSAGE_SENDER' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'DEPLOYER_WHITELIST' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LEGACY_ERC20_ETH' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'L1_BLOCK_NUMBER' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'LEGACY_MESSAGE_PASSER' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'PROXY_ADMIN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BASE_FEE_VAULT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'L1_FEE_VAULT' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'GOVERNANCE_TOKEN' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'SCHEMA_REGISTRY' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'EAS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ARBGAS_ADDR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ARBGAS' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ARB_MAINNET_CHAIN_ID' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ARB_GOERLI_TESTNET_CHAIN_ID' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'ARB_SEPOLIA_TESTNET_CHAIN_ID' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'L1_FEE_DATA_PADDING' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OVM_GASPRICEORACLE_ADDR' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OVM_GASPRICEORACLE' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OP_MAINNET_CHAIN_ID' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OP_GOERLI_CHAIN_ID' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'OP_SEPOLIA_CHAIN_ID' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BASE_MAINNET_CHAIN_ID' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BASE_GOERLI_CHAIN_ID' detected while merging libraries |
| warning | W121 | duplicate constant state variable 'BASE_SEPOLIA_CHAIN_ID' detected while merging libraries |
| warning | W200 | function 'toUint248' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint248' which is not marked 'virtual' |
| warning | W200 | function 'toUint248' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint240' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint240' which is not marked 'virtual' |
| warning | W200 | function 'toUint240' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint232' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint232' which is not marked 'virtual' |
| warning | W200 | function 'toUint232' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint224' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint224' which is not marked 'virtual' |
| warning | W200 | function 'toUint224' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint216' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint216' which is not marked 'virtual' |
| warning | W200 | function 'toUint216' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint208' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint208' which is not marked 'virtual' |
| warning | W200 | function 'toUint208' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint200' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint200' which is not marked 'virtual' |
| warning | W200 | function 'toUint200' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint192' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint192' which is not marked 'virtual' |
| warning | W200 | function 'toUint192' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint184' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint184' which is not marked 'virtual' |
| warning | W200 | function 'toUint184' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint176' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint176' which is not marked 'virtual' |
| warning | W200 | function 'toUint176' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint168' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint168' which is not marked 'virtual' |
| warning | W200 | function 'toUint168' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint160' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint160' which is not marked 'virtual' |
| warning | W200 | function 'toUint160' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint152' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint152' which is not marked 'virtual' |
| warning | W200 | function 'toUint152' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint144' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint144' which is not marked 'virtual' |
| warning | W200 | function 'toUint144' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint136' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint136' which is not marked 'virtual' |
| warning | W200 | function 'toUint136' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint120' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint120' which is not marked 'virtual' |
| warning | W200 | function 'toUint120' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint112' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint112' which is not marked 'virtual' |
| warning | W200 | function 'toUint112' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint104' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint104' which is not marked 'virtual' |
| warning | W200 | function 'toUint104' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint96' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint96' which is not marked 'virtual' |
| warning | W200 | function 'toUint96' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint88' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint88' which is not marked 'virtual' |
| warning | W200 | function 'toUint88' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint80' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint80' which is not marked 'virtual' |
| warning | W200 | function 'toUint80' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint72' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint72' which is not marked 'virtual' |
| warning | W200 | function 'toUint72' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint64' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint64' which is not marked 'virtual' |
| warning | W200 | function 'toUint64' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint56' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint56' which is not marked 'virtual' |
| warning | W200 | function 'toUint56' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint48' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint48' which is not marked 'virtual' |
| warning | W200 | function 'toUint48' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint40' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint40' which is not marked 'virtual' |
| warning | W200 | function 'toUint40' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint32' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint32' which is not marked 'virtual' |
| warning | W200 | function 'toUint32' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint24' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint24' which is not marked 'virtual' |
| warning | W200 | function 'toUint24' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint16' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint16' which is not marked 'virtual' |
| warning | W200 | function 'toUint16' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint8' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint8' which is not marked 'virtual' |
| warning | W200 | function 'toUint8' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint256' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toUint256' which is not marked 'virtual' |
| warning | W200 | function 'toUint256' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt248' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt248' which is not marked 'virtual' |
| warning | W200 | function 'toInt248' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt240' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt240' which is not marked 'virtual' |
| warning | W200 | function 'toInt240' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt232' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt232' which is not marked 'virtual' |
| warning | W200 | function 'toInt232' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt224' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt224' which is not marked 'virtual' |
| warning | W200 | function 'toInt224' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt216' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt216' which is not marked 'virtual' |
| warning | W200 | function 'toInt216' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt208' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt208' which is not marked 'virtual' |
| warning | W200 | function 'toInt208' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt200' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt200' which is not marked 'virtual' |
| warning | W200 | function 'toInt200' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt192' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt192' which is not marked 'virtual' |
| warning | W200 | function 'toInt192' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt184' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt184' which is not marked 'virtual' |
| warning | W200 | function 'toInt184' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt176' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt176' which is not marked 'virtual' |
| warning | W200 | function 'toInt176' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt168' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt168' which is not marked 'virtual' |
| warning | W200 | function 'toInt168' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt160' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt160' which is not marked 'virtual' |
| warning | W200 | function 'toInt160' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt152' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt152' which is not marked 'virtual' |
| warning | W200 | function 'toInt152' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt144' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt144' which is not marked 'virtual' |
| warning | W200 | function 'toInt144' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt136' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt136' which is not marked 'virtual' |
| warning | W200 | function 'toInt136' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt120' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt120' which is not marked 'virtual' |
| warning | W200 | function 'toInt120' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt112' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt112' which is not marked 'virtual' |
| warning | W200 | function 'toInt112' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt104' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt104' which is not marked 'virtual' |
| warning | W200 | function 'toInt104' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt96' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt96' which is not marked 'virtual' |
| warning | W200 | function 'toInt96' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt88' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt88' which is not marked 'virtual' |
| warning | W200 | function 'toInt88' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt80' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt80' which is not marked 'virtual' |
| warning | W200 | function 'toInt80' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt72' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt72' which is not marked 'virtual' |
| warning | W200 | function 'toInt72' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt64' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt64' which is not marked 'virtual' |
| warning | W200 | function 'toInt64' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt56' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt56' which is not marked 'virtual' |
| warning | W200 | function 'toInt56' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt48' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt48' which is not marked 'virtual' |
| warning | W200 | function 'toInt48' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt40' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt40' which is not marked 'virtual' |
| warning | W200 | function 'toInt40' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt32' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt32' which is not marked 'virtual' |
| warning | W200 | function 'toInt32' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt24' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt24' which is not marked 'virtual' |
| warning | W200 | function 'toInt24' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt16' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt16' which is not marked 'virtual' |
| warning | W200 | function 'toInt16' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt8' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt8' which is not marked 'virtual' |
| warning | W200 | function 'toInt8' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt256' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::toInt256' which is not marked 'virtual' |
| warning | W200 | function 'toInt256' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getCurrentTxL1GasFees' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::_getCurrentTxL1GasFees' which is not marked 'virtual' |
| warning | W200 | function '_getCurrentTxL1GasFees' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function '_isArbitrumChainId' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::_isArbitrumChainId' which is not marked 'virtual' |
| warning | W200 | function '_isArbitrumChainId' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function '_isOptimismChainId' in 'ConfirmedOwner' overrides 'ConfirmedOwnerWithProposal::_isOptimismChainId' which is not marked 'virtual' |
| warning | W200 | function '_isOptimismChainId' in 'ConfirmedOwner' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint248' in 'OCR2Abstract' overrides 'ConfirmedOwner::toUint248' which is not marked 'virtual' |
| warning | W200 | function 'toUint248' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint240' in 'OCR2Abstract' overrides 'ConfirmedOwner::toUint240' which is not marked 'virtual' |
| warning | W200 | function 'toUint240' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint232' in 'OCR2Abstract' overrides 'ConfirmedOwner::toUint232' which is not marked 'virtual' |
| warning | W200 | function 'toUint232' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint224' in 'OCR2Abstract' overrides 'ConfirmedOwner::toUint224' which is not marked 'virtual' |
| warning | W200 | function 'toUint224' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint216' in 'OCR2Abstract' overrides 'ConfirmedOwner::toUint216' which is not marked 'virtual' |
| warning | W200 | function 'toUint216' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint208' in 'OCR2Abstract' overrides 'ConfirmedOwner::toUint208' which is not marked 'virtual' |
| warning | W200 | function 'toUint208' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint200' in 'OCR2Abstract' overrides 'ConfirmedOwner::toUint200' which is not marked 'virtual' |
| warning | W200 | function 'toUint200' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint192' in 'OCR2Abstract' overrides 'ConfirmedOwner::toUint192' which is not marked 'virtual' |
| warning | W200 | function 'toUint192' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint184' in 'OCR2Abstract' overrides 'ConfirmedOwner::toUint184' which is not marked 'virtual' |
| warning | W200 | function 'toUint184' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint176' in 'OCR2Abstract' overrides 'ConfirmedOwner::toUint176' which is not marked 'virtual' |
| warning | W200 | function 'toUint176' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint168' in 'OCR2Abstract' overrides 'ConfirmedOwner::toUint168' which is not marked 'virtual' |
| warning | W200 | function 'toUint168' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint160' in 'OCR2Abstract' overrides 'ConfirmedOwner::toUint160' which is not marked 'virtual' |
| warning | W200 | function 'toUint160' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint152' in 'OCR2Abstract' overrides 'ConfirmedOwner::toUint152' which is not marked 'virtual' |
| warning | W200 | function 'toUint152' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint144' in 'OCR2Abstract' overrides 'ConfirmedOwner::toUint144' which is not marked 'virtual' |
| warning | W200 | function 'toUint144' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint136' in 'OCR2Abstract' overrides 'ConfirmedOwner::toUint136' which is not marked 'virtual' |
| warning | W200 | function 'toUint136' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'OCR2Abstract' overrides 'ConfirmedOwner::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint120' in 'OCR2Abstract' overrides 'ConfirmedOwner::toUint120' which is not marked 'virtual' |
| warning | W200 | function 'toUint120' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint112' in 'OCR2Abstract' overrides 'ConfirmedOwner::toUint112' which is not marked 'virtual' |
| warning | W200 | function 'toUint112' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint104' in 'OCR2Abstract' overrides 'ConfirmedOwner::toUint104' which is not marked 'virtual' |
| warning | W200 | function 'toUint104' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint96' in 'OCR2Abstract' overrides 'ConfirmedOwner::toUint96' which is not marked 'virtual' |
| warning | W200 | function 'toUint96' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint88' in 'OCR2Abstract' overrides 'ConfirmedOwner::toUint88' which is not marked 'virtual' |
| warning | W200 | function 'toUint88' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint80' in 'OCR2Abstract' overrides 'ConfirmedOwner::toUint80' which is not marked 'virtual' |
| warning | W200 | function 'toUint80' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint72' in 'OCR2Abstract' overrides 'ConfirmedOwner::toUint72' which is not marked 'virtual' |
| warning | W200 | function 'toUint72' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint64' in 'OCR2Abstract' overrides 'ConfirmedOwner::toUint64' which is not marked 'virtual' |
| warning | W200 | function 'toUint64' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint56' in 'OCR2Abstract' overrides 'ConfirmedOwner::toUint56' which is not marked 'virtual' |
| warning | W200 | function 'toUint56' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint48' in 'OCR2Abstract' overrides 'ConfirmedOwner::toUint48' which is not marked 'virtual' |
| warning | W200 | function 'toUint48' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint40' in 'OCR2Abstract' overrides 'ConfirmedOwner::toUint40' which is not marked 'virtual' |
| warning | W200 | function 'toUint40' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint32' in 'OCR2Abstract' overrides 'ConfirmedOwner::toUint32' which is not marked 'virtual' |
| warning | W200 | function 'toUint32' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint24' in 'OCR2Abstract' overrides 'ConfirmedOwner::toUint24' which is not marked 'virtual' |
| warning | W200 | function 'toUint24' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint16' in 'OCR2Abstract' overrides 'ConfirmedOwner::toUint16' which is not marked 'virtual' |
| warning | W200 | function 'toUint16' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint8' in 'OCR2Abstract' overrides 'ConfirmedOwner::toUint8' which is not marked 'virtual' |
| warning | W200 | function 'toUint8' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint256' in 'OCR2Abstract' overrides 'ConfirmedOwner::toUint256' which is not marked 'virtual' |
| warning | W200 | function 'toUint256' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt248' in 'OCR2Abstract' overrides 'ConfirmedOwner::toInt248' which is not marked 'virtual' |
| warning | W200 | function 'toInt248' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt240' in 'OCR2Abstract' overrides 'ConfirmedOwner::toInt240' which is not marked 'virtual' |
| warning | W200 | function 'toInt240' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt232' in 'OCR2Abstract' overrides 'ConfirmedOwner::toInt232' which is not marked 'virtual' |
| warning | W200 | function 'toInt232' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt224' in 'OCR2Abstract' overrides 'ConfirmedOwner::toInt224' which is not marked 'virtual' |
| warning | W200 | function 'toInt224' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt216' in 'OCR2Abstract' overrides 'ConfirmedOwner::toInt216' which is not marked 'virtual' |
| warning | W200 | function 'toInt216' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt208' in 'OCR2Abstract' overrides 'ConfirmedOwner::toInt208' which is not marked 'virtual' |
| warning | W200 | function 'toInt208' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt200' in 'OCR2Abstract' overrides 'ConfirmedOwner::toInt200' which is not marked 'virtual' |
| warning | W200 | function 'toInt200' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt192' in 'OCR2Abstract' overrides 'ConfirmedOwner::toInt192' which is not marked 'virtual' |
| warning | W200 | function 'toInt192' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt184' in 'OCR2Abstract' overrides 'ConfirmedOwner::toInt184' which is not marked 'virtual' |
| warning | W200 | function 'toInt184' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt176' in 'OCR2Abstract' overrides 'ConfirmedOwner::toInt176' which is not marked 'virtual' |
| warning | W200 | function 'toInt176' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt168' in 'OCR2Abstract' overrides 'ConfirmedOwner::toInt168' which is not marked 'virtual' |
| warning | W200 | function 'toInt168' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt160' in 'OCR2Abstract' overrides 'ConfirmedOwner::toInt160' which is not marked 'virtual' |
| warning | W200 | function 'toInt160' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt152' in 'OCR2Abstract' overrides 'ConfirmedOwner::toInt152' which is not marked 'virtual' |
| warning | W200 | function 'toInt152' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt144' in 'OCR2Abstract' overrides 'ConfirmedOwner::toInt144' which is not marked 'virtual' |
| warning | W200 | function 'toInt144' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt136' in 'OCR2Abstract' overrides 'ConfirmedOwner::toInt136' which is not marked 'virtual' |
| warning | W200 | function 'toInt136' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'OCR2Abstract' overrides 'ConfirmedOwner::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt120' in 'OCR2Abstract' overrides 'ConfirmedOwner::toInt120' which is not marked 'virtual' |
| warning | W200 | function 'toInt120' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt112' in 'OCR2Abstract' overrides 'ConfirmedOwner::toInt112' which is not marked 'virtual' |
| warning | W200 | function 'toInt112' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt104' in 'OCR2Abstract' overrides 'ConfirmedOwner::toInt104' which is not marked 'virtual' |
| warning | W200 | function 'toInt104' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt96' in 'OCR2Abstract' overrides 'ConfirmedOwner::toInt96' which is not marked 'virtual' |
| warning | W200 | function 'toInt96' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt88' in 'OCR2Abstract' overrides 'ConfirmedOwner::toInt88' which is not marked 'virtual' |
| warning | W200 | function 'toInt88' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt80' in 'OCR2Abstract' overrides 'ConfirmedOwner::toInt80' which is not marked 'virtual' |
| warning | W200 | function 'toInt80' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt72' in 'OCR2Abstract' overrides 'ConfirmedOwner::toInt72' which is not marked 'virtual' |
| warning | W200 | function 'toInt72' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt64' in 'OCR2Abstract' overrides 'ConfirmedOwner::toInt64' which is not marked 'virtual' |
| warning | W200 | function 'toInt64' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt56' in 'OCR2Abstract' overrides 'ConfirmedOwner::toInt56' which is not marked 'virtual' |
| warning | W200 | function 'toInt56' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt48' in 'OCR2Abstract' overrides 'ConfirmedOwner::toInt48' which is not marked 'virtual' |
| warning | W200 | function 'toInt48' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt40' in 'OCR2Abstract' overrides 'ConfirmedOwner::toInt40' which is not marked 'virtual' |
| warning | W200 | function 'toInt40' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt32' in 'OCR2Abstract' overrides 'ConfirmedOwner::toInt32' which is not marked 'virtual' |
| warning | W200 | function 'toInt32' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt24' in 'OCR2Abstract' overrides 'ConfirmedOwner::toInt24' which is not marked 'virtual' |
| warning | W200 | function 'toInt24' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt16' in 'OCR2Abstract' overrides 'ConfirmedOwner::toInt16' which is not marked 'virtual' |
| warning | W200 | function 'toInt16' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt8' in 'OCR2Abstract' overrides 'ConfirmedOwner::toInt8' which is not marked 'virtual' |
| warning | W200 | function 'toInt8' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt256' in 'OCR2Abstract' overrides 'ConfirmedOwner::toInt256' which is not marked 'virtual' |
| warning | W200 | function 'toInt256' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getCurrentTxL1GasFees' in 'OCR2Abstract' overrides 'ConfirmedOwner::_getCurrentTxL1GasFees' which is not marked 'virtual' |
| warning | W200 | function '_getCurrentTxL1GasFees' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function '_isArbitrumChainId' in 'OCR2Abstract' overrides 'ConfirmedOwner::_isArbitrumChainId' which is not marked 'virtual' |
| warning | W200 | function '_isArbitrumChainId' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function '_isOptimismChainId' in 'OCR2Abstract' overrides 'ConfirmedOwner::_isOptimismChainId' which is not marked 'virtual' |
| warning | W200 | function '_isOptimismChainId' in 'OCR2Abstract' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint248' in 'OCR2Base' overrides 'OCR2Abstract::toUint248' which is not marked 'virtual' |
| warning | W200 | function 'toUint248' in 'OCR2Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint240' in 'OCR2Base' overrides 'OCR2Abstract::toUint240' which is not marked 'virtual' |
| warning | W200 | function 'toUint240' in 'OCR2Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint232' in 'OCR2Base' overrides 'OCR2Abstract::toUint232' which is not marked 'virtual' |
| warning | W200 | function 'toUint232' in 'OCR2Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint224' in 'OCR2Base' overrides 'OCR2Abstract::toUint224' which is not marked 'virtual' |
| warning | W200 | function 'toUint224' in 'OCR2Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint216' in 'OCR2Base' overrides 'OCR2Abstract::toUint216' which is not marked 'virtual' |
| warning | W200 | function 'toUint216' in 'OCR2Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint208' in 'OCR2Base' overrides 'OCR2Abstract::toUint208' which is not marked 'virtual' |
| warning | W200 | function 'toUint208' in 'OCR2Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint200' in 'OCR2Base' overrides 'OCR2Abstract::toUint200' which is not marked 'virtual' |
| warning | W200 | function 'toUint200' in 'OCR2Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint192' in 'OCR2Base' overrides 'OCR2Abstract::toUint192' which is not marked 'virtual' |
| warning | W200 | function 'toUint192' in 'OCR2Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint184' in 'OCR2Base' overrides 'OCR2Abstract::toUint184' which is not marked 'virtual' |
| warning | W200 | function 'toUint184' in 'OCR2Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint176' in 'OCR2Base' overrides 'OCR2Abstract::toUint176' which is not marked 'virtual' |
| warning | W200 | function 'toUint176' in 'OCR2Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint168' in 'OCR2Base' overrides 'OCR2Abstract::toUint168' which is not marked 'virtual' |
| warning | W200 | function 'toUint168' in 'OCR2Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint160' in 'OCR2Base' overrides 'OCR2Abstract::toUint160' which is not marked 'virtual' |
| warning | W200 | function 'toUint160' in 'OCR2Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint152' in 'OCR2Base' overrides 'OCR2Abstract::toUint152' which is not marked 'virtual' |
| warning | W200 | function 'toUint152' in 'OCR2Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint144' in 'OCR2Base' overrides 'OCR2Abstract::toUint144' which is not marked 'virtual' |
| warning | W200 | function 'toUint144' in 'OCR2Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint136' in 'OCR2Base' overrides 'OCR2Abstract::toUint136' which is not marked 'virtual' |
| warning | W200 | function 'toUint136' in 'OCR2Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'OCR2Base' overrides 'OCR2Abstract::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'OCR2Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint120' in 'OCR2Base' overrides 'OCR2Abstract::toUint120' which is not marked 'virtual' |
| warning | W200 | function 'toUint120' in 'OCR2Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint112' in 'OCR2Base' overrides 'OCR2Abstract::toUint112' which is not marked 'virtual' |
| warning | W200 | function 'toUint112' in 'OCR2Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint104' in 'OCR2Base' overrides 'OCR2Abstract::toUint104' which is not marked 'virtual' |
| warning | W200 | function 'toUint104' in 'OCR2Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint96' in 'OCR2Base' overrides 'OCR2Abstract::toUint96' which is not marked 'virtual' |
| warning | W200 | function 'toUint96' in 'OCR2Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint88' in 'OCR2Base' overrides 'OCR2Abstract::toUint88' which is not marked 'virtual' |
| warning | W200 | function 'toUint88' in 'OCR2Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint80' in 'OCR2Base' overrides 'OCR2Abstract::toUint80' which is not marked 'virtual' |
| warning | W200 | function 'toUint80' in 'OCR2Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint72' in 'OCR2Base' overrides 'OCR2Abstract::toUint72' which is not marked 'virtual' |
| warning | W200 | function 'toUint72' in 'OCR2Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint64' in 'OCR2Base' overrides 'OCR2Abstract::toUint64' which is not marked 'virtual' |
| warning | W200 | function 'toUint64' in 'OCR2Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint56' in 'OCR2Base' overrides 'OCR2Abstract::toUint56' which is not marked 'virtual' |
| warning | W200 | function 'toUint56' in 'OCR2Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint48' in 'OCR2Base' overrides 'OCR2Abstract::toUint48' which is not marked 'virtual' |
| warning | W200 | function 'toUint48' in 'OCR2Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint40' in 'OCR2Base' overrides 'OCR2Abstract::toUint40' which is not marked 'virtual' |
| warning | W200 | function 'toUint40' in 'OCR2Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint32' in 'OCR2Base' overrides 'OCR2Abstract::toUint32' which is not marked 'virtual' |
| warning | W200 | function 'toUint32' in 'OCR2Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint24' in 'OCR2Base' overrides 'OCR2Abstract::toUint24' which is not marked 'virtual' |
| warning | W200 | function 'toUint24' in 'OCR2Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint16' in 'OCR2Base' overrides 'OCR2Abstract::toUint16' which is not marked 'virtual' |
| warning | W200 | function 'toUint16' in 'OCR2Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint8' in 'OCR2Base' overrides 'OCR2Abstract::toUint8' which is not marked 'virtual' |
| warning | W200 | function 'toUint8' in 'OCR2Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint256' in 'OCR2Base' overrides 'OCR2Abstract::toUint256' which is not marked 'virtual' |
| warning | W200 | function 'toUint256' in 'OCR2Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt248' in 'OCR2Base' overrides 'OCR2Abstract::toInt248' which is not marked 'virtual' |
| warning | W200 | function 'toInt248' in 'OCR2Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt240' in 'OCR2Base' overrides 'OCR2Abstract::toInt240' which is not marked 'virtual' |
| warning | W200 | function 'toInt240' in 'OCR2Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt232' in 'OCR2Base' overrides 'OCR2Abstract::toInt232' which is not marked 'virtual' |
| warning | W200 | function 'toInt232' in 'OCR2Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt224' in 'OCR2Base' overrides 'OCR2Abstract::toInt224' which is not marked 'virtual' |
| warning | W200 | function 'toInt224' in 'OCR2Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt216' in 'OCR2Base' overrides 'OCR2Abstract::toInt216' which is not marked 'virtual' |
| warning | W200 | function 'toInt216' in 'OCR2Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt208' in 'OCR2Base' overrides 'OCR2Abstract::toInt208' which is not marked 'virtual' |
| warning | W200 | function 'toInt208' in 'OCR2Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt200' in 'OCR2Base' overrides 'OCR2Abstract::toInt200' which is not marked 'virtual' |
| warning | W200 | function 'toInt200' in 'OCR2Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt192' in 'OCR2Base' overrides 'OCR2Abstract::toInt192' which is not marked 'virtual' |
| warning | W200 | function 'toInt192' in 'OCR2Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt184' in 'OCR2Base' overrides 'OCR2Abstract::toInt184' which is not marked 'virtual' |
| warning | W200 | function 'toInt184' in 'OCR2Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt176' in 'OCR2Base' overrides 'OCR2Abstract::toInt176' which is not marked 'virtual' |
| warning | W200 | function 'toInt176' in 'OCR2Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt168' in 'OCR2Base' overrides 'OCR2Abstract::toInt168' which is not marked 'virtual' |
| warning | W200 | function 'toInt168' in 'OCR2Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt160' in 'OCR2Base' overrides 'OCR2Abstract::toInt160' which is not marked 'virtual' |
| warning | W200 | function 'toInt160' in 'OCR2Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt152' in 'OCR2Base' overrides 'OCR2Abstract::toInt152' which is not marked 'virtual' |
| warning | W200 | function 'toInt152' in 'OCR2Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt144' in 'OCR2Base' overrides 'OCR2Abstract::toInt144' which is not marked 'virtual' |
| warning | W200 | function 'toInt144' in 'OCR2Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt136' in 'OCR2Base' overrides 'OCR2Abstract::toInt136' which is not marked 'virtual' |
| warning | W200 | function 'toInt136' in 'OCR2Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'OCR2Base' overrides 'OCR2Abstract::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'OCR2Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt120' in 'OCR2Base' overrides 'OCR2Abstract::toInt120' which is not marked 'virtual' |
| warning | W200 | function 'toInt120' in 'OCR2Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt112' in 'OCR2Base' overrides 'OCR2Abstract::toInt112' which is not marked 'virtual' |
| warning | W200 | function 'toInt112' in 'OCR2Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt104' in 'OCR2Base' overrides 'OCR2Abstract::toInt104' which is not marked 'virtual' |
| warning | W200 | function 'toInt104' in 'OCR2Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt96' in 'OCR2Base' overrides 'OCR2Abstract::toInt96' which is not marked 'virtual' |
| warning | W200 | function 'toInt96' in 'OCR2Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt88' in 'OCR2Base' overrides 'OCR2Abstract::toInt88' which is not marked 'virtual' |
| warning | W200 | function 'toInt88' in 'OCR2Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt80' in 'OCR2Base' overrides 'OCR2Abstract::toInt80' which is not marked 'virtual' |
| warning | W200 | function 'toInt80' in 'OCR2Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt72' in 'OCR2Base' overrides 'OCR2Abstract::toInt72' which is not marked 'virtual' |
| warning | W200 | function 'toInt72' in 'OCR2Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt64' in 'OCR2Base' overrides 'OCR2Abstract::toInt64' which is not marked 'virtual' |
| warning | W200 | function 'toInt64' in 'OCR2Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt56' in 'OCR2Base' overrides 'OCR2Abstract::toInt56' which is not marked 'virtual' |
| warning | W200 | function 'toInt56' in 'OCR2Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt48' in 'OCR2Base' overrides 'OCR2Abstract::toInt48' which is not marked 'virtual' |
| warning | W200 | function 'toInt48' in 'OCR2Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt40' in 'OCR2Base' overrides 'OCR2Abstract::toInt40' which is not marked 'virtual' |
| warning | W200 | function 'toInt40' in 'OCR2Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt32' in 'OCR2Base' overrides 'OCR2Abstract::toInt32' which is not marked 'virtual' |
| warning | W200 | function 'toInt32' in 'OCR2Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt24' in 'OCR2Base' overrides 'OCR2Abstract::toInt24' which is not marked 'virtual' |
| warning | W200 | function 'toInt24' in 'OCR2Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt16' in 'OCR2Base' overrides 'OCR2Abstract::toInt16' which is not marked 'virtual' |
| warning | W200 | function 'toInt16' in 'OCR2Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt8' in 'OCR2Base' overrides 'OCR2Abstract::toInt8' which is not marked 'virtual' |
| warning | W200 | function 'toInt8' in 'OCR2Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt256' in 'OCR2Base' overrides 'OCR2Abstract::toInt256' which is not marked 'virtual' |
| warning | W200 | function 'toInt256' in 'OCR2Base' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getCurrentTxL1GasFees' in 'OCR2Base' overrides 'OCR2Abstract::_getCurrentTxL1GasFees' which is not marked 'virtual' |
| warning | W200 | function '_getCurrentTxL1GasFees' in 'OCR2Base' overrides a base function but is not marked 'override' |
| warning | W200 | function '_isArbitrumChainId' in 'OCR2Base' overrides 'OCR2Abstract::_isArbitrumChainId' which is not marked 'virtual' |
| warning | W200 | function '_isArbitrumChainId' in 'OCR2Base' overrides a base function but is not marked 'override' |
| warning | W200 | function '_isOptimismChainId' in 'OCR2Base' overrides 'OCR2Abstract::_isOptimismChainId' which is not marked 'virtual' |
| warning | W200 | function '_isOptimismChainId' in 'OCR2Base' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint248' in 'Routable' overrides 'OCR2Base::toUint248' which is not marked 'virtual' |
| warning | W200 | function 'toUint248' in 'Routable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint240' in 'Routable' overrides 'OCR2Base::toUint240' which is not marked 'virtual' |
| warning | W200 | function 'toUint240' in 'Routable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint232' in 'Routable' overrides 'OCR2Base::toUint232' which is not marked 'virtual' |
| warning | W200 | function 'toUint232' in 'Routable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint224' in 'Routable' overrides 'OCR2Base::toUint224' which is not marked 'virtual' |
| warning | W200 | function 'toUint224' in 'Routable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint216' in 'Routable' overrides 'OCR2Base::toUint216' which is not marked 'virtual' |
| warning | W200 | function 'toUint216' in 'Routable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint208' in 'Routable' overrides 'OCR2Base::toUint208' which is not marked 'virtual' |
| warning | W200 | function 'toUint208' in 'Routable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint200' in 'Routable' overrides 'OCR2Base::toUint200' which is not marked 'virtual' |
| warning | W200 | function 'toUint200' in 'Routable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint192' in 'Routable' overrides 'OCR2Base::toUint192' which is not marked 'virtual' |
| warning | W200 | function 'toUint192' in 'Routable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint184' in 'Routable' overrides 'OCR2Base::toUint184' which is not marked 'virtual' |
| warning | W200 | function 'toUint184' in 'Routable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint176' in 'Routable' overrides 'OCR2Base::toUint176' which is not marked 'virtual' |
| warning | W200 | function 'toUint176' in 'Routable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint168' in 'Routable' overrides 'OCR2Base::toUint168' which is not marked 'virtual' |
| warning | W200 | function 'toUint168' in 'Routable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint160' in 'Routable' overrides 'OCR2Base::toUint160' which is not marked 'virtual' |
| warning | W200 | function 'toUint160' in 'Routable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint152' in 'Routable' overrides 'OCR2Base::toUint152' which is not marked 'virtual' |
| warning | W200 | function 'toUint152' in 'Routable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint144' in 'Routable' overrides 'OCR2Base::toUint144' which is not marked 'virtual' |
| warning | W200 | function 'toUint144' in 'Routable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint136' in 'Routable' overrides 'OCR2Base::toUint136' which is not marked 'virtual' |
| warning | W200 | function 'toUint136' in 'Routable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'Routable' overrides 'OCR2Base::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'Routable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint120' in 'Routable' overrides 'OCR2Base::toUint120' which is not marked 'virtual' |
| warning | W200 | function 'toUint120' in 'Routable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint112' in 'Routable' overrides 'OCR2Base::toUint112' which is not marked 'virtual' |
| warning | W200 | function 'toUint112' in 'Routable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint104' in 'Routable' overrides 'OCR2Base::toUint104' which is not marked 'virtual' |
| warning | W200 | function 'toUint104' in 'Routable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint96' in 'Routable' overrides 'OCR2Base::toUint96' which is not marked 'virtual' |
| warning | W200 | function 'toUint96' in 'Routable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint88' in 'Routable' overrides 'OCR2Base::toUint88' which is not marked 'virtual' |
| warning | W200 | function 'toUint88' in 'Routable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint80' in 'Routable' overrides 'OCR2Base::toUint80' which is not marked 'virtual' |
| warning | W200 | function 'toUint80' in 'Routable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint72' in 'Routable' overrides 'OCR2Base::toUint72' which is not marked 'virtual' |
| warning | W200 | function 'toUint72' in 'Routable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint64' in 'Routable' overrides 'OCR2Base::toUint64' which is not marked 'virtual' |
| warning | W200 | function 'toUint64' in 'Routable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint56' in 'Routable' overrides 'OCR2Base::toUint56' which is not marked 'virtual' |
| warning | W200 | function 'toUint56' in 'Routable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint48' in 'Routable' overrides 'OCR2Base::toUint48' which is not marked 'virtual' |
| warning | W200 | function 'toUint48' in 'Routable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint40' in 'Routable' overrides 'OCR2Base::toUint40' which is not marked 'virtual' |
| warning | W200 | function 'toUint40' in 'Routable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint32' in 'Routable' overrides 'OCR2Base::toUint32' which is not marked 'virtual' |
| warning | W200 | function 'toUint32' in 'Routable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint24' in 'Routable' overrides 'OCR2Base::toUint24' which is not marked 'virtual' |
| warning | W200 | function 'toUint24' in 'Routable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint16' in 'Routable' overrides 'OCR2Base::toUint16' which is not marked 'virtual' |
| warning | W200 | function 'toUint16' in 'Routable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint8' in 'Routable' overrides 'OCR2Base::toUint8' which is not marked 'virtual' |
| warning | W200 | function 'toUint8' in 'Routable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint256' in 'Routable' overrides 'OCR2Base::toUint256' which is not marked 'virtual' |
| warning | W200 | function 'toUint256' in 'Routable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt248' in 'Routable' overrides 'OCR2Base::toInt248' which is not marked 'virtual' |
| warning | W200 | function 'toInt248' in 'Routable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt240' in 'Routable' overrides 'OCR2Base::toInt240' which is not marked 'virtual' |
| warning | W200 | function 'toInt240' in 'Routable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt232' in 'Routable' overrides 'OCR2Base::toInt232' which is not marked 'virtual' |
| warning | W200 | function 'toInt232' in 'Routable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt224' in 'Routable' overrides 'OCR2Base::toInt224' which is not marked 'virtual' |
| warning | W200 | function 'toInt224' in 'Routable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt216' in 'Routable' overrides 'OCR2Base::toInt216' which is not marked 'virtual' |
| warning | W200 | function 'toInt216' in 'Routable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt208' in 'Routable' overrides 'OCR2Base::toInt208' which is not marked 'virtual' |
| warning | W200 | function 'toInt208' in 'Routable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt200' in 'Routable' overrides 'OCR2Base::toInt200' which is not marked 'virtual' |
| warning | W200 | function 'toInt200' in 'Routable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt192' in 'Routable' overrides 'OCR2Base::toInt192' which is not marked 'virtual' |
| warning | W200 | function 'toInt192' in 'Routable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt184' in 'Routable' overrides 'OCR2Base::toInt184' which is not marked 'virtual' |
| warning | W200 | function 'toInt184' in 'Routable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt176' in 'Routable' overrides 'OCR2Base::toInt176' which is not marked 'virtual' |
| warning | W200 | function 'toInt176' in 'Routable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt168' in 'Routable' overrides 'OCR2Base::toInt168' which is not marked 'virtual' |
| warning | W200 | function 'toInt168' in 'Routable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt160' in 'Routable' overrides 'OCR2Base::toInt160' which is not marked 'virtual' |
| warning | W200 | function 'toInt160' in 'Routable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt152' in 'Routable' overrides 'OCR2Base::toInt152' which is not marked 'virtual' |
| warning | W200 | function 'toInt152' in 'Routable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt144' in 'Routable' overrides 'OCR2Base::toInt144' which is not marked 'virtual' |
| warning | W200 | function 'toInt144' in 'Routable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt136' in 'Routable' overrides 'OCR2Base::toInt136' which is not marked 'virtual' |
| warning | W200 | function 'toInt136' in 'Routable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'Routable' overrides 'OCR2Base::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'Routable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt120' in 'Routable' overrides 'OCR2Base::toInt120' which is not marked 'virtual' |
| warning | W200 | function 'toInt120' in 'Routable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt112' in 'Routable' overrides 'OCR2Base::toInt112' which is not marked 'virtual' |
| warning | W200 | function 'toInt112' in 'Routable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt104' in 'Routable' overrides 'OCR2Base::toInt104' which is not marked 'virtual' |
| warning | W200 | function 'toInt104' in 'Routable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt96' in 'Routable' overrides 'OCR2Base::toInt96' which is not marked 'virtual' |
| warning | W200 | function 'toInt96' in 'Routable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt88' in 'Routable' overrides 'OCR2Base::toInt88' which is not marked 'virtual' |
| warning | W200 | function 'toInt88' in 'Routable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt80' in 'Routable' overrides 'OCR2Base::toInt80' which is not marked 'virtual' |
| warning | W200 | function 'toInt80' in 'Routable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt72' in 'Routable' overrides 'OCR2Base::toInt72' which is not marked 'virtual' |
| warning | W200 | function 'toInt72' in 'Routable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt64' in 'Routable' overrides 'OCR2Base::toInt64' which is not marked 'virtual' |
| warning | W200 | function 'toInt64' in 'Routable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt56' in 'Routable' overrides 'OCR2Base::toInt56' which is not marked 'virtual' |
| warning | W200 | function 'toInt56' in 'Routable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt48' in 'Routable' overrides 'OCR2Base::toInt48' which is not marked 'virtual' |
| warning | W200 | function 'toInt48' in 'Routable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt40' in 'Routable' overrides 'OCR2Base::toInt40' which is not marked 'virtual' |
| warning | W200 | function 'toInt40' in 'Routable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt32' in 'Routable' overrides 'OCR2Base::toInt32' which is not marked 'virtual' |
| warning | W200 | function 'toInt32' in 'Routable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt24' in 'Routable' overrides 'OCR2Base::toInt24' which is not marked 'virtual' |
| warning | W200 | function 'toInt24' in 'Routable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt16' in 'Routable' overrides 'OCR2Base::toInt16' which is not marked 'virtual' |
| warning | W200 | function 'toInt16' in 'Routable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt8' in 'Routable' overrides 'OCR2Base::toInt8' which is not marked 'virtual' |
| warning | W200 | function 'toInt8' in 'Routable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt256' in 'Routable' overrides 'OCR2Base::toInt256' which is not marked 'virtual' |
| warning | W200 | function 'toInt256' in 'Routable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getCurrentTxL1GasFees' in 'Routable' overrides 'OCR2Base::_getCurrentTxL1GasFees' which is not marked 'virtual' |
| warning | W200 | function '_getCurrentTxL1GasFees' in 'Routable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_isArbitrumChainId' in 'Routable' overrides 'OCR2Base::_isArbitrumChainId' which is not marked 'virtual' |
| warning | W200 | function '_isArbitrumChainId' in 'Routable' overrides a base function but is not marked 'override' |
| warning | W200 | function '_isOptimismChainId' in 'Routable' overrides 'OCR2Base::_isOptimismChainId' which is not marked 'virtual' |
| warning | W200 | function '_isOptimismChainId' in 'Routable' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint248' in 'FunctionsBilling' overrides 'Routable::toUint248' which is not marked 'virtual' |
| warning | W200 | function 'toUint248' in 'FunctionsBilling' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint240' in 'FunctionsBilling' overrides 'Routable::toUint240' which is not marked 'virtual' |
| warning | W200 | function 'toUint240' in 'FunctionsBilling' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint232' in 'FunctionsBilling' overrides 'Routable::toUint232' which is not marked 'virtual' |
| warning | W200 | function 'toUint232' in 'FunctionsBilling' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint224' in 'FunctionsBilling' overrides 'Routable::toUint224' which is not marked 'virtual' |
| warning | W200 | function 'toUint224' in 'FunctionsBilling' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint216' in 'FunctionsBilling' overrides 'Routable::toUint216' which is not marked 'virtual' |
| warning | W200 | function 'toUint216' in 'FunctionsBilling' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint208' in 'FunctionsBilling' overrides 'Routable::toUint208' which is not marked 'virtual' |
| warning | W200 | function 'toUint208' in 'FunctionsBilling' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint200' in 'FunctionsBilling' overrides 'Routable::toUint200' which is not marked 'virtual' |
| warning | W200 | function 'toUint200' in 'FunctionsBilling' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint192' in 'FunctionsBilling' overrides 'Routable::toUint192' which is not marked 'virtual' |
| warning | W200 | function 'toUint192' in 'FunctionsBilling' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint184' in 'FunctionsBilling' overrides 'Routable::toUint184' which is not marked 'virtual' |
| warning | W200 | function 'toUint184' in 'FunctionsBilling' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint176' in 'FunctionsBilling' overrides 'Routable::toUint176' which is not marked 'virtual' |
| warning | W200 | function 'toUint176' in 'FunctionsBilling' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint168' in 'FunctionsBilling' overrides 'Routable::toUint168' which is not marked 'virtual' |
| warning | W200 | function 'toUint168' in 'FunctionsBilling' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint160' in 'FunctionsBilling' overrides 'Routable::toUint160' which is not marked 'virtual' |
| warning | W200 | function 'toUint160' in 'FunctionsBilling' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint152' in 'FunctionsBilling' overrides 'Routable::toUint152' which is not marked 'virtual' |
| warning | W200 | function 'toUint152' in 'FunctionsBilling' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint144' in 'FunctionsBilling' overrides 'Routable::toUint144' which is not marked 'virtual' |
| warning | W200 | function 'toUint144' in 'FunctionsBilling' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint136' in 'FunctionsBilling' overrides 'Routable::toUint136' which is not marked 'virtual' |
| warning | W200 | function 'toUint136' in 'FunctionsBilling' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'FunctionsBilling' overrides 'Routable::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'FunctionsBilling' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint120' in 'FunctionsBilling' overrides 'Routable::toUint120' which is not marked 'virtual' |
| warning | W200 | function 'toUint120' in 'FunctionsBilling' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint112' in 'FunctionsBilling' overrides 'Routable::toUint112' which is not marked 'virtual' |
| warning | W200 | function 'toUint112' in 'FunctionsBilling' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint104' in 'FunctionsBilling' overrides 'Routable::toUint104' which is not marked 'virtual' |
| warning | W200 | function 'toUint104' in 'FunctionsBilling' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint96' in 'FunctionsBilling' overrides 'Routable::toUint96' which is not marked 'virtual' |
| warning | W200 | function 'toUint96' in 'FunctionsBilling' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint88' in 'FunctionsBilling' overrides 'Routable::toUint88' which is not marked 'virtual' |
| warning | W200 | function 'toUint88' in 'FunctionsBilling' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint80' in 'FunctionsBilling' overrides 'Routable::toUint80' which is not marked 'virtual' |
| warning | W200 | function 'toUint80' in 'FunctionsBilling' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint72' in 'FunctionsBilling' overrides 'Routable::toUint72' which is not marked 'virtual' |
| warning | W200 | function 'toUint72' in 'FunctionsBilling' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint64' in 'FunctionsBilling' overrides 'Routable::toUint64' which is not marked 'virtual' |
| warning | W200 | function 'toUint64' in 'FunctionsBilling' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint56' in 'FunctionsBilling' overrides 'Routable::toUint56' which is not marked 'virtual' |
| warning | W200 | function 'toUint56' in 'FunctionsBilling' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint48' in 'FunctionsBilling' overrides 'Routable::toUint48' which is not marked 'virtual' |
| warning | W200 | function 'toUint48' in 'FunctionsBilling' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint40' in 'FunctionsBilling' overrides 'Routable::toUint40' which is not marked 'virtual' |
| warning | W200 | function 'toUint40' in 'FunctionsBilling' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint32' in 'FunctionsBilling' overrides 'Routable::toUint32' which is not marked 'virtual' |
| warning | W200 | function 'toUint32' in 'FunctionsBilling' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint24' in 'FunctionsBilling' overrides 'Routable::toUint24' which is not marked 'virtual' |
| warning | W200 | function 'toUint24' in 'FunctionsBilling' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint16' in 'FunctionsBilling' overrides 'Routable::toUint16' which is not marked 'virtual' |
| warning | W200 | function 'toUint16' in 'FunctionsBilling' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint8' in 'FunctionsBilling' overrides 'Routable::toUint8' which is not marked 'virtual' |
| warning | W200 | function 'toUint8' in 'FunctionsBilling' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint256' in 'FunctionsBilling' overrides 'Routable::toUint256' which is not marked 'virtual' |
| warning | W200 | function 'toUint256' in 'FunctionsBilling' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt248' in 'FunctionsBilling' overrides 'Routable::toInt248' which is not marked 'virtual' |
| warning | W200 | function 'toInt248' in 'FunctionsBilling' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt240' in 'FunctionsBilling' overrides 'Routable::toInt240' which is not marked 'virtual' |
| warning | W200 | function 'toInt240' in 'FunctionsBilling' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt232' in 'FunctionsBilling' overrides 'Routable::toInt232' which is not marked 'virtual' |
| warning | W200 | function 'toInt232' in 'FunctionsBilling' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt224' in 'FunctionsBilling' overrides 'Routable::toInt224' which is not marked 'virtual' |
| warning | W200 | function 'toInt224' in 'FunctionsBilling' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt216' in 'FunctionsBilling' overrides 'Routable::toInt216' which is not marked 'virtual' |
| warning | W200 | function 'toInt216' in 'FunctionsBilling' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt208' in 'FunctionsBilling' overrides 'Routable::toInt208' which is not marked 'virtual' |
| warning | W200 | function 'toInt208' in 'FunctionsBilling' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt200' in 'FunctionsBilling' overrides 'Routable::toInt200' which is not marked 'virtual' |
| warning | W200 | function 'toInt200' in 'FunctionsBilling' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt192' in 'FunctionsBilling' overrides 'Routable::toInt192' which is not marked 'virtual' |
| warning | W200 | function 'toInt192' in 'FunctionsBilling' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt184' in 'FunctionsBilling' overrides 'Routable::toInt184' which is not marked 'virtual' |
| warning | W200 | function 'toInt184' in 'FunctionsBilling' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt176' in 'FunctionsBilling' overrides 'Routable::toInt176' which is not marked 'virtual' |
| warning | W200 | function 'toInt176' in 'FunctionsBilling' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt168' in 'FunctionsBilling' overrides 'Routable::toInt168' which is not marked 'virtual' |
| warning | W200 | function 'toInt168' in 'FunctionsBilling' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt160' in 'FunctionsBilling' overrides 'Routable::toInt160' which is not marked 'virtual' |
| warning | W200 | function 'toInt160' in 'FunctionsBilling' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt152' in 'FunctionsBilling' overrides 'Routable::toInt152' which is not marked 'virtual' |
| warning | W200 | function 'toInt152' in 'FunctionsBilling' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt144' in 'FunctionsBilling' overrides 'Routable::toInt144' which is not marked 'virtual' |
| warning | W200 | function 'toInt144' in 'FunctionsBilling' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt136' in 'FunctionsBilling' overrides 'Routable::toInt136' which is not marked 'virtual' |
| warning | W200 | function 'toInt136' in 'FunctionsBilling' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'FunctionsBilling' overrides 'Routable::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'FunctionsBilling' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt120' in 'FunctionsBilling' overrides 'Routable::toInt120' which is not marked 'virtual' |
| warning | W200 | function 'toInt120' in 'FunctionsBilling' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt112' in 'FunctionsBilling' overrides 'Routable::toInt112' which is not marked 'virtual' |
| warning | W200 | function 'toInt112' in 'FunctionsBilling' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt104' in 'FunctionsBilling' overrides 'Routable::toInt104' which is not marked 'virtual' |
| warning | W200 | function 'toInt104' in 'FunctionsBilling' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt96' in 'FunctionsBilling' overrides 'Routable::toInt96' which is not marked 'virtual' |
| warning | W200 | function 'toInt96' in 'FunctionsBilling' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt88' in 'FunctionsBilling' overrides 'Routable::toInt88' which is not marked 'virtual' |
| warning | W200 | function 'toInt88' in 'FunctionsBilling' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt80' in 'FunctionsBilling' overrides 'Routable::toInt80' which is not marked 'virtual' |
| warning | W200 | function 'toInt80' in 'FunctionsBilling' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt72' in 'FunctionsBilling' overrides 'Routable::toInt72' which is not marked 'virtual' |
| warning | W200 | function 'toInt72' in 'FunctionsBilling' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt64' in 'FunctionsBilling' overrides 'Routable::toInt64' which is not marked 'virtual' |
| warning | W200 | function 'toInt64' in 'FunctionsBilling' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt56' in 'FunctionsBilling' overrides 'Routable::toInt56' which is not marked 'virtual' |
| warning | W200 | function 'toInt56' in 'FunctionsBilling' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt48' in 'FunctionsBilling' overrides 'Routable::toInt48' which is not marked 'virtual' |
| warning | W200 | function 'toInt48' in 'FunctionsBilling' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt40' in 'FunctionsBilling' overrides 'Routable::toInt40' which is not marked 'virtual' |
| warning | W200 | function 'toInt40' in 'FunctionsBilling' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt32' in 'FunctionsBilling' overrides 'Routable::toInt32' which is not marked 'virtual' |
| warning | W200 | function 'toInt32' in 'FunctionsBilling' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt24' in 'FunctionsBilling' overrides 'Routable::toInt24' which is not marked 'virtual' |
| warning | W200 | function 'toInt24' in 'FunctionsBilling' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt16' in 'FunctionsBilling' overrides 'Routable::toInt16' which is not marked 'virtual' |
| warning | W200 | function 'toInt16' in 'FunctionsBilling' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt8' in 'FunctionsBilling' overrides 'Routable::toInt8' which is not marked 'virtual' |
| warning | W200 | function 'toInt8' in 'FunctionsBilling' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt256' in 'FunctionsBilling' overrides 'Routable::toInt256' which is not marked 'virtual' |
| warning | W200 | function 'toInt256' in 'FunctionsBilling' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getCurrentTxL1GasFees' in 'FunctionsBilling' overrides 'Routable::_getCurrentTxL1GasFees' which is not marked 'virtual' |
| warning | W200 | function '_getCurrentTxL1GasFees' in 'FunctionsBilling' overrides a base function but is not marked 'override' |
| warning | W200 | function '_isArbitrumChainId' in 'FunctionsBilling' overrides 'Routable::_isArbitrumChainId' which is not marked 'virtual' |
| warning | W200 | function '_isArbitrumChainId' in 'FunctionsBilling' overrides a base function but is not marked 'override' |
| warning | W200 | function '_isOptimismChainId' in 'FunctionsBilling' overrides 'Routable::_isOptimismChainId' which is not marked 'virtual' |
| warning | W200 | function '_isOptimismChainId' in 'FunctionsBilling' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint248' in 'FunctionsCoordinator' overrides 'FunctionsBilling::toUint248' which is not marked 'virtual' |
| warning | W200 | function 'toUint248' in 'FunctionsCoordinator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint240' in 'FunctionsCoordinator' overrides 'FunctionsBilling::toUint240' which is not marked 'virtual' |
| warning | W200 | function 'toUint240' in 'FunctionsCoordinator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint232' in 'FunctionsCoordinator' overrides 'FunctionsBilling::toUint232' which is not marked 'virtual' |
| warning | W200 | function 'toUint232' in 'FunctionsCoordinator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint224' in 'FunctionsCoordinator' overrides 'FunctionsBilling::toUint224' which is not marked 'virtual' |
| warning | W200 | function 'toUint224' in 'FunctionsCoordinator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint216' in 'FunctionsCoordinator' overrides 'FunctionsBilling::toUint216' which is not marked 'virtual' |
| warning | W200 | function 'toUint216' in 'FunctionsCoordinator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint208' in 'FunctionsCoordinator' overrides 'FunctionsBilling::toUint208' which is not marked 'virtual' |
| warning | W200 | function 'toUint208' in 'FunctionsCoordinator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint200' in 'FunctionsCoordinator' overrides 'FunctionsBilling::toUint200' which is not marked 'virtual' |
| warning | W200 | function 'toUint200' in 'FunctionsCoordinator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint192' in 'FunctionsCoordinator' overrides 'FunctionsBilling::toUint192' which is not marked 'virtual' |
| warning | W200 | function 'toUint192' in 'FunctionsCoordinator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint184' in 'FunctionsCoordinator' overrides 'FunctionsBilling::toUint184' which is not marked 'virtual' |
| warning | W200 | function 'toUint184' in 'FunctionsCoordinator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint176' in 'FunctionsCoordinator' overrides 'FunctionsBilling::toUint176' which is not marked 'virtual' |
| warning | W200 | function 'toUint176' in 'FunctionsCoordinator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint168' in 'FunctionsCoordinator' overrides 'FunctionsBilling::toUint168' which is not marked 'virtual' |
| warning | W200 | function 'toUint168' in 'FunctionsCoordinator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint160' in 'FunctionsCoordinator' overrides 'FunctionsBilling::toUint160' which is not marked 'virtual' |
| warning | W200 | function 'toUint160' in 'FunctionsCoordinator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint152' in 'FunctionsCoordinator' overrides 'FunctionsBilling::toUint152' which is not marked 'virtual' |
| warning | W200 | function 'toUint152' in 'FunctionsCoordinator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint144' in 'FunctionsCoordinator' overrides 'FunctionsBilling::toUint144' which is not marked 'virtual' |
| warning | W200 | function 'toUint144' in 'FunctionsCoordinator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint136' in 'FunctionsCoordinator' overrides 'FunctionsBilling::toUint136' which is not marked 'virtual' |
| warning | W200 | function 'toUint136' in 'FunctionsCoordinator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint128' in 'FunctionsCoordinator' overrides 'FunctionsBilling::toUint128' which is not marked 'virtual' |
| warning | W200 | function 'toUint128' in 'FunctionsCoordinator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint120' in 'FunctionsCoordinator' overrides 'FunctionsBilling::toUint120' which is not marked 'virtual' |
| warning | W200 | function 'toUint120' in 'FunctionsCoordinator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint112' in 'FunctionsCoordinator' overrides 'FunctionsBilling::toUint112' which is not marked 'virtual' |
| warning | W200 | function 'toUint112' in 'FunctionsCoordinator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint104' in 'FunctionsCoordinator' overrides 'FunctionsBilling::toUint104' which is not marked 'virtual' |
| warning | W200 | function 'toUint104' in 'FunctionsCoordinator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint96' in 'FunctionsCoordinator' overrides 'FunctionsBilling::toUint96' which is not marked 'virtual' |
| warning | W200 | function 'toUint96' in 'FunctionsCoordinator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint88' in 'FunctionsCoordinator' overrides 'FunctionsBilling::toUint88' which is not marked 'virtual' |
| warning | W200 | function 'toUint88' in 'FunctionsCoordinator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint80' in 'FunctionsCoordinator' overrides 'FunctionsBilling::toUint80' which is not marked 'virtual' |
| warning | W200 | function 'toUint80' in 'FunctionsCoordinator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint72' in 'FunctionsCoordinator' overrides 'FunctionsBilling::toUint72' which is not marked 'virtual' |
| warning | W200 | function 'toUint72' in 'FunctionsCoordinator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint64' in 'FunctionsCoordinator' overrides 'FunctionsBilling::toUint64' which is not marked 'virtual' |
| warning | W200 | function 'toUint64' in 'FunctionsCoordinator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint56' in 'FunctionsCoordinator' overrides 'FunctionsBilling::toUint56' which is not marked 'virtual' |
| warning | W200 | function 'toUint56' in 'FunctionsCoordinator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint48' in 'FunctionsCoordinator' overrides 'FunctionsBilling::toUint48' which is not marked 'virtual' |
| warning | W200 | function 'toUint48' in 'FunctionsCoordinator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint40' in 'FunctionsCoordinator' overrides 'FunctionsBilling::toUint40' which is not marked 'virtual' |
| warning | W200 | function 'toUint40' in 'FunctionsCoordinator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint32' in 'FunctionsCoordinator' overrides 'FunctionsBilling::toUint32' which is not marked 'virtual' |
| warning | W200 | function 'toUint32' in 'FunctionsCoordinator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint24' in 'FunctionsCoordinator' overrides 'FunctionsBilling::toUint24' which is not marked 'virtual' |
| warning | W200 | function 'toUint24' in 'FunctionsCoordinator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint16' in 'FunctionsCoordinator' overrides 'FunctionsBilling::toUint16' which is not marked 'virtual' |
| warning | W200 | function 'toUint16' in 'FunctionsCoordinator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint8' in 'FunctionsCoordinator' overrides 'FunctionsBilling::toUint8' which is not marked 'virtual' |
| warning | W200 | function 'toUint8' in 'FunctionsCoordinator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toUint256' in 'FunctionsCoordinator' overrides 'FunctionsBilling::toUint256' which is not marked 'virtual' |
| warning | W200 | function 'toUint256' in 'FunctionsCoordinator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt248' in 'FunctionsCoordinator' overrides 'FunctionsBilling::toInt248' which is not marked 'virtual' |
| warning | W200 | function 'toInt248' in 'FunctionsCoordinator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt240' in 'FunctionsCoordinator' overrides 'FunctionsBilling::toInt240' which is not marked 'virtual' |
| warning | W200 | function 'toInt240' in 'FunctionsCoordinator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt232' in 'FunctionsCoordinator' overrides 'FunctionsBilling::toInt232' which is not marked 'virtual' |
| warning | W200 | function 'toInt232' in 'FunctionsCoordinator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt224' in 'FunctionsCoordinator' overrides 'FunctionsBilling::toInt224' which is not marked 'virtual' |
| warning | W200 | function 'toInt224' in 'FunctionsCoordinator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt216' in 'FunctionsCoordinator' overrides 'FunctionsBilling::toInt216' which is not marked 'virtual' |
| warning | W200 | function 'toInt216' in 'FunctionsCoordinator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt208' in 'FunctionsCoordinator' overrides 'FunctionsBilling::toInt208' which is not marked 'virtual' |
| warning | W200 | function 'toInt208' in 'FunctionsCoordinator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt200' in 'FunctionsCoordinator' overrides 'FunctionsBilling::toInt200' which is not marked 'virtual' |
| warning | W200 | function 'toInt200' in 'FunctionsCoordinator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt192' in 'FunctionsCoordinator' overrides 'FunctionsBilling::toInt192' which is not marked 'virtual' |
| warning | W200 | function 'toInt192' in 'FunctionsCoordinator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt184' in 'FunctionsCoordinator' overrides 'FunctionsBilling::toInt184' which is not marked 'virtual' |
| warning | W200 | function 'toInt184' in 'FunctionsCoordinator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt176' in 'FunctionsCoordinator' overrides 'FunctionsBilling::toInt176' which is not marked 'virtual' |
| warning | W200 | function 'toInt176' in 'FunctionsCoordinator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt168' in 'FunctionsCoordinator' overrides 'FunctionsBilling::toInt168' which is not marked 'virtual' |
| warning | W200 | function 'toInt168' in 'FunctionsCoordinator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt160' in 'FunctionsCoordinator' overrides 'FunctionsBilling::toInt160' which is not marked 'virtual' |
| warning | W200 | function 'toInt160' in 'FunctionsCoordinator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt152' in 'FunctionsCoordinator' overrides 'FunctionsBilling::toInt152' which is not marked 'virtual' |
| warning | W200 | function 'toInt152' in 'FunctionsCoordinator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt144' in 'FunctionsCoordinator' overrides 'FunctionsBilling::toInt144' which is not marked 'virtual' |
| warning | W200 | function 'toInt144' in 'FunctionsCoordinator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt136' in 'FunctionsCoordinator' overrides 'FunctionsBilling::toInt136' which is not marked 'virtual' |
| warning | W200 | function 'toInt136' in 'FunctionsCoordinator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt128' in 'FunctionsCoordinator' overrides 'FunctionsBilling::toInt128' which is not marked 'virtual' |
| warning | W200 | function 'toInt128' in 'FunctionsCoordinator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt120' in 'FunctionsCoordinator' overrides 'FunctionsBilling::toInt120' which is not marked 'virtual' |
| warning | W200 | function 'toInt120' in 'FunctionsCoordinator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt112' in 'FunctionsCoordinator' overrides 'FunctionsBilling::toInt112' which is not marked 'virtual' |
| warning | W200 | function 'toInt112' in 'FunctionsCoordinator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt104' in 'FunctionsCoordinator' overrides 'FunctionsBilling::toInt104' which is not marked 'virtual' |
| warning | W200 | function 'toInt104' in 'FunctionsCoordinator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt96' in 'FunctionsCoordinator' overrides 'FunctionsBilling::toInt96' which is not marked 'virtual' |
| warning | W200 | function 'toInt96' in 'FunctionsCoordinator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt88' in 'FunctionsCoordinator' overrides 'FunctionsBilling::toInt88' which is not marked 'virtual' |
| warning | W200 | function 'toInt88' in 'FunctionsCoordinator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt80' in 'FunctionsCoordinator' overrides 'FunctionsBilling::toInt80' which is not marked 'virtual' |
| warning | W200 | function 'toInt80' in 'FunctionsCoordinator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt72' in 'FunctionsCoordinator' overrides 'FunctionsBilling::toInt72' which is not marked 'virtual' |
| warning | W200 | function 'toInt72' in 'FunctionsCoordinator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt64' in 'FunctionsCoordinator' overrides 'FunctionsBilling::toInt64' which is not marked 'virtual' |
| warning | W200 | function 'toInt64' in 'FunctionsCoordinator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt56' in 'FunctionsCoordinator' overrides 'FunctionsBilling::toInt56' which is not marked 'virtual' |
| warning | W200 | function 'toInt56' in 'FunctionsCoordinator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt48' in 'FunctionsCoordinator' overrides 'FunctionsBilling::toInt48' which is not marked 'virtual' |
| warning | W200 | function 'toInt48' in 'FunctionsCoordinator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt40' in 'FunctionsCoordinator' overrides 'FunctionsBilling::toInt40' which is not marked 'virtual' |
| warning | W200 | function 'toInt40' in 'FunctionsCoordinator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt32' in 'FunctionsCoordinator' overrides 'FunctionsBilling::toInt32' which is not marked 'virtual' |
| warning | W200 | function 'toInt32' in 'FunctionsCoordinator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt24' in 'FunctionsCoordinator' overrides 'FunctionsBilling::toInt24' which is not marked 'virtual' |
| warning | W200 | function 'toInt24' in 'FunctionsCoordinator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt16' in 'FunctionsCoordinator' overrides 'FunctionsBilling::toInt16' which is not marked 'virtual' |
| warning | W200 | function 'toInt16' in 'FunctionsCoordinator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt8' in 'FunctionsCoordinator' overrides 'FunctionsBilling::toInt8' which is not marked 'virtual' |
| warning | W200 | function 'toInt8' in 'FunctionsCoordinator' overrides a base function but is not marked 'override' |
| warning | W200 | function 'toInt256' in 'FunctionsCoordinator' overrides 'FunctionsBilling::toInt256' which is not marked 'virtual' |
| warning | W200 | function 'toInt256' in 'FunctionsCoordinator' overrides a base function but is not marked 'override' |
| warning | W200 | function '_getCurrentTxL1GasFees' in 'FunctionsCoordinator' overrides 'FunctionsBilling::_getCurrentTxL1GasFees' which is not marked 'virtual' |
| warning | W200 | function '_getCurrentTxL1GasFees' in 'FunctionsCoordinator' overrides a base function but is not marked 'override' |
| warning | W200 | function '_isArbitrumChainId' in 'FunctionsCoordinator' overrides 'FunctionsBilling::_isArbitrumChainId' which is not marked 'virtual' |
| warning | W200 | function '_isArbitrumChainId' in 'FunctionsCoordinator' overrides a base function but is not marked 'override' |
| warning | W200 | function '_isOptimismChainId' in 'FunctionsCoordinator' overrides 'FunctionsBilling::_isOptimismChainId' which is not marked 'virtual' |
| warning | W200 | function '_isOptimismChainId' in 'FunctionsCoordinator' overrides a base function but is not marked 'override' |
| warning | MANIFEST_WILDCARD_CONTRACT | contract 'FunctionsCoordinator' requires wildcard contract manifest permissions (contract='*') due to dynamic contract calls. This is riskier than fixed contract hashes; use --deny-wildcard-contracts to make this a hard error. |

## References

- Global audit report: [Famous Contracts on NeoVM](/solidity/famous-contracts-neo-audit)
- Per-contract index: [Original Famous Contracts](/solidity/original-contracts/)
- Upstream contract path: `third_party/famous-contracts/sources/@chainlink/contracts/src/v0.8/functions/v1_3_0/FunctionsCoordinator.sol`