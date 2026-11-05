# Bootstrap

Builds the **self-hosted Raptor compiler** (sources in `selfhost/`) from a checked-in LLVM IR file, **without** the Rust toolchain.

The self-hosted compiler is the **primary** way to compile Raptor programs in this repository. CI bootstraps it and runs the self-host test suite against it.

## Contents

| File           | Description                                                          |
| -------------- | -------------------------------------------------------------------- |
| `bootstrap.sh` | Builds the compiler executable from `raptor.ll`                      |
| `raptor.ll`    | LLVM IR of the self-hosted compiler (generated; do not edit by hand) |
| `raptor`       | Resulting executable (created by `bootstrap.sh`, not committed)      |

## Requirements

- LLVM 18: `llc-18` and `clang-18` (override with `LLVM_VERSION=19 …`)
- Linux (tested under WSL)

`raptor.ll` is written for LLVM 18 and may not load in other versions.

## Build

From the repository root:

```sh
./bootstrap/bootstrap.sh
```

This creates `build/` if needed and writes `bootstrap/raptor`.

Optional arguments (paths relative to the current directory):

```sh
./bootstrap/bootstrap.sh [ir.ll] [output]
```

## Use

```sh
# Compile a program (IR + link under build/)
./bootstrap/raptor ./examples/demo.rp

# Compile and run
./bootstrap/raptor --run ./examples/demo.rp

# Memory mode of the *compiled program* (default: arc)
./bootstrap/raptor --memory arc  ./examples/demo.rp
./bootstrap/raptor --memory leak ./examples/demo.rp
```

Flags match the self-hosted CLI (see root [README.md](../README.md)): `-v`, `--unsafe`, `-o`, `--link`, `-O0`…`-O3`, `--overflow`, `--memory`, `--run`.

Low-level linking of an existing `.ll` file can also be done with `selfhost/raptor.sh` (opt → llc → clang):

```sh
./selfhost/raptor.sh -o ./build/demo ./build/out.ll
./selfhost/raptor.sh --run -O2 -o ./build/demo ./build/out.ll
```

Notes:

- Every compile overwrites IR under `build/`. Copy it if you need to keep a particular revision.
- Prefer checking process exit code and diagnostics on stderr when a program is rejected.

## Regenerating `raptor.ll`

After changing anything under `selfhost/`, rebuild the IR with the compiler itself:

```sh
./bootstrap/raptor ./selfhost/raptor.rp
# Copy the IR the driver wrote (typically under build/) into bootstrap/
cp ./build/raptor.ll ./bootstrap/raptor.ll   # or ./build/out.ll — use the file your driver emits
./bootstrap/bootstrap.sh
```

A healthy bootstrap is a **fixpoint**: compiling `selfhost/raptor.rp` again with the new binary should produce byte-identical IR:

```sh
cp ./build/out.ll /tmp/before.ll    # adjust path to the IR your driver writes
./bootstrap/raptor ./selfhost/raptor.rp
cmp ./build/out.ll /tmp/before.ll && echo "fixpoint OK"
```

## Memory modes

The self-hosted backend can emit code under two strategies (CLI `--memory`):

| Mode                | Behaviour                                                                                                                                         |
| ------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------- |
| **`arc`** (default) | Automatic reference counting for `str`, vectors, structs, and enums. Runtime helpers (`rt_alloc`, `rt_retain`, `rt_release`, …) manage lifetimes. |
| **`leak`**          | Plain `malloc`; nothing is freed. Fine for short-lived compiles; memory grows with program size and input.                                        |

Details of the ARC model: [docs/memory-management.md](../docs/memory-management.md).

## Status

Verified in normal development / CI:

- The compiler compiles itself and reaches a self-compile fixpoint.
- End-to-end cases under `selfhost/tests/` run against `./bootstrap/raptor` with `--memory arc`.
- Example programs (including networking samples) are exercised as part of that suite and manual checks.

Known limitations (non-exhaustive):

- Reference cycles are not collected (inherent to plain refcounting).
- By-reference arguments for primitives (`&i64 x`) write back only when the argument is a plain variable; field/element form uses a temporary.
- No separate interpret-only mode — the self-hosted tool always compiles.

## Relation to the Rust tree

`src/` still contains the original Rust frontend, interpreter, and LLVM backend, used mainly for the **`lsp`** binary and as a reference implementation. Day-to-day compilation of Raptor programs is expected to go through **bootstrap + selfhost**, not `cargo run --bin raptor`.
