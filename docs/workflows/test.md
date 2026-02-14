# Test Contracts

The project includes layered validation for compiler, runtime, and deployment behavior.

## Fast checks

```bash
make test
bash examples/test_compilation.sh
```

## Compiler strict compatibility sweep

```bash
make test-compile-strict
```

## Neo-Express smoke suites

```bash
make test-deploy-smoke-full
make test-deploy-new-showcases-smoke
```

## Workspace-wide gates

```bash
make test-all
make test-all-full
```

## Recommended CI sequence

1. `cargo fmt --all -- --check`
2. `cargo clippy --all-targets --all-features -- -D warnings`
3. `cargo test --workspace --all-features`
4. `make test-compile-strict`
5. `make test-deploy-smoke-full`

For one-command validation, use [Production Readiness](/workflows/production).
