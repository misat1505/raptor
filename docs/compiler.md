# Compiler

Raptor compiles programs to native code through **LLVM 18**.

The **primary** implementation is the self-hosted compiler in `selfhost/` (bootstrapped from `bootstrap/raptor.ll`). A Rust LLVM backend remains under `src/backend/llvm/` as a reference; the language server still links against the Rust crates, but normal program builds use the self-hosted path.

## Responsibilities

The compiler:

1. receives a checked (or `--unsafe`) AST after lexing, parsing, import resolution, and macro expansion;
2. lowers the program to LLVM IR;
3. optionally records optimization level for the link step;
4. writes IR under `build/`;
5. invokes `llc` (object file) and `clang` (link), optionally `opt` when `-O1`/`-O2`/`-O3` is set;
6. optionally runs the resulting executable (`--run`).

## Pipeline

![](./diagram/compiler_pipeline.png)

## Self-hosted backend

Sources live under `selfhost/backend/llvm/`:

- textual IR emission (no inkwell);
- libc / runtime declarations (`libc-decls.rp`);
- expression and statement lowering;
- standard functions (I/O, vectors, files, **TCP**, …);
- ARC runtime hooks when `--memory arc` is selected.

Bootstrap entry: [bootstrap/README.md](../bootstrap/README.md).

## LLVM version

The driver targets LLVM 18 by default:

```text
LLVM_VERSION = 18
```

Tools resolved as `tool-18` with fallback to unversioned `tool`:

```text
llc-18 / llc
clang-18 / clang
opt-18 / opt   (only when an -O level is requested)
```

## Output files

Compilation artifacts are placed under:

```text
build/
```

For an input such as `basic.rp`, the driver typically produces IR, an object file, and an executable named from the file stem (exact names depend on `-o` and the driver defaults).

## Optimization

```text
-O0    No optimization (default; opt may be skipped)
-O1    Basic optimization
-O2    Default optimization
-O3    Aggressive optimization
```

Levels are applied with LLVM’s pass pipeline via `opt` before `llc`, when requested.

## Memory mode

```text
--memory arc    Reference counting for managed types (default)
--memory leak   malloc only; nothing freed
```

See [memory-management.md](memory-management.md).

## Integer overflow

```text
--overflow ignore   (default)
--overflow warn
--overflow error
```

Non-`ignore` policies emit LLVM overflow intrinsics and diagnostics for checked arithmetic.

## Compile and run

```bash
./bootstrap/raptor examples/basic.rp
./bootstrap/raptor --run examples/basic.rp
./bootstrap/raptor -o ./build/basic -O2 --run examples/basic.rp
```

`--run` implies a full compile + link, then executes the binary.

Linking a pre-existing `.ll` file:

```bash
./selfhost/raptor.sh -o ./build/out ./build/out.ll
./selfhost/raptor.sh --run -O2 -o ./build/out ./build/out.ll
```

## Networking codegen

TCP builtins (`tcp_listen`, `tcp_accept`, `tcp_connect`, `tcp_read`, `tcp_write`, `tcp_close`) lower to libc socket calls (`socket`, `bind`, `listen`, `accept`, `connect`, `recv`, `send`, `close`, `inet_addr`, …) declared in the IR runtime section. Handles are plain file descriptors (`i64` in the language).

## Errors

Compiler diagnostics use:

```text
error: <message>
  --> <file>:<line>:<column>
```

Failures while running `llc`, `clang`, or `opt` are reported as build errors; a failure in one stage stops the next.

## Error boundary

- **Semantic checker** — static types and related rules.
- **Compiler / link** — IR generation and native toolchain.
- **Runtime** — behaviour of the generated executable (including ARC diagnostics if enabled).

The self-hosted compiler has **no interpreter**; runtime issues appear only after a successful native build.
