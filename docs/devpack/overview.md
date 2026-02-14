# Devpack Overview

The devpack provides Solidity-facing libraries and standards for Neo N3.

Important: these libraries are primarily compiler intrinsics surfaces.

## Layout

- `devpack/contracts/`: syscall/native-call wrappers and framework contracts
- `devpack/libraries/`: high-level helpers (`Runtime`, `Storage`, `Neo`)
- `devpack/standards/`: NEP-17, NEP-11, NEP-24 contracts
- `devpack/examples/`: production-oriented usage examples

## Core capabilities

- Neo runtime and storage intrinsics
- Native contract wrappers
- NEP token standard bases
- ERC-to-NEP migration diagnostics and patterns

## Usage

Compile contracts importing devpack files with include path:

```bash
neo-solc MyContract.sol -I devpack -O2 -o build/MyContract
```

## Permission-conscious development

Prefer fixed target/method wrappers (`NativeCalls.*`) to avoid wildcard permissions.

Validate with strict flags:

```bash
neo-solc MyContract.sol -I devpack \
  --deny-wildcard-contracts \
  --deny-wildcard-methods \
  -o build/MyContract
```

## References

- [`devpack/README.md`](https://github.com/r3e-network/neo-solidity/blob/main/devpack/README.md)
- [`devpack/DEVPACK_GUIDE.md`](https://github.com/r3e-network/neo-solidity/blob/main/devpack/DEVPACK_GUIDE.md)
- [Standards Mapping](/devpack/standards)
