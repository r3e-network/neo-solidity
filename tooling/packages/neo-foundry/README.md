# @neo-solidity/neo-foundry

Neo Foundry is the companion CLI that mirrors Foundry/Hardhat ergonomics for the Neo chain. It is still mostly scaffold-level, but `neo-forge init` now creates a working project layout and config, and the package can be imported as a library without executing the CLI parser as a side effect.

## Current State (Nov 2025)

| Command | Status |
| --- | --- |
| `neo-forge init` | ✅ Creates `neo-foundry.toml`, `src/Counter.sol`, and `test/Counter.t.sol` |
| `neo-forge build/test/clean` | ⚠️ Build/test remain scaffold-only; they stop with explicit “not implemented yet” errors |
| `neo-cast` (`call`, `send`, `balance`, etc.) | ⚠️ Reads configs and validates inputs but does not submit real transactions |
| `neo-anvil` | ⚠️ In-memory mock chain with a JSON-RPC façade; useful for CLI demos, not for executing NEF code |
| `neo-foundry` programmatic API (`NeoForge`, `NeoCast`, `NeoAnvil`) | ⚠️ Typed stubs raising “not implemented” errors when called |

## Installation

```bash
npm install -g @neo-solidity/neo-foundry
```

The CLI exposes three entry points:

- `neo-forge` – project/build/test workflows (`init` implemented; build/test still placeholder)
- `neo-cast` – contract interaction helper (placeholder)
- `neo-anvil` – mock RPC server for local development demos

## Configuration

The package understands a Foundry-like `neo-foundry.toml` via `ConfigManager`, and `neo-forge init` writes that file plus a starter project skeleton. Future work will connect the same config to real compiler runs, RPC signing, and artifact resolution.

## Roadmap

1. Wire `neo-forge build` to the Neo Solidity compiler pipeline and persist artifacts.
2. Implement `neo-forge test` by driving the Neo VM test harness.
3. Turn `neo-cast send/deploy` into real RPC transactions using the shared `NeoRpcClient`.
4. Replace `neo-anvil` with an actual Neo VM sandbox (or connect to Neo Express).

Until the roadmap ships, expect every command to terminate with an explicit “feature not implemented” message. This documentation keeps expectations clear for anyone installing the package from npm.
