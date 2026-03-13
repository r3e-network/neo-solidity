# CLI Reference

Complete reference for the `neo-solc` command-line compiler. This page documents every option, output format, and integration pattern.

## Command Syntax

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

## Complete Flag Summary

Quick reference of every flag accepted by `neo-solc`. Each is documented in detail below.

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
| `--nef-source`                |       | `STRING`   |            | Override NEF source metadata field                |
| `--deployer`                  |       | `HASH160`  |            | Compute predicted deployed contract hash          |
| `--deny-wildcard-permissions` |       |            |            | Fail if manifest needs full wildcard              |
| `--deny-wildcard-contracts`   |       |            |            | Fail if manifest needs wildcard contract          |
| `--deny-wildcard-methods`     |       |            |            | Fail if manifest needs wildcard methods           |
| `--manifest-permissions`      |       | `FILE`     |            | JSON file with manifest permissions               |
| `--manifest-permissions-mode` |       | `MODE`     | `merge`    | How to apply permission overrides                 |
| `--json-errors`               |       |            |            | Emit errors as JSON lines on stderr               |
| `--json-warnings`             |       |            |            | Emit warnings as JSON lines on stderr             |
| `--Wno`                       |       | `CODE`     |            | Suppress warnings by code prefix (repeatable)     |
| `--Werror`                    |       | `CODE`     |            | Promote warnings to errors by prefix (repeatable) |
| `--standard-json`             |       |            |            | Enable standard JSON input/output mode            |
| `--input`                     |       | `FILE`     | stdin      | Standard JSON input file                          |
| `--version`                   |       |            |            | Print compiler version                            |
| `--help`                      |       |            |            | Print help with all options                       |

## Core Options

### `-o, --output <FILE>`

Output prefix (normal mode) or JSON output file (standard-json mode). In normal mode this specifies the directory or file prefix for generated `.nef` and `.manifest.json` files. Defaults to the current directory if omitted.

```bash
# Write to build/ directory
neo-solc contract.sol -o build/

# Write with explicit prefix
neo-solc contract.sol -o build/MyContract
```

### `-O, --optimize <LEVEL>`

Set the optimization level from 0 to 3. Default: `2`.

| Level | Passes Enabled                                                                               | Description                                                                    |
| ----- | -------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------ |
| `0`   | None                                                                                         | No optimization. Emit IR directly as bytecode. Useful for debugging.           |
| `1`   | Constant folding                                                                             | Evaluate constant expressions at compile time. Minimal code changes.           |
| `2`   | Constant folding, dead code elimination                                                      | Remove unreachable code after returns. Good balance of size and debuggability. |
| `3`   | Constant folding, dead code elimination, function inlining, common subexpression elimination | Aggressive optimization. Smallest bytecode, but harder to debug.               |

```bash
# Debug build -- no optimization
neo-solc contract.sol -O 0 -o build/

# Production build -- maximum optimization
neo-solc contract.sol -O 3 -o build/
```

::: tip
Level 2 is the default and recommended for most workflows. Use level 3 only when bytecode size is critical and you have thorough test coverage.
:::

### `-f, --format <FORMAT>`

Output format. Default: `complete`.

| Format     | Output                                       |
| ---------- | -------------------------------------------- |
| `nef`      | Only the `.nef` bytecode file                |
| `manifest` | Only the `.manifest.json` file               |
| `complete` | Both `.nef` and `.manifest.json` (default)   |
| `assembly` | Human-readable NeoVM assembly listing        |
| `json`     | JSON representation of compilation artifacts |

```bash
# Inspect generated assembly
neo-solc contract.sol -f assembly -I devpack

# Generate only the manifest for review
neo-solc contract.sol -f manifest -I devpack -o build/
```

### `-I, --include-path <DIR>`

Add a directory to the import search path. Repeatable. The compiler searches these directories when resolving `import` statements in Solidity source files.

```bash
# Single include path
neo-solc contract.sol -I devpack -o build/

# Multiple include paths
neo-solc contract.sol -I devpack -I ./lib -I ./vendor -o build/
```

::: info
Include paths are resolved relative to the current working directory. Use absolute paths in CI pipelines to avoid ambiguity.
:::

