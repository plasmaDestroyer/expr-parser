#[derive(Debug)]
pub enum Token {
    Number(i64),
    Operator(Op),
}

#[derive(Debug, Clone, Copy)]
pub enum Op {
    Plus,
    Minus,
    Multiply,
    Divide,
}
pub fn lex(input: &str) -> Vec<Token> {

    let expression: Vec<&str> = input.split_whitespace().collect();
    let mut tokens: Vec<Token> = vec![];

    println!("Tokens: {:?}", expression);

    for token in expression {
        if let Ok(num) = token.parse::<i64>() {
            tokens.push(Token::Number(num));
        } else {
            match token {
                "+" => tokens.push(Token::Operator(Op::Plus)),
                "-" => tokens.push(Token::Operator(Op::Minus)),
                "*" => tokens.push(Token::Operator(Op::Multiply)),
                "/" => tokens.push(Token::Operator(Op::Divide)),
                _ => (),
            }
        }
    }

    tokens
}