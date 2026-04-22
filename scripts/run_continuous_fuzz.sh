#!/usr/bin/env bash
# Continuous fuzz runner for neo-solidity
# Runs proptest suite in a loop + cargo-fuzz targets in background.

set -euo pipefail
cd "$(dirname "$0")/.."

mkdir -p /tmp/fuzz-continuous

PROPTEST_CASES="${PROPTEST_CASES:-50}"
export PROPTEST_CASES

CARGO_FUZZ_TIME="${CARGO_FUZZ_TIME:-300}"

echo "Starting continuous fuzz loop..."
echo "  Proptest cases per run: $PROPTEST_CASES"
echo "  Cargo-fuzz time per target: ${CARGO_FUZZ_TIME}s"
echo "  Press Ctrl+C to stop"
echo

ROUND=0
while true; do
  ROUND=$((ROUND + 1))
  echo "========== ROUND $ROUND =========="

  # 1. Proptest suite
  echo "[$(date)] Running proptest suite..."
  if cargo test --test fuzz_tests > /tmp/fuzz-continuous/proptest-$ROUND.log 2>&1; then
    PASSED=$(grep -c "^test .* ok$" /tmp/fuzz-continuous/proptest-$ROUND.log || true)
    echo "[$(date)] ✅ Proptest passed ($PASSED tests)"
  else
    echo "[$(date)] ❌ Proptest FAILED (round $ROUND)"
    cat /tmp/fuzz-continuous/proptest-$ROUND.log
    exit 1
  fi

  # 2. Cargo-fuzz target 1 (compiler/parser) - short burst
  echo "[$(date)] Running cargo-fuzz target 1 (compiler/parser) for ${CARGO_FUZZ_TIME}s..."
  timeout "$CARGO_FUZZ_TIME" cargo +nightly fuzz run fuzz_target_1 > /tmp/fuzz-continuous/cargofuzz1-$ROUND.log 2>&1 || true
  if grep -q "ERROR:" /tmp/fuzz-continuous/cargofuzz1-$ROUND.log; then
    echo "[$(date)] ❌ cargo-fuzz target 1 found crashes!"
    cat /tmp/fuzz-continuous/cargofuzz1-$ROUND.log
    exit 1
  fi
  COV1=$(grep "^#.*cov:" /tmp/fuzz-continuous/cargofuzz1-$ROUND.log | tail -1 | awk '{for(i=1;i<=NF;i++) if($i=="cov:") print $(i+1)}' || true)
  echo "[$(date)] ✅ cargo-fuzz target 1 done (cov: ${COV1:-N/A})"

  # 3. Cargo-fuzz target 2 (NEF parser) - short burst
  echo "[$(date)] Running cargo-fuzz target 2 (NEF parser) for ${CARGO_FUZZ_TIME}s..."
  timeout "$CARGO_FUZZ_TIME" cargo +nightly fuzz run fuzz_target_2 > /tmp/fuzz-continuous/cargofuzz2-$ROUND.log 2>&1 || true
  if grep -q "ERROR:" /tmp/fuzz-continuous/cargofuzz2-$ROUND.log; then
    echo "[$(date)] ❌ cargo-fuzz target 2 found crashes!"
    cat /tmp/fuzz-continuous/cargofuzz2-$ROUND.log
    exit 1
  fi
  COV2=$(grep "^#.*cov:" /tmp/fuzz-continuous/cargofuzz2-$ROUND.log | tail -1 | awk '{for(i=1;i<=NF;i++) if($i=="cov:") print $(i+1)}' || true)
  echo "[$(date)] ✅ cargo-fuzz target 2 done (cov: ${COV2:-N/A})"

  echo "[$(date)] Round $ROUND complete. Sleeping 5s..."
  sleep 5
done
