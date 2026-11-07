# Semantic Checker

The semantic checker validates the AST after import resolution and macro expansion, before LLVM lowering.

Implementation: **`src/semantic-checker/`** (Raptor). The command-line driver runs it unless `--unsafe` is passed.

The same checker powers editor features: `tools/lsp/backend` runs the frontend through this stage and emits **diagnostics**, **hovers**, and **definitions** as JSON for the Rust LSP host.

## Responsibilities

- Traverse the program (statements, expressions, functions, types)
- Resolve and check types (including vectors, structs, enums)
- Validate calls (arity, argument types, pass-by-value / reference)
- Validate assignments and declarations
- Track scopes via `src/semantic-checker/stack/`
- Type arithmetic / comparisons via `type-alu/`
- Collect **all** diagnostics (does not stop at the first error)
- Collect **hover** and **go-to-definition** metadata for the LSP backend

## Pipeline position

```text
Lexer → Parser → Import resolver → Macro expander → Semantic checker → Compiler (LLVM)
                                                      │
                                                      ├── errors / warnings
                                                      └── hovers + definitions (LSP)
```

## Type checking

Raptor is strongly and statically typed. Example rejection:

```raptor
i64 foo = 42;
foo = "hello";
```

```text
error: Cannot assign 'str' to variable 'foo' which was previously declared as 'i64'.
  --> basic.rp:4:1
```

## Vectors

```raptor
i64[] values = [1, 2, 3];
i64[][] matrix = [[1, 2], [3, 4]];
```

Element types and nesting depth are checked at assignments, indexing, and calls.

## Functions and references

- Parameter and return types must match at each call site
- `&T` parameters require a compatible reference argument
- Unknown functions and arity mismatches are diagnostics

## Control flow

- `if` / `while` / `for` conditions must be `bool`
- `return` types must match the enclosing function
- `match` scrutinees and arms are checked against enum declarations
- `break` / `continue` only inside appropriate loops

## LSP metadata

While checking, the self-hosted checker records:

````text
hovers:      { span, contents }   # often a ```raptor fenced type/signature
definitions: { use span, def span }
````

`tools/lsp-backend` serializes these alongside diagnostics for `raptor-lsp`.

## Disabling the checker

```bash
./bootstrap/raptor --unsafe file.rp
```

Useful for IR experiments; type errors become backend or runtime failures.

## Layout

```text
src/semantic-checker/
  semantic-checker.rp      # main passes
  semantic-helpers.rp
  semantic-checker-mod.rp
  stack/stack.rp
  type-alu/type-alu.rp
```
