use crate::lexer::{Token, Op};

#[derive(Debug)]
pub enum Node {
    Number(i64),
    Variable(String),
    Assignment(String, Box<Node>),
    Operator(Op, Box<Node>, Box<Node>),
}

pub fn parse(tokens: &[Token]) -> Node {
    if tokens.len() > 1 {
        match tokens[1] {
            Token::Assignment => {
                match &tokens[0] {
                    Token::Variable(var) => {
                        Node::Assignment(var.clone(), Box::new(parse_low(tokens, &mut 2)))
                    },
                    _ => todo!()
                }
            },
            _ => parse_low(tokens, &mut 0)
        }
    } else {
        parse_low(tokens, &mut 0)
    }

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
    
    let mut left: Node = parse_number_or_variable(tokens, pos);

    loop {
        if *pos >= tokens.len() { break; }

        let Token::Operator(op) = tokens[*pos] else { break; }; 
        
        match op {
            Op::Multiply | Op::Divide => {
                *pos += 1;
                let right: Node = parse_number_or_variable(tokens, pos);
                left = Node::Operator(op, Box::new(left), Box::new(right));
            },
            _ => break
        }
    }

    left
}

fn parse_number_or_variable(tokens: &[Token], pos: &mut usize) -> Node {
    
    match &tokens[*pos] {
        Token::Number(num) => {
            *pos += 1;
            Node::Number(*num)
        },
        Token::Variable(var) => {
            *pos += 1;
            Node::Variable(var.clone())
        },
        _ => todo!()
    }

}