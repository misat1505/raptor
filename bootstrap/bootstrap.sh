#!/usr/bin/env bash
# Builds the self-hosted Raptor compiler from a checked-in LLVM IR file.
set -xe

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT="$(dirname "$DIR")"

mkdir -p "$ROOT/build"

LL="${1:-$DIR/raptor.ll}"
OUT="${2:-$DIR/raptor}"
V="${LLVM_VERSION:-18}"

tool() { command -v "$1-$V" || command -v "$1" || { echo "error: $1-$V not found" >&2; exit 1; }; }
LLC="$(tool llc)"
CLANG="$(tool clang)"

[[ -f "$LL" ]] || { echo "error: IR file not found: $LL" >&2; exit 1; }

mkdir -p "$(dirname "$OUT")"
"$LLC" "$LL" -filetype=obj -o "$OUT.o"
"$CLANG" "$OUT.o" -o "$OUT" -no-pie -lm
rm -f "$OUT.o"

echo "Built: $OUT"
