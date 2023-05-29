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
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Option<Expression> {
        let mut left = match self.current_token.ttype {
            TokenType::Ident => self.parse_identifier(),
            TokenType::Number => self.parse_integer_literal(),
            TokenType::Bang | TokenType::Sub => self.parse_prefix_expression(),
            TokenType::Keyword(KeywordType::True) | TokenType::Keyword(KeywordType::False) => {
                self.parse_boolean()
            }
            TokenType::LParen => self.parse_group_expr(),
            TokenType::Keyword(KeywordType::If) => self.parse_if_expr(),
            _ => return None,
        };

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
                | TokenType::Sub => self.parse_infix_expression(left.unwrap()),
                _ => return left,
            };
        }

        left
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
            TokenType::Assign | TokenType::NotEq => Precedence::Equals,
            TokenType::Lt | TokenType::Gt => Precedence::LessGreater,
            TokenType::Add | TokenType::Sub => Precedence::Sum,
            TokenType::Div | TokenType::Mul => Precedence::Product,
            TokenType::LParen => Precedence::Call,
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
        Some(Expression::Boolean(Literal::Boolean(
            self.current_token.ttype == TokenType::Keyword(KeywordType::True),
        )))
    }

    fn parse_integer_literal(&mut self) -> Option<Expression> {
        let int = self.current_token.literal.parse::<i64>().unwrap();
        let lit = Expression::Integer(Literal::Integer(int));

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

#[cfg(test)]
mod test {
    use super::Parser;
    use crate::lexer::Lexer;
    use crate::parser::Statement;

    #[test]
    fn if_statement() {
        let input = String::from(
            r#"
            if (x < y) {
                return x;
            } else {
                return y;
            }
            "#,
        );
        let mut l = Lexer::new(input);
        let tokens = l.gen_tokens();

        let mut p = Parser::new(tokens);
        let program = p.parse_program();
        if let Some(program) = program {
            if program.len() != 1 {
                panic!(
                    "Program does not contain 1 statement, got {}",
                    program.len()
                );
            }

            let stmt = &program[0];
            match stmt {
                Statement::Expression { value, .. } => {
                    if value.to_string() != "((x < y) {[return x;]} else [return y;])" {
                        panic!(
                            "Expected value to be ((x < y) {{[ return true; ]}} else {{[ return false; ]}}), got {}",
                            value
                        );
                    }
                }
                _ => {
                    panic!("Expected statement to be expression, got {:?}", stmt);
                }
            }
        } else {
            panic!("Parse program returned None");
        }
    }

    #[test]
    fn group_expr() {
        let input = String::from("(5 + 5) * 2;");
        let mut l = Lexer::new(input);
        let tokens = l.gen_tokens();

        let mut p = Parser::new(tokens);
        let program = p.parse_program();

        if let Some(program) = program {
            if program.len() != 1 {
                panic!(
                    "Program does not contain 1 statement, got {}, program: {:?}",
                    program.len(),
                    program
                );
            }

            let stmt = &program[0];
            match stmt {
                Statement::Expression { value, .. } => {
                    if value.to_string() != "((5 + 5) * 2)" {
                        panic!("Expected value to be ((5 + 5) * 2), got {}", value);
                    }
                }
                _ => {
                    panic!("Expected statement to be expression, got {:?}", stmt);
                }
            }
        } else {
            panic!("Parse program returned None");
        }
    }

    #[test]
    fn boolean_expr() {
        let input = String::from("true;");

        let mut l = Lexer::new(input);
        let tokens = l.gen_tokens();

        let mut p = Parser::new(tokens);
        let program = p.parse_program();

        if let Some(program) = program {
            if program.len() != 1 {
                panic!(
                    "Program does not contain 1 statement, got {}",
                    program.len()
                );
            }

            let stmt = &program[0];
            match stmt {
                Statement::Expression { value, .. } => {
                    if value.to_string() != "true" {
                        panic!("Expected value to be true, got {}", value);
                    }
                }
                _ => {
                    panic!("Expected statement to be expression, got {:?}", stmt);
                }
            }
        } else {
            panic!("Parse program returned None");
        }
    }

    #[test]
    fn infix_expr() {
        let input = String::from("5 + 5 * 2;");
        let mut l = Lexer::new(input);
        let tokens = l.gen_tokens();

        let mut p = Parser::new(tokens);
        let program = p.parse_program();

        if let Some(program) = program {
            if program.len() != 1 {
                panic!(
                    "Program does not contain 1 statement, got {}",
                    program.len()
                );
            }

            let stmt = &program[0];
            match stmt {
                Statement::Expression { value, .. } => {
                    if value.to_string() != "(5 + (5 * 2))" {
                        panic!("Expected value to be (5 + (5 * 2)), got {}", value);
                    }
                }
                _ => {
                    panic!("Expected statement to be expression, got {:?}", stmt);
                }
            }
        } else {
            panic!("Parse program returned None");
        }
    }

    #[test]
    fn prefix_expr() {
        let input = String::from("-5;");
        let mut l = Lexer::new(input);
        let tokens = l.gen_tokens();

        let mut p = Parser::new(tokens);
        let program = p.parse_program();

        if let Some(program) = program {
            if program.len() != 1 {
                panic!(
                    "Program does not contain 1 statement, got {}",
                    program.len()
                );
            }

            let stmt = &program[0];
            match stmt {
                Statement::Expression { value, .. } => {
                    if value.to_string() != "(-5)" {
                        panic!("Expected value to be -5, got {}", value);
                    }
                }
                _ => {
                    panic!("Expected statement to be expression, got {:?}", stmt);
                }
            }
        } else {
            panic!("Parse program returned None");
        }
    }

    #[test]
    fn integer_expr() {
        let input = String::from("5;");

        let mut l = Lexer::new(input);
        let tokens = l.gen_tokens();

        let mut p = Parser::new(tokens);
        let program = p.parse_program();

        if let Some(program) = program {
            if program.len() != 1 {
                panic!(
                    "Program does not contain 1 statement, got {}",
                    program.len()
                );
            }

            let stmt = &program[0];
            match stmt {
                Statement::Expression { value, .. } => {
                    if value.to_string() != "5" {
                        panic!("Expected value to be 5, got {}", value);
                    }
                }
                _ => {
                    panic!("Expected statement to be expression, got {:?}", stmt);
                }
            }
        } else {
            panic!("Parse program returned None");
        }
    }

    #[test]
    fn identifier_expr() {
        let input = String::from("foobar;");

        let mut l = Lexer::new(input);
        let tokens = l.gen_tokens();

        let mut p = Parser::new(tokens);
        let program = p.parse_program();

        if let Some(program) = program {
            if program.len() != 1 {
                panic!(
                    "Program does not contain 1 statement, got {}",
                    program.len()
                );
            }

            let stmt = &program[0];
            match stmt {
                Statement::Expression { value, .. } => {
                    if value.to_string() != "foobar" {
                        panic!("Expected value to be foobar, got {}", value);
                    }
                }
                _ => {
                    panic!("Expected statement to be expression, got {:?}", stmt);
                }
            }
        } else {
            panic!("Parse program returned None");
        }
    }

    #[test]
    fn return_statement() {
        let input = String::from(
            r#"
            return 5;
            return 10;
            return 993322;
            "#,
        );
        let mut l = Lexer::new(input);
        let tokens = l.gen_tokens();

        let mut p = Parser::new(tokens);
        let program = p.parse_program();
        if let Some(program) = program {
            if program.len() != 3 {
                panic!(
                    "Program does not contain 3 statements, got {}",
                    program.len()
                );
            }

            // let tests = vec!["5", "10", "993322"];

            // for (i, tt) in tests.iter().enumerate() {
            //     let stmt = &program[i];
            //     match stmt {
            //         Statement::return { value, .. } => {
            //             if value.to_string() != tt.to_string() {
            //                 panic!("Expected value to be {}, got {}", tt, value);
            //             }
            //         }
            //         _ => {
            //             panic!("Expected statement to be return, got {:?}", stmt);
            //         }
            //     }
            // }
        } else {
            panic!("Parse program returned None");
        }
    }

    #[test]
    fn let_statement() {
        let input = String::from(
            r#"
            let x = 5;
            let y = 10;
            let foobar = 838383;
            "#,
        );
        let mut l = Lexer::new(input);
        let tokens = l.gen_tokens();

        let mut p = Parser::new(tokens);
        let program = p.parse_program();

        if let Some(program) = program {
            if program.len() != 3 {
                panic!(
                    "Program does not contain 3 statements, got {}",
                    program.len()
                );
            }

            let tests = vec!["x", "y", "foobar"];

            for (i, tt) in tests.iter().enumerate() {
                let stmt = &program[i];
                match stmt {
                    Statement::Let { name, .. } => {
                        if name.value != *tt {
                            panic!("Expected name to be {}, got {}", tt, name);
                        }
                    }
                    _ => {
                        panic!("Expected statement to be let, got {:?}", stmt);
                    }
                }
            }
        } else {
            panic!("Parse program returned None");
        }
    }
}
