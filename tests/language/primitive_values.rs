use std::io::BufReader;

use crate::common::assert_same_output;

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

#[test]
fn string_is_correctly_passed_to_struct_literal() {
    let text = BufReader::new(
        r#"
struct Person {
    str name
};

let name = "A";
let person = Person { name };

person.name[0] = 'B';

println(name);
println(person.name);
        "#
        .as_bytes(),
    );

    assert_same_output(text, "A\nB\n");
}

#[test]
fn string_is_correctly_copied_on_indexed_assignment() {
    let text = BufReader::new(
        r#"
struct Person {
    str name
};

let c = "A";
let vec = ["X"];
let person = Person { name: "X" };

vec[0] = c;
person.name = c;
c[0] = 'B';

println(c);
println(vec[0]);
println(person.name);
        "#
        .as_bytes(),
    );

    assert_same_output(text, "B\nA\nA\n");
}

#[test]
fn string_is_correctly_copied_on_return() {
    let text = BufReader::new(
        r#"
fn identity(str x): str {
    return x;
}

let a = "A";
let b = identity(a);

b[0] = 'B';

println(a);
println(b);
        "#
        .as_bytes(),
    );

    assert_same_output(text, "A\nB\n");
}

#[test]
fn primitives_are_copied_on_assignment() {
    let text = BufReader::new(
        r#"
let a = 1 as i64;
let b = a;
a = 2 as i64;
println(a as str);
println(b as str);

i8 c = 1 as i8;
i8 d = c;
c = 2 as i8;
println(c as str);
println(d as str);

i16 e = 1 as i16;
i16 f = e;
e = 2 as i16;
println(e as str);
println(f as str);

i32 g = 1 as i32;
i32 h = g;
g = 2 as i32;
println(g as str);
println(h as str);

u8 i2 = 1 as u8;
u8 j = i2;
i2 = 2 as u8;
println(i2 as str);
println(j as str);

u16 k = 1 as u16;
u16 l = k;
k = 2 as u16;
println(k as str);
println(l as str);

u32 m = 1 as u32;
u32 n = m;
m = 2 as u32;
println(m as str);
println(n as str);

u64 o = 1 as u64;
u64 p = o;
o = 2 as u64;
println(o as str);
println(p as str);

let q = 1.5;
let r = q;
q = 2.5;
println(q as str);
println(r as str);

let s = true;
let t = s;
s = false;
println(s as str);
println(t as str);

let u = 'A';
let v = u;
u = 'B';
println(u as str);
println(v as str);
        "#
        .as_bytes(),
    );

    assert_same_output(text, "2\n1\n2\n1\n2\n1\n2\n1\n2\n1\n2\n1\n2\n1\n2\n1\n2.5\n1.5\nfalse\ntrue\nB\nA\n");
}

#[test]
fn primitives_are_correctly_passed_to_functions() {
    let text = BufReader::new(
        r#"
fn i64_by_value(i64 x): void {
    x = 99 as i64;
}

fn i64_by_ref(&i64 x): void {
    x = 99 as i64;
}

fn f64_by_value(f64 x): void {
    x = 9.9;
}

fn f64_by_ref(&f64 x): void {
    x = 9.9;
}

fn bool_by_value(bool x): void {
    x = false;
}

fn bool_by_ref(&bool x): void {
    x = false;
}

fn char_by_value(char x): void {
    x = 'Z';
}

fn char_by_ref(&char x): void {
    x = 'Z';
}

let a = 1 as i64;
i64_by_value(a);
println(a as str);
i64_by_ref(&a);
println(a as str);

let f = 1.1;
f64_by_value(f);
println(f as str);
f64_by_ref(&f);
println(f as str);

let b = true;
bool_by_value(b);
println(b as str);
bool_by_ref(&b);
println(b as str);

let c = 'A';
char_by_value(c);
println(c as str);
char_by_ref(&c);
println(c as str);
        "#
        .as_bytes(),
    );

    assert_same_output(text, "1\n99\n1.1\n9.9\ntrue\nfalse\nA\nZ\n");
}

#[test]
fn primitives_are_correctly_copied_into_vector_literal() {
    let text = BufReader::new(
        r#"
let a = 1 as i64;
let vec_i = [a];
a = 2 as i64;
println(a as str);
println(vec_i[0] as str);

let f = 1.1;
let vec_f = [f];
f = 2.2;
println(f as str);
println(vec_f[0] as str);

let b = true;
let vec_b = [b];
b = false;
println(b as str);
println(vec_b[0] as str);

let c = 'A';
let vec_c = [c];
c = 'B';
println(c as str);
println(vec_c[0] as str);
        "#
        .as_bytes(),
    );

    assert_same_output(text, "2\n1\n2.2\n1.1\nfalse\ntrue\nB\nA\n");
}

#[test]
fn primitives_are_correctly_copied_into_struct_literal() {
    let text = BufReader::new(
        r#"
struct Numbers {
    i64 value,
    f64 fvalue,
    bool bvalue,
    char cvalue
};

let a = 1 as i64;
let f = 1.1;
let b = true;
let c = 'A';

let n = Numbers { value: a, fvalue: f, bvalue: b, cvalue: c };

a = 2 as i64;
f = 2.2;
b = false;
c = 'B';

println(a as str);
println(n.value as str);

println(f as str);
println(n.fvalue as str);

println(b as str);
println(n.bvalue as str);

println(c as str);
println(n.cvalue as str);
        "#
        .as_bytes(),
    );

    assert_same_output(text, "2\n1\n2.2\n1.1\nfalse\ntrue\nB\nA\n");
}
