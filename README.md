# Raptor

Raptor is a custom interpreted and compiled programming language written in Rust.

It is a strongly and statically typed language with mutable variables, scoped execution,
functions, references, multidimensional vectors, type conversions, and structured control flow.

The project provides **two executables**:

* `raptor` — the Raptor compiler and interpreter
* `lsp` — the Language Server Protocol (LSP) server for editor integration

## Language pipeline

```text
                 ┌──────────────┐
Source ─────────►│    Lexer     │
                 └──────┬───────┘
                        │ tokens
                        ▼
                 ┌──────────────┐
                 │    Parser    │
                 └──────┬───────┘
                        │ AST
                        ▼
                 ┌──────────────┐
                 │   Semantic   │
                 │    Checker   │
                 └──────┬───────┘
                        │ checked AST
                 ┌──────┴───────┐
                 ▼              ▼
          ┌────────────┐  ┌────────────┐
          │ Interpreter│  │  Compiler  │
          └────────────┘  └─────┬──────┘
                                │ LLVM IR
                                ▼
                           LLVM 18 tools
                                │
                                ▼
                            executable
```

The semantic checker performs **type checking** and other static validation before the
program is interpreted or compiled, unless `--unsafe` is explicitly used.

## Executables

### `raptor`

The `raptor` executable is the main Raptor command-line tool. It provides both the
interpreter and compiler.

It can:

* interpret Raptor source files;
* compile Raptor programs to native executables;
* compile and immediately run programs;
* control compiler optimization levels;
* optionally skip semantic checking with `--unsafe`.

### `lsp`

The `lsp` executable is the Raptor Language Server Protocol implementation.

It provides language-server functionality for editors and IDEs that support LSP,
allowing Raptor source files to be integrated with development environments.

The LSP server is a separate executable from the `raptor` compiler/interpreter.

## Documentation

| Component        | Documentation                                        |
| ---------------- | ---------------------------------------------------- |
| Grammar          | [docs/grammar.md](docs/grammar.md)                   |
| Lexer            | [docs/lexer.md](docs/lexer.md)                       |
| Parser           | [docs/parser.md](docs/parser.md)                     |
| Semantic Checker | [docs/semantic-checker.md](docs/semantic-checker.md) |
| Interpreter      | [docs/interpreter.md](docs/interpreter.md)           |
| Compiler         | [docs/compiler.md](docs/compiler.md)                 |

## Quick start

### Build the project

The project contains two executable targets: `raptor` and `lsp`.

Build both in release mode:

```bash
cargo build --release
```

The resulting executables will be available at:

```text
target/release/raptor
target/release/lsp
```

### Build only `raptor`

```bash
cargo build --release --bin raptor
```

The executable will be available at:

```text
target/release/raptor
```

### Build only `lsp`

```bash
cargo build --release --bin lsp
```

The executable will be available at:

```text
target/release/lsp
```

## Running Raptor programs

### Run a program with the interpreter

The default behavior of `raptor` is to interpret a Raptor source file:

```bash
./target/release/raptor examples/basic.rp
```

### Compile a program

Use `--compile` to compile a Raptor source file to a native executable:

```bash
./target/release/raptor --compile examples/basic.rp
```

Generated compilation artifacts are written to `build/`.

### Compile and run a program

Use `--run` to compile the program and immediately execute the resulting native
executable:

```bash
./target/release/raptor --run examples/basic.rp
```

`--run` implies `--compile`.

The same operation can also be written explicitly as:

```bash
./target/release/raptor --compile --run examples/basic.rp
```

## Development

During development, `cargo run` can be used to run either executable directly.

### Run the interpreter

```bash
cargo run --bin raptor -- examples/basic.rp
```

### Compile a program

```bash
cargo run --bin raptor -- --compile examples/basic.rp
```

### Compile and run a program

```bash
cargo run --bin raptor -- --run examples/basic.rp
```

### Run the LSP server

```bash
cargo run --bin lsp
```

For normal local usage, it is recommended to build the project once in release mode:

```bash
cargo build --release
```

Then use the generated executables directly:

```bash
./target/release/raptor --run examples/basic.rp
```

or:

```bash
./target/release/lsp
```

## CLI options

The `raptor` executable supports the following options:

```text
-h, --help      Show help
--unsafe        Skip semantic checking
--compile       Compile instead of interpreting
--run           Compile and then run
-O0             No optimization
-O1             Basic optimization
-O2             Default optimization
-O3             Aggressive optimization
```

The compiler writes generated artifacts to `build/`.

The `lsp` executable is a separate LSP server and does not use the `raptor` command-line
interface described above.

## Example program

The following program demonstrates several core Raptor features: variables, functions,
references, loops, conditionals, vectors, and static typing.

