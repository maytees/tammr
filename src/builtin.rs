use std::collections::HashMap;

use crate::object::Object;

// will implement functions later
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

pub fn builtins() -> HashMap<String, Object> {
    let mut map = HashMap::new();

    // Len
    map.insert(
        "len".to_string(),
        Object::BuiltinFunction(|args| {
            if args.len() != 1 {
                return Object::Error(format!(
                    "Wrong number of arguments. Got {}, expected 1",
                    args.len()
                ));
            }

            match &args[0] {
                Object::String(string) => Object::Integer(string.len() as i64),
                Object::Array(array) => Object::Integer(array.len() as i64),
                _ => Object::Error(format!("Argument to `len` not supported, got {}", args[0])),
            }
        }),
    );

    map.insert(
        "first".to_string(),
        Object::BuiltinFunction(|args| {
            if args.len() != 1 {
                return Object::Error(format!(
                    "Wrong number of arguments. Got {}, expected 1",
                    args.len()
                ));
            }

            match &args[0] {
                Object::Array(array) => {
                    if !array.is_empty() {
                        return array[0].clone();
                    }
                    Object::Null
                }
                Object::String(string) => {
                    if !string.is_empty() {
                        return Object::String(string.chars().take(1).collect());
                    }
                    Object::Null
                }
                _ => Object::Error(format!(
                    "Argument to `first` must be ARRAY, got {}",
                    args[0]
                )),
            }
        }),
    );

    map.insert(
        "print".to_string(),
        Object::BuiltinFunction(|args| {
            println!(
                "{}",
                args.iter()
                    .map(|arg| format!("{} ", arg))
                    .collect::<String>()
            );
            Object::Empty
        }),
    );

    map.insert(
        "push".to_string(),
        Object::BuiltinFunction(|args| {
            if args.len() != 2 {
                return Object::Error(format!(
                    "Wrong number of arguments. Got {}, expected 2",
                    args.len()
                ));
            }

            match &args[0] {
                Object::Array(array) => {
                    let mut new_array = array.clone();
                    new_array.push(args[1].clone());
                    Object::Array(new_array)
                }
                _ => Object::Error(format!("Argument to `push` must be ARRAY, got {}", args[0])),
            }
        }),
    );

    map
}
