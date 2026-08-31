use std::{io::BufReader, println};

use crate::{
    common::errors::IError,
    frontend::{
        lexer::{
            lazy_stream_reader::LazyStreamReader,
            lexer::{ILexer, Lexer, LexerOptions},
        },
        tokens::{TokenCategory, TokenValue},
    },
};

fn on_warning(warning: Box<dyn IError>) {
    println!("{}", warning.message());
}

fn create_lexer(text: &str) -> Lexer {
    let owned_text: &'static str = Box::leak(text.to_owned().into_boxed_str());
    let code = BufReader::new(owned_text.as_bytes());
    let reader = LazyStreamReader::new(code, None);

    let lexer_options = LexerOptions {
        max_comment_length: 100,
        max_identifier_length: 20,
    };

    let lexer = Lexer::new_unsafe(reader, lexer_options, on_warning).unwrap();

    lexer
}

fn create_lexer_with_skip(text: &str) -> Lexer {
    let mut lexer = create_lexer(text);
    let _ = lexer.generate_token().unwrap();

    lexer
}

#[test]
fn constructor() {
    let text = "123";
    let mut lexer = create_lexer(text);
    assert!(lexer.current().is_none());
    let token = lexer.generate_token().unwrap();
    assert_eq!(token.category, TokenCategory::STX);
}

#[test]
fn last_token() {
    let mut lexer = create_lexer_with_skip("");
    let mut token = lexer.generate_token().unwrap();
    assert_eq!(token.category, TokenCategory::ETX);
    token = lexer.generate_token().unwrap();
    assert_eq!(token.category, TokenCategory::ETX);
}

#[test]
fn signs() {
    let text = "( ) [  ] {} ;   :, ";
    let mut lexer = create_lexer_with_skip(text);
    let expected_tokens: Vec<TokenCategory> = vec![
        TokenCategory::ParenOpen,
        TokenCategory::ParenClose,
        TokenCategory::BracketOpen,
        TokenCategory::BracketClose,
        TokenCategory::BraceOpen,
        TokenCategory::BraceClose,
        TokenCategory::Semicolon,
        TokenCategory::Colon,
        TokenCategory::Comma,
    ];

    for expected_token in &expected_tokens {
        let token = lexer.generate_token().unwrap();
        assert_eq!(token.category, *expected_token);
    }
}

#[test]
fn operators() {
    let text = "+* / --> <<= > >= ! != = == & && || ";
    let mut lexer = create_lexer_with_skip(text);
    let expected_tokens: Vec<TokenCategory> = vec![
        TokenCategory::Plus,
        TokenCategory::Multiply,
        TokenCategory::Divide,
        TokenCategory::Minus,
        TokenCategory::Arrow,
        TokenCategory::Less,
        TokenCategory::LessOrEqual,
        TokenCategory::Greater,
        TokenCategory::GreaterOrEqual,
        TokenCategory::Negate,
        TokenCategory::NotEqual,
        TokenCategory::Assign,
        TokenCategory::Equal,
        TokenCategory::Reference,
        TokenCategory::And,
        TokenCategory::Or,
    ];

    for expected_token in &expected_tokens {
        let token = lexer.generate_token().unwrap();
        assert_eq!(token.category, *expected_token);
    }
}

#[test]
fn passes_through_comments() {
    let text = "# this is a comment
        # another
        i64";
    let mut lexer = create_lexer_with_skip(text);

    let token = lexer.generate_token().unwrap();
    assert_eq!(token.category, TokenCategory::I64);
}

#[test]
fn string() {
    let text = r#""string1"    " string2  ""string3""#;
    let mut lexer = create_lexer_with_skip(text);

    let mut token = lexer.generate_token().unwrap();
    assert_eq!(token.category, TokenCategory::StringValue);
    assert_eq!(token.value, TokenValue::String(String::from("string1")));

    token = lexer.generate_token().unwrap();
    assert_eq!(token.category, TokenCategory::StringValue);
    assert_eq!(token.value, TokenValue::String(String::from(" string2  ")));

    token = lexer.generate_token().unwrap();
    assert_eq!(token.category, TokenCategory::StringValue);
    assert_eq!(token.value, TokenValue::String(String::from("string3")));
}

