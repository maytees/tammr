#[cfg(test)]
mod test {
    use tammr::ast::Program;
    use tammr::lexer::Lexer;
    use tammr::object::Object;
    use tammr::parser::Parser;

    use tammr::eval::Evaluator;

    #[test]
    fn test_dot_notation() {
        let tests = vec![
            (
                r#"
                let kv person = {"name": "Joe"};
                person.name
                "#,
                Object::String("Joe".to_string()),
            ),
            (
                r#"
            let person = {"name": "Joe", "age": 90};
            person.age
            "#,
                Object::Integer(90),
            ),
        ];

        for (input, expected) in tests {
            // create new lexer with input
            let mut l = Lexer::new(input.to_string());
            // generate tokens from lexer
            let tokens = l.gen_tokens();

            // create new parser with tokens
            let mut parser = Parser::new(tokens.expect("Could not tokenize"));
            // parse program from parser
            let program: Option<Program> = parser.parse_program();

            // if program exists
            if let Some(program) = program {
                // create new evaluator
                let mut evaluator = Evaluator::new();
                // evaluate program
                if let Some(result) = evaluator.eval(&program) {
                    // assert that result is equal to expected
                    println!("{} - {}", result, expected);
                    assert_eq!(result, expected);
                } else {
                    panic!("No result");
                }
            }
        }
    }

    #[test]
    fn test_hash_index() {
        let tests = vec![
            (
                r#"
                let kv myHash = {"one": 1, "two": 2};
                myHash["one"]
                "#,
                Object::Integer(1),
            ),
            (
                r#"
                let kv myHash = {"one": 1, "two": 2};
                myHash["two"]
                "#,
                Object::Integer(2),
            ),
            (
                r#"
                let kv myHash = {"one": 1, "two": 2};
                myHash["three"]
                "#,
                Object::Null,
            ),
            (
                r#"
                let kv myHash = {"one": 1, "two": 2};
                myHash["one"] + myHash["two"]
                "#,
                Object::Integer(3),
            ),
        ];

        for (input, expected) in tests {
            // create new lexer with input
            let mut l = Lexer::new(input.to_string());
            // generate tokens from lexer
            let tokens = l.gen_tokens();

            // create new parser with tokens
            let mut parser = Parser::new(tokens.expect("Could not tokenize"));
            // parse program from parser
            let program: Option<Program> = parser.parse_program();

            // if program exists
            if let Some(program) = program {
                // create new evaluator
                let mut evaluator = Evaluator::new();
                // evaluate program
                if let Some(result) = evaluator.eval(&program) {
                    // assert that result is equal to expected
                    println!("{} - {}", result, expected);
                    assert_eq!(result, expected);
                } else {
                    panic!("No result");
                }
            }
        }
    }

    #[test]
    fn test_hash_literal() {
        let tests = vec![(
            r#"
                {
                    "one": 10 - 9,
                    "three": 6 / 2,
                }
                "#,
            vec![
                (Object::String("one".to_string()), Object::Integer(1)),
                (Object::String("three".to_string()), Object::Integer(3)),
            ],
        )];

        for (input, object) in tests {
            let mut l = Lexer::new(input.to_string());
            let tokens = l.gen_tokens();

            let mut parser = Parser::new(tokens.expect("Could not tokenize"));
            let program: Option<Program> = parser.parse_program();

            let mut evaluator = Evaluator::new();

            if let Some(program) = program {
                if let Some(result) = evaluator.eval(&program) {
                    match result {
                        Object::Hash(hash) => {
                            for (key, value) in hash.iter() {
                                for (expected_key, expected_value) in object.iter() {
                                    if key == expected_key {
                                        assert_eq!(value, expected_value);
                                    }
                                }
                            }
                        }
                        _ => panic!("Expected hash, got {}", result),
                    }
                }
            }
        }
    }

