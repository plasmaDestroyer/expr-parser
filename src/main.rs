mod lexer;
mod parser;
mod eval;

use lexer::lex;
use parser::parse;
use eval::evaluate;
use std::io::{self, Write};

fn main() {

    // let input: &str = "10 + 20 * 2";

    loop {
        println!("Enter an expression to evaluate: (type \'exit\' to quit)");
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input = input.trim();

        match input {
            "exit" => {
                println!("Exiting program.\n");
                break;
            }
            _ => {

                let tokens = lex(&input);
            
                println!("Tokens: {:?}", tokens);
            
                let root = parse(&tokens, &mut 0);
            
                println!("Tree: {:?}", root);
            
                let res: i64 = evaluate(root);
            
                println!("Answer: {:?} \n", res);

            }
        }
    }
}