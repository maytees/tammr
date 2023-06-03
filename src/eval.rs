use crate::ast::{BlockStatement, Expression, Identifier, Literal, Program, Statement};
use crate::builtin;
use crate::env::Env;
use crate::lexer::Token;
use crate::object::Object;

pub struct Evaluator {}

impl Evaluator {
    pub fn new() -> Self {
        Self {}
    }

    pub fn eval(&mut self, program: &Program, env: &mut Env) -> Option<Object> {
        let mut result: Option<Object> = None;

        for stmt in program {
            match self.eval_statement(stmt, env) {
                Some(Object::Return(obj)) => return Some(Object::Return(obj)),
                Some(Object::Error(msg)) => println!("{}", msg),
                Some(Object::Empty) => continue,
                Some(obj) => result = Some(obj),
                None => {
                    return Some(
                        self.new_error(&format!("Could not evaluate statement: {:?}", stmt)),
                    )
                }
            }
        }

        result
    }

    fn new_error(&self, msg: &str) -> Object {
        Object::Error(msg.to_string())
    }

    fn eval_block_statement(&mut self, stmts: &BlockStatement, env: &mut Env) -> Option<Object> {
        let mut result: Option<Object> = None;

        for stmt in stmts {
            match self.eval_statement(stmt, env) {
                Some(Object::Return(obj)) => return Some(Object::Return(obj)),
                Some(Object::Error(msg)) => return Some(Object::Error(msg)),
                Some(Object::Empty) => continue,
                Some(obj) => result = Some(obj),
                None => {
                    return Some(self.new_error(&format!("Cannot evaluate statement: {:?}", stmt)))
                }
            }
        }

        result
    }

    fn eval_statement(&mut self, stmt: &Statement, env: &mut Env) -> Option<Object> {
        match stmt {
            Statement::Expression { token: _, value } => self.eval_expression(value, env),
            Statement::Return { token: _, value } => self.eval_return(value, env),
            Statement::Let {
                token: _,
                name,
                value,
            } => {
                let value = self.eval_expression(value, env)?;
                env.set(&name.value, value);
                Some(Object::Empty)
            }
        }
    }

    fn eval_return(&mut self, value: &Expression, env: &mut Env) -> Option<Object> {
        let value = self.eval_expression(value, env);

        if let Some(value) = value {
            return Some(Object::Return(Box::new(value)));
        }

        None
    }

    fn eval_expression(&mut self, value: &Expression, env: &mut Env) -> Option<Object> {
        match value {
            Expression::Literal(lit) => self.eval_literal(lit, env),
            Expression::Prefix {
                token: _,
                operator,
                right,
            } => self.eval_prefix_expression(operator, right, env),
            Expression::Infix {
                token: _,
                left,
                operator,
                right,
            } => self.eval_infix_expression(left, operator, right, env),
            Expression::If {
                token,
                condition,
                consequence,
                alternative,
            } => self.eval_if_expression(token, condition, consequence, alternative, env),
            Expression::Identifier(iden) => self.eval_identifier(iden, env),
            Expression::FunctionCall {
                token: _,
                function,
                arguments,
            } => self.eval_function_call(function, arguments, env),
            Expression::FunctionLiteral {
                token: _,
                parameters,
                body,
            } => Some(Object::Function {
                parameters: parameters.clone(),
                body: *body.clone(),
                env: Env::extend(env.clone()),
            }),
            Expression::IndexExpression {
                token: _,
                left,
                index,
            } => self.eval_index_expression(left, index, env),
        }
    }

    fn eval_index_expression(
        &mut self,
        left: &Expression,
        index: &Expression,
        env: &mut Env,
    ) -> Option<Object> {
        let left = self.eval_expression(left, env);
        let index = self.eval_expression(index, env);

        if let Some(left) = left {
            if let Some(index) = index {
                match (left, index) {
                    (Object::Array(arr), Object::Integer(int)) => {
                        if int <= -1 {
                            if let Some(item) =
                                arr.iter().nth_back((int.unsigned_abs() - 1) as usize)
                            {
                                return Some(item.clone());
                            }
                        }

                        if int >= arr.len() as i64 {
                            return Some(Object::Null);
                        }

                        return Some(arr[int as usize].clone());
                    }
                    (Object::String(str), Object::Integer(int)) => {
                        // Is negative, go backwards. i.e -1
                        if int <= -1 {
                            if let Some(char) =
                                str.chars().nth_back((int.unsigned_abs() - 1) as usize)
                            {
                                return Some(Object::String(char.to_string()));
                            }
                        }

                        if int >= str.len() as i64 {
                            return Some(Object::Null);
                        }

                        if let Some(char) = str.chars().nth(int as usize) {
                            return Some(Object::String(char.to_string()));
                        }
                    }
                    _ => return Some(self.new_error("Use index expression on arrays or strings")),
                }
            }
        }

        None
    }

