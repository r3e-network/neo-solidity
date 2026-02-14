# Quick Start

This path takes you from source code to deployable Neo N3 artifacts in minutes.

## 1. Build the compiler

```bash
git clone https://github.com/r3e-network/neo-solidity.git
cd neo-solidity
cargo build --release
```

Compiler binary:

- `target/release/neo-solc`

## 2. Compile a contract

```bash
./target/release/neo-solc examples/SimpleStorage.sol -I devpack -O2 -o build/SimpleStorage
```

Output:

- `build/SimpleStorage.nef`
- `build/SimpleStorage.manifest.json`

## 3. Inspect generated manifest

```bash
jq '.name, .supportedstandards, .abi.methods | length' build/SimpleStorage.manifest.json
```

## 4. Compile with strict manifest policy (recommended)

```bash
./target/release/neo-solc examples/SimpleStorage.sol \
  -I devpack \
  --deny-wildcard-contracts \
  --deny-wildcard-methods \
  -o build/SimpleStorageStrict
```

## 5. Validate end-to-end quickly

```bash
bash examples/test_compilation.sh
make test-deploy-smoke
```

## Next

1. [Compile Workflow](/workflows/compile)
2. [Deploy Workflow](/workflows/deploy)
3. [Test Workflow](/workflows/test)
4. [Manifest Spec](/manifests/manifest-spec)
