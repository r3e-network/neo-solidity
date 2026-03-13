# Troubleshooting

Common issues, error patterns, and solutions when working with the Neo Solidity compiler and devpack.

## Import Resolution Failures

### E4003: Unresolved Import

```
error[E4003]: unresolved import 'libraries/Storage.sol'
  --> MyContract.sol:3:1
   |
3  | import "libraries/Storage.sol";
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = help: pass the devpack root with -I: neo-solc MyContract.sol -I devpack
```

**Cause:** The compiler cannot find the imported file relative to any include path.

**Solutions:**

1. Pass the devpack root as an include path:

   ```bash
   neo-solc MyContract.sol -I devpack -o build/MyContract
   ```

2. Verify the import path matches the devpack directory structure:

   ```
   devpack/
   ├── contracts/    → import "contracts/NativeCalls.sol"
   ├── libraries/    → import "libraries/Storage.sol"
   └── standards/    → import "standards/NEP17.sol"
   ```

3. For multiple include paths, pass `-I` multiple times:
   ```bash
   neo-solc MyContract.sol -I devpack -I ./my-libs -o build/MyContract
   ```

::: tip
Import paths are resolved relative to each `-I` directory in order. The compiler does not use Solidity remappings or package managers.
:::

---

## Unsupported EVM Features

### E3001: UnsupportedFeature

```
error[E3001]: unsupported feature: delegatecall
  --> MyContract.sol:15:9
   |
15 |         address(impl).delegatecall(data);
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: NeoVM has no delegate call mechanism
   = help: use ContractManagement.update() for upgradeable contracts
```

**Blocked constructs and their Neo alternatives:**

| Blocked Feature        | Error | Neo Alternative                                 |
| ---------------------- | ----- | ----------------------------------------------- |
| `delegatecall`         | E3001 | `ContractManagement.update()` for upgrades      |
| `assembly { ... }`     | E3001 | Use devpack libraries instead                   |
| `new Contract(...)`    | E3001 | `ContractManagement.deploy(...)`                |
| `type(X).creationCode` | E3001 | Deploy via `ContractManagement.deploy(...)`     |
| `type(X).runtimeCode`  | E3001 | No Neo equivalent                               |

::: warning
These constructs have no safe 1:1 NeoVM equivalent. Refactor to the listed Neo-native patterns.
:::

---

## Manifest and Permission Issues

### Wildcard Permission Rejection

```
error: wildcard contract permission detected
  --> manifest generation
   |
   = note: contract call to dynamic target requires {"contract":"*","methods":"*"}
   = help: use --manifest-permissions to supply explicit overrides, or remove --deny-wildcard-contracts
```

**Cause:** Your code calls a contract through a dynamic address (not a compile-time constant), and you compiled with `--deny-wildcard-contracts`.

**Solutions:**

1. Replace dynamic calls with fixed-target wrappers:

   ```solidity
   // ❌ Dynamic target — forces wildcard permission
   Syscalls.contractCall(dynamicAddr, "transfer", ...);

   // ✅ Fixed target — generates specific permission
   NativeCalls.gasTransfer(from, to, amount, "");
   ```

2. If dynamic dispatch is unavoidable, supply explicit permissions:

   ```bash
   neo-solc MyContract.sol -I devpack \
     --manifest-permissions '{"contract":"0xabcd...","methods":["transfer"]}' \
     -o build/MyContract
   ```

3. Remove the strict flags for development (not recommended for production):
   ```bash
   neo-solc MyContract.sol -I devpack -o build/MyContract
   # Without --deny-wildcard-contracts, wildcards are allowed with a warning
   ```

### Standards Not Detected

```
info: contract does not implement any recognized NEP standard
  --> manifest generation
   |
   = note: supportedstandards will be empty
```

**Cause:** The compiler's auto-detection didn't find a complete set of required methods.

**Checklist for NEP-17 detection:**

- [ ] `symbol()` — public/external, returns string
- [ ] `decimals()` — public/external, returns integer
- [ ] `totalSupply()` — public/external, returns integer
- [ ] `balanceOf(address)` — public/external, returns integer
- [ ] `transfer(address, address, uint256, Any)` — public/external, 4 parameters
- [ ] `ownerOf` must NOT be present (its presence triggers NEP-11 instead)

