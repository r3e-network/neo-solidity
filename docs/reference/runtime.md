# Runtime Specification

The embedded runtime specification is documented in:

- [`docs/RUNTIME_SPEC.md`](../RUNTIME_SPEC.md)

## Highlights

- Broad Neo N3 opcode family coverage
- Structured exception support (`TRY`/`ENDTRY`/`ENDFINALLY` semantics)
- Storage and iterator syscall handling
- Native contract integration surfaces
- Syscall gas hinting for closer runtime behavior

## What to use this for

- Understanding generated contract execution semantics
- Interpreting runtime-related tests/failures
- Reviewing parity boundaries before production rollout

For known remaining fidelity gaps, see [Parity and Limitations](/reference/parity-limitations).
