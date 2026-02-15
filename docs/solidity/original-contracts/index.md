# Original Famous Solidity Contracts (Per Contract)

This section documents **upstream famous Solidity contracts** (npm sources), not simplified demo ports.

- Generated at (UTC): `2026-02-15T07:10:37.202Z`
- Compiler: `neo-solc 0.12.0`
- Contracts in this section: `92`

Each contract has a dedicated page with:

1. Compilation status on NeoVM
1. Primary blocker and required Neo-side capability/refactor
1. Full diagnostics captured by the audit run

## Project Summary

| Project | Contracts | Pass | Fail |
| --- | ---: | ---: | ---: |
| Aave V3 | 13 | 1 | 12 |
| Chainlink | 15 | 3 | 12 |
| OpenZeppelin | 31 | 2 | 29 |
| OpenZeppelin Upgradeable | 9 | 0 | 9 |
| Safe | 8 | 0 | 8 |
| Uniswap V2 Core | 3 | 0 | 3 |
| Uniswap V4 Core | 7 | 0 | 7 |
| Uniswap V4 Periphery | 6 | 0 | 6 |

## Aave V3

| Contract | Status | Blocker | Need On Neo |
| --- | --- | --- | --- |
| [AaveOracle](/solidity/original-contracts/aave-v3/aaveoracle) | ❌ fail | `name_resolution` | 需要补齐对应语义 lowering（命名解析/继承展平），或对源码做 Neo 兼容重写 |
| [ACLManager](/solidity/original-contracts/aave-v3/aclmanager) | ❌ fail | `abi_overload` | 需要避免同参数个数的重载（重命名公开方法），或扩展 Neo ABI 到签名级调度 |
| [AToken](/solidity/original-contracts/aave-v3/atoken) | ❌ fail | `assembly` | 需要移除 `assembly`，改用 `NativeCalls.sol` / `Syscalls.sol` / 高级 Solidity 语义重写 |
| [FlashLoanReceiverBase](/solidity/original-contracts/aave-v3/flashloanreceiverbase) | ❌ fail | `other` | 需要扩展 neo-solidity 对该语义的 IR lowering，或用 Neo 等价模式重写该模块 |
| [InitializableImmutableAdminUpgradeabilityProxy](/solidity/original-contracts/aave-v3/initializableimmutableadminupgradeabilityproxy) | ❌ fail | `assembly` | 需要移除 `assembly`，改用 `NativeCalls.sol` / `Syscalls.sol` / 高级 Solidity 语义重写 |
| [L2Pool](/solidity/original-contracts/aave-v3/l2pool) | ❌ fail | `import_cycle` | 需要编译器导入解析支持循环依赖图，或对上游源码做解环拆分 |
| [Pool](/solidity/original-contracts/aave-v3/pool) | ❌ fail | `import_cycle` | 需要编译器导入解析支持循环依赖图，或对上游源码做解环拆分 |
| [PoolAddressesProvider](/solidity/original-contracts/aave-v3/pooladdressesprovider) | ❌ fail | `assembly` | 需要移除 `assembly`，改用 `NativeCalls.sol` / `Syscalls.sol` / 高级 Solidity 语义重写 |
| [PoolAddressesProviderRegistry](/solidity/original-contracts/aave-v3/pooladdressesproviderregistry) | ❌ fail | `abi_overload` | 需要避免同参数个数的重载（重命名公开方法），或扩展 Neo ABI 到签名级调度 |
| [PoolConfigurator](/solidity/original-contracts/aave-v3/poolconfigurator) | ❌ fail | `unsupported_param_type` | 需要扩展接口/结构体参数类型 lowering（复杂参数序列化），或先重构为基础类型边界 |
| [StableDebtToken](/solidity/original-contracts/aave-v3/stabledebttoken) | ❌ fail | `assembly` | 需要移除 `assembly`，改用 `NativeCalls.sol` / `Syscalls.sol` / 高级 Solidity 语义重写 |
| [VariableDebtToken](/solidity/original-contracts/aave-v3/variabledebttoken) | ❌ fail | `assembly` | 需要移除 `assembly`，改用 `NativeCalls.sol` / `Syscalls.sol` / 高级 Solidity 语义重写 |
| [WETH9](/solidity/original-contracts/aave-v3/weth9) | ✅ pass | `none` | 可直接编译为 NeoVM（如需生产使用，仍需做 manifest 权限最小化和业务安全审计） |

