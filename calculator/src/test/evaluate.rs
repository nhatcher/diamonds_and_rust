use crate::{
    analyzer::Context,
    evaluate::evaluate_in_context,
    parser::{ExpressionNode, Operator},
};

#[test]
fn number() {
    let expr = ExpressionNode::Number(2.0);
    let context = Context {
        globals: &Vec::new(),
        functions: &Vec::new(),
        locals: &Vec::new(),
    };
    let result = evaluate_in_context(&expr, &context).unwrap();
    assert_eq!(result, 2.0);
}


#[test]
fn multiply_numbers() {
    // 2*3
    let expr = ExpressionNode::BinaryOp {
        op: Operator::Times,
        left: Box::new(ExpressionNode::Number(2.0)),
        right: Box::new(ExpressionNode::Number(3.0)),
    };
    let context = Context {
        globals: &Vec::new(),
        functions: &Vec::new(),
        locals: &Vec::new(),
    };
    let result = evaluate_in_context(&expr, &context).unwrap();
    assert_eq!(result, 6.0);
}
