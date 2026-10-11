# Bootstrap

Builds the **self-hosted Raptor compiler** (the one written in Raptor, in `selfhost/`) from a checked-in LLVM IR file, without needing the Rust toolchain.

> [!WARNING]
> **This is unstable and not properly tested. For anything real, use the Rust compiler.**
>
> The self-hosted compiler can compile itself (see [Status](#status)), but that is all that has been verified.
> It should be treated as an experiment, not as a replacement for the Rust implementation.
>
> **It has no memory management at all.** Strings are raw C strings, and nothing is ever freed: no reference counting, no `free`, no bounds checks. Every allocation leaks until the process exits. This is fine for a short-lived compile of a small program, but memory use grows with the size of the input and can run out on large programs.

## Contents

| File | Description |
|---|---|
| `bootstrap.sh` | Builds the compiler executable from `raptor.ll` |
| `raptor.ll` | LLVM IR of the self-hosted compiler (generated, do not edit by hand) |
| `raptor` | The resulting executable (created by `bootstrap.sh`, not committed) |

## Requirements

- LLVM 18: `llc-18` and `clang-18` (override the version with `LLVM_VERSION=19 ...`)
- Linux (tested under WSL)

`raptor.ll` is written for LLVM 18 and may not load in other versions.

## Build

From the repository root:

```sh
./bootstrap/bootstrap.sh
```

This creates `build/` (if it does not exist) and builds `bootstrap/raptor`.

Optional arguments (relative to the current directory):

```sh
./bootstrap/bootstrap.sh [ir.ll] [output]
```

## Use

The self-hosted compiler only produces LLVM IR. It always writes it to `./build/out.ll` (relative to the current directory), then you link it with `selfhost/raptor.sh`:

```sh
./bootstrap/raptor ./examples/demo.rp
./selfhost/raptor.sh -o ./build/demo ./build/out.ll
./build/demo
```

Notes:

- `selfhost/raptor.sh` takes the **`.ll` file** as its input argument and the executable path via `-o`. Passing the compiler binary as input fails with `expected top-level entity`.
- Every run overwrites `build/out.ll`. Copy it if you want to keep it.
- The compiler exits with code `0` even when it rejects a program. Check for the error output, or for the absence of a fresh `out.ll`.

## Regenerating `raptor.ll`

After changing anything in `selfhost/`, rebuild the IR with the compiler itself:

```sh
./bootstrap/raptor ./selfhost/raptor.rp
cp ./build/raptor.ll ./bootstrap/raptor.ll
./bootstrap/bootstrap.sh
```

A healthy bootstrap is a fixpoint: compiling `selfhost/raptor.rp` again with the new binary should produce a byte-identical `out.ll`:

```sh
cp ./build/out.ll /tmp/before.ll
./bootstrap/raptor ./selfhost/raptor.rp
cmp ./build/out.ll /tmp/before.ll && echo "fixpoint OK"
```

## Status

What has been checked:

- The compiler compiles itself, and repeated self-compilation reaches a fixpoint (the generated IR is identical).
- A few programs from `examples/` (`demo`, `enums`, `fibbonacci`, `primes`) produce the same IR as stage 1 and behave the same as when built with the Rust compiler.

What is **not** covered:

- The test suite in `tests/` has not been run against it.
- At least `examples/stdlib_demo.rp` builds but crashes at runtime (segfault) when compiled by the self-hosted compiler.
- Most other examples have not been tried.

## Known limitations

- **No memory management.** Strings are plain NUL-terminated C strings, not the reference-counted `StrHeader` used by the Rust backend. Nothing is freed.
- **No bounds checking** on indexing. Out-of-range access reads arbitrary memory instead of reporting an error.
- By-reference arguments for primitives (`&i64 x`) are written back only when the argument is a plain variable. For a field or element (`&obj.count`) the callee modifies a temporary copy.
- Elements of vectors of primitive types (`u8[]`, `i64[]`, ...) are loaded as pointers, which happens to work in many cases but is not correct in general.
- `vector_stringify` is a stub and always returns `"[vector]"`.
- No optimization pass: the IR is linked as is.
- Integer arithmetic is done as 64-bit with zero extension, so negative values of narrower integer types may behave incorrectly.