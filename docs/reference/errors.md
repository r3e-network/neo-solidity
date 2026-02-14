# Error Reference

Complete reference for all error codes, warning codes, and diagnostic output produced by the Neo Solidity compiler.

## Error and Warning System Overview

The compiler uses a structured diagnostic system with four severity levels:

| Severity    | Behavior                                                                                          |
| ----------- | ------------------------------------------------------------------------------------------------- |
| **error**   | Compilation fails. Must be fixed before artifacts can be generated.                               |
| **warning** | Compilation succeeds, but the issue should be reviewed. Can be promoted to error with `--Werror`. |
| **info**    | Informational note. Does not affect compilation.                                                  |
| **hint**    | Suggestion for improvement. Does not affect compilation.                                          |

Each diagnostic includes:

- A numeric error code (format `E<NNNN>`)
- A human-readable message
- Source location (file, line, column) when available
- Optional fix suggestions

## Error Code Format

Error codes follow the pattern `E<NNNN>` where the first digit indicates the category:

| Range   | Category                                            |
| ------- | --------------------------------------------------- |
| `E1xxx` | Parse errors -- syntax and tokenization             |
| `E2xxx` | Semantic errors -- types, scopes, validation        |
| `E25xx` | Type errors -- overflow, bounds, casts              |
| `E3xxx` | Codegen errors -- bytecode generation, NeoVM limits |
| `E4xxx` | I/O errors -- files, imports, permissions           |
| `E5xxx` | Security warnings -- reentrancy, unsafe patterns    |

## Parse Errors (E1xxx)

These errors occur during Stage 1 (Frontend) when the `solang-parser` processes Solidity source code. Parse errors block all subsequent compilation stages.

| Code    | Name              | Description               | Common Cause                             |
| ------- | ----------------- | ------------------------- | ---------------------------------------- |
| `E1001` | UnexpectedToken   | Unexpected token in input | Typo, missing operator, wrong syntax     |
| `E1002` | UnexpectedEof     | Unexpected end of file    | Unclosed brace, missing semicolon at EOF |
| `E1003` | InvalidSyntax     | Invalid syntax            | Unsupported Solidity syntax construct    |
| `E1004` | MissingSemicolon  | Missing semicolon         | Statement not terminated                 |
| `E1005` | MissingBrace      | Missing brace             | Unclosed `{` or extra `}`                |
| `E1006` | InvalidNumber     | Invalid number literal    | Malformed hex, overflow in literal       |
| `E1007` | InvalidString     | Invalid string literal    | Unclosed string, invalid escape sequence |
| `E1008` | InvalidIdentifier | Invalid identifier        | Reserved word used as identifier         |

::: tip
Parse errors must be fixed first. The compiler cannot proceed to semantic analysis or code generation until all parse errors are resolved.
:::

### Example: E1005 MissingBrace

```
error[E1005]: missing closing brace
  --> MyContract.sol:42:1
   |
42 | function transfer(address to, uint256 amount) public {
   |                                                       ^ expected matching '}'
```

**Fix:** Ensure every `{` has a corresponding `}`. Check for accidentally deleted lines or copy-paste errors.

## Semantic Errors (E2xxx)

These errors occur during Stage 3 (Semantic Analysis) when the compiler validates types, scopes, and contract structure.

| Code    | Name                  | Description                   | Common Cause                                                 |
| ------- | --------------------- | ----------------------------- | ------------------------------------------------------------ |
| `E2001` | UndefinedVariable     | Undefined variable            | Typo in variable name, missing declaration                   |
| `E2002` | TypeMismatch          | Type mismatch                 | Assigning incompatible types (e.g., `address` to `uint256`)  |
| `E2003` | DuplicateDefinition   | Duplicate definition          | Two functions/variables with the same name in the same scope |
| `E2004` | UndefinedFunction     | Undefined function            | Calling a function that does not exist                       |
| `E2005` | UndefinedType         | Undefined type                | Using a struct or contract name that is not defined          |
| `E2006` | InvalidAssignment     | Invalid assignment            | Assigning to a constant or read-only value                   |
| `E2007` | ImmutableModification | Cannot modify immutable value | Writing to an `immutable` or `constant` variable             |
| `E2008` | VisibilityError       | Visibility error              | Calling a `private` function from outside its contract       |
| `E2009` | InvalidOverride       | Invalid override              | Override without `virtual`/`override` keywords               |
| `E2010` | MissingReturn         | Missing return statement      | Non-void function path without `return`                      |
| `E2011` | UnreachableCode       | Unreachable code              | Code after `return`, `revert`, or `require(false)`           |
| `E2012` | UnusedVariable        | Unused variable               | Declared variable never read                                 |
| `E2013` | UnusedFunction        | Unused function               | Internal function never called                               |
| `E2014` | ShadowedVariable      | Variable shadows outer scope  | Local variable hides a state variable                        |
| `E2015` | InvalidModifier       | Invalid modifier              | Modifier used incorrectly or on wrong function type          |

