use crate::ast::{BlockStatement, Expression, Identifier, Literal, Program, Statement};
use crate::lexer::{KeywordType, Token, TokenType};

// Partial ord allows for < >, etc comparisons
#[derive(PartialOrd, PartialEq)]
enum Precedence {
    Lowest,
    Equals,      // ==
    LessGreater, // > or <
    Sum,         // +
    Product,     // *
    Prefix,      // -X or !X
    Call,        // myFunction(X)
    Index,       // array[index]
    Dot,         // x.y
}

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

    fn parse_statement(&mut self) -> Option<Statement> {
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

    fn parse_expression(&mut self, precedence: Precedence) -> Option<Expression> {
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

    fn parse_block_statement(&mut self) -> BlockStatement {
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

    fn token_precedence(&mut self, ttype: TokenType) -> Precedence {
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

    fn cur_precedence(&mut self) -> Precedence {
        self.token_precedence(self.current_token.ttype.clone())
    }

    fn peek_precedence(&mut self) -> Precedence {
        self.token_precedence(self.peek_token.ttype.clone())
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
        })
    }

    fn expect_peek(&mut self, ttype: TokenType) -> bool {
        if self.peek_token.ttype == ttype {
            self.next_token();
            true
        } else {
            false
        }
    }

    fn next_token(&mut self) {
        self.index += 1;
        self.current_token = self.tokens[self.index].clone();
        if self.index + 1 < self.tokens.len() {
            self.peek_token = self.tokens[self.index + 1].clone();
        }
    }
}
