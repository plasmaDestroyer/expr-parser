use crate::lexer::{Token, Op};

#[derive(Debug)]
pub enum Node {
    Number(i64),
    Operator(Op, Box<Node>, Box<Node>),
}

pub fn parse(tokens: &[Token], pos: &mut usize) -> Node {
    parse_low(tokens, pos)
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