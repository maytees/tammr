use crate::lexer::Token;

mod expression;
mod literal;
mod statement;

pub use expression::Expression;
pub use literal::Literal;
pub use statement::{BlockStatement, Program, Statement};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Identifier {
    pub token: Token,
    pub value: String,
}

impl std::fmt::Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", &self.value)
    }
}
