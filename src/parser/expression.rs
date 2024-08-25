// parser/expression.rs
use super::precedence::Precedence;
use super::Parser;
use crate::ast::{Expression, Identifier, Literal};
use crate::lexer::{KeywordType, TokenType};

impl Parser {
    pub(crate) fn parse_expression(&mut self, precedence: Precedence) -> Option<Expression> {
        // Prefix
        let mut left = match self.current_token.ttype {
            TokenType::Ident => self.parse_identifier(),
            TokenType::String => self.parse_string_literal(),
            TokenType::Number => self.parse_integer_literal(),
            TokenType::Bang | TokenType::Sub => self.parse_prefix_expression(),
            TokenType::Keyword(KeywordType::True) | TokenType::Keyword(KeywordType::False) => {
                self.parse_boolean()
            }
            TokenType::LBrace => self.parse_hash_expr(),
            TokenType::LParen => self.parse_group_expr(),
            TokenType::LBracket => self.parse_array_literal(),
            TokenType::Keyword(KeywordType::If) => self.parse_if_expr(),
            TokenType::Keyword(KeywordType::Fn) => self.parse_fn_literal(),
            _ => return None,
        };

        // Infix
        while self.peek_token.ttype != TokenType::Semicolon && precedence < self.peek_precedence() {
            self.next_token();

            left = match self.current_token.ttype {
                TokenType::Add
                | TokenType::Assign
                | TokenType::Div
                | TokenType::Gt
                | TokenType::Lt
                | TokenType::Mul
                | TokenType::NotEq
                | TokenType::Eq
                | TokenType::Sub => self.parse_infix_expression(left.unwrap()),
                TokenType::LParen => self.parse_fn_call(left.unwrap()),
                TokenType::LBracket => self.parse_index_expression(left.unwrap()),
                TokenType::Period => self.parse_dot_notation(left.unwrap()),
                _ => return left,
            };
        }

        left
    }

    fn parse_dot_notation(&mut self, left: Expression) -> Option<Expression> {
        self.next_token();

        let right = self.parse_expression(Precedence::Dot);

        if let Some(right) = right {
            Some(Expression::DotNotation {
                token: self.current_token.clone(),
                left: Box::new(left),
                right: Box::new(right),
            })
        } else {
            None
        }
    }

    fn parse_fn_call(&mut self, function: Expression) -> Option<Expression> {
        Some(Expression::FunctionCall {
            token: self.current_token.clone(),
            function: Box::new(function),
            arguments: self.parse_fn_arguments(),
        })
    }

    fn parse_hash_expr(&mut self) -> Option<Expression> {
        let mut pairs: Vec<(Expression, Expression)> = Vec::new();

        while self.peek_token.ttype != TokenType::RBrace {
            self.next_token();

            let key = self.parse_expression(Precedence::Lowest).unwrap();

            if !self.expect_peek(TokenType::Colon) {
                return None;
            }

            self.next_token();
            let value = self.parse_expression(Precedence::Lowest).unwrap();

            pairs.push((key, value));

            if self.peek_token.ttype != TokenType::RBrace && !self.expect_peek(TokenType::Comma) {
                return None;
            }
        }

        if !self.expect_peek(TokenType::RBrace) {
            return None;
        }

        Some(Expression::Literal(Literal::Hash(pairs)))
    }

    fn parse_index_expression(&mut self, left: Expression) -> Option<Expression> {
        self.next_token();

        let index = self.parse_expression(Precedence::Lowest);

        if !self.expect_peek(TokenType::RBracket) {
            return None;
        }

        Some(Expression::IndexExpression {
            token: self.current_token.clone(),
            left: Box::new(left),
            index: Box::new(index.unwrap()),
        })
    }

    fn parse_array_literal(&mut self) -> Option<Expression> {
        Some(Expression::Literal(Literal::Array(
            self.parse_array_elements(),
        )))
    }

    fn parse_array_elements(&mut self) -> Vec<Expression> {
        let mut elements = Vec::new();

        if self.peek_token.ttype == TokenType::RBracket {
            self.next_token();
            return elements;
        }

        self.next_token();

        elements.push(self.parse_expression(Precedence::Lowest).unwrap());

        while self.peek_token.ttype == TokenType::Comma {
            self.next_token();
            self.next_token();

            elements.push(self.parse_expression(Precedence::Lowest).unwrap());
        }

        if !self.expect_peek(TokenType::RBracket) {
            return Vec::new();
        }

        elements
    }