    fn eval_function_call(
        &mut self,
        function: &Expression,
        arguments: &Vec<Expression>,
        env: &mut Env,
    ) -> Option<Object> {
        let function = self.eval_expression(function, env)?;

        let arguments = self.eval_expressions(arguments, env)?;

        match function {
            Object::Function {
                parameters,
                body,
                env,
            } => {
                let mut extended_env = Env::extend(env);

                for (i, param) in parameters.iter().enumerate() {
                    extended_env.set(&param.value, arguments[i].clone());
                }

                let evaluated = self.eval_block_statement(&body, &mut extended_env)?;

                match evaluated {
                    Object::Return(obj) => Some(*obj),
                    _ => Some(evaluated),
                }
            }
            Object::BuiltinFunction(func) => Some(func(arguments)),
            _ => Some(self.new_error("Use function call on functions")),
        }
    }

    fn eval_expressions(
        &mut self,
        expressions: &Vec<Expression>,
        env: &mut Env,
    ) -> Option<Vec<Object>> {
        let mut result = Vec::new();

        for expr in expressions {
            let evaluated = self.eval_expression(expr, env)?;
            result.push(evaluated);
        }

        Some(result)
    }

    fn eval_identifier(&mut self, iden: &Identifier, env: &mut Env) -> Option<Object> {
        let value = env.get(&iden.value);

        if let Some(value) = value {
            return Some(value);
        }

        if builtin::builtins().contains_key(&iden.value) {
            return Some(builtin::builtins()[&iden.value].clone());
        }

        Some(self.new_error(&format!("Identifier not found: {}", iden.value)))
    }

    fn eval_if_expression(
        &mut self,
        _token: &Token,
        condition: &Expression,
        consequence: &Program,
        alternative: &Option<Box<Program>>,
        env: &mut Env,
    ) -> Option<Object> {
        let condition = self.eval_expression(condition, env)?;

        match condition {
            Object::Boolean(bool) => {
                if bool {
                    self.eval_block_statement(consequence, env)
                } else if let Some(alt) = alternative {
                    self.eval_block_statement(alt, env)
                } else {
                    Some(Object::Null)
                }
            }
            _ => Some(self.new_error("Use if conditionals on booleans")),
        }
    }

    fn eval_infix_expression(
        &mut self,
        left: &Expression,
        operator: &str,
        right: &Expression,
        env: &mut Env,
    ) -> Option<Object> {
        let left = self.eval_expression(left, env)?;
        let right = self.eval_expression(right, env)?;

        match (right, left) {
            (Object::Integer(right_value), Object::Integer(left_value)) => {
                self.eval_integer_infix_expression(&left_value, operator, &right_value)
            }
            (Object::Boolean(right_value), Object::Boolean(left_value)) => {
                self.eval_boolean_infix_expression(&left_value, operator, &right_value)
            }
            (Object::String(right_value), Object::String(left_value)) => {
                self.eval_string_infix_expression(&left_value, operator, &right_value)
            }
            _ => Some(self.new_error("Use infix operators on integers")),
        }
    }

    fn eval_string_infix_expression(
        &mut self,
        left: &str,
        operator: &str,
        right: &str,
    ) -> Option<Object> {
        match operator {
            "+" => Some(Object::String(format!("{}{}", left, right))),
            _ => Some(self.new_error(&format!("Invalid operator: {}", operator))),
        }
    }

    fn eval_boolean_infix_expression(
        &mut self,
        left: &bool,
        operator: &str,
        right: &bool,
    ) -> Option<Object> {
        match operator {
            "==" => Some(Object::Boolean(left == right)),
            "!=" => Some(Object::Boolean(left != right)),
            _ => Some(self.new_error(&format!("Invalid operator: {}", operator))),
        }
    }

    fn eval_integer_infix_expression(
        &mut self,
        left: &i64,
        operator: &str,
        right: &i64,
    ) -> Option<Object> {
        match operator {
            "+" => Some(Object::Integer(left + right)),
            "-" => Some(Object::Integer(left - right)),
            "*" => Some(Object::Integer(left * right)),
            "/" => Some(Object::Integer(left / right)),
            "<" => Some(Object::Boolean(left < right)),
            ">" => Some(Object::Boolean(left > right)),
            "==" => Some(Object::Boolean(left == right)),
            "!=" => Some(Object::Boolean(left != right)),
            _ => Some(self.new_error(&format!("Invalid operator: {}", operator))),
        }
    }

