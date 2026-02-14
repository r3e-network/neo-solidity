# CLI Reference

Primary binary: `neo-solc`

## Core syntax

```bash
neo-solc <source...> [options]
```

## Main options

| Option | Description |
| --- | --- |
| `-o, --output <FILE>` | Output prefix (normal mode) or JSON output file (standard-json mode) |
| `-O, --optimize <0-3>` | Optimization level (default `2`) |
| `-f, --format <FORMAT>` | `nef`, `manifest`, `complete`, `assembly`, `json` |
| `-I, --include-path <DIR>` | Additional import path (repeatable) |
| `--contract <NAME>` | Emit only selected contract name(s) |
| `--callt` | Emit CALLT + method tokens for native calls |
| `--nef-source <STRING>` | Override NEF source metadata |
| `--deployer <HASH160>` | Predict deployed contract hash for sender |

## Manifest safety options

| Option | Behavior |
| --- | --- |
| `--deny-wildcard-permissions` | Fail on full wildcard (`contract='*', methods='*'`) |
| `--deny-wildcard-contracts` | Fail on wildcard contract scope |
| `--deny-wildcard-methods` | Fail on wildcard methods |
| `--manifest-permissions <FILE>` | Merge/replace inferred permissions from JSON |
| `--manifest-permissions-mode <MODE>` | `merge` or `replace-wildcards` |

## Standard JSON mode

```bash
neo-solc --standard-json --input input.json --output output.json
```

## Machine-readable diagnostics

| Option | Description |
| --- | --- |
| `--json-errors` | Emit errors as JSON lines on stderr |
| `--json-warnings` | Emit warnings as JSON lines on stderr |
| `--Wno <CODE>` | Suppress warnings by code prefix |
| `--Werror <CODE>` | Promote warnings to errors by code prefix |

## Help/version

```bash
neo-solc --help
neo-solc --version
```
