use std::io::BufReader;

use crate::tests::common::helpers::assert_same_output;

#[test]
fn reference() {
    let text = BufReader::new(
        r#"
    fn foo(&i64 x): void {
      x = x + 1;
    }

    i64 x = 2;
    foo(&x);
    println(x as str);
    "#
        .as_bytes(),
    );

    assert_same_output(text, "3\n");
}

#[test]
fn pass_vector_by_reference_mutates_original() {
    let text = BufReader::new(
        r#"
    fn append_one(&i64[] arr): void {
      vector_push(&arr, 1);
    }

    i64[] numbers = [];
    append_one(&numbers);
    append_one(&numbers);
    i64 size = vector_size(&numbers);

    println(size as str);
    "#
        .as_bytes(),
    );

    assert_same_output(text, "2\n");
}

#[test]
fn reference_argument_evaluation_order() {
    let text = BufReader::new(
        r#"
    fn set_both(&i64 a, &i64 b): void {
      a = 1;
      b = 2;
    }

    i64 x = 0;
    i64 y = 0;
    set_both(&x, &y);

    println(x as str);
    println(y as str);
    "#
        .as_bytes(),
    );

    assert_same_output(text, "1\n2\n");
}
