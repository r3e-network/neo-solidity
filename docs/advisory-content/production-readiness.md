# Production Readiness

This page provides a complete pre-deployment checklist for taking a Neo Solidity contract from development to mainnet. Follow these steps before deploying to Neo N3 TestNet or MainNet.

## Production Gate

The single most important command before any production deployment:

```bash
make production-gate
```

This runs the full validation pipeline:

| Step               | Command                                                               | What It Checks                                           |
| ------------------ | --------------------------------------------------------------------- | -------------------------------------------------------- |
| 1. Formatting      | `cargo fmt --all -- --check`                                          | Code style consistency                                   |
| 2. Linting         | `cargo clippy --all-targets --all-features -- -D warnings`            | Rust code quality and common mistakes                    |
| 3. Release build   | `cargo build --release`                                               | Compilation succeeds with optimizations                  |
| 4. Full test suite | `cargo test --workspace --all-features`                               | Rust unit, integration, manifest, runtime, and E2E tests |
| 5. Tooling tests   | `make tooling-test`                                                   | TypeScript tooling packages build and test               |
| 6. Tooling lint    | `make tooling-lint`                                                   | Tooling ESLint checks pass                               |
| 7. Runtime tests   | `make runtime-test`                                                   | Maintained optional C# runtime test slice passes         |
| 8. E2E tests       | `cargo test --test e2e_compilation_tests -- --test-threads=4`         | All example contracts compile correctly                  |
| 9. Strict sweep    | `STRICT_SWEEP_FAIL_ON_UNEXPECTED_WARNINGS=1 make test-compile-strict` | No unexpected warnings in devpack or examples            |
| 10. Smoke tests    | `make test-deploy-smoke-full`                                         | All 16+ deploy/invoke scenarios pass on Neo-Express      |

If `make production-gate` passes, the compiler toolchain is verified end-to-end.

::: warning
Never skip the production gate before deploying to TestNet or MainNet. It catches regressions that unit tests alone cannot detect.
:::

## Pre-Deployment Checklist

### 1. Compile with Production Settings

Use the strictest compilation flags:

```bash
neo-solc contract.sol \
  -I devpack \
  -O3 \
  --callt \
  --deny-wildcard-contracts \
  --deny-wildcard-methods \
  --json-errors \
  --json-warnings \
  -o build/contract
```

Flags explained:

| Flag                        | Purpose                                                              |
| --------------------------- | -------------------------------------------------------------------- |
| `-O3`                       | Maximum optimization for smallest bytecode and lowest execution cost |
| `--callt`                   | Use CALLT instructions for efficient native contract calls           |
| `--deny-wildcard-contracts` | Reject wildcard contract permissions                                 |
| `--deny-wildcard-methods`   | Reject wildcard method permissions                                   |
| `--json-errors`             | Structured error output for CI parsing                               |
| `--json-warnings`           | Structured warning output for CI parsing                             |

### 2. Audit the Manifest

Inspect every field of the generated manifest:

```bash
# Full manifest
jq '.' build/contract.manifest.json

# Contract name
jq '.name' build/contract.manifest.json

# Permissions -- should NOT contain wildcards
jq '.permissions' build/contract.manifest.json

# Supported standards
jq '.supportedstandards' build/contract.manifest.json

# Method signatures
jq '.abi.methods[] | {name, parameters: [.parameters[].type], returntype, safe}' \
  build/contract.manifest.json

# Events
jq '.abi.events[] | {name, parameters: [.parameters[].type]}' \
  build/contract.manifest.json

# Trust settings
jq '.trusts' build/contract.manifest.json
```

### Permission Audit Guide

Review each permission entry:

```bash
jq '.permissions[] | {contract, methods}' build/contract.manifest.json
```

For each entry, verify:

- **contract** is a specific hash (e.g., `0xef4073a0f2b305a38ec4050e4d3d28bc40ea63f5`), not `*`.
- **methods** is a specific list (e.g., `["transfer", "balanceOf"]`), not `*`.
- Every listed method is actually called by your contract.
- No unnecessary permissions are included.

If the compiler cannot narrow permissions (e.g., due to dynamic calls), provide an explicit allowlist:

```bash
neo-solc contract.sol \
  -I devpack \
  --manifest-permissions permissions.json \
  --manifest-permissions-mode replace-wildcards \
  --deny-wildcard-contracts \
  --deny-wildcard-methods \
  -o build/contract
```

### 3. Verify the NEF

Check the NEF file size and structure:

```bash
# File size
ls -la build/contract.nef

# Verify NEF magic bytes (should start with 4E 45 46 33)
xxd build/contract.nef | head -1
```

Optionally inspect the NeoVM assembly:

```bash
neo-solc contract.sol -I devpack -O3 -f assembly -o build/contract.asm
cat build/contract.asm
```

### 4. Predict the Contract Hash

If you need to know the contract hash before deployment (e.g., for cross-contract references):

```bash
neo-solc contract.sol \
  -I devpack \
  -O3 \
  --deployer 0x<your-deployer-scripthash> \
  -o build/contract
```