### `--contract <NAME>`

Emit output only for the named contract. Repeatable. When a source file contains multiple contracts, use this to select which ones to compile.

```bash
# Compile only the Token contract from a multi-contract file
neo-solc contracts.sol --contract Token -o build/

# Compile two specific contracts
neo-solc contracts.sol --contract Token --contract Governance -o build/
```

### `-v, --verbose`

Enable verbose output. Prints additional information during compilation including IR function details, optimization statistics, and timing.

```bash
neo-solc contract.sol -v -I devpack -o build/
```

Example verbose output:

```
Semantic model built: 5 functions, 3 state variables
   • IR function '_deploy' (kind: Deploy) => 12 instruction(s)
   • IR function 'transfer' (kind: Public) => 28 instruction(s)
   • IR function 'balanceOf' (kind: Public) => 8 instruction(s)
```

## Native Call Options

### `--callt`

Emit `CALLT` instructions with method tokens for native contract calls instead of `SYSCALL`-based dispatch. This is a Neo N3 optimization that reduces bytecode size and gas cost for calls to native contracts (NEO, GAS, ContractManagement, etc.).

```bash
neo-solc contract.sol --callt -I devpack -o build/
```

::: tip
Use `--callt` for production deployments. The generated NEF will include a method token table that the Neo N3 runtime resolves at deploy time. This saves approximately 10-20 bytes per native call and reduces gas consumption.
:::

### `--nef-source <STRING>`

Override the value embedded in the NEF `source` metadata field. By default, the compiler uses the canonical input file path. The NEF source field is limited to 240 bytes; values exceeding this limit are truncated with a warning (`NEF_SOURCE_TRUNCATED`).

```bash
neo-solc contract.sol --nef-source "MyProject v1.0" -o build/
```

::: warning
If the value exceeds 240 bytes, it will be silently truncated and a `NEF_SOURCE_TRUNCATED` warning will be emitted. Keep source strings short.
:::

### `--deployer <HASH160>`

Compute the predicted deployed contract hash for the given sender address. The value must be a `0x`-prefixed big-endian hex UInt160 (20-byte Neo address hash).

```bash
neo-solc contract.sol --deployer 0x1234abcd...ef -I devpack -o build/
```

This is useful for contracts that need to know their own hash at compile time, or for pre-computing contract addresses in deployment scripts.

## Manifest Safety Options

These options enforce security policies on the generated manifest permissions. Use them in CI pipelines and production builds to prevent overly permissive contracts from being deployed.

### `--deny-wildcard-permissions`

Fail compilation if the manifest would require full wildcard permissions (`contract='*'` AND `methods='*'`). This is the strictest check -- it rejects any permission entry where both the contract and method fields are wildcards.

```bash
neo-solc contract.sol --deny-wildcard-permissions -I devpack -o build/
```

::: info
NEP callback methods (`onNEP11Payment`, `onNEP17Payment`, `onOracleResponse`) with `contract='*'` are exempt from `--deny-wildcard-contracts` because they must accept calls from any contract by design. However, `--deny-wildcard-permissions` still rejects full wildcards even for callbacks.
:::

### `--deny-wildcard-contracts`

Fail compilation if the manifest would require any wildcard contract scope (`contract='*'`), regardless of the methods field. Permissions that only list NEP callback methods (`onNEP11Payment`, `onNEP17Payment`, `onOracleResponse`) are exempt from this check.

```bash
neo-solc contract.sol --deny-wildcard-contracts -I devpack -o build/
```

### `--deny-wildcard-methods`

Fail compilation if the manifest would require any wildcard method scope (`methods='*'`), regardless of the contract field.

```bash
neo-solc contract.sol --deny-wildcard-methods -I devpack -o build/
```

### `--manifest-permissions <FILE>`

Path to a JSON file containing manifest permissions to merge with or replace the compiler-inferred permissions. The file can be either a bare JSON array of permission objects or an object with a `"permissions"` key.

**Array format:**

```json
[
  {
    "contract": "0xef4073a0f2b305a38ec4050e4d3d28bc40ea63f5",
    "methods": ["transfer", "balanceOf"]
  }
]
```

