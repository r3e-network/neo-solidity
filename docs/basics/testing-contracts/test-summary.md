---
title: "Test Contracts: Test Summary"
description: "Test Summary from Test Contracts."
---

# Test Summary

[Back to Test Contracts](/basics/testing-contracts)

| Command                                   | Scope               | Duration | Requires                |
| ----------------------------------------- | ------------------- | -------- | ----------------------- |
| `cargo test --workspace`                  | All Rust tests      | ~30s     | Rust                    |
| `cargo test --test e2e_compilation_tests` | E2E compilation     | ~10s     | Rust                    |
| `cargo test --test conformance_tests`     | Conformance vectors | ~5s      | Rust                    |
| `make test-compile-strict`                | Strict sweep        | ~20s     | Rust                    |
| `make test-deploy-smoke`                  | Basic deploy        | ~30s     | Rust, .NET, jq          |
| `make test-deploy-smoke-full`             | All smoke tests     | ~5min    | Rust, .NET, jq          |
| `make test-all`                           | Rust + tooling      | ~1min    | Rust, Node.js           |
| `make production-gate`                    | Everything          | ~8min    | Rust, Node.js, .NET, jq |
