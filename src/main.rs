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
            .expect("Failed to read line\n");

        let input = input.trim();

        match input {
            "exit" => {
                println!("Exiting program!\n");
                break;
            }
            _ => {

                match lex(&input) {
                    Ok(tokens) => {

                        // println!("Tokens: {:?}", tokens);
                    
                        match parse(&tokens) {
                            Ok(root) => {
                            
                                // println!("Tree: {:?}", root);
                            
                                match evaluate(root, &mut vars) {
                                    Ok(res) => println!("{:?}\n", res),
                                    Err(err) => println!("{}\n", err)
                                }
                            },
                            Err(err) => println!("{}\n", err)
                        }

                    },
                    Err(err) => println!("{}\n", err)
                }
            }
        }
    }
}