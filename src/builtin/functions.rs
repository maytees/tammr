use crate::object::Object;
use std::collections::HashMap;

pub fn builtins() -> HashMap<String, Object> {
    let mut map = HashMap::new();

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
        "println".to_string(),
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
        "fprintln".to_string(),
        Object::BuiltinFunction(|args| {
            if args.is_empty() {
                return Object::Error(
                    "fprintln requires at least one argument (format string)".to_string(),
                );
            }

            let format_string = match &args[0] {
                Object::String(s) => s,
                _ => {
                    return Object::Error("First argument to fprintln must be a string".to_string())
                }
            };

            let mut result = String::new();
            let mut arg_index = 1;
            let mut chars = format_string.chars().peekable();

            while let Some(ch) = chars.next() {
                if ch == '{' {
                    if chars.peek() == Some(&'}') {
                        chars.next(); // consume the closing '}'
                        if arg_index < args.len() {
                            result.push_str(&args[arg_index].to_string());
                            arg_index += 1;
                        } else {
                            return Object::Error(
                                "Not enough arguments provided for format string".to_string(),
                            );
                        }
                    } else {
                        result.push(ch);
                    }
                } else if ch == '}' {
                    if chars.peek() == Some(&'}') {
                        chars.next(); // consume the second '}'
                        result.push('}');
                    } else {
                        return Object::Error("Invalid format string: unmatched '}'".to_string());
                    }
                } else {
                    result.push(ch);
                }
            }

            if arg_index < args.len() {
                return Object::Error("Too many arguments provided for format string".to_string());
            }

            println!("{}", result);
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

    map.insert(
        "pop".to_string(),
        Object::BuiltinFunction(|args| {
            if args.len() != 1 {
                return Object::Error(format!(
                    "Wrong number of arguments. Got {}, expected 1",
                    args.len()
                ));
            }

            match &args[0] {
                Object::Array(array) => {
                    let mut new_array = array.clone();
                    new_array.pop();
                    Object::Array(new_array)
                }
                _ => Object::Error(format!("Argument to `push` must be ARRAY, got {}", args[0])),
            }
        }),
    );

    map
}
