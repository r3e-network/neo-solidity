# Quick Start

This guide walks you through writing, compiling, inspecting, and deploying a Solidity smart contract on Neo N3. By the end, you will have a working contract running on a local Neo-Express chain.

## Prerequisites

Before starting, ensure you have:

- `neo-solc` built and available (see [Installation](/basics/installing-the-compiler))
- `jq` installed for JSON inspection
- Neo-Express installed (optional, for local deployment)

All commands below assume you are in the `neo-solidity` repository root and have built the release binary.

## Step 1: Write a Contract

Create a file called `MyStorage.sol`:

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.34;

/// @title MyStorage - A simple storage contract for Neo N3
/// @notice Stores and retrieves a single integer value
contract MyStorage {
    uint256 private value;

    event ValueChanged(uint256 indexed oldValue, uint256 indexed newValue, address indexed setter);

    /// @notice Store a new value
    /// @param newValue The value to store
    function set(uint256 newValue) public {
        uint256 oldValue = value;
        value = newValue;
        emit ValueChanged(oldValue, newValue, msg.sender);
    }

    /// @notice Retrieve the stored value
    /// @return The current stored value
    function get() public view returns (uint256) {
        return value;
    }

    /// @notice Increment the stored value by one
    /// @return The new value after incrementing
    function increment() public returns (uint256) {
        value += 1;
        return value;
    }
}
```

This contract demonstrates the core patterns you will use in Neo Solidity:

- **State variables** (`value`) persist in Neo Storage.
- **Events** (`ValueChanged`) map to `Runtime.Notify` on Neo.
- **`msg.sender`** maps to `Runtime.GetCallingScriptHash()`.
- **`view` functions** are marked `safe` in the Neo manifest, meaning they can be invoked without a transaction.

## Step 2: Compile the Contract

```bash
./target/release/neo-solc MyStorage.sol -I devpack -O2 -o build/MyStorage
```

Flags explained:

| Flag                 | Purpose                                                                                    |
| -------------------- | ------------------------------------------------------------------------------------------ |
| `MyStorage.sol`      | Input Solidity source file                                                                 |
| `-I devpack`         | Add the `devpack/` directory as an import search path (provides Neo N3 library interfaces) |
| `-O2`                | Optimization level 2 (standard optimization: inlining, peephole, DCE, constant folding)    |
| `-o build/MyStorage` | Output prefix -- generates `build/MyStorage.nef` and `build/MyStorage.manifest.json`       |

Expected output (with `-v` for verbose):

```bash
./target/release/neo-solc MyStorage.sol -I devpack -O2 -v -o build/MyStorage
```

```
[info] Parsing MyStorage.sol
[info] Extracting metadata for contract MyStorage
[info] Building semantic model
[info] Generating IR (optimization level 2)
[info] Emitting NeoVM bytecode
[info] Writing build/MyStorage.nef (XXX bytes)
[info] Writing build/MyStorage.manifest.json
```

## Step 3: Inspect the Output Artifacts

### NEF File

The `.nef` file is a binary format. You can verify it was created and check its size:

```bash
ls -la build/MyStorage.nef
```

The NEF (Neo Executable Format) contains:

- **Magic bytes** (`0x3346454E`) -- identifies the file as a NEF
- **Compiler metadata** -- compiler name and version
- **Method tokens** -- references to native contract methods (when using `--callt`)
- **Script** -- the NeoVM bytecode
- **Checksum** -- SHA-256 hash for integrity verification

### Manifest File

The `.manifest.json` file is human-readable. Inspect it with `jq`:

```bash
jq '.' build/MyStorage.manifest.json
```

Example output (abbreviated):

```json
{
    "name": "MyStorage",
    "groups": [],
    "features": {},
    "supportedstandards": [],
    "abi": {
        "methods": [
            {
                "name": "set",
                "parameters": [{ "name": "newValue", "type": "Integer" }],
                "returntype": "Void",
                "offset": 0,
                "safe": false
            },
            {
                "name": "get",
                "parameters": [],
                "returntype": "Integer",
                "offset": 42,
                "safe": true
            },
            {
                "name": "increment",
                "parameters": [],
                "returntype": "Integer",
                "offset": 56,
                "safe": false
            }
        ],
        "events": [
            {
                "name": "ValueChanged",
                "parameters": [
                    { "name": "oldValue", "type": "Integer" },
                    { "name": "newValue", "type": "Integer" },
                    { "name": "setter", "type": "Hash160" }
                ]
            }
        ]
    },
    "permissions": [
        {
            "contract": "*",
            "methods": "*"
        }
    ],
    "trusts": [],
    "extra": null
}
```

Key fields to understand:

| Field                | Description                                                                                                       |
| -------------------- | ----------------------------------------------------------------------------------------------------------------- |
| `name`               | Contract name, derived from the Solidity contract name                                                            |
| `abi.methods`        | Each public/external function becomes a method entry. `safe: true` means the method is read-only (`view`/`pure`). |
| `abi.events`         | Each Solidity `event` becomes an event entry with typed parameters.                                               |
| `permissions`        | Declares which other contracts and methods this contract may call. Wildcards (`*`) mean unrestricted.             |
| `supportedstandards` | Lists NEP standards the contract implements (e.g., `["NEP-17"]`).                                                 |

::: tip Inspect Specific Fields
Quick commands to extract common fields:

```bash
# Contract name
jq '.name' build/MyStorage.manifest.json

