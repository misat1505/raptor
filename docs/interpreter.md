# Interpreter

The interpreter executes the AST directly.

It implements the visitor pattern and maintains the runtime state required for
expressions, variables, functions, scopes, and control flow.

## Responsibilities

The interpreter handles:

- expression evaluation;
- variable declarations and assignments;
- function calls and recursion;
- control flow;
- type conversions;
- built-in functions;
- runtime error reporting.

## Runtime values

The language provides:

```text
i64
f64
str
bool
void
```

It also supports vectors, including multidimensional vectors:

```text
i64[]
i64[][]
i64[][][]
```

Variables are mutable and are visible only in their applicable scopes.

## Vectors

Vector literals may be nested:

```text
i64[][] matrix = [
    [1, 2, 3],
    [4, 5, 6]
];
```

Elements can be accessed with repeated indexing:

```text
matrix[0][2]
```

Passing a vector by value performs a **shallow copy**. The outer vector structure is
copied, but nested vector data is not recursively deep-copied.

## Runtime state

The interpreter maintains:

- `last_result` — the most recent intermediate computation result;
- `last_arguments` — arguments associated with a function call;
- `is_breaking` — pending `break` state;
- `is_returning` — pending `return` state.

These values allow individual visitor operations to communicate control-flow and
evaluation results.

## Stack and scopes

Function calls are represented by a `Stack`.

Each `StackFrame` contains a `ScopeManager`, which manages nested scopes.

Conceptually:

```text
Interpreter
    │
    ▼
  Stack
    │
    ▼
StackFrame
    │
    ▼
ScopeManager
    │
    ├── Scope
    ├── Scope
    └── ...
```

Each scope maps variable names to value pointers.

The `Value` abstraction represents runtime values, while the ALU performs operations
on those values.

## Functions

Functions support value and reference parameters and may return a typed result.

Example:

```text
fn sum(i64[] values): i64 {
    i64 total = 0;

    for (i64 i = 0; i < 3; i = i + 1) {
        total = total + values[i];
    }

    return total;
}
```

Recursive functions are supported as well:

```text
fn factorial(i64 n): i64 {
    if (n <= 1) {
        return 1;
    }

    return n * factorial(n - 1);
}
```

## Control flow

Raptor supports:

- `if` / `else`;
- `for`;
- `while`;
- `switch`;
- `break`;
- `continue`;
- `return`.

## Built-in functions

The documented built-ins are:

```text
print(text)
input(text)
mod(a, b)
```

## Errors

Interpreter errors are runtime errors that occur while executing the AST.

For example, an invalid assignment can produce:

```text
error: Cannot assign 'str' to variable 'foo' which was previously declared as 'i64'.
  --> basic.rp:4:1
```

The interpreter can report errors for operations such as:

- incompatible value assignments;
- incompatible function arguments;
- invalid return values;
- variable redeclaration;
- non-boolean conditions;
- invalid `break` or `return` placement;
- stack overflow;
- arithmetic overflow;
- invalid type conversions.

For example:

```text
error: Cannot cast String 'abc' to i64.
  --> basic.rp:18:9
```

## Interpreter versus semantic checker

The semantic checker performs static type checking before interpretation.

The interpreter still validates runtime operations because some failures depend on
runtime values or execution state.

The same kind of diagnostic can therefore appear in both stages when the operation is
invalid, but the semantic checker is the normal first line of defense.
