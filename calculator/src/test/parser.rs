use crate::parser::{ExpressionNode, Parser, StatementNode, UnaryOperator, CompareNode};

fn compare_condition(_left: &CompareNode, _right: &CompareNode) -> bool {
    panic!("Not implemented")
}

fn compare_expressions(left: &ExpressionNode, right: &ExpressionNode) -> bool {
    match (left, right) {
        (ExpressionNode::Number(x), ExpressionNode::Number(y)) => (x-y).abs() < f64::EPSILON,
        (ExpressionNode::Variable(a), ExpressionNode::Variable(b)) => a == b,
        (
            ExpressionNode::BinaryOp { op, left, right },
            ExpressionNode::BinaryOp { op: op2, left: left2, right:right2 },
        ) => {
            if op != op2 {
                return false;
            };
            if !compare_expressions(left, left2) {
                return false;
            }
            compare_expressions(right, right2)
        },
        (ExpressionNode::UnaryOp { op, right }, ExpressionNode::UnaryOp { op: op2, right: right2 }) => {
            if op != op2 {
                return false;
            };
            compare_expressions(right, right2)
        },
        (
            ExpressionNode::FunctionCall { name, args },
            ExpressionNode::FunctionCall { name: name2, args: args2 },
        ) => {
            if name != name2 {
                return false;
            }
            if args.len() != args2.len() {
                return false;
            }
            for i in 0..args.len() {
                if !compare_expressions(&args[i], &args2[i]) {
                    return false;
                }
            }
            true
        },
        (
            ExpressionNode::IfExpression {
                condition,
                if_true,
                if_false,
            },
            ExpressionNode::IfExpression {
                condition: condition2,
                if_true: if_true2,
                if_false: if_false2,
            },
        ) => {
            if compare_condition(condition, condition2) {
                return false;
            }
            if !compare_expressions(if_true, if_true2) {
                return false;
            }
            compare_expressions(if_false, if_false2)
        },
        (
            ExpressionNode::SumExpression { value: _, range: _ },
            ExpressionNode::SumExpression { value: _value2, range: _range2 },
        ) => panic!("Not implemented"),
        _ => false,
    }
}

fn compare_statements(left: &StatementNode, right: &StatementNode) -> bool {
    match (left, right) {
        (
            StatementNode::ConstantAssignment { name, value },
            StatementNode::ConstantAssignment {
                name: name_right,
                value: value_right,
            },
        ) => {
            if name != name_right {
                return false;
            }
            compare_expressions(value, value_right)
        }
        (
            StatementNode::Slider {
                name,
                default_value,
                minimum_value,
                maximum_value,
            },
            StatementNode::Slider {
                name: name_right,
                default_value: default_value_right,
                minimum_value: minimum_value_right,
                maximum_value: maximum_value_right,
            },
        ) => {
            if name != name_right {
                return false;
            }
            if !compare_expressions(default_value, default_value_right) {
                return false;
            }
            if !compare_expressions(minimum_value, minimum_value_right) {
                return false;
            }
            compare_expressions(maximum_value, maximum_value_right)
        }
        (
            StatementNode::FunctionDeclaration {
                name,
                arguments,
                value,
            },
            StatementNode::FunctionDeclaration {
                name: name_right,
                arguments: arguments_right,
                value: value_right,
            },
        ) => {
            if name != name_right {
                return false;
            }
            if arguments != arguments_right {
                return false;
            }
            compare_expressions(value, value_right)
        }
        (
            StatementNode::PlotStatement {
                functions: _,
                x_range: _,
                y_range: _,
            },
            StatementNode::PlotStatement {
                functions: _functions_right,
                x_range: _x_range_right,
                y_range: _y_range_right,
            },
        ) => panic!("Not implemented"),
        (
            StatementNode::PrintStatement { argument },
            StatementNode::PrintStatement {
                argument: argument_right,
            },
        ) => compare_expressions(argument, argument_right),
        _ => false,
    }
}

#[test]
fn parses_assign() {
    let script = "a = -3";
    let program = Parser::parse(script).unwrap();
    let statements = program.statements;
    assert_eq!(statements.len(), 1);
    let stm = StatementNode::ConstantAssignment {
        value: ExpressionNode::UnaryOp {
            op: UnaryOperator::Minus,
            right: Box::new(ExpressionNode::Number(3.0)),
        },
        name: "a".to_string(),
    };
    assert!(compare_statements(&statements[0], &stm));
}