```text
fn sum(i64[] values): i64 {
    i64 total = 0;

    for (i64 i = 0; i < vector_size(&values); i += 1) {
        total = total + values[i];
    }

    return total;
}

fn max(i64[] values): i64 {
    i64 result = values[0];

    for (i64 i = 1; i < vector_size(&values); i += 1) {
        if (values[i] > result) result = values[i];
    }

    return result;
}

fn average(i64[] values): f64 {
    return sum(values) as f64 / vector_size(&values) as f64;
}

fn add_bonus(&i64 score, i64 bonus): void {
    score = score + bonus;

    if (score > 100) score = 100;
}

fn main(): void {
    i64[] scores = [72, 85, 91, 68, 94];

    i64 total = sum(scores);
    i64 best = max(scores);
    f64 avg = average(scores);

    println("Results:");
    println("---------");

    print("Total: ");
    println(total as str);

    print("Best: ");
    println(best as str);

    print("Average: ");
    println(avg as str);

    i64 final_score = best;
    add_bonus(&final_score, 5);

    print("Final score: ");
    println(final_score as str);

    if (final_score >= 90) {
        println("Status: excellent");
    } else if (final_score >= 75) {
        println("Status: good");
    } else {
        println("Status: needs improvement");
    }
}

main();
```

Save the program as `examples/demo.rp`.

Run it using the interpreter:

```bash
./target/release/raptor examples/demo.rp
```

Or compile it to a native executable:

```bash
./target/release/raptor --compile examples/demo.rp
```

To compile and immediately execute it:

```bash
./target/release/raptor --run examples/demo.rp
```

## Language overview

Raptor currently supports:

* `i64`, `f64`, `str`, `bool`, and `void`;
* mutable variables with block-based scoping;
* functions and recursion;
* parameters passed by value or by reference;
* `if`, `for`, `while`, and `switch`;
* `break`, `continue`, and `return`;
* arithmetic, comparison, and logical operators;
* explicit casts with `as`;
* vectors, including multidimensional types such as `i64[][]`;
* built-in functions such as `print`, `input`, and `mod`.

### Vectors

Vector types may have multiple dimensions:

```text
i64[]       # one-dimensional vector
i64[][]     # two-dimensional vector
i64[][][]   # three-dimensional vector
```

When a vector is passed **by value**, the language uses a **shallow copy**. The vector
structure is copied, while nested vector data is not recursively deep-copied.

## Errors and diagnostics

Raptor diagnostics use a compact compiler-style format:

```text
error: <message>
  --> <file>:<line>:<column>
```

Different pipeline stages report different classes of errors.

* The lexer reports malformed lexical input.
* The parser reports syntax errors.
* The semantic checker performs static type checking and related validation.
* The interpreter reports runtime errors.
* The compiler reports code-generation and compilation errors.

See the individual component documentation for examples and details.

## Testing

Run the complete test suite with:

```bash
cargo test
```

The project includes unit tests for core components and integration tests for the
language pipeline.

## LLVM

The native compilation pipeline currently targets **LLVM 18** and invokes:

```text
llc-18
clang-18
```

These tools must be available on `PATH` when using `--compile` or `--run`.

The LLVM toolchain is only required for native compilation. Running a program through
the interpreter does not require the native compilation step.

## Cargo targets

The project defines the following Cargo targets:

```toml
[lib]
name = "raptor_lib"
path = "src/lib.rs"

[[bin]]
name = "raptor"
path = "src/main.rs"

[[bin]]
name = "lsp"
path = "src/bin/lsp.rs"
```

This means the project builds:

* `raptor` — compiler/interpreter CLI;
* `lsp` — Language Server Protocol server;
* `raptor_lib` — shared library crate.

Build both executables with:

```bash
cargo build --release
```

Build only the compiler/interpreter:

```bash
cargo build --release --bin raptor
```

Build only the LSP server:

```bash
cargo build --release --bin lsp
```

## Summary

Raptor consists of a library and two executable targets:

```text
┌─────────────────────┐
│       raptor        │
│                     │
│  Interpreter        │
│  Compiler           │
│  CLI                │
└─────────────────────┘

┌─────────────────────┐
│         lsp         │
│                     │
│  Language Server    │
│  Protocol (LSP)     │
└─────────────────────┘

┌─────────────────────┐
│     raptor_lib      │
│                     │
│   Raptor library    │
└─────────────────────┘
```

Build the complete project:

```bash
cargo build --release
```

Run a Raptor program:

```bash
./target/release/raptor program.rp
```

Compile a Raptor program:

```bash
./target/release/raptor --compile program.rp
```

Compile and run a Raptor program:

```bash
./target/release/raptor --run program.rp
```

Start the LSP server:

```bash
./target/release/lsp
```
