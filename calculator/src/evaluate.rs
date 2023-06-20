use crate::errors::{EvaluationError, Result};
use crate::parser::{Operator, UnaryOperator};
use crate::{analyzer::Context, parser::ExpressionNode};

pub(crate) fn evaluate_in_context(expr: &ExpressionNode, context: &Context) -> Result<f64> {
    match expr {
        ExpressionNode::Number(f) => Ok(*f),
        ExpressionNode::Variable(name) => {
            for global in context.globals {
                if &global.name == name {
                    return Ok(global.value);
                }
            }
            Err(EvaluationError{message: format!("Undefined variable: '{}'", name)}.into())
        },
        ExpressionNode::BinaryOp { op, left, right } => {
            let l = evaluate_in_context(left, context)?;
            let r = evaluate_in_context(right, context)?;
            match op {
                Operator::Plus => Ok(l+r),
                Operator::Minus => Ok(l-r),
                Operator::Times => Ok(l*r),
                Operator::Divide => {
                    if r == 0.0 {
                        return Err(EvaluationError{message: "Division by 0".to_string()}.into())
                    }
                    Ok(l/r)
                },
                Operator::Power => {
                    let result = l.powf(r);
                    Ok(result)
                },
            }
        },
        ExpressionNode::UnaryOp { op, right } => {
            let r = evaluate_in_context(right, context)?;
            match op {
                UnaryOperator::Plus => Ok(r),
                UnaryOperator::Minus => Ok(r),
            }
        },
        ExpressionNode::FunctionCall { .. } => {
            Err(EvaluationError{message: "Cannot use functions in this context".to_string()}.into())
        },
        ExpressionNode::IfExpression { .. } => {
            Err(EvaluationError{message: "Cannot use If expressions in this context".to_string()}.into())
        },
        ExpressionNode::SumExpression { .. } => {
            Err(EvaluationError{message: "Cannot use Sum expressions in this context".to_string()}.into())
        },
    }
}