## Chainlink

| Contract | Status | Blocker | Need On Neo |
| --- | --- | --- | --- |
| [ArbitrumSequencerUptimeFeed](/solidity/original-contracts/chainlink/arbitrumsequenceruptimefeed) | ❌ fail | `duplicate_state_var` | 需要修复状态变量命名冲突解析（编译器语义分析）或在源码层拆分冲突字段 |
| [AutomationCompatible](/solidity/original-contracts/chainlink/automationcompatible) | ✅ pass | `none` | 可直接编译为 NeoVM（如需生产使用，仍需做 manifest 权限最小化和业务安全审计） |
| [AutomationRegistry2_3](/solidity/original-contracts/chainlink/automationregistry2-3) | ❌ fail | `abi_overload` | 需要避免同参数个数的重载（重命名公开方法），或扩展 Neo ABI 到签名级调度 |
| [DataFeedsCache](/solidity/original-contracts/chainlink/datafeedscache) | ❌ fail | `value_call_options` | 需要把 `{value: ...}` 风格调用改成显式 NEP-17 转账（`NativeCalls.gasTransfer` / `NativeCalls.neoTransfer`）并使用 `onNEP17Payment` 回调 |
| [FunctionsClient](/solidity/original-contracts/chainlink/functionsclient) | ❌ fail | `abi_overload` | 需要避免同参数个数的重载（重命名公开方法），或扩展 Neo ABI 到签名级调度 |
| [FunctionsClient_v1_3_0](/solidity/original-contracts/chainlink/functionsclient-v1-3-0) | ❌ fail | `abi_overload` | 需要避免同参数个数的重载（重命名公开方法），或扩展 Neo ABI 到签名级调度 |
| [FunctionsCoordinator](/solidity/original-contracts/chainlink/functionscoordinator) | ❌ fail | `named_mapping` | 需要编译器补齐命名 `mapping(address key =&gt; T)` 语法 lowering，或改写为 `mapping(address =&gt; T)` |
| [FunctionsCoordinator_v1_3_0](/solidity/original-contracts/chainlink/functionscoordinator-v1-3-0) | ❌ fail | `other` | 需要扩展 neo-solidity 对该语义的 IR lowering，或用 Neo 等价模式重写该模块 |
| [FunctionsRouter](/solidity/original-contracts/chainlink/functionsrouter) | ❌ fail | `value_call_options` | 需要把 `{value: ...}` 风格调用改成显式 NEP-17 转账（`NativeCalls.gasTransfer` / `NativeCalls.neoTransfer`）并使用 `onNEP17Payment` 回调 |
| [MockV3Aggregator](/solidity/original-contracts/chainlink/mockv3aggregator) | ✅ pass | `none` | 可直接编译为 NeoVM（如需生产使用，仍需做 manifest 权限最小化和业务安全审计） |
| [Operator](/solidity/original-contracts/chainlink/operator) | ❌ fail | `named_mapping` | 需要编译器补齐命名 `mapping(address key =&gt; T)` 语法 lowering，或改写为 `mapping(address =&gt; T)` |
| [OwnerIsCreator](/solidity/original-contracts/chainlink/owneriscreator) | ❌ fail | `name_resolution` | 需要补齐对应语义 lowering（命名解析/继承展平），或对源码做 Neo 兼容重写 |
| [VRFConsumerBaseV2](/solidity/original-contracts/chainlink/vrfconsumerbasev2) | ✅ pass | `none` | 可直接编译为 NeoVM（如需生产使用，仍需做 manifest 权限最小化和业务安全审计） |
| [VRFCoordinatorV2_5](/solidity/original-contracts/chainlink/vrfcoordinatorv2-5) | ❌ fail | `abi_overload` | 需要避免同参数个数的重载（重命名公开方法），或扩展 Neo ABI 到签名级调度 |
| [ZKSyncFunctionsRouter](/solidity/original-contracts/chainlink/zksyncfunctionsrouter) | ❌ fail | `assembly` | 需要移除 `assembly`，改用 `NativeCalls.sol` / `Syscalls.sol` / 高级 Solidity 语义重写 |

