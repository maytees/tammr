#[derive(PartialEq, Debug)]
pub enum Object {
    Integer(i64),
    Boolean(bool),
    Null,
}

impl std::fmt::Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Object::Integer(int) => write!(f, "{}", int),
            Object::Boolean(bool) => write!(f, "{}", bool),
            Object::Null => write!(f, "null"),
        }
    }
}
