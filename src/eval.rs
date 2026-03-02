use crate::parser::Node;
use crate::lexer::Op;

use std::collections::HashMap;

pub fn evaluate(node: Node, vars: &mut HashMap<String, i64>) -> i64 {

    let res: i64;

    match node {
        Node::Number(num) => res = num,
        Node::Variable(var) => {
            res = *vars.get(&var).expect("Undefined Variable!");
        },
        Node::Assignment(var, expr) => {
            res = evaluate(*expr, vars);
            vars.insert(var.clone(), res);
        }
        Node::Operator(op, left, right) => {
            let left_num = evaluate(*left, vars);
            let right_num = evaluate(*right, vars);
            match op {
                Op::Plus => res = left_num + right_num,
                Op::Minus => res = left_num - right_num,
                Op::Multiply => res = left_num * right_num,
                Op::Divide => res = left_num / right_num
            }
        },

    }

    res
}