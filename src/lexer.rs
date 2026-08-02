pub enum TokenType {
    Identifier(String),
    Number(i64),
    String(String),

    LeftParen,
    RightParen,
    Dot,
    Equals,
    Semicolon,
    QuotationMark,
    LeftSquareBracket,
    RightSquareBracket,
    LeftCurlyBracket,
    RightCurlyBracket,
    Comma,
    At,

    KeywordIf,
    KeywordElse,
    KeywordFor,

    EOF,
}

#[allow(non_snake_case)]
pub struct Token {
    pub tokenType: TokenType,
    pub line: usize,
    pub column: usize,
}

#[allow(non_snake_case)]
pub fn lex(source: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut ptr = 0;

    let mut line = 0;
    let mut column = 0;
    loop {
        let char = source.chars().nth(ptr);
        match char {
            Some(c) => {
                match c {

                    // ( and )
                    '(' => {
                        tokens.push(Token {
                            tokenType: TokenType::LeftParen,
                            line: line,
                            column: column
                        });
                        ptr += 1;
                        column += 1;
                    }

                    ')' => {
                        tokens.push(Token {
                            tokenType: TokenType::RightParen,
                            line: line,
                            column: column
                        });
                        ptr += 1;
                        column += 1;
                    }
                    
                    // Decimal point
                    '.' => {
                        tokens.push(Token {
                            tokenType: TokenType::Dot,
                            line: line,
                            column: column
                        });
                        ptr += 1;
                        column += 1;
                    }

                    // Equals
                    '=' => {
                        tokens.push(Token {
                            tokenType: TokenType::Equals,
                            line: line,
                            column: column
                        });
                        ptr += 1;
                        column += 1;
                    }

                    // Semocolon
                    ';' => {
                        tokens.push(Token {
                            tokenType: TokenType::Semicolon,
                            line: line,
                            column: column
                        });
                        ptr += 1;
                        column += 1;
                    }

                    // Quotation marks (' or ")
                    '"' | '\'' => {
                        tokens.push(Token {
                            tokenType: TokenType::QuotationMark,
                            line: line,
                            column: column
                        });
                        ptr += 1;
                        column += 1;
                    }

                    // [ and ]
                    '[' => {
                        tokens.push(Token {
                            tokenType: TokenType::LeftSquareBracket,
                            line: line,
                            column: column
                        });
                        ptr += 1;
                        column += 1;
                    }
                    ']' => {
                        tokens.push(Token {
                            tokenType: TokenType::RightSquareBracket,
                            line: line,
                            column: column
                        });
                        ptr += 1;
                        column += 1;
                    }

                    // { and }
                    '{' => {
                        tokens.push(Token {
                            tokenType: TokenType::LeftCurlyBracket,
                            line: line,
                            column: column
                        });
                        ptr += 1;
                        column += 1;
                    }
                    '}' => {
                        tokens.push(Token {
                            tokenType: TokenType::RightCurlyBracket,
                            line: line,
                            column: column
                        });
                        ptr += 1;
                        column += 1;
                    }

                    // Comma
                    ',' => {
                        tokens.push(Token {
                            tokenType: TokenType::Comma,
                            line: line,
                            column: column
                        });
                        ptr += 1;
                        column += 1;
                    }

                    // @ Symbol
                    '@' => {
                        tokens.push(Token {
                            tokenType: TokenType::At,
                            line: line,
                            column: column
                        });
                        ptr += 1;
                        column += 1;
                    }

                    // Newline
                    '\n' => {
                        line += 1;
                        column = 0;
                        ptr += 1;
                    }

                    _ => {
                        // Number
                        if c.is_digit(10) {
                            let mut number = String::new();
                            while let Some(d) = source.chars().nth(ptr) {
                                if d.is_digit(10) {
                                    number.push(d);
                                    ptr += 1;
                                    column += 1;
                                } else {
                                    break;
                                }
                            }
                            tokens.push(Token {
                                tokenType: TokenType::Number(number.parse().unwrap()),
                                line: line,
                                column: column
                            });
                        }

                        // String
                        else if c == '"' || c == '\'' {
                            let quote = c;
                            let mut string = String::new();
                            ptr += 1;
                            column += 1;
                            while let Some(s) = source.chars().nth(ptr) {
                                if s != quote {
                                    string.push(s);
                                    ptr += 1;
                                    column += 1;
                                }
                                else {
                                    ptr += 1;
                                    column += 1;
                                    break;
                                }
                            }
                            tokens.push(Token {
                                tokenType: TokenType::String(string),
                                line: line,
                                column: column
                            });
                        }

                        // Keywords or identifier
                        else if c.is_alphabetic() || c == '_' || c == '*'{
                            let mut identifier = String::new();
                            while let Some(i) = source.chars().nth(ptr) {
                                if i.is_alphabetic() || i == '_' || i == '*' || i.is_digit(10) {
                                    identifier.push(i);
                                    ptr += 1;
                                    column += 1;
                                }
                                else {
                                    break;
                                }
                            }

                            match identifier.as_str() {
                                "if" => {
                                    tokens.push(Token {
                                        tokenType: TokenType::KeywordIf,
                                        line: line,
                                        column: column
                                    })
                                }
                                "else" => {
                                    tokens.push(Token {
                                        tokenType: TokenType::KeywordElse,
                                        line: line,
                                        column: column
                                    })
                                }
                                "for" => {
                                    tokens.push(Token {
                                        tokenType: TokenType::KeywordFor,
                                        line: line,
                                        column: column
                                    })
                                }
                                _ => {
                                    tokens.push(Token {
                                        tokenType: TokenType::Identifier(identifier),
                                        line: line,
                                        column: column
                                    })
                                }
                            }
                        }
                        else {
                            ptr += 1;
                            column += 1;
                        }
                    }

                }
            }
            None => {
                tokens.push(Token {
                    tokenType: TokenType::EOF,
                    line: line,
                    column: column
                });
                break tokens;
            }
        }
    }
}

#[allow(non_snake_case)]
pub fn printToken(token: &Token) {
    match &token.tokenType {
        // Literals
        TokenType::Identifier(name) => { println!("IDENTIFIER: {name}") }
        TokenType::Number(value) => { println!("NUMBER: {value}") }
        TokenType::String(value) => { println!("STRING: {value}") }

        // Symbols
        TokenType::LeftParen => { println!("(") }
        TokenType::RightParen => { println!(")") }
        TokenType::Dot => { println!(".") }
        TokenType::Equals => { println!("=") }
        TokenType::Semicolon => { println!(";") }
        TokenType::QuotationMark => { println!("\"") }
        TokenType::LeftSquareBracket => { println!("[") }
        TokenType::RightSquareBracket => { println!("]") }
        TokenType::LeftCurlyBracket => { println!("{{") }
        TokenType::RightCurlyBracket => { println!("}}") }
        TokenType::Comma => { println!(",") }
        TokenType::At => { println!("@") }

        // Keywords
        TokenType::KeywordIf => { println!("KEYWORD_IF") }
        TokenType::KeywordElse => { println!("KEYWORD_ELSE") }
        TokenType::KeywordFor => { println!("KEYWORD_FOR") }

        // End of file
        TokenType::EOF => { println!("EOF") }
    }
}