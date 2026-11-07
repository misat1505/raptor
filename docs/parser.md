# Parser

The parser builds a typed AST from the token stream.

Implementation: **`src/frontend/parser/`** (modular `.rp` files by construct), orchestrated through `parser-mod.rp` / `program.rp`.

## Responsibilities

- Consume tokens from the lexer
- Parse declarations, statements, expressions, types, and modules
- Produce `Program` (`statements`, `functions`, `extern_functions`, `declared_types`)
- Attach **spans** to every major node for diagnostics and LSP
- Surface `ParserError` diagnostics

## AST design (self-hosted)

Raptor has no generics, so the tree uses explicit `*Node { value, span }` wrappers instead of a single `Node<T>` type. Recursive expression forms use small carrier structs (e.g. binary operators) where a Rust enum would use multi-field variants.

See `src/frontend/parser/ast.rp`.

## Modules and functions

Top-level forms include:

- function declarations and `extern` functions
- struct / enum declarations (with optional `derives`)
- statements (`let`, assignments, calls, control flow, `import`, …)

## Expressions

Parsed with precedence layers (literals, calls, field access, indexing, unary, casts, multiplicative, additive, relational, logical, …) under `src/frontend/parser/expressions/`.

## Pipeline position

```text
Lexer → Parser → Import resolver → Macro expander → Semantic checker → LLVM
```

## Errors

Syntax errors include a message, severity, and span. The driver prints them in the standard compiler style and exits before codegen when appropriate.