#[test]
fn escapes() {
    let text = r#""ala\"ma\nkota\tjana\\i\szympansa""#;
    let mut lexer = create_lexer_with_skip(text);

    let expected = "ala\"ma\nkota\tjana\\i\\szympansa";

    let token = lexer.generate_token().unwrap();
    assert_eq!(token.category, TokenCategory::StringValue);
    assert_eq!(token.value, TokenValue::String(expected.to_string()));
}

#[test]
fn numbers() {
    let text = "123 0 5 12.3 2.0 0.0";
    let mut lexer = create_lexer_with_skip(text);

    let expected: Vec<(TokenCategory, TokenValue)> = vec![
        (TokenCategory::I64Value, TokenValue::I64(123)),
        (TokenCategory::I64Value, TokenValue::I64(0)),
        (TokenCategory::I64Value, TokenValue::I64(5)),
        (TokenCategory::F64Value, TokenValue::F64(12.3)),
        (TokenCategory::F64Value, TokenValue::F64(2.0)),
        (TokenCategory::F64Value, TokenValue::F64(0.0)),
    ];

    for (category, value) in &expected {
        let token = lexer.generate_token().unwrap();
        assert_eq!(token.category, *category);
        assert_eq!(token.value, *value);
    }
}

#[test]
fn keyword_or_identifier() {
    let text = "fn for if else return i64 f64
        str void bool true false as switch break my_identifier1";
    let mut lexer = create_lexer_with_skip(text);

    let expected: Vec<(TokenCategory, TokenValue)> = vec![
        (TokenCategory::Fn, TokenValue::Null),
        (TokenCategory::For, TokenValue::Null),
        (TokenCategory::If, TokenValue::Null),
        (TokenCategory::Else, TokenValue::Null),
        (TokenCategory::Return, TokenValue::Null),
        (TokenCategory::I64, TokenValue::Null),
        (TokenCategory::F64, TokenValue::Null),
        (TokenCategory::String, TokenValue::Null),
        (TokenCategory::Void, TokenValue::Null),
        (TokenCategory::Bool, TokenValue::Null),
        (TokenCategory::True, TokenValue::Null),
        (TokenCategory::False, TokenValue::Null),
        (TokenCategory::As, TokenValue::Null),
        (TokenCategory::Switch, TokenValue::Null),
        (TokenCategory::Break, TokenValue::Null),
        (TokenCategory::Identifier, TokenValue::String("my_identifier1".to_owned())),
    ];

    for (category, value) in &expected {
        let token = lexer.generate_token().unwrap();
        assert_eq!(token.category, *category);
        assert_eq!(token.value, *value);
    }
}

#[test]
fn modulo_operators() {
    let text = "% %= ";
    let mut lexer = create_lexer_with_skip(text);
    let expected_tokens: Vec<TokenCategory> = vec![TokenCategory::Modulo, TokenCategory::ModuloEquals];

    for expected_token in &expected_tokens {
        let token = lexer.generate_token().unwrap();
        assert_eq!(token.category, *expected_token);
    }
}

#[test]
fn compound_assignment_operators() {
    let text = "+= -= *= /= ";
    let mut lexer = create_lexer_with_skip(text);
    let expected_tokens: Vec<TokenCategory> = vec![
        TokenCategory::PlusEquals,
        TokenCategory::MinusEquals,
        TokenCategory::TimesEquals,
        TokenCategory::DivideEquals,
    ];

    for expected_token in &expected_tokens {
        let token = lexer.generate_token().unwrap();
        assert_eq!(token.category, *expected_token);
    }
}

#[test]
fn continue_keyword() {
    let text = "continue";
    let mut lexer = create_lexer_with_skip(text);

    let token = lexer.generate_token().unwrap();
    assert_eq!(token.category, TokenCategory::Continue);
    assert_eq!(token.value, TokenValue::Null);
}

#[test]
fn digit_separator() {
    let text = "1'000'000";
    let mut lexer = create_lexer_with_skip(text);

    let token = lexer.generate_token().unwrap();
    assert_eq!(token.category, TokenCategory::I64Value);
    assert_eq!(token.value, TokenValue::I64(1_000_000));
}

#[test]
fn digit_separator_in_fraction() {
    let text = "1.2'3";
    let mut lexer = create_lexer_with_skip(text);

    let token = lexer.generate_token().unwrap();
    assert_eq!(token.category, TokenCategory::F64Value);
    assert_eq!(token.value, TokenValue::F64(1.23));
}

