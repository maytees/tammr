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

#[derive(Clone)]
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

pub enum Expression {
    Identifier(Identifier),
    Integer(Literal),
    Boolean(Literal),
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
}

impl std::fmt::Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Expression::Identifier(ident) => write!(f, "{}", ident.value),
            Expression::Integer(lit) => write!(f, "{}", lit),
            Expression::Prefix {
                operator, right, ..
            } => write!(f, "({}{})", operator, right),
            Expression::Infix {
                left,
                operator,
                right,
                ..
            } => write!(f, "({} {} {})", left, operator, right),
            Expression::Boolean(lit) => write!(f, "{}", lit),
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
        }
    }
}

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
