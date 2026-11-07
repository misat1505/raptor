# Bootstrap

This directory holds the **seed** that makes the self-hosted compiler runnable without an existing Raptor binary:

| File           | Purpose                                                               |
| -------------- | --------------------------------------------------------------------- |
| `raptor.ll`    | LLVM IR of the compiler (`src/raptor.rp` and its imports), checked in |
| `bootstrap.sh` | Invokes `clang` / `llc` as needed to produce `./bootstrap/raptor`     |

After bootstrap, **`./bootstrap/raptor` is a normal native executable**. It depends only on **libc** (and the dynamic loader). It does not ship a Rust runtime or an interpreter.

## Requirements

- LLVM 18 toolchain: `clang-18` (or `clang`), typically also `llc-18` when regenerating IR elsewhere
- System libc development files (usual Linux/WSL defaults are enough)

## Building the bootstrap binary

```sh
chmod +x ./bootstrap/bootstrap.sh
./bootstrap/bootstrap.sh
./bootstrap/raptor --help
```

## Compiling programs

```sh
./bootstrap/raptor path/to/file.rp
./bootstrap/raptor --run path/to/file.rp
./bootstrap/raptor -o ./build/out -O2 --memory arc path/to/file.rp
```

Compiled programs are also native binaries that link **libc** only (unless you add `--link`).

## Regenerating `raptor.ll`

After changing sources under `src/`:

```sh
./bootstrap/raptor ./src/raptor.rp -o ./build/raptor
# Copy the IR the driver wrote into bootstrap/ (name may be build/raptor.ll or similar)
cp ./build/raptor.ll ./bootstrap/raptor.ll   # adjust to the path your driver emits
./bootstrap/bootstrap.sh
```

A healthy bootstrap is a **fixpoint**: compiling `src/raptor.rp` again should produce IR that matches the previous emission (modulo intentional changes).

```sh
./bootstrap/raptor ./src/raptor.rp
cmp ./build/raptor.ll /tmp/before.ll && echo "fixpoint OK"
```

## Memory modes

| Mode                | Behaviour                                                                                                                                                           |
| ------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **`arc`** (default) | Reference counting for `str`, vectors, structs, enums via runtime helpers (`rt_alloc`, `rt_retain`, `rt_release`, …) implemented on top of **libc** `malloc`/`free` |
| **`leak`**          | `malloc` only; nothing is freed                                                                                                                                     |

See [docs/memory-management.md](../docs/memory-management.md).

## Status

- The compiler compiles itself and is exercised in CI with `./bootstrap/bootstrap.sh` and `tests/test_runner.py` under ARC.
- No interpret-only mode: the driver always goes through LLVM to a native binary.
- Reference cycles are not collected (plain refcounting).

## Relation to `tools/lsp`

Editor analysis uses a separate Raptor binary (`lsp-backend`) driven by a small Rust LSP host. That host is **not** required to compile or run Raptor programs. Program builds stay on `./bootstrap/raptor` + libc.
