use crate::ast::{BlockStatement, Program};
use crate::env::Env;
use crate::object::Object;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Evaluator {
    pub(crate) env: Rc<RefCell<Env>>,
}

impl Evaluator {
    pub fn new() -> Self {
        Self {
            env: Rc::new(RefCell::new(Env::new())),
        }
    }

    pub fn eval(&mut self, program: &Program) -> Option<Object> {
        let mut result: Option<Object> = None;

        for stmt in program {
            match self.eval_statement(stmt) {
                Some(Object::Return(obj)) => return Some(*obj),
                Some(Object::Error(msg)) => println!("{}", msg),
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

    pub(crate) fn new_error(&self, msg: &str) -> Object {
        Object::Error(msg.to_string())
    }

    pub(crate) fn eval_block_statement(&mut self, stmts: BlockStatement) -> Option<Object> {
        let mut result: Option<Object> = None;

        for stmt in stmts {
            match self.eval_statement(&stmt) {
                Some(Object::Return(obj)) => return Some(Object::Return(obj)),
                Some(Object::Error(msg)) => println!("{}", msg),
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
}
