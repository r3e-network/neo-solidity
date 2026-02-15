# FunctionsCoordinator (Chainlink)

## Audit Snapshot

- Status: ❌ fail
- Source type: `npm`
- Source path: `node_modules/@chainlink/contracts/src/v0.8/functions/v1_0_0/FunctionsCoordinator.sol`
- Primary issue: state variable 's_requestCommitments' has unsupported type 'mapping(bytes32 requestId =&gt; bytes32 commitmentHash)'
- Audit corpus size: 98 contracts

## What Must Change To Compile On NeoVM

- Primary blocker tag: `named_mapping`
- Need on Neo (from audit): 需要编译器补齐命名 `mapping(address key =&gt; T)` 语法 lowering，或改写为 `mapping(address =&gt; T)`

### Migration Playbook: Named mapping syntax/shape unsupported in current pipeline

1. Rewrite to plain mapping declarations (for example `mapping(address => uint256)`).
1. Flatten nested mapping wrappers where possible to reduce type complexity.
1. Track compiler updates for full named mapping lowering and migrate back if desired.

## Diagnostics

| Severity | Code | Message |
| --- | --- | --- |
| error | UNSUPPORTED_STATE_TYPE | state variable 's_requestCommitments' has unsupported type 'mapping(bytes32 requestId =&gt; bytes32 commitmentHash)' |
| error | UNSUPPORTED_STATE_TYPE | state variable 's_withdrawableTokens' has unsupported type 'mapping(address transmitter =&gt; uint96 balanceJuelsLink)' |

## References

- Global audit report: [Famous Contracts on NeoVM](/solidity/famous-contracts-neo-audit)
- Per-contract index: [Original Famous Contracts](/solidity/original-contracts/)
- Upstream contract path: `node_modules/@chainlink/contracts/src/v0.8/functions/v1_0_0/FunctionsCoordinator.sol`