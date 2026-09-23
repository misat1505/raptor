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
                category: TokenCategory::StringValue,
                value: TokenValue::String("Lexer error"),
                span: Span {
                    start: Position {
                        line: 28,
                        column: 25,
                        offset: 708,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 28,
                        column: 38,
                        offset: 721,
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
                        column: 38,
                        offset: 721,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 28,
                        column: 39,
                        offset: 722,
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
                        column: 39,
                        offset: 722,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 28,
                        column: 40,
                        offset: 723,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("println"),
                span: Span {
                    start: Position {
                        line: 29,
                        column: 17,
                        offset: 741,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 29,
                        column: 24,
                        offset: 748,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::ParenOpen,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 29,
                        column: 24,
                        offset: 748,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 29,
                        column: 25,
                        offset: 749,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("err"),
                span: Span {
                    start: Position {
                        line: 29,
                        column: 25,
                        offset: 749,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 29,
                        column: 28,
                        offset: 752,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Dot,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 29,
                        column: 28,
                        offset: 752,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 29,
                        column: 29,
                        offset: 753,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("message"),
                span: Span {
                    start: Position {
                        line: 29,
                        column: 29,
                        offset: 753,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 29,
                        column: 36,
                        offset: 760,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::ParenClose,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 29,
                        column: 36,
                        offset: 760,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 29,
                        column: 37,
                        offset: 761,
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
                        column: 37,
                        offset: 761,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 29,
                        column: 38,
                        offset: 762,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Break,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 30,
                        column: 17,
                        offset: 780,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 30,
                        column: 22,
                        offset: 785,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Semicolon,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 30,
                        column: 22,
                        offset: 785,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 30,
                        column: 23,
                        offset: 786,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::BraceClose,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 31,
                        column: 13,
                        offset: 800,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 31,
                        column: 14,
                        offset: 801,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Comma,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 31,
                        column: 14,
                        offset: 801,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 31,
                        column: 15,
                        offset: 802,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("ResultOptionToken"),
                span: Span {
                    start: Position {
                        line: 32,
                        column: 13,
                        offset: 816,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 32,
                        column: 30,
                        offset: 833,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::DoubleColon,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 32,
                        column: 30,
                        offset: 833,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 32,
                        column: 32,
                        offset: 835,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("Ok"),
                span: Span {
                    start: Position {
                        line: 32,
                        column: 32,
                        offset: 835,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 32,
                        column: 34,
                        offset: 837,
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
                        column: 34,
                        offset: 837,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 32,
                        column: 35,
                        offset: 838,
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
                        column: 35,
                        offset: 838,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 32,
                        column: 40,
                        offset: 843,
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
                        column: 40,
                        offset: 843,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 32,
                        column: 41,
                        offset: 844,
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
                        column: 42,
                        offset: 845,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 32,
                        column: 43,
                        offset: 846,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Match,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 33,
                        column: 17,
                        offset: 864,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 33,
                        column: 22,
                        offset: 869,
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
                        column: 23,
                        offset: 870,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 33,
                        column: 24,
                        offset: 871,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("token"),
                span: Span {
                    start: Position {
                        line: 33,
                        column: 24,
                        offset: 871,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 33,
                        column: 29,
                        offset: 876,
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
                        column: 29,
                        offset: 876,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 33,
                        column: 30,
                        offset: 877,
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
                        column: 31,
                        offset: 878,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 33,
                        column: 32,
                        offset: 879,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("OptionToken"),
                span: Span {
                    start: Position {
                        line: 34,
                        column: 21,
                        offset: 901,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 34,
                        column: 32,
                        offset: 912,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::DoubleColon,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 34,
                        column: 32,
                        offset: 912,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 34,
                        column: 34,
                        offset: 914,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("Some"),
                span: Span {
                    start: Position {
                        line: 34,
                        column: 34,
                        offset: 914,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 34,
                        column: 38,
                        offset: 918,
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
                        column: 38,
                        offset: 918,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 34,
                        column: 39,
                        offset: 919,
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
                        column: 39,
                        offset: 919,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 34,
                        column: 44,
                        offset: 924,
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
                        column: 44,
                        offset: 924,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 34,
                        column: 45,
                        offset: 925,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::BraceOpen,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 34,
                        column: 46,
                        offset: 926,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 34,
                        column: 47,
                        offset: 927,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("vector_push"),
                span: Span {
                    start: Position {
                        line: 35,
                        column: 25,
                        offset: 953,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 35,
                        column: 36,
                        offset: 964,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::ParenOpen,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 35,
                        column: 36,
                        offset: 964,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 35,
                        column: 37,
                        offset: 965,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Reference,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 35,
                        column: 37,
                        offset: 965,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 35,
                        column: 38,
                        offset: 966,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("token_capture"),
                span: Span {
                    start: Position {
                        line: 35,
                        column: 38,
                        offset: 966,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 35,
                        column: 51,
                        offset: 979,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Dot,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 35,
                        column: 51,
                        offset: 979,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 35,
                        column: 52,
                        offset: 980,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("tokens"),
                span: Span {
                    start: Position {
                        line: 35,
                        column: 52,
                        offset: 980,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 35,
                        column: 58,
                        offset: 986,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Comma,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 35,
                        column: 58,
                        offset: 986,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 35,
                        column: 59,
                        offset: 987,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("value"),
                span: Span {
                    start: Position {
                        line: 35,
                        column: 60,
                        offset: 988,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 35,
                        column: 65,
                        offset: 993,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::ParenClose,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 35,
                        column: 65,
                        offset: 993,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 35,
                        column: 66,
                        offset: 994,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Semicolon,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 35,
                        column: 66,
                        offset: 994,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 35,
                        column: 67,
                        offset: 995,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::If,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 37,
                        column: 25,
                        offset: 1023,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 37,
                        column: 27,
                        offset: 1025,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::ParenOpen,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 37,
                        column: 28,
                        offset: 1026,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 37,
                        column: 29,
                        offset: 1027,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("value"),
                span: Span {
                    start: Position {
                        line: 37,
                        column: 29,
                        offset: 1027,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 37,
                        column: 34,
                        offset: 1032,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Dot,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 37,
                        column: 34,
                        offset: 1032,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 37,
                        column: 35,
                        offset: 1033,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("category"),
                span: Span {
                    start: Position {
                        line: 37,
                        column: 35,
                        offset: 1033,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 37,
                        column: 43,
                        offset: 1041,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Matches,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 37,
                        column: 44,
                        offset: 1042,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 37,
                        column: 51,
                        offset: 1049,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("TokenCategory"),
                span: Span {
                    start: Position {
                        line: 37,
                        column: 52,
                        offset: 1050,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 37,
                        column: 65,
                        offset: 1063,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::DoubleColon,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 37,
                        column: 65,
                        offset: 1063,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 37,
                        column: 67,
                        offset: 1065,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("ETX"),
                span: Span {
                    start: Position {
                        line: 37,
                        column: 67,
                        offset: 1065,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 37,
                        column: 70,
                        offset: 1068,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::ParenClose,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 37,
                        column: 70,
                        offset: 1068,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 37,
                        column: 71,
                        offset: 1069,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::BraceOpen,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 37,
                        column: 72,
                        offset: 1070,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 37,
                        column: 73,
                        offset: 1071,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Break,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 38,
                        column: 29,
                        offset: 1101,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 38,
                        column: 34,
                        offset: 1106,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Semicolon,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 38,
                        column: 34,
                        offset: 1106,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 38,
                        column: 35,
                        offset: 1107,
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
                        column: 25,
                        offset: 1133,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 39,
                        column: 26,
                        offset: 1134,
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
                        offset: 1162,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 41,
                        column: 28,
                        offset: 1165,
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
                        column: 29,
                        offset: 1166,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 41,
                        column: 40,
                        offset: 1177,
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
                        offset: 1178,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 41,
                        column: 42,
                        offset: 1179,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("token_category_to_string"),
                span: Span {
                    start: Position {
                        line: 41,
                        column: 43,
                        offset: 1180,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 41,
                        column: 67,
                        offset: 1204,
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
                        column: 67,
                        offset: 1204,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 41,
                        column: 68,
                        offset: 1205,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("value"),
                span: Span {
                    start: Position {
                        line: 41,
                        column: 68,
                        offset: 1205,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 41,
                        column: 73,
                        offset: 1210,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Dot,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 41,
                        column: 73,
                        offset: 1210,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 41,
                        column: 74,
                        offset: 1211,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("category"),
                span: Span {
                    start: Position {
                        line: 41,
                        column: 74,
                        offset: 1211,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 41,
                        column: 82,
                        offset: 1219,
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
                        column: 82,
                        offset: 1219,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 41,
                        column: 83,
                        offset: 1220,
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
                        column: 83,
                        offset: 1220,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 41,
                        column: 84,
                        offset: 1221,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Let,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 42,
                        column: 25,
                        offset: 1247,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 42,
                        column: 28,
                        offset: 1250,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("whitespaces"),
                span: Span {
                    start: Position {
                        line: 42,
                        column: 29,
                        offset: 1251,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 42,
                        column: 40,
                        offset: 1262,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Assign,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 42,
                        column: 41,
                        offset: 1263,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 42,
                        column: 42,
                        offset: 1264,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::I64Value,
                value: TokenValue::I64(20),
                span: Span {
                    start: Position {
                        line: 42,
                        column: 43,
                        offset: 1265,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 42,
                        column: 45,
                        offset: 1267,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Minus,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 42,
                        column: 46,
                        offset: 1268,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 42,
                        column: 47,
                        offset: 1269,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("str_len"),
                span: Span {
                    start: Position {
                        line: 42,
                        column: 48,
                        offset: 1270,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 42,
                        column: 55,
                        offset: 1277,
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
                        offset: 1277,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 42,
                        column: 56,
                        offset: 1278,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("stringified"),
                span: Span {
                    start: Position {
                        line: 42,
                        column: 56,
                        offset: 1278,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 42,
                        column: 67,
                        offset: 1289,
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
                        column: 67,
                        offset: 1289,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 42,
                        column: 68,
                        offset: 1290,
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
                        column: 68,
                        offset: 1290,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 42,
                        column: 69,
                        offset: 1291,
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
                        column: 25,
                        offset: 1317,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 43,
                        column: 30,
                        offset: 1322,
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
                        column: 30,
                        offset: 1322,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 43,
                        column: 31,
                        offset: 1323,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("token_category_to_string"),
                span: Span {
                    start: Position {
                        line: 43,
                        column: 31,
                        offset: 1323,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 43,
                        column: 55,
                        offset: 1347,
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
                        column: 55,
                        offset: 1347,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 43,
                        column: 56,
                        offset: 1348,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("value"),
                span: Span {
                    start: Position {
                        line: 43,
                        column: 56,
                        offset: 1348,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 43,
                        column: 61,
                        offset: 1353,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Dot,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 43,
                        column: 61,
                        offset: 1353,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 43,
                        column: 62,
                        offset: 1354,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("category"),
                span: Span {
                    start: Position {
                        line: 43,
                        column: 62,
                        offset: 1354,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 43,
                        column: 70,
                        offset: 1362,
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
                        column: 70,
                        offset: 1362,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 43,
                        column: 71,
                        offset: 1363,
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
                        column: 71,
                        offset: 1363,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 43,
                        column: 72,
                        offset: 1364,
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
                        column: 72,
                        offset: 1364,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 43,
                        column: 73,
                        offset: 1365,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::For,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 44,
                        column: 25,
                        offset: 1391,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 44,
                        column: 28,
                        offset: 1394,
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
                        column: 29,
                        offset: 1395,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 44,
                        column: 30,
                        offset: 1396,
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
                        column: 30,
                        offset: 1396,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 44,
                        column: 33,
                        offset: 1399,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("i"),
                span: Span {
                    start: Position {
                        line: 44,
                        column: 34,
                        offset: 1400,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 44,
                        column: 35,
                        offset: 1401,
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
                        column: 36,
                        offset: 1402,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 44,
                        column: 37,
                        offset: 1403,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::I64Value,
                value: TokenValue::I64(0),
                span: Span {
                    start: Position {
                        line: 44,
                        column: 38,
                        offset: 1404,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 44,
                        column: 39,
                        offset: 1405,
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
                        column: 39,
                        offset: 1405,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 44,
                        column: 40,
                        offset: 1406,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("i"),
                span: Span {
                    start: Position {
                        line: 44,
                        column: 41,
                        offset: 1407,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 44,
                        column: 42,
                        offset: 1408,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Less,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 44,
                        column: 43,
                        offset: 1409,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 44,
                        column: 44,
                        offset: 1410,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("whitespaces"),
                span: Span {
                    start: Position {
                        line: 44,
                        column: 45,
                        offset: 1411,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 44,
                        column: 56,
                        offset: 1422,
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
                        column: 56,
                        offset: 1422,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 44,
                        column: 57,
                        offset: 1423,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("i"),
                span: Span {
                    start: Position {
                        line: 44,
                        column: 58,
                        offset: 1424,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 44,
                        column: 59,
                        offset: 1425,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::PlusEquals,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 44,
                        column: 60,
                        offset: 1426,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 44,
                        column: 62,
                        offset: 1428,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::I64Value,
                value: TokenValue::I64(1),
                span: Span {
                    start: Position {
                        line: 44,
                        column: 63,
                        offset: 1429,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 44,
                        column: 64,
                        offset: 1430,
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
                        column: 64,
                        offset: 1430,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 44,
                        column: 65,
                        offset: 1431,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("print"),
                span: Span {
                    start: Position {
                        line: 44,
                        column: 66,
                        offset: 1432,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 44,
                        column: 71,
                        offset: 1437,
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
                        column: 71,
                        offset: 1437,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 44,
                        column: 72,
                        offset: 1438,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::StringValue,
                value: TokenValue::String(" "),
                span: Span {
                    start: Position {
                        line: 44,
                        column: 72,
                        offset: 1438,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 44,
                        column: 75,
                        offset: 1441,
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
                        column: 75,
                        offset: 1441,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 44,
                        column: 76,
                        offset: 1442,
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
                        column: 76,
                        offset: 1442,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 44,
                        column: 77,
                        offset: 1443,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("println"),
                span: Span {
                    start: Position {
                        line: 45,
                        column: 25,
                        offset: 1469,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 45,
                        column: 32,
                        offset: 1476,
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
                        column: 32,
                        offset: 1476,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 45,
                        column: 33,
                        offset: 1477,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("position_display"),
                span: Span {
                    start: Position {
                        line: 45,
                        column: 33,
                        offset: 1477,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 45,
                        column: 49,
                        offset: 1493,
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
                        column: 49,
                        offset: 1493,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 45,
                        column: 50,
                        offset: 1494,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Reference,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 45,
                        column: 50,
                        offset: 1494,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 45,
                        column: 51,
                        offset: 1495,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("value"),
                span: Span {
                    start: Position {
                        line: 45,
                        column: 51,
                        offset: 1495,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 45,
                        column: 56,
                        offset: 1500,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Dot,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 45,
                        column: 56,
                        offset: 1500,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 45,
                        column: 57,
                        offset: 1501,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("span"),
                span: Span {
                    start: Position {
                        line: 45,
                        column: 57,
                        offset: 1501,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 45,
                        column: 61,
                        offset: 1505,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Dot,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 45,
                        column: 61,
                        offset: 1505,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 45,
                        column: 62,
                        offset: 1506,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("start"),
                span: Span {
                    start: Position {
                        line: 45,
                        column: 62,
                        offset: 1506,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 45,
                        column: 67,
                        offset: 1511,
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
                        column: 67,
                        offset: 1511,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 45,
                        column: 68,
                        offset: 1512,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Plus,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 45,
                        column: 69,
                        offset: 1513,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 45,
                        column: 70,
                        offset: 1514,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::StringValue,
                value: TokenValue::String("	-> "),
                span: Span {
                    start: Position {
                        line: 45,
                        column: 71,
                        offset: 1515,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 45,
                        column: 78,
                        offset: 1522,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Plus,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 45,
                        column: 79,
                        offset: 1523,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 45,
                        column: 80,
                        offset: 1524,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("position_display"),
                span: Span {
                    start: Position {
                        line: 45,
                        column: 81,
                        offset: 1525,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 45,
                        column: 97,
                        offset: 1541,
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
                        column: 97,
                        offset: 1541,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 45,
                        column: 98,
                        offset: 1542,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Reference,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 45,
                        column: 98,
                        offset: 1542,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 45,
                        column: 99,
                        offset: 1543,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("value"),
                span: Span {
                    start: Position {
                        line: 45,
                        column: 99,
                        offset: 1543,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 45,
                        column: 104,
                        offset: 1548,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Dot,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 45,
                        column: 104,
                        offset: 1548,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 45,
                        column: 105,
                        offset: 1549,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("span"),
                span: Span {
                    start: Position {
                        line: 45,
                        column: 105,
                        offset: 1549,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 45,
                        column: 109,
                        offset: 1553,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Dot,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 45,
                        column: 109,
                        offset: 1553,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 45,
                        column: 110,
                        offset: 1554,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("end"),
                span: Span {
                    start: Position {
                        line: 45,
                        column: 110,
                        offset: 1554,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 45,
                        column: 113,
                        offset: 1557,
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
                        column: 113,
                        offset: 1557,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 45,
                        column: 114,
                        offset: 1558,
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
                        column: 114,
                        offset: 1558,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 45,
                        column: 115,
                        offset: 1559,
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
                        column: 115,
                        offset: 1559,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 45,
                        column: 116,
                        offset: 1560,
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
                        column: 21,
                        offset: 1582,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 46,
                        column: 22,
                        offset: 1583,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Comma,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 46,
                        column: 22,
                        offset: 1583,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 46,
                        column: 23,
                        offset: 1584,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("OptionToken"),
                span: Span {
                    start: Position {
                        line: 48,
                        column: 21,
                        offset: 1608,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 48,
                        column: 32,
                        offset: 1619,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::DoubleColon,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 48,
                        column: 32,
                        offset: 1619,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 48,
                        column: 34,
                        offset: 1621,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("None"),
                span: Span {
                    start: Position {
                        line: 48,
                        column: 34,
                        offset: 1621,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 48,
                        column: 38,
                        offset: 1625,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::BraceOpen,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 48,
                        column: 39,
                        offset: 1626,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 48,
                        column: 40,
                        offset: 1627,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("println"),
                span: Span {
                    start: Position {
                        line: 49,
                        column: 25,
                        offset: 1653,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 49,
                        column: 32,
                        offset: 1660,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::ParenOpen,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 49,
                        column: 32,
                        offset: 1660,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 49,
                        column: 33,
                        offset: 1661,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::StringValue,
                value: TokenValue::String("No token produced"),
                span: Span {
                    start: Position {
                        line: 49,
                        column: 33,
                        offset: 1661,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 49,
                        column: 52,
                        offset: 1680,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::ParenClose,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 49,
                        column: 52,
                        offset: 1680,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 49,
                        column: 53,
                        offset: 1681,
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
                        column: 53,
                        offset: 1681,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 49,
                        column: 54,
                        offset: 1682,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Break,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 50,
                        column: 25,
                        offset: 1708,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 50,
                        column: 30,
                        offset: 1713,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Semicolon,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 50,
                        column: 30,
                        offset: 1713,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 50,
                        column: 31,
                        offset: 1714,
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
                        column: 21,
                        offset: 1736,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 51,
                        column: 22,
                        offset: 1737,
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
                        column: 17,
                        offset: 1755,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 52,
                        column: 18,
                        offset: 1756,
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
                        column: 13,
                        offset: 1770,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 53,
                        column: 14,
                        offset: 1771,
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
                        column: 9,
                        offset: 1781,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 54,
                        column: 10,
                        offset: 1782,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::BraceClose,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 55,
                        column: 5,
                        offset: 1788,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 55,
                        column: 6,
                        offset: 1789,
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
                        offset: 1896,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 58,
                        column: 8,
                        offset: 1899,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("output_path"),
                span: Span {
                    start: Position {
                        line: 58,
                        column: 9,
                        offset: 1900,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 58,
                        column: 20,
                        offset: 1911,
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
                        column: 21,
                        offset: 1912,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 58,
                        column: 22,
                        offset: 1913,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::StringValue,
                value: TokenValue::String("./selfhost/tokens.rs"),
                span: Span {
                    start: Position {
                        line: 58,
                        column: 23,
                        offset: 1914,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 58,
                        column: 45,
                        offset: 1936,
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
                        column: 45,
                        offset: 1936,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 58,
                        column: 46,
                        offset: 1937,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Let,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 59,
                        column: 5,
                        offset: 1943,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 59,
                        column: 8,
                        offset: 1946,
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
                        column: 9,
                        offset: 1947,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 59,
                        column: 18,
                        offset: 1956,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Assign,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 59,
                        column: 19,
                        offset: 1957,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 59,
                        column: 20,
                        offset: 1958,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::StringValue,
                value: TokenValue::String("fn main() {let foo = "),
                span: Span {
                    start: Position {
                        line: 59,
                        column: 21,
                        offset: 1959,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 59,
                        column: 44,
                        offset: 1982,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Plus,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 59,
                        column: 45,
                        offset: 1983,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 59,
                        column: 46,
                        offset: 1984,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("token_capture_debug"),
                span: Span {
                    start: Position {
                        line: 59,
                        column: 47,
                        offset: 1985,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 59,
                        column: 66,
                        offset: 2004,
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
                        column: 66,
                        offset: 2004,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 59,
                        column: 67,
                        offset: 2005,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Reference,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 59,
                        column: 67,
                        offset: 2005,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 59,
                        column: 68,
                        offset: 2006,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("token_capture"),
                span: Span {
                    start: Position {
                        line: 59,
                        column: 68,
                        offset: 2006,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 59,
                        column: 81,
                        offset: 2019,
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
                        column: 81,
                        offset: 2019,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 59,
                        column: 82,
                        offset: 2020,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Plus,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 59,
                        column: 83,
                        offset: 2021,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 59,
                        column: 84,
                        offset: 2022,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::StringValue,
                value: TokenValue::String(";}"),
                span: Span {
                    start: Position {
                        line: 59,
                        column: 85,
                        offset: 2023,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 59,
                        column: 89,
                        offset: 2027,
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
                        column: 89,
                        offset: 2027,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 59,
                        column: 90,
                        offset: 2028,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("write_file"),
                span: Span {
                    start: Position {
                        line: 60,
                        column: 5,
                        offset: 2034,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 60,
                        column: 15,
                        offset: 2044,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::ParenOpen,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 60,
                        column: 15,
                        offset: 2044,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 60,
                        column: 16,
                        offset: 2045,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("output_path"),
                span: Span {
                    start: Position {
                        line: 60,
                        column: 16,
                        offset: 2045,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 60,
                        column: 27,
                        offset: 2056,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Comma,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 60,
                        column: 27,
                        offset: 2056,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 60,
                        column: 28,
                        offset: 2057,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("rust_code"),
                span: Span {
                    start: Position {
                        line: 60,
                        column: 29,
                        offset: 2058,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 60,
                        column: 38,
                        offset: 2067,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::ParenClose,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 60,
                        column: 38,
                        offset: 2067,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 60,
                        column: 39,
                        offset: 2068,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Semicolon,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 60,
                        column: 39,
                        offset: 2068,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 60,
                        column: 40,
                        offset: 2069,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::BraceClose,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 61,
                        column: 1,
                        offset: 2071,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 61,
                        column: 2,
                        offset: 2072,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Identifier,
                value: TokenValue::String("main"),
                span: Span {
                    start: Position {
                        line: 63,
                        column: 1,
                        offset: 2076,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 63,
                        column: 5,
                        offset: 2080,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::ParenOpen,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 63,
                        column: 5,
                        offset: 2080,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 63,
                        column: 6,
                        offset: 2081,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::ParenClose,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 63,
                        column: 6,
                        offset: 2081,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 63,
                        column: 7,
                        offset: 2082,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::Semicolon,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 63,
                        column: 7,
                        offset: 2082,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 63,
                        column: 8,
                        offset: 2083,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
            Token {
                category: TokenCategory::ETX,
                value: TokenValue::Null,
                span: Span {
                    start: Position {
                        line: 64,
                        column: 1,
                        offset: 2085,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                    end: Position {
                        line: 64,
                        column: 1,
                        offset: 2085,
                        filename: OptionStr::Some("./selfhost/main.rp"),
                    },
                },
            },
        ],
    };
}
