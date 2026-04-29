---
title: "Test Contracts: Conformance Tests"
description: "Conformance Tests from Test Contracts."
---

# Conformance Tests

[Back to Test Contracts](/basics/testing-contracts)

Conformance tests compare compiler output against reference Neo N3 implementations:

```bash
cargo test --test conformance_tests
```

The suite includes 32 test vectors with a 93.8% pass rate. Each vector specifies:

- Input Solidity source
- Expected NeoVM opcodes or behavior
- Expected manifest structure

Failing conformance tests indicate behavioral divergence from the Neo N3 reference. These are tracked in `docs/NEO_VM_PARITY_TODO.md`.

::: warning Conformance Gaps
A 93.8% pass rate means some edge cases diverge from the Neo N3 reference. Check the parity TODO before relying on behavior covered by failing vectors.
:::