## OpenZeppelin

| Contract | Status | Blocker | Need On Neo |
| --- | --- | --- | --- |
| [AccessControl](/solidity/original-contracts/openzeppelin/accesscontrol) | ❌ fail | `named_mapping` | 需要编译器补齐命名 `mapping(address key =&gt; T)` 语法 lowering，或改写为 `mapping(address =&gt; T)` |
| [AccessManager](/solidity/original-contracts/openzeppelin/accessmanager) | ❌ fail | `abi_overload` | 需要避免同参数个数的重载（重命名公开方法），或扩展 Neo ABI 到签名级调度 |
| [Account](/solidity/original-contracts/openzeppelin/account) | ❌ fail | `abi_overload` | 需要避免同参数个数的重载（重命名公开方法），或扩展 Neo ABI 到签名级调度 |
| [BeaconProxy](/solidity/original-contracts/openzeppelin/beaconproxy) | ❌ fail | `abi_overload` | 需要避免同参数个数的重载（重命名公开方法），或扩展 Neo ABI 到签名级调度 |
| [EIP712](/solidity/original-contracts/openzeppelin/eip712) | ❌ fail | `abi_overload` | 需要避免同参数个数的重载（重命名公开方法），或扩展 Neo ABI 到签名级调度 |
| [ERC1155](/solidity/original-contracts/openzeppelin/erc1155) | ❌ fail | `abi_overload` | 需要避免同参数个数的重载（重命名公开方法），或扩展 Neo ABI 到签名级调度 |
| [ERC1967Proxy](/solidity/original-contracts/openzeppelin/erc1967proxy) | ❌ fail | `abi_overload` | 需要避免同参数个数的重载（重命名公开方法），或扩展 Neo ABI 到签名级调度 |
| [ERC20](/solidity/original-contracts/openzeppelin/erc20) | ❌ fail | `named_mapping` | 需要编译器补齐命名 `mapping(address key =&gt; T)` 语法 lowering，或改写为 `mapping(address =&gt; T)` |
| [ERC20Capped](/solidity/original-contracts/openzeppelin/erc20capped) | ❌ fail | `ctor_modifier_mismatch` | 需要修复构造器/修饰器参数传递路径，或扩展编译器对复杂构造器链的 lowering |
| [ERC20FlashMint](/solidity/original-contracts/openzeppelin/erc20flashmint) | ❌ fail | `named_mapping` | 需要编译器补齐命名 `mapping(address key =&gt; T)` 语法 lowering，或改写为 `mapping(address =&gt; T)` |
| [ERC20Permit](/solidity/original-contracts/openzeppelin/erc20permit) | ❌ fail | `abi_overload` | 需要避免同参数个数的重载（重命名公开方法），或扩展 Neo ABI 到签名级调度 |
| [ERC20Votes](/solidity/original-contracts/openzeppelin/erc20votes) | ❌ fail | `abi_overload` | 需要避免同参数个数的重载（重命名公开方法），或扩展 Neo ABI 到签名级调度 |
| [ERC2771Forwarder](/solidity/original-contracts/openzeppelin/erc2771forwarder) | ❌ fail | `abi_overload` | 需要避免同参数个数的重载（重命名公开方法），或扩展 Neo ABI 到签名级调度 |
| [ERC2981](/solidity/original-contracts/openzeppelin/erc2981) | ✅ pass | `none` | 可直接编译为 NeoVM（如需生产使用，仍需做 manifest 权限最小化和业务安全审计） |
| [ERC4626](/solidity/original-contracts/openzeppelin/erc4626) | ❌ fail | `abi_overload` | 需要避免同参数个数的重载（重命名公开方法），或扩展 Neo ABI 到签名级调度 |
| [ERC721](/solidity/original-contracts/openzeppelin/erc721) | ❌ fail | `abi_overload` | 需要避免同参数个数的重载（重命名公开方法），或扩展 Neo ABI 到签名级调度 |
| [ERC721Enumerable](/solidity/original-contracts/openzeppelin/erc721enumerable) | ❌ fail | `abi_overload` | 需要避免同参数个数的重载（重命名公开方法），或扩展 Neo ABI 到签名级调度 |
| [ERC721URIStorage](/solidity/original-contracts/openzeppelin/erc721uristorage) | ❌ fail | `abi_overload` | 需要避免同参数个数的重载（重命名公开方法），或扩展 Neo ABI 到签名级调度 |
| [ERC721Votes](/solidity/original-contracts/openzeppelin/erc721votes) | ❌ fail | `abi_overload` | 需要避免同参数个数的重载（重命名公开方法），或扩展 Neo ABI 到签名级调度 |
| [Governor](/solidity/original-contracts/openzeppelin/governor) | ❌ fail | `abi_overload` | 需要避免同参数个数的重载（重命名公开方法），或扩展 Neo ABI 到签名级调度 |
| [GovernorSettings](/solidity/original-contracts/openzeppelin/governorsettings) | ❌ fail | `abi_overload` | 需要避免同参数个数的重载（重命名公开方法），或扩展 Neo ABI 到签名级调度 |
| [GovernorTimelockControl](/solidity/original-contracts/openzeppelin/governortimelockcontrol) | ❌ fail | `abi_overload` | 需要避免同参数个数的重载（重命名公开方法），或扩展 Neo ABI 到签名级调度 |
| [GovernorVotes](/solidity/original-contracts/openzeppelin/governorvotes) | ❌ fail | `abi_overload` | 需要避免同参数个数的重载（重命名公开方法），或扩展 Neo ABI 到签名级调度 |
| [Initializable](/solidity/original-contracts/openzeppelin/initializable) | ❌ fail | `assembly` | 需要移除 `assembly`，改用 `NativeCalls.sol` / `Syscalls.sol` / 高级 Solidity 语义重写 |
| [Multicall](/solidity/original-contracts/openzeppelin/multicall) | ❌ fail | `value_call_options` | 需要把 `{value: ...}` 风格调用改成显式 NEP-17 转账（`NativeCalls.gasTransfer` / `NativeCalls.neoTransfer`）并使用 `onNEP17Payment` 回调 |
| [Ownable](/solidity/original-contracts/openzeppelin/ownable) | ✅ pass | `none` | 可直接编译为 NeoVM（如需生产使用，仍需做 manifest 权限最小化和业务安全审计） |
| [ProxyAdmin](/solidity/original-contracts/openzeppelin/proxyadmin) | ❌ fail | `import_cycle` | 需要编译器导入解析支持循环依赖图，或对上游源码做解环拆分 |
| [TimelockController](/solidity/original-contracts/openzeppelin/timelockcontroller) | ❌ fail | `value_call_options` | 需要把 `{value: ...}` 风格调用改成显式 NEP-17 转账（`NativeCalls.gasTransfer` / `NativeCalls.neoTransfer`）并使用 `onNEP17Payment` 回调 |
| [TransparentUpgradeableProxy](/solidity/original-contracts/openzeppelin/transparentupgradeableproxy) | ❌ fail | `import_cycle` | 需要编译器导入解析支持循环依赖图，或对上游源码做解环拆分 |
| [UUPSUpgradeable](/solidity/original-contracts/openzeppelin/uupsupgradeable) | ❌ fail | `abi_overload` | 需要避免同参数个数的重载（重命名公开方法），或扩展 Neo ABI 到签名级调度 |
| [VestingWallet](/solidity/original-contracts/openzeppelin/vestingwallet) | ❌ fail | `inheritance_linearization` | 需要调整继承层次（或扩展编译器的 C3 线性化兼容），避免多重继承顺序冲突 |

