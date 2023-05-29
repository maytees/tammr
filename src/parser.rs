use crate::ast::{Expression, Identifier, Program, Statement};
use crate::lexer::{KeywordType, Token, TokenType};
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

        while self.current_token.ttype != TokenType::EOF {
            let stmt = self.parse_statement();

            if let Some(stmt) = stmt {
                program.push(stmt);
            }

            self.next_token();
        }

        Some(program)
    }

    pub fn parse_statement(&mut self) -> Option<Statement> {
        match self.current_token.ttype {
            TokenType::KEYWORD(KeywordType::LET) => self.parse_let_statement(),
            _ => None,
        }
    }

    pub fn parse_let_statement(&mut self) -> Option<Statement> {
        if !self.expect_peek(TokenType::IDENT) {
            return None;
        }

        let name = Identifier {
            token: self.current_token.clone(),
            value: self.current_token.literal.clone(),
        };

        if !self.expect_peek(TokenType::ASSIGN) {
            return None;
        }

        // TODO: Skip expression until semicolon
        while self.current_token.ttype != TokenType::SEMICOLON {
            self.next_token();
        }

        Some(Statement::Let {
            token: self.current_token.clone(),
            name: name.clone(),
            value: Expression::Identifier(name), // TODO: huh
        })
    }

    pub fn expect_peek(&mut self, ttype: TokenType) -> bool {
        if self.peek_token.ttype == ttype {
            self.next_token();
            true
        } else {
            false
        }
    }

    pub fn next_token(&mut self) {
        self.index += 1;
        self.current_token = self.tokens[self.index].clone();
        if self.index + 1 < self.tokens.len() {
            self.peek_token = self.tokens[self.index + 1].clone();
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn let_statement() {
        use super::Parser;
        use crate::lexer::Lexer;
        use crate::parser::Statement;

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
                        if name.value != tt.to_string() {
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