    #[test]
    fn test_array_index() {
        let tests = vec![
            ("[1, 2, 3][0]", Object::Integer(1)),
            ("[1, 2, 3][1]", Object::Integer(2)),
            ("[1, 2, 3][2]", Object::Integer(3)),
            ("let i = 0; [1][i];", Object::Integer(1)),
            ("[1, 2, 3][1 + 1];", Object::Integer(3)),
            ("let myArray = [1, 2, 3]; myArray[2];", Object::Integer(3)),
            (
                "let myArray = [1, 2, 3]; myArray[0] + myArray[1] + myArray[2];",
                Object::Integer(6),
            ),
            (
                "let myArray = [1, 2, 3]; let i = myArray[0]; myArray[i]",
                Object::Integer(2),
            ),
            ("[1, 2, 3][3]", Object::Null),
            ("[1, 2, 3][-1]", Object::Integer(3)),
        ];

        for (input, expected) in tests {
            // create new lexer with input
            let mut l = Lexer::new(input.to_string());
            // generate tokens from lexer
            let tokens = l.gen_tokens();

            // create new parser with tokens
            let mut parser = Parser::new(tokens.expect("Could not tokenize"));
            // parse program from parser
            let program: Option<Program> = parser.parse_program();

            // if program exists
            if let Some(program) = program {
                // create new evaluator
                let mut evaluator = Evaluator::new();
                // evaluate program
                if let Some(result) = evaluator.eval(&program) {
                    // assert that result is equal to expected
                    assert_eq!(result, expected);
                }
            }
        }
    }

    #[test]
    fn test_array_literal() {
        let test = vec![
            (
                "[1, 2 * 2, 3 + 3]",
                vec![Object::Integer(1), Object::Integer(4), Object::Integer(6)],
            ),
            (
                "[1, 2, 3]",
                vec![Object::Integer(1), Object::Integer(2), Object::Integer(3)],
            ),
            ("[]", vec![]),
        ];

        for (input, expected) in test {
            let mut l = Lexer::new(input.to_string());
            let tokens = l.gen_tokens();

            let mut parser = Parser::new(tokens.expect("Could not tokenize"));
            let program: Option<Program> = parser.parse_program();

            if let Some(program) = program {
                let mut evaluator = Evaluator::new();

                if let Some(result) = evaluator.eval(&program) {
                    match result {
                        Object::Array(arr) => {
                            for (i, obj) in arr.iter().enumerate() {
                                assert_eq!(*obj, expected[i]);
                            }
                        }
                        _ => panic!("Expected array, got {}", result),
                    }
                }
            }
        }
    }

    #[test]
    fn test_builtin_len() {
        let tests = vec![
            ("len(\"\")", Object::Integer(0)),
            ("len(\"four\")", Object::Integer(4)),
            ("len(\"hello world\")", Object::Integer(11)),
            (
                "len(1)",
                Object::Error("Argument to `len` not supported, got Integer".to_string()),
            ),
            (
                "len(\"one\", \"two\")",
                Object::Error("Wrong number of arguments. Got 2, expected 1".to_string()),
            ),
        ];

        for (input, expected) in tests {
            // create new lexer with input
            let mut l = Lexer::new(input.to_string());
            // generate tokens from lexer
            let tokens = l.gen_tokens();

            // create new parser with tokens
            let mut parser = Parser::new(tokens.expect("Could not tokenize"));
            // parse program from parser
            let program: Option<Program> = parser.parse_program();

            // if program exists
            if let Some(program) = program {
                // create new evaluator
                let mut evaluator = Evaluator::new();
                // evaluate program
                if let Some(result) = evaluator.eval(&program) {
                    // assert that result is equal to expected
                    assert_eq!(result, expected);
                }
            }
        }
    }

    #[test]
    fn test_string_concatenation() {
        let tests = vec![
            (
                "\"Hello\" + \" \" + \"World!\"",
                Object::String("Hello World!".to_string()),
            ),
            (
                "\"Hello\" + \" \" + \"World!\" + \" \" + \"From\" + \" \" + \"Rust!\"",
                Object::String("Hello World! From Rust!".to_string()),
            ),
        ];

        for (input, expected) in tests {
            // create new lexer with input
            let mut l = Lexer::new(input.to_string());
            // generate tokens from lexer
            let tokens = l.gen_tokens();

            // create new parser with tokens
            let mut parser = Parser::new(tokens.expect("Could not tokenize"));
            // parse program from parser
            let program: Option<Program> = parser.parse_program();

            // if program exists
            if let Some(program) = program {
                // create new evaluator
                let mut evaluator = Evaluator::new();
                // evaluate program
                if let Some(result) = evaluator.eval(&program) {
                    // assert that result is equal to expected
                    assert_eq!(result, expected);
                }
            }
        }
    }

    #[test]
    fn test_string_literal() {
        let test = "\"Hello World!\"";

        let mut l = Lexer::new(test.to_string());
        let tokens = l.gen_tokens();

        let mut parser = Parser::new(tokens.expect("Could not tokenize"));
        let program: Option<Program> = parser.parse_program();

        let mut evaluator = Evaluator::new();

        if let Some(program) = program {
            if let Some(result) = evaluator.eval(&program) {
                assert_eq!(result, Object::String("Hello World!".to_string()));
            }
        }
    }