## OpenZeppelin Upgradeable

| Contract | Status | Blocker | Need On Neo |
| --- | --- | --- | --- |
| [ERC20PermitUpgradeable](/solidity/original-contracts/openzeppelin-upgradeable/erc20permitupgradeable) | ❌ fail | `abi_overload` | 需要避免同参数个数的重载（重命名公开方法），或扩展 Neo ABI 到签名级调度 |
| [ERC20Upgradeable](/solidity/original-contracts/openzeppelin-upgradeable/erc20upgradeable) | ❌ fail | `inheritance_linearization` | 需要调整继承层次（或扩展编译器的 C3 线性化兼容），避免多重继承顺序冲突 |
| [ERC721Upgradeable](/solidity/original-contracts/openzeppelin-upgradeable/erc721upgradeable) | ❌ fail | `abi_overload` | 需要避免同参数个数的重载（重命名公开方法），或扩展 Neo ABI 到签名级调度 |
| [GovernorTimelockControlUpgradeable](/solidity/original-contracts/openzeppelin-upgradeable/governortimelockcontrolupgradeable) | ❌ fail | `abi_overload` | 需要避免同参数个数的重载（重命名公开方法），或扩展 Neo ABI 到签名级调度 |
| [GovernorUpgradeable](/solidity/original-contracts/openzeppelin-upgradeable/governorupgradeable) | ❌ fail | `abi_overload` | 需要避免同参数个数的重载（重命名公开方法），或扩展 Neo ABI 到签名级调度 |
| [GovernorVotesUpgradeable](/solidity/original-contracts/openzeppelin-upgradeable/governorvotesupgradeable) | ❌ fail | `abi_overload` | 需要避免同参数个数的重载（重命名公开方法），或扩展 Neo ABI 到签名级调度 |
| [InitializableUpgradeable](/solidity/original-contracts/openzeppelin-upgradeable/initializableupgradeable) | ❌ fail | `assembly` | 需要移除 `assembly`，改用 `NativeCalls.sol` / `Syscalls.sol` / 高级 Solidity 语义重写 |
| [OwnableUpgradeable](/solidity/original-contracts/openzeppelin-upgradeable/ownableupgradeable) | ❌ fail | `inheritance_linearization` | 需要调整继承层次（或扩展编译器的 C3 线性化兼容），避免多重继承顺序冲突 |
| [UUPSUpgradeable](/solidity/original-contracts/openzeppelin-upgradeable/uupsupgradeable) | ❌ fail | `abi_overload` | 需要避免同参数个数的重载（重命名公开方法），或扩展 Neo ABI 到签名级调度 |

