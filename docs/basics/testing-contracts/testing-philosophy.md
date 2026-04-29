---
title: "Testing Contracts: Testing Philosophy"
description: "Testing Philosophy from Testing Contracts."
---

# Testing Philosophy

[Back to Testing Contracts](/basics/testing-contracts)

The test suite is organized in layers of increasing scope:

| Layer                | What It Tests                                                                  | Count       | Command                                   |
| -------------------- | ------------------------------------------------------------------------------ | ----------- | ----------------------------------------- |
| Unit tests           | Individual compiler components (lexer, parser, semantic, IR, codegen, runtime) | 400+        | `cargo test --workspace`                  |
| E2E compilation      | Full pipeline from Solidity source to NEF + manifest                           | 74 tests    | `cargo test --test e2e_compilation_tests` |
| Conformance          | Output correctness against reference Neo implementations                       | 32 vectors  | `cargo test --test conformance_tests`     |
| Strict compatibility | Compilation of all devpack and example contracts                               | ~50 files   | `make test-compile-strict`                |
| Neo-Express smoke    | Deploy + invoke on a real local Neo chain                                      | 16+ scripts | `make test-deploy-smoke-full`             |
| Tooling              | TypeScript packages (Hardhat, Foundry, ABI router)                             | varies      | `make tooling-test`                       |
| C# runtime           | EVM-compatible runtime primitives                                              | varies      | `make runtime-test`                       |

Each layer catches different classes of bugs. Unit tests catch logic errors in individual passes. E2E tests catch integration issues between passes. Conformance tests catch behavioral divergence from Neo N3. Smoke tests catch deployment and invocation failures that only manifest on a real chain.

::: tip Quick Feedback Loop
During development, `cargo test --workspace` is the fastest way to validate changes. It runs all Rust tests (unit + integration + E2E + conformance) in under 30 seconds.
:::
