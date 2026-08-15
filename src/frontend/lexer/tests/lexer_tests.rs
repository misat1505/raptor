#[cfg(test)]
mod tests {
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

    #[test]
    fn import_nonexistent_file_fails() {
        let text = r#"import "this_file_does_not_exist_12345.rp";"#;
        let mut lexer = create_lexer_with_skip(text);

        let result = lexer.generate_token();
        assert!(result.is_err());
    }

    #[test]
    fn cyclic_import_detected() {
        use std::fs;
        use std::io::Write;

        let path_a = std::env::temp_dir().join(format!("rp_lexer_cyclic_a_{}.rp", std::process::id()));
        let path_b = std::env::temp_dir().join(format!("rp_lexer_cyclic_b_{}.rp", std::process::id()));

        {
            let mut file_a = fs::File::create(&path_a).unwrap();
            writeln!(file_a, r#"import "{}";"#, path_b.to_str().unwrap()).unwrap();
        }
        {
            let mut file_b = fs::File::create(&path_b).unwrap();
            writeln!(file_b, r#"import "{}";"#, path_a.to_str().unwrap()).unwrap();
        }

        let text = format!(r#"import "{}";"#, path_a.to_str().unwrap());
        let mut lexer = create_lexer_with_skip(&text);

        let mut result: Result<TokenCategory, Box<dyn IError>> = Ok(TokenCategory::STX);
        for _ in 0..10 {
            match lexer.generate_token() {
                Ok(t) => result = Ok(t.category),
                Err(e) => {
                    assert!(e.message().contains("Cyclic import"));
                    let _ = fs::remove_file(&path_a);
                    let _ = fs::remove_file(&path_b);
                    return;
                }
            }
        }

        let _ = fs::remove_file(&path_a);
        let _ = fs::remove_file(&path_b);
        panic!("Expected cyclic import error, got: {:?}", result);
    }
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
            tokens::TokenCategory,
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
}