## Safe

| Contract | Status | Blocker | Need On Neo |
| --- | --- | --- | --- |
| [CompatibilityFallbackHandler](/solidity/original-contracts/safe/compatibilityfallbackhandler) | ❌ fail | `name_resolution` | 需要补齐对应语义 lowering（命名解析/继承展平），或对源码做 Neo 兼容重写 |
| [MultiSend](/solidity/original-contracts/safe/multisend) | ❌ fail | `assembly` | 需要移除 `assembly`，改用 `NativeCalls.sol` / `Syscalls.sol` / 高级 Solidity 语义重写 |
| [MultiSendCallOnly](/solidity/original-contracts/safe/multisendcallonly) | ❌ fail | `assembly` | 需要移除 `assembly`，改用 `NativeCalls.sol` / `Syscalls.sol` / 高级 Solidity 语义重写 |
| [Safe](/solidity/original-contracts/safe/safe) | ❌ fail | `name_resolution` | 需要补齐对应语义 lowering（命名解析/继承展平），或对源码做 Neo 兼容重写 |
| [SafeL2](/solidity/original-contracts/safe/safel2) | ❌ fail | `name_resolution` | 需要补齐对应语义 lowering（命名解析/继承展平），或对源码做 Neo 兼容重写 |
| [SafeProxy](/solidity/original-contracts/safe/safeproxy) | ❌ fail | `assembly` | 需要移除 `assembly`，改用 `NativeCalls.sol` / `Syscalls.sol` / 高级 Solidity 语义重写 |
| [SafeProxyFactory](/solidity/original-contracts/safe/safeproxyfactory) | ❌ fail | `assembly` | 需要移除 `assembly`，改用 `NativeCalls.sol` / `Syscalls.sol` / 高级 Solidity 语义重写 |
| [SignMessageLib](/solidity/original-contracts/safe/signmessagelib) | ❌ fail | `name_resolution` | 需要补齐对应语义 lowering（命名解析/继承展平），或对源码做 Neo 兼容重写 |