# Number of methods
jq '.abi.methods | length' build/MyStorage.manifest.json

# Method names
jq '[.abi.methods[].name]' build/MyStorage.manifest.json

# Event names
jq '[.abi.events[].name]' build/MyStorage.manifest.json

# Permissions
jq '.permissions' build/MyStorage.manifest.json
```

:::

### NeoVM Assembly (Optional)

To see the generated NeoVM opcodes:

```bash
./target/release/neo-solc MyStorage.sol -I devpack -O2 -f assembly -o build/MyStorage.asm
cat build/MyStorage.asm
```

This produces a human-readable disassembly showing each NeoVM instruction.

## Step 4: Compile with Strict Manifest Policy

For production contracts, you should deny wildcard permissions. This ensures the manifest explicitly lists every contract and method your code calls:

```bash
./target/release/neo-solc MyStorage.sol \
  -I devpack \
  -O2 \
  --deny-wildcard-contracts \
  --deny-wildcard-methods \
  -o build/MyStorageStrict
```

If the contract requires permissions that cannot be narrowed (e.g., dynamic calls), the compiler will fail with an error. In that case, provide an explicit allowlist:

```bash
./target/release/neo-solc MyStorage.sol \
  -I devpack \
  -O2 \
  --manifest-permissions permissions.json \
  --manifest-permissions-mode replace-wildcards \
  --deny-wildcard-contracts \
  --deny-wildcard-methods \
  -o build/MyStorageStrict
```

See [Compile Workflow](/compiler/analysing-the-compiler-output) for full details on manifest hardening.

## Step 5: Deploy to Neo-Express (Local Chain)

Neo-Express provides a local Neo N3 blockchain for testing. This section walks through a manual deployment.

### 5a. Create a Local Chain

```bash
NEOXP=./build/dotnet-tools/neoxp

# Create a fresh single-node chain
$NEOXP create -f -o chain.neo-express

