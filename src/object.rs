use crate::ast::{BlockStatement, Identifier};
use crate::env::Env;

#[derive(PartialEq, Debug, Clone, Eq)]
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
    BuiltinFunction(fn(Vec<Object>) -> Object),
    Array(Vec<Object>),
    Hash(Vec<(Object, Object)>),
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
            Object::BuiltinFunction(_) => write!(f, "builtin function"),
            Object::Array(array) => {
                write!(f, "[")?;
                for (i, obj) in array.iter().enumerate() {
                    if i == array.len() - 1 {
                        write!(f, "{}", obj)?;
                    } else {
                        write!(f, "{}, ", obj)?;
                    }
                }
                write!(f, "]")
            }
            Object::Hash(hash) => {
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