**Object format:**

```json
{
  "permissions": [
    {
      "contract": "0xef4073a0f2b305a38ec4050e4d3d28bc40ea63f5",
      "methods": ["transfer", "balanceOf"]
    }
  ]
}
```

Each entry requires:

- `"contract"` -- a `0x`-prefixed big-endian hex UInt160 hash or `"*"` for wildcard
- `"methods"` -- an array of method name strings or `"*"` for wildcard

```bash
neo-solc contract.sol --manifest-permissions permissions.json -I devpack -o build/
```

::: warning
Contract hashes in the permissions file must be valid `0x`-prefixed big-endian hex UInt160 values. Invalid hashes will produce a parse error with the entry index.
:::

### `--manifest-permissions-mode <MODE>`

Controls how `--manifest-permissions` entries are applied. Requires `--manifest-permissions`. Default: `merge`.

| Mode                | Behavior                                                                                                                         |
| ------------------- | -------------------------------------------------------------------------------------------------------------------------------- |
| `merge`             | Merge the provided permissions with compiler-inferred permissions. Both sets are included in the final manifest.                 |
| `replace-wildcards` | Replace only the wildcard permission entries with the provided explicit permissions. Non-wildcard inferred permissions are kept. |

```bash
# Replace wildcard permissions with explicit ones
neo-solc contract.sol \
  --manifest-permissions permissions.json \
  --manifest-permissions-mode replace-wildcards \
  -I devpack -o build/
```

::: tip
The `replace-wildcards` mode is ideal for CI pipelines: let the compiler infer permissions for known contracts, then replace only the dynamic (wildcard) entries with your explicit permission set.
:::

## Diagnostic Options

### `--json-errors`

Emit compiler errors as JSON lines on stderr. Each error is a single JSON object per line, suitable for machine parsing by editors and CI tools.

```bash
neo-solc contract.sol --json-errors -I devpack -o build/ 2>errors.jsonl
```

Output format:

```json
{
  "component": "neo-solidity",
  "severity": "error",
  "type": "CompilerError",
  "code": "VALIDATION_ERROR",
  "message": "Storage variable 'balances' must be explicitly typed",
  "formattedMessage": "Storage variable 'balances' must be explicitly typed",
  "location": {
    "file": "MyContract.sol",
    "line": 15,
    "column": 5
  }
}
```

### `--json-warnings`

Emit compiler warnings as JSON lines on stderr. Same format as `--json-errors`.

```bash
neo-solc contract.sol --json-errors --json-warnings -I devpack -o build/ 2>diagnostics.jsonl
```

### `--Wno <CODE>`

Suppress warnings matching the given code prefix. Repeatable. The code is matched as a prefix, so `--Wno W1` suppresses all warnings starting with `W1`.

```bash
# Suppress a specific warning
neo-solc contract.sol --Wno W101 -I devpack -o build/

# Suppress multiple warning categories
neo-solc contract.sol --Wno W1 --Wno W2 -I devpack -o build/
```

### `--Werror <CODE>`

Promote warnings matching the given code prefix to errors. Repeatable. Compilation will fail if any promoted warning is emitted.

```bash
# Treat security warnings as errors
neo-solc contract.sol --Werror W5 -I devpack -o build/
```

::: tip
Combine `--Werror E5` with `--Wno E2012` to enforce security warnings as errors while suppressing unused-variable noise during development.
:::

## Standard JSON Mode

The standard JSON interface provides a structured input/output format compatible with build tools and IDE integrations.

### `--standard-json`

Enable standard JSON input/output mode. When this flag is set, the compiler reads a JSON input specification and produces a JSON output. Source files are not passed as positional arguments in this mode.

### `--input <FILE>`

Path to the standard JSON input file. If omitted, the compiler reads from stdin. Requires `--standard-json`.

### `--output <FILE>` (in standard-json mode)

Path to write the standard JSON output. If omitted, the compiler writes to stdout.

```bash
# File-based standard JSON
neo-solc --standard-json --input input.json --output output.json

# Pipe-based standard JSON
neo-solc --standard-json < input.json > output.json

# Mixed: file input, stdout output
neo-solc --standard-json --input input.json
```

