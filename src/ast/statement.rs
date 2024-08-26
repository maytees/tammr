use crate::lexer::{PrimitiveKind, Token};

use super::{expression::Expression, Identifier};

#[derive(Clone, PartialEq, Eq)]
pub enum Statement {
    Let {
        token: Token,
        name: Identifier,
        value: Expression,
        value_kind: Option<PrimitiveKind>,
    },
    ReAssign {
        token: Token,
        name: Identifier,
        value: Expression,
    },
    Return {
        token: Token,
        value: Expression,
    },
    Expression {
        token: Token,
        value: Expression,
    },
}

impl std::fmt::Debug for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Statement::Let { name, value, .. } => {
                write!(f, "let {} = {}) ", name, value)
            }
            Statement::Return { value, .. } => write!(f, "return {};", value),
            Statement::Expression { value, .. } => write!(f, "{}", value),
            Statement::ReAssign { name, value, .. } => {
                write!(f, "reassign {} = {}", name, value)
            }
        }
    }
}

pub type BlockStatement = Vec<Statement>;
pub type Program = BlockStatement;
