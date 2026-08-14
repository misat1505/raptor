use std::fs::File;
use std::io::BufReader;
use std::vec;

use phf::phf_map;

use crate::errors::{ErrorSeverity, IError, LexerError};
use crate::lazy_stream_reader::{ILazyStreamReader, LazyStreamReader, Position, ETX};
use crate::tokens::{Token, TokenCategory, TokenValue};

pub struct LexerOptions {
    pub max_comment_length: u32,
    pub max_identifier_length: u32,
}

pub trait ILexer {
    fn current(&self) -> &Option<Token>;
    fn next(&mut self) -> Result<Token, Box<dyn IError>>;
}

pub struct Lexer {
    pub src: Vec<Box<dyn ILazyStreamReader>>,
    imported_paths: Vec<String>,
    import_stack: Vec<String>,
    current: Option<Token>,
    position: Position,
    options: LexerOptions,
    on_warning: fn(warning: Box<dyn IError>),
}

impl ILexer for Lexer {
    fn current(&self) -> &Option<Token> {
        &self.current
    }

    fn next(&mut self) -> Result<Token, Box<dyn IError>> {
        self.generate_token()
    }
}

impl Lexer {
    pub fn new(
        src: impl ILazyStreamReader + 'static,
        options: LexerOptions,
        on_warning: fn(warning: Box<dyn IError>),
    ) -> Result<Self, Box<dyn IError>> {
        let position = src.position().clone();
        let mut lexer = Lexer {
            src: vec![Box::new(src)],
            imported_paths: vec![String::from(position.filename.unwrap_or("<input>"))],
            import_stack: vec![String::from(position.filename.unwrap_or("<input>"))],
            current: None,
            position,
            options,
            on_warning,
        };

        lexer.generate_token()?;
        lexer.generate_token()?;

        Ok(lexer)
    }

    #[allow(dead_code)]
    pub fn new_unsafe(
        src: impl ILazyStreamReader + 'static,
        options: LexerOptions,
        on_warning: fn(warning: Box<dyn IError>),
    ) -> Result<Self, Box<dyn IError>> {
        let position = src.position().clone();
        let lexer = Lexer {
            src: vec![Box::new(src)],
            imported_paths: vec![String::from(position.filename.unwrap_or("<input>"))],
            import_stack: vec![String::from(position.filename.unwrap_or("<input>"))],
            current: None,
            position,
            options,
            on_warning,
        };

        Ok(lexer)
    }

    fn token_text(token: &Token) -> String {
        match &token.value {
            TokenValue::F64(value) => value.to_string(),
            TokenValue::I64(value) => value.to_string(),
            TokenValue::String(value) => value.clone(),
            TokenValue::Null => format!("{:?}", token.category),
        }
    }

    fn consume_must_be(&mut self, category: TokenCategory) -> Result<Token, Box<dyn IError>> {
        let position = self.position.clone();
        let current_token = self
            .current
            .take()
            .ok_or_else(|| Box::new(LexerError::at(ErrorSeverity::HIGH, "Expected a token".to_string(), position)) as Box<dyn IError>)?;

        if current_token.category == category {
            self.generate_token()?;
            return Ok(current_token);
        }

        Err(Box::new(LexerError::expected_found(
            ErrorSeverity::HIGH,
            "Unexpected token".to_string(),
            format!("{:?}", category),
            Self::token_text(&current_token),
            current_token.position,
        )))
    }

    fn import_file(&mut self) -> Result<Token, Box<dyn IError>> {
        let _ = self.consume_must_be(TokenCategory::Import)?;
        let path_token = self.consume_must_be(TokenCategory::StringValue)?;

        let path = match path_token.value {
            TokenValue::String(p) => p,
            v => {
                return Err(Box::new(LexerError::expected_found(
                    ErrorSeverity::HIGH,
                    "Unexpected token value".to_string(),
                    format!("string"),
                    format!("{:?}", v),
                    path_token.position,
                )))
            }
        };

        let is_in_import_stack = self.import_stack.iter().find(|v| **v == path).is_some();
        if is_in_import_stack {
            let import_stack = self.import_stack.join("\n    ↓\n    ");
            return Err(Box::new(LexerError::at(
                ErrorSeverity::HIGH,
                format!("Cyclic import detected:\n    {}\n    ↓\n    {}\n\n", import_stack, path),
                path_token.position,
            )));
        }

        let is_already_imported = self.imported_paths.iter().find(|v| **v == path).is_some();
        if !is_already_imported {
            let file = match File::open(path.as_str()) {
                Ok(f) => f,
                Err(_) => {
                    return Err(Box::new(LexerError::at(
                        ErrorSeverity::HIGH,
                        format!("File '{}' not found.", path),
                        path_token.position,
                    )));
                }
            };

            let code = BufReader::new(file);
            let filename: &'static str = Box::leak(path.clone().into_boxed_str());
            self.src.push(Box::new(LazyStreamReader::new(code, Some(filename))));

            self.import_stack.push(path.clone());
            self.imported_paths.push(path);
        }

        let _ = self.src.last_mut().unwrap().next();
        self.consume_must_be(TokenCategory::Semicolon)
    }

