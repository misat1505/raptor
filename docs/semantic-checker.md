# Semantic Checker

The semantic checker validates the parsed AST before normal interpretation or compilation.

Its most important responsibility is **type checking**.

## Responsibilities

The semantic checker:

- traverses the AST using the visitor pattern;
- checks expressions and statements for static type correctness;
- validates function calls;
- checks argument counts and passing modes;
- checks assignments against declared variable types;
- collects diagnostics instead of stopping at the first semantic error.

## Pipeline position

```text
Lexer
  │
  ▼
Parser
  │
  ▼
AST
  │
  ▼
Semantic Checker
  │
  ├── errors → stop
  │
  └── valid → Interpreter / Compiler
```

The command-line driver runs the checker unless `--unsafe` is supplied.

## Type checking

Raptor is strongly and statically typed.

For example:

```text
i64 foo = 42;
foo = "hello";
```

is rejected by the semantic checker because `foo` has already been declared as `i64`.

The corresponding diagnostic is:

```text
error: Cannot assign 'str' to variable 'foo' which was previously declared as 'i64'.
  --> basic.rp:4:1
```

This is a semantic error rather than a parser error: both statements are syntactically
valid.

## Vectors and types

Vector types participate in type checking just like scalar types.

Examples:

```text
i64[] values = [1, 2, 3];

i64[][] matrix = [
    [1, 2],
    [3, 4]
];
```

A vector may have multiple dimensions:

```text
i64[][][]
```

When a vector is passed by value, the language specifies a **shallow copy** rather
than recursively copying nested vector contents.

The semantic checker is responsible for validating that vector expressions and
assignments are used with compatible types.

## Function calls

The checker validates:

- whether a called function exists;
- whether the number of arguments is correct;
- whether arguments have compatible types;
- whether value/reference passing matches the function declaration.

For example, a function expecting a reference parameter cannot receive an ordinary
value argument in its place.

## Collected diagnostics

The semantic checker stores discovered errors internally. This allows a checking pass
to report multiple problems rather than terminating at the first one.

The command-line driver summarizes the result after the pass.

## Errors

Semantic errors use the same compiler-style location format:

```text
error: <message>
  --> <file>:<line>:<column>
```

Example:

```text
error: Cannot assign 'str' to variable 'foo' which was previously declared as 'i64'.
  --> basic.rp:4:1
```

If semantic errors are present, normal execution or compilation is stopped.

## `--unsafe`

The CLI option:

```text
--unsafe
```

skips the semantic checker.

This is an explicit escape hatch and changes the normal pipeline from:

```text
Lexer → Parser → Semantic Checker → Interpreter / Compiler
```

to:

```text
Lexer → Parser → Interpreter / Compiler
```

## Design boundary

The semantic checker should answer:

> Can this parsed program be shown to satisfy the language's static rules?

Runtime failures remain the interpreter's responsibility, while code-generation failures
remain the compiler's responsibility.
