use super::Position;
use std::fmt;

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
