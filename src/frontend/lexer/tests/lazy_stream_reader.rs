
use std::io::BufReader;

use crate::frontend::lexer::lazy_stream_reader::{ILazyStreamReader, LazyStreamReader, ETX, STX};

#[test]
fn test_lazy_stream_reader() {
    let code = BufReader::new(
        r#"hello
world"#
            .as_bytes(),
    );
    let mut stream_reader = LazyStreamReader::new(code, None);

    let expected: Vec<(char, u32, u32)> = vec![
        ('h', 1, 1),
        ('e', 1, 2),
        ('l', 1, 3),
        ('l', 1, 4),
        ('o', 1, 5),
        ('\n', 1, 6),
        ('w', 2, 1),
        ('o', 2, 2),
        ('r', 2, 3),
        ('l', 2, 4),
        ('d', 2, 5),
        (ETX, 2, 6),
        (ETX, 2, 6),
    ];

    assert_eq!(*stream_reader.current(), STX);
    assert_eq!(stream_reader.position().line, 0);
    assert_eq!(stream_reader.position().column, 0);

    for (exp_char, exp_line, exp_col) in &expected {
        assert_eq!(*stream_reader.next().unwrap(), *exp_char);
        assert_eq!(stream_reader.position().line, *exp_line);
        assert_eq!(stream_reader.position().column, *exp_col);
    }
}

#[test]
fn empty_input_yields_etx_immediately() {
    let code = BufReader::new("".as_bytes());
    let mut reader = LazyStreamReader::new(code, None);
    assert_eq!(*reader.current(), STX);
    assert_eq!(*reader.next().unwrap(), ETX);
    assert_eq!(*reader.next().unwrap(), ETX); // stays at ETX
    assert_eq!(reader.position().line, 1);
    assert_eq!(reader.position().column, 1);
}

#[test]
fn single_line_no_newline() {
    let code = BufReader::new("abc".as_bytes());
    let mut reader = LazyStreamReader::new(code, None);
    assert_eq!(*reader.current(), STX);
    assert_eq!(*reader.next().unwrap(), 'a');
    assert_eq!(reader.position(), crate::common::position::Position::new(1, 1, 0, None));
    assert_eq!(*reader.next().unwrap(), 'b');
    assert_eq!(reader.position().column, 2);
    assert_eq!(*reader.next().unwrap(), 'c');
    assert_eq!(reader.position().column, 3);
    assert_eq!(*reader.next().unwrap(), ETX);
    assert_eq!(reader.position().column, 4);
}

#[test]
fn handles_lf_newline() {
    let code = BufReader::new("a\nb".as_bytes());
    let mut reader = LazyStreamReader::new(code, None);
    assert_eq!(*reader.next().unwrap(), 'a');
    assert_eq!(reader.position().line, 1);
    assert_eq!(reader.position().column, 1);
    assert_eq!(*reader.next().unwrap(), '\n');
    assert_eq!(reader.position().line, 1);
    assert_eq!(reader.position().column, 2);
    assert_eq!(*reader.next().unwrap(), 'b');
    assert_eq!(reader.position().line, 2);
    assert_eq!(reader.position().column, 1);
    assert_eq!(*reader.next().unwrap(), ETX);
}

#[test]
fn handles_crlf_newline() {
    let code = BufReader::new("a\r\nb".as_bytes());
    let mut reader = LazyStreamReader::new(code, None);
    assert_eq!(*reader.next().unwrap(), 'a');
    assert_eq!(*reader.next().unwrap(), '\n'); // \r\n collapsed to \n
    assert_eq!(reader.position().line, 1);
    assert_eq!(reader.position().column, 2);
    assert_eq!(*reader.next().unwrap(), 'b');
    assert_eq!(reader.position().line, 2);
    assert_eq!(reader.position().column, 1);
}

#[test]
fn handles_lone_cr_as_newline() {
    let code = BufReader::new("a\rb".as_bytes());
    let mut reader = LazyStreamReader::new(code, None);
    assert_eq!(*reader.next().unwrap(), 'a');
    assert_eq!(*reader.next().unwrap(), '\n'); // lone \r → \n
    assert_eq!(*reader.next().unwrap(), 'b');
    assert_eq!(reader.position().line, 2);
    assert_eq!(reader.position().column, 1);
}

#[test]
fn multiple_newlines() {
    let code = BufReader::new("\n\n\nx".as_bytes());
    let mut reader = LazyStreamReader::new(code, None);
    assert_eq!(*reader.next().unwrap(), '\n');
    assert_eq!(reader.position().line, 1);
    assert_eq!(*reader.next().unwrap(), '\n');
    assert_eq!(reader.position().line, 2);
    assert_eq!(*reader.next().unwrap(), '\n');
    assert_eq!(reader.position().line, 3);
    assert_eq!(*reader.next().unwrap(), 'x');
    assert_eq!(reader.position().line, 4);
    assert_eq!(reader.position().column, 1);
}

#[test]
fn position_offset_advances() {
    let code = BufReader::new("ab\nc".as_bytes());
    let mut reader = LazyStreamReader::new(code, None);
    reader.next().unwrap(); // 'a'  → offset still based on previous
    reader.next().unwrap(); // 'b'
    let pos_after_b = reader.position();
    reader.next().unwrap(); // '\n'
    let pos_after_nl = reader.position();
    // after newline offset should have advanced by the length of the newline sequence
    assert!(pos_after_nl.offset > pos_after_b.offset);
    reader.next().unwrap(); // 'c'
    assert_eq!(reader.position().line, 2);
    assert_eq!(reader.position().column, 1);
}

#[test]
fn stays_at_etx_after_eof() {
    let code = BufReader::new("x".as_bytes());
    let mut reader = LazyStreamReader::new(code, None);
    assert_eq!(*reader.next().unwrap(), 'x');
    assert_eq!(*reader.next().unwrap(), ETX);
    assert_eq!(*reader.next().unwrap(), ETX);
    assert_eq!(*reader.next().unwrap(), ETX);
    // position should not change once at ETX
    let pos = reader.position();
    assert_eq!(*reader.next().unwrap(), ETX);
    assert_eq!(reader.position(), pos);
}

#[test]
fn filename_is_stored_in_position() {
    let code = BufReader::new("a".as_bytes());
    let mut reader = LazyStreamReader::new(code, Some("test.src"));
    reader.next().unwrap();
    assert_eq!(reader.position().filename, Some("test.src"));
}

#[test]
fn only_stx_then_etx_on_empty() {
    let code = BufReader::new("".as_bytes());
    let mut reader = LazyStreamReader::new(code, None);
    assert_eq!(*reader.current(), STX);
    assert_eq!(reader.position().line, 0);
    assert_eq!(reader.position().column, 0);
    assert_eq!(*reader.next().unwrap(), ETX);
}
