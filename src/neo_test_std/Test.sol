// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "neo-std/Vm.sol";

/// @title neo-test standard test base (forge-std-style)
/// @notice Inherit `Test` in your `*.t.sol` contracts to get assertions and the
/// cheatcode handle `vm` (from `CheatVm`). A test PASSES when it does not
/// revert; an assertion that fails calls `revert(reason)`, which `neo-test`
/// reports as `[FAIL] name (revert: reason)`.
///
///   import "neo-std/Test.sol";
///   contract MyTest is Test {
///       function setUp() public { /* ... */ }
///       function testThing() public { vm.warp(1000); assertEq(1 + 1, 2); }
///   }
abstract contract Test is CheatVm {
    // ---- boolean ----
    function assertTrue(bool condition) internal pure {
        if (!condition) revert("assertTrue: expected true");
    }
    function assertTrue(bool condition, string memory err) internal pure {
        if (!condition) revert(err);
    }
    function assertFalse(bool condition) internal pure {
        if (condition) revert("assertFalse: expected false");
    }
    function assertFalse(bool condition, string memory err) internal pure {
        if (condition) revert(err);
    }

    // ---- equality ----
    function assertEq(uint256 a, uint256 b) internal pure {
        if (a != b) revert("assertEq(uint): a != b");
    }
    function assertEq(uint256 a, uint256 b, string memory err) internal pure {
        if (a != b) revert(err);
    }
    function assertEq(int256 a, int256 b) internal pure {
        if (a != b) revert("assertEq(int): a != b");
    }
    function assertEq(bool a, bool b) internal pure {
        if (a != b) revert("assertEq(bool): a != b");
    }
    function assertEq(address a, address b) internal pure {
        if (a != b) revert("assertEq(address): a != b");
    }
    function assertEq(bytes32 a, bytes32 b) internal pure {
        if (a != b) revert("assertEq(bytes32): a != b");
    }
    function assertEq(string memory a, string memory b) internal pure {
        if (keccak256(bytes(a)) != keccak256(bytes(b))) revert("assertEq(string): a != b");
    }
    function assertEq(bytes memory a, bytes memory b) internal pure {
        if (keccak256(a) != keccak256(b)) revert("assertEq(bytes): a != b");
    }

    // ---- inequality ----
    function assertNotEq(uint256 a, uint256 b) internal pure {
        if (a == b) revert("assertNotEq(uint): a == b");
    }
    function assertNotEq(address a, address b) internal pure {
        if (a == b) revert("assertNotEq(address): a == b");
    }
    function assertNotEq(bytes32 a, bytes32 b) internal pure {
        if (a == b) revert("assertNotEq(bytes32): a == b");
    }

    // ---- ordering (unsigned) ----
    function assertGt(uint256 a, uint256 b) internal pure {
        if (!(a > b)) revert("assertGt: a <= b");
    }
    function assertGe(uint256 a, uint256 b) internal pure {
        if (!(a >= b)) revert("assertGe: a < b");
    }
    function assertLt(uint256 a, uint256 b) internal pure {
        if (!(a < b)) revert("assertLt: a >= b");
    }
    function assertLe(uint256 a, uint256 b) internal pure {
        if (!(a <= b)) revert("assertLe: a > b");
    }

    // ---- explicit failure ----
    function fail() internal pure {
        revert("fail()");
    }
    function fail(string memory err) internal pure {
        revert(err);
    }
}