#[test]
fn escapes_extended() {
    let text = r#""a\rb\0c\ed""#;
    let mut lexer = create_lexer_with_skip(text);

    let expected = "a\rb\0c\x1bd";

    let token = lexer.generate_token().unwrap();
    assert_eq!(token.category, TokenCategory::StringValue);
    assert_eq!(token.value, TokenValue::String(expected.to_string()));
}

#[cfg(test)]
mod edge_case_tests {
    use std::io::BufReader;

    use crate::{
        common::errors::IError,
        frontend::{
            lexer::{
                lazy_stream_reader::LazyStreamReader,
                lexer::{Lexer, LexerOptions},
            },
            tokens::{TokenCategory, TokenValue},
        },
    };

    fn on_warning(warning: Box<dyn IError>) {
        println!("{}", warning.message());
    }

    fn create_lexer(text: &str) -> Lexer {
        let owned_text: &'static str = Box::leak(text.to_owned().into_boxed_str());
        let code = BufReader::new(owned_text.as_bytes());
        let reader = LazyStreamReader::new(code, None);

        let lexer_options = LexerOptions {
            max_comment_length: 100,
            max_identifier_length: 20,
        };

        let lexer = Lexer::new_unsafe(reader, lexer_options, on_warning).unwrap();

        lexer
    }

    fn create_lexer_with_skip(text: &str) -> Lexer {
        let mut lexer = create_lexer(text);
        let _ = lexer.generate_token().unwrap();

        lexer
    }

    #[test]
    fn too_long_comment() {
        let chars = "a".repeat(150);
        let text = format!("# {}", chars);
        let mut lexer = create_lexer_with_skip(text.as_str());

        let result = lexer.generate_token();
        assert!(result.is_err());
    }

    #[test]
    fn too_long_identifier() {
        let text = "a".repeat(30);
        let mut lexer = create_lexer_with_skip(text.as_str());

        let result = lexer.generate_token();
        assert!(result.is_err());
    }

    #[test]
    fn extend_to_next_or_warning() {
        let text = "|";
        let mut lexer = create_lexer_with_skip(text);

        let result = lexer.generate_token();
        assert_eq!(result.unwrap().category, TokenCategory::Or);
    }

    #[test]
    fn newline_in_string() {
        let text = r#""my
        string""#;
        let mut lexer = create_lexer_with_skip(text);

        let result = lexer.generate_token();
        assert!(result.is_err());
    }

    #[test]
    fn string_unclosed() {
        let text = r#""my_string"#;
        let mut lexer = create_lexer_with_skip(text);

        let result = lexer.generate_token();
        assert_eq!(result.unwrap().category, TokenCategory::StringValue);
    }

    #[test]
    fn int_overflow() {
        // 1 more than limit
        let text = "9223372036854775808";
        let mut lexer = create_lexer_with_skip(text);

        let result = lexer.generate_token();
        assert!(result.is_err());
    }

    #[test]
    fn disallow_zero_prefix() {
        let text = "007";
        let mut lexer = create_lexer_with_skip(text);

        let result = lexer.generate_token();
        assert!(result.is_err());
    }

    #[test]
    fn digit_separator_not_between_digits_leading() {
        let text = "1' ";
        let mut lexer = create_lexer_with_skip(text);

        let result = lexer.generate_token();
        assert!(result.is_err());
    }

    #[test]
    fn digit_separator_not_between_digits_in_fraction() {
        let text = "1.2' ";
        let mut lexer = create_lexer_with_skip(text);

        let result = lexer.generate_token();
        assert!(result.is_err());
    }

    #[test]
    fn char_literal_simple() {
        let text = "'a' 'Z' '0'";
        let mut lexer = create_lexer_with_skip(text);
        for expected in ['a', 'Z', '0'] {
            let token = lexer.generate_token().unwrap();
            assert_eq!(token.category, TokenCategory::CharValue);
            assert_eq!(token.value, TokenValue::Char(expected));
        }
    }

