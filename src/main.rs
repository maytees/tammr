use std::io::{stdout, Write};

mod lexer;

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
        for token in l.gen_tokens() {
            println!("{:?}", token);
        }
    }
}