### Standard JSON Input Format

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

### Standard JSON Output Format

```json
{
  "contracts": {
    "MyContract.sol": {
      "MyContract": {
        "abi": { ... },
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

## Help and Version

```bash
# Print help with all options
neo-solc --help

# Print compiler version
neo-solc --version
```

## Exit Codes

| Code | Meaning                                              |
| ---- | ---------------------------------------------------- |
| `0`  | Compilation succeeded                                |
| `1`  | Compilation error (source errors, semantic failures) |
| `2`  | Invalid command-line arguments                       |
| `3`  | File not found (source file or include path)         |
| `4`  | Permission denied (filesystem)                       |

## Integration Patterns

### Makefile

```makefile
NEO_SOLC := neo-solc
DEVPACK  := devpack
BUILD    := build
SOURCES  := $(wildcard contracts/*.sol)
NEFS     := $(patsubst contracts/%.sol,$(BUILD)/%.nef,$(SOURCES))

.PHONY: all clean

all: $(NEFS)

$(BUILD)/%.nef: contracts/%.sol | $(BUILD)
	$(NEO_SOLC) $< -I $(DEVPACK) -O 3 --callt \
		--deny-wildcard-permissions -o $(BUILD)/

$(BUILD):
	mkdir -p $(BUILD)

clean:
	rm -rf $(BUILD)
```

### CI Pipeline (GitHub Actions)

```yaml
- name: Compile contracts
  run: |
    neo-solc contracts/*.sol \
      -I devpack \
      -O 3 \
      --callt \
      --deny-wildcard-permissions \
      --deny-wildcard-contracts \
      --json-errors \
      -o build/ 2>compile-errors.jsonl

- name: Check for warnings
  run: |
    neo-solc contracts/*.sol \
      -I devpack \
      --json-warnings \
      --Werror W5 \
      -o /dev/null 2>warnings.jsonl
    if [ -s warnings.jsonl ]; then
      echo "Security warnings found"
      cat warnings.jsonl
      exit 1
    fi
```

### Shell Script

```bash
#!/usr/bin/env bash
set -euo pipefail

CONTRACT="$1"
OUTPUT_DIR="${2:-build}"

mkdir -p "$OUTPUT_DIR"

neo-solc "$CONTRACT" \
  -I devpack \
  -O 2 \
  --callt \
  --deny-wildcard-permissions \
  --manifest-permissions permissions.json \
  --manifest-permissions-mode replace-wildcards \
  --json-errors \
  --json-warnings \
  -o "$OUTPUT_DIR" 2>"$OUTPUT_DIR/diagnostics.jsonl"

echo "Compiled: $OUTPUT_DIR/$(basename "${CONTRACT%.sol}").nef"
```

## Common Recipes

### Debug compilation with full diagnostics

```bash
neo-solc contract.sol -O 0 -v --json-errors --json-warnings -I devpack -o build/
```

### Production build with all safety checks

```bash
neo-solc contract.sol \
  -O 3 \
  --callt \
  --deny-wildcard-permissions \
  --deny-wildcard-contracts \
  --deny-wildcard-methods \
  --manifest-permissions permissions.json \
  --manifest-permissions-mode replace-wildcards \
  -I devpack \
  -o build/
```

### Inspect assembly output

```bash
neo-solc contract.sol -f assembly -O 2 -I devpack
```

### Compile specific contract from multi-contract file

```bash
neo-solc contracts.sol --contract MyToken -O 2 -I devpack -o build/
```

### Predict deployed contract hash

```bash
neo-solc contract.sol --deployer 0xabcdef...1234 -I devpack -o build/
```

### Batch compile with error collection

```bash
for sol in contracts/*.sol; do
  neo-solc "$sol" -I devpack -O 2 --json-errors -o build/ 2>>"errors.jsonl"
done
```

## See Also

- [Error Reference](/advisory-content/error-reference) -- diagnostic codes and failure scenarios
- [Troubleshooting](/advisory-content/troubleshooting) -- common issues and solutions
- [Architecture](/internals/architecture) -- compiler pipeline details
- [Runtime Specification](/internals/runtime-specification) -- NeoVM execution model
