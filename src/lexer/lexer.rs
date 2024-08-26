use std::{error::Error, fmt};

use super::{token::PrimitiveKind, KeywordType, Position, Token, TokenType};

pub struct Lexer {
    src: String,
    position: Position,
    current: char,
}

const KEYWORDS: &[&str] = &[
    "let", "function", "return", "if", "else", "do", "end", "loop", "exit", "true", "false",
    "null", "try", "catch", "throw", "and", "or", "not", "is", "import", "as", "foreach", "from",
    "to", "str", "number", "kv", "arr", "bool",
];

#[derive(Debug)]
pub enum LexerError {
    UnexpectedCharacter(char, Position),
    UnterminatedString(Position),
    // Add more error types as needed
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LexerError::UnexpectedCharacter(c, pos) => {
                write!(f, "Unexpected character '{}' at {:?}", c, pos)
            }
            LexerError::UnterminatedString(pos) => write!(f, "Unterminated string at {:?}", pos),
        }
    }
}

impl Error for LexerError {}

impl Lexer {
    pub fn new(src: String) -> Self {
        Self {
            src: src.clone(),
            position: Position::new(),
            current: src.chars().next().unwrap_or('\0'),
        }
    }
    pub fn gen_tokens(&mut self) -> Result<Vec<Token>, LexerError> {
        let mut tokens = Vec::new();

        while self.current != '\0' {
            if self.current.is_whitespace() {
                self.skip_whitespace();
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

            match self.tokenize_single() {
                Ok(Some(token)) => tokens.push(token),
                Ok(None) => {} // Comment case, just continue
                Err(e) => return Err(e),
            }

            self.advance();
        }

        tokens.push(Token {
            ttype: TokenType::Eof,
            literal: String::from(""),
            position: self.position.clone(),
        });

        Ok(tokens)
    }

    fn skip_whitespace(&mut self) {
        while self.current.is_whitespace() {
            self.advance();
        }
    }

    fn tokenize_single(&mut self) -> Result<Option<Token>, LexerError> {
        match self.current {
            ';' => Ok(Some(Token {
                ttype: TokenType::Semicolon,
                literal: String::from(";"),
                position: self.position.clone(),
            })),
            '+' => Ok(Some(Token {
                ttype: TokenType::Add,
                literal: String::from("+"),
                position: self.position.clone(),
            })),
            '-' => Ok(Some(Token {
                ttype: TokenType::Sub,
                literal: String::from("-"),
                position: self.position.clone(),
            })),
            '*' => Ok(Some(Token {
                ttype: TokenType::Mul,
                literal: String::from("*"),
                position: self.position.clone(),
            })),
            '.' => Ok(Some(Token {
                ttype: TokenType::Period,
                literal: String::from("."),
                position: self.position.clone(),
            })),
            '/' => {
                if self.peek() == '/' {
                    self.skip_single_line_comment();
                    Ok(None)
                } else if self.peek() == '*' {
                    self.skip_multi_line_comment()?;
                    Ok(None)
                } else {
                    Ok(Some(Token {
                        ttype: TokenType::Div,
                        literal: String::from("/"),
                        position: self.position.clone(),
                    }))
                }
            }
            '=' => {
                if self.peek() == '=' {
                    self.advance();
                    return Ok(Some(Token {
                        ttype: TokenType::Eq,
                        literal: String::from("=="),
                        position: self.position.clone(),
                    }));
                }

                Ok(Some(Token {
                    ttype: TokenType::Assign,
                    literal: String::from("="),
                    position: self.position.clone(),
                }))
            }
            '!' => {
                if self.peek() == '=' {
                    self.advance();
                    return Ok(Some(Token {
                        ttype: TokenType::NotEq,
                        literal: String::from("!="),
                        position: self.position.clone(),
                    }));
                }

                Ok(Some(Token {
                    ttype: TokenType::Bang,
                    literal: String::from("!"),
                    position: self.position.clone(),
                }))
            }
            '<' => Ok(Some(Token {
                ttype: TokenType::Lt,
                literal: String::from("<"),
                position: self.position.clone(),
            })),
            '>' => Ok(Some(Token {
                ttype: TokenType::Gt,
                literal: String::from(">"),
                position: self.position.clone(),
            })),
            '(' => Ok(Some(Token {
                ttype: TokenType::LParen,
                literal: String::from("("),
                position: self.position.clone(),
            })),
            ')' => Ok(Some(Token {
                ttype: TokenType::RParen,
                literal: String::from(")"),
                position: self.position.clone(),
            })),
            '{' => Ok(Some(Token {
                ttype: TokenType::LBrace,
                literal: String::from("{"),
                position: self.position.clone(),
            })),
            '}' => Ok(Some(Token {
                ttype: TokenType::RBrace,
                literal: String::from("}"),
                position: self.position.clone(),
            })),
            '[' => Ok(Some(Token {
                ttype: TokenType::LBracket,
                literal: String::from("["),
                position: self.position.clone(),
            })),
            ']' => Ok(Some(Token {
                ttype: TokenType::RBracket,
                literal: String::from("]"),
                position: self.position.clone(),
            })),
            ',' => Ok(Some(Token {
                ttype: TokenType::Comma,
                literal: String::from(","),
                position: self.position.clone(),
            })),
            ':' => Ok(Some(Token {
                ttype: TokenType::Colon,
                literal: String::from(":"),
                position: self.position.clone(),
            })),
            // '"' | '\'' => self.gen_string(),
            '"' | '\'' => {
                let mut string = String::new();
                let starting_quote_type = self.current.clone();
                self.advance();

                while self.current != starting_quote_type {
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

                Ok(Some(Token {
                    ttype: TokenType::String,
                    literal: string,
                    position: self.position.clone(),
                }))
            }
            c => Err(LexerError::UnexpectedCharacter(c, self.position.clone())),
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
                "function" => KeywordType::Fn, // Changed from "fn" to "function"
                "do" => KeywordType::Do,
                "end" => KeywordType::End,
                "loop" => KeywordType::Loop,
                "exit" => KeywordType::Exit,
                "null" => KeywordType::Null,
                "try" => KeywordType::Try,
                "catch" => KeywordType::Catch,
                "throw" => KeywordType::Throw,
                "and" => KeywordType::And,
                "or" => KeywordType::Or,
                "not" => KeywordType::Not,
                "is" => KeywordType::Is,
                "as" => KeywordType::As,
                "import" => KeywordType::Import,
                "from" => KeywordType::From,
                "to" => KeywordType::To,
                "foreach" => KeywordType::Foreach,
                "bool" => KeywordType::Primitive(PrimitiveKind::Boolean),
                "str" => KeywordType::Primitive(PrimitiveKind::String),
                "arr" => KeywordType::Primitive(PrimitiveKind::Array),
                "number" => KeywordType::Primitive(PrimitiveKind::Number),
                "kv" => KeywordType::Primitive(PrimitiveKind::Kv),
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

    fn skip_single_line_comment(&mut self) {
        while self.current != '\n' && self.current != '\0' {
            self.advance();
        }
    }

    fn skip_multi_line_comment(&mut self) -> Result<(), LexerError> {
        self.advance(); // Consume the '*'
        let start_position = self.position.clone();

        while !(self.current == '*' && self.peek() == '/') {
            if self.current == '\0' {
                return Err(LexerError::UnterminatedString(start_position)); // Reusing UnterminatedString for unterminated comment
            }
            self.advance();
        }

        self.advance(); // Consume the '*'
        self.advance(); // Consume the '/'
        Ok(())
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
