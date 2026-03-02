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

    let expression = input.chars().peekable();
    let mut tokens: Vec<Token> = vec![];

    println!("Tokens: {:?}", expression);
    let mut num: String = String::new();

    for c in expression {
        if c.is_whitespace() { 
            continue;
        } else if c.is_digit(10) {
            num.push_str(&c.to_string());
        } else {
            tokens.push(Token::Number(num.parse::<i64>().expect("Error while parsing number in lexer!")));
            num = String::new();
            match c {
                '+' => tokens.push(Token::Operator(Op::Plus)),
                '-' => tokens.push(Token::Operator(Op::Minus)),
                '*' => tokens.push(Token::Operator(Op::Multiply)),
                '/' => tokens.push(Token::Operator(Op::Divide)),
                _ => (),
            }
        }
    }

    if !num.is_empty() {
            tokens.push(Token::Number(num.parse::<i64>().expect("Error while parsing number in lexer!")));
    }

    tokens
}