The compiler prints the predicted hash. Verify it matches your expectations.

## Release Build Profile

The Cargo release profile is configured for maximum optimization:

```toml
[profile.release]
opt-level = 3
lto = true
codegen-units = 1
panic = "abort"
```

This produces the smallest and fastest compiler binary. Always use `cargo build --release` for production compilation.

## Deployment Rehearsal

Follow this progression to catch issues at each stage:

### Stage 1: Local Neo-Express

```bash
# Create a fresh chain
NEOXP=./build/dotnet-tools/neoxp
$NEOXP create -f -o chain.neo-express
$NEOXP transfer -i chain.neo-express 100 GAS genesis node1

# Deploy
$NEOXP contract deploy -i chain.neo-express build/contract.nef node1

# Test every public method
# Test every event emission
# Test error conditions (require failures, access control)
# Test constructor arguments if applicable
# Test contract update if applicable
```

### Stage 2: Neo N3 TestNet

1. Obtain test GAS from the Neo TestNet faucet.
2. Deploy the exact same `.nef` and `.manifest.json` files.
3. Run the same test scenarios as local.
4. Verify transaction results on a Neo block explorer.
5. Test with multiple accounts and concurrent transactions.

### Stage 3: Neo N3 MainNet

1. Verify the contract hash matches your prediction.
2. Deploy with a funded MainNet account.
3. Start with small-value transactions to verify behavior.
4. Monitor the contract for unexpected behavior.

## Security Considerations

### Smart Contract Security

- **Reentrancy protection**: Use a `locked` state variable with a `nonReentrant` modifier.
- **Access control**: Use `msg.sender` checks (maps to `Runtime.GetCallingScriptHash()`).
- **Input validation**: Validate all function parameters with `require`.
- **Integer overflow**: Solidity 0.8+ has built-in overflow protection. NeoVM uses arbitrary-precision BigInteger, so overflow is not a concern on Neo.
- **Event logging**: Emit events for all state changes to enable off-chain monitoring.

### Neo-Specific Security

::: tip Neo Authorization Model
Neo uses witness-based authorization (`Runtime.checkWitness`) rather than EVM's `msg.sender`-only model. Always use witness checks for sensitive operations, even if your Solidity source uses `msg.sender` guards.
:::

- **Witness checks**: Use `Runtime.checkWitness(addr)` for authorization instead of relying solely on `msg.sender`.
- **Permission minimization**: Use `--deny-wildcard-contracts --deny-wildcard-methods` to enforce least-privilege permissions.
- **NEP-17 callbacks**: If your contract receives tokens, implement `onNEP17Payment` with proper validation.
- **Contract updates**: Protect the update mechanism with strict access control. Test the update path thoroughly.
- **Storage key collisions**: The compiler uses prefix-based storage keys. Be aware of key layout when upgrading contracts.

### External Audit

For high-value contracts:

1. Use external Solidity analysis tools (Slither, Mythril) on the source before compilation.
2. Review the generated NeoVM assembly for unexpected behavior.
3. Consider a professional audit of both the Solidity source and the generated Neo artifacts.
4. Test on TestNet with adversarial scenarios.

## Monitoring and Verification

After deployment:

- **Block explorer**: Monitor contract transactions and events on a Neo N3 block explorer.
- **RPC queries**: Use Neo RPC to query contract state and verify expected values.
- **Event subscriptions**: Subscribe to contract notifications for real-time monitoring.
- **Gas usage**: Track GAS consumption per method to detect anomalies.

## Feature Parity Review

Before production deployment, review these pages for known behavioral differences:

- [Solidity Feature Support](/solidity/feature-support) -- Which Solidity features are supported, partial, or blocked.
- [Language Description](/language-description/types) -- How EVM concepts translate to Neo N3.
- [Parity and Limitations](/internals/parity-and-limitations) -- Known runtime behavior gaps.
- [Error Reference](/advisory-content/error-reference) -- Compiler error codes and their meanings.

## Production Checklist Summary

Use this checklist before every mainnet deployment:

- [ ] `make production-gate` passes
- [ ] Compiled with `-O3 --callt --deny-wildcard-contracts --deny-wildcard-methods`
- [ ] Manifest permissions audited -- no unnecessary wildcards
- [ ] All public methods tested on Neo-Express
- [ ] Constructor arguments tested (if applicable)
- [ ] Contract update path tested (if applicable)
- [ ] Deployed and tested on Neo N3 TestNet
- [ ] Contract hash verified with `--deployer`
- [ ] Security review completed (reentrancy, access control, input validation)
- [ ] Events verified for all state changes
- [ ] GAS costs estimated and acceptable
- [ ] Monitoring plan in place

## Related Pages

- [Compile Workflow](/compiler/analysing-the-compiler-output) -- Full CLI reference and hardening options.
- [Deploy Workflow](/basics/deploying-contracts) -- Deployment procedures for all environments.
- [Test Workflow](/basics/testing-contracts) -- Complete test suite documentation.
- [Manifest Spec](/internals/contract-metadata) -- Manifest structure and permission model.
