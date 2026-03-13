# Using the Compiler

## Using the Commandline Compiler

One of the build targets of the Neo Solidity repository is `neo-solc`, the commandline compiler. Using `neo-solc --help` provides you with an explanation of all options. The compiler can produce various outputs, ranging from simple binaries and JSON manifests over assembly to standard JSON configurations.

### Command Syntax

```bash
neo-solc <source...> [options]
```

The compiler accepts one or more Solidity source files and produces Neo N3 deployment artifacts (`.nef` + `.manifest.json`).

```bash
# Single file
neo-solc MyContract.sol -o build/

# Multiple files
neo-solc contracts/*.sol -o build/

# With devpack imports
neo-solc MyToken.sol -I devpack -o build/
```

### Complete Flag Summary

| Flag                          | Short | Argument   | Default    | Description                                       |
| ----------------------------- | ----- | ---------- | ---------- | ------------------------------------------------- |
| `<source...>`                 |       | positional | required   | Input Solidity file(s)                            |
| `--output`                    | `-o`  | `FILE`     | `.`        | Output prefix/directory or JSON output file       |
| `--optimize`                  | `-O`  | `LEVEL`    | `2`        | Optimization level (0-3)                          |
| `--format`                    | `-f`  | `FORMAT`   | `complete` | Output format                                     |
| `--include-path`              | `-I`  | `DIR`      |            | Import search path (repeatable)                   |
| `--contract`                  |       | `NAME`     |            | Emit only named contract (repeatable)             |
| `--verbose`                   | `-v`  |            |            | Enable verbose output                             |
| `--callt`                     |       |            |            | Emit CALLT + method tokens for native calls       |
| `--deny-wildcard-permissions` |       |            |            | Fail if manifest needs full wildcard              |
| `--deny-wildcard-contracts`   |       |            |            | Fail if manifest needs wildcard contract          |
| `--deny-wildcard-methods`     |       |            |            | Fail if manifest needs wildcard methods           |
| `--manifest-permissions`      |       | `FILE`     |            | JSON file with manifest permissions               |
| `--manifest-permissions-mode` |       | `MODE`     | `merge`    | How to apply permission overrides                 |
| `--standard-json`             |       |            |            | Enable standard JSON input/output mode            |

### Base Path and Include Paths

The compiler resolves `import` directives by looking into the current directory and any directories provided via `--include-path` (or `-I`).

```bash
neo-solc contract.sol -I devpack -I ./lib -o build/
```

## Setting the EVM Version to Target

::: tip 💡 NeoVM Difference
Neo Solidity targets Neo N3 (`NeoVM`) exclusively. Standard EVM version targets (like `istanbul`, `paris`, `cancun`) do not apply and are safely ignored by the compiler if provided through standard JSON configurations.
:::

Because Neo N3 relies on standard execution mechanics and syscall interfaces, versioning is primarily handled at the node runtime level rather than requiring specific hardfork flags during compilation.

## Compiler Input and Output JSON Description

These JSON formats are used by the compiler API as well as the standard JSON interface of the commandline compiler. These interfaces are recommended for any programmatic interactions (like IDEs and framework toolchains).

To use the standard JSON interface from the command line, run:
```bash
neo-solc --standard-json --input input.json > output.json
```

### Input JSON Description

The input JSON format is completely compatible with the Ethereum Solidity Standard JSON format.

```json
{
  "language": "Solidity",
  "sources": {
    "MyContract.sol": {
      "content": "// SPDX-License-Identifier: MIT\npragma solidity ^0.8.0;\n..."
    }
  },
  "settings": {
    "optimizer": {
      "enabled": true,
      "level": 2
    },
    "outputSelection": {
      "*": {
        "*": ["abi", "nef", "manifest"]
      }
    }
  }
}
```

### Output JSON Description

The output JSON includes any compiler diagnostics (errors/warnings) and the compiled artifacts.

::: tip 💡 NeoVM Difference
In EVM output, contracts produce `evm.bytecode` and `abi` fields. In Neo Solidity, the output produces a Base64-encoded `nef` string and the structural Neo `manifest`.
:::

```json
{
  "contracts": {
    "MyContract.sol": {
      "MyContract": {
        "abi": { 
            "methods": [ ... ],
            "events": [ ... ]
        },
        "nef": "<base64-encoded NEF>",
        "manifest": { ... }
      }
    }
  },
  "errors": [
    {
      "component": "neo-solidity",
      "severity": "warning",
      "code": "COMPILER_WARNING",
      "message": "...",
      "formattedMessage": "...",
      "location": { "file": "MyContract.sol", "line": 10, "column": 1 }
    }
  ]
}
```