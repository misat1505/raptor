# Lexer

The lexer turns source text into a stream of tokens for the parser.

Implementation: **`src/frontend/lexer/`** (`lexer.rp`, `lazy-stream-reader.rp`), shared by the compiler driver and the LSP backend.

## Responsibilities

- Read UTF-8 / ASCII source through a lazy stream
- Recognize keywords, identifiers, literals, operators, and punctuation
- Track **positions** (`line`, `column`, `offset`, optional `filename`) for diagnostics
- Enforce configurable limits (identifier length, comment length)
- Report lexical errors without aborting the whole compiler pipeline early when the parser can still recover

## Inputs

- Source string
- Optional file name (for diagnostics)
- `LexerOptions` (`max_comment_length`, `max_identifier_length`)

## Outputs

- Tokens consumed by the parser (`src/frontend/tokens.rp`)
- `LexerError` values with severity and span

## Position and spans

Each token carries a `Span` (`start` / `end` `Position`). Later stages (parser, checker, LSP) reuse these spans for underlines, hovers, and go-to-definition.

## Keywords and literals

The lexer classifies language keywords (`fn`, `let`, `if`, `struct`, `enum`, `match`, …), boolean literals, character and string literals, and numeric literals used by the type system (`i64`, `f64`, …).

## Pipeline position

```text
Source → Lexer → Parser → …
```
