use std::io::BufReader;

use crate::tests::common::helpers::assert_same_output;

#[test]
fn vector_assignment_is_by_reference() {
    let text = BufReader::new(
        r#"
let a = [1 as i64, 2 as i64];
let b = a;

a[0] = 99 as i64;

println(a[0] as str);
println(b[0] as str);
        "#
        .as_bytes(),
    );

    assert_same_output(text, "99\n99\n");
}

#[test]
fn struct_assignment_is_by_reference() {
    let text = BufReader::new(
        r#"
struct Point {
    i64 x
};

let a = Point { x: 1 as i64 };
let b = a;

a.x = 99 as i64;

println(a.x as str);
println(b.x as str);
        "#
        .as_bytes(),
    );

    assert_same_output(text, "99\n99\n");
}

#[test]
fn composites_are_correctly_passed_by_reference() {
    let text = BufReader::new(
        r#"
struct Point {
    i64 x
};

fn vec_by_ref(&i64[] v): void {
    v[0] = 99 as i64;
}

fn struct_by_ref(&Point p): void {
    p.x = 99 as i64;
}

let vec = [1 as i64, 2 as i64];
vec_by_ref(&vec);
println(vec[0] as str);

let point = Point { x: 1 as i64 };
struct_by_ref(&point);
println(point.x as str);
        "#
        .as_bytes(),
    );

    assert_same_output(text, "99\n99\n");
}

#[test]
fn vector_passed_by_value_is_a_shallow_copy() {
    let text = BufReader::new(
        r#"
fn vec_by_value(i64[] v): void {
    v[0] = 99 as i64;
}

let vec = [1 as i64, 2 as i64];
vec_by_value(vec);

println(vec[0] as str);
        "#
        .as_bytes(),
    );

    assert_same_output(text, "1\n");
}

#[test]
fn struct_passed_by_value_copies_primitive_fields_but_aliases_composite_fields() {
    let text = BufReader::new(
        r#"
struct Inner {
    i64 value
};

struct Outer {
    i64 primitive_field,
    Inner composite_field
};

fn mutate(Outer o): void {
    o.primitive_field = 99 as i64;
    o.composite_field.value = 99 as i64;
}

let inner = Inner { value: 1 as i64 };
let outer = Outer { primitive_field: 1 as i64, composite_field: inner };

mutate(outer);

println(outer.primitive_field as str);

println(outer.composite_field.value as str);
        "#
        .as_bytes(),
    );

    assert_same_output(text, "1\n99\n");
}