    fn parse_string_literal(&mut self) -> Option<Expression> {
        Some(Expression::Literal(Literal::String(
            self.current_token.literal.clone(),
        )))
    }

    fn parse_fn_arguments(&mut self) -> Vec<Expression> {
        let mut args = Vec::new();

        if self.peek_token.ttype == TokenType::RParen {
            self.next_token();
            return args;
        }

        self.next_token();

        args.push(self.parse_expression(Precedence::Lowest).unwrap());

        while self.peek_token.ttype == TokenType::Comma {
            self.next_token();
            self.next_token();

            args.push(self.parse_expression(Precedence::Lowest).unwrap());
        }

        if !self.expect_peek(TokenType::RParen) {
            return Vec::new();
        }

        args
    }

    fn parse_fn_literal(&mut self) -> Option<Expression> {
        let token = self.current_token.clone();

        if !self.expect_peek(TokenType::LParen) {
            return None;
        }

        let parameters = self.parse_fn_parameters();

        if !self.expect_peek(TokenType::LBrace) {
            return None;
        }

        let body = self.parse_block_statement();

        Some(Expression::FunctionLiteral {
            token,
            parameters,
            body: Box::new(body),
        })
    }

    fn parse_fn_parameters(&mut self) -> Vec<Identifier> {
        let mut identifiers = Vec::new();

        if self.peek_token.ttype == TokenType::RParen {
            self.next_token();
            return identifiers;
        }

        self.next_token();

        let ident = Identifier {
            token: self.current_token.clone(),
            value: self.current_token.literal.clone(),
        };

        identifiers.push(ident);

        while self.peek_token.ttype == TokenType::Comma {
            self.next_token();
            self.next_token();

            let ident = Identifier {
                token: self.current_token.clone(),
                value: self.current_token.literal.clone(),
            };

            identifiers.push(ident);
        }

        if !self.expect_peek(TokenType::RParen) {
            return Vec::new();
        }

        identifiers
    }

    fn parse_if_expr(&mut self) -> Option<Expression> {
        let token = self.current_token.clone();

        self.next_token();
        let condition = self.parse_expression(Precedence::Lowest);

        if !self.expect_peek(TokenType::LBrace) {
            return None;
        }

        let consequence = self.parse_block_statement();

        let mut alternative = None;

        if self.peek_token.ttype == TokenType::Keyword(KeywordType::Else) {
            self.next_token();

            if !self.expect_peek(TokenType::LBrace) {
                return None;
            }

            alternative = Some(self.parse_block_statement());
        }

        Some(Expression::If {
            token,
            condition: Box::new(condition.unwrap()),
            consequence: Box::new(consequence),
            alternative: alternative.map(Box::new),
        })
    }

    fn parse_group_expr(&mut self) -> Option<Expression> {
        self.next_token();

        let expr = self.parse_expression(Precedence::Lowest);

        if !self.expect_peek(TokenType::RParen) {
            return None;
        }

        expr
    }

    fn parse_infix_expression(&mut self, left: Expression) -> Option<Expression> {
        let operator = self.current_token.literal.clone();
        let precedence = self.cur_precedence();

        self.next_token();

        let right = self.parse_expression(precedence);

        if let Some(right) = right {
            Some(Expression::Infix {
                token: self.current_token.clone(),
                left: Box::new(left),
                operator,
                right: Box::new(right),
            })
        } else {
            None
        }
    }

    fn parse_prefix_expression(&mut self) -> Option<Expression> {
        let operator = self.current_token.literal.clone();

        self.next_token();

        let right = self.parse_expression(Precedence::Prefix);

        if let Some(right) = right {
            Some(Expression::Prefix {
                token: self.current_token.clone(),
                operator,
                right: Box::new(right),
            })
        } else {
            None
        }
    }

    fn parse_boolean(&mut self) -> Option<Expression> {
        Some(Expression::Literal(Literal::Boolean(
            self.current_token.ttype == TokenType::Keyword(KeywordType::True),
        )))
    }

    fn parse_integer_literal(&mut self) -> Option<Expression> {
        let int = self.current_token.literal.parse::<i64>().unwrap();
        let lit = Expression::Literal(Literal::Integer(int));

        Some(lit)
    }

    fn parse_identifier(&mut self) -> Option<Expression> {
        Some(Expression::Identifier(Identifier {
            token: self.current_token.clone(),
            value: self.current_token.literal.clone(),
        }))
    }
}
