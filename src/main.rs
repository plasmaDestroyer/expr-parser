#[derive(Debug)]
enum Token {
    Number(i64),
    Operator(Op),
}

#[derive(Debug)]
enum Node {
    Number(i64),
    Operator(Op, Box<Node>, Box<Node>),
}

#[derive(Debug, Clone, Copy)]
enum Op {
    Plus,
    Minus,
    Multiply,
    Divide,
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
                _ => (),
            }
        }
    }

    println!("Tokens: {:?}", tokens);

    let root = parse_low(&tokens, &mut 0);

    println!("Tree: {:?}", root);

    let res: i64 = evaluate(root);

    println!("Answer: {:?}", res);
}

fn parse_low(tokens: &[Token], pos: &mut usize) -> Node {
    
    let mut left: Node = parse_high(tokens, pos);

    loop {
        if *pos >= tokens.len() { break; }

        let Token::Operator(op) = tokens[*pos] else { break; }; 
        
        match op {
            Op::Plus | Op::Minus => {
                *pos += 1;
                let right: Node = parse_high(tokens, pos);
                left = Node::Operator(op, Box::new(left), Box::new(right));
            },
            _ => break
        }
    }

    left
}

fn parse_high(tokens: &[Token], pos: &mut usize) -> Node {
    
    let mut left: Node = parse_number(tokens, pos);

    loop {
        if *pos >= tokens.len() { break; }

        let Token::Operator(op) = tokens[*pos] else { break; }; 
        
        match op {
            Op::Multiply | Op::Divide => {
                *pos += 1;
                let right: Node = parse_number(tokens, pos);
                left = Node::Operator(op, Box::new(left), Box::new(right));
            },
            _ => break
        }
    }

    left
}

fn parse_number(tokens: &[Token], pos: &mut usize) -> Node {
    
    match tokens[*pos] {
        Token::Number(num) => {
            *pos += 1;
            Node::Number(num)
        },
        _ => todo!()
    }

}

fn evaluate(node: Node) -> i64 {

    let res: i64;

    match node {
        Node::Number(num) => res = num,
        Node::Operator(op, left, right) => {
            let left_num = evaluate(*left);
            let right_num = evaluate(*right);
            match op {
                Op::Plus => res = left_num + right_num,
                Op::Minus => res = left_num - right_num,
                Op::Multiply => res = left_num * right_num,
                Op::Divide => res = left_num / right_num
            }
        }
    }

    res
}