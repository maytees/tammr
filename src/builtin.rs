use std::collections::HashMap;

use crate::object::Object;

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
                _ => Object::Error(format!("Argument to `len` not supported, got {}", args[0])),
            }
        }),
    );

    map
}
