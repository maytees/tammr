use std::cell::RefCell;
use std::rc::Rc;

use super::Evaluator;
use crate::ast::{Expression, Identifier, Program};
use crate::builtin::{self, DotBuiltinKind};
use crate::env::Env;
use crate::lexer::Token;
use crate::object::Object;

impl Evaluator {
    pub(crate) fn eval_expression(&mut self, value: &Expression) -> Option<Object> {
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
            Expression::Identifier(iden) => self.eval_identifier(iden),
            Expression::FunctionCall {
                token: _,
                function,
                arguments,
            } => self.eval_function_call(function, arguments),
            Expression::FunctionLiteral {
                token: _,
                parameters,
                body,
            } => Some(Object::Function {
                parameters: parameters.clone(),
                body: *body.clone(),
                env: Rc::clone(&self.env),
            }),
            Expression::IndexExpression {
                token: _,
                left,
                index,
            } => self.eval_index_expression(left, index),
            Expression::DotNotation {
                token: _,
                left,
                right,
            } => self.eval_dot_notation(left, right),
        }
    }
    fn eval_dot_expr(
        &mut self,
        value: &Expression,
    ) -> Option<(String, Option<Expression>, Option<Vec<Expression>>)> {
        match value {
            Expression::Identifier(iden) => Some((iden.value.clone(), None, None)),
            Expression::FunctionCall {
                token,
                function,
                arguments,
            } => Some((
                token.literal.clone(),
                Some(*function.clone()),
                Some(arguments.clone()),
            )),
            _ => None,
        }
    }

    fn eval_dot_notation(&mut self, left: &Expression, right: &Expression) -> Option<Object> {
        let left = self.eval_expression(left);

        if let Some(left) = left {
            match left {
                Object::Hash(hash) => {
                    for (k, v) in hash {
                        if let Object::String(k) = k {
                            if k == right.to_string() {
                                return Some(v);
                            }
                        }
                    }

                    return Some(Object::Null);
                }
                Object::String(string) => {
                    let right = self.eval_dot_expr(right);

                    if right.is_none() {
                        return Some(self.new_error("Use dot notation on strings"));
                    }

                    let (name, _func, _args) = right.unwrap();

                    // Is property
                    return builtin::dot_str_builtins(&string, DotBuiltinKind::Property(name));
                }
                _ => return Some(self.new_error("Use dot notation properly")),
            }
        }

        None
    }

    fn eval_index_expression(&mut self, left: &Expression, index: &Expression) -> Option<Object> {
        let left = self.eval_expression(left);
        let index = self.eval_expression(index);

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
                    (Object::Hash(hash), Object::String(key)) => {
                        for (k, v) in hash {
                            if let Object::String(k) = k {
                                if k == key {
                                    return Some(v);
                                }
                            }
                        }

                        return Some(Object::Null);
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
        arguments: &[Expression],
    ) -> Option<Object> {
        let function = self.eval_expression(function)?;

        let arguments = self.eval_expressions(arguments)?;

        match function {
            Object::Function {
                parameters,
                body,
                env,
            } => {
                if arguments.len() != parameters.len() {
                    Some(self.new_error(&format!(
                        "Wrong number of arguments. Expected {}, got {}",
                        parameters.len(),
                        arguments.len()
                    )))
                } else {
                    let old_env = Rc::clone(&self.env);
                    let mut new_env = Env::extend(Rc::clone(&env));
                    let zipped = parameters.iter().zip(arguments);
                    for (_, (Identifier { token: _, value }, o)) in zipped.enumerate() {
                        new_env.set(value, o);
                    }

                    self.env = Rc::new(RefCell::new(new_env));
                    let object = self.eval_block_statement(body);
                    self.env = old_env;

                    object
                }
            }
            Object::BuiltinFunction(func) => Some(func(arguments)),
            _ => Some(self.new_error(&format!("Not a function: {}", function))),
        }
    }

    fn eval_expressions(&mut self, expressions: &[Expression]) -> Option<Vec<Object>> {
        Some(
            expressions
                .iter()
                .map(|expr| self.eval_expression(&expr.clone()).unwrap_or(Object::Null))
                .collect::<Vec<_>>(),
        )
    }

    fn eval_identifier(&mut self, iden: &Identifier) -> Option<Object> {
        let value = self.env.borrow_mut().get(&iden.value);

        if let Some(value) = value {
            return Some(value);
        }

        if builtin::builtins().contains_key(&iden.value) {
            return Some(builtin::builtins()[&iden.value].clone());
        }

        Some(self.new_error(&format!(
            "Identifier not found (eval_identifier): {}",
            iden.value
        )))
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
                    self.eval_block_statement(consequence.to_vec())
                } else if let Some(alt) = alternative {
                    self.eval_block_statement(alt.to_vec())
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
            "==" => Some(Object::Boolean(left == right)),
            "!=" => Some(Object::Boolean(left != right)),
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

    fn eval_prefix_expression(&mut self, operator: &str, right: &Expression) -> Option<Object> {
        let right = self.eval_expression(right)?;

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
}
