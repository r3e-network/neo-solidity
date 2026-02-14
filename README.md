# Documentation Index

This index provides an overview of all documentation available in the Neo Solidity Compiler project.

## Getting Started

| Document                              | Description                                                              |
| ------------------------------------- | ------------------------------------------------------------------------ |
| [README.md](../README.md)             | Main project documentation with quick start, examples, and API reference |
| [CONTRIBUTING.md](../CONTRIBUTING.md) | Guidelines for contributing to the project                               |
| [TESTING.md](../TESTING.md)           | Testing framework documentation                                          |
| [SECURITY.md](../SECURITY.md)         | Security policy and best practices                                       |

## Architecture

| Document                                            | Description                                               |
| --------------------------------------------------- | --------------------------------------------------------- |
| [docs/ARCHITECTURE.md](ARCHITECTURE.md)             | Detailed architecture of the compiler and runtime         |
| [docs/RUNTIME_SPEC.md](RUNTIME_SPEC.md)             | NeoVM runtime specification and opcode support            |
| [docs/NEO_VM_PARITY_TODO.md](NEO_VM_PARITY_TODO.md) | Known gaps between implementation and NeoVM specification |
| [docs/ERROR_REFERENCE.md](ERROR_REFERENCE.md)       | Error codes, warnings, and troubleshooting guide          |

## Design Documents

| Document                                                      | Description                                  |
| ------------------------------------------------------------- | -------------------------------------------- |
| [docs/mapping_lowering_design.md](mapping_lowering_design.md) | Design for Solidity mapping storage lowering |

## Archive

Historical documents were removed during repository cleanup to keep the project focused on
active compiler/devpack documentation. See [`docs/archive/README.md`](archive/README.md).

## Quick Links

### For Users

1. Read [README.md](../README.md) for quick start
2. Check [examples/](../examples/) for sample contracts
3. Run `neo-solc --help` for CLI options

### For Developers

1. Read [CONTRIBUTING.md](../CONTRIBUTING.md) for contribution guidelines
2. Review [ARCHITECTURE.md](ARCHITECTURE.md) for system design
3. Check [RUNTIME_SPEC.md](RUNTIME_SPEC.md) for runtime details

### For Security

1. Read [SECURITY.md](../SECURITY.md) for security policy
2. Review known issues in [NEO_VM_PARITY_TODO.md](NEO_VM_PARITY_TODO.md)
3. Report vulnerabilities to security@r3e.network

## Document Updates

When updating documentation:

- Update this index if adding new documents
- Keep README.md as the primary entry point
- Add document metadata (author, date) for design docs
- Keep `docs/archive/README.md` up to date when archiving/removing docs

## Version Compatibility

- Rust: 1.70+
- Node.js: 16.0+ (for tooling)
- Neo N3: 3.0+

See [README.md](../README.md) for detailed requirements.
