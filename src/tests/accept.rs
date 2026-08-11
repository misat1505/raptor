#[cfg(test)]
mod tests {
    use std::{
        cell::RefCell,
        io::{BufReader, Read},
        rc::Rc,
    };

    use crate::{
        ast::{Program, Type},
        errors::IError,
        interpreter::Interpreter,
        lazy_stream_reader::LazyStreamReader,
        lexer::{Lexer, LexerOptions},
        parser::{IParser, Parser},
        semantic_checker::SemanticChecker,
        value::Value,
    };

    fn on_warning(_err: Box<dyn IError>) {}

    fn setup_program(text: BufReader<&[u8]>) -> Program {
        let mut text = text;
        let mut content = String::new();
        text.read_to_string(&mut content).unwrap();

        let owned_text: &'static str = Box::leak(content.into_boxed_str());
        let code = BufReader::new(owned_text.as_bytes());
        let reader = LazyStreamReader::new(code, None);

        let lexer_options = LexerOptions {
            max_comment_length: 100,
            max_identifier_length: 20,
        };
        let lexer = Lexer::new(reader, lexer_options, on_warning).unwrap();
        let mut parser = Parser::new(lexer);
        let program = parser.parse().unwrap();
        let mut checker = SemanticChecker::new(&program).unwrap();
        checker.check();
        assert_eq!(checker.errors.len(), 0);
        program
    }

