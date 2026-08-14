# Lexer

The lexer is the first stage of the Raptor compiler pipeline. It converts the source
character stream into tokens consumed by the parser.

## Responsibilities

The lexer:

- reads the source lazily through `LazyStreamReader`;
- recognizes identifiers, literals, keywords, operators, and punctuation;
- tracks source locations;
- detects malformed lexical input;
- reports lexical errors and warnings.

## Input stream

The lexer works with `LazyStreamReader`, which provides:

- `current()` — inspect the current character;
- `next()` — consume the next character;
- `position()` — obtain the current source position.

Token generation is lazy: the lexer reads only as much input as is needed to produce
the next token.

## Lexical forms

Raptor supports identifiers, integer and floating-point literals, strings, booleans,
comments, operators, punctuation, and type names.

Examples:

```text
counter
total_2
42
3.1415
"hello, Raptor"
true
false
```

Comments begin with `#` and continue to the end of the line:

```text
# calculate the next value
i64 next = current + 1;
```

The primitive type names are:

```text
i64
f64
str
bool
```

Vector types extend these types with one or more `[]` suffixes:

```text
i64[]
i64[][]
f64[][][]
```

## Lexer configuration

The current command-line entry point configures:

```text
max_comment_length    = 100
max_identifier_length = 20
```

## Errors

Lexer errors mean that the source cannot be tokenized correctly.

The diagnostic format is:

```text
error: <message>
  --> <file>:<line>:<column>
```

For example:

```text
error: Overflow occurred while parsing integer
  --> basic.rp:47:20
```

Numeric overflow is detected while processing an integer literal.

Other lexical errors can occur when the input cannot be represented by any valid token
or when configured lexical limits are exceeded.

## Warnings

The lexer may also emit warnings when the input is close enough to a recognized form
that the likely intention can be identified.

Warnings do not necessarily stop processing. They are forwarded to the higher-level
diagnostic callback.

## Boundary with the parser

The lexer answers:

> Which token does this sequence of characters represent?

The parser answers:

> Does this sequence of tokens form a valid Raptor program?

Keeping these responsibilities separate makes lexical and syntax diagnostics easier
to understand.
