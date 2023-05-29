use crate::lexer::Token;

#[derive(Clone)]
pub struct Identifier {
    pub token: Token,
    pub value: String,
}

impl std::fmt::Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", &self.value)
    }
}

pub enum Expression {
    Identifier(Identifier),
}

impl std::fmt::Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Expression::Identifier(ident) => write!(f, "{}", ident.value),
        }
    }
}

pub enum Statement {
    Let {
        token: Token,
        name: Identifier,
        value: Expression,
    },
}

impl std::fmt::Debug for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Statement::Let { name, value, .. } => {
                write!(f, "(LET; IDENT: {}; VALUE: {}) ", name, value)
            }
        }
    }
}

pub type Program = Vec<Statement>;