    fn eval_prefix_expression(
        &mut self,
        operator: &str,
        right: &Expression,
        env: &mut Env,
    ) -> Option<Object> {
        let right = self.eval_expression(right, env)?;

        match operator {
            "!" => self.eval_bang_prefix(right),
            "-" => self.eval_minus_prefix(right),
            _ => Some(self.new_error("Invalid prefix operator")),
        }
    }

    fn eval_bang_prefix(&mut self, right: Object) -> Option<Object> {
        match right {
            Object::Boolean(bool) => Some(Object::Boolean(!bool)),
            _ => Some(self.new_error("Use ! prefix operator on booleans!")),
        }
    }

    fn eval_minus_prefix(&mut self, right: Object) -> Option<Object> {
        match right {
            Object::Integer(int) => Some(Object::Integer(-int)),
            _ => Some(self.new_error("Use - prefix operator on integers or floats")),
        }
    }

    fn eval_literal(&mut self, lit: &Literal, env: &mut Env) -> Option<Object> {
        match lit {
            Literal::Integer(int) => Some(Object::Integer(*int)),
            Literal::Boolean(bool) => Some(Object::Boolean(*bool)),
            Literal::String(string) => Some(Object::String(string.clone())),
            Literal::Array(array) => {
                let mut result = Vec::new();

                for expr in array {
                    let evaluated = self.eval_expression(expr, env)?;
                    result.push(evaluated);
                }

                Some(Object::Array(result))
            }
            Literal::Hash(pairs) => self.eval_hash_literal(pairs.to_vec(), env),
        }
    }

    fn eval_hash_literal(
        &mut self,
        pairs: Vec<(Expression, Expression)>,
        env: &mut Env,
    ) -> Option<Object> {
        let mut hash: Vec<(Object, Object)> = Vec::new();

        for (k, v) in pairs {
            let key = self.eval_expression(&k, env)?;

            match key {
                Object::String(_) => {}
                _ => return Some(self.new_error("Hash keys must be strings")),
            };

            let value = self.eval_expression(&v, &mut Env::new())?;

            hash.push((key, value));
        }

        Some(Object::Hash(hash))
    }
}

#[cfg(test)]
mod test {
    use crate::ast::Program;
    use crate::env::Env;
    use crate::lexer::Lexer;
    use crate::object::Object;
    use crate::parser::Parser;

    use super::Evaluator;

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

            let mut parser = Parser::new(tokens);
            let program: Option<Program> = parser.parse_program();

            let mut evaluator = Evaluator::new();

