# Testing Solidity on Neo with `neo-test`

`neo-test` is a Foundry-style test runner for Solidity contracts compiled to
NeoVM. You write test contracts in Solidity exactly like you would for Ethereum
(`function testFoo()`, `setUp()`, `assertEq`, `console.log`); `neo-test`
compiles each one with `neo-solc` and executes every test function on the
in-tree NeoVM — the same VM the compiler's own test suite trusts — reporting
PASS/FAIL, gas, decoded revert reasons, and log output.

## Quick start

```bash
cargo build --release --bin neo-test     # or: make build
```

Write a test file `test/Counter.t.sol`:

```solidity
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

import "neo-std/Test.sol";       // assertions (bundled, no setup)
import "neo-std/console.sol";    // console.log debug output (bundled)

contract Counter {
    uint256 public count;
    function inc() public { count += 1; }
    function dec() public { require(count > 0, "underflow"); count -= 1; }
}

contract CounterTest is Test {
    Counter c;

    function setUp() public {
        c = new Counter();          // runs fresh before EACH test
    }

    function testIncrements() public {
        c.inc();
        console.log("count", c.count());
        assertEq(c.count(), 1);
    }

    function testFailUnderflow() public {
        c.dec();                    // count is 0 -> reverts -> this test PASSES
    }
}
```

Run it:

```bash
neo-test test/Counter.t.sol --gas
# or just `neo-test` to scan ./test (then .) for *.t.sol
```

```
Running 2 test(s) for test/Counter.t.sol:CounterTest
  [PASS] testFailUnderflow (gas: 160)
  [PASS] testIncrements (gas: 4001431)
    log: count = 1

Test result: ok. 2 passed; 0 failed; finished in 2.84ms
```

## Conventions (Foundry-compatible)

| Convention      | Meaning |
|-----------------|---------|
| `function test*()`     | A test. **Passes when it does not revert.** |
| `function testFail*()` | **Passes when it reverts** (negative test). |
| `function setUp()`     | Runs before **each** test, against fresh state (per-test isolation). |
| `require` / `assert` / `revert` | The assertion mechanism — a revert fails the test and its reason is shown. |

Each test runs in a fresh VM instance, so storage written by one test never
leaks into another (exactly like Foundry). Constructor + state-variable
initializers + `setUp()` run before every test.

## The bundled `neo-std` library

`import "neo-std/Test.sol"` and `import "neo-std/console.sol"` resolve
automatically (no remapping or install needed):

- **`Test`** — inherit it for `assertTrue` / `assertFalse` / `assertEq` (uint,
  int, bool, address, bytes32, string, bytes) / `assertNotEq` / `assertGt` /
  `assertGe` / `assertLt` / `assertLe` / `fail`. Each takes an optional message:
  `assertEq(a, b, "balances diverged")`.
- **`console`** — `console.log("here")`, `console.log("balance", amount)`,
  and typed overloads for `uint256`/`int256`/`bool`/`address`/`bytes`. Output is
  printed under each test (always for a failing test, and with `-v` for all).
  Under the hood it emits `System.Runtime.Log`, so the same calls surface as
  application logs on a real node.

## Decoded failure reasons

`neo-test` decodes the on-chain revert payload, so failures read clearly:

- `revert("msg")` / `require(cond, "msg")` → `revert: msg`
- a Solidity `Panic` → `panic: 0x12 (division or modulo by zero)`, `0x11`
  (overflow/underflow), `0x32` (array out of bounds), etc.
- custom errors → the error name.

## Options

```
neo-test [PATH ...] [OPTIONS]

PATH                       A .sol file or a directory (scanned for *.t.sol, else *.sol).
-m, --match-test <S>       Only run test fns whose name contains <S>
-c, --match-contract <S>   Only run contracts whose name contains <S>
-I, --include <DIR>        Extra import root (repeatable; e.g. node_modules)
-v, --verbose              Show logs and revert reasons for every test
--gas                      Show per-test gas usage
--no-color                 Disable ANSI color
```

Imports are resolved from disk, so multi-file projects, relative imports, a
`remappings.txt`, and a co-located `node_modules` (`@openzeppelin/...`) all work
just as with `neo-solc`.

## Exit code

`neo-test` exits `0` when every test passes and `1` on any failure or
compile/import error — drop it straight into CI.

## Known limitations

The runner executes against the in-tree NeoVM simulator, which today has two
gaps worth knowing:

- **`msg.sender` across `new`-deployed contracts.** Inside the test contract,
  `msg.sender` and `address(this)` behave correctly, and cross-contract calls
  with *explicit* address arguments work (e.g. `token.mint(addr, 100)`). But a
  contract you `new` and then call does **not yet** observe the test contract as
  its `msg.sender` — so a callee that keys storage on `bal[msg.sender]` reads a
  different slot than `bal[address(testContract)]`. Workarounds: test the
  contract's logic directly (make the test contract *be* or inherit the contract
  under test), or pass addresses explicitly. (Caller-account propagation for
  `new`-deployed callees is a tracked follow-up.)
- **No cheatcodes yet** (`vm.prank`, `vm.expectRevert`, `vm.warp`, `deal`, …).
  Use `testFail*` for expected-revert tests and `setUp()` for fixtures.