## Uniswap V2 Core

| Contract | Status | Blocker | Need On Neo |
| --- | --- | --- | --- |
| [UniswapV2ERC20](/solidity/original-contracts/uniswap-v2-core/uniswapv2erc20) | ❌ fail | `solidity_version` | 需要将源码迁移到 Solidity 0.8.x 范围并处理破坏性变更 |
| [UniswapV2Factory](/solidity/original-contracts/uniswap-v2-core/uniswapv2factory) | ❌ fail | `solidity_version` | 需要将源码迁移到 Solidity 0.8.x 范围并处理破坏性变更 |
| [UniswapV2Pair](/solidity/original-contracts/uniswap-v2-core/uniswapv2pair) | ❌ fail | `solidity_version` | 需要将源码迁移到 Solidity 0.8.x 范围并处理破坏性变更 |

## Uniswap V4 Core

| Contract | Status | Blocker | Need On Neo |
| --- | --- | --- | --- |
| [ERC6909](/solidity/original-contracts/uniswap-v4-core/erc6909) | ❌ fail | `named_mapping` | 需要编译器补齐命名 `mapping(address key =&gt; T)` 语法 lowering，或改写为 `mapping(address =&gt; T)` |
| [ERC6909Claims](/solidity/original-contracts/uniswap-v4-core/erc6909claims) | ❌ fail | `named_mapping` | 需要编译器补齐命名 `mapping(address key =&gt; T)` 语法 lowering，或改写为 `mapping(address =&gt; T)` |
| [Extsload](/solidity/original-contracts/uniswap-v4-core/extsload) | ❌ fail | `abi_overload` | 需要避免同参数个数的重载（重命名公开方法），或扩展 Neo ABI 到签名级调度 |
| [Exttload](/solidity/original-contracts/uniswap-v4-core/exttload) | ❌ fail | `abi_overload` | 需要避免同参数个数的重载（重命名公开方法），或扩展 Neo ABI 到签名级调度 |
| [NoDelegateCall](/solidity/original-contracts/uniswap-v4-core/nodelegatecall) | ❌ fail | `abi_overload` | 需要避免同参数个数的重载（重命名公开方法），或扩展 Neo ABI 到签名级调度 |
| [PoolManager](/solidity/original-contracts/uniswap-v4-core/poolmanager) | ❌ fail | `import_cycle` | 需要编译器导入解析支持循环依赖图，或对上游源码做解环拆分 |
| [ProtocolFees](/solidity/original-contracts/uniswap-v4-core/protocolfees) | ❌ fail | `import_cycle` | 需要编译器导入解析支持循环依赖图，或对上游源码做解环拆分 |

## Uniswap V4 Periphery

| Contract | Status | Blocker | Need On Neo |
| --- | --- | --- | --- |
| [PositionDescriptor](/solidity/original-contracts/uniswap-v4-periphery/positiondescriptor) | ❌ fail | `import_cycle` | 需要编译器导入解析支持循环依赖图，或对上游源码做解环拆分 |
| [PositionManager](/solidity/original-contracts/uniswap-v4-periphery/positionmanager) | ❌ fail | `import_cycle` | 需要编译器导入解析支持循环依赖图，或对上游源码做解环拆分 |
| [StateView](/solidity/original-contracts/uniswap-v4-periphery/stateview) | ❌ fail | `import_cycle` | 需要编译器导入解析支持循环依赖图，或对上游源码做解环拆分 |
| [UniswapV4DeployerCompetition](/solidity/original-contracts/uniswap-v4-periphery/uniswapv4deployercompetition) | ❌ fail | `assembly` | 需要移除 `assembly`，改用 `NativeCalls.sol` / `Syscalls.sol` / 高级 Solidity 语义重写 |
| [V4Quoter](/solidity/original-contracts/uniswap-v4-periphery/v4quoter) | ❌ fail | `import_cycle` | 需要编译器导入解析支持循环依赖图，或对上游源码做解环拆分 |
| [V4Router](/solidity/original-contracts/uniswap-v4-periphery/v4router) | ❌ fail | `import_cycle` | 需要编译器导入解析支持循环依赖图，或对上游源码做解环拆分 |