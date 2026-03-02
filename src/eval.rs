use crate::parser::Node;
use crate::lexer::Op;

pub fn evaluate(node: Node) -> i64 {

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