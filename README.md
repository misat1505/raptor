# Raptor

Raptor is a custom interpreted and compiled programming language written in Rust.

It is a strongly and statically typed language with mutable variables, scoped execution,
functions, references, multidimensional vectors, type conversions, and structured control flow.

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

## Documentation

| Component        | Documentation                                        |
| ---------------- | ---------------------------------------------------- |
| Lexer            | [docs/lexer.md](docs/lexer.md)                       |
| Parser           | [docs/parser.md](docs/parser.md)                     |
| Semantic Checker | [docs/semantic-checker.md](docs/semantic-checker.md) |
| Interpreter      | [docs/interpreter.md](docs/interpreter.md)           |
| Compiler         | [docs/compiler.md](docs/compiler.md)                 |

## Quick start

### Build Raptor

Build the project in release mode:

```bash
cargo build --release
```

The Raptor executable will be available at:

```text
target/release/raptor
```

### Run a program with the interpreter

```bash
./target/release/raptor basic.rp
```

### Compile a program

Use `--compile` to compile a Raptor source file to a native executable:

```bash
./target/release/raptor --compile basic.rp
```

Generated compilation artifacts are written to `build/`.

### Compile and run a program

Use `--run` to compile the program and immediately execute the resulting executable:

```bash
./target/release/raptor --run basic.rp
```

`--run` implies `--compile`.

For development, `cargo run` can still be used as a convenient alternative, for example:

```bash
cargo run -- basic.rp
cargo run -- --compile basic.rp
cargo run -- --run basic.rp
```

However, the recommended way to run Raptor locally is to build it once in release mode
and invoke the resulting executable directly:

```bash
cargo build --release
./target/release/raptor --run examples/basic.rp
```

### CLI options

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

Save the program as `examples/demo.rp` and run it with:

```bash
cargo build --release
./target/release/raptor examples/demo.rp
```

Or compile it to a native executable:

```bash
./target/release/raptor --compile examples/demo.rp
```

To compile and immediately execute it:

```bash
./target/release/raptor --compile --run examples/demo.rp
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

## Project structure

```text
src/
├── lexer
├── parser
├── semantic_checker
├── interpreter
├── compiler
├── ast
├── errors
├── scope_manager
├── stack
├── value
├── tokens
└── ...
```

## Testing

Run the test suite with:

```bash
cargo test
```

The project includes unit tests for core components and integration tests for the
language pipeline.

## LLVM

The native compilation pipeline currently targets LLVM 18 and invokes:

```text
llc-18
clang-18
```

These tools must be available on `PATH` when using `--compile` or `--run`.
