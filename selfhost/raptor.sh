#!/usr/bin/env bash
# ============================================================
# Mirrors src/main.rs build_executable + optional --run
#   Write IR (already done by selfhost) → opt → llc → clang → run
#
# Usage:
#   ./selfhost/scripts/raptor.sh [options] [file.ll]
#
# Options:
#   -o <path>     Output executable (default: ./build/raptor_out)
#   -O0|-O1|-O2|-O3   Optimization level for `opt` (default: none, like rust without -O*)
#   --run         Run the executable after linking
#   --link <obj>  Extra object/library to pass to clang (repeatable)
#   -v            Verbose
#   -h|--help     Help
#
# Env:
#   LLVM_VERSION  Suffix for tools (default: 18) → llc-18, clang-18, opt-18
#                 Falls back to unversioned llc/clang/opt if versioned missing.
# ============================================================
set -euo pipefail

LLVM_VERSION="${LLVM_VERSION:-18}"

LL="./build/out.ll"
OUT="./build/raptor_out"
OPT_LEVEL=""          # empty = skip opt pass (rust default without -O*)
SHOULD_RUN=0
VERBOSE=0
LINK_OBJS=()

usage() {
  sed -n '2,25p' "$0" | sed 's/^# \?//'
}

log() {
  if [[ "$VERBOSE" -eq 1 ]]; then
    printf '\033[36m[raptor]\033[0m %s\n' "$*"
  fi
}

die() {
  printf 'error: %s\n' "$*" >&2
  exit 1
}

# Resolve tool: prefer name-VERSION, else plain name
find_tool() {
  local base="$1"
  if command -v "${base}-${LLVM_VERSION}" >/dev/null 2>&1; then
    echo "${base}-${LLVM_VERSION}"
  elif command -v "${base}" >/dev/null 2>&1; then
    echo "${base}"
  else
    die "tool '${base}-${LLVM_VERSION}' (or '${base}') not found on PATH"
  fi
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    -h|--help)
      usage
      exit 0
      ;;
    -v|--verbose)
      VERBOSE=1
      shift
      ;;
    -o)
      [[ $# -ge 2 ]] || die "-o requires a path"
      OUT="$2"
      shift 2
      ;;
    -O0) OPT_LEVEL="O0"; shift ;;
    -O1) OPT_LEVEL="O1"; shift ;;
    -O2) OPT_LEVEL="O2"; shift ;;
    -O3) OPT_LEVEL="O3"; shift ;;
    --run)
      SHOULD_RUN=1
      shift
      ;;
    --link)
      [[ $# -ge 2 ]] || die "--link requires a file"
      LINK_OBJS+=("$2")
      shift 2
      ;;
    -*)
      die "unknown option: $1"
      ;;
    *)
      LL="$1"
      shift
      ;;
  esac
done

[[ -f "$LL" ]] || die "IR file not found: $LL"

OUT_DIR="$(dirname "$OUT")"
mkdir -p "$OUT_DIR"

OBJ="${OUT}.o"
# Intermediate optimized IR (only if -O* given)
OPT_LL="${OUT}.opt.ll"

LLC="$(find_tool llc)"
CLANG="$(find_tool clang)"
OPT_BIN=""
if [[ -n "$OPT_LEVEL" ]]; then
  OPT_BIN="$(find_tool opt)"
fi

log "llc=$LLC  clang=$CLANG  opt=${OPT_BIN:-<skip>}"
log "input=$LL  out=$OUT"

IR_FOR_LLC="$LL"

# --- opt (mirrors Compiler::optimize + -O0..-O3) ---
if [[ -n "$OPT_LEVEL" ]]; then
  log "opt -$OPT_LEVEL  $LL -> $OPT_LL"
  # Same idea as inkwell run_passes("default<O*>")
  case "$OPT_LEVEL" in
    O0) PASSES="default<O0>" ;;
    O1) PASSES="default<O1>" ;;
    O2) PASSES="default<O2>" ;;
    O3) PASSES="default<O3>" ;;
  esac
  if ! "$OPT_BIN" -passes="$PASSES" -S "$LL" -o "$OPT_LL" 2>/tmp/raptor-opt.err; then
    # Older opt CLI fallback
    if ! "$OPT_BIN" -"$OPT_LEVEL" -S "$LL" -o "$OPT_LL" 2>/tmp/raptor-opt.err; then
      cat /tmp/raptor-opt.err >&2
      die "opt failed"
    fi
  fi
  IR_FOR_LLC="$OPT_LL"
  log "optimized IR: $OPT_LL"
fi

# --- llc: .ll -> .o  (mirrors build_object_file) ---
log "llc -filetype=obj  $IR_FOR_LLC -> $OBJ"
if ! "$LLC" "$IR_FOR_LLC" -filetype=obj -o "$OBJ"; then
  die "llc failed"
fi

# --- clang link  (mirrors link_executable: -no-pie + extra objs) ---
CLANG_ARGS=("$OBJ" -o "$OUT" -no-pie)
if [[ ${#LINK_OBJS[@]} -gt 0 ]]; then
  CLANG_ARGS+=("${LINK_OBJS[@]}")
fi
# math lib often needed
CLANG_ARGS+=(-lm)

log "clang ${CLANG_ARGS[*]}"
if ! "$CLANG" "${CLANG_ARGS[@]}"; then
  die "clang link failed"
fi

printf 'Built executable: %s\n' "$OUT"

# --- run (mirrors --run) ---
if [[ "$SHOULD_RUN" -eq 1 ]]; then
  RUN_PATH="$OUT"
  if [[ "$OUT" != /* && "$OUT" != ./* ]]; then
    RUN_PATH="./$OUT"
  fi
  log "running $RUN_PATH"
  set +e
  "$RUN_PATH"
  CODE=$?
  set -e
  log "exit code $CODE"
  exit "$CODE"
fi