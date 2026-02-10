#!/usr/bin/env bash
# Strict compatibility sweep for Neo Solidity contracts.
#
# Compiles curated Solidity sets with strict manifest denial flags and fails on:
# - any compilation error
# - missing .nef/.manifest.json output
# - compatibility/warning diagnostics in compiler output

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"

resolve_neo_solc_once() {
  if [ -n "${NEO_SOLC:-}" ]; then
    return
  fi

  if command -v neo-solc >/dev/null 2>&1; then
    export NEO_SOLC="neo-solc"
    return
  fi

  echo "(info) Building neo-solc once for strict compatibility sweep..."
  (cd "$ROOT_DIR" && cargo build --bin neo-solc >/dev/null)
  export NEO_SOLC="$ROOT_DIR/target/debug/neo-solc"
}

resolve_neo_solc_once

echo "(info) Using compiler: $NEO_SOLC"

WORK_DIR="$(mktemp -d "${TMPDIR:-/tmp}/neo-solidity-strict-sweep.XXXXXX")"
trap 'rm -rf "$WORK_DIR"' EXIT

STRICT_FLAGS=(
  --deny-wildcard-permissions
  --deny-wildcard-contracts
  --deny-wildcard-methods
)

declare -a FILES=()
while IFS= read -r -d '' f; do FILES+=("$f"); done < <(find "$ROOT_DIR/devpack/contracts" -maxdepth 1 -type f -name '*.sol' -print0 | sort -z)
while IFS= read -r -d '' f; do FILES+=("$f"); done < <(find "$ROOT_DIR/devpack/standards" -maxdepth 1 -type f -name '*.sol' -print0 | sort -z)
while IFS= read -r -d '' f; do FILES+=("$f"); done < <(find "$ROOT_DIR/devpack/examples" -maxdepth 1 -type f -name '*.sol' -print0 | sort -z)
while IFS= read -r -d '' f; do FILES+=("$f"); done < <(find "$ROOT_DIR/examples/new" -maxdepth 1 -type f -name '*.sol' -print0 | sort -z)

if [ "${#FILES[@]}" -eq 0 ]; then
  echo "error: no Solidity files found for strict compatibility sweep"
  exit 1
fi

failures=0
warnings=0

for file in "${FILES[@]}"; do
  rel="${file#"$ROOT_DIR"/}"
  stem="${rel//\//__}"
  stem="${stem%.sol}"
  prefix="$WORK_DIR/$stem"
  out="$WORK_DIR/$stem.out"
  err="$WORK_DIR/$stem.err"

  if ! "$NEO_SOLC" "$file" "${STRICT_FLAGS[@]}" -o "$prefix" >"$out" 2>"$err"; then
    echo "❌ strict compile failed: $rel"
    sed -n '1,80p' "$out"
    sed -n '1,80p' "$err"
    failures=$((failures + 1))
    continue
  fi

  shopt -s nullglob
  nef_files=("${prefix}"*.nef)
  manifest_files=("${prefix}"*.manifest.json)
  shopt -u nullglob

  if [ "${#nef_files[@]}" -eq 0 ] || [ "${#manifest_files[@]}" -eq 0 ]; then
    echo "❌ missing output artifacts: $rel"
    failures=$((failures + 1))
    continue
  fi

  if grep -Eiq '(^|[^a-z])(warning|compat)([^a-z]|$)' "$out" "$err"; then
    echo "⚠️  strict compile produced diagnostics: $rel"
    sed -n '1,80p' "$out" | rg -n "warning|compat" || true
    sed -n '1,80p' "$err" | rg -n "warning|compat" || true
    warnings=$((warnings + 1))
  fi
done

echo "strict_sweep_total=${#FILES[@]}"
echo "strict_sweep_failures=$failures"
echo "strict_sweep_warning_contracts=$warnings"

if [ "$failures" -ne 0 ] || [ "$warnings" -ne 0 ]; then
  exit 1
fi

echo "✅ strict compatibility sweep passed"
