# Raptor

Raptor is a **statically typed, compiled, self-hosted** programming language.
The compiler is written in Raptor itself and lowers programs to native code through **LLVM 18**.

The language supports mutable variables, block scoping, functions, references, structs, enums with payloads, pattern matching, multidimensional vectors, type conversions, structured control flow, macros, TCP networking, and linking against external C libraries (e.g. Raylib).

## Runtime dependency: libc only

Both of the following link **only against the C standard library**:

| Artifact                        | What it is                                     | Runtime deps                                         |
| ------------------------------- | ---------------------------------------------- | ---------------------------------------------------- |
| **`bootstrap/raptor`**          | The Raptor compiler itself (self-hosted)       | **libc** (+ system linker/loader)                    |
| **Programs compiled by Raptor** | Native executables produced from `.rp` sources | **libc** (plus any libraries you pass with `--link`) |

Codegen talks to the OS through explicit `libc` declarations (`malloc`, `printf`, sockets, file I/O, …). Optional third-party libraries are only pulled in when you request them at link time.

Build-time tools (`llc`, `clang`, optionally `opt`) are required to **compile**, not to **run**, finished executables.

## What ships in this repo

| Component                                                | Role                                                                                                   |
| -------------------------------------------------------- | ------------------------------------------------------------------------------------------------------ |
| **Self-hosted compiler** (`src/`, entry `src/raptor.rp`) | Full pipeline: lexer → parser → imports → macros → semantic checker → LLVM IR → object → executable    |
| **Bootstrap** (`bootstrap/`)                             | Checked-in `raptor.ll` + script that links `bootstrap/raptor`                                          |
| **LSP** (`tools/lsp/`)                                   | Thin Rust `tower-lsp` server; analysis is done by a Raptor `lsp-backend` subprocess (JSON over stdout) |
| **Rapwash** (`tools/rapwash/`)                           | Source **formatter** for `.rp` files (layout / pretty-print)                                           |
| **Stdlib** (`stdlib/`)                                   | Standard library sources imported by programs                                                          |
| **Examples** (`examples/`)                               | Sample programs (including TCP and Raylib)                                                             |
| **Tests** (`tests/`)                                     | End-to-end suite driven by `tests/test_runner.py`                                                      |
| **Plugins** (`plugins/`)                                 | Editor integrations                                                                                    |

## Table of contents

