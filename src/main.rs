use std::io::{stdout, Write};

use crate::{ast::Program, env::Env, eval::Evaluator};

mod ast;
mod builtin;
mod env;
mod eval;
mod lexer;
mod object;
mod parser;

fn main() {
    println!("Welcome to the Monkey language repl!");
    let mut env = Env::new();

    loop {
        print!(">> ");
        let mut input = String::new();
        stdout().flush().unwrap();

        std::io::stdin().read_line(&mut input).unwrap();
        if input == "exit\n" {
            break;
        }

        let mut l = lexer::Lexer::new(input);
        let tokens = l.gen_tokens();

        // Lexer output
        // for token in &tokens {
        //     println!("{:?}", token);
        // }

        let mut parser = parser::Parser::new(tokens);
        let program: Option<Program> = parser.parse_program();

        if let Some(program) = program {
            // Parser output
            // for stmt in &program {
            //     println!("AST {:?}", stmt);
            // }

            let mut evaluator = Evaluator::new();

            if let Some(result) = evaluator.eval(&program, &mut env) {
                println!("{}", result);
            }
        }
    }
}
