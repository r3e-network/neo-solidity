# Production Readiness

Use this checklist before mainnet deployment.

## Mandatory gate

```bash
make production-gate
```

This runs formatting, linting, release build, tests, strict compile sweep, and Neo-Express smoke suites.

## Manifest policy

For production builds, deny wildcards unless explicitly justified:

```bash
neo-solc contract.sol \
  --deny-wildcard-contracts \
  --deny-wildcard-methods \
  -o build/contract
```

If dynamic calls are required, provide an explicit `--manifest-permissions` allowlist.

## Release build profile

```bash
cargo build --release
```

## Deployment rehearsal

1. Deploy on local Neo-Express.
2. Deploy on public Neo testnet.
3. Verify expected manifest fields and permissions.
4. Validate event and cross-contract behavior.

## Audit surfaces to review

- Solidity feature parity: [Feature Support](/solidity/feature-support)
- Runtime behavior: [Runtime Spec](/reference/runtime)
- Known parity limits: [Parity and Limitations](/reference/parity-limitations)
- Error diagnostics: [Error Reference](/reference/errors)
