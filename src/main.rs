#[derive(Debug)]
enum Token {
    Number(i64),
    Operator(Op)
}

#[derive(Debug)]
enum Op {
    Plus,
    Minus,
    Multiply,
    Divide
}

fn main() {
    let input: &str = "10 + 20 * 2";

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
                _ => ()
            }
        }
    }

    println!("Tokens: {:?}", tokens);

    let mut res = match tokens[0] { Token::Number(num) => num, _ => 0 };

    for i in (1..tokens.len()).step_by(2) {
        match (&tokens[i], &tokens[i + 1]) {
            (Token::Operator(op), Token::Number(num)) => match op {
                Op::Plus => res += num,
                Op::Minus => res -= num,
                Op::Multiply => res *= num,
                Op::Divide => res /= num,
            },
            _ => ()
        }
    }

    println!("Answer: {:?}", res);

}