    fn handle_etx(&mut self) -> Result<Token, Box<dyn IError>> {
        if self.src.len() > 1 {
            self.src.pop();
            self.import_stack.pop();
            return self.generate_token();
        }
        return Ok(self.current.clone().unwrap());
    }

    #[allow(irrefutable_let_patterns)]
    pub fn generate_token(&mut self) -> Result<Token, Box<dyn IError>> {
        self.skip_whitespaces();
        self.position = self.src.last().unwrap().position().clone();

        let result_methods = [
            Self::try_generating_sign,
            Self::try_generating_operator,
            Self::try_generating_comment,
            Self::try_generating_string,
            Self::try_generating_number,
            Self::try_creating_identifier_or_keyword,
        ];

        for generator in &result_methods {
            if let Some(token) = generator(self)? {
                match token.category {
                    TokenCategory::Import => {
                        self.current = Some(token);
                        return self.import_file();
                    }
                    TokenCategory::ETX => {
                        self.current = Some(token);
                        return self.handle_etx();
                    }
                    TokenCategory::Comment => return self.generate_token(),
                    _ => {
                        self.current = Some(token.clone());
                        return Ok(token);
                    }
                }
            }
        }

        Err(self.create_lexer_error(String::from("Unexpected token")))
    }

    fn skip_whitespaces(&mut self) {
        while self.src.last().unwrap().current().is_whitespace() {
            let _ = self.src.last_mut().unwrap().next();
        }
    }

    fn try_generating_comment(&mut self) -> Result<Option<Token>, Box<dyn IError>> {
        let current_char = self.src.last().unwrap().current();
        if *current_char != '#' {
            return Ok(None);
        }

        let mut comment = String::new();
        while let Ok(current) = self.src.last_mut().unwrap().next() {
            if *current == '\n' || *current == ETX {
                break;
            }
            if (comment.len() as u32) == self.options.max_comment_length {
                return Err(self.create_lexer_error(format!("Comment too long. Max comment length: {}", self.options.max_comment_length)));
            }
            comment.push(*current);
        }

        Ok(Some(Token {
            category: TokenCategory::Comment,
            value: TokenValue::String(comment),
            position: self.position,
        }))
    }

    fn try_generating_sign(&mut self) -> Result<Option<Token>, Box<dyn IError>> {
        let current_char = self.src.last().unwrap().current();
        match SIGNS.get(current_char) {
            None => Ok(None),
            Some(token_category) => {
                let token = Token {
                    category: token_category.clone(),
                    value: TokenValue::Null,
                    position: self.position,
                };
                let _ = self.src.last_mut().unwrap().next();
                Ok(Some(token))
            }
        }
    }

    fn try_generating_operator(&mut self) -> Result<Option<Token>, Box<dyn IError>> {
        let current_char = self.src.last().unwrap().current();
        let token = match current_char {
            '+' => Some(self.extend_to_next('=', TokenCategory::Plus, TokenCategory::PlusEquals)),
            '-' => Some(self.extend_minus()),
            '*' => Some(self.extend_to_next('=', TokenCategory::Multiply, TokenCategory::TimesEquals)),
            '/' => Some(self.extend_to_next('=', TokenCategory::Divide, TokenCategory::DivideEquals)),
            '%' => Some(self.extend_to_next('=', TokenCategory::Modulo, TokenCategory::ModuloEquals)),
            '<' => Some(self.extend_to_next('=', TokenCategory::Less, TokenCategory::LessOrEqual)),
            '>' => Some(self.extend_to_next('=', TokenCategory::Greater, TokenCategory::GreaterOrEqual)),
            '!' => Some(self.extend_to_next('=', TokenCategory::Negate, TokenCategory::NotEqual)),
            '=' => Some(self.extend_to_next('=', TokenCategory::Assign, TokenCategory::Equal)),
            '&' => Some(self.extend_to_next('&', TokenCategory::Reference, TokenCategory::And)),
            '|' => Some(self.extend_to_next_or_warning('|', TokenCategory::Or)),
            _ => None,
        };
        Ok(token)
    }

    // this is not used but might be helpful in the future
    #[allow(dead_code)]
    fn single_char(&mut self, category: TokenCategory) -> Token {
        let _ = self.src.last_mut().unwrap().next();
        Token {
            category,
            value: TokenValue::Null,
            position: self.position,
        }
    }

