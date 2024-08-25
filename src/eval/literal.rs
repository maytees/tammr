use super::Evaluator;
use crate::ast::{Expression, Literal};
use crate::object::Object;

impl Evaluator {
    pub(crate) fn eval_literal(&mut self, lit: &Literal) -> Option<Object> {
        match lit {
            Literal::Integer(int) => Some(Object::Integer(*int)),
            Literal::Boolean(bool) => Some(Object::Boolean(*bool)),
            Literal::String(string) => Some(Object::String(string.clone())),
            Literal::Array(array) => {
                let mut result = Vec::new();

                for expr in array {
                    let evaluated = self.eval_expression(expr)?;
                    result.push(evaluated);
                }

                Some(Object::Array(result))
            }
            Literal::Hash(pairs) => self.eval_hash_literal(pairs.to_vec()),
        }
    }

    fn eval_hash_literal(&mut self, pairs: Vec<(Expression, Expression)>) -> Option<Object> {
        let mut hash: Vec<(Object, Object)> = Vec::new();

        for (k, v) in pairs {
            let key = self.eval_expression(&k)?;

            match key {
                Object::String(_) => {}
                _ => return Some(self.new_error("Hash keys must be strings")),
            };

            let value = self.eval_expression(&v)?;

            hash.push((key, value));
        }

        Some(Object::Hash(hash))
    }
}
