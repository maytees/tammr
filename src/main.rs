use std::io::{stdout, Write};

use crate::ast::{Program, Statement};

mod ast;
mod lexer;
mod parser;
fn main() {
    println!("Welcome to the Monkey language repl!");
    loop {
        print!(">> ");
        let mut input = String::new();
        stdout().flush().unwrap();

        std::io::stdin().read_line(&mut input).unwrap();
        if input == "exit\n" {
            break;
        }

        let mut l = lexer::Lexer::new(input);

        let mut parser = parser::Parser::new(l.gen_tokens());
        let program: Option<Program> = parser.parse_program();

        if let Some(program) = program {
            for stmt in program {
                println!("{:?}", stmt);
            }
        }
    }
}
