mod lexer;
mod parser;
mod eval;

use lexer::lex;
use parser::parse;
use eval::evaluate;

use std::io::{self, Write};
use std::collections::HashMap;

fn main() {

    // let input: &str = "10 + 20 * 2";
    let mut vars: HashMap<String, i64> = HashMap::new();

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
                println!("\nExiting program!");
                break;
            }
            _ => {

                let tokens = lex(&input);
            
                println!("Tokens: {:?}", tokens);
            
                let root = parse(&tokens);
            
                println!("Tree: {:?}", root);
            
                let res: i64 = evaluate(root, &mut vars);
            
                println!("Answer: {:?} \n", res);

            }
        }
    }
}