    fn create_interpreter<'a>(program: &'a Program) -> Interpreter<'a> {
        Interpreter::new(program)
    }

    #[test]
    fn if_statement() {
        let text = BufReader::new(
            r#"
    i64 x = 2;
    i64 y = 2;
    str text;
    if (x == y) {
        text = "equal";
    } else {
        text = "not equal";
    }
    "#
            .as_bytes(),
        );

        let program = setup_program(text);
        let mut interpreter = create_interpreter(&program);
        interpreter.interpret().unwrap();
        assert_eq!(
            interpreter.stack().get_variable("text").unwrap().clone(),
            Rc::new(RefCell::new(Value::String(String::from("equal"))))
        );
    }

    #[test]
    fn loop_with_break() {
        let text = BufReader::new(
            r#"
    i64 i = 0;
    for (; i < 5; i = i + 1) {
      if (i == 2) {
        break;
      }
    }
    "#
            .as_bytes(),
        );

        let program = setup_program(text);
        let mut interpreter = create_interpreter(&program);
        interpreter.interpret().unwrap();
        assert_eq!(
            interpreter.stack().get_variable("i").unwrap().clone(),
            Rc::new(RefCell::new(Value::I64(2)))
        );
    }

    #[test]
    fn functions() {
        let text = BufReader::new(
            r#"
    fn add(i64 a, i64 b): i64 {
      return a + b;
    }

    i64 a = add(1, 2);
    "#
            .as_bytes(),
        );

        let program = setup_program(text);
        let mut interpreter = create_interpreter(&program);
        interpreter.interpret().unwrap();
        assert_eq!(
            interpreter.stack().get_variable("a").unwrap().clone(),
            Rc::new(RefCell::new(Value::I64(3)))
        );
    }

    #[test]
    fn reference() {
        let text = BufReader::new(
            r#"
    fn foo(&i64 x): void {
      x = x + 1;
    }

    i64 x = 2;
    foo(&x);
    "#
            .as_bytes(),
        );

        let program = setup_program(text);
        let mut interpreter = create_interpreter(&program);
        interpreter.interpret().unwrap();
        assert_eq!(
            interpreter.stack().get_variable("x").unwrap().clone(),
            Rc::new(RefCell::new(Value::I64(3)))
        );
    }

    #[test]
    fn recursion() {
        let text = BufReader::new(
            r#"
    fn fib(i64 x): i64 {
      if (x == 1 || x == 2) {
        return 1;
      }

      return fib(x - 1) + fib(x - 2);
    }

    i64 x = fib(6);
    "#
            .as_bytes(),
        );

        let program = setup_program(text);
        let mut interpreter = create_interpreter(&program);
        interpreter.interpret().unwrap();
        assert_eq!(
            interpreter.stack().get_variable("x").unwrap().clone(),
            Rc::new(RefCell::new(Value::I64(8)))
        );
    }

    #[test]
    fn is_prime() {
        let text = BufReader::new(
            r#"
    fn is_prime(i64 x): bool {
      if (x < 2) {
        return false;
      }

      for (i64 i = 2; i < x / 2; i = i + 1) {
        if (x % i == 0) {
          return false;
        }
      }

      return true;
    }

    bool is_5 = is_prime(5);
    bool is_6 = is_prime(6);
    "#
            .as_bytes(),
        );

        let program = setup_program(text);
        let mut interpreter = create_interpreter(&program);
        interpreter.interpret().unwrap();
        assert_eq!(
            interpreter.stack().get_variable("is_5").unwrap().clone(),
            Rc::new(RefCell::new(Value::Bool(true)))
        );
        assert_eq!(
            interpreter.stack().get_variable("is_6").unwrap().clone(),
            Rc::new(RefCell::new(Value::Bool(false)))
        );
    }

    #[test]
    fn pattern_matching() {
        let text = BufReader::new(
            r#"
    str text;
    i64 x = 10;
    switch (x) {
      (x > 0) -> {
        text = ">0";
      }
      (x > 1) -> {
        text = ">1";
        break;
      }
      (x > 2) -> {
        text = ">2";
      }
    }
    "#
            .as_bytes(),
        );

        let program = setup_program(text);
        let mut interpreter = create_interpreter(&program);
        interpreter.interpret().unwrap();
        assert_eq!(
            interpreter.stack().get_variable("text").unwrap().clone(),
            Rc::new(RefCell::new(Value::String(String::from(">1"))))
        );
    }

    #[test]
    fn pass_vector_by_value() {
        let text = BufReader::new(
            r#"
    fn pass_by_change_outer(i64[][] arr): void {
        arr[0] = [42];
    }

    i64[][] arr = [[1], [2]];
    pass_by_change_outer(arr);

    fn pass_by_change_inner(i64[][] arr): void {
        arr[1][0] = 69;
    }
    pass_by_change_inner(arr);
    "#
            .as_bytes(),
        );

        let program = setup_program(text);
        let mut interpreter = create_interpreter(&program);
        interpreter.interpret().unwrap();
        let expected = Value::Vector {
            kind: Box::new(Type::Vector(Box::new(Type::Vector(Box::new(Type::I64))))),
            values: Rc::new(RefCell::new(vec![
                Rc::new(RefCell::new(Value::Vector {
                    kind: Box::new(Type::Vector(Box::new(Type::I64))),
                    values: Rc::new(RefCell::new(vec![Rc::new(RefCell::new(Value::I64(1)))])),
                })),
                Rc::new(RefCell::new(Value::Vector {
                    kind: Box::new(Type::Vector(Box::new(Type::I64))),
                    values: Rc::new(RefCell::new(vec![Rc::new(RefCell::new(Value::I64(69)))])),
                })),
            ])),
        };

        assert_eq!(interpreter.stack().get_variable("arr").unwrap().clone(), Rc::new(RefCell::new(expected)));
    }

    #[test]
    fn game_of_life() {
        let text = BufReader::new(
            r##"
fn next_state(&str[][] board): str[][] {
  str[][] next_board = [];

  for (i64 i = 0; i < vector_size(&board); i += 1) {
    str[] row = board[i];
    str[] next_state_row = [];
    for (i64 j = 0; j < vector_size(&row); j += 1) {
      str current_cell = board[i][j];

      i64 alive_neighbours = 0;
      i64 dead_neighbours = 0;

      for (i64 dx = -1; dx <= 1; dx += 1) {
        for (i64 dy = -1; dy <= 1; dy += 1) {
          i64 x = j + dx;
          i64 y = i + dy;
          bool is_x_in_bounds = x >= 0 && x < vector_size(&row);
          bool is_y_in_bounds = y >= 0 && y < vector_size(&board);
          bool is_current_cell = (dx == 0 && dy == 0);
          bool is_valid_neighbour = is_x_in_bounds && is_y_in_bounds && !is_current_cell;
          if (is_valid_neighbour) {
            if (board[y][x] == "#") alive_neighbours += 1;
            else dead_neighbours += 1;
            
          }
        }
      }

      if (current_cell == ".") {
        if (alive_neighbours == 3) vector_push(&next_state_row, "#");
        else vector_push(&next_state_row, ".");
      } else if (current_cell == "#") {
        if (alive_neighbours == 2 || alive_neighbours == 3) vector_push(&next_state_row, "#");
        else vector_push(&next_state_row, ".");
      }
    }

    vector_push(&next_board, next_state_row);
  }

  return next_board;
}

str[][] board = [
  [".", ".", ".", ".", "."],
  [".", ".", "#", ".", "."],
  [".", ".", "#", ".", "."],
  [".", ".", "#", ".", "."],
  [".", ".", ".", ".", "."]
];

board = next_state(&board);
    "##
            .as_bytes(),
        );

        let program = setup_program(text);
        let mut interpreter = create_interpreter(&program);
        interpreter.interpret().unwrap();

        fn str_row(cells: [&str; 5]) -> Rc<RefCell<Value>> {
            Rc::new(RefCell::new(Value::Vector {
                kind: Box::new(Type::Vector(Box::new(Type::Str))),
                values: Rc::new(RefCell::new(
                    cells.iter().map(|c| Rc::new(RefCell::new(Value::String(c.to_string())))).collect(),
                )),
            }))
        }

        let expected = Value::Vector {
            kind: Box::new(Type::Vector(Box::new(Type::Vector(Box::new(Type::Str))))),
            values: Rc::new(RefCell::new(vec![
                str_row([".", ".", ".", ".", "."]),
                str_row([".", ".", ".", ".", "."]),
                str_row([".", "#", "#", "#", "."]),
                str_row([".", ".", ".", ".", "."]),
                str_row([".", ".", ".", ".", "."]),
            ])),
        };

        assert_eq!(
            interpreter.stack().get_variable("board").unwrap().clone(),
            Rc::new(RefCell::new(expected))
        );
    }

    #[test]
    fn casting() {
        let text = BufReader::new(
            r#"
    i64 a = 5;
    f64 b = a as f64;
    str c = a as str;
    bool d = a as bool;
    bool e = 0 as bool;
    i64 f = "123" as i64;
    "#
            .as_bytes(),
        );

        let program = setup_program(text);
        let mut interpreter = create_interpreter(&program);
        interpreter.interpret().unwrap();

        assert_eq!(
            interpreter.stack().get_variable("b").unwrap().clone(),
            Rc::new(RefCell::new(Value::F64(5.0)))
        );
        assert_eq!(
            interpreter.stack().get_variable("c").unwrap().clone(),
            Rc::new(RefCell::new(Value::String(String::from("5"))))
        );
        assert_eq!(
            interpreter.stack().get_variable("d").unwrap().clone(),
            Rc::new(RefCell::new(Value::Bool(true)))
        );
        assert_eq!(
            interpreter.stack().get_variable("e").unwrap().clone(),
            Rc::new(RefCell::new(Value::Bool(false)))
        );
        assert_eq!(
            interpreter.stack().get_variable("f").unwrap().clone(),
            Rc::new(RefCell::new(Value::I64(123)))
        );
    }

    #[test]
    fn while_loop() {
        let text = BufReader::new(
            r#"
    i64 i = 0;
    i64 total = 0;
    while (i < 5) {
      total += i;
      i += 1;
    }
    "#
            .as_bytes(),
        );

        let program = setup_program(text);
        let mut interpreter = create_interpreter(&program);
        interpreter.interpret().unwrap();

        assert_eq!(
            interpreter.stack().get_variable("i").unwrap().clone(),
            Rc::new(RefCell::new(Value::I64(5)))
        );
        assert_eq!(
            interpreter.stack().get_variable("total").unwrap().clone(),
            Rc::new(RefCell::new(Value::I64(10)))
        );
    }

    #[test]
    fn continue_in_for_loop() {
        let text = BufReader::new(
            r#"
    i64 total = 0;
    for (i64 i = 0; i < 5; i += 1) {
      if (i % 2 == 0) continue;
      total = total + i;
    }
    "#
            .as_bytes(),
        );

        let program = setup_program(text);
        let mut interpreter = create_interpreter(&program);
        interpreter.interpret().unwrap();

        // 1 + 3 = 4 (pomija liczby parzyste: 0, 2, 4)
        assert_eq!(
            interpreter.stack().get_variable("total").unwrap().clone(),
            Rc::new(RefCell::new(Value::I64(4)))
        );
    }

    #[test]
    fn continue_in_while_loop() {
        let text = BufReader::new(
            r#"
    i64 i = 0;
    i64 total = 0;
    while (i < 5) {
      i = i + 1;
      if (i % 2 == 0) {
        continue;
      }
      total = total + i;
    }
    "#
            .as_bytes(),
        );

        let program = setup_program(text);
        let mut interpreter = create_interpreter(&program);
        interpreter.interpret().unwrap();

        assert_eq!(
            interpreter.stack().get_variable("total").unwrap().clone(),
            Rc::new(RefCell::new(Value::I64(9)))
        );
    }

    #[test]
    fn string_concatenation() {
        let text = BufReader::new(
            r#"
    str a = "Hello, ";
    str b = "world!";
    str c = a + b;
    "#
            .as_bytes(),
        );

        let program = setup_program(text);
        let mut interpreter = create_interpreter(&program);
        interpreter.interpret().unwrap();

        assert_eq!(
            interpreter.stack().get_variable("c").unwrap().clone(),
            Rc::new(RefCell::new(Value::String(String::from("Hello, world!"))))
        );
    }

    #[test]
    fn modulo_operator() {
        let text = BufReader::new(
            r#"
    i64 a = 10 % 3;
    i64 b = 10 % 3;
    i64 c = 10;
    c %= 3;
    "#
            .as_bytes(),
        );

        let program = setup_program(text);
        let mut interpreter = create_interpreter(&program);
        interpreter.interpret().unwrap();

        assert_eq!(
            interpreter.stack().get_variable("a").unwrap().clone(),
            Rc::new(RefCell::new(Value::I64(1)))
        );
        assert_eq!(
            interpreter.stack().get_variable("b").unwrap().clone(),
            Rc::new(RefCell::new(Value::I64(1)))
        );
        assert_eq!(
            interpreter.stack().get_variable("c").unwrap().clone(),
            Rc::new(RefCell::new(Value::I64(1)))
        );
    }

    #[test]
    fn operator_precedence() {
        let text = BufReader::new(
            r#"
    i64 a = 2 + 3 * 4;
    i64 b = (2 + 3) * 4;
    bool c = 2 + 2 == 4 && 1 < 2;
    bool d = !false || false;
    i64 e = 10 - 2 - 3;
    "#
            .as_bytes(),
        );

        let program = setup_program(text);
        let mut interpreter = create_interpreter(&program);
        interpreter.interpret().unwrap();

        assert_eq!(
            interpreter.stack().get_variable("a").unwrap().clone(),
            Rc::new(RefCell::new(Value::I64(14)))
        );
        assert_eq!(
            interpreter.stack().get_variable("b").unwrap().clone(),
            Rc::new(RefCell::new(Value::I64(20)))
        );
        assert_eq!(
            interpreter.stack().get_variable("c").unwrap().clone(),
            Rc::new(RefCell::new(Value::Bool(true)))
        );
        assert_eq!(
            interpreter.stack().get_variable("d").unwrap().clone(),
            Rc::new(RefCell::new(Value::Bool(true)))
        );
        // lewostronna łączność: (10 - 2) - 3 = 5, a nie 10 - (2 - 3) = 11
        assert_eq!(
            interpreter.stack().get_variable("e").unwrap().clone(),
            Rc::new(RefCell::new(Value::I64(5)))
        );
    }

    #[test]
    fn switch_with_alias() {
        let text = BufReader::new(
            r#"
    str result;
    switch (5 + 5: sum) {
      (sum == 10) -> {
        result = "ten";
      }
    }
    "#
            .as_bytes(),
        );

        let program = setup_program(text);
        let mut interpreter = create_interpreter(&program);
        interpreter.interpret().unwrap();

        assert_eq!(
            interpreter.stack().get_variable("result").unwrap().clone(),
            Rc::new(RefCell::new(Value::String(String::from("ten"))))
        );
    }

    #[test]
    fn vector_index_assignment_on_declared_variable() {
        let text = BufReader::new(
            r#"
    i64[] arr = [1, 2, 3];
    arr[1] = 99;
    i64 first = arr[0];
    i64 second = arr[1];
    "#
            .as_bytes(),
        );

        let program = setup_program(text);
        let mut interpreter = create_interpreter(&program);
        interpreter.interpret().unwrap();

        assert_eq!(
            interpreter.stack().get_variable("first").unwrap().clone(),
            Rc::new(RefCell::new(Value::I64(1)))
        );
        assert_eq!(
            interpreter.stack().get_variable("second").unwrap().clone(),
            Rc::new(RefCell::new(Value::I64(99)))
        );
    }

    #[test]
    fn function_returning_vector() {
        let text = BufReader::new(
            r#"
    fn make_range(i64 n): i64[] {
      i64[] result = [];
      for (i64 i = 0; i < n; i = i + 1) {
        vector_push(&result, i);
      }
      return result;
    }

    i64[] range = make_range(4);
    i64 sum = 0;
    for (i64 i = 0; i < vector_size(&range); i += 1) {
      sum += range[i];
    }
    "#
            .as_bytes(),
        );

        let program = setup_program(text);
        let mut interpreter = create_interpreter(&program);
        interpreter.interpret().unwrap();

        // 0+1+2+3 = 6
        assert_eq!(
            interpreter.stack().get_variable("sum").unwrap().clone(),
            Rc::new(RefCell::new(Value::I64(6)))
        );
    }

    #[test]
    fn boolean_short_circuit_not_required_but_correct_result() {
        let text = BufReader::new(
            r#"
    bool a = true || false;
    bool b = true && false;
    bool c = false || false;
    bool d = true && true;
    "#
            .as_bytes(),
        );

        let program = setup_program(text);
        let mut interpreter = create_interpreter(&program);
        interpreter.interpret().unwrap();

        assert_eq!(
            interpreter.stack().get_variable("a").unwrap().clone(),
            Rc::new(RefCell::new(Value::Bool(true)))
        );
        assert_eq!(
            interpreter.stack().get_variable("b").unwrap().clone(),
            Rc::new(RefCell::new(Value::Bool(false)))
        );
        assert_eq!(
            interpreter.stack().get_variable("c").unwrap().clone(),
            Rc::new(RefCell::new(Value::Bool(false)))
        );
        assert_eq!(
            interpreter.stack().get_variable("d").unwrap().clone(),
            Rc::new(RefCell::new(Value::Bool(true)))
        );
    }
}
