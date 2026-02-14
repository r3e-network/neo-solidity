# Compile Contracts

The `neo-solc` compiler translates Solidity 0.8.x source files into Neo N3 deployment artifacts. This page covers every CLI option, output format, and compilation pattern.

## Core Command

```bash
neo-solc <source.sol> [OPTIONS]
```

If you built from source and have not installed the binary system-wide:

```bash
./target/release/neo-solc <source.sol> [OPTIONS]
```

The default behavior compiles the input file with optimization level 2 and writes both `.nef` and `.manifest.json` to the current directory.

## CLI Options Reference

### Input and Output

| Option                 | Short | Description                                                                    |
| ---------------------- | ----- | ------------------------------------------------------------------------------ |
| `<source>...`          |       | One or more Solidity input files (positional)                                  |
| `--output <FILE>`      | `-o`  | Output prefix or directory. Generates `<FILE>.nef` and `<FILE>.manifest.json`. |
| `--format <FORMAT>`    | `-f`  | Output format: `complete` (default), `nef`, `manifest`, `assembly`, `json`     |
| `--contract <NAME>`    |       | Only emit outputs for the named contract (repeatable)                          |
| `--include-path <DIR>` | `-I`  | Additional import search path (repeatable)                                     |
| `--verbose`            | `-v`  | Print detailed compilation progress                                            |

### Optimization

| Option               | Short | Description                                                       |
| -------------------- | ----- | ----------------------------------------------------------------- |
| `--optimize <LEVEL>` | `-O`  | Optimization level 0-3 (default: `2`)                             |
| `--callt`            |       | Emit CALLT instructions + method tokens for native contract calls |

### Manifest Hardening

| Option                               | Description                                                                                                     |
| ------------------------------------ | --------------------------------------------------------------------------------------------------------------- |
| `--deny-wildcard-permissions`        | Fail if the manifest requires full wildcard permissions (`contract='*'` AND `methods='*'`)                      |
| `--deny-wildcard-contracts`          | Fail if any permission entry has `contract='*'`                                                                 |
| `--deny-wildcard-methods`            | Fail if any permission entry has `methods='*'`                                                                  |
| `--manifest-permissions <FILE>`      | Path to a JSON file with explicit permissions (array or `{"permissions": [...]}`)                               |
| `--manifest-permissions-mode <MODE>` | How to apply the permissions file: `merge` (default) or `replace-wildcards`. Requires `--manifest-permissions`. |

### Diagnostics

| Option            | Description                                                                                   |
| ----------------- | --------------------------------------------------------------------------------------------- |
| `--json-errors`   | Emit compiler errors as JSON lines on stderr                                                  |
| `--json-warnings` | Emit compiler warnings as JSON lines on stderr                                                |
| `--Wno <CODE>`    | Suppress warnings matching the given code prefix (repeatable, e.g., `--Wno W101`)             |
| `--Werror <CODE>` | Promote warnings matching the given code prefix to errors (repeatable, e.g., `--Werror W101`) |

### Metadata

| Option                  | Description                                                                                    |
| ----------------------- | ---------------------------------------------------------------------------------------------- |
| `--nef-source <STRING>` | Override the NEF `source` field (defaults to the canonical input path)                         |
| `--deployer <HASH160>`  | Compute the predicted deployed contract hash for the given sender (0x-prefixed big-endian hex) |

### Standard JSON Mode

| Option            | Description                                                                       |
| ----------------- | --------------------------------------------------------------------------------- |
| `--standard-json` | Use Solidity standard JSON input/output mode                                      |
| `--input <FILE>`  | Path to standard JSON input file (defaults to stdin). Requires `--standard-json`. |

When `--standard-json` is used, `--output` specifies the JSON output file (defaults to stdout).

## Optimization Levels

The optimizer applies progressively more aggressive transformations:

| Level | Name       | Passes                                                              | Use Case                         |
| ----- | ---------- | ------------------------------------------------------------------- | -------------------------------- |
| `-O0` | None       | No optimization                                                     | Debugging, inspecting raw output |
| `-O1` | Basic      | Dead code elimination, constant folding/propagation                 | Testing, CI builds               |
| `-O2` | Standard   | + function inlining (bounded), peephole optimizations               | Production builds (default)      |
| `-O3` | Aggressive | + stack height reduction, NeoVM-specific cleanups, max optimization | Performance-critical contracts   |

### Comparing Optimization Levels

```bash
# Compile at each level and compare NEF sizes
neo-solc contract.sol -O0 -o build/contract-O0
neo-solc contract.sol -O1 -o build/contract-O1
neo-solc contract.sol -O2 -o build/contract-O2
neo-solc contract.sol -O3 -o build/contract-O3

ls -la build/contract-O*.nef
```

::: tip
`-O2` is the default and recommended for most production contracts. Use `-O3` only when bytecode size or execution cost is critical. Use `-O0` when you need to inspect the generated assembly without optimization noise.
:::

## Output Formats

The `-f` / `--format` flag controls what the compiler writes:

| Format     | Files Generated           | Description                                  |
| ---------- | ------------------------- | -------------------------------------------- |
| `complete` | `.nef` + `.manifest.json` | Both deployment artifacts (default)          |
| `nef`      | `.nef`                    | NeoVM bytecode only                          |
| `manifest` | `.manifest.json`          | Contract manifest only                       |
| `assembly` | `.asm`                    | Human-readable NeoVM disassembly             |
| `json`     | `.json`                   | JSON bundle with bytecode, ABI, and metadata |

### Examples

```bash
# Default: both files
neo-solc contract.sol -o build/contract

# NEF only
neo-solc contract.sol -f nef -o build/contract.nef

# Manifest only
neo-solc contract.sol -f manifest -o build/contract.manifest.json

# NeoVM assembly (useful for debugging)
neo-solc contract.sol -f assembly -o build/contract.asm

# JSON bundle
neo-solc contract.sol -f json -o build/contract.json
```

## Include Paths and Import Resolution

Solidity `import` statements are resolved against include paths. The `-I` flag adds directories to the search path. It is repeatable.

```bash
# Single include path
neo-solc contracts/Token.sol -I devpack -o build/Token

# Multiple include paths
neo-solc contracts/Token.sol -I devpack -I contracts -I lib -o build/Token
```

The compiler searches for imported files in this order:

1. Relative to the importing file's directory
2. Each `-I` directory, in the order specified

The `devpack/` directory ships with the repository and provides Solidity interfaces for Neo N3 syscalls, native contracts, and NEP standards. Most contracts should include it:

```bash
neo-solc contract.sol -I devpack -o build/contract
```

## Multi-Contract Compilation

### The `--contract` Flag

When a Solidity file defines multiple contracts (or imports files that do), the compiler emits artifacts for all of them by default. Use `--contract` to select specific contracts:

```bash
# Only emit Token artifacts
neo-solc multi.sol --contract Token -o build/Token

# Emit both Token and Vault
neo-solc multi.sol --contract Token --contract Vault -o build/out
```

This is useful when your main contract imports helper contracts or interfaces that should not be deployed separately.

### Batch Compilation

Compile all Solidity files in a directory:

```bash
mkdir -p build/examples
for f in examples/*.sol; do
  neo-solc "$f" -I devpack -O2 -o "build/examples/$(basename "$f" .sol)"
done
```

Or compile multiple files in a single invocation:

```bash
neo-solc contracts/*.sol -o build/
```

## CALLT and Method Tokens

The `--callt` flag enables a Neo N3 optimization for native contract calls. Instead of using `System.Contract.Call` with a contract hash and method name, the compiler emits `CALLT` instructions that reference a method token table embedded in the NEF.

```bash
neo-solc contract.sol --callt -O3 -o build/contract
```

Benefits:

- Smaller bytecode (no inline contract hash + method name strings)
- Faster execution (direct token lookup instead of hash resolution)
- Applies to calls to Neo native contracts (NeoToken, GasToken, PolicyContract, LedgerContract, CryptoLib, StdLib, ContractManagement, RoleManagement)

::: info
CALLT is a Neo N3 optimization. It does not change the contract's behavior, only its efficiency. Contracts compiled with and without `--callt` are functionally identical.
:::

## Standard JSON Mode

For integration with Solidity-compatible toolchains (Hardhat, Foundry), the compiler supports the standard JSON input/output interface:

```bash
# Via stdin/stdout
neo-solc --standard-json < input.json > output.json

# Via explicit files
neo-solc --standard-json --input input.json --output output.json
```

The input JSON follows the Solidity compiler's standard JSON format. The output JSON contains compilation results, errors, and warnings.

## Manifest Permission Hardening

Neo N3 manifests declare which contracts and methods a contract is allowed to call. By default, the compiler emits wildcard permissions (`contract: '*', methods: '*'`), which grants unrestricted access. For production contracts, you should restrict permissions.

### Deny Wildcard Flags

```bash
# Fail if full wildcard (contract='*' AND methods='*')
neo-solc contract.sol --deny-wildcard-permissions -o build/contract

# Fail if any wildcard contract
neo-solc contract.sol --deny-wildcard-contracts -o build/contract

# Fail if any wildcard methods
neo-solc contract.sol --deny-wildcard-methods -o build/contract

# Strictest: deny both
neo-solc contract.sol \
  --deny-wildcard-contracts \
  --deny-wildcard-methods \
  -o build/contract
```

### Permission Override Files

When a contract uses dynamic calls that the compiler cannot statically narrow, provide an explicit allowlist:

Create `permissions.json`:

```json
[
    {
        "contract": "0xef4073a0f2b305a38ec4050e4d3d28bc40ea63f5",
        "methods": ["transfer", "balanceOf"]
    },
    {
        "contract": "0xd2a4cff31913016155e38e474a2c06d08be276cf",
        "methods": ["balanceOf"]
    }
]
```

Apply it:

```bash
# Merge with compiler-detected permissions
neo-solc contract.sol \
  --manifest-permissions permissions.json \
  -o build/contract

# Replace only wildcard entries
neo-solc contract.sol \
  --manifest-permissions permissions.json \
  --manifest-permissions-mode replace-wildcards \
  --deny-wildcard-contracts \
  --deny-wildcard-methods \
  -o build/contract
```

The `merge` mode (default) adds the file's permissions to the compiler-detected set. The `replace-wildcards` mode replaces wildcard entries with the explicit entries from the file while keeping non-wildcard entries intact.

See [Manifest Spec](/manifests/manifest-spec) for the full manifest structure and permission model.

## NatSpec Manifest Overrides

You can override selected manifest fields directly in your Solidity source using NatSpec custom tags:

```solidity
/**
 * @custom:neo.manifest.name "MyCustomName"
 * @custom:neo.manifest.supportedstandards ["NEP-17"]
 * @custom:neo.manifest.trusts ["0x1111111111111111111111111111111111111111"]
 * @custom:neo.manifest.extra.Repository "https://github.com/acme/project"
 * @custom:neo.manifest.extra.Build {"commit":"abc123","pipeline":"ci"}
 */
contract MyContract {
    // ...
}
```

Supported fields:

| Tag                  | Type                | Description                                                      |
| -------------------- | ------------------- | ---------------------------------------------------------------- |
| `name`               | string              | Override the contract name in the manifest                       |
| `groups`             | JSON array          | Set the `groups` field                                           |
| `features`           | JSON object         | Set the `features` field (must be `{}` for Neo N3 compatibility) |
| `supportedstandards` | JSON array          | Declare supported NEP standards                                  |
| `trusts`             | JSON array or `"*"` | Set trusted contract hashes                                      |
| `extra.<Key>`        | any JSON            | Add custom metadata under the `extra` field                      |

