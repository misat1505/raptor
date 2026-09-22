fn main() {
    let foo = TokenCapture {
        tokens: [
            Token {
                category: TokenCategory::STX,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 0,
                        column: 0,
                        offset: 0,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 1,
                        column: 1,
                        offset: 0,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Import,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 1,
                        column: 1,
                        offset: 0,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 1,
                        column: 7,
                        offset: 6,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::StringValue,
                value: TokenValue::String("./frontend/lexer/lazy-stream-reader.rp"),
                span: Span {
                    start: Position {
                        line: 1,
                        column: 8,
                        offset: 7,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 1,
                        column: 48,
                        offset: 47,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Semicolon,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 1,
                        column: 48,
                        offset: 47,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 1,
                        column: 49,
                        offset: 48,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Import,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 2,
                        column: 1,
                        offset: 50,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 2,
                        column: 7,
                        offset: 56,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::StringValue,
                value: TokenValue::String("./frontend/lexer/lexer.rp"),
                span: Span {
                    start: Position {
                        line: 2,
                        column: 8,
                        offset: 57,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 2,
                        column: 35,
                        offset: 84,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Semicolon,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 2,
                        column: 35,
                        offset: 84,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 2,
                        column: 36,
                        offset: 85,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Struct,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 4,
                        column: 1,
                        offset: 89,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 4,
                        column: 7,
                        offset: 95,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("TokenCapture"),
                span: Span {
                    start: Position {
                        line: 4,
                        column: 8,
                        offset: 96,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 4,
                        column: 20,
                        offset: 108,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Derives,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 4,
                        column: 21,
                        offset: 109,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 4,
                        column: 28,
                        offset: 116,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("Debug"),
                span: Span {
                    start: Position {
                        line: 4,
                        column: 29,
                        offset: 117,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 4,
                        column: 34,
                        offset: 122,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::BraceOpen,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 4,
                        column: 35,
                        offset: 123,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 4,
                        column: 36,
                        offset: 124,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("Token"),
                span: Span {
                    start: Position {
                        line: 5,
                        column: 5,
                        offset: 130,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 5,
                        column: 10,
                        offset: 135,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::BracketOpen,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 5,
                        column: 10,
                        offset: 135,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 5,
                        column: 11,
                        offset: 136,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::BracketClose,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 5,
                        column: 11,
                        offset: 136,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 5,
                        column: 12,
                        offset: 137,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("tokens"),
                span: Span {
                    start: Position {
                        line: 5,
                        column: 13,
                        offset: 138,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 5,
                        column: 19,
                        offset: 144,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::BraceClose,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 6,
                        column: 1,
                        offset: 146,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 6,
                        column: 2,
                        offset: 147,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Semicolon,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 6,
                        column: 2,
                        offset: 147,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 6,
                        column: 3,
                        offset: 148,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Fn,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 8,
                        column: 1,
                        offset: 152,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 8,
                        column: 3,
                        offset: 154,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("main"),
                span: Span {
                    start: Position {
                        line: 8,
                        column: 4,
                        offset: 155,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 8,
                        column: 8,
                        offset: 159,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::ParenOpen,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 8,
                        column: 8,
                        offset: 159,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 8,
                        column: 9,
                        offset: 160,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::ParenClose,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 8,
                        column: 9,
                        offset: 160,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 8,
                        column: 10,
                        offset: 161,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Colon,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 8,
                        column: 10,
                        offset: 161,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 8,
                        column: 11,
                        offset: 162,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Void,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 8,
                        column: 12,
                        offset: 163,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 8,
                        column: 16,
                        offset: 167,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::BraceOpen,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 8,
                        column: 17,
                        offset: 168,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 8,
                        column: 18,
                        offset: 169,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Let,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 9,
                        column: 5,
                        offset: 175,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 9,
                        column: 8,
                        offset: 178,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("myself"),
                span: Span {
                    start: Position {
                        line: 9,
                        column: 9,
                        offset: 179,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 9,
                        column: 15,
                        offset: 185,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Assign,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 9,
                        column: 16,
                        offset: 186,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 9,
                        column: 17,
                        offset: 187,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::StringValue,
                value: TokenValue::String("./selfhost/main.rp"),
                span: Span {
                    start: Position {
                        line: 9,
                        column: 18,
                        offset: 188,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 9,
                        column: 38,
                        offset: 208,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Semicolon,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 9,
                        column: 38,
                        offset: 208,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 9,
                        column: 39,
                        offset: 209,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Let,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 10,
                        column: 5,
                        offset: 215,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 10,
                        column: 8,
                        offset: 218,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("source"),
                span: Span {
                    start: Position {
                        line: 10,
                        column: 9,
                        offset: 219,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 10,
                        column: 15,
                        offset: 225,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Assign,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 10,
                        column: 16,
                        offset: 226,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 10,
                        column: 17,
                        offset: 227,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("read_file"),
                span: Span {
                    start: Position {
                        line: 10,
                        column: 18,
                        offset: 228,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 10,
                        column: 27,
                        offset: 237,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::ParenOpen,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 10,
                        column: 27,
                        offset: 237,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 10,
                        column: 28,
                        offset: 238,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("myself"),
                span: Span {
                    start: Position {
                        line: 10,
                        column: 28,
                        offset: 238,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 10,
                        column: 34,
                        offset: 244,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::ParenClose,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 10,
                        column: 34,
                        offset: 244,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 10,
                        column: 35,
                        offset: 245,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Semicolon,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 10,
                        column: 35,
                        offset: 245,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 10,
                        column: 36,
                        offset: 246,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Let,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 12,
                        column: 5,
                        offset: 254,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 12,
                        column: 8,
                        offset: 257,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("src"),
                span: Span {
                    start: Position {
                        line: 12,
                        column: 9,
                        offset: 258,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 12,
                        column: 12,
                        offset: 261,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Assign,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 12,
                        column: 13,
                        offset: 262,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 12,
                        column: 14,
                        offset: 263,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("lazy_stream_reader_new"),
                span: Span {
                    start: Position {
                        line: 12,
                        column: 15,
                        offset: 264,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 12,
                        column: 37,
                        offset: 286,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::ParenOpen,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 12,
                        column: 37,
                        offset: 286,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 12,
                        column: 38,
                        offset: 287,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("source"),
                span: Span {
                    start: Position {
                        line: 12,
                        column: 38,
                        offset: 287,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 12,
                        column: 44,
                        offset: 293,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Comma,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 12,
                        column: 44,
                        offset: 293,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 12,
                        column: 45,
                        offset: 294,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("OptionStr"),
                span: Span {
                    start: Position {
                        line: 12,
                        column: 46,
                        offset: 295,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 12,
                        column: 55,
                        offset: 304,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::DoubleColon,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 12,
                        column: 55,
                        offset: 304,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 12,
                        column: 57,
                        offset: 306,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("Some"),
                span: Span {
                    start: Position {
                        line: 12,
                        column: 57,
                        offset: 306,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 12,
                        column: 61,
                        offset: 310,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::ParenOpen,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 12,
                        column: 61,
                        offset: 310,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 12,
                        column: 62,
                        offset: 311,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("myself"),
                span: Span {
                    start: Position {
                        line: 12,
                        column: 62,
                        offset: 311,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 12,
                        column: 68,
                        offset: 317,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::ParenClose,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 12,
                        column: 68,
                        offset: 317,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 12,
                        column: 69,
                        offset: 318,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::ParenClose,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 12,
                        column: 69,
                        offset: 318,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 12,
                        column: 70,
                        offset: 319,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Semicolon,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 12,
                        column: 70,
                        offset: 319,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 12,
                        column: 71,
                        offset: 320,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Let,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 14,
                        column: 5,
                        offset: 328,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 14,
                        column: 8,
                        offset: 331,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("options"),
                span: Span {
                    start: Position {
                        line: 14,
                        column: 9,
                        offset: 332,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 14,
                        column: 16,
                        offset: 339,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Assign,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 14,
                        column: 17,
                        offset: 340,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 14,
                        column: 18,
                        offset: 341,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("LexerOptions"),
                span: Span {
                    start: Position {
                        line: 14,
                        column: 19,
                        offset: 342,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 14,
                        column: 31,
                        offset: 354,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::BraceOpen,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 14,
                        column: 32,
                        offset: 355,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 14,
                        column: 33,
                        offset: 356,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("max_comment_length"),
                span: Span {
                    start: Position {
                        line: 15,
                        column: 9,
                        offset: 366,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 15,
                        column: 27,
                        offset: 384,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Colon,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 15,
                        column: 27,
                        offset: 384,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 15,
                        column: 28,
                        offset: 385,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::I64Value,
                value: TokenValue::I64(1024),
                span: Span {
                    start: Position {
                        line: 15,
                        column: 29,
                        offset: 386,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 15,
                        column: 33,
                        offset: 390,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Comma,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 15,
                        column: 33,
                        offset: 390,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 15,
                        column: 34,
                        offset: 391,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("max_identifier_length"),
                span: Span {
                    start: Position {
                        line: 16,
                        column: 9,
                        offset: 401,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 16,
                        column: 30,
                        offset: 422,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Colon,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 16,
                        column: 30,
                        offset: 422,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 16,
                        column: 31,
                        offset: 423,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::I64Value,
                value: TokenValue::I64(256),
                span: Span {
                    start: Position {
                        line: 16,
                        column: 32,
                        offset: 424,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 16,
                        column: 35,
                        offset: 427,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::BraceClose,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 17,
                        column: 5,
                        offset: 433,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 17,
                        column: 6,
                        offset: 434,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Semicolon,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 17,
                        column: 6,
                        offset: 434,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 17,
                        column: 7,
                        offset: 435,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Let,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 19,
                        column: 5,
                        offset: 443,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 19,
                        column: 8,
                        offset: 446,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("lexer"),
                span: Span {
                    start: Position {
                        line: 19,
                        column: 9,
                        offset: 447,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 19,
                        column: 14,
                        offset: 452,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Assign,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 19,
                        column: 15,
                        offset: 453,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 19,
                        column: 16,
                        offset: 454,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("lexer_new"),
                span: Span {
                    start: Position {
                        line: 19,
                        column: 17,
                        offset: 455,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 19,
                        column: 26,
                        offset: 464,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::ParenOpen,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 19,
                        column: 26,
                        offset: 464,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 19,
                        column: 27,
                        offset: 465,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("src"),
                span: Span {
                    start: Position {
                        line: 19,
                        column: 27,
                        offset: 465,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 19,
                        column: 30,
                        offset: 468,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Comma,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 19,
                        column: 30,
                        offset: 468,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 19,
                        column: 31,
                        offset: 469,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("options"),
                span: Span {
                    start: Position {
                        line: 19,
                        column: 32,
                        offset: 470,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 19,
                        column: 39,
                        offset: 477,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::ParenClose,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 19,
                        column: 39,
                        offset: 477,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 19,
                        column: 40,
                        offset: 478,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Semicolon,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 19,
                        column: 40,
                        offset: 478,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 19,
                        column: 41,
                        offset: 479,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Let,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 21,
                        column: 5,
                        offset: 487,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 21,
                        column: 8,
                        offset: 490,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("token_capture"),
                span: Span {
                    start: Position {
                        line: 21,
                        column: 9,
                        offset: 491,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 21,
                        column: 22,
                        offset: 504,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Assign,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 21,
                        column: 23,
                        offset: 505,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 21,
                        column: 24,
                        offset: 506,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("TokenCapture"),
                span: Span {
                    start: Position {
                        line: 21,
                        column: 25,
                        offset: 507,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 21,
                        column: 37,
                        offset: 519,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::BraceOpen,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 21,
                        column: 38,
                        offset: 520,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 21,
                        column: 39,
                        offset: 521,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("tokens"),
                span: Span {
                    start: Position {
                        line: 21,
                        column: 40,
                        offset: 522,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 21,
                        column: 46,
                        offset: 528,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Colon,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 21,
                        column: 46,
                        offset: 528,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 21,
                        column: 47,
                        offset: 529,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::BracketOpen,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 21,
                        column: 48,
                        offset: 530,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 21,
                        column: 49,
                        offset: 531,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::BracketClose,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 21,
                        column: 49,
                        offset: 531,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 21,
                        column: 50,
                        offset: 532,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::BraceClose,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 21,
                        column: 51,
                        offset: 533,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 21,
                        column: 52,
                        offset: 534,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Semicolon,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 21,
                        column: 52,
                        offset: 534,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 21,
                        column: 53,
                        offset: 535,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::While,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 23,
                        column: 5,
                        offset: 543,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 23,
                        column: 10,
                        offset: 548,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::ParenOpen,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 23,
                        column: 11,
                        offset: 549,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 23,
                        column: 12,
                        offset: 550,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::True,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 23,
                        column: 12,
                        offset: 550,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 23,
                        column: 16,
                        offset: 554,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::ParenClose,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 23,
                        column: 16,
                        offset: 554,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 23,
                        column: 17,
                        offset: 555,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::BraceOpen,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 23,
                        column: 18,
                        offset: 556,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 23,
                        column: 19,
                        offset: 557,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Let,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 24,
                        column: 9,
                        offset: 567,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 24,
                        column: 12,
                        offset: 570,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("token"),
                span: Span {
                    start: Position {
                        line: 24,
                        column: 13,
                        offset: 571,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 24,
                        column: 18,
                        offset: 576,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Assign,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 24,
                        column: 19,
                        offset: 577,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 24,
                        column: 20,
                        offset: 578,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("lexer_next"),
                span: Span {
                    start: Position {
                        line: 24,
                        column: 21,
                        offset: 579,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 24,
                        column: 31,
                        offset: 589,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::ParenOpen,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 24,
                        column: 31,
                        offset: 589,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 24,
                        column: 32,
                        offset: 590,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Reference,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 24,
                        column: 32,
                        offset: 590,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 24,
                        column: 33,
                        offset: 591,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("lexer"),
                span: Span {
                    start: Position {
                        line: 24,
                        column: 33,
                        offset: 591,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 24,
                        column: 38,
                        offset: 596,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::ParenClose,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 24,
                        column: 38,
                        offset: 596,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 24,
                        column: 39,
                        offset: 597,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Semicolon,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 24,
                        column: 39,
                        offset: 597,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 24,
                        column: 40,
                        offset: 598,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Match,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 26,
                        column: 9,
                        offset: 610,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 26,
                        column: 14,
                        offset: 615,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::ParenOpen,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 26,
                        column: 15,
                        offset: 616,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 26,
                        column: 16,
                        offset: 617,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("token"),
                span: Span {
                    start: Position {
                        line: 26,
                        column: 16,
                        offset: 617,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 26,
                        column: 21,
                        offset: 622,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::ParenClose,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 26,
                        column: 21,
                        offset: 622,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 26,
                        column: 22,
                        offset: 623,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::BraceOpen,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 26,
                        column: 23,
                        offset: 624,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 26,
                        column: 24,
                        offset: 625,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("OptionToken"),
                span: Span {
                    start: Position {
                        line: 27,
                        column: 13,
                        offset: 639,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 27,
                        column: 24,
                        offset: 650,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::DoubleColon,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 27,
                        column: 24,
                        offset: 650,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 27,
                        column: 26,
                        offset: 652,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("Some"),
                span: Span {
                    start: Position {
                        line: 27,
                        column: 26,
                        offset: 652,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 27,
                        column: 30,
                        offset: 656,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::ParenOpen,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 27,
                        column: 30,
                        offset: 656,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 27,
                        column: 31,
                        offset: 657,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("value"),
                span: Span {
                    start: Position {
                        line: 27,
                        column: 31,
                        offset: 657,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 27,
                        column: 36,
                        offset: 662,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::ParenClose,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 27,
                        column: 36,
                        offset: 662,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 27,
                        column: 37,
                        offset: 663,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::BraceOpen,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 27,
                        column: 38,
                        offset: 664,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 27,
                        column: 39,
                        offset: 665,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("vector_push"),
                span: Span {
                    start: Position {
                        line: 28,
                        column: 17,
                        offset: 683,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 28,
                        column: 28,
                        offset: 694,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::ParenOpen,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 28,
                        column: 28,
                        offset: 694,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 28,
                        column: 29,
                        offset: 695,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Reference,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 28,
                        column: 29,
                        offset: 695,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 28,
                        column: 30,
                        offset: 696,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("token_capture"),
                span: Span {
                    start: Position {
                        line: 28,
                        column: 30,
                        offset: 696,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 28,
                        column: 43,
                        offset: 709,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Dot,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 28,
                        column: 43,
                        offset: 709,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 28,
                        column: 44,
                        offset: 710,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("tokens"),
                span: Span {
                    start: Position {
                        line: 28,
                        column: 44,
                        offset: 710,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 28,
                        column: 50,
                        offset: 716,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Comma,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 28,
                        column: 50,
                        offset: 716,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 28,
                        column: 51,
                        offset: 717,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("value"),
                span: Span {
                    start: Position {
                        line: 28,
                        column: 52,
                        offset: 718,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 28,
                        column: 57,
                        offset: 723,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::ParenClose,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 28,
                        column: 57,
                        offset: 723,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 28,
                        column: 58,
                        offset: 724,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Semicolon,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 28,
                        column: 58,
                        offset: 724,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 28,
                        column: 59,
                        offset: 725,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::If,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 30,
                        column: 17,
                        offset: 745,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 30,
                        column: 19,
                        offset: 747,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::ParenOpen,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 30,
                        column: 20,
                        offset: 748,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 30,
                        column: 21,
                        offset: 749,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("value"),
                span: Span {
                    start: Position {
                        line: 30,
                        column: 21,
                        offset: 749,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 30,
                        column: 26,
                        offset: 754,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Dot,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 30,
                        column: 26,
                        offset: 754,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 30,
                        column: 27,
                        offset: 755,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("category"),
                span: Span {
                    start: Position {
                        line: 30,
                        column: 27,
                        offset: 755,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 30,
                        column: 35,
                        offset: 763,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Matches,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 30,
                        column: 36,
                        offset: 764,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 30,
                        column: 43,
                        offset: 771,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("TokenCategory"),
                span: Span {
                    start: Position {
                        line: 30,
                        column: 44,
                        offset: 772,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 30,
                        column: 57,
                        offset: 785,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::DoubleColon,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 30,
                        column: 57,
                        offset: 785,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 30,
                        column: 59,
                        offset: 787,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("ETX"),
                span: Span {
                    start: Position {
                        line: 30,
                        column: 59,
                        offset: 787,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 30,
                        column: 62,
                        offset: 790,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::ParenClose,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 30,
                        column: 62,
                        offset: 790,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 30,
                        column: 63,
                        offset: 791,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::BraceOpen,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 30,
                        column: 64,
                        offset: 792,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 30,
                        column: 65,
                        offset: 793,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Break,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 31,
                        column: 21,
                        offset: 815,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 31,
                        column: 26,
                        offset: 820,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Semicolon,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 31,
                        column: 26,
                        offset: 820,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 31,
                        column: 27,
                        offset: 821,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::BraceClose,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 32,
                        column: 17,
                        offset: 839,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 32,
                        column: 18,
                        offset: 840,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::BraceClose,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 33,
                        column: 13,
                        offset: 854,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 33,
                        column: 14,
                        offset: 855,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Comma,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 33,
                        column: 14,
                        offset: 855,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 33,
                        column: 15,
                        offset: 856,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("OptionToken"),
                span: Span {
                    start: Position {
                        line: 35,
                        column: 13,
                        offset: 872,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 35,
                        column: 24,
                        offset: 883,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::DoubleColon,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 35,
                        column: 24,
                        offset: 883,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 35,
                        column: 26,
                        offset: 885,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("None"),
                span: Span {
                    start: Position {
                        line: 35,
                        column: 26,
                        offset: 885,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 35,
                        column: 30,
                        offset: 889,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::BraceOpen,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 35,
                        column: 31,
                        offset: 890,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 35,
                        column: 32,
                        offset: 891,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("println"),
                span: Span {
                    start: Position {
                        line: 36,
                        column: 17,
                        offset: 909,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 36,
                        column: 24,
                        offset: 916,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::ParenOpen,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 36,
                        column: 24,
                        offset: 916,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 36,
                        column: 25,
                        offset: 917,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::StringValue,
                value: TokenValue::String("LEXER ERROR / UNKNOWN TOKEN"),
                span: Span {
                    start: Position {
                        line: 36,
                        column: 25,
                        offset: 917,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 36,
                        column: 54,
                        offset: 946,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::ParenClose,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 36,
                        column: 54,
                        offset: 946,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 36,
                        column: 55,
                        offset: 947,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Semicolon,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 36,
                        column: 55,
                        offset: 947,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 36,
                        column: 56,
                        offset: 948,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Break,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 37,
                        column: 17,
                        offset: 966,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 37,
                        column: 22,
                        offset: 971,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Semicolon,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 37,
                        column: 22,
                        offset: 971,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 37,
                        column: 23,
                        offset: 972,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::BraceClose,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 38,
                        column: 13,
                        offset: 986,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 38,
                        column: 14,
                        offset: 987,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::BraceClose,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 39,
                        column: 9,
                        offset: 997,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 39,
                        column: 10,
                        offset: 998,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::BraceClose,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 40,
                        column: 5,
                        offset: 1004,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 40,
                        column: 6,
                        offset: 1005,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Let,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 43,
                        column: 5,
                        offset: 1115,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 43,
                        column: 8,
                        offset: 1118,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("output_path"),
                span: Span {
                    start: Position {
                        line: 43,
                        column: 9,
                        offset: 1119,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 43,
                        column: 20,
                        offset: 1130,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Assign,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 43,
                        column: 21,
                        offset: 1131,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 43,
                        column: 22,
                        offset: 1132,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::StringValue,
                value: TokenValue::String("./selfhost/tokens.rs"),
                span: Span {
                    start: Position {
                        line: 43,
                        column: 23,
                        offset: 1133,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 43,
                        column: 45,
                        offset: 1155,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Semicolon,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 43,
                        column: 45,
                        offset: 1155,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 43,
                        column: 46,
                        offset: 1156,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Let,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 44,
                        column: 5,
                        offset: 1162,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 44,
                        column: 8,
                        offset: 1165,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("rust_code"),
                span: Span {
                    start: Position {
                        line: 44,
                        column: 9,
                        offset: 1166,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 44,
                        column: 18,
                        offset: 1175,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Assign,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 44,
                        column: 19,
                        offset: 1176,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 44,
                        column: 20,
                        offset: 1177,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::StringValue,
                value: TokenValue::String("fn main() {let foo = "),
                span: Span {
                    start: Position {
                        line: 44,
                        column: 21,
                        offset: 1178,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 44,
                        column: 44,
                        offset: 1201,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Plus,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 44,
                        column: 45,
                        offset: 1202,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 44,
                        column: 46,
                        offset: 1203,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("token_capture_debug"),
                span: Span {
                    start: Position {
                        line: 44,
                        column: 47,
                        offset: 1204,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 44,
                        column: 66,
                        offset: 1223,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::ParenOpen,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 44,
                        column: 66,
                        offset: 1223,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 44,
                        column: 67,
                        offset: 1224,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Reference,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 44,
                        column: 67,
                        offset: 1224,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 44,
                        column: 68,
                        offset: 1225,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("token_capture"),
                span: Span {
                    start: Position {
                        line: 44,
                        column: 68,
                        offset: 1225,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 44,
                        column: 81,
                        offset: 1238,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::ParenClose,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 44,
                        column: 81,
                        offset: 1238,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 44,
                        column: 82,
                        offset: 1239,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Plus,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 44,
                        column: 83,
                        offset: 1240,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 44,
                        column: 84,
                        offset: 1241,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::StringValue,
                value: TokenValue::String(";}"),
                span: Span {
                    start: Position {
                        line: 44,
                        column: 85,
                        offset: 1242,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 44,
                        column: 89,
                        offset: 1246,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Semicolon,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 44,
                        column: 89,
                        offset: 1246,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 44,
                        column: 90,
                        offset: 1247,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("write_file"),
                span: Span {
                    start: Position {
                        line: 45,
                        column: 5,
                        offset: 1253,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 45,
                        column: 15,
                        offset: 1263,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::ParenOpen,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 45,
                        column: 15,
                        offset: 1263,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 45,
                        column: 16,
                        offset: 1264,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("output_path"),
                span: Span {
                    start: Position {
                        line: 45,
                        column: 16,
                        offset: 1264,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 45,
                        column: 27,
                        offset: 1275,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Comma,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 45,
                        column: 27,
                        offset: 1275,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 45,
                        column: 28,
                        offset: 1276,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("rust_code"),
                span: Span {
                    start: Position {
                        line: 45,
                        column: 29,
                        offset: 1277,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 45,
                        column: 38,
                        offset: 1286,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::ParenClose,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 45,
                        column: 38,
                        offset: 1286,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 45,
                        column: 39,
                        offset: 1287,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Semicolon,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 45,
                        column: 39,
                        offset: 1287,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 45,
                        column: 40,
                        offset: 1288,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::BraceClose,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 46,
                        column: 1,
                        offset: 1290,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 46,
                        column: 2,
                        offset: 1291,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("main"),
                span: Span {
                    start: Position {
                        line: 48,
                        column: 1,
                        offset: 1295,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 48,
                        column: 5,
                        offset: 1299,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::ParenOpen,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 48,
                        column: 5,
                        offset: 1299,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 48,
                        column: 6,
                        offset: 1300,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::ParenClose,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 48,
                        column: 6,
                        offset: 1300,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 48,
                        column: 7,
                        offset: 1301,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Semicolon,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 48,
                        column: 7,
                        offset: 1301,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 48,
                        column: 8,
                        offset: 1302,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::ETX,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 49,
                        column: 1,
                        offset: 1304,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 49,
                        column: 1,
                        offset: 1304,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
        ],
    };
}
