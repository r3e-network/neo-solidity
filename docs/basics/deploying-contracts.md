# Deploy Contracts

Deploying a Neo N3 smart contract requires two artifacts produced by `neo-solc`:

- `<contract>.nef` -- the NeoVM bytecode
- `<contract>.manifest.json` -- the ABI, permissions, and metadata

This page covers the Neo N3 deployment model, local deployment with Neo-Express, constructor arguments, contract upgrades, and production deployment considerations.

## Neo N3 Deployment Model

On Neo N3, contract deployment is a transaction that calls the native `ContractManagement.deploy` method. The transaction payload includes:

1. The NEF binary (bytecode + method tokens + checksum)
2. The manifest JSON (ABI + permissions + standards)
3. Optional deployment data (passed to the contract's `_deploy` method)

The Neo node validates the manifest, computes a contract hash from the sender address + NEF checksum + manifest name, and stores the contract on-chain. After deployment, the contract is callable by its hash.

::: info Contract Hash
The contract hash is deterministic: it is derived from `hash160(sender_scripthash + nef_checksum + contract_name)`. You can predict it before deployment using `neo-solc --deployer <sender_hash>`.
:::

## Local Deployment with Neo-Express

[Neo-Express](https://github.com/neo-project/neo-express) provides a local Neo N3 blockchain for development. It supports single-node chains, account management, contract deployment, and method invocation.

### Prerequisites

- Neo-Express installed (see [Installation](/basics/installing-the-compiler))
- `jq` for JSON inspection
- A compiled contract (`.nef` + `.manifest.json`)

### Step 1: Create a Local Chain

```bash
NEOXP=./build/dotnet-tools/neoxp

# Create a fresh single-node chain
$NEOXP create -f -o chain.neo-express
```

The `-f` flag forces overwrite if a chain file already exists.

### Step 2: Fund the Deployer Account

Neo-Express creates a `genesis` account with all NEO and GAS. Transfer GAS to the deployer:

```bash
$NEOXP transfer -i chain.neo-express 100 GAS genesis node1
```

This gives `node1` enough GAS to cover deployment and invocation costs.

### Step 3: Deploy the Contract

```bash
$NEOXP contract deploy -i chain.neo-express build/MyContract.nef node1
```

The output includes:

- `contract-hash` -- the deployed contract's script hash
- `tx-hash` -- the deployment transaction hash

To capture the contract hash programmatically:

```bash
DEPLOY_OUT="$($NEOXP contract deploy -i chain.neo-express build/MyContract.nef node1 -j)"
CONTRACT_HASH="$(echo "$DEPLOY_OUT" | jq -r '.["contract-hash"]')"
echo "Deployed at: $CONTRACT_HASH"
```

### Step 4: Invoke Methods

Create a `.neo-invoke.json` file for each invocation:

```json
{
    "contract": "0x<contract-hash>",
    "operation": "methodName",
    "args": [42, "hello"]
}
```

**Write invocation** (creates a transaction, costs GAS):

```bash
$NEOXP contract invoke -i chain.neo-express invoke.neo-invoke.json node1
```

**Read-only invocation** (no transaction, no GAS cost):

```bash
$NEOXP contract invoke -r -j -i chain.neo-express invoke.neo-invoke.json node1
```

The `-r` flag runs the invocation as a test (read-only). The `-j` flag outputs JSON for programmatic parsing.

### Step 5: Verify Transaction Results

After a write invocation, inspect the transaction's application log:

```bash
TX_HASH="0x<transaction-hash>"
$NEOXP show transaction -i chain.neo-express "$TX_HASH"
```

Check the VM state and notifications:

```bash
APP_LOG="$($NEOXP show transaction -i chain.neo-express "$TX_HASH")"

# VM state should be "HALT" (success)
echo "$APP_LOG" | jq '.["application-log"].executions[0].vmstate'

# Check emitted events
echo "$APP_LOG" | jq '.["application-log"].executions[0].notifications'
```

## Constructor Arguments

Solidity constructors map to Neo's `_deploy(data, update)` entry point. The compiler automatically generates this entry point for contracts with constructors.

### How It Works

1. The compiler emits a `_deploy(data, update)` method in the NEF.
2. On initial deployment (`update = false`), the deploy stub deserializes `data` and passes the values to your Solidity constructor logic.
3. On contract update (`update = true`), the constructor is skipped.

### Passing Constructor Arguments

**Neo-Express / CLI tooling:** Pass a JSON array string as the `-d` flag:

```bash
# Constructor: constructor(uint256 initialValue)
$NEOXP contract deploy -i chain.neo-express -d '[7]' build/Counter.nef node1

# Constructor: constructor(uint256 amount, string name)
$NEOXP contract deploy -i chain.neo-express -d '[1000, "MyToken"]' build/Token.nef node1
```

**Neo SDKs (C#, Python, JS):** Pass a StackItem Array directly through the SDK's deploy API.

**Contract-to-contract deployment:** Use `abi.encode(...)` to produce StdLib.serialize bytes, which the deploy stub can also accept.

### Manifest Requirements

Contracts with parameterized constructors require these permissions in the manifest:

- `StdLib.jsonDeserialize`
- `StdLib.deserialize`

The compiler adds these automatically. You can verify:

```bash
jq '.permissions' build/Counter.manifest.json
```

::: warning
If you use `--deny-wildcard-methods` and the compiler cannot narrow the StdLib permissions, compilation will fail. In that case, provide an explicit `--manifest-permissions` file that includes the required StdLib methods.
:::

## Contract Upgrade Lifecycle

Neo N3 supports upgrading deployed contracts through the native `ContractManagement` system. The lifecycle is:

### Deploy

Initial deployment creates the contract on-chain:

```bash
$NEOXP contract deploy -i chain.neo-express build/MyContract.nef node1
```

### Update

Update replaces the contract's NEF and manifest while preserving its storage and hash:

```bash
$NEOXP contract update -i chain.neo-express "$CONTRACT_HASH" build/MyContractV2.nef node1
```

When updating:

- The `_deploy(data, update)` method is called with `update = true`.
- The Solidity constructor logic is skipped (only runs on initial deploy).
- Storage is preserved -- existing state variables retain their values.
- The contract hash remains the same.

### Destroy

Remove a contract from the chain (irreversible):

```bash
# Via devpack: ContractManagement.destroy() in your contract code
# Or via neo-cli / SDK calls to ContractManagement
```

::: danger
`destroy` permanently removes the contract and its storage. This cannot be undone. Only include destroy functionality if your contract genuinely needs it, and protect it with proper access control.
:::

### Update Smoke Test

The repository includes an automated update smoke test:

```bash
make test-deploy-update-smoke
```

This script:

1. Deploys a v1 contract with constructor args
2. Updates it to v2
3. Verifies that the constructor did not re-run on update
4. Verifies that new methods from v2 are callable

## Manifest Verification Before Deploy

Before deploying to TestNet or MainNet, verify the manifest:

```bash
# Check contract name
jq '.name' build/MyContract.manifest.json

# Check permissions are not overly broad
jq '.permissions' build/MyContract.manifest.json

# Check supported standards
jq '.supportedstandards' build/MyContract.manifest.json

# Check all method signatures
jq '.abi.methods[] | {name, parameters: [.parameters[].type], returntype, safe}' \
  build/MyContract.manifest.json

# Check events
jq '.abi.events[] | {name, parameters: [.parameters[].type]}' \
  build/MyContract.manifest.json
```

::: tip
For production contracts, always compile with `--deny-wildcard-contracts --deny-wildcard-methods` to ensure the manifest has minimal permissions. See [Compile Workflow](/compiler/analysing-the-compiler-output) for details.
:::

## TestNet Deployment

### Prerequisites

- Neo CLI (`neo-cli`) installed and synced with Neo N3 TestNet
- A funded TestNet account (get test GAS from the Neo TestNet faucet)

### Deploy via neo-cli

```bash
neo-cli contract deploy build/MyContract.nef build/MyContract.manifest.json
```

For contracts with constructor arguments:

```bash
neo-cli contract deploy build/MyContract.nef build/MyContract.manifest.json -- [1000]
```

### Verify Deployment

```bash
# Invoke a view method to confirm the contract is live
neo-cli contract invoke <contract-hash> getNumber
```

## MainNet Considerations

Before deploying to MainNet:

1. **Test thoroughly on TestNet.** Deploy the exact same artifacts and run all your test scenarios.
2. **Audit the manifest permissions.** Ensure no wildcard permissions remain unless explicitly justified.
3. **Review the contract hash.** Use `--deployer` to predict the hash and verify it matches your expectations.
4. **Check GAS costs.** MainNet GAS is real value. Estimate deployment and invocation costs on TestNet first.
5. **Plan for upgrades.** If your contract needs to be upgradeable, ensure the update mechanism is tested.
6. **Run the production gate.** `make production-gate` validates the entire toolchain.

See [Production Readiness](/advisory-content/production-readiness) for the complete pre-deployment checklist.

## Common Deployment Errors

| Error                        | Cause                                                   | Solution                                                       |
| ---------------------------- | ------------------------------------------------------- | -------------------------------------------------------------- |
| `FAULT` on deploy            | Manifest permissions insufficient for constructor logic | Add required permissions or use `--manifest-permissions`       |
| `contract already exists`    | Contract with same hash already deployed                | Use `update` instead of `deploy`, or change the sender account |
| `insufficient GAS`           | Deployer account lacks GAS                              | Transfer more GAS to the deployer                              |
| `invalid NEF`                | Corrupted or incompatible NEF file                      | Rebuild with `cargo build --release && neo-solc ...`           |
| `manifest validation failed` | Manifest has invalid fields                             | Check `features` is `{}`, verify JSON structure                |

## Automated Smoke Tests

The repository ships 16+ deployment smoke tests that validate end-to-end behavior:

```bash
# Basic deploy + invoke
make test-deploy-smoke

# Constructor arguments
make test-deploy-constructor-smoke

# Contract update lifecycle
make test-deploy-update-smoke

# Manifest permissions for native contracts
make test-deploy-permissions-smoke

# abi.encode / abi.decode round-trip
make test-deploy-encoding-smoke

# Revert reason propagation
make test-deploy-abortmsg-smoke

# Low-level calls
make test-deploy-lowlevel-call-smoke

# External member calls
make test-deploy-external-call-smoke

# View/ReadOnly calls
make test-deploy-view-readonly-call-smoke

# All smoke tests
make test-deploy-smoke-full
```

Each script creates a fresh Neo-Express chain, deploys a purpose-built contract, invokes methods, and validates results. They are safe to run repeatedly and clean up after themselves.

## Related Pages

- [Compile Workflow](/compiler/analysing-the-compiler-output) -- CLI reference and compilation patterns.
- [Test Workflow](/basics/testing-contracts) -- Full test suite documentation.
- [Production Readiness](/advisory-content/production-readiness) -- Pre-deployment checklist.
- [NeoVM Native Contracts](/internals/native-contracts) -- Native contract interfaces.
- [Quick Start](/basics/quickstart) -- First deployment walkthrough.
