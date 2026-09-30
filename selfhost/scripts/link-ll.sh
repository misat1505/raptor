#!/usr/bin/env bash
# Compile Raptor-emitted LLVM IR (.ll) -> object -> executable.
# Usage: ./selfhost/scripts/link-ll.sh [path/to/file.ll] [output_exe]
set -euo pipefail

LL="${1:-./build/out.ll}"
OUT="${2:-./build/raptor_out}"
OBJ="${OUT}.o"

if [[ ! -f "$LL" ]]; then
  echo "error: IR file not found: $LL" >&2
  exit 1
fi

mkdir -p "$(dirname "$OUT")"

echo "[1/2] llc  $LL -> $OBJ"
llc -filetype=obj -o "$OBJ" "$LL"

echo "[2/2] clang $OBJ -> $OUT"
clang -O2 -o "$OUT" "$OBJ" -lm

echo "OK: executable $OUT"
echo "Run: $OUT [args...]"
