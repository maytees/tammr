use std::fmt;

#[derive(Clone, PartialEq, Eq)]
pub struct Position {
    line: usize,
    col: usize,
    index: usize,
}

impl Position {
    pub fn new() -> Self {
        Self {
            line: 0,
            col: 0,
            index: 0,
        }
    }
}

impl fmt::Debug for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "(line: {}, col: {}, index: {})",
            self.line, self.col, self.index
        )
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct Token {
    pub ttype: TokenType,
    pub literal: String,
    pub position: Position,
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[Type: {:?}, Literal: {}, Position: {:?}]",
            self.ttype, self.literal, self.position
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenType {
    Keyword(KeywordType),
    Ident,
    Number,
    Semicolon,
    Mul,
    Add,
    Sub,
    Div,
    Assign,
    NotEq,
    Colon,
    Lt,
    Gt,
    Eq,
    Bang,
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Comma,
    Period,
    String,
    Eof,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KeywordType {
    Let,
    Return,
    True,
    False,
    If,
    Else,
    Fn,
}

pub struct Lexer {
    src: String,
    position: Position,
    current: char,
}

const KEYWORDS: &[&str] = &["let", "return", "true", "false", "if", "else", "fn"];

impl Lexer {
    pub fn new(src: String) -> Self {
        Self {
            src: src.clone(),
            position: Position::new(),
            current: src.chars().next().unwrap_or('\0'),
        }
    }

    pub fn gen_tokens(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while self.current != '\0' {
            if self.current.is_whitespace() {
                self.advance();
                continue;
            }

            if self.current.is_alphabetic() {
                tokens.push(self.gen_ident());
                continue;
            }

            if self.current.is_numeric() {
                tokens.push(self.gen_number());
                continue;
            }

            let single = self.tokenize_single();

            if let Some(single) = single {
                tokens.push(single);
                self.advance();
                continue;
            }

            panic!(
                "Unknown token: {} ascci code: {}",
                self.current, self.current as u8
            );
        }

        tokens.push(Token {
            ttype: TokenType::Eof,
            literal: String::from(""),
            position: self.position.clone(),
        });
        tokens
    }

    fn tokenize_single(&mut self) -> Option<Token> {
        match self.current {
            ';' => Some(Token {
                ttype: TokenType::Semicolon,
                literal: String::from(";"),
                position: self.position.clone(),
            }),
            '+' => Some(Token {
                ttype: TokenType::Add,
                literal: String::from("+"),
                position: self.position.clone(),
            }),
            '-' => Some(Token {
                ttype: TokenType::Sub,
                literal: String::from("-"),
                position: self.position.clone(),
            }),
            '*' => Some(Token {
                ttype: TokenType::Mul,
                literal: String::from("*"),
                position: self.position.clone(),
            }),
            '.' => Some(Token {
                ttype: TokenType::Period,
                literal: String::from("."),
                position: self.position.clone(),
            }),
            '/' => Some(Token {
                ttype: TokenType::Div,
                literal: String::from("/"),
                position: self.position.clone(),
            }),
            '=' => {
                if self.peek() == '=' {
                    self.advance();
                    return Some(Token {
                        ttype: TokenType::Eq,
                        literal: String::from("=="),
                        position: self.position.clone(),
                    });
                }

                Some(Token {
                    ttype: TokenType::Assign,
                    literal: String::from("="),
                    position: self.position.clone(),
                })
            }
            '!' => {
                if self.peek() == '=' {
                    self.advance();
                    return Some(Token {
                        ttype: TokenType::NotEq,
                        literal: String::from("!="),
                        position: self.position.clone(),
                    });
                }

                Some(Token {
                    ttype: TokenType::Bang,
                    literal: String::from("!"),
                    position: self.position.clone(),
                })
            }
            '<' => Some(Token {
                ttype: TokenType::Lt,
                literal: String::from("<"),
                position: self.position.clone(),
            }),
            '>' => Some(Token {
                ttype: TokenType::Gt,
                literal: String::from(">"),
                position: self.position.clone(),
            }),
            '(' => Some(Token {
                ttype: TokenType::LParen,
                literal: String::from("("),
                position: self.position.clone(),
            }),
            ')' => Some(Token {
                ttype: TokenType::RParen,
                literal: String::from(")"),
                position: self.position.clone(),
            }),
            '{' => Some(Token {
                ttype: TokenType::LBrace,
                literal: String::from("{"),
                position: self.position.clone(),
            }),
            '}' => Some(Token {
                ttype: TokenType::RBrace,
                literal: String::from("}"),
                position: self.position.clone(),
            }),
            '[' => Some(Token {
                ttype: TokenType::LBracket,
                literal: String::from("["),
                position: self.position.clone(),
            }),
            ']' => Some(Token {
                ttype: TokenType::RBracket,
                literal: String::from("]"),
                position: self.position.clone(),
            }),
            ',' => Some(Token {
                ttype: TokenType::Comma,
                literal: String::from(","),
                position: self.position.clone(),
            }),
            ':' => Some(Token {
                ttype: TokenType::Colon,
                literal: String::from(":"),
                position: self.position.clone(),
            }),
            '"' => {
                let mut string = String::new();
                self.advance();

                while self.current != '"' {
                    if self.current == '\\' {
                        self.advance();
                        match self.current {
                            'n' => string.push('\n'),
                            't' => string.push('\t'),
                            'r' => string.push('\r'),
                            '\\' => string.push('\\'),
                            '"' => string.push('"'),
                            _ => panic!("Unknown escape character: {}", self.current),
                        }
                    } else {
                        string.push(self.current);
                    }
                    self.advance();
                }

                Some(Token {
                    ttype: TokenType::String,
                    literal: string,
                    position: self.position.clone(),
                })
            }
            _ => None,
        }
    }

    fn peek(&self) -> char {
        self.src
            .chars()
            .nth(self.position.index + 1)
            .unwrap_or('\0')
    }

    fn gen_ident(&mut self) -> Token {
        let mut ident = String::new();

        while self.current.is_alphabetic() || self.current == '_' {
            ident.push(self.current);
            self.advance();
        }

        if KEYWORDS.contains(&ident.as_str()) {
            let keyword = match ident.as_str() {
                "let" => KeywordType::Let,
                "return" => KeywordType::Return,
                "true" => KeywordType::True,
                "false" => KeywordType::False,
                "if" => KeywordType::If,
                "else" => KeywordType::Else,
                "fn" => KeywordType::Fn,
                _ => panic!("Unknown Keyword: {}", ident),
            };

            return Token {
                ttype: TokenType::Keyword(keyword),
                literal: ident,
                position: self.position.clone(),
            };
        }

        Token {
            ttype: TokenType::Ident,
            literal: ident,
            position: self.position.clone(),
        }
    }

    fn gen_number(&mut self) -> Token {
        let mut number = String::new();

        while self.current.is_numeric() {
            number.push(self.current);
            self.advance();
        }

        Token {
            ttype: TokenType::Number,
            literal: number,
            position: self.position.clone(),
        }
    }

    pub fn advance(&mut self) {
        self.position.index += 1;
        self.position.col += 1;
        self.current = self.src.chars().nth(self.position.index).unwrap_or('\0');

        if self.current == '\n' {
            self.position.line += 1;
            self.position.col = 0;
        }
    }
}
