use std::io::BufReader;

use crate::common::assert_same_output;

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
    println(vector_stringify(arr));
    "#
        .as_bytes(),
    );

    assert_same_output(text, "[[1], [69]]\n");
}

#[test]
fn vector_index_assignment_on_declared_variable() {
    let text = BufReader::new(
        r#"
    i64[] arr = [1, 2, 3];
    arr[1] = 99;
    i64 first = arr[0];
    i64 second = arr[1];

    println(first as str);
    println(second as str);
    "#
        .as_bytes(),
    );

    assert_same_output(text, "1\n99\n");
}

#[test]
fn deeply_nested_vector_index_assignment() {
    let text = BufReader::new(
        r#"
    i64[][][] cube = [[[1, 2], [3, 4]], [[5, 6], [7, 8]]];
    cube[1][0][1] = 99;
    i64 x = cube[1][0][1];
    i64 untouched = cube[0][0][0];

    println(x as str);
    println(untouched as str);
    "#
        .as_bytes(),
    );

    assert_same_output(text, "99\n1\n");
}

#[test]
fn empty_vector_declaration() {
    let text = BufReader::new(
        r#"
    i64[] empty = [];
    i64 size = vector_size(&empty);
    vector_push(&empty, 1);
    i64 size_after = vector_size(&empty);

    println(size as str);
    println(size_after as str);
    "#
        .as_bytes(),
    );

    assert_same_output(text, "0\n1\n");
}
