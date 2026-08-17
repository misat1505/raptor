use std::io::BufReader;

use crate::tests::common::helpers::assert_same_output;

#[test]
fn casting() {
    let text = BufReader::new(
        r#"
    i64 a = 5;
    f64 b = a as f64 + 0.1;
    str c = a as str;
    bool d = a as bool;
    bool e = 0 as bool;
    i64 f = "123" as i64;

    println(a as str);
    println(b as str);
    println(c);
    println(d as str);
    println(e as str);
    println(f as str);
    "#
        .as_bytes(),
    );

    assert_same_output(text, "5\n5.1\n5\ntrue\nfalse\n123\n");
}

#[test]
fn cast_negative_number_to_bool() {
    let text = BufReader::new(
        r#"
    i64 neg = 0 - 5;
    bool a = neg as bool;
    f64 neg_f = 0.0 - 3.5;
    bool b = neg_f as bool;

    println(a as str);
    println(b as str);
    "#
        .as_bytes(),
    );

    assert_same_output(text, "false\nfalse\n");
}

#[test]
fn cast_empty_and_nonempty_string_to_bool() {
    let text = BufReader::new(
        r#"
    str empty = "";
    str nonempty = "x";
    bool a = empty as bool;
    bool b = nonempty as bool;

    println(a as str);
    println(b as str);
    "#
        .as_bytes(),
    );

    assert_same_output(text, "false\ntrue\n");
}

#[test]
fn default_values_for_all_types() {
    let text = BufReader::new(
        r#"
    i64 a;
    f64 b;
    str c;
    bool d;
    i64[] e;
    i64[][] f;

    println(a as str);
    println(b as str);
    println(c);
    println(d as str);
    println(vector_stringify(e));
    println(vector_stringify(f));
    "#
        .as_bytes(),
    );

    assert_same_output(text, "0\n0\n\nfalse\n[]\n[]\n");
}