**Checklist for NEP-11 detection:**

- [ ] `balanceOf(address)` AND `ownerOf(bytes32)` both present
- [ ] At least one of: `transfer`, `transferFrom`, `tokensOf`

::: tip
After compilation, verify detection results:

```bash
cat build/MyContract/MyContract.manifest.json | jq '.supportedstandards'
```

:::

---

## Constructor and Deployment Issues

### Constructor Parameter Mismatch

```
error[E2015]: constructor parameter type mismatch
  --> MyToken.sol:8:5
   |
8  |     NEP17("My Token", "MYT", 8, 100000000, 0)
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: expected (string, string, uint8, uint256, uint256)
```

**Cause:** The base contract constructor expects specific parameter types that don't match what you're passing.

**Solution:** Check the base contract's constructor signature in the devpack:

```solidity
// NEP17 constructor signature:
constructor(
    string memory name_,    // Token name
    string memory symbol_,  // Token symbol
    uint8 decimals_,        // Decimal places (max 18)
    uint256 initialSupply_, // Initial supply in smallest units
    uint256 maxSupply_      // Max supply (0 = unlimited)
)

// NEP11 constructor signature:
constructor(
    string memory name_,    // Collection name
    string memory symbol_,  // Collection symbol
    uint8 decimals_,        // 0 for indivisible, >0 for divisible
    string memory baseURI_, // Base URI for metadata
    uint256 maxSupply_,     // Max supply
    bool divisible_         // Whether tokens are divisible
)
```

### Deployment Fails on Neo-Express

```
error: deployment failed: insufficient GAS for deployment
```

**Solutions:**

1. Ensure your account has enough GAS:

   ```bash
   neo-express transfer GAS genesis alice 1000
   ```

2. Check the NEF file size — large contracts cost more to deploy:

   ```bash
   ls -la build/MyContract/MyContract.nef
   # If > 512KB, consider splitting into multiple contracts
   ```

3. Verify Neo-Express is running:
   ```bash
   neo-express run
   ```

---

## Gas and Execution Issues

### Insufficient GAS at Runtime

```
error: execution failed: insufficient GAS
```

**Cause:** The transaction's system fee doesn't cover the execution cost.

**Diagnosis:**

1. Check gas consumption with `gasleft()`:

   ```solidity
   function expensiveOperation() external {
       uint256 gasBefore = gasleft();
       // ... operation ...
       uint256 gasUsed = gasBefore - gasleft();
       Runtime.log(string(abi.encodePacked("Gas used: ", gasUsed)));
   }
   ```

2. Use the `withGasLimit` modifier from `FrameworkBase`:
   ```solidity
   function criticalOp() external withGasLimit(50000000) {
       // Reverts early if < 0.5 GAS remaining
   }
   ```

**Common gas-heavy operations:**

| Operation                      | Approximate Cost       |
| ------------------------------ | ---------------------- |
| `System.Storage.Put` (new key) | 200,000+ GAS units     |
| `System.Storage.Put` (update)  | 100,000+ GAS units     |
| `System.Contract.Call`         | 32,768 GAS units       |
| `CryptoLib.verifyWithECDsa`    | 1,000,000+ GAS units   |
| Large array iteration          | Scales with array size |

::: warning
Neo GAS uses 10^8 decimals (not 10^18 like Ethereum). `1 GAS = 100,000,000` fractional units. Adjust your constants accordingly.
:::

---

## ERC to NEP Migration Pitfalls

### Common ERC-20 → NEP-17 Mistakes

1. **Keeping 2-parameter transfer:**

   ```solidity
   // ❌ ERC-20 style — won't be detected as NEP-17
   function transfer(address to, uint256 amount) public returns (bool) { ... }

   // ✅ NEP-17 style — 4 parameters
   function transfer(address from, address to, uint256 amount, Any calldata data)
       public returns (bool) { ... }
   ```

2. **Using msg.sender for authorization:**

   ```solidity
   // ❌ EVM pattern
   require(msg.sender == from, "unauthorized");

   // ✅ Neo pattern — cryptographic witness verification
   require(Runtime.checkWitness(from), "unauthorized");
   ```

