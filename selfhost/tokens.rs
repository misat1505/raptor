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
                value: TokenValue::String("token_result"),
                span: Span {
                    start: Position {
                        line: 24,
                        column: 13,
                        offset: 571,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 24,
                        column: 25,
                        offset: 583,
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
                        column: 26,
                        offset: 584,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 24,
                        column: 27,
                        offset: 585,
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
                        column: 28,
                        offset: 586,
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
                category: TokenCategory::ParenOpen,
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
                category: TokenCategory::Reference,
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
                category: TokenCategory::Identifier,
                value: TokenValue::String("lexer"),
                span: Span {
                    start: Position {
                        line: 24,
                        column: 40,
                        offset: 598,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 24,
                        column: 45,
                        offset: 603,
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
                        column: 45,
                        offset: 603,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 24,
                        column: 46,
                        offset: 604,
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
                        column: 46,
                        offset: 604,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 24,
                        column: 47,
                        offset: 605,
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
                        offset: 617,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 26,
                        column: 14,
                        offset: 622,
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
                        offset: 623,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 26,
                        column: 16,
                        offset: 624,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("token_result"),
                span: Span {
                    start: Position {
                        line: 26,
                        column: 16,
                        offset: 624,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 26,
                        column: 28,
                        offset: 636,
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
                        column: 28,
                        offset: 636,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 26,
                        column: 29,
                        offset: 637,
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
                        column: 30,
                        offset: 638,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 26,
                        column: 31,
                        offset: 639,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("ResultOptionToken"),
                span: Span {
                    start: Position {
                        line: 27,
                        column: 13,
                        offset: 653,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 27,
                        column: 30,
                        offset: 670,
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
                        column: 30,
                        offset: 670,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 27,
                        column: 32,
                        offset: 672,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("Err"),
                span: Span {
                    start: Position {
                        line: 27,
                        column: 32,
                        offset: 672,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 27,
                        column: 35,
                        offset: 675,
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
                        column: 35,
                        offset: 675,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 27,
                        column: 36,
                        offset: 676,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("err"),
                span: Span {
                    start: Position {
                        line: 27,
                        column: 36,
                        offset: 676,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 27,
                        column: 39,
                        offset: 679,
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
                        column: 39,
                        offset: 679,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 27,
                        column: 40,
                        offset: 680,
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
                        column: 41,
                        offset: 681,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 27,
                        column: 42,
                        offset: 682,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("println"),
                span: Span {
                    start: Position {
                        line: 28,
                        column: 17,
                        offset: 700,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 28,
                        column: 24,
                        offset: 707,
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
                        column: 24,
                        offset: 707,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 28,
                        column: 25,
                        offset: 708,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("lexer_error_get_stderr_message"),
                span: Span {
                    start: Position {
                        line: 28,
                        column: 25,
                        offset: 708,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 28,
                        column: 55,
                        offset: 738,
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
                        column: 55,
                        offset: 738,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 28,
                        column: 56,
                        offset: 739,
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
                        column: 56,
                        offset: 739,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 28,
                        column: 57,
                        offset: 740,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("err"),
                span: Span {
                    start: Position {
                        line: 28,
                        column: 57,
                        offset: 740,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 28,
                        column: 60,
                        offset: 743,
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
                        column: 60,
                        offset: 743,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 28,
                        column: 61,
                        offset: 744,
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
                        column: 61,
                        offset: 744,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 28,
                        column: 62,
                        offset: 745,
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
                        column: 62,
                        offset: 745,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 28,
                        column: 63,
                        offset: 746,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Break,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 29,
                        column: 17,
                        offset: 764,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 29,
                        column: 22,
                        offset: 769,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Semicolon,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 29,
                        column: 22,
                        offset: 769,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 29,
                        column: 23,
                        offset: 770,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::BraceClose,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 30,
                        column: 13,
                        offset: 784,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 30,
                        column: 14,
                        offset: 785,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Comma,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 30,
                        column: 14,
                        offset: 785,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 30,
                        column: 15,
                        offset: 786,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("ResultOptionToken"),
                span: Span {
                    start: Position {
                        line: 31,
                        column: 13,
                        offset: 800,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 31,
                        column: 30,
                        offset: 817,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::DoubleColon,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 31,
                        column: 30,
                        offset: 817,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 31,
                        column: 32,
                        offset: 819,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("Ok"),
                span: Span {
                    start: Position {
                        line: 31,
                        column: 32,
                        offset: 819,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 31,
                        column: 34,
                        offset: 821,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::ParenOpen,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 31,
                        column: 34,
                        offset: 821,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 31,
                        column: 35,
                        offset: 822,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("token"),
                span: Span {
                    start: Position {
                        line: 31,
                        column: 35,
                        offset: 822,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 31,
                        column: 40,
                        offset: 827,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::ParenClose,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 31,
                        column: 40,
                        offset: 827,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 31,
                        column: 41,
                        offset: 828,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::BraceOpen,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 31,
                        column: 42,
                        offset: 829,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 31,
                        column: 43,
                        offset: 830,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Match,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 32,
                        column: 17,
                        offset: 848,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 32,
                        column: 22,
                        offset: 853,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::ParenOpen,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 32,
                        column: 23,
                        offset: 854,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 32,
                        column: 24,
                        offset: 855,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("token"),
                span: Span {
                    start: Position {
                        line: 32,
                        column: 24,
                        offset: 855,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 32,
                        column: 29,
                        offset: 860,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::ParenClose,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 32,
                        column: 29,
                        offset: 860,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 32,
                        column: 30,
                        offset: 861,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::BraceOpen,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 32,
                        column: 31,
                        offset: 862,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 32,
                        column: 32,
                        offset: 863,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("OptionToken"),
                span: Span {
                    start: Position {
                        line: 33,
                        column: 21,
                        offset: 885,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 33,
                        column: 32,
                        offset: 896,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::DoubleColon,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 33,
                        column: 32,
                        offset: 896,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 33,
                        column: 34,
                        offset: 898,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("Some"),
                span: Span {
                    start: Position {
                        line: 33,
                        column: 34,
                        offset: 898,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 33,
                        column: 38,
                        offset: 902,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::ParenOpen,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 33,
                        column: 38,
                        offset: 902,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 33,
                        column: 39,
                        offset: 903,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("value"),
                span: Span {
                    start: Position {
                        line: 33,
                        column: 39,
                        offset: 903,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 33,
                        column: 44,
                        offset: 908,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::ParenClose,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 33,
                        column: 44,
                        offset: 908,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 33,
                        column: 45,
                        offset: 909,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::BraceOpen,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 33,
                        column: 46,
                        offset: 910,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 33,
                        column: 47,
                        offset: 911,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("vector_push"),
                span: Span {
                    start: Position {
                        line: 34,
                        column: 25,
                        offset: 937,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 34,
                        column: 36,
                        offset: 948,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::ParenOpen,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 34,
                        column: 36,
                        offset: 948,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 34,
                        column: 37,
                        offset: 949,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Reference,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 34,
                        column: 37,
                        offset: 949,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 34,
                        column: 38,
                        offset: 950,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("token_capture"),
                span: Span {
                    start: Position {
                        line: 34,
                        column: 38,
                        offset: 950,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 34,
                        column: 51,
                        offset: 963,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Dot,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 34,
                        column: 51,
                        offset: 963,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 34,
                        column: 52,
                        offset: 964,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("tokens"),
                span: Span {
                    start: Position {
                        line: 34,
                        column: 52,
                        offset: 964,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 34,
                        column: 58,
                        offset: 970,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Comma,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 34,
                        column: 58,
                        offset: 970,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 34,
                        column: 59,
                        offset: 971,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("value"),
                span: Span {
                    start: Position {
                        line: 34,
                        column: 60,
                        offset: 972,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 34,
                        column: 65,
                        offset: 977,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::ParenClose,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 34,
                        column: 65,
                        offset: 977,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 34,
                        column: 66,
                        offset: 978,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Semicolon,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 34,
                        column: 66,
                        offset: 978,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 34,
                        column: 67,
                        offset: 979,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::If,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 36,
                        column: 25,
                        offset: 1007,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 36,
                        column: 27,
                        offset: 1009,
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
                        column: 28,
                        offset: 1010,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 36,
                        column: 29,
                        offset: 1011,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("value"),
                span: Span {
                    start: Position {
                        line: 36,
                        column: 29,
                        offset: 1011,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 36,
                        column: 34,
                        offset: 1016,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Dot,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 36,
                        column: 34,
                        offset: 1016,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 36,
                        column: 35,
                        offset: 1017,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("category"),
                span: Span {
                    start: Position {
                        line: 36,
                        column: 35,
                        offset: 1017,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 36,
                        column: 43,
                        offset: 1025,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Matches,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 36,
                        column: 44,
                        offset: 1026,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 36,
                        column: 51,
                        offset: 1033,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("TokenCategory"),
                span: Span {
                    start: Position {
                        line: 36,
                        column: 52,
                        offset: 1034,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 36,
                        column: 65,
                        offset: 1047,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::DoubleColon,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 36,
                        column: 65,
                        offset: 1047,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 36,
                        column: 67,
                        offset: 1049,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("ETX"),
                span: Span {
                    start: Position {
                        line: 36,
                        column: 67,
                        offset: 1049,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 36,
                        column: 70,
                        offset: 1052,
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
                        column: 70,
                        offset: 1052,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 36,
                        column: 71,
                        offset: 1053,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::BraceOpen,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 36,
                        column: 72,
                        offset: 1054,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 36,
                        column: 73,
                        offset: 1055,
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
                        column: 29,
                        offset: 1085,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 37,
                        column: 34,
                        offset: 1090,
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
                        column: 34,
                        offset: 1090,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 37,
                        column: 35,
                        offset: 1091,
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
                        column: 25,
                        offset: 1117,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 38,
                        column: 26,
                        offset: 1118,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Let,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 40,
                        column: 25,
                        offset: 1146,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 40,
                        column: 28,
                        offset: 1149,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("stringified"),
                span: Span {
                    start: Position {
                        line: 40,
                        column: 29,
                        offset: 1150,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 40,
                        column: 40,
                        offset: 1161,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Assign,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 40,
                        column: 41,
                        offset: 1162,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 40,
                        column: 42,
                        offset: 1163,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("token_category_to_string"),
                span: Span {
                    start: Position {
                        line: 40,
                        column: 43,
                        offset: 1164,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 40,
                        column: 67,
                        offset: 1188,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::ParenOpen,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 40,
                        column: 67,
                        offset: 1188,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 40,
                        column: 68,
                        offset: 1189,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("value"),
                span: Span {
                    start: Position {
                        line: 40,
                        column: 68,
                        offset: 1189,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 40,
                        column: 73,
                        offset: 1194,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Dot,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 40,
                        column: 73,
                        offset: 1194,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 40,
                        column: 74,
                        offset: 1195,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("category"),
                span: Span {
                    start: Position {
                        line: 40,
                        column: 74,
                        offset: 1195,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 40,
                        column: 82,
                        offset: 1203,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::ParenClose,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 40,
                        column: 82,
                        offset: 1203,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 40,
                        column: 83,
                        offset: 1204,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Semicolon,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 40,
                        column: 83,
                        offset: 1204,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 40,
                        column: 84,
                        offset: 1205,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Let,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 41,
                        column: 25,
                        offset: 1231,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 41,
                        column: 28,
                        offset: 1234,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("whitespaces"),
                span: Span {
                    start: Position {
                        line: 41,
                        column: 29,
                        offset: 1235,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 41,
                        column: 40,
                        offset: 1246,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Assign,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 41,
                        column: 41,
                        offset: 1247,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 41,
                        column: 42,
                        offset: 1248,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::I64Value,
                value: TokenValue::I64(20),
                span: Span {
                    start: Position {
                        line: 41,
                        column: 43,
                        offset: 1249,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 41,
                        column: 45,
                        offset: 1251,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Minus,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 41,
                        column: 46,
                        offset: 1252,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 41,
                        column: 47,
                        offset: 1253,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("str_len"),
                span: Span {
                    start: Position {
                        line: 41,
                        column: 48,
                        offset: 1254,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 41,
                        column: 55,
                        offset: 1261,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::ParenOpen,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 41,
                        column: 55,
                        offset: 1261,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 41,
                        column: 56,
                        offset: 1262,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("stringified"),
                span: Span {
                    start: Position {
                        line: 41,
                        column: 56,
                        offset: 1262,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 41,
                        column: 67,
                        offset: 1273,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::ParenClose,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 41,
                        column: 67,
                        offset: 1273,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 41,
                        column: 68,
                        offset: 1274,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Semicolon,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 41,
                        column: 68,
                        offset: 1274,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 41,
                        column: 69,
                        offset: 1275,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("print"),
                span: Span {
                    start: Position {
                        line: 42,
                        column: 25,
                        offset: 1301,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 42,
                        column: 30,
                        offset: 1306,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::ParenOpen,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 42,
                        column: 30,
                        offset: 1306,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 42,
                        column: 31,
                        offset: 1307,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("token_category_to_string"),
                span: Span {
                    start: Position {
                        line: 42,
                        column: 31,
                        offset: 1307,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 42,
                        column: 55,
                        offset: 1331,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::ParenOpen,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 42,
                        column: 55,
                        offset: 1331,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 42,
                        column: 56,
                        offset: 1332,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("value"),
                span: Span {
                    start: Position {
                        line: 42,
                        column: 56,
                        offset: 1332,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 42,
                        column: 61,
                        offset: 1337,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Dot,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 42,
                        column: 61,
                        offset: 1337,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 42,
                        column: 62,
                        offset: 1338,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("category"),
                span: Span {
                    start: Position {
                        line: 42,
                        column: 62,
                        offset: 1338,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 42,
                        column: 70,
                        offset: 1346,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::ParenClose,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 42,
                        column: 70,
                        offset: 1346,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 42,
                        column: 71,
                        offset: 1347,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::ParenClose,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 42,
                        column: 71,
                        offset: 1347,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 42,
                        column: 72,
                        offset: 1348,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Semicolon,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 42,
                        column: 72,
                        offset: 1348,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 42,
                        column: 73,
                        offset: 1349,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::For,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 43,
                        column: 25,
                        offset: 1375,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 43,
                        column: 28,
                        offset: 1378,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::ParenOpen,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 43,
                        column: 29,
                        offset: 1379,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 43,
                        column: 30,
                        offset: 1380,
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
                        column: 30,
                        offset: 1380,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 43,
                        column: 33,
                        offset: 1383,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("i"),
                span: Span {
                    start: Position {
                        line: 43,
                        column: 34,
                        offset: 1384,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 43,
                        column: 35,
                        offset: 1385,
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
                        column: 36,
                        offset: 1386,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 43,
                        column: 37,
                        offset: 1387,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::I64Value,
                value: TokenValue::I64(0),
                span: Span {
                    start: Position {
                        line: 43,
                        column: 38,
                        offset: 1388,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 43,
                        column: 39,
                        offset: 1389,
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
                        column: 39,
                        offset: 1389,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 43,
                        column: 40,
                        offset: 1390,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("i"),
                span: Span {
                    start: Position {
                        line: 43,
                        column: 41,
                        offset: 1391,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 43,
                        column: 42,
                        offset: 1392,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Less,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 43,
                        column: 43,
                        offset: 1393,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 43,
                        column: 44,
                        offset: 1394,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("whitespaces"),
                span: Span {
                    start: Position {
                        line: 43,
                        column: 45,
                        offset: 1395,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 43,
                        column: 56,
                        offset: 1406,
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
                        column: 56,
                        offset: 1406,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 43,
                        column: 57,
                        offset: 1407,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("i"),
                span: Span {
                    start: Position {
                        line: 43,
                        column: 58,
                        offset: 1408,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 43,
                        column: 59,
                        offset: 1409,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::PlusEquals,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 43,
                        column: 60,
                        offset: 1410,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 43,
                        column: 62,
                        offset: 1412,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::I64Value,
                value: TokenValue::I64(1),
                span: Span {
                    start: Position {
                        line: 43,
                        column: 63,
                        offset: 1413,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 43,
                        column: 64,
                        offset: 1414,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::ParenClose,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 43,
                        column: 64,
                        offset: 1414,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 43,
                        column: 65,
                        offset: 1415,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("print"),
                span: Span {
                    start: Position {
                        line: 43,
                        column: 66,
                        offset: 1416,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 43,
                        column: 71,
                        offset: 1421,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::ParenOpen,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 43,
                        column: 71,
                        offset: 1421,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 43,
                        column: 72,
                        offset: 1422,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::StringValue,
                value: TokenValue::String(" "),
                span: Span {
                    start: Position {
                        line: 43,
                        column: 72,
                        offset: 1422,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 43,
                        column: 75,
                        offset: 1425,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::ParenClose,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 43,
                        column: 75,
                        offset: 1425,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 43,
                        column: 76,
                        offset: 1426,
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
                        column: 76,
                        offset: 1426,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 43,
                        column: 77,
                        offset: 1427,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("println"),
                span: Span {
                    start: Position {
                        line: 44,
                        column: 25,
                        offset: 1453,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 44,
                        column: 32,
                        offset: 1460,
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
                        column: 32,
                        offset: 1460,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 44,
                        column: 33,
                        offset: 1461,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("position_display"),
                span: Span {
                    start: Position {
                        line: 44,
                        column: 33,
                        offset: 1461,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 44,
                        column: 49,
                        offset: 1477,
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
                        column: 49,
                        offset: 1477,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 44,
                        column: 50,
                        offset: 1478,
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
                        column: 50,
                        offset: 1478,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 44,
                        column: 51,
                        offset: 1479,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("value"),
                span: Span {
                    start: Position {
                        line: 44,
                        column: 51,
                        offset: 1479,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 44,
                        column: 56,
                        offset: 1484,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Dot,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 44,
                        column: 56,
                        offset: 1484,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 44,
                        column: 57,
                        offset: 1485,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("span"),
                span: Span {
                    start: Position {
                        line: 44,
                        column: 57,
                        offset: 1485,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 44,
                        column: 61,
                        offset: 1489,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Dot,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 44,
                        column: 61,
                        offset: 1489,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 44,
                        column: 62,
                        offset: 1490,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("start"),
                span: Span {
                    start: Position {
                        line: 44,
                        column: 62,
                        offset: 1490,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 44,
                        column: 67,
                        offset: 1495,
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
                        column: 67,
                        offset: 1495,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 44,
                        column: 68,
                        offset: 1496,
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
                        column: 69,
                        offset: 1497,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 44,
                        column: 70,
                        offset: 1498,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::StringValue,
                value: TokenValue::String("	-> "),
                span: Span {
                    start: Position {
                        line: 44,
                        column: 71,
                        offset: 1499,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 44,
                        column: 78,
                        offset: 1506,
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
                        column: 79,
                        offset: 1507,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 44,
                        column: 80,
                        offset: 1508,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("position_display"),
                span: Span {
                    start: Position {
                        line: 44,
                        column: 81,
                        offset: 1509,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 44,
                        column: 97,
                        offset: 1525,
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
                        column: 97,
                        offset: 1525,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 44,
                        column: 98,
                        offset: 1526,
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
                        column: 98,
                        offset: 1526,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 44,
                        column: 99,
                        offset: 1527,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("value"),
                span: Span {
                    start: Position {
                        line: 44,
                        column: 99,
                        offset: 1527,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 44,
                        column: 104,
                        offset: 1532,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Dot,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 44,
                        column: 104,
                        offset: 1532,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 44,
                        column: 105,
                        offset: 1533,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("span"),
                span: Span {
                    start: Position {
                        line: 44,
                        column: 105,
                        offset: 1533,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 44,
                        column: 109,
                        offset: 1537,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Dot,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 44,
                        column: 109,
                        offset: 1537,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 44,
                        column: 110,
                        offset: 1538,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("end"),
                span: Span {
                    start: Position {
                        line: 44,
                        column: 110,
                        offset: 1538,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 44,
                        column: 113,
                        offset: 1541,
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
                        column: 113,
                        offset: 1541,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 44,
                        column: 114,
                        offset: 1542,
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
                        column: 114,
                        offset: 1542,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 44,
                        column: 115,
                        offset: 1543,
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
                        column: 115,
                        offset: 1543,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 44,
                        column: 116,
                        offset: 1544,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::BraceClose,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 45,
                        column: 21,
                        offset: 1566,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 45,
                        column: 22,
                        offset: 1567,
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
                        column: 22,
                        offset: 1567,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 45,
                        column: 23,
                        offset: 1568,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("OptionToken"),
                span: Span {
                    start: Position {
                        line: 47,
                        column: 21,
                        offset: 1592,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 47,
                        column: 32,
                        offset: 1603,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::DoubleColon,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 47,
                        column: 32,
                        offset: 1603,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 47,
                        column: 34,
                        offset: 1605,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("None"),
                span: Span {
                    start: Position {
                        line: 47,
                        column: 34,
                        offset: 1605,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 47,
                        column: 38,
                        offset: 1609,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::BraceOpen,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 47,
                        column: 39,
                        offset: 1610,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 47,
                        column: 40,
                        offset: 1611,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("println"),
                span: Span {
                    start: Position {
                        line: 48,
                        column: 25,
                        offset: 1637,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 48,
                        column: 32,
                        offset: 1644,
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
                        column: 32,
                        offset: 1644,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 48,
                        column: 33,
                        offset: 1645,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::StringValue,
                value: TokenValue::String("No token produced"),
                span: Span {
                    start: Position {
                        line: 48,
                        column: 33,
                        offset: 1645,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 48,
                        column: 52,
                        offset: 1664,
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
                        column: 52,
                        offset: 1664,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 48,
                        column: 53,
                        offset: 1665,
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
                        column: 53,
                        offset: 1665,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 48,
                        column: 54,
                        offset: 1666,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Break,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 49,
                        column: 25,
                        offset: 1692,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 49,
                        column: 30,
                        offset: 1697,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Semicolon,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 49,
                        column: 30,
                        offset: 1697,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 49,
                        column: 31,
                        offset: 1698,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::BraceClose,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 50,
                        column: 21,
                        offset: 1720,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 50,
                        column: 22,
                        offset: 1721,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::BraceClose,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 51,
                        column: 17,
                        offset: 1739,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 51,
                        column: 18,
                        offset: 1740,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::BraceClose,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 52,
                        column: 13,
                        offset: 1754,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 52,
                        column: 14,
                        offset: 1755,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::BraceClose,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 53,
                        column: 9,
                        offset: 1765,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 53,
                        column: 10,
                        offset: 1766,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::BraceClose,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 54,
                        column: 5,
                        offset: 1772,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 54,
                        column: 6,
                        offset: 1773,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Let,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 57,
                        column: 5,
                        offset: 1880,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 57,
                        column: 8,
                        offset: 1883,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("output_path"),
                span: Span {
                    start: Position {
                        line: 57,
                        column: 9,
                        offset: 1884,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 57,
                        column: 20,
                        offset: 1895,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Assign,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 57,
                        column: 21,
                        offset: 1896,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 57,
                        column: 22,
                        offset: 1897,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::StringValue,
                value: TokenValue::String("./selfhost/tokens.rs"),
                span: Span {
                    start: Position {
                        line: 57,
                        column: 23,
                        offset: 1898,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 57,
                        column: 45,
                        offset: 1920,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Semicolon,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 57,
                        column: 45,
                        offset: 1920,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 57,
                        column: 46,
                        offset: 1921,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Let,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 58,
                        column: 5,
                        offset: 1927,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 58,
                        column: 8,
                        offset: 1930,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("rust_code"),
                span: Span {
                    start: Position {
                        line: 58,
                        column: 9,
                        offset: 1931,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 58,
                        column: 18,
                        offset: 1940,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Assign,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 58,
                        column: 19,
                        offset: 1941,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 58,
                        column: 20,
                        offset: 1942,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::StringValue,
                value: TokenValue::String("fn main() {let foo = "),
                span: Span {
                    start: Position {
                        line: 58,
                        column: 21,
                        offset: 1943,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 58,
                        column: 44,
                        offset: 1966,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Plus,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 58,
                        column: 45,
                        offset: 1967,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 58,
                        column: 46,
                        offset: 1968,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("token_capture_debug"),
                span: Span {
                    start: Position {
                        line: 58,
                        column: 47,
                        offset: 1969,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 58,
                        column: 66,
                        offset: 1988,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::ParenOpen,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 58,
                        column: 66,
                        offset: 1988,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 58,
                        column: 67,
                        offset: 1989,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Reference,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 58,
                        column: 67,
                        offset: 1989,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 58,
                        column: 68,
                        offset: 1990,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("token_capture"),
                span: Span {
                    start: Position {
                        line: 58,
                        column: 68,
                        offset: 1990,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 58,
                        column: 81,
                        offset: 2003,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::ParenClose,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 58,
                        column: 81,
                        offset: 2003,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 58,
                        column: 82,
                        offset: 2004,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Plus,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 58,
                        column: 83,
                        offset: 2005,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 58,
                        column: 84,
                        offset: 2006,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::StringValue,
                value: TokenValue::String(";}"),
                span: Span {
                    start: Position {
                        line: 58,
                        column: 85,
                        offset: 2007,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 58,
                        column: 89,
                        offset: 2011,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Semicolon,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 58,
                        column: 89,
                        offset: 2011,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 58,
                        column: 90,
                        offset: 2012,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("write_file"),
                span: Span {
                    start: Position {
                        line: 59,
                        column: 5,
                        offset: 2018,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 59,
                        column: 15,
                        offset: 2028,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::ParenOpen,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 59,
                        column: 15,
                        offset: 2028,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 59,
                        column: 16,
                        offset: 2029,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("output_path"),
                span: Span {
                    start: Position {
                        line: 59,
                        column: 16,
                        offset: 2029,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 59,
                        column: 27,
                        offset: 2040,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Comma,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 59,
                        column: 27,
                        offset: 2040,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 59,
                        column: 28,
                        offset: 2041,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("rust_code"),
                span: Span {
                    start: Position {
                        line: 59,
                        column: 29,
                        offset: 2042,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 59,
                        column: 38,
                        offset: 2051,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::ParenClose,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 59,
                        column: 38,
                        offset: 2051,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 59,
                        column: 39,
                        offset: 2052,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Semicolon,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 59,
                        column: 39,
                        offset: 2052,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 59,
                        column: 40,
                        offset: 2053,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::BraceClose,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 60,
                        column: 1,
                        offset: 2055,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 60,
                        column: 2,
                        offset: 2056,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("main"),
                span: Span {
                    start: Position {
                        line: 62,
                        column: 1,
                        offset: 2060,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 62,
                        column: 5,
                        offset: 2064,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::ParenOpen,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 62,
                        column: 5,
                        offset: 2064,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 62,
                        column: 6,
                        offset: 2065,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::ParenClose,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 62,
                        column: 6,
                        offset: 2065,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 62,
                        column: 7,
                        offset: 2066,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Semicolon,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 62,
                        column: 7,
                        offset: 2066,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 62,
                        column: 8,
                        offset: 2067,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::ETX,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 63,
                        column: 1,
                        offset: 2069,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 63,
                        column: 1,
                        offset: 2069,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
        ],
    };
}