            if let Some(program) = program {
                if let Some(result) = evaluator.eval(&program, &mut Env::new()) {
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
            let mut parser = Parser::new(tokens);
            // parse program from parser
            let program: Option<Program> = parser.parse_program();

            // if program exists
            if let Some(program) = program {
                // create new evaluator
                let mut evaluator = Evaluator::new();
                // evaluate program
                if let Some(result) = evaluator.eval(&program, &mut Env::new()) {
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

            let mut parser = Parser::new(tokens);
            let program: Option<Program> = parser.parse_program();

            if let Some(program) = program {
                let mut evaluator = Evaluator::new();

                if let Some(result) = evaluator.eval(&program, &mut Env::new()) {
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
            let mut parser = Parser::new(tokens);
            // parse program from parser
            let program: Option<Program> = parser.parse_program();

            // if program exists
            if let Some(program) = program {
                // create new evaluator
                let mut evaluator = Evaluator::new();
                // evaluate program
                if let Some(result) = evaluator.eval(&program, &mut Env::new()) {
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
            let mut parser = Parser::new(tokens);
            // parse program from parser
            let program: Option<Program> = parser.parse_program();

            // if program exists
            if let Some(program) = program {
                // create new evaluator
                let mut evaluator = Evaluator::new();
                // evaluate program
                if let Some(result) = evaluator.eval(&program, &mut Env::new()) {
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

        let mut parser = Parser::new(tokens);
        let program: Option<Program> = parser.parse_program();

        let mut evaluator = Evaluator::new();

        if let Some(program) = program {
            if let Some(result) = evaluator.eval(&program, &mut Env::new()) {
                assert_eq!(result, Object::String("Hello World!".to_string()));
            }
        }
    }

    #[test]
    fn function_call_test() {
        let tests = vec![
            (
                "let identity = fn(x) { x; }; identity(5);",
                Object::Integer(5),
            ),
            (
                "let identity = fn(x) { return x; }; identity(5);",
                Object::Integer(5),
            ),
            (
                "let double = fn(x) { x * 2; }; double(5);",
                Object::Integer(10),
            ),
            (
                "let add = fn(x, y) { x + y; }; add(5, 5);",
                Object::Integer(10),
            ),
            (
                "let add = fn(x, y) { x + y; }; add(5 + 5, add(5, 5));",
                Object::Integer(20),
            ),
            ("fn(x) { x; }(5)", Object::Integer(5)),
        ];

        for (input, expected) in tests {
            // create new lexer with input
            let mut l = Lexer::new(input.to_string());
            // generate tokens from lexer
            let tokens = l.gen_tokens();

            // create new parser with tokens
            let mut parser = Parser::new(tokens);
            // parse program from parser
            let program: Option<Program> = parser.parse_program();

            // if program exists
            if let Some(program) = program {
                // create new evaluator
                let mut evaluator = Evaluator::new();
                // evaluate program
                if let Some(result) = evaluator.eval(&program, &mut Env::new()) {
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
            let mut parser = Parser::new(tokens);
            // parse program from parser
            let program: Option<Program> = parser.parse_program();

            // if program exists
            if let Some(program) = program {
                // create new evaluator
                let mut evaluator = Evaluator::new();
                // evaluate program
                if let Some(result) = evaluator.eval(&program, &mut Env::new()) {
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
            let mut parser = Parser::new(tokens);
            // parse program from parser
            let program: Option<Program> = parser.parse_program();

            // if program exists
            if let Some(program) = program {
                // create new evaluator
                let mut evaluator = Evaluator::new();
                // evaluate program
                if let Some(result) = evaluator.eval(&program, &mut Env::new()) {
                    // assert that result is equal to expected
                    match result {
                        Object::Return(obj) => {
                            assert_eq!(*obj, expected);
                        }
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
            let mut parser = Parser::new(tokens);
            // parse program from parser
            let program: Option<Program> = parser.parse_program();

            // if program exists
            if let Some(program) = program {
                // create new evaluator
                let mut evaluator = Evaluator::new();
                // evaluate program
                if let Some(result) = evaluator.eval(&program, &mut Env::new()) {
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

            let mut parser = Parser::new(tokens);
            let program: Option<Program> = parser.parse_program();

            if let Some(program) = program {
                let mut evaluator = Evaluator::new();
                if let Some(result) = evaluator.eval(&program, &mut Env::new()) {
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

            let mut parser = Parser::new(tokens);
            let program: Option<Program> = parser.parse_program();

            if let Some(program) = program {
                let mut evaluator = Evaluator::new();

                if let Some(result) = evaluator.eval(&program, &mut Env::new()) {
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

            let mut parser = Parser::new(tokens);
            let program: Option<Program> = parser.parse_program();

            if let Some(program) = program {
                let mut evaluator = Evaluator::new();

                if let Some(result) = evaluator.eval(&program, &mut Env::new()) {
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

        let mut parser = Parser::new(tokens);
        let program: Option<Program> = parser.parse_program();

        if let Some(program) = program {
            let mut evaluator = Evaluator::new();

            if let Some(result) = evaluator.eval(&program, &mut Env::new()) {
                assert_eq!(result, Object::Boolean(false));
            }
        }
    }

    #[test]
    fn eval_int() {
        let input = "5";
        let mut l = Lexer::new(input.to_string());
        let tokens = l.gen_tokens();

        let mut parser = Parser::new(tokens);
        let program: Option<Program> = parser.parse_program();

        if let Some(program) = program {
            let mut evaluator = Evaluator::new();

            if let Some(result) = evaluator.eval(&program, &mut Env::new()) {
                assert_eq!(result, Object::Integer(5));
            }
        }
    }

    #[test]
    fn eval_boolean() {
        let input = "true;";
        let mut l = Lexer::new(input.to_string());
        let tokens = l.gen_tokens();

        let mut parser = Parser::new(tokens);
        let program: Option<Program> = parser.parse_program();

        if let Some(program) = program {
            let mut evaluator = Evaluator::new();

            if let Some(result) = evaluator.eval(&program, &mut Env::new()) {
                assert_eq!(result, Object::Boolean(true));
            }
        }
    }
}
