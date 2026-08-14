# Compiler

The compiler translates the Raptor AST into LLVM IR and then uses LLVM 18 tools to
produce a native executable.

## Responsibilities

The compiler:

1. traverses the AST;
2. generates LLVM IR;
3. optionally optimizes the generated IR;
4. writes the IR to disk;
5. invokes `llc-18` to create an object file;
6. invokes `clang-18` to link the final executable.

## Pipeline

```text
Raptor source
     │
     ▼
   Lexer
     │
     ▼
   Parser
     │
     ▼
Semantic Checker
     │
     ▼
 Compiler
     │
     ▼
 LLVM IR (.ll)
     │
     ▼
  llc-18
     │
     ▼
 object (.o)
     │
     ▼
 clang-18
     │
     ▼
 executable
```

The semantic checker normally runs before compilation and performs type checking.
`--unsafe` can bypass that stage.

## LLVM version

The current driver targets LLVM 18:

```text
LLVM_VERSION = 18
```

It invokes:

```text
llc-18
clang-18
```

Both tools need to be installed and available on `PATH`.

## Output files

Compilation artifacts are placed in:

```text
build/
```

For an input such as:

```text
basic.rp
```

the driver creates paths based on the file stem:

```text
build/basic.ll
build/basic.o
build/basic       # Unix-like systems
build/basic.exe   # Windows
```

## Optimization

The CLI supports:

```text
-O0    No optimization
-O1    Basic optimization
-O2    Default optimization
-O3    Aggressive optimization
```

These are mapped to LLVM optimization levels before the IR is written.

## Compile and run

Compile only:

```bash
cargo run -- --compile basic.rp
```

Compile and run:

```bash
cargo run -- --run basic.rp
```

`--run` implies `--compile`.

The driver reports the generated IR and executable paths after successful stages.

## Errors

Compiler diagnostics use the same compiler-style format:

```text
error: <message>
  --> <file>:<line>:<column>
```

For example:

```text
error: Undeclared variable 'doesnt_exists'.
  --> basic.rp:7:9
```

This represents a compiler-stage diagnostic for a variable that cannot be resolved
during code generation.

The compiler can also report failures while invoking external LLVM tools. If `llc-18`
or `clang-18` cannot be started, the driver reports that the tool is missing or not
available on `PATH`. If a tool exits unsuccessfully, its diagnostic output is returned
as part of the build error.

A failure in one compilation stage stops subsequent stages.

## Error boundary

The compiler is responsible for diagnostics that arise during code generation and the
native build pipeline.

Static type checking belongs to the semantic checker. Runtime execution failures
belong to the interpreter.