- [Language pipeline](#language-pipeline)
- [Quick start](#quick-start)
- [CLI options](#cli-options)
- [Example program](#example-program)
- [Language overview](#language-overview)
- [Documentation](#documentation)
- [Testing](#testing)
- [LSP](#lsp)
- [LLVM](#llvm)
- [Repository layout](#repository-layout)

## Language pipeline

![](./docs/diagram/compiler_pipeline.png)

## Quick start

### Requirements

- Linux (WSL is fine)
- **LLVM 18** tools on `PATH`: `llc-18` / `llc`, `clang-18` / `clang` (and `opt-18` when using `-O1`/`-O2`/`-O3`)
- A working system **libc** and linker (normally provided with `clang`)

### Bootstrap the compiler

```bash
chmod +x ./bootstrap/bootstrap.sh
./bootstrap/bootstrap.sh
# produces ./bootstrap/raptor
```

### Compile a program

```bash
./bootstrap/raptor examples/basic.rp
./bootstrap/raptor --run examples/basic.rp
./bootstrap/raptor -o ./build/hello -O2 --run examples/basic.rp
```

Artifacts land under `build/` by default (IR, object, executable). The resulting executable depends only on **libc** (unless you pass extra `--link` objects/libs).

### Rebuild the compiler from sources

After editing anything under `src/`:

```bash
./bootstrap/raptor ./src/raptor.rp -o ./build/raptor
# promote IR / binary as described in bootstrap/README.md
```

## CLI options

```text
./bootstrap/raptor [OPTIONS] <file.rp>

  -v, --verbose          Phase timings
  --unsafe               Skip semantic checker
  --run                  Compile, link, then execute
  -o <FILE>              Output executable path
  --link <OBJ>           Extra object/library to link (repeatable)
  -O0 | -O1 | -O2 | -O3  Optimization level (default -O0)
  --memory arc|leak      ARC (default) or leak-everything mode
  --overflow ignore|warn|error
  --dump                 Dump AST (debug)
  --arc / --leak         Shortcuts for --memory
```

## Example program

```raptor
fn main(): void {
    println("Hello, Raptor");
}
```

```bash
./bootstrap/raptor --run hello.rp
```

## Language overview

| Area             | Support                                                                                                                       |
| ---------------- | ----------------------------------------------------------------------------------------------------------------------------- |
| Types            | `void`, `bool`, `char`, `str`, integers (`i8`…`i64`, `u8`…`u64`), `f64`, vectors `T[]`, structs, enums with optional payloads |
| Functions        | Nested scopes, pass-by-value / pass-by-reference (`&T`)                                                                       |
| Control flow     | `if` / `else`, `while`, `for`, `switch`, `match`, `break`, `continue`, `return`                                               |
| Modules          | `import "path.rp"`                                                                                                            |
| Interop          | `extern fn` declarations; link with `--link`                                                                                  |
| Macros / derives | `derives Debug`, `Json`, `Clone` (and related expansion)                                                                      |
| Networking       | std helpers such as `tcp_listen`, `tcp_accept`, `tcp_connect`, `tcp_read`, `tcp_write`, …                                     |

Full grammar: [docs/grammar.md](docs/grammar.md).

## Documentation

| Topic                    | Document                                               |
| ------------------------ | ------------------------------------------------------ |
| Grammar                  | [docs/grammar.md](docs/grammar.md)                     |
| Lexer                    | [docs/lexer.md](docs/lexer.md)                         |
| Parser                   | [docs/parser.md](docs/parser.md)                       |
| Semantic checker         | [docs/semantic-checker.md](docs/semantic-checker.md)   |
| Compiler / LLVM pipeline | [docs/compiler.md](docs/compiler.md)                   |
| Memory management        | [docs/memory-management.md](docs/memory-management.md) |
| Bootstrap                | [bootstrap/README.md](bootstrap/README.md)             |
| LSP                      | [tools/lsp/README.md](tools/lsp/README.md)             |

## Testing

CI bootstraps the compiler and runs the ARC test suite:

```bash
./bootstrap/bootstrap.sh
python3 ./tests/test_runner.py run \
  --compiler ./bootstrap/raptor \
  --memory arc
```

## LSP

Editor support lives under `tools/lsp/`:

- **`raptor-lsp`** - Rust process speaking LSP (`tower-lsp`)
- **`lsp-backend`** - Raptor program that runs the frontend through the semantic checker and prints JSON (diagnostics, hovers, definitions)

See [tools/lsp/README.md](tools/lsp/README.md). VS Code: set `raptor.serverPath` to the `raptor-lsp` binary and `RAPTOR_LSP_BACKEND` to the backend executable.

## LLVM

Native codegen targets **LLVM 18**. Tools must be available as `*-18` or unversioned fallbacks. LLVM is a **compile-time** dependency of the driver; linked programs do not embed LLVM.

## Repository layout

```text
bootstrap/     Checked-in IR + bootstrap.sh → bootstrap/raptor
src/           Compiler written in Raptor (*.rp)
  raptor.rp    Driver / CLI entry
  frontend/    Lexer, parser, AST
  semantic-checker/
  backend/llvm/
  …
stdlib/        Standard library
tests/         End-to-end tests + test_runner.py
tools/lsp/     Language server (Rust host + Raptor backend)
tools/rapwash/ Formatting / doc utilities
examples/      Example programs
docs/          Documentation
plugins/vscode/ VS Code extension
```

## Errors and diagnostics

```text
error: <message>
  --> <file>:<line>:<column>
```

Stages: lexer, parser, import resolver, macro expander, semantic checker, and the LLVM / link steps.

## License

Apache License 2.0 - see [LICENSE](LICENSE).
