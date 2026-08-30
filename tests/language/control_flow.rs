use std::io::BufReader;

use crate::common::assert_same_output;

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

        println(text);
        "#
        .as_bytes(),
    );

    assert_same_output(text, "equal\n");
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

        println(i as str);
        "#
        .as_bytes(),
    );

    assert_same_output(text, "2\n");
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

        println(i as str);
        println(total as str);
        "#
        .as_bytes(),
    );

    assert_same_output(text, "5\n10\n");
}

#[test]
fn continue_in_for_loop() {
    let text = BufReader::new(
        r#"
        i64 total = 0;

        for (i64 i = 0; i < 5; i += 1) {
            if (i % 2 == 0) {
                continue;
            }

            total = total + i;
        }

        println(total as str);
        "#
        .as_bytes(),
    );

    assert_same_output(text, "4\n");
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

        println(total as str);
        "#
        .as_bytes(),
    );

    assert_same_output(text, "9\n");
}

#[test]
fn for_loop_without_declaration_and_assignment() {
    let text = BufReader::new(
        r#"
    i64 i = 0;
    for (; i < 3;) {
      i = i + 1;
    }

    println(i as str);
    "#
        .as_bytes(),
    );

    assert_same_output(text, "3\n");
}

#[test]
fn nested_for_loops_with_break_only_breaks_inner() {
    let text = BufReader::new(
        r#"
    i64 outer_count = 0;
    i64 inner_count = 0;
    for (i64 i = 0; i < 3; i = i + 1) {
      outer_count = outer_count + 1;
      for (i64 j = 0; j < 10; j = j + 1) {
        if (j == 2) {
          break;
        }
        inner_count = inner_count + 1;
      }
    }

    println(outer_count as str);
    println(inner_count as str);
    "#
        .as_bytes(),
    );

    assert_same_output(text, "3\n6\n");
}
