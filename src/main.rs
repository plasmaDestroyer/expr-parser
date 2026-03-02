mod lexer;
mod parser;
mod eval;

use lexer::lex;
use parser::parse;
use eval::evaluate;

fn main() {

    let input: &str = "10 + 20 * 2";

    let tokens = lex(input);

    println!("Tokens: {:?}", tokens);

    let root = parse(&tokens, &mut 0);

    println!("Tree: {:?}", root);

    let res: i64 = evaluate(root);

    println!("Answer: {:?}", res);
}