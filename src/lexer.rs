#[derive(Debug)]
pub enum Token {
    Number(i64),
    Variable(String),
    Operator(Op),
    Assignment,
}

#[derive(Debug, Clone, Copy)]
pub enum Op {
    Plus,
    Minus,
    Multiply,
    Divide,
}

pub fn lex(input: &str) -> Result<Vec<Token>, String> {

    let mut expression = input.chars().peekable();
    let mut tokens: Vec<Token> = vec![];

    println!("Tokens: {:?}", expression);

    while let Some(c) = expression.next() {
        if c.is_whitespace() { 
            continue;
        } else if c.is_digit(10) {

            let mut num: String = String::new();
            num.push(c);

            while expression.peek().is_some_and(|d| d.is_digit(10)) {
                num.push(expression.next().expect("Error while lexing number!"));
            }

            tokens.push(Token::Number(num.parse::<i64>().map_err(|e| format!("Failed to parse number: {:?}", e))?));

        } else if c.is_alphabetic() {

            let mut variable: String = String::new();
            variable.push(c);

            while expression.peek().is_some_and(|d| d.is_alphanumeric()) {
                variable.push(expression.next().expect("Error while lexing variable name!"));
            }

            tokens.push(Token::Variable(variable));

        } else {
            match c {
                '+' => tokens.push(Token::Operator(Op::Plus)),
                '-' => tokens.push(Token::Operator(Op::Minus)),
                '*' => tokens.push(Token::Operator(Op::Multiply)),
                '/' => tokens.push(Token::Operator(Op::Divide)),
                '=' => tokens.push(Token::Assignment),
                _ => return Err(format!("Unknown Character: {c}"))
            }
        }
    }

    Ok(tokens)
}