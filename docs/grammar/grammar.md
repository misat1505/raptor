# Grammar

This document describes the concrete syntax of Raptor as accepted by the
self-hosted parser under `src/frontend/parser/`.

The formal grammar lives in [`grammar.ebnf`](grammar.ebnf) - that file is the
single source of truth. The production blocks in this document are copied from
it by a script (see [Keeping this document in sync](#keeping-this-document-in-sync)),
so **do not edit the `ebnf` blocks by hand**; edit `grammar.ebnf` instead.
Prose and examples are written by hand.

The executable specification is still the parser itself. Where the parser is
stricter or looser than a rule can express, the rule carries a comment and the
[Parser notes](#parser-notes) section lists it.

Runtime of compiled programs and of the compiler binary depends only on
**libc** (see the root [README](../README.md)).

## Contents

- [Notation](#notation)
- [Program structure](#program-structure)
- [Declarations](#declarations)
- [Statements](#statements)
- [Expressions](#expressions)
- [Lexical part](#lexical-part)
- [Parser notes](#parser-notes)
- [Keeping this document in sync](#keeping-this-document-in-sync)

---

## Notation

| Symbol  | Meaning                    |
| ------- | -------------------------- |
| `=`     | definition                 |
| `,`     | concatenation              |
| `\|`    | alternative                |
| `[ x ]` | optional                   |
| `{ x }` | zero or more repetitions   |
| `( x )` | grouping                   |
| `"x"`   | terminal (literal text)    |
| `? x ?` | informal description       |
| `;`     | end of a rule              |

Whitespace and comments may appear between any two tokens. Rule names are
`lower_snake_case`; every keyword is a quoted terminal.

---

## Program structure

A file is a sequence of top-level items in any order. There is no required
`main` function: statements may appear directly at file level and run in
order.

<!-- ebnf: program top_level_item top_level_statement -->
```ebnf
(* Items may appear in any order; there is no required `main`. *)
program = { top_level_item } ;

top_level_item =
      import_declaration
    | struct_declaration
    | enum_declaration
    | function_declaration
    | extern_function_declaration
    | top_level_statement ;

(* Statements allowed directly at file level (no return/break/continue). *)
top_level_statement =
      assign_or_call
    | if_statement
    | for_statement
    | while_statement
    | switch_statement
    | typed_declaration, ";"
    | let_declaration, ";"
    | match_statement ;
```
<!-- /ebnf -->

`return`, `break` and `continue` are not allowed at file level.

```raptor
import "../utils.rp";

struct Point { i64 x, i64 y };

fn main(): void {
    println("Hello, Raptor");
}

let answer = 42;
println(answer as str);
```

---

## Declarations

### Imports

<!-- ebnf: import_declaration -->
```ebnf
(* The path must be a string literal. *)
import_declaration = "import", string_literal, ";" ;
```
<!-- /ebnf -->

```raptor
import "../utils.rp";
```

### Structs

<!-- ebnf: struct_declaration struct_members struct_member derives_clause -->
```ebnf
(* No trailing comma after the last member. Methods follow the members and
   are NOT separated by commas. Member names must be unique. *)
struct_declaration =
    "struct", identifier, [ derives_clause ],
    "{", [ struct_members ], { method_declaration }, "}", ";" ;

struct_members = struct_member, { ",", struct_member } ;

struct_member  = type, identifier ;

derives_clause = "derives", identifier, { ",", identifier } ;
```
<!-- /ebnf -->

```raptor
struct Hobby {
    u64 id,
    str description
};

struct Person derives Debug, Clone, Json {
    u64 id,
    str name,
    Hobby[] hobbies
};
```

Members are comma-separated with **no trailing comma**. Methods follow the
members and are not separated by commas (see [Methods](#methods)).

### Enums

<!-- ebnf: enum_declaration enum_members enum_member -->
```ebnf
(* Same layout rules as structs. Variant names must be unique. *)
enum_declaration =
    "enum", identifier, [ derives_clause ],
    "{", [ enum_members ], { method_declaration }, "}", ";" ;

enum_members = enum_member, { ",", enum_member } ;

enum_member  = identifier, [ "(", type, ")" ] ;
```
<!-- /ebnf -->

```raptor
enum Task {
    InProgress(Deadline),
    Completed(str),
    Aborted
};
```

A variant carries at most one payload, and its type is written in parentheses.

### Methods

<!-- ebnf: method_declaration method_parameters -->
```ebnf
(* Method of a struct or enum. `self` is an ordinary identifier that is
   treated specially only when it is the first parameter. A method without
   `self` is a "class method" and is called as `Type.method(...)`.
   `self,` must be followed by at least one parameter. *)
method_declaration =
    "fn", identifier, "(", [ method_parameters ], ")", ":", return_type,
    statement_block ;

method_parameters = "self", [ ",", parameters ] | parameters ;
```
<!-- /ebnf -->

Methods are declared inside the body of a `struct` or `enum`, after its
members. A method whose first parameter is `self` is an instance method; one
without `self` is a class method called as `Type.method(...)`.

```raptor
enum Status derives Debug {
    Active,
    OutOfStock

    fn is_active(self): bool {
        if (self matches Status::Active) {
            return true;
        }
        return false;
    }
};

struct Product derives Debug, Clone {
    i64 id,
    i64 price

    fn total(self, &i64 quantity): i64 {
        return self.price * quantity;
    }

    fn new(i64 id, i64 price): Product {
        return Product { id, price };
    }
};

let p = Product.new(1, 10);
println(p.total(&3) as str);
```

### Functions

<!-- ebnf: function_declaration return_type parameters parameter -->
```ebnf
(* Function, struct, enum and extern names share one namespace;
   redeclaration is a parse error. *)
function_declaration =
    "fn", identifier, "(", [ parameters ], ")", ":", return_type,
    statement_block ;

return_type = type | "void" ;

parameters = parameter, { ",", parameter } ;

parameter  = [ "&" ], type, identifier ;
```
<!-- /ebnf -->

`&` in front of a parameter passes it by reference; the caller must write `&`
in front of the argument as well.

```raptor
fn add(i64 a, i64 b): i64 {
    return a + b;
}

fn is_prime(i64 x, &i64 total_iters): bool {
    return true;
}
```

Function, struct, enum and extern names share one namespace; declaring a name
twice is a parse error.

### Extern functions

<!-- ebnf: extern_function_declaration -->
```ebnf
(* `as` gives the C symbol a different name on the Raptor side. *)
extern_function_declaration =
    "extern", "fn", identifier, "(", [ parameters ], ")", ":", return_type,
    [ "as", identifier ], ";" ;
```
<!-- /ebnf -->

```raptor
extern fn InitWindow(i64 x, i64 y, str name): void as init_window;
extern fn PrintValue(i64 value): void as print_value;
```

---

## Statements

### Blocks

<!-- ebnf: statement_block statement -->
```ebnf
(* A block is either braced or a single statement without braces.
   NOTE: with the brace-less form `else` binds to the nearest `if`. *)
statement_block = "{", { statement }, "}" | statement ;

statement =
      assign_or_call
    | if_statement
    | for_statement
    | while_statement
    | switch_statement
    | return_statement
    | break_statement
    | continue_statement
    | typed_declaration, ";"
    | let_declaration, ";"
    | match_statement ;
```
<!-- /ebnf -->

Anywhere a block is expected you may write a single statement instead of
`{ ... }`:

```raptor
if (x > 0) println("positive"); else println("not positive");
```

### Assignments and calls

<!-- ebnf: assign_or_call assign_or_call_core index_or_field method_call -->
```ebnf
assign_or_call = assign_or_call_core, ";" ;

(* Three shapes:
     1. a call of a free function:           foo(1, &x)
     2. an assignment to a variable, element or field:
                                             a[0].b = 1   a.b += 2
     3. a method call, optionally followed by more accessors:
                                             a.b[0].m(1)   Type.make(1).show()
   A free-function call cannot be followed by accessors as a statement
   (`f().g();` is rejected), and a method-call chain cannot be assigned to. *)
assign_or_call_core =
      identifier, "(", [ arguments ], ")"
    | identifier, { index_or_field }, assign_operator, expression
    | identifier, { index_or_field }, method_call, { access_tail } ;

index_or_field = "[", expression, "]" | ".", identifier ;

method_call    = ".", identifier, "(", [ arguments ], ")" ;
```
<!-- /ebnf -->

```raptor
x = 5;
my_fun(5, 2);

x[0][0] = 10;
x[1][0] += 2;
person.name = "Bob";

result = add(10, 20);
repo.products[0].describe();
Product.new(1, 10).show();
```

### Variable declarations

<!-- ebnf: let_declaration -->
```ebnf
let_declaration = "let", identifier, [ ":", type ], "=", expression ;
```
<!-- /ebnf -->

```raptor
let is_valid = true;
let counter = 0;
let result = 10.5;
let message = "Hello, Raptor!";
let colors = ["white", "black"];
let array: str[] = [];
```

The initialiser is mandatory. The type annotation is optional and is needed
where it cannot be inferred, such as for an empty vector.

#### Typed declarations (deprecated)

<!-- ebnf: typed_declaration -->
```ebnf
(* DEPRECATED - will be removed; use `let`.
   Works with built-in types everywhere. With a user-defined type name
   (`Person p = ...;`) it is only accepted in `for` initialisers, because as a
   statement the leading identifier is first tried as assign_or_call. *)
typed_declaration = type, identifier, [ "=", expression ] ;
```
<!-- /ebnf -->

```raptor
bool is_valid = true;
i64 counter = 0;
f64 result = 10.5;
str message = "Hello, Raptor!";
```

> **Deprecated.** Declaring a variable by writing its type first is planned to
> be removed; use `let` in new code. It is documented here because the parser
> still accepts it.

### `if`

<!-- ebnf: if_statement -->
```ebnf
(* `x matches Pattern` form: the block runs when `x` has that variant.
   With `not`, the block runs when it does not, and the pattern must not bind
   a payload. `else` is only available on `if`. *)
if_statement =
      "if", "(", expression, ")", statement_block,
          [ "else", statement_block ]
    | "if", "(", expression, [ "not" ], "matches", match_pattern, ")",
          statement_block, [ "else", statement_block ] ;
```
<!-- /ebnf -->

```raptor
if (x == 5) {
    println("x is five.");
} else {
    println("x is not five.");
}

if (foo matches MyEnum::Variant(boo)) {
    println(boo);
} else {}

if (foo not matches MyEnum::Variant) {
    println("doesn't match");
} else {
    println("matches");
}
```

### `while`

<!-- ebnf: while_statement -->
```ebnf
while_statement =
      "while", "(", expression, ")", statement_block
    | "while", "(", expression, [ "not" ], "matches", match_pattern, ")",
          statement_block ;
```
<!-- /ebnf -->

```raptor
while (x < 5) {
    x += 1;
}

while (foo matches MyEnum::Variant(boo)) {
    println("still matches");
}

while (foo not matches MyEnum::Variant) {
    println("still doesn't match");
}
```

### `for`

<!-- ebnf: for_statement for_init for_step -->
```ebnf
(* The condition is mandatory; init and step are optional. *)
for_statement =
    "for", "(", [ for_init ], ";", expression, ";", [ for_step ], ")",
    statement_block ;

for_init = let_declaration | typed_declaration ;

for_step = assign_or_call_core ;
```
<!-- /ebnf -->

The condition is mandatory. The initialiser and the step may be omitted:

```raptor
for (let i = 0; i < 10; i += 1) {
    println(i as str);
}

let j = 0;
for (; j < 10 ;) {
    j += 1;
}
```

### `match`

<!-- ebnf: match_statement match_body match_arm rest_arm match_pattern -->
```ebnf
(* Arms are separated by commas; no trailing comma. A `rest` arm, if present,
   comes last and is preceded by a comma when there are other arms. *)
match_statement = "match", "(", expression, ")", "{", [ match_body ], "}" ;

match_body =
      rest_arm
    | match_arm, { ",", match_arm }, [ ",", rest_arm ] ;

match_arm = match_pattern, statement_block ;

rest_arm  = "rest", statement_block ;

(* `Enum::Variant` or `Enum::Variant(binding)`; the binding is a fresh name. *)
match_pattern = identifier, "::", identifier, [ "(", identifier, ")" ] ;
```
<!-- /ebnf -->

```raptor
match (task) {
    Task::InProgress(deadline) {
        # deadline: Deadline - available only in this scope
    },
    Task::Aborted {
        # Aborted doesn't contain any data
    },
    rest {
        # catches every variant not listed above
    }
}
```

Without a `rest` arm every variant must be covered (checked by the semantic
checker, not the parser).

### `switch`

<!-- ebnf: switch_statement switch_expression switch_case -->
```ebnf
(* Each `expression [: alias]` names a value; case conditions are arbitrary
   boolean expressions that may use those aliases. Cases are NOT separated. *)
switch_statement =
    "switch", "(", switch_expression, { ",", switch_expression }, ")",
    "{", { switch_case }, "}" ;

switch_expression = expression, [ ":", identifier ] ;

switch_case       = "(", expression, ")", "->", statement_block ;
```
<!-- /ebnf -->

```raptor
switch (x: temp1, y: temp2) {
    (x < 5 && temp2 < 5) -> {
        print("Less than 5.");
    }

    (temp1 < 10 && y < 10) -> {
        print("Less than 10.");
        break;
    }
}
```

### `return`, `break`, `continue`

<!-- ebnf: return_statement break_statement continue_statement -->
```ebnf
return_statement   = "return", [ expression ], ";" ;

break_statement    = "break", ";" ;

continue_statement = "continue", ";" ;
```
<!-- /ebnf -->

```raptor
fn first_non_negative(i64[] values): i64 {
    for (let i = 0; i < 10; i += 1) {
        if (values[i] < 0) continue;
        if (values[i] > 100) break;
        return values[i];
    }
    return -1;
}
```

---

## Expressions

Operator precedence, from lowest to highest binding:

| Level | Operators                       | Notes                            |
| ----: | ------------------------------- | -------------------------------- |
|     1 | `\|\|`                          | left-associative                 |
|     2 | `&&`                            | left-associative                 |
|     3 | `==` `!=` `<` `<=` `>` `>=`     | non-associative                  |
|     4 | `+` `-`                         | left-associative                 |
|     5 | `*` `/` `%`                     | left-associative                 |
|     6 | `as`                            | at most one per operand          |
|     7 | unary `-` `!`                   | one prefix operator, then a factor |

<!-- ebnf: expression logical_and relation additive multiplicative cast unary -->
```ebnf
(* Lowest to highest precedence. All binary operators are left-associative,
   except that relational operators are non-associative (`a < b < c` is an
   error) and `as` may appear at most once per operand.
   (Previously documented as `concatenation_term` = logical_and.) *)
expression = logical_and, { "||", logical_and } ;

logical_and = relation, { "&&", relation } ;

relation = additive, [ relation_operator, additive ] ;

additive = multiplicative, { ( "+" | "-" ), multiplicative } ;

multiplicative = cast, { ( "*" | "/" | "%" ), cast } ;

cast = unary, [ "as", type ] ;

(* A single prefix operator that applies to a factor: `--x` and `!!x` are not
   valid, write `-(-x)` / `!(!x)`. `-2 as str` is `(-2) as str`. *)
unary = [ "-" | "!" ], factor ;
```
<!-- /ebnf -->

```raptor
let a = x == y;
let b = x >= y && !done || fallback;
let c = 1 + (1 + 2) / (2 + 3);
let d = x * 10 % 3;
let e = -(x + 5);
let f = !(x == 5);
```

Because `as` binds tighter than the arithmetic operators and looser than unary
minus, `-2 as str` is `(-2) as str`. To convert twice, parenthesise:
`(x as i64) as f64`.

### Casts

```raptor
let a = (x + add(2, 2)) as f64;

let b = 2 as i64;        # 2
let c = 2 as f64;        # 2.0
let d = 2 as str;        # "2"
let e = -2 as str;       # "-2"
let f = 2 as bool;       # true
let g = 0 as bool;       # false

let h = "123" as i64;    # 123
let i = "fdsfs" as i64;  # runtime error

let j = "" as bool;      # false
let k = "a" as bool;     # true
```

### Primary expressions

<!-- ebnf: factor vector_literal -->
```ebnf
factor =
      literal
    | "(", expression, ")"
    | vector_literal
    | identifier_expression ;

vector_literal = "[", [ expression, { ",", expression } ], "]" ;
```
<!-- /ebnf -->

```raptor
let a = 5;
let b = 2.2;
let c = (2.2 + 3 as f64);
let d = x;
let e = fun(5);
let f = [];
let g = [1, 2, 3];
let h = ["hello", "world"];
let i = [
    [1, 2],
    [3, 4]
];
```

### Names, calls, struct and enum literals

<!-- ebnf: identifier_expression struct_literal_body struct_literal_field enum_literal_tail call_or_access access_tail -->
```ebnf
identifier_expression =
    identifier, ( struct_literal_body | enum_literal_tail | call_or_access ) ;

(* A struct or enum literal cannot be followed by `.`/`[` accessors; bind it
   to a variable first. `{ x }` is shorthand for `{ x: x }`. *)
struct_literal_body =
    "{", [ struct_literal_field, { ",", struct_literal_field } ], "}" ;

struct_literal_field = identifier, [ ":", expression ] ;

(* The payload, if any, is a single expression. *)
enum_literal_tail = "::", identifier, [ "(", expression, ")" ] ;

(* variable, call, indexing, field access, method call (and chains) *)
call_or_access = [ "(", [ arguments ], ")" ], { access_tail } ;

access_tail =
      "[", expression, "]"
    | ".", identifier, [ "(", [ arguments ], ")" ] ;
```
<!-- /ebnf -->

```raptor
let a = x;
let b = fun(5);

let c = x[0][0];
let d = fun(5)[0][0];

let e = Person { id: 123 as u64, name: "Bob" };
let f = Hobby { id: 1, description: "Play football" };
let g = Person { id, name };               # shorthand for { id: id, name: name }

let h = person.hobbies[0].description;
let i = person.name.size();

let j = HttpStatus::NotFound("user not found");
let k = HttpStatus::Ok;
```

### Arguments

<!-- ebnf: arguments argument -->
```ebnf
(* `&expr` passes by reference and must match a `&T` parameter. *)
arguments = argument, { ",", argument } ;

argument  = [ "&" ], expression ;
```
<!-- /ebnf -->

```raptor
calculate(a + 2, &b, c);
```

### Operators

<!-- ebnf: relation_operator assign_operator -->
```ebnf
relation_operator = "==" | "!=" | "<" | "<=" | ">" | ">=" ;

assign_operator   = "=" | "+=" | "-=" | "*=" | "/=" | "%=" ;
```
<!-- /ebnf -->

---

## Lexical part

### Types

<!-- ebnf: type base_type -->
```ebnf
(* `void` is only valid as a return type. Any identifier that is not a
   built-in type name is treated as a user-defined struct/enum type. *)
type = base_type, { "[", "]" } ;

base_type =
      "i8" | "i16" | "i32" | "i64"
    | "u8" | "u16" | "u32" | "u64"
    | "f64" | "bool" | "char" | "str"
    | identifier ;
```
<!-- /ebnf -->

Any identifier that is not a built-in type name is a user-defined struct or
enum type. `T[]` is a vector of `T`; vectors nest (`i64[][]`). `void` is only
valid as a return type.

### Identifiers and keywords

<!-- ebnf: identifier -->
```ebnf
(* ASCII only; must start with a letter (not `_`); must not be a keyword.
   Keywords: fn for while if else return switch break continue import extern
   let struct enum match rest derives matches not as true false void
   and the type names above. Length is limited by the lexer
   (`max_identifier_length`). *)
identifier = letter, { letter | digit | "_" } ;
```
<!-- /ebnf -->

```text
super_variable_123
counter
result_value
```

Reserved words:

```text
fn  for  while  if  else  return  switch  break  continue  import  extern
let  struct  enum  match  rest  derives  matches  not  as  true  false  void
i8  i16  i32  i64  u8  u16  u32  u64  f64  bool  char  str
```

`self` is not reserved: it is an ordinary identifier that is treated specially
only as the first parameter of a method.

### Literals

<!-- ebnf: literal integer_literal float_literal boolean_literal string_literal string_character char_literal char_character escape_sequence -->
```ebnf
literal =
      integer_literal | float_literal | boolean_literal
    | string_literal | char_literal ;

(* No sign: negative numbers are unary minus applied to a literal.
   A single quote may separate digits: 1'000'000. *)
integer_literal = digit, { [ "'" ], digit } ;

float_literal   = integer_literal, ".", { digit } ;

boolean_literal = "true" | "false" ;

(* Strings cannot contain a raw newline. Unknown escapes are kept verbatim
   and produce a lexer warning. *)
string_literal = '"', { string_character | escape_sequence }, '"' ;

string_character = ? any character except '"', "\" and newline ? ;

(* Exactly one character or escape. `'\''` is not supported. *)
char_literal = "'", ( char_character | escape_sequence ), "'" ;

char_character = ? any character except "'", "\" and newline ? ;

escape_sequence = "\", ( "n" | "r" | "t" | "e" | "0" | '"' | "\" ) ;
```
<!-- /ebnf -->

```raptor
let a = 1;
let b = 1'000'000;
let c = 0.00001;
let d = true;
let e = "line one\nline two";
let f = 'x';
let g = '\n';
```

Negative numbers are the unary minus operator applied to a literal, not part of
the literal.

### Comments

<!-- ebnf: comment -->
```ebnf
(* Line comment only; runs to the end of the line. *)
comment = "#", { ? any character except newline ? } ;
```
<!-- /ebnf -->

```raptor
# a comment
let x = 1; # trailing comment
```

### Character classes

<!-- ebnf: letter digit -->
```ebnf
letter = "a" | "b" | "c" | "d" | "e" | "f" | "g" | "h" | "i" | "j" | "k"
       | "l" | "m" | "n" | "o" | "p" | "q" | "r" | "s" | "t" | "u" | "v"
       | "w" | "x" | "y" | "z"
       | "A" | "B" | "C" | "D" | "E" | "F" | "G" | "H" | "I" | "J" | "K"
       | "L" | "M" | "N" | "O" | "P" | "Q" | "R" | "S" | "T" | "U" | "V"
       | "W" | "X" | "Y" | "Z" ;

digit = "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" ;
```
<!-- /ebnf -->

---

## Parser notes

Behaviours that follow from how the parser is written rather than from the
grammar rules alone.

- **No trailing commas** in struct/enum members, `match` arms, parameters,
  arguments, struct-literal fields or vector literals.
- **Separators differ by construct.** Struct/enum members, `match` arms,
  parameters and arguments are comma-separated; `switch` cases and methods are
  not separated at all; struct and enum declarations end with `;`, function
  declarations do not.
- **Struct and enum literals end an expression.** `Point { x: 1 }.show()` and
  `E::A(1).show()` are not accepted; bind to a variable first.
- **Method calls on literals are not accepted**, e.g. `[1, 2, 3].size()`.
- **Statement-level calls.** `f().g();` is rejected (a free-function call cannot
  be followed by accessors as a statement), although `a.g().h();` is fine.
  A method-call chain cannot be assigned to (`a.g() = 1;`).
- **One prefix operator, one `as`.** `--x`, `!!x` and `x as i64 as f64` are
  errors; add parentheses.
- **Relational operators do not chain.** `a < b < c` is an error.
- **Typed declarations with a user type** (`Person p = ...;`) are not accepted
  as statements because the parser first tries the line as an assignment or
  call; use `let p: Person = ...;`. (Typed declarations are deprecated anyway.)
- **Brace-less blocks.** `if (a) if (b) x(); else y();` attaches `else` to the
  nearest `if`.
- **`if ... not matches`** cannot bind the payload (`not matches E::A(v)` is an
  error), since in the negated branch the variant is known not to match.
- **Name collisions** among functions, extern functions (by alias if given),
  structs and enums, and duplicate member or method names, are reported by the
  parser. Collisions with standard-library names are reported later by the
  semantic checker.

---

## Keeping this document in sync

`grammar.ebnf` is the only place where production rules are edited. Every
block like

```text
<!-- ebnf: RULE_NAMES -->
...generated...
<!-- /ebnf -->
```

in this file is regenerated from it. After changing the grammar run:

```bash
python3 docs/sync_grammar_md.py          # rewrite the blocks in grammar.md
python3 docs/sync_grammar_md.py --check  # CI: fail if grammar.md is out of date
```

The script also fails if a rule in `grammar.ebnf` is not shown anywhere in this
document, or if a block names a rule that does not exist.
