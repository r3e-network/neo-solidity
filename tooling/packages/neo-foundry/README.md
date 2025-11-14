# @neo-solidity/neo-foundry

Neo Foundry is the companion CLI that mirrors Foundry/Hardhat ergonomics for the Neo chain. **It is still a scaffold**: all commands print the intended UX and stop with clear messages instead of touching live RPC nodes. Treat it as a preview until deployment/testing support lands.

## Current State (Nov 2025)

| Command | Status |
| --- | --- |
| `neo-forge build/test/clean` | ⚠️ Prints placeholder output and throws `HardhatPluginError` equivalents; no compiler/runtime integration yet |
| `neo-cast` (`call`, `send`, `balance`, etc.) | ⚠️ Reads configs and validates inputs but does not submit real transactions |
| `neo-anvil` | ⚠️ In-memory mock chain with a JSON-RPC façade; useful for CLI demos, not for executing NEF code |
| `neo-foundry` programmatic API (`NeoForge`, `NeoCast`, `NeoAnvil`) | ⚠️ Typed stubs raising “not implemented” errors when called |

## Installation

```bash
npm install -g @neo-solidity/neo-foundry
```

The CLI exposes three entry points:

- `neo-forge` – project/build/test workflows (currently placeholder)
- `neo-cast` – contract interaction helper (placeholder)
- `neo-anvil` – mock RPC server for local development demos

## Configuration

The scaffold understands a Foundry-like `neo-foundry.toml` via `ConfigManager`, but the values are only used for logging today. Future work will connect these options to real compiler runs, RPC signing, and artifact resolution.

## Roadmap

1. Wire `neo-forge build` to the Neo Solidity compiler pipeline and persist artifacts.
2. Implement `neo-forge test` by driving the Neo VM test harness.
3. Turn `neo-cast send/deploy` into real RPC transactions using the shared `NeoRpcClient`.
4. Replace `neo-anvil` with an actual Neo VM sandbox (or connect to Neo Express).

Until the roadmap ships, expect every command to terminate with an explicit “feature not implemented” message. This documentation keeps expectations clear for anyone installing the package from npm.
