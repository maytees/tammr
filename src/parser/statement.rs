use super::precedence::Precedence;
use super::Parser;
use crate::ast::{BlockStatement, Identifier, Statement};
use crate::lexer::{KeywordType, PrimitiveKind, TokenType};

impl Parser {
    pub(crate) fn parse_statement(&mut self) -> Option<Statement> {
        match self.current_token.ttype {
            TokenType::Keyword(KeywordType::Let) => self.parse_let_statement(),
            TokenType::Keyword(KeywordType::Return) => self.parse_return_statement(),
            TokenType::Ident => {
                if self.peek_token.ttype == TokenType::Assign {
                    self.parse_reassign_statement()
                } else {
                    self.parse_expression_statement()
                }
            }
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_reassign_statement(&mut self) -> Option<Statement> {
        let name = Identifier {
            token: self.current_token.clone(),
            value: self.current_token.literal.clone(),
        };

        if !self.expect_peek(TokenType::Assign) {
            return None;
        }

        self.next_token();

        let value = self.parse_expression(Precedence::Lowest).unwrap();

        if self.peek_token.ttype == TokenType::Semicolon {
            self.next_token();
        }

        Some(Statement::ReAssign {
            token: self.current_token.clone(),
            name,
            value,
        })
    }

    fn parse_expression_statement(&mut self) -> Option<Statement> {
        let expr = self.parse_expression(Precedence::Lowest);

        if self.peek_token.ttype == TokenType::Semicolon {
            self.next_token();
        }

        if let Some(expr) = expr {
            Some(Statement::Expression {
                token: self.current_token.clone(),
                value: expr,
            })
        } else {
            None
        }
    }

    fn parse_return_statement(&mut self) -> Option<Statement> {
        let token = self.current_token.clone();

        self.next_token();

        let value = self.parse_expression(Precedence::Lowest).unwrap();

        if self.peek_token.ttype == TokenType::Semicolon {
            self.next_token();
        }

        Some(Statement::Return { token, value })
    }

    fn parse_let_statement(&mut self) -> Option<Statement> {
        let primitive_kind: Option<PrimitiveKind>;

        primitive_kind = match &self.peek_token.ttype {
            TokenType::Keyword(KeywordType::Primitive(p)) => Some(p.clone()),
            _ => None,
        };

        if primitive_kind.is_some() {
            self.next_token();
        }

        if !self.expect_peek(TokenType::Ident) {
            return None;
        }

        let name = Identifier {
            token: self.current_token.clone(),
            value: self.current_token.literal.clone(),
        };

        if !self.expect_peek(TokenType::Assign) {
            return None;
        }

        self.next_token();

        let value = self.parse_expression(Precedence::Lowest).unwrap();

        if self.peek_token.ttype == TokenType::Semicolon {
            self.next_token();
        }

        Some(Statement::Let {
            token: self.current_token.clone(),
            name,
            value,
            value_kind: primitive_kind.clone(),
        })
    }

    pub(crate) fn parse_block_statement(&mut self) -> BlockStatement {
        self.next_token();
        let mut block = Vec::new();

        while self.current_token.ttype != TokenType::RBrace
            && self.current_token.ttype != TokenType::Eof
        {
            let stmt = self.parse_statement();

            if let Some(stmt) = stmt {
                block.push(stmt);
            }

            self.next_token();
        }

        block
    }
}
