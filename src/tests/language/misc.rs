use std::io::BufReader;

use crate::tests::common::helpers::assert_same_output;

#[test]
fn comments_are_ignored() {
    let text = BufReader::new(
        r#"
    # this is a full line comment
    i64 x = 5; # trailing comment
    # another comment
    i64 y = x + 1;

    println(y as str);
    "#
        .as_bytes(),
    );

    assert_same_output(text, "6\n");
}

#[test]
fn empty_program_does_nothing() {
    let text = BufReader::new(r#""#.as_bytes());

    assert_same_output(text, "");
}
