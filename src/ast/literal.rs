use super::expression::Expression;

#[derive(Clone, PartialEq, Eq)]
pub enum Literal {
    Integer(i64),
    Boolean(bool),
    String(String),
    Array(Vec<Expression>),
    Hash(Vec<(Expression, Expression)>),
}

impl std::fmt::Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Literal::Integer(int) => write!(f, "{}", int),
            Literal::Boolean(bool) => write!(f, "{}", bool),
            Literal::String(string) => write!(f, "{}", string),
            Literal::Array(array) => {
                write!(f, "[")?;
                for (i, expr) in array.iter().enumerate() {
                    if i == array.len() - 1 {
                        write!(f, "{}", expr)?;
                    } else {
                        write!(f, "{}, ", expr)?;
                    }
                }
                write!(f, "]")
            }
            Literal::Hash(hash) => {
                write!(f, "{{")?;
                for (i, (key, value)) in hash.iter().enumerate() {
                    if i == hash.len() - 1 {
                        write!(f, "{}: {}", key, value)?;
                    } else {
                        write!(f, "{}: {}, ", key, value)?;
                    }
                }
                write!(f, "}}")
            }
        }
    }
}