# Transfer GAS to the deployer account
$NEOXP transfer -i chain.neo-express 100 GAS genesis node1
```

### 5b. Deploy the Contract

```bash
$NEOXP contract deploy -i chain.neo-express build/MyStorage.nef node1
```

The output includes the contract hash and transaction hash. Note the contract hash -- you will need it to invoke methods.

### 5c. Invoke a Method (Write)

Create an invocation file `invoke-set.neo-invoke.json`:

```json
{
    "contract": "0x<your-contract-hash>",
    "operation": "set",
    "args": [42]
}
```

Invoke it:

```bash
$NEOXP contract invoke -i chain.neo-express invoke-set.neo-invoke.json node1
```

### 5d. Invoke a Method (Read)

Create `invoke-get.neo-invoke.json`:

```json
{
    "contract": "0x<your-contract-hash>",
    "operation": "get",
    "args": []
}
```

Invoke with `-r` for read-only (no transaction, no GAS cost):

```bash
$NEOXP contract invoke -r -j -i chain.neo-express invoke-get.neo-invoke.json node1
```

Expected output:

```json
{
    "state": "HALT",
    "stack": [
        {
            "type": "Integer",
            "value": "42"
        }
    ]
}
```

### 5e. Run the Automated Smoke Test

Instead of manual steps, you can run the repository's built-in smoke test:

```bash
make test-deploy-smoke
```

This script automatically:

1. Builds `neo-solc` if needed
2. Compiles a test contract
3. Creates a fresh Neo-Express chain
4. Deploys the contract
5. Invokes `set(7)`, verifies the `ValueSet` event
6. Invokes `get()`, verifies it returns `7`
7. Invokes `height()`, verifies it returns an integer (tests native contract calls)

## Step 6: Use the SimpleStorage Example

The repository ships a more complete example at `examples/SimpleStorage.sol` with key-value storage, ownership, reentrancy guards, and balance tracking.

Compile it:

```bash
./target/release/neo-solc examples/SimpleStorage.sol -I devpack -O2 -o build/SimpleStorage
```

Inspect the manifest:

```bash
# List all methods
jq '[.abi.methods[].name]' build/SimpleStorage.manifest.json
```

Expected methods:

```json
[
    "setNumber",
    "getNumber",
    "setString",
    "getString",
    "setKeyValue",
    "getKeyValue",
    "setKeyString",
    "getKeyString",
    "increment",
    "decrement",
    "add",
    "getOwner",
    "transferOwnership",
    "reset",
    "hasKey",
    "balanceOf",
    "setBalance"
]
```

## Step 7: Constructor Arguments

If your contract has a parameterized constructor, Neo handles it through the `_deploy(data, update)` entry point. The compiler automatically generates this entry point.

Example contract with a constructor:

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.34;

contract Counter {
    uint256 private count;

    constructor(uint256 initialCount) {
        count = initialCount;
    }

    function get() public view returns (uint256) {
        return count;
    }
}
```

Compile:

```bash
./target/release/neo-solc Counter.sol -I devpack -O2 -o build/Counter
```

Deploy with constructor arguments via Neo-Express:

```bash
$NEOXP contract deploy -i chain.neo-express -d '[100]' build/Counter.nef node1
```

The `-d '[100]'` flag passes a JSON array as the `data` parameter to `_deploy`. The compiler's deploy stub deserializes this array and passes the values to your Solidity constructor.

::: warning Manifest Permissions
Contracts with parameterized constructors require `StdLib.jsonDeserialize` and `StdLib.deserialize` permissions in the manifest. The compiler adds these automatically.
:::

## Next Steps

Now that you have compiled and deployed your first contract:

1. [Compile Workflow](/compiler/analysing-the-compiler-output) -- Full CLI reference with all options and patterns.
2. [Deploy Workflow](/basics/deploying-contracts) -- Detailed deployment guide for Neo-Express, TestNet, and MainNet.
3. [Test Workflow](/basics/testing-contracts) -- Run the full test suite and set up CI.
4. [Production Readiness](/advisory-content/production-readiness) -- Pre-deployment checklist and manifest hardening.
5. [Solidity Feature Support](/solidity/feature-support) -- What Solidity features work on NeoVM.
6. [Language Description](/language-description/types) -- How EVM concepts translate to Neo N3.
