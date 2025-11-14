# @neo-solidity/cli-tools

CLI utility scaffolding for the Neo Solidity toolchain. The package exports the new compiler CLI (`neo-solc`), Cast-like helpers, and a shared CLI framework. **Everything is still a scaffold**: commands print the intended UX and throw `Error/HardhatPluginError` before performing real work.

## Status

| Command / API | Status |
| --- | --- |
| `neo-solc` compile (`CompilerCLI`) | ⚠️ Constructs JSON input and shells out to `neo-solc`, but the surrounding task runner (artifact persistence, plugin integration) is still under construction |
| CLI framework (`NeoSolidityCLI`) | ✅ Usable to register commands/log/spinners. Can power your own scripts. |
| Utility helpers in `src/cli-framework.ts` | ✅ Logging/spinner/progress abstractions |
| Higher-level CLI commands (forge/cast/anvil wrappers) | ⚠️ Each prints placeholder output and throws “not implemented yet”. |

## Installation

```bash
npm install -g @neo-solidity/cli-tools
```

## Usage

```bash
# Compile contracts (scaffold)
neo-solc compile contracts/**/*.sol --optimize --gas-model hybrid

# Show version (delegates to bundled neo-solc path)
neo-solc --version
```

The `CompilerCLI` module is also exportable so tool authors can integrate it programmatically:

```ts
import { CompilerCLI } from "@neo-solidity/cli-tools";

const cli = new CompilerCLI();
await cli.compile(["contracts/MyToken.sol"], { optimize: true });
```

## Roadmap

1. Wire compiler output into the artifact format consumed by `@neo-solidity/hardhat-solc-neo` and Neo Foundry.
2. Move `neo-forge`/`neo-cast` commands into this package once they perform real RPC transactions.
3. Provide a `neo-anvil` CLI that launches a real Neo Express instance or VM sandbox.

Until these land, consider the CLI tools experimental.