    #[test]
    fn char_literal_escapes() {
        // note: '\'' is not valid in the current implementation (escape map has no '\''),
        // so we only test the supported ones
        let text = r#"'\n' '\t' '\\' '\0' '\e' '\r'"#;
        let mut lexer = create_lexer_with_skip(text);
        let expected = ['\n', '\t', '\\', '\0', '\x1b', '\r'];
        for ch in expected {
            let token = lexer.generate_token().unwrap();
            assert_eq!(token.category, TokenCategory::CharValue);
            assert_eq!(token.value, TokenValue::Char(ch));
        }
    }

    #[test]
    fn char_literal_empty_fails() {
        let text = "''";
        let mut lexer = create_lexer_with_skip(text);
        assert!(lexer.generate_token().is_err());
    }

    #[test]
    fn char_literal_newline_fails() {
        let text = "'\n'";
        let mut lexer = create_lexer_with_skip(text);
        assert!(lexer.generate_token().is_err());
    }

    #[test]
    fn char_literal_unclosed_eof_fails() {
        let text = "'a";
        let mut lexer = create_lexer_with_skip(text);
        assert!(lexer.generate_token().is_err());
    }

    #[test]
    fn char_literal_too_long_fails() {
        let text = "'ab'";
        let mut lexer = create_lexer_with_skip(text);
        assert!(lexer.generate_token().is_err());
    }

    #[test]
    fn char_literal_invalid_escape_fails() {
        let text = r#"'\q'"#;
        let mut lexer = create_lexer_with_skip(text);
        assert!(lexer.generate_token().is_err());
    }

    #[test]
    fn remaining_keywords() {
        let text = "while else i8 i16 i32 u8 u16 u32 u64 char extern";
        let mut lexer = create_lexer_with_skip(text);
        let expected = [
            TokenCategory::While,
            TokenCategory::Else,
            TokenCategory::I8,
            TokenCategory::I16,
            TokenCategory::I32,
            TokenCategory::U8,
            TokenCategory::U16,
            TokenCategory::U32,
            TokenCategory::U64,
            TokenCategory::Char,
            TokenCategory::Extern,
        ];
        for cat in expected {
            let token = lexer.generate_token().unwrap();
            assert_eq!(token.category, cat);
            assert_eq!(token.value, TokenValue::Null);
        }
    }

    #[test]
    fn identifier_with_underscores_and_digits() {
        let text = "my_var_1 _leading ok123";
        let mut lexer = create_lexer_with_skip(text);
        // note: identifiers must start with alphabetic, so "_leading" is NOT an identifier
        let token = lexer.generate_token().unwrap();
        assert_eq!(token.category, TokenCategory::Identifier);
        assert_eq!(token.value, TokenValue::String("my_var_1".into()));

        // "_leading" starts with '_' → unexpected token
        assert!(lexer.generate_token().is_err());
    }

    #[test]
    fn identifier_starting_with_letter_then_underscore() {
        let text = "a_b_c";
        let mut lexer = create_lexer_with_skip(text);
        let token = lexer.generate_token().unwrap();
        assert_eq!(token.category, TokenCategory::Identifier);
        assert_eq!(token.value, TokenValue::String("a_b_c".into()));
    }

    #[test]
    fn skips_all_whitespace() {
        let text = " \t\n\r  42";
        let mut lexer = create_lexer_with_skip(text);
        let token = lexer.generate_token().unwrap();
        assert_eq!(token.category, TokenCategory::I64Value);
        assert_eq!(token.value, TokenValue::I64(42));
    }

    #[test]
    fn unexpected_character_fails() {
        let text = "@";
        let mut lexer = create_lexer_with_skip(text);
        assert!(lexer.generate_token().is_err());
    }

    #[test]
    fn float_with_only_fraction_after_dot() {
        let text = "0.5 3.1415";
        let mut lexer = create_lexer_with_skip(text);
        let t1 = lexer.generate_token().unwrap();
        assert_eq!(t1.category, TokenCategory::F64Value);
        assert_eq!(t1.value, TokenValue::F64(0.5));
        let t2 = lexer.generate_token().unwrap();
        assert_eq!(t2.category, TokenCategory::F64Value);
        assert_eq!(t2.value, TokenValue::F64(3.1415));
    }

    #[test]
    fn comment_then_etx() {
        let text = "# only a comment";
        let mut lexer = create_lexer_with_skip(text);
        let token = lexer.generate_token().unwrap();
        assert_eq!(token.category, TokenCategory::ETX);
    }
}