### Example: E2002 TypeMismatch

```
error[E2002]: type mismatch: expected uint256, found address
  --> MyContract.sol:15:5
   |
15 |     uint256 x = msg.sender;
   |     ^^^^^^^^^^^^^^^^^^^^^^
   |
   = help: use explicit cast: uint256(uint160(msg.sender))
```

**Fix:** Correct the type usage. In Neo Solidity, addresses and integers are distinct types just as in standard Solidity. Use explicit casts when conversion is intentional.

### Example: E2003 DuplicateDefinition

```
error[E2003]: duplicate definition: function 'transfer' already defined
  --> MyContract.sol:25:5
```

**Fix:** Rename one of the conflicting definitions, or use function overloading with different parameter signatures.

## Type Errors (E25xx)

| Code    | Name             | Description               | Common Cause                                          |
| ------- | ---------------- | ------------------------- | ----------------------------------------------------- |
| `E2501` | IntegerOverflow  | Integer overflow          | Compile-time constant exceeds type range              |
| `E2502` | IntegerUnderflow | Integer underflow         | Compile-time constant below type minimum              |
| `E2503` | DivisionByZero   | Division by zero          | Compile-time division by zero constant                |
| `E2504` | ArrayOutOfBounds | Array index out of bounds | Compile-time index exceeds fixed array length         |
| `E2505` | InvalidCast      | Invalid type cast         | Incompatible explicit cast (e.g., `string` to `uint`) |
| `E2506` | NullReference    | Null reference            | Accessing member on potentially null reference        |

### Example: E2501 IntegerOverflow

```
error[E2501]: integer overflow: value 999999999999999999999 exceeds uint256 range
  --> MyContract.sol:8:20
```

**Fix:** Use a value within the valid range for the target type, or split the computation across multiple steps.

## Codegen Errors (E3xxx)

These errors occur during Stage 5 (IR Generation) or Stage 7 (Code Generation) when the compiler translates Solidity constructs to NeoVM bytecode.

| Code    | Name                | Description           | Common Cause                                                              |
| ------- | ------------------- | --------------------- | ------------------------------------------------------------------------- |
| `E3001` | UnsupportedFeature  | Unsupported feature   | Using EVM-only features (`delegatecall`, inline assembly, `selfdestruct`) |
| `E3002` | InvalidBytecode     | Invalid bytecode      | Internal compiler error during bytecode emission                          |
| `E3003` | StackOverflow       | Stack overflow        | Expression too deeply nested for NeoVM stack                              |
| `E3004` | GasLimitExceeded    | Gas limit exceeded    | Contract initialization exceeds gas budget                                |
| `E3005` | ContractTooLarge    | Contract too large    | Generated bytecode exceeds NEF size limit                                 |
| `E3006` | InvalidOpcode       | Invalid opcode        | Internal error: generated opcode not in NeoVM spec                        |
| `E3007` | BreakOutsideLoop    | Break outside loop    | `break` statement not inside a `for`/`while` loop                         |
| `E3008` | ContinueOutsideLoop | Continue outside loop | `continue` statement not inside a `for`/`while` loop                      |
| `E3009` | InvalidJumpOffset   | Invalid jump offset   | Internal error: jump target out of bytecode range                         |

### Example: E3001 UnsupportedFeature

```
error[E3001]: unsupported feature: delegatecall is not available on NeoVM
  --> MyContract.sol:30:9
   |
30 |         target.delegatecall(data);
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^
```

**Blocked features include:**

- `delegatecall` / `staticcall` (EVM-specific call semantics)
- Inline assembly (`assembly { ... }`)
- `selfdestruct`
- Bytecode introspection (`extcodesize`, `extcodecopy`)
- `CREATE2` opcode semantics
- `block.difficulty` / `block.prevrandao`
- `tx.gasprice`

**Fix:** Refactor to use Neo-native patterns. See [Solidity Feature Support](/solidity/feature-support) for the full compatibility matrix and [Parity and Limitations](/reference/parity-limitations) for details.

### Example: E3003 StackOverflow

```
error[E3003]: stack overflow: expression nesting depth exceeds NeoVM limit
  --> MyContract.sol:45:9
```

**Fix:** Break deeply nested expressions into intermediate variables. NeoVM has a maximum evaluation stack depth of 2048 items, but deeply nested expressions can exhaust this limit during evaluation.

