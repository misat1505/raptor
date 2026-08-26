use std::io::BufReader;

use crate::tests::common::helpers::assert_same_output;

#[test]
fn string_is_a_primitive_value() {
    let text = BufReader::new(
        r#"
let a = "A";
let b = a;

a[0] = 'B';

println(a);
println(b);

str c = "A";
str d = c;

c[0] = 'B';

println(c);
println(d);

a = b;
b[0] = 'B';

println(a);
println(b);
        "#
        .as_bytes(),
    );

    assert_same_output(text, "B\nA\nB\nA\nA\nB\n");
}

#[test]
fn string_is_correctly_passed_to_functions() {
    let text = BufReader::new(
        r#"
fn str_by_value(str text): void {
    text[0] = 'B';
}

fn str_by_ref(&str text): void {
    text[0] = 'B';
}

let text = "A";

str_by_value(text);
println(text);

str_by_ref(&text);
println(text);
        "#
        .as_bytes(),
    );

    assert_same_output(text, "A\nB\n");
}

#[test]
fn string_is_correctly_passed_to_vector_literal() {
    let text = BufReader::new(
        r#"
let text = "A";
let vec = [text];

vec[0][0] = 'B';

println(text);
println(vec[0]);
        "#
        .as_bytes(),
    );

    assert_same_output(text, "A\nB\n");
}