    #[test]
    fn function_call_test() {
        let tests = vec![
            (
                "let identity = function(x) { x; }; identity(5);",
                Object::Integer(5),
            ),
            (
                "let identity = function(x) { return x; }; identity(5);",
                Object::Integer(5),
            ),
            (
                "let double = function(x) { x * 2; }; double(5);",
                Object::Integer(10),
            ),
            (
                "let add = function(x, y) { x + y; }; add(5, 5);",
                Object::Integer(10),
            ),
            (
                "let add = function(x, y) { x + y; }; add(5 + 5, add(5, 5));",
                Object::Integer(20),
            ),
            ("function(x) { x; }(5)", Object::Integer(5)),
        ];

        for (input, expected) in tests {
            // create new lexer with input
            let mut l = Lexer::new(input.to_string());
            // generate tokens from lexer
            let tokens = l.gen_tokens();

            // create new parser with tokens
            let mut parser = Parser::new(tokens.expect("Could not tokenize"));
            // parse program from parser
            let program: Option<Program> = parser.parse_program();

            // if program exists
            if let Some(program) = program {
                // create new evaluator
                let mut evaluator = Evaluator::new();
                // evaluate program
                if let Some(result) = evaluator.eval(&program) {
                    // assert that result is equal to expected
                    assert_eq!(result, expected);
                }
            }
        }
    }

    #[test]
    fn let_env() {
        let tests = vec![
            ("let x = 10; x;", Object::Integer(10)),
            ("let x = 10 * 10; x;", Object::Integer(100)),
            ("let x = 10; let y = 10; x + y;", Object::Integer(20)),
            (
                "let x = 10; let y = 10; let z = x + y; z;",
                Object::Integer(20),
            ),
        ];

        for (input, expected) in tests {
            // create new lexer with input
            let mut l = Lexer::new(input.to_string());
            // generate tokens from lexer
            let tokens = l.gen_tokens();

            // create new parser with tokens
            let mut parser = Parser::new(tokens.expect("Could not tokenize"));
            // parse program from parser
            let program: Option<Program> = parser.parse_program();

            // if program exists
            if let Some(program) = program {
                // create new evaluator
                let mut evaluator = Evaluator::new();
                // evaluate program
                if let Some(result) = evaluator.eval(&program) {
                    // assert that result is equal to expected
                    assert_eq!(result, expected);
                }
            }
        }
    }

    #[test]
    fn test_return_statements() {
        let tests = vec![
            ("return 10;", Object::Integer(10)),
            ("return 10; 9;", Object::Integer(10)),
            ("return 2 * 5; 9;", Object::Integer(10)),
            ("9; return 2 * 5; 9;", Object::Integer(10)),
            (
                "if (10 > 1) {
                    if (10 > 1) {
                        return 10;
                    }
                    return 1;
                }",
                Object::Integer(10),
            ),
        ];

