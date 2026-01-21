#!/usr/bin/env bash
# End-to-end abi.encode/abi.decode ordering smoke test using bundled Neo-Express (neoxp).
#
# Verifies that neo-solc preserves argument ordering across:
#   abi.encode(...) -> StdLib.serialize
#   abi.decode(...) -> StdLib.deserialize
#
# This catches a common NeoVM pitfall: PACK pops values from the stack and reverses order.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"

WORK_DIR="$(mktemp -d "${TMPDIR:-/tmp}/neo-solidity-neoxp-encode.XXXXXX")"

cleanup() {
  rm -rf "$WORK_DIR"
}
trap cleanup EXIT

resolve_neo_solc() {
  if [ -n "${NEO_SOLC:-}" ]; then
    echo "$NEO_SOLC"
    return
  fi

  if command -v neo-solc >/dev/null 2>&1; then
    echo "neo-solc"
    return
  fi

  echo "(info) Building neo-solc..." >&2
  (cd "$ROOT_DIR" && cargo build --bin neo-solc >/dev/null)
  echo "$ROOT_DIR/target/debug/neo-solc"
}

resolve_neoxp() {
  if [ -n "${NEOXP:-}" ]; then
    echo "$NEOXP"
    return
  fi

  if [ -x "$ROOT_DIR/build/dotnet-tools/neoxp" ]; then
    echo "$ROOT_DIR/build/dotnet-tools/neoxp"
    return
  fi

  if command -v neoxp >/dev/null 2>&1; then
    echo "neoxp"
    return
  fi

  if command -v dotnet >/dev/null 2>&1; then
    echo "(info) Installing Neo-Express (neoxp)..." >&2
    mkdir -p "$ROOT_DIR/build/dotnet-tools"
    dotnet tool install Neo.Express --tool-path "$ROOT_DIR/build/dotnet-tools" >/dev/null
    if [ -x "$ROOT_DIR/build/dotnet-tools/neoxp" ]; then
      echo "$ROOT_DIR/build/dotnet-tools/neoxp"
      return
    fi
  fi

  echo "error: neoxp not found (expected $ROOT_DIR/build/dotnet-tools/neoxp or on PATH)" >&2
  exit 1
}

if ! command -v jq >/dev/null 2>&1; then
  echo "error: jq is required for this smoke test" >&2
  exit 1
fi

NEO_SOLC_BIN="$(resolve_neo_solc)"
NEOXP_BIN="$(resolve_neoxp)"
NEOXP_HOME="$WORK_DIR/neoxp-home"
mkdir -p "$NEOXP_HOME"

run_neoxp() {
  HOME="$NEOXP_HOME" "$NEOXP_BIN" "$@"
}

echo "(info) Work dir: $WORK_DIR"
echo "(info) Compiler: $NEO_SOLC_BIN"
echo "(info) Neo-Express: $NEOXP_BIN"

cd "$WORK_DIR"

cat > EncodingSmoke.sol <<'SOL'
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

contract EncodingSmoke {
    function check() public pure returns (uint256) {
        bytes memory data = abi.encode(uint256(1), uint256(2));
        // This toolchain maps abi.decode -> StdLib.deserialize and ignores the type tuple.
        // The serialized payload should round-trip into a NeoVM Array [1,2].
        uint256[] memory values = abi.decode(data, (uint256[]));
        if (values.length != 2) return 0;
        return values[0] * 10 + values[1];
    }

    function hasSelf() public view returns (bool) {
        address selfHash = Syscalls.getExecutingScriptHash();
        // ContractManagement.hasMethod(hash, method, paramCount)
        return NativeCalls.hasMethod(selfHash, "hasSelf", 0);
    }
}
SOL

"$NEO_SOLC_BIN" "$WORK_DIR/EncodingSmoke.sol" -o EncodingSmoke >/dev/null

CHAIN="$WORK_DIR/chain.neo-express"
run_neoxp create -f -o "$CHAIN" >/dev/null

# Ensure deployer has enough GAS for deploy + invoke.
run_neoxp transfer -i "$CHAIN" 100 GAS genesis node1 >/dev/null

CONTRACT_HASH="$(run_neoxp contract hash -i "$CHAIN" EncodingSmoke.nef node1 | tr -d '\r')"
if [ -z "$CONTRACT_HASH" ]; then
  echo "error: failed to compute contract hash"
  exit 1
fi
echo "(info) Contract hash: $CONTRACT_HASH"

run_neoxp contract deploy -i "$CHAIN" EncodingSmoke.nef node1 -j >/dev/null

cat > invoke-check.neo-invoke.json <<JSON
{
  "contract": "$CONTRACT_HASH",
  "operation": "check",
  "args": []
}
JSON

OUT="$(run_neoxp contract invoke -r -j -i "$CHAIN" invoke-check.neo-invoke.json node1)"
if [ "$(echo "$OUT" | jq -r '.state')" != "HALT" ]; then
  echo "error: check() did not HALT"
  echo "$OUT"
  exit 1
fi
if [ "$(echo "$OUT" | jq -r '.stack[0].type')" != "Integer" ]; then
  echo "error: check() returned non-Integer"
  echo "$OUT"
  exit 1
fi
if [ "$(echo "$OUT" | jq -r '.stack[0].value')" != "12" ]; then
  echo "error: abi.encode/abi.decode ordering is wrong (expected 12)"
  echo "$OUT"
  exit 1
fi

echo "✅ neoxp abi.encode/abi.decode smoke test passed"

cat > invoke-hasSelf.neo-invoke.json <<JSON
{
  "contract": "$CONTRACT_HASH",
  "operation": "hasSelf",
  "args": []
}
JSON

HAS_OUT="$(run_neoxp contract invoke -r -j -i "$CHAIN" invoke-hasSelf.neo-invoke.json node1)"
if [ "$(echo "$HAS_OUT" | jq -r '.state')" != "HALT" ]; then
  echo "error: hasSelf() did not HALT"
  echo "$HAS_OUT"
  exit 1
fi
if [ "$(echo "$HAS_OUT" | jq -r '.stack[0].type')" != "Boolean" ]; then
  echo "error: hasSelf() returned non-Boolean"
  echo "$HAS_OUT"
  exit 1
fi
if [ "$(echo "$HAS_OUT" | jq -r '.stack[0].value')" != "true" ]; then
  echo "error: expected hasSelf() to return true"
  echo "$HAS_OUT"
  exit 1
fi

echo "✅ neoxp native call argument-order test passed"
