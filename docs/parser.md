# Parser

The parser consumes lexer tokens and builds the Raptor abstract syntax tree (AST).

## Responsibilities

The parser:

- validates token sequences against the grammar;
- constructs the AST;
- parses declarations, statements, functions, and expressions;
- enforces expression precedence;
- reports syntax errors;
- rejects invalid top-level or statement structure.

## Functions

Functions have a typed return value or `void`:

```text
fn clamp(i64 value, i64 minimum, i64 maximum): i64 {
    if (value < minimum) {
        return minimum;
    }

    if (value > maximum) {
        return maximum;
    }

    return value;
}
```

Parameters may be passed by value or by reference:

```text
fn increment(&i64 value): void {
    value = value + 1;
}
```

## Statements

Raptor supports declarations, assignments, function calls, conditionals, loops,
switch statements, and control-transfer statements.

### Conditional

```text
if (score >= 60) {
    print("passed");
} else {
    print("failed");
}
```

### Loop

```text
for (i64 i = 0; i < 10; i = i + 1) {
    print(i as str);
}
```

### Switch

```text
switch (score: current) {
    (current >= 90) -> {
        print("excellent");
    }

    (current >= 60) -> {
        print("passed");
    }

    (current < 60) -> {
        print("failed");
    }
}
```

## Expressions

The documented operator precedence, from highest to lowest, is:

| Priority | Operators |
| ---: | --- |
| 7 | unary `-`, `!` |
| 6 | `as` |
| 5 | `*`, `/`, `%` |
| 4 | `+`, `-` |
| 3 | `>`, `>=`, `<`, `<=`, `==`, `!=` |
| 2 | `&&` |
| 1 | `||` |

For example:

```text
total = base + bonus * multiplier;
```

is grouped according to precedence rather than simply left-to-right evaluation.

Parentheses can be used when explicit grouping is desired:

```text
average = total / (count + 1);
```

## Vectors

Vector literals can be nested:

```text
i64[] values = [10, 20, 30];

i64[][] matrix = [
    [1, 2, 3],
    [4, 5, 6]
];
```

Indexing can also be multidimensional:

```text
matrix[1][2] = 42;
```

Types such as `i64[][]` are therefore valid Raptor types.

## Errors

Parser errors are syntax errors: the token stream does not match the expected grammar.

Raptor uses compiler-style diagnostics:

```text
error: Unexpected token
  --> basic.rp:47:4
  expected: (
  found:    ;
```

The parser can therefore show both the expected token and the token that was actually
found.

A parser error prevents a valid AST from being produced, so later stages do not run.

## Parser versus semantic checker

The distinction is intentional:

- **Parser:** Is the token sequence syntactically valid?
- **Semantic checker:** Is the resulting program statically valid?

For example, a syntactically valid assignment can still fail semantic type checking:

```text
i64 foo = 10;
foo = "hello";
```

The parser can construct this statement successfully; the semantic checker rejects it.
