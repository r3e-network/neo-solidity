# Parity and Limitations

Runtime parity tracking lives in:

- [`docs/NEO_VM_PARITY_TODO.md`](../NEO_VM_PARITY_TODO.md)

## Current focus areas

- Gas precision parity vs Neo reference behavior
- Exception/gas edge-case verification
- Iterator streaming efficiency
- ByteString vs Buffer distinction depth
- Broader native contract method surface completeness

## Compiler-level limitations to treat explicitly

- EVM-only blocked features (`delegatecall`, inline assembly, bytecode introspection)
- Partial features with Neo-specific semantics (`msg.value`, overload dispatch constraints)
- Dynamic call sites that can require wildcard permissions

## Deployment guidance

Treat parity and limitation docs as release criteria inputs, not optional reading:

1. Confirm your contract avoids blocked features.
2. Confirm manifest safety flags pass.
3. Run smoke tests on local Neo-Express and testnet before mainnet.
