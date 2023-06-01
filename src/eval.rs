use crate::ast::{Expression, Literal, Program, Statement};
use crate::lexer::Token;
use crate::object::Object;

pub struct Evaluator {}

impl Evaluator {
    pub fn new() -> Self {
        Self {}
    }

    pub fn eval(&mut self, program: &Program) -> Option<Object> {
        let mut result: Option<Object> = None;

        for stmt in program {
            match self.eval_statement(stmt) {
                Some(stmt) => {
                    result = Some(stmt);
                }
                None => todo!(),
            }
        }

        result
    }

    fn eval_statement(&mut self, stmt: &Statement) -> Option<Object> {
        match stmt {
            Statement::Expression { token: _, value } => self.eval_expression(value),
            _ => None,
        }
    }

    fn eval_expression(&mut self, value: &Expression) -> Option<Object> {
        match value {
            Expression::Literal(lit) => self.eval_literal(lit),
            Expression::Prefix {
                token: _,
                operator,
                right,
            } => self.eval_prefix_expression(operator, right),
            Expression::Infix {
                token: _,
                left,
                operator,
                right,
            } => self.eval_infix_expression(left, operator, right),
            Expression::If {
                token,
                condition,
                consequence,
                alternative,
            } => self.eval_if_expression(token, condition, consequence, alternative),
            _ => None,
        }
    }

    fn eval_if_expression(
        &mut self,
        _token: &Token,
        condition: &Expression,
        consequence: &Program,
        alternative: &Option<Box<Program>>,
    ) -> Option<Object> {
        let condition = self.eval_expression(condition)?;

        match condition {
            Object::Boolean(bool) => {
                if bool {
                    self.eval(consequence)
                } else if let Some(alt) = alternative {
                    self.eval(alt)
                } else {
                    Some(Object::Null)
                }
            }
            _ => panic!("Use if conditionals on booleans"),
        }
    }

    fn eval_infix_expression(
        &mut self,
        left: &Expression,
        operator: &str,
        right: &Expression,
    ) -> Option<Object> {
        let left = self.eval_expression(left)?;
        let right = self.eval_expression(right)?;

        match (right, left) {
            (Object::Integer(right_value), Object::Integer(left_value)) => {
                self.eval_integer_infix_expression(&left_value, operator, &right_value)
            }
            (Object::Boolean(right_value), Object::Boolean(left_value)) => {
                self.eval_boolean_infix_expression(&left_value, operator, &right_value)
            }
            _ => panic!("Use infix operators on integers"),
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
            _ => panic!("Invalid operator: {}", operator),
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
            _ => panic!("Invalid operator: {}", operator),
        }
    }

    fn eval_prefix_expression(&mut self, operator: &str, right: &Expression) -> Option<Object> {
        let right = self.eval_expression(right)?;

        match operator {
            "!" => self.eval_bang_prefix(right),
            "-" => self.eval_minus_prefix(right),
            _ => None,
        }
    }

    fn eval_bang_prefix(&mut self, right: Object) -> Option<Object> {
        match right {
            Object::Boolean(bool) => Some(Object::Boolean(!bool)),
            _ => panic!("Use ! prefix operator on booleans!"),
        }
    }

    fn eval_minus_prefix(&mut self, right: Object) -> Option<Object> {
        match right {
            Object::Integer(int) => Some(Object::Integer(-int)),
            _ => panic!("Use - prefix operator on integers or floats"),
        }
    }

    fn eval_literal(&mut self, lit: &Literal) -> Option<Object> {
        match lit {
            Literal::Integer(int) => Some(Object::Integer(*int)),
            Literal::Boolean(bool) => Some(Object::Boolean(*bool)),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::ast::Program;
    use crate::lexer::Lexer;
    use crate::object::Object;
    use crate::parser::Parser;

    use super::Evaluator;

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

            let mut parser = Parser::new(tokens);
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

            let mut parser = Parser::new(tokens);
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

            let mut parser = Parser::new(tokens);
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

        let mut parser = Parser::new(tokens);
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

        let mut parser = Parser::new(tokens);
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

        let mut parser = Parser::new(tokens);
        let program: Option<Program> = parser.parse_program();

        if let Some(program) = program {
            let mut evaluator = Evaluator::new();

            if let Some(result) = evaluator.eval(&program) {
                assert_eq!(result, Object::Boolean(true));
            }
        }
    }
}
