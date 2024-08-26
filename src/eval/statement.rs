use super::Evaluator;
use crate::ast::{Expression, Identifier, Statement};
use crate::object::Object;

impl Evaluator {
    pub(crate) fn eval_statement(&mut self, stmt: &Statement) -> Option<Object> {
        match stmt {
            Statement::Expression { token: _, value } => self.eval_expression(value),
            Statement::Return { token: _, value } => self.eval_return(value),
            Statement::Let {
                token: _,
                name,
                value,
                value_kind: _,
            } => {
                let value = self.eval_expression(value)?;
                self.env.borrow_mut().set(&name.value, value);
                Some(Object::Empty)
            }
            Statement::ReAssign {
                token: _,
                name,
                value,
            } => self.eval_reassign(name, value),
        }
    }

    fn eval_reassign(&mut self, name: &Identifier, value: &Expression) -> Option<Object> {
        let value = self.eval_expression(value)?;

        if self.env.borrow_mut().get(&name.value).is_some() {
            self.env.borrow_mut().set(&name.value, value);
            return Some(Object::Empty);
        }

        Some(self.new_error(&format!("Identifier not found: {}", name.value)))
    }

    fn eval_return(&mut self, value: &Expression) -> Option<Object> {
        let value = self.eval_expression(value);

        if let Some(value) = value {
            return Some(Object::Return(Box::new(value)));
        }

        None
    }
}
