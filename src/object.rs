use crate::ast::{BlockStatement, Identifier};
use crate::env::Env;

#[derive(PartialEq, Debug, Clone)]

pub enum Object {
    Integer(i64),
    Boolean(bool),
    Null,
    Return(Box<Object>),
    Error(String),
    Empty,
    Function {
        parameters: Vec<Identifier>,
        body: BlockStatement,
        env: Env,
    },
    String(String),
}

impl std::fmt::Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Object::Integer(int) => write!(f, "{}", int),
            Object::Boolean(bool) => write!(f, "{}", bool),
            Object::Null => write!(f, "null"),
            Object::Return(obj) => write!(f, "{}", obj),
            Object::Error(msg) => write!(f, "{}", msg),
            Object::Empty => write!(f, ""),
            Object::Function {
                parameters,
                body,
                env: _,
            } => {
                let mut params = String::new();
                for param in parameters {
                    params.push_str(&format!("{}, ", param));
                }
                write!(f, "fn({}) {{\n{:?}\n}}", params, body)
            }
            Object::String(string) => write!(f, "{}", string),
        }
    }
}
