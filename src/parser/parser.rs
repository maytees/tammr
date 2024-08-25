use super::precedence::Precedence;
use crate::ast::Program;
use crate::lexer::{Token, TokenType};

pub struct Parser {
    pub current_token: Token,
    pub peek_token: Token,
    pub tokens: Vec<Token>,
    pub index: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            current_token: tokens[0].clone(),
            peek_token: tokens[1].clone(),
            tokens,
            index: 0,
        }
    }

    pub fn parse_program(&mut self) -> Option<Program> {
        let mut program: Program = Vec::new();
        while self.current_token.ttype != TokenType::Eof {
            let stmt = self.parse_statement();

            if let Some(stmt) = stmt {
                program.push(stmt);
            }

            self.next_token();
        }

        Some(program)
    }

    pub(crate) fn expect_peek(&mut self, ttype: TokenType) -> bool {
        if self.peek_token.ttype == ttype {
            self.next_token();
            true
        } else {
            false
        }
    }

    pub(crate) fn next_token(&mut self) {
        self.index += 1;
        self.current_token = self.tokens[self.index].clone();
        if self.index + 1 < self.tokens.len() {
            self.peek_token = self.tokens[self.index + 1].clone();
        }
    }

    pub(crate) fn token_precedence(&mut self, ttype: TokenType) -> Precedence {
        match ttype {
            TokenType::Assign | TokenType::NotEq | TokenType::Eq => Precedence::Equals,
            TokenType::Lt | TokenType::Gt => Precedence::LessGreater,
            TokenType::Add | TokenType::Sub => Precedence::Sum,
            TokenType::Div | TokenType::Mul => Precedence::Product,
            TokenType::LParen => Precedence::Call,
            TokenType::LBracket => Precedence::Index,
            TokenType::Period => Precedence::Dot,
            _ => Precedence::Lowest,
        }
    }

    pub(crate) fn cur_precedence(&mut self) -> Precedence {
        self.token_precedence(self.current_token.ttype.clone())
    }

    pub(crate) fn peek_precedence(&mut self) -> Precedence {
        self.token_precedence(self.peek_token.ttype.clone())
    }
}
