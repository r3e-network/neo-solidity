#!/usr/bin/env bash
# Neo-Express smoke test: Solidity `delete` semantics.
#
# Verifies that neo-solc can:
# - delete mapping entries of fixed bytes + address values
# - delete mapping entries whose values are structs
# - read missing storage entries as Solidity defaults (address(0), bytes32(0), etc.)

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"

WORK_DIR="$(mktemp -d "${TMPDIR:-/tmp}/neo-solidity-neoxp.XXXXXX")"

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

cat > DeleteSmoke.sol <<'SOL'
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

contract DeleteSmoke {
    mapping(bytes32 => bytes32) private mBytes;
    mapping(bytes32 => address) private mAddr;

    struct S {
        address a;
        uint256 b;
        bool c;
        bytes32 d;
    }

    mapping(bytes32 => S) private mStruct;

    function run() public returns (uint256) {
        bytes32 k1 = bytes32(uint256(1));
        bytes32 k2 = bytes32(uint256(2));

        mBytes[k1] = bytes32(uint256(9));
        mAddr[k1] = msg.sender;

        S storage s = mStruct[k1];
        s.a = msg.sender;
        s.b = 7;
        s.c = true;
        s.d = bytes32(uint256(11));

        delete mBytes[k1];
        delete mAddr[k1];
        delete mStruct[k1];

        uint256 ok = 0;

        if (mBytes[k1] == bytes32(0)) ok += 1;
        if (mAddr[k1] == address(0)) ok += 2;
        S memory afterDelete = mStruct[k1];
        if (afterDelete.a == address(0)) ok += 4;
        if (afterDelete.b == 0) ok += 8;
        if (afterDelete.c == false) ok += 16;
        if (afterDelete.d == bytes32(0)) ok += 32;

        if (mBytes[k2] == bytes32(0)) ok += 64;
        if (mAddr[k2] == address(0)) ok += 128;
        S memory missing = mStruct[k2];
        if (missing.a == address(0)) ok += 256;
        if (missing.b == 0) ok += 512;
        if (missing.c == false) ok += 1024;
        if (missing.d == bytes32(0)) ok += 2048;

        return ok;
    }
}
SOL

"$NEO_SOLC_BIN" "$WORK_DIR/DeleteSmoke.sol" -o DeleteSmoke >/dev/null

CHAIN="$WORK_DIR/chain.neo-express"
run_neoxp create -f -o "$CHAIN" >/dev/null

# Ensure deployer has enough GAS for deploy + invoke.
run_neoxp transfer -i "$CHAIN" 100 GAS genesis node1 >/dev/null

CONTRACT_HASH="$(run_neoxp contract hash -i "$CHAIN" DeleteSmoke.nef node1 | tr -d '\r')"
if [ -z "$CONTRACT_HASH" ]; then
  echo "error: failed to compute contract hash"
  exit 1
fi
echo "(info) Contract hash: $CONTRACT_HASH"

run_neoxp contract deploy -i "$CHAIN" DeleteSmoke.nef node1 -j >/dev/null

cat > invoke-run.neo-invoke.json <<JSON
{
  "contract": "$CONTRACT_HASH",
  "operation": "run",
  "args": []
}
JSON

RUN_TX_HASH="$(run_neoxp contract invoke -i "$CHAIN" invoke-run.neo-invoke.json node1 | grep -oE '0x[0-9a-fA-F]{64}' | head -n 1)"
if [ -z "$RUN_TX_HASH" ]; then
  echo "error: failed to capture run() invocation tx hash"
  exit 1
fi

RUN_LOG="$(run_neoxp show transaction -i "$CHAIN" "$RUN_TX_HASH")"
RUN_VMSTATE="$(echo "$RUN_LOG" | jq -r '.["application-log"].executions[0].vmstate')"
if [ "$RUN_VMSTATE" != "HALT" ]; then
  echo "error: run() vmstate=$RUN_VMSTATE"
  echo "$RUN_LOG" | jq '.["application-log"].executions[0]'
  exit 1
fi

if [ "$(echo "$RUN_LOG" | jq -r '.["application-log"].executions[0].stack[0].type')" != "Integer" ]; then
  echo "error: run() returned non-Integer"
  echo "$RUN_LOG" | jq '.["application-log"].executions[0].stack'
  exit 1
fi
if [ "$(echo "$RUN_LOG" | jq -r '.["application-log"].executions[0].stack[0].value')" != "4095" ]; then
  echo "error: run() returned unexpected value"
  echo "$RUN_LOG" | jq '.["application-log"].executions[0].stack'
  exit 1
fi

echo "✅ neoxp delete smoke test passed"

