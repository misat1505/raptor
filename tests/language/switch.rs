use std::io::BufReader;

use crate::common::assert_same_output;

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
    println(text);
    "#
        .as_bytes(),
    );

    assert_same_output(text, ">1\n");
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
    println(result);
    "#
        .as_bytes(),
    );

    assert_same_output(text, "ten\n");
}

#[test]
fn multiple_switch_expressions_with_aliases() {
    let text = BufReader::new(
        r#"
    i64 a = 3;
    i64 b = 7;
    str result;
    switch (a: x, b: y) {
      (x + y == 10) -> {
        result = "sums to ten";
      }
    }

    println(result);
    "#
        .as_bytes(),
    );

    assert_same_output(text, "sums to ten\n");
}

#[test]
fn switch_executes_all_matching_cases_without_break() {
    let text = BufReader::new(
        r#"
    i64 x = 5;
    i64 counter = 0;
    switch (x) {
      (x > 0) -> {
        counter = counter + 1;
      }
      (x > 1) -> {
        counter = counter + 1;
      }
      (x > 100) -> {
        counter = counter + 1;
      }
    }

    println(counter as str);
    "#
        .as_bytes(),
    );

    assert_same_output(text, "2\n");
}

#[test]
fn switch_no_matching_case_leaves_variable_untouched() {
    let text = BufReader::new(
        r#"
    i64 x = -5;
    str result = "default";
    switch (x) {
      (x > 0) -> {
        result = "positive";
      }
      (x == 0) -> {
        result = "zero";
      }
    }

    println(result);
    "#
        .as_bytes(),
    );

    assert_same_output(text, "default\n");
}

#[test]
fn nested_switch_inside_loop_with_continue() {
    let text = BufReader::new(
        r#"
    i64 total = 0;
    for (i64 i = 0; i < 5; i += 1) {
      switch (i) {
        (i % 2 == 0) -> {
          continue;
        }
      }
      total += i;
    }

    println(total as str);
    "#
        .as_bytes(),
    );

    assert_same_output(text, "4\n");
}
