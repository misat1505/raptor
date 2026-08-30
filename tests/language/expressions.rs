use std::io::BufReader;

use crate::common::assert_same_output;

#[test]
fn string_concatenation() {
    let text = BufReader::new(
        r#"
    str a = "Hello, ";
    str b = "world!";
    str c = a + b;
    println(c);
    "#
        .as_bytes(),
    );

    assert_same_output(text, "Hello, world!\n");
}

#[test]
fn modulo_operator() {
    let text = BufReader::new(
        r#"
    i64 a = 10 % 3;
    i64 b = 10 % 3;
    i64 c = 10;
    c %= 3;
    println(a as str);
    println(b as str);
    println(c as str);
    "#
        .as_bytes(),
    );

    assert_same_output(text, "1\n1\n1\n");
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

    println(a as str);
    println(b as str);
    println(c as str);
    println(d as str);
    println(e as str);
    "#
        .as_bytes(),
    );

    assert_same_output(text, "14\n20\ntrue\ntrue\n5\n");
}

#[test]
fn boolean_short_circuit_not_required_but_correct_result() {
    let text = BufReader::new(
        r#"
    bool a = true || false;
    bool b = true && false;
    bool c = false || false;
    bool d = true && true;

    println(a as str);
    println(b as str);
    println(c as str);
    println(d as str);
    "#
        .as_bytes(),
    );

    assert_same_output(text, "true\nfalse\nfalse\ntrue\n");
}

#[test]
fn integer_division_truncates() {
    let text = BufReader::new(
        r#"
    i64 a = 7 / 2;
    i64 b = -7 / 2;

    println(a as str);
    println(b as str);
    "#
        .as_bytes(),
    );

    assert_same_output(text, "3\n-3\n");
}

#[test]
fn unary_negation_chain() {
    let text = BufReader::new(
        r#"
    i64 a = 5;
    i64 b = -(-a);
    i64 c = -a - -a;
    bool d = !(!true);

    println(b as str);
    println(c as str);
    println(d as str);
    "#
        .as_bytes(),
    );

    assert_same_output(text, "5\n0\ntrue\n");
}

#[test]
fn string_equality_and_inequality() {
    let text = BufReader::new(
        r#"
    str a = "hello";
    str b = "hello";
    str c = "world";
    bool eq = a == b;
    bool neq = a != c;

    println(eq as str);
    println(neq as str);
    "#
        .as_bytes(),
    );

    assert_same_output(text, "true\ntrue\n");
}

#[test]
fn float_comparisons() {
    let text = BufReader::new(
        r#"
    f64 a = 1.5;
    f64 b = 2.5;
    bool less = a < b;
    bool greater_equal = b >= a;
    bool equal_same = a == 1.5;

    println(less as str);
    println(greater_equal as str);
    println(equal_same as str);
    "#
        .as_bytes(),
    );

    assert_same_output(text, "true\ntrue\ntrue\n");
}
