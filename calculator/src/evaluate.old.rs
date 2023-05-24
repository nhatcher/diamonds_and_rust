use std::fs;

use crate::{
    compiler::compile,
    parser::{Function, Node, Parser, ParserError, UnaryOperator},
    tokens::Operator,
};

pub fn evaluate(input: &str) -> Result<f64, ParserError> {
    let node = Parser::parse(input)?;
    if let Node::Function {
        index: Function::Compile,
        arg,
    } = &node
    {
        let data = compile(arg)?;
        fs::write("main.wasm", data).expect("Unable to write file");
        println!("Saved as 'main.wasm'");
        return evaluate_node(arg);
    }
    evaluate_node(&node)
}

pub fn evaluate_node(node: &Node) -> Result<f64, ParserError> {
    match node {
        Node::Number(f) => Ok(*f),
        Node::Variable(s) => {
            if s == "PI" {
                Ok(std::f64::consts::PI)
            } else {
                Err(ParserError {
                    position: 0,
                    message: format!("Unknown constant: {s}"),
                })
            }
        }
        Node::Function { index, arg } => {
            let argument = evaluate_node(arg)?;
            match index {
                Function::Cos => Ok(argument.cos()),
                Function::Sin => Ok(argument.sin()),
                Function::Tan => Ok(argument.tan()),
                Function::Log => Ok(argument.ln()),
                Function::Exp => Ok(argument.exp()),
                Function::Compile => Err(ParserError {
                    position: 0,
                    message: "Can only compile the full function".to_string(),
                }),
            }
        }
        Node::BinaryOp { op, left, right } => {
            let lhs = evaluate_node(left)?;
            let rhs = evaluate_node(right)?;
            match op {
                Operator::Plus => Ok(lhs + rhs),
                Operator::Minus => Ok(lhs - rhs),
                Operator::Times => Ok(lhs * rhs),
                Operator::Divide => {
                    if rhs == 0.0 {
                        return Err(ParserError {
                            position: 0,
                            message: "Division by 0".to_string(),
                        });
                    }
                    Ok(lhs / rhs)
                }
                Operator::Power => {
                    let x = lhs.powf(rhs);
                    if x.is_infinite() || x.is_nan() {
                        return Err(ParserError {
                            position: 0,
                            message: format!("Undefined operation {lhs}^{rhs}"),
                        });
                    }
                    Ok(x)
                }
            }
        }
        Node::UnaryOp { op, right } => match op {
            UnaryOperator::Plus => Ok(evaluate_node(right)?),
            UnaryOperator::Minus => Ok(-evaluate_node(right)?),
        },
    }
}
