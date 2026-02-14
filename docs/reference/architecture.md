# Architecture

Full architecture documentation:

- [`docs/ARCHITECTURE.md`](../ARCHITECTURE.md)

## Pipeline summary

1. Parse Solidity (0.8.x)
2. Build semantic model and diagnostics
3. Lower into Neo-oriented IR
4. Optimize IR
5. Generate NeoVM bytecode
6. Emit NEF + manifest artifacts

## Key implementation areas

- `src/frontend*` and `src/solidity*`: parse/metadata
- `src/ir/`: lowering and typed IR
- `src/optimizer*`: optimization passes
- `src/cli/bytecode/`: bytecode emission
- `src/cli/cli_parts/cli_manifest/`: manifest generation and permission inference
- `src/runtime/`: embedded NeoVM/runtime emulation