### Example: E3005 ContractTooLarge

```
error[E3005]: contract too large: generated bytecode (524,800 bytes) exceeds NEF limit (524,288 bytes)
```

**Fix:** Reduce contract size by:

- Increasing the optimization level (`-O 3`)
- Splitting the contract into multiple smaller contracts
- Removing unused functions and dead code
- Using external library contracts for shared logic

::: warning
The NEF format has a maximum script size of 512 KB (524,288 bytes). Contracts approaching this limit should be refactored into smaller modules.
:::

## I/O Errors (E4xxx)

| Code    | Name             | Description              | Common Cause                                |
| ------- | ---------------- | ------------------------ | ------------------------------------------- |
| `E4001` | FileNotFound     | File not found           | Source file path does not exist             |
| `E4002` | PermissionDenied | Permission denied        | Cannot read source file or write output     |
| `E4003` | ImportNotFound   | Import not found         | Imported file not found in any include path |
| `E4004` | CircularImport   | Circular import detected | File A imports B which imports A            |

### Example: E4003 ImportNotFound

```
error[E4003]: import not found: '@neo/devpack/NeoToken.sol'
  --> MyContract.sol:3:1
   |
 3 | import "@neo/devpack/NeoToken.sol";
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
```

**Fix:** Add the correct include path:

```bash
neo-solc contract.sol -I devpack -I ./node_modules -o build/
```

**Checklist:**

- Verify the import path matches the actual file location
- Check that `-I` paths are relative to the working directory or absolute
- Ensure no typos in the import statement
- For devpack imports, ensure the `devpack/` directory is present in the project

### Example: E4004 CircularImport

```
error[E4004]: circular import detected: A.sol -> B.sol -> A.sol
```

**Fix:** Break the circular dependency by extracting shared types into a third file that both can import, or restructure the contract hierarchy.

## Security Warnings (E5xxx)

| Code    | Name                | Description                        | Recommendation                                    |
| ------- | ------------------- | ---------------------------------- | ------------------------------------------------- |
| `E5001` | ReentrancyRisk      | Potential reentrancy vulnerability | Use checks-effects-interactions pattern           |
| `E5002` | UncheckedCall       | Unchecked external call            | Check return value of external calls              |
| `E5003` | TxOriginUsage       | `tx.origin` used for authorization | Use `msg.sender` instead                          |
| `E5004` | UnsafeDelegate      | Unsafe delegatecall                | `delegatecall` is blocked on NeoVM; refactor      |
| `E5005` | IntegerOverflowRisk | Potential integer overflow         | Use SafeMath or Solidity 0.8.x checked arithmetic |

::: warning
Security warnings (E5xxx) should be treated as errors in production builds. Use `--Werror E5` to enforce this in your CI pipeline.
:::

### Example: E5001 ReentrancyRisk

```
warning[E5001]: potential reentrancy vulnerability in function 'withdraw'
  --> MyContract.sol:20:5
   |
20 |     payable(msg.sender).transfer(balance);
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = help: move state changes before external calls (checks-effects-interactions pattern)
```

**Fix:** Reorder operations so state changes happen before external calls:

```solidity
// Before (vulnerable)
function withdraw() public {
    uint256 balance = balances[msg.sender];
    payable(msg.sender).transfer(balance);  // external call first
    balances[msg.sender] = 0;               // state change after
}

// After (safe)
function withdraw() public {
    uint256 balance = balances[msg.sender];
    balances[msg.sender] = 0;               // state change first
    payable(msg.sender).transfer(balance);  // external call after
}
```

## Manifest Warning Codes

These are string-based warning codes emitted during manifest generation (Stage 8):

| Code                         | Description                  | Trigger                                            |
| ---------------------------- | ---------------------------- | -------------------------------------------------- |
| `COMPILER_WARNING`           | General compiler warning     | Various conditions                                 |
| `NEF_SOURCE_TRUNCATED`       | NEF source field truncated   | `--nef-source` value exceeds 240 bytes             |
| `MANIFEST_FULL_WILDCARD`     | Full wildcard permission     | Manifest requires `{"contract":"*","methods":"*"}` |
| `MANIFEST_WILDCARD_CONTRACT` | Wildcard contract permission | Manifest requires `{"contract":"*",...}`           |
| `MANIFEST_WILDCARD_METHODS`  | Wildcard method permission   | Manifest requires `{...,"methods":"*"}`            |

### Constructor Deploy Warning

When a contract has a parameterised constructor, the compiler emits a warning if the manifest does not include `StdLib.jsonDeserialize` and `StdLib.deserialize` permissions. The injected deploy prologue uses these methods to parse constructor arguments.