    fn extend_to_next(&mut self, char_to_search: char, not_found: TokenCategory, found: TokenCategory) -> Token {
        let next_char = self.src.last_mut().unwrap().next().unwrap();
        if *next_char == char_to_search {
            let _ = self.src.last_mut().unwrap().next();
            return Token {
                category: found,
                value: TokenValue::Null,
                position: self.position,
            };
        }
        return Token {
            category: not_found,
            value: TokenValue::Null,
            position: self.position,
        };
    }

    fn extend_to_next_or_warning(&mut self, char_to_search: char, found: TokenCategory) -> Token {
        let next_char = self.src.last_mut().unwrap().next().unwrap();
        if *next_char == char_to_search {
            let _ = self.src.last_mut().unwrap().next();
        } else {
            (self.on_warning)(Box::new(LexerError::new(
                ErrorSeverity::LOW,
                self.prepare_warning_message(format!("Expected '{}'", char_to_search)),
            )));
        }
        return Token {
            category: found,
            value: TokenValue::Null,
            position: self.position,
        };
    }

    fn extend_minus(&mut self) -> Token {
        let next_char = self.src.last_mut().unwrap().next().unwrap();
        if *next_char == '>' {
            let _ = self.src.last_mut().unwrap().next();
            return Token {
                category: TokenCategory::Arrow,
                value: TokenValue::Null,
                position: self.position,
            };
        }
        if *next_char == '=' {
            let _ = self.src.last_mut().unwrap().next();
            return Token {
                category: TokenCategory::MinusEquals,
                value: TokenValue::Null,
                position: self.position,
            };
        }
        Token {
            category: TokenCategory::Minus,
            value: TokenValue::Null,
            position: self.position,
        }
    }

    fn try_generating_string(&mut self) -> Result<Option<Token>, Box<dyn IError>> {
        let mut current_char = self.src.last().unwrap().current().clone();
        if current_char != '"' {
            return Ok(None);
        }
        let mut created_string = String::new();
        current_char = self.src.last_mut().unwrap().next().unwrap().clone();
        while current_char != '"' {
            // escaping
            if current_char == '\\' {
                let next_char = self.src.last_mut().unwrap().next().unwrap().clone();
                match ESCAPES.get(&next_char) {
                    Some(char) => {
                        created_string.push(*char);
                        current_char = *self.src.last_mut().unwrap().next().unwrap();
                        continue;
                    }
                    None => {
                        (self.on_warning)(Box::new(LexerError::new(
                            ErrorSeverity::LOW,
                            self.prepare_warning_message(format!("Invalid escape symbol detected '\\{}'", next_char)),
                        )));
                        let default_escape = '\\';
                        created_string.push(default_escape);
                        current_char = next_char;
                        continue;
                    }
                }
            }
            if current_char == '\n' {
                return Err(self.create_lexer_error(String::from("Unexpected newline in string")));
            }
            if current_char == ETX {
                (self.on_warning)(Box::new(LexerError::new(
                    ErrorSeverity::LOW,
                    self.prepare_warning_message(String::from("String not closed")),
                )));
                return Ok(Some(Token {
                    category: TokenCategory::StringValue,
                    value: TokenValue::String(created_string),
                    position: self.position,
                }));
            }
            created_string.push(current_char);
            current_char = self.src.last_mut().unwrap().next().unwrap().clone();
        }
        // consume next "
        let _ = self.src.last_mut().unwrap().next();
        Ok(Some(Token {
            category: TokenCategory::StringValue,
            value: TokenValue::String(created_string),
            position: self.position,
        }))
    }

    fn try_generating_number(&mut self) -> Result<Option<Token>, Box<dyn IError>> {
        let mut current_char = self.src.last().unwrap().current().clone();
        if !current_char.is_ascii_digit() {
            return Ok(None);
        }

        let mut decimal = 0;
        if current_char != '0' {
            (decimal, _) = self.parse_integer()?;
        } else {
            let next_char = self.src.last_mut().unwrap().next().unwrap();
            if next_char.is_ascii_digit() {
                return Err(self.create_lexer_error(String::from("Cannot prefix number with 0's.")));
            }
        }

        current_char = self.src.last().unwrap().current().clone();
        if current_char != '.' {
            return Ok(Some(Token {
                category: TokenCategory::I64Value,
                value: TokenValue::I64(decimal),
                position: self.position,
            }));
        }

        let _ = self.src.last_mut().unwrap().next();
        let (fraction, fraction_length) = self.parse_integer()?;
        let float_value = Self::merge_to_float(decimal, fraction, fraction_length);
        Ok(Some(Token {
            category: TokenCategory::F64Value,
            value: TokenValue::F64(float_value),
            position: self.position,
        }))
    }

