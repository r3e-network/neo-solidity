# Compile Contracts

This compiler targets Neo N3 directly and emits deployment-ready artifacts.

## Core command

```bash
neo-solc <source.sol> -o <output-prefix>
```

For source build usage:

```bash
./target/release/neo-solc <source.sol> -o <output-prefix>
```

## Typical usage patterns

### Single contract

```bash
neo-solc contract.sol -I devpack -O2 -o build/contract
```

### Explicit output format

```bash
neo-solc contract.sol -f nef -o build/contract.nef
neo-solc contract.sol -f manifest -o build/contract.manifest.json
neo-solc contract.sol -f assembly -o build/contract.asm
neo-solc contract.sol -f json -o build/contract.json
```

### Native call optimization (CALLT)

```bash
neo-solc contract.sol --callt -O3 -o build/contract
```

### Compile only selected contract names

```bash
neo-solc multi.sol --contract Token --contract Vault -o build/out
```

### Batch compile examples

```bash
mkdir -p build/examples
for f in examples/*.sol; do
  neo-solc "$f" -I devpack -O2 -o "build/examples/$(basename "$f" .sol)"
done
```

## Solidity Standard JSON mode

```bash
neo-solc --standard-json --input input.json --output output.json
```

Use this mode for tool integrations that follow Solidity's JSON interface.

## Manifest permission hardening

Enforce least privilege in CI:

```bash
neo-solc contract.sol \
  --deny-wildcard-permissions \
  --deny-wildcard-contracts \
  --deny-wildcard-methods \
  -o build/contract
```

Override/merge permissions from JSON:

```bash
neo-solc contract.sol \
  --manifest-permissions permissions.json \
  --manifest-permissions-mode replace-wildcards \
  -o build/contract
```

See [Manifest Spec](/manifests/manifest-spec) for exact behavior.