```
warning: contract 'MyToken' has a parameterised constructor; deploy it by passing
constructor args through `_deploy(data, update)`. Neo-Express: pass a JSON array
string via `-d '[7]'`; SDKs that support StackItems may pass an Array directly.
```

**Fix:** Ensure the manifest permissions include StdLib access, or pass constructor arguments as a StackItem Array directly from your SDK.

## Internal Error Types

The compiler uses several internal error categories that appear in JSON diagnostic output:

| Type                 | Source                           | Description                              |
| -------------------- | -------------------------------- | ---------------------------------------- |
| `CompilerError`      | Validation and semantic analysis | Standard compilation errors with E-codes |
| `IrGeneration`       | IR lowering (Stage 5)            | Errors during IR generation with context |
| `ManifestGeneration` | Artifact builder (Stage 8)       | Manifest permission policy violations    |
| `Generic`            | Various stages                   | Catch-all for unclassified errors        |

IR generation errors include additional context fields in JSON output:

```json
{
  "component": "neo-solidity",
  "severity": "error",
  "type": "IrGeneration",
  "message": "unsupported feature: inline assembly",
  "functionName": "computeHash",
  "errorCode": "E3001",
  "suggestion": "Use CryptoLib.sha256() from the devpack instead"
}
```

## Diagnostic Output Formats

### Human-Readable (default)

```
error[E2002]: type mismatch: expected uint256, found address
  --> MyContract.sol:15:5
   |
15 |     uint256 x = msg.sender;
   |     ^^^^^^^^^^^^^^^^^^^^^^
   |
   = help: use explicit cast: uint256(uint160(msg.sender))
```

### JSON Lines (`--json-errors`, `--json-warnings`)

Each diagnostic is a single JSON object on one line of stderr:

```json
{
  "component": "neo-solidity",
  "severity": "error",
  "type": "CompilerError",
  "code": "E2002",
  "message": "type mismatch: expected uint256, found address",
  "formattedMessage": "type mismatch: expected uint256, found address",
  "location": {
    "file": "MyContract.sol",
    "line": 15,
    "column": 5
  }
}
```

## Warning Suppression and Promotion

### Suppress warnings by code prefix

```bash
# Suppress all unused-variable warnings (E2012)
neo-solc contract.sol --Wno E2012 -I devpack -o build/

# Suppress all semantic warnings (E2xxx)
neo-solc contract.sol --Wno E2 -I devpack -o build/
```

### Promote warnings to errors

```bash
# Treat all security warnings as errors
neo-solc contract.sol --Werror E5 -I devpack -o build/

# Treat specific warning as error
neo-solc contract.sol --Werror E5001 -I devpack -o build/
```

### Combine suppression and promotion

```bash
# Errors on security, suppress unused variables
neo-solc contract.sol --Werror E5 --Wno E2012 -I devpack -o build/
```

## Debugging Workflow

A step-by-step approach to resolving compilation failures:

1. **Compile with full diagnostics:**

   ```bash
   neo-solc contract.sol -I devpack --json-errors --json-warnings -o build/ 2>diag.jsonl
   ```

2. **Fix highest-severity errors first.** Parse errors (E1xxx) block all later stages. Fix them before addressing semantic errors.

3. **Address semantic errors (E2xxx).** These are type and scope issues in your Solidity code.

4. **Review codegen errors (E3xxx).** These usually indicate unsupported EVM features that need refactoring for NeoVM.

5. **Run with manifest safety flags:**

   ```bash
   neo-solc contract.sol -I devpack \
     --deny-wildcard-permissions \
     --deny-wildcard-contracts \
     -o build/
   ```

6. **Validate with Neo-Express smoke tests** before deploying to testnet or mainnet.

::: tip
Use `-v` (verbose) to see optimization statistics and IR details. This can help diagnose unexpected bytecode size or behavior.
:::

## Platform-Specific Notes

### Linux

Ensure `libclang` is installed for the `solang-parser` dependency:

```bash
apt-get install libclang-dev
```

### macOS

Xcode command-line tools are required:

```bash
xcode-select --install
```

### Windows

Visual Studio Build Tools and the Rust MSVC target are required.

## See Also

- [CLI Reference](/reference/cli) -- all compiler options
- [Troubleshooting](/reference/troubleshooting) -- common issues and solutions
- [Architecture](/reference/architecture) -- compiler pipeline details
- [Runtime Specification](/reference/runtime) -- NeoVM execution model
- [Parity and Limitations](/reference/parity-limitations) -- known gaps
- [Solidity Feature Support](/solidity/feature-support) -- language compatibility
