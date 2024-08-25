use crate::object::Object;

pub enum DotBuiltinKind {
    Property(String),
}

pub fn dot_str_builtins(string: &str, kind: DotBuiltinKind) -> Option<Object> {
    match kind {
        DotBuiltinKind::Property(name) => match name.as_str() {
            "length" => Some(Object::Integer(string.len() as i64)),
            "chars" => Some(Object::Array(
                string
                    .chars()
                    .map(|c| Object::String(c.to_string()))
                    .collect(),
            )),
            "bytes" => Some(Object::Array(
                string.chars().map(|c| Object::Integer(c as i64)).collect(),
            )),
            "is_empty" => Some(Object::Boolean(string.is_empty())),
            "is_numeric" => Some(Object::Boolean(
                string.chars().all(|c| c.is_numeric() || c.is_whitespace()),
            )),
            "is_alpha" => Some(Object::Boolean(
                string
                    .chars()
                    .all(|c| c.is_alphabetic() || c.is_whitespace()),
            )),
            "is_alphanumeric" => Some(Object::Boolean(
                string
                    .chars()
                    .all(|c| c.is_alphanumeric() || c.is_whitespace()),
            )),
            "is_ascii" => Some(Object::Boolean(string.is_ascii())),
            "is_capitalized" => Some(Object::Boolean(
                string.chars().next().unwrap().is_uppercase(),
            )),
            "is_lowercase" => Some(Object::Boolean(
                string
                    .chars()
                    .all(|c| c.is_lowercase() || c.is_whitespace()),
            )),
            "is_uppercase" => Some(Object::Boolean(
                string
                    .chars()
                    .all(|c| c.is_uppercase() || c.is_whitespace()),
            )),
            "is_titlecase" => Some(Object::Boolean(
                string
                    .chars()
                    .all(|c| c.is_uppercase() || c.is_whitespace()),
            )),
            "is_whitespace" => Some(Object::Boolean(
                string
                    .chars()
                    .all(|c| c.is_whitespace() || c.is_whitespace()),
            )),
            "is_punctuation" => Some(Object::Boolean(
                string
                    .chars()
                    .all(|c| c.is_ascii_punctuation() || c.is_whitespace()),
            )),
            _ => Some(Object::Error(format!("No property named {}", name))),
        },
    }
}
