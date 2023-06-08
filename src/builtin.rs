use std::collections::HashMap;

use crate::object::Object;

// will implement functions later
pub enum DotBuiltinKind {
    Property(String),
}

fn string_to_array(string: &str) -> Object {
    let mut arr: Vec<Object> = Vec::new();

    for char in string.chars() {
        arr.push(Object::String(char.to_string()))
    }

    Object::Array(arr)
}

fn string_to_byte_array(string: &str) -> Object {
    let mut arr: Vec<Object> = Vec::new();

    for char in string.chars() {
        arr.push(Object::Integer((char as u8) as i64))
    }

    Object::Array(arr)
}

pub fn dot_str_builtins(string: &str, kind: DotBuiltinKind) -> Option<Object> {
    match kind {
        DotBuiltinKind::Property(name) => match name.as_str() {
            "length" => Some(Object::Integer(string.len() as i64)),
            "chars" => Some(string_to_array(string)),
            "bytes" => Some(string_to_byte_array(string)),
            "is_empty" => Some(Object::Boolean(string.is_empty())),
            "is_numeric" => Some(Object::Boolean(string.chars().all(|c| c.is_numeric()))),
            "is_alpha" => Some(Object::Boolean(string.chars().all(|c| c.is_alphabetic()))),
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
