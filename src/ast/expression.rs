use crate::lexer::Token;

use super::{literal::Literal, statement::BlockStatement, Identifier};

#[derive(Clone, PartialEq, Eq)]
pub enum Expression {
    Identifier(Identifier),
    Literal(Literal),
    Prefix {
        token: Token,
        operator: String,
        right: Box<Expression>,
    },
    Infix {
        token: Token, // Operator tok
        left: Box<Expression>,
        operator: String,
        right: Box<Expression>,
    },
    If {
        token: Token, // if tok
        condition: Box<Expression>,
        consequence: Box<BlockStatement>,
        alternative: Option<Box<BlockStatement>>,
    },
    FunctionLiteral {
        token: Token, // fn tok
        parameters: Vec<Identifier>,
        body: Box<BlockStatement>,
    },
    FunctionCall {
        token: Token,              // (
        function: Box<Expression>, // Identifier or FunctionLiteral
        arguments: Vec<Expression>,
    },
    IndexExpression {
        token: Token, // [
        left: Box<Expression>,
        index: Box<Expression>,
    },
    DotNotation {
        token: Token, // .
        left: Box<Expression>,
        right: Box<Expression>,
    },
}

impl std::fmt::Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Expression::Identifier(ident) => write!(f, "{}", ident.value),
            Expression::Literal(lit) => write!(f, "{}", lit),
            Expression::Prefix {
                operator, right, ..
            } => write!(f, "({}{})", operator, right),
            Expression::Infix {
                left,
                operator,
                right,
                ..
            } => write!(f, "({} {} {})", left, operator, right),
            Expression::If {
                condition,
                consequence,
                alternative,
                ..
            } => {
                write!(f, "({} {{{:?}}}", condition, consequence)?;
                if let Some(alt) = alternative {
                    write!(f, " else {:?})", alt)?;
                }
                Ok(())
            }
            Expression::FunctionLiteral {
                parameters, body, ..
            } => {
                write!(f, "fn(")?;
                for (i, param) in parameters.iter().enumerate() {
                    if i == parameters.len() - 1 {
                        write!(f, "{}", param)?;
                    } else {
                        write!(f, "{}, ", param)?;
                    }
                }
                write!(f, ") {{{:?}}}", body)
            }
            Expression::FunctionCall {
                function,
                arguments,
                ..
            } => {
                write!(f, "{}(", function)?;
                for (i, arg) in arguments.iter().enumerate() {
                    if i == arguments.len() - 1 {
                        write!(f, "{}", arg)?;
                    } else {
                        write!(f, "{}, ", arg)?;
                    }
                }
                write!(f, ")")
            }
            Expression::IndexExpression { left, index, .. } => write!(f, "({}[{}])", left, index),
            Expression::DotNotation { left, right, .. } => write!(f, "({}.{})", left, right),
        }
    }
}