        for (input, expected) in tests {
            // create new lexer with input
            let mut l = Lexer::new(input.to_string());
            // generate tokens from lexer
            let tokens = l.gen_tokens();

            // create new parser with tokens
            let mut parser = Parser::new(tokens.expect("Could not tokenize"));
            // parse program from parser
            let program: Option<Program> = parser.parse_program();

            // if program exists
            if let Some(program) = program {
                // create new evaluator
                let mut evaluator = Evaluator::new();
                // evaluate program
                if let Some(result) = evaluator.eval(&program) {
                    // assert that result is equal to expected
                    match result {
                        Object::Integer(int) => assert_eq!(Object::Integer(int), expected),
                        Object::Return(obj) => assert_eq!(*obj, expected),
                        _ => panic!("Expected {}, got {}", expected, result),
                    }
                }
            }
        }
    }

    #[test]
    fn test_conditionals() {
        let tests = vec![
            ("if (true) { 10 }", Object::Integer(10)),
            ("if (false) { 10 }", Object::Null),
            ("if (1 < 2) { 10 }", Object::Integer(10)),
            ("if (1 > 2) { 10 }", Object::Null),
            ("if (1 > 2) { 10 } else { 20 }", Object::Integer(20)),
            ("if (1 < 2) { 10 } else { 20 }", Object::Integer(10)),
        ];

        for (input, expected) in tests {
            // create new lexer with input
            let mut l = Lexer::new(input.to_string());
            // generate tokens from lexer
            let tokens = l.gen_tokens();

            // create new parser with tokens
            let mut parser = Parser::new(tokens.expect("Could not tokenize"));
            // parse program from parser
            let program: Option<Program> = parser.parse_program();

            // if program exists
            if let Some(program) = program {
                // create new evaluator
                let mut evaluator = Evaluator::new();
                // evaluate program
                if let Some(result) = evaluator.eval(&program) {
                    // assert that result is equal to expected
                    assert_eq!(result, expected);
                }
            }
        }
    }

    #[test]
    fn test_infix_conditionals() {
        let tests = vec![
            ("1 < 2", true),
            ("1 > 2", false),
            ("1 == 2", false),
            ("1 != 2", true),
            ("true == true", true),
            ("true != true", false),
            ("false == false", true),
            ("false != false", false),
            ("false == true", false),
            ("false != true", true),
            ("true == false", false),
            ("true != false", true),
        ];

        for (input, expected) in tests {
            let mut l = Lexer::new(input.to_string());
            let tokens = l.gen_tokens();

            let mut parser = Parser::new(tokens.expect("Could not tokenize"));
            let program: Option<Program> = parser.parse_program();

            if let Some(program) = program {
                let mut evaluator = Evaluator::new();
                if let Some(result) = evaluator.eval(&program) {
                    assert_eq!(result, Object::Boolean(expected));
                }
            }
        }
    }
    #[test]
    fn test_prefix_bang_minus() {
        let tests = vec![
            ("!true", false),
            ("!false", true),
            ("!!true", true),
            ("!!false", false),
        ];

        for (input, expected) in tests {
            let mut l = Lexer::new(input.to_string());
            let tokens = l.gen_tokens();

            let mut parser = Parser::new(tokens.expect("Could not tokenize"));
            let program: Option<Program> = parser.parse_program();

            if let Some(program) = program {
                let mut evaluator = Evaluator::new();

                if let Some(result) = evaluator.eval(&program) {
                    assert_eq!(result, Object::Boolean(expected));
                }
            }
        }
    }

    #[test]
    fn test_int_minus_prefix() {
        let tests = vec![
            ("-5", -5),
            ("-10", -10),
            ("-15", -15),
            ("-20", -20),
            ("-25", -25),
            ("-30", -30),
            ("-35", -35),
            ("-40", -40),
            ("-45", -45),
            ("-50", -50),
        ];

        for (input, expected) in tests {
            let mut l = Lexer::new(input.to_string());
            let tokens = l.gen_tokens();

            let mut parser = Parser::new(tokens.expect("Could not tokenize"));
            let program: Option<Program> = parser.parse_program();

            if let Some(program) = program {
                let mut evaluator = Evaluator::new();

                if let Some(result) = evaluator.eval(&program) {
                    assert_eq!(result, Object::Integer(expected));
                }
            }
        }
    }

    #[test]
    fn eval_bang_prefix() {
        let input = "!true";
        let mut l = Lexer::new(input.to_string());
        let tokens = l.gen_tokens();

        let mut parser = Parser::new(tokens.expect("Could not tokenize"));
        let program: Option<Program> = parser.parse_program();

        if let Some(program) = program {
            let mut evaluator = Evaluator::new();

            if let Some(result) = evaluator.eval(&program) {
                assert_eq!(result, Object::Boolean(false));
            }
        }
    }

    #[test]
    fn eval_int() {
        let input = "5";
        let mut l = Lexer::new(input.to_string());
        let tokens = l.gen_tokens();

        let mut parser = Parser::new(tokens.expect("Could not tokenize"));
        let program: Option<Program> = parser.parse_program();

        if let Some(program) = program {
            let mut evaluator = Evaluator::new();

            if let Some(result) = evaluator.eval(&program) {
                assert_eq!(result, Object::Integer(5));
            }
        }
    }

    #[test]
    fn eval_boolean() {
        let input = "true;";
        let mut l = Lexer::new(input.to_string());
        let tokens = l.gen_tokens();

        let mut parser = Parser::new(tokens.expect("Could not tokenize"));
        let program: Option<Program> = parser.parse_program();

        if let Some(program) = program {
            let mut evaluator = Evaluator::new();

            if let Some(result) = evaluator.eval(&program) {
                assert_eq!(result, Object::Boolean(true));
            }
        }
    }
}
