use std::fmt;

#[derive(Clone)]
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

#[derive(Clone)]
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

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    KEYWORD,
    IDENT,
    NUMBER,
    SEMICOLON,
    MUL,
    ADD,
    SUB,
    DIV,
    ASSIGN,
    EOF,
}

pub struct Lexer {
    src: String,
    position: Position,
    current: char,
}

const KEYWORDS: &'static [&'static str] = &["let"];

impl Lexer {
    pub fn new(src: String) -> Self {
        Self {
            src: src.clone(),
            position: Position::new(),
            current: src.chars().nth(0).unwrap_or('\0'),
        }
    }

    pub fn gen_tokens(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while self.current != '\0' {
            if self.current == ' ' || self.current == '\n' {
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

            panic!("Unknown token: {}", self.current);
        }

        tokens.push(Token {
            ttype: TokenType::EOF,
            literal: String::from(""),
            position: self.position.clone(),
        });
        tokens
    }

    fn tokenize_single(&mut self) -> Option<Token> {
        return match self.current {
            '=' => Some(Token {
                ttype: TokenType::ASSIGN,
                literal: String::from("="),
                position: self.position.clone(),
            }),
            ';' => Some(Token {
                ttype: TokenType::SEMICOLON,
                literal: String::from(";"),
                position: self.position.clone(),
            }),
            '+' => Some(Token {
                ttype: TokenType::ADD,
                literal: String::from("+"),
                position: self.position.clone(),
            }),
            '-' => Some(Token {
                ttype: TokenType::SUB,
                literal: String::from("-"),
                position: self.position.clone(),
            }),
            '*' => Some(Token {
                ttype: TokenType::MUL,
                literal: String::from("*"),
                position: self.position.clone(),
            }),
            '/' => Some(Token {
                ttype: TokenType::DIV,
                literal: String::from("/"),
                position: self.position.clone(),
            }),
            _ => None,
        };
    }

    fn gen_ident(&mut self) -> Token {
        let mut ident = String::new();

        while self.current.is_alphabetic() {
            ident.push(self.current);
            self.advance();
        }

        if KEYWORDS.contains(&ident.as_str()) {
            return Token {
                ttype: TokenType::KEYWORD,
                literal: ident,
                position: self.position.clone(),
            };
        }

        Token {
            ttype: TokenType::IDENT,
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
            ttype: TokenType::NUMBER,
            literal: number,
            position: self.position.clone(),
        }
    }

    pub fn advance(&mut self) {
        self.position.index += 1;
        self.position.col += 1;
        self.current = self
            .src
            .chars()
            .nth(self.position.index as usize)
            .unwrap_or('\0');

        if self.current == '\n' {
            self.position.line += 1;
            self.position.col = 0;
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn lexer_test() {
        use super::Lexer;
        use super::TokenType;

        let input = String::from("let five = 5;");
        let mut l = Lexer::new(input);
        let tokens = l.gen_tokens();

        assert_eq!(tokens[0].ttype, TokenType::KEYWORD);
        assert_eq!(tokens[0].literal, String::from("let"));
        assert_eq!(tokens[1].ttype, TokenType::IDENT);
        assert_eq!(tokens[1].literal, String::from("five"));
        assert_eq!(tokens[2].ttype, TokenType::ASSIGN);
        assert_eq!(tokens[2].literal, String::from("="));
        assert_eq!(tokens[3].ttype, TokenType::NUMBER);
        assert_eq!(tokens[3].literal, String::from("5"));
        assert_eq!(tokens[4].ttype, TokenType::SEMICOLON);
        assert_eq!(tokens[4].literal, String::from(";"));
    }
}