Both `@custom:neo.manifest.*` and `@custom:manifest.*` prefixes are accepted.

::: warning
For Neo N3 compatibility, `features` must remain an empty object (`{}`). Populated feature keys are rejected at deploy time.
:::

## Diagnostic Output

### Structured JSON Diagnostics

For CI/CD integration, emit diagnostics as JSON lines on stderr:

```bash
# JSON errors
neo-solc contract.sol --json-errors -o build/contract

# JSON warnings
neo-solc contract.sol --json-warnings -o build/contract

# Both
neo-solc contract.sol --json-errors --json-warnings -o build/contract
```

Warning codes include: `COMPILER_WARNING`, `NEF_SOURCE_TRUNCATED`, `MANIFEST_FULL_WILDCARD`, `MANIFEST_WILDCARD_CONTRACT`, `MANIFEST_WILDCARD_METHODS`, and validation codes like `DUPLICATE_SIGNATURE`, `INVALID_STORAGE_PARAM`.

Error codes include: `VALIDATION_ERROR`, `IR_GENERATION_ERROR`, `MANIFEST_GENERATION_ERROR`, `GENERIC_ERROR`, `IO_ERROR`.

These flags do not alter file outputs -- they only change how diagnostics are printed to stderr.

### Warning Suppression and Promotion

```bash
# Suppress warnings with code prefix W101
neo-solc contract.sol --Wno W101 -o build/contract

# Promote W101 warnings to errors
neo-solc contract.sol --Werror W101 -o build/contract

# Combine: suppress W102, promote W101
neo-solc contract.sol --Wno W102 --Werror W101 -o build/contract
```

## Real-World Examples

### Simple Token Contract

```bash
neo-solc examples/SimpleStorage.sol -I devpack -O2 -o build/SimpleStorage
```

### Production Build with Full Hardening

```bash
neo-solc contracts/Token.sol \
  -I devpack \
  -O3 \
  --callt \
  --deny-wildcard-contracts \
  --deny-wildcard-methods \
  --json-errors \
  --json-warnings \
  -o build/Token
```

### DeFi Contract with Permission Allowlist

```bash
neo-solc contracts/AMM.sol \
  -I devpack \
  -O3 \
  --callt \
  --manifest-permissions permissions.json \
  --manifest-permissions-mode replace-wildcards \
  --deny-wildcard-contracts \
  --deny-wildcard-methods \
  -o build/AMM
```

### CI Pipeline Compilation

```bash
# Strict compilation with JSON diagnostics for CI parsing
neo-solc contracts/MyContract.sol \
  -I devpack \
  -O2 \
  --deny-wildcard-contracts \
  --deny-wildcard-methods \
  --json-errors \
  --json-warnings \
  --Werror W101 \
  -o build/MyContract
```

### Inspect Assembly for Debugging

```bash
neo-solc contract.sol -I devpack -O0 -f assembly -o build/contract.asm
cat build/contract.asm
```

### Predict Deployed Contract Hash

```bash
neo-solc contract.sol \
  -I devpack \
  -O2 \
  --deployer 0x0123456789abcdef0123456789abcdef01234567 \
  -o build/contract
```

The `--deployer` flag computes the contract hash that Neo will assign based on the sender address, NEF checksum, and manifest name. This is useful for pre-computing contract addresses before deployment.

## Related Pages

- [Deploy Workflow](/workflows/deploy) -- Deploy compiled artifacts to Neo-Express and TestNet.
- [Test Workflow](/workflows/test) -- Validate compilation with the test suite.
- [Production Readiness](/workflows/production) -- Pre-deployment checklist.
- [Manifest Spec](/manifests/manifest-spec) -- Full manifest structure and permission model.
- [Quick Start](/getting-started/quickstart) -- Step-by-step first compilation.
