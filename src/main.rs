use std::io::{stdout, Write};

use crate::{ast::Program, eval::Evaluator};

mod ast;
mod builtin;
mod env;
mod eval;
mod lexer;
mod object;
mod parser;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 {
        run_file(&args[1]);
    } else {
        repl();
    }
}

fn run_file(file: &String) {
    let input = std::fs::read_to_string(file).expect("SOmething went wrong when opning the file");

    let mut l = lexer::Lexer::new(input);
    let tokens = l.gen_tokens();

    let mut parser = parser::Parser::new(tokens);
    let program: Option<Program> = parser.parse_program();

    if let Some(program) = program {
        let mut evaluator = Evaluator::new();

        if let Some(result) = evaluator.eval(&program) {
            match result {
                object::Object::Null => (),
                object::Object::Error(msg) => println!("Error: {}", msg),
                _ => (),
            };
        }
    }
}

fn repl() {
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

            if let Some(result) = evaluator.eval(&program) {
                match result {
                    object::Object::Null => println!("null"),
                    object::Object::Error(msg) => println!("Error: {}", msg),
                    _ => (),
                };
            }
        }
    }
}