    fn parse_integer(&mut self) -> Result<(i64, i64), Box<dyn IError>> {
        let mut current_char = self.src.last().unwrap().current();
        let mut length = 0;
        let mut total: i64 = 0;

        loop {
            if current_char.is_ascii_digit() {
                let digit = *current_char as i64 - '0' as i64;
                total = total
                    .checked_mul(10)
                    .ok_or_else(|| self.create_lexer_error(String::from("Overflow occurred while parsing integer")))?;
                total = total
                    .checked_add(digit)
                    .ok_or_else(|| self.create_lexer_error(String::from("Overflow occurred while parsing integer")))?;
                length += 1;
                current_char = self.src.last_mut().unwrap().next().unwrap();
                continue;
            }

            if *current_char == '\'' {
                let next_char = self.src.last_mut().unwrap().next().unwrap();
                if !next_char.is_ascii_digit() {
                    return Err(self.create_lexer_error(String::from("Digit separator ' must be placed between two digits")));
                }
                current_char = next_char;
                continue;
            }

            break;
        }
        Ok((total, length))
    }

    fn merge_to_float(decimal: i64, fraction: i64, fraction_length: i64) -> f64 {
        let fraction_value = fraction as f64 / f64::powi(10.0, fraction_length as i32);
        let float_value = decimal as f64 + fraction_value;
        float_value
    }

    fn try_creating_identifier_or_keyword(&mut self) -> Result<Option<Token>, Box<dyn IError>> {
        let mut current_char = self.src.last().unwrap().current().clone();
        if !current_char.is_ascii_alphabetic() {
            return Ok(None);
        }
        let mut created_string = String::new();
        while current_char.is_ascii_digit() || current_char.is_ascii_alphabetic() || current_char == '_' {
            if (created_string.len() as u32) == self.options.max_identifier_length {
                return Err(self.create_lexer_error(format!(
                    "Identifier name too long. Max identifier length: {}",
                    self.options.max_identifier_length
                )));
            }
            created_string.push(current_char);
            current_char = self.src.last_mut().unwrap().next().unwrap().clone();
        }
        match KEYWORDS.get(created_string.as_str()) {
            Some(category) => Ok(Some(Token {
                category: category.clone(),
                value: TokenValue::Null,
                position: self.position,
            })),
            None => Ok(Some(Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String(created_string),
                position: self.position,
            })),
        }
    }

    fn create_lexer_error(&mut self, text: String) -> Box<dyn IError> {
        let position = self.src.last().unwrap().position();
        Box::new(LexerError::at(ErrorSeverity::HIGH, text, position))
    }

    fn prepare_warning_message(&self, text: String) -> String {
        let position = self.src.last().unwrap().position();
        let error = LexerError::at(ErrorSeverity::LOW, text, position);
        return error.message();
    }
}

static SIGNS: phf::Map<char, TokenCategory> = phf_map! {
    '('     => TokenCategory::ParenOpen,
    ')'     => TokenCategory::ParenClose,
    '['     => TokenCategory::BracketOpen,
    ']'     => TokenCategory::BracketClose,
    '{'     => TokenCategory::BraceOpen,
    '}'     => TokenCategory::BraceClose,
    ';'     => TokenCategory::Semicolon,
    ':'     => TokenCategory::Colon,
    ','     => TokenCategory::Comma,
    '\u{2}' => TokenCategory::STX,
    '\u{3}' => TokenCategory::ETX,

};

static KEYWORDS: phf::Map<&'static str, TokenCategory> = phf_map! {
    "fn" => TokenCategory::Fn,
    "for" => TokenCategory::For,
    "while" => TokenCategory::While,
    "if" => TokenCategory::If,
    "else" => TokenCategory::Else,
    "return" => TokenCategory::Return,
    "i64" => TokenCategory::I64,
    "f64" => TokenCategory::F64,
    "str" => TokenCategory::String,
    "void" => TokenCategory::Void,
    "bool" => TokenCategory::Bool,
    "true" => TokenCategory::True,
    "false" => TokenCategory::False,
    "as" => TokenCategory::As,
    "switch" => TokenCategory::Switch,
    "break" => TokenCategory::Break,
    "continue" => TokenCategory::Continue,
    "import" => TokenCategory::Import,
    "extern" => TokenCategory::Extern,
};

static ESCAPES: phf::Map<char, char> = phf_map! {
    'n'  => '\n',
    'r'  => '\r',
    't'  => '\t',
    '"'  => '"',
    '\\' => '\\',
    'e'  => '\x1b',
    '0'  => '\0',
};
