use crate::lexer::Token;

#[derive(Clone, Debug, PartialEq)]
pub struct Identifier {
    pub token: Token,
    pub value: String,
}

impl std::fmt::Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", &self.value)
    }
}

#[derive(Clone, PartialEq)]
pub enum Literal {
    Integer(i64),
    Boolean(bool),
}

impl std::fmt::Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Literal::Integer(int) => write!(f, "{}", int),
            Literal::Boolean(bool) => write!(f, "{}", bool),
        }
    }
}

#[derive(Clone, PartialEq)]
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
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum Statement {
    Let {
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
        }
    }
}

pub type BlockStatement = Vec<Statement>;
pub type Program = BlockStatement;