3. **Keeping approve/allowance:**

   ```solidity
   // ❌ Not part of NEP-17 spec — compiler warns W103
   function approve(address spender, uint256 amount) public { ... }

   // ✅ Remove — witness model replaces approvals entirely
   ```

4. **Missing onNEP17Payment callback:**

   ```solidity
   // ❌ EVM pattern
   receive() external payable { ... }

   // ✅ NEP-17 callback — required for contracts receiving tokens
   function onNEP17Payment(address from, uint256 amount, Any calldata data) external { ... }
   ```

### Common ERC-721 → NEP-11 Mistakes

1. **Using uint256 token IDs instead of bytes32**
2. **Keeping transferFrom instead of 3-param transfer**
3. **Missing required `tokensOf()` and `properties()` methods**
4. **Missing `decimals()` returning 0 for indivisible NFTs**

---

## Compiler Warning Reference

### Auto-Mapping Warnings

These warnings indicate EVM globals that are auto-mapped to approximate Neo equivalents:

| Warning            | Pattern                                    | Action                                               |
| ------------------ | ------------------------------------------ | ---------------------------------------------------- |
| `block.coinbase`   | Auto-mapped to `address(0)`                | Review — always zero on Neo (no block miner in dBFT) |
| `block.difficulty` | Auto-mapped to `Runtime.getRandom()`       | Review — different randomness model                  |
| `block.gaslimit`   | Auto-mapped to `Policy.getExecFeeFactor()` | Review — different gas accounting                    |
| `block.basefee`    | Auto-mapped to `Policy.getFeePerByte()`    | Review — different fee model                         |
| Ether literals     | Parsed but warned                          | Convert to Neo GAS units (10^8 decimals)             |

### Standards Migration Warnings

| Code | Pattern                              | Fix                                             |
| ---- | ------------------------------------ | ----------------------------------------------- |
| W101 | 2-param `transfer(to, amount)`       | Add `from` and `data` params for NEP-17         |
| W102 | 3-param `transfer(from, to, amount)` | Add `data` parameter for NEP-17                 |
| W103 | `approve`/`allowance`/`transferFrom` | Remove or keep as optional extensions           |
| W104 | `transferFrom(from, to, tokenId)`    | Replace with `transfer(to, tokenId, data)`      |
| W105 | `receive()` / `fallback()`           | Replace with `onNEP17Payment` callback          |
| W106 | `supportsInterface(bytes4)`          | Remove — manifest handles detection             |
| W107 | ERC-1155 multi-token pattern         | Split into separate NEP-17 and NEP-11 contracts |
| W108 | ERC-2612 permit pattern              | Use `Runtime.checkWitness()` instead            |

---

## Build and Toolchain Issues

### VitePress Build Failures (Documentation)

```bash
# Build the documentation site
npm run docs:build

# Common fix: dead links
# Check the error output for the specific file and link path
# Ensure all internal links use absolute VitePress paths: /section/page
```

### Cargo Build Failures (Compiler)

```bash
# Clean and rebuild
cargo clean && cargo build --release

# If solang-parser fails, check Rust version
rustc --version  # Requires 1.82+

# Run tests to verify
cargo test --workspace
```

### Neo-Express Issues

```bash
# Reset if state is corrupted
neo-express reset

# Create fresh network
neo-express create
neo-express run

# Check if running
neo-express show balances
```

---

## Getting Help

If you encounter an issue not covered here:

1. Check the [Error Reference](/reference/errors) for specific error codes
2. Check the [Parity and Limitations](/reference/parity-limitations) for known gaps
3. Review the [CLI Reference](/reference/cli) for compiler flags that may help
4. File an issue at [GitHub](https://github.com/r3e-network/neo-solidity/issues)

## See Also

- [Error Reference](/reference/errors) — Complete error code listing
- [CLI Reference](/reference/cli) — All compiler flags and options
- [Parity and Limitations](/reference/parity-limitations) — Known implementation gaps
- [Standards and Contracts](/devpack/standards) — NEP standard migration guides
- [EVM to NeoVM Mapping](/mapping/evm-to-neovm) — Semantic mapping reference
