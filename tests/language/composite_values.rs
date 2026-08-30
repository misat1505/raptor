use std::io::BufReader;

use crate::common::assert_same_output;

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

#[test]
fn nested_vector_of_vectors_indexed_assignment() {
    let text = BufReader::new(
        r#"
let matrix = [[1 as i64, 2 as i64], [3 as i64, 4 as i64]];

matrix[0][1] = 99 as i64;
matrix[1][0] = 88 as i64;

println(matrix[0][0] as str);
println(matrix[0][1] as str);
println(matrix[1][0] as str);
println(matrix[1][1] as str);

# row is the same underlying object as matrix[0] (vector assignment is by reference)
let row = matrix[0];
row[0] = 77 as i64;

println(matrix[0][0] as str);
println(row[0] as str);
        "#
        .as_bytes(),
    );

    assert_same_output(text, "1\n99\n88\n4\n77\n77\n");
}

#[test]
fn vector_of_structs_indexed_field_assignment() {
    let text = BufReader::new(
        r#"
struct Item {
    i64 value
};

let a = Item { value: 1 as i64 };
let b = Item { value: 2 as i64 };
let items = [a, b];

items[0].value = 99 as i64;

println(items[0].value as str);
println(items[1].value as str);

# items[0] shares the same underlying struct object as `a` (struct in vector is not copied)
println(a.value as str);
        "#
        .as_bytes(),
    );

    assert_same_output(text, "99\n2\n99\n");
}

#[test]
fn deeply_nested_struct_vector_struct_field_chain_assignment() {
    let text = BufReader::new(
        r#"
struct Tag {
    str label
};

struct Item {
    i64 value,
    Tag[] tags
};

struct Container {
    str name,
    Item[] items
};

let tag1 = Tag { label: "x" };
let item1 = Item { value: 1 as i64, tags: [tag1] };
let container = Container { name: "c", items: [item1] };

container.items[0].value = 99 as i64;
container.items[0].tags[0].label = "changed";

println(container.items[0].value as str);
println(container.items[0].tags[0].label as str);

# intermediate objects were not copied along the chain - mutation through the chain
# is visible from the originally referenced objects too
println(item1.value as str);
println(tag1.label as str);
        "#
        .as_bytes(),
    );

    assert_same_output(text, "99\nchanged\n99\nchanged\n");
}

#[test]
fn three_levels_of_nested_struct_field_assignment() {
    let text = BufReader::new(
        r#"
struct Level3 {
    i64 value
};

struct Level2 {
    Level3 inner
};

struct Level1 {
    Level2 inner
};

let l3 = Level3 { value: 1 as i64 };
let l2 = Level2 { inner: l3 };
let l1 = Level1 { inner: l2 };

l1.inner.inner.value = 99 as i64;

println(l1.inner.inner.value as str);
println(l2.inner.value as str);
println(l3.value as str);
        "#
        .as_bytes(),
    );

    assert_same_output(text, "99\n99\n99\n");
}

#[test]
fn vector_of_structs_passed_by_value_shallow_copies_the_array_but_aliases_the_elements() {
    let text = BufReader::new(
        r#"
struct Item {
    i64 value
};

fn replace_first(Item[] items): void {
    # reassigning an element only affects this function's own (shallow-copied) array
    items[0] = Item { value: 999 as i64 };
}

fn mutate_first_field(Item[] items): void {
    # mutating a field on an aliased element affects the caller's original struct object
    items[0].value = 555 as i64;
}

let a = Item { value: 1 as i64 };
let items = [a];

replace_first(items);
println(items[0].value as str);

mutate_first_field(items);
println(items[0].value as str);
println(a.value as str);
        "#
        .as_bytes(),
    );

    assert_same_output(text, "1\n555\n555\n");
}

#[test]
fn struct_with_vector_field_passed_by_value_mixes_copy_and_alias_semantics() {
    let text = BufReader::new(
        r#"
struct Item {
    i64 value
};

struct Bag {
    i64 count,
    Item[] items
};

fn mutate(Bag bag): void {
    # primitive field: this mutation is local to the by-value copy
    bag.count = 999 as i64;

    # composite field (vector): aliased, so this mutation is visible to the caller
    bag.items[0].value = 999 as i64;
}

let item = Item { value: 1 as i64 };
let bag = Bag { count: 1 as i64, items: [item] };

mutate(bag);

println(bag.count as str);
println(bag.items[0].value as str);
println(item.value as str);
        "#
        .as_bytes(),
    );

    assert_same_output(text, "1\n999\n999\n");
}

#[test]
fn deeply_nested_struct_vector_vector_vector_struct_struct_vector_struct_chain() {
    let text = BufReader::new(
        r#"
struct C {
    i64 value
};

struct B {
    C[] items
};

struct A {
    B b
};

struct Root {
    A[][][] cube
};

let c = C { value: 1 as i64 };
let b = B { items: [c] };
let a = A { b: b };
let cube = [[[a]]];
let root = Root { cube: cube };

root.cube[0][0][0].b.items[0].value = 99 as i64;

println(root.cube[0][0][0].b.items[0].value as str);

# nothing along the chain was copied - every original object sees the mutation
println(a.b.items[0].value as str);
println(b.items[0].value as str);
println(c.value as str);

# mutate one level higher in the chain (reassign the whole C struct via the vector)
root.cube[0][0][0].b.items[0] = C { value: 42 as i64 };

println(root.cube[0][0][0].b.items[0].value as str);
println(b.items[0].value as str);

# mutate through an intermediate alias obtained by indexing the vector-of-vectors
let inner_cube = root.cube[0][0];
inner_cube[0].b.items[0].value = 7 as i64;

println(root.cube[0][0][0].b.items[0].value as str);
println(a.b.items[0].value as str);
        "#
        .as_bytes(),
    );

    assert_same_output(text, "99\n99\n99\n99\n42\n42\n7\n7\n");
}
