## Grammar

### Syntax Part

**program** = { function_declaration | extern_function_declaration | assign_or_call | if_statement | for_statement | while_statement | switch_statement | declaration, ";" };

**comment** = "#" , {unicode_character - "\n"}, "\n";

**extern_function_declaration** = "extern", "fn", identifier, "(", parameters, ")", ":", type | "void", [ "as", identifier ] ";";

```
extern fn InitWindow(i64 x, i64 y, str name): void as init_window;
```

**function_declaration** = “fn”, identifier, "(", parameters, ")", “:”, type | “void”, statement_block;

```
fn is_prime(i64 x, &i64 total_iters): bool {
    return true;
}
```

**parameters** = [ parameter, { ",", parameter } ];

**parameter** = [“&”], type, identifier;

**statement_block** = ("{", {statement}, "}") | statement;

**statement** = assign_or_call | if_statement | for_statement | while_statement | switch_statement | declaration, ";" | return_statement | break_statement | continue_statement;

**assign_or_call_without_semicolon** = identifier, ( { "[", expression, "]" }, ("=" | "+=" | "-=" | "*=" | "/=" | "%="), expression | "(", arguments, ")");

**assign_or_call** = assign_or_call_without_semicolon, ";";

```
x = 5;
my_fun(5, 2);
x[0][0] = 10;
x[1][0] += 2;
```


**declaration** = type, identifier, [ "=", expression ];

```
bool is_valid = true;
```

**if_statement** = "if", "(", expression, ")", statement_block, [ "else", statement_block ];

```
if (x == 5) {} else {}
```

**for_statement** = "for", "(", [ declaration ], “;”, expression, “;”, [ assign_or_call_without_semicolon ], ")", statement_block;

```
for (i64 i = 0; i < 10; i = i + 1) {}
```

**while_statement** = "while", "(", expression, ")", statement_block

```
while (x < 5) {
  x += 1;
}
```

```
i64 i = 0
for (; i < 10 ;) {
    i = i + 1;
}
```

**break_statement** = "break", ";";

```
break;
```

**continue_statement** = "continue", ";";

```
continue;
```

**return_statement** = "return", [ expression ], ";";

```
return a + 2 * b;
```

**argument** = [“&”], expression;

**arguments** = [ argument, {",", argument} ];

```
a + 2, &b, c
```

**expression** = concatenation_term { “||”, concatenation_term };

```
a == b && b || c
```

**concatenation_term** = relation_term, { “&&”, relation_term };

```
a == b && b
```

**relation_term** = additive_term, [ relation_operands, additive_term ];

```
x == y
```

**additive_term** = multiplicative_term , { ("+" | "-"), multiplicative_term };

```
1 + (1 + 2) / (2 + 3)
```

**multiplicative_term** = casted_term, { ("\*" | "/" | "%"), casted_term };

```
(1 + 2) / (2 + 3)
```

**casted_term** = unary_term, [ “as”, type ];

```
(x + add(2, 2)) as f64
2 as i64                # 2
2 as f64                # 2.0
2 as str                # “2”
-2 as str               # "-2"
2 as bool               # true
0 as bool               # false
“123” as i64            # 123
“fdsfs” as i64          # error
“” as bool              # false
“a” as bool             # true
```

**unary_term** = [ ("-", "!") ], factor;

```
-2
-(x + 5)
!true
```

**factor** = literal | ( "(", expression, ")" ) | identifier_or_call | vector_literal;

```
5
(2.2 + 3 as f64)
x
fun(5)
```

**vector_literal** = "[", [ expression, { ",", expression } ], "]";

```
[]
[1, 2, 3]
["hello", "world"]
[[1,2], [3,4]]
```

**identifier_or_call** = identifier, [ "(", arguments, ")" ], { "[", expression, "]" };

```
x
fun(5)
x[0][0]
fun(5)[0][0]
```

**literal** = integer_literal | float_literal | boolean_literal | string_literal;

**identifier** = letter, {character};

```
super_variable_123
```

**switch_statement** = "switch", "(", switch_expressions, ")", "{", {switch_case}, "}";

**switch_expression** = expression, [ ":", identifier ];

**switch_expressions** = switch_expression, { “,”, switch_expression };

**switch_case** = "(", expression, ")", "->", statement_block;

```
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

### Lexical Part

**letter** = "a" - "z" | "A" - "Z";

**type** = (“i64“| “f64” | “bool” | “str”), { "[]" };

**relation_operands** = "==" | "<" | "<=" | ">" | ">=" | "!=";

**digit** = "0" - “9”;

**non_zero_digit** = "1" - "9";

**integer_literal** = ( non_zero_digit, {digit} ) | “0”;

```
1, 12, 10, 0
```

**float_literal** = integer_literal, ".", {digit}

```
1.0, 1.2, 10.0, 0.0, 0.00001;
```

**string_literal** = “\””, {unicode_character - “\””}, “\””;

**boolean_literal** = “true” | “false”;

**character** = "a" - "z" | "A" - "Z" | "0" - "9" | "\_";

**unicode_character** = (all unicode characters)

## Operator priority

<table>
  <tr>
   <td>operator
   </td>
   <td>priority
   </td>
  </tr>
  <tr>
   <td>- (number negetion)
   </td>
   <td>7
   </td>
  </tr>
  <tr>
   <td>!
   </td>
   <td>7
   </td>
  </tr>
  <tr>
   <td>as
   </td>
   <td>6
   </td>
  </tr>
  <tr>
   <td>*
   </td>
   <td>5
   </td>
  </tr>
  <tr>
   <td>/
   </td>
   <td>5
   </td>
  </tr>
   <tr>
   <td>%
   </td>
   <td>5
   </td>
  </tr>
  <tr>
   <td>+
   </td>
   <td>4
   </td>
  </tr>
  <tr>
   <td>- (subtraction)
   </td>
   <td>4
   </td>
  </tr>
  <tr>
   <td>>
   </td>
   <td>3
   </td>
  </tr>
  <tr>
   <td>>=
   </td>
   <td>3
   </td>
  </tr>
  <tr>
   <td><
   </td>
   <td>3
   </td>
  </tr>
  <tr>
   <td><=
   </td>
   <td>3
   </td>
  </tr>
  <tr>
   <td>==
   </td>
   <td>3
   </td>
  </tr>
  <tr>
   <td>!=
   </td>
   <td>3
   </td>
  </tr>
  <tr>
   <td>&&
   </td>
   <td>2
   </td>
  </tr>
  <tr>
   <td>||
   </td>
   <td>1
   </td>
  </tr>
</table>