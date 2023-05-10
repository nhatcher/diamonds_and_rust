use crate::parser::{CompareNode, ExpressionNode, ProgramNode, Range, StatementNode};

pub(crate) fn pretty_print(node: &ProgramNode) -> String {
    let mut str = "".to_string();
    node.statements.iter().for_each(|statement| {
        match statement {
            StatementNode::ConstantAssignment { name, value } => {
                str.push_str(&format!("{name} = {};", pretty_print_expression(value)));
            }
            StatementNode::Slider {
                name,
                default_value,
                minimum_value,
                maximum_value,
            } => {
                str.push_str(&format!(
                    "{name} = {{ {}, {}, {} }}",
                    pretty_print_expression(default_value),
                    pretty_print_expression(minimum_value),
                    pretty_print_expression(maximum_value)
                ));
            }
            StatementNode::FunctionDeclaration {
                name,
                arguments,
                value,
            } => {
                let args = arguments.join(",");
                str.push_str(&format!(
                    "{name}({args}) = {}",
                    pretty_print_expression(value)
                ));
            }
            StatementNode::PlotStatement {
                functions,
                x_range,
                y_range,
            } => match y_range {
                Some(y) => str.push_str(&format!(
                    "Plot(.., {}, {})",
                    pretty_print_range(x_range),
                    pretty_print_range(y)
                )),
                None => str.push_str(&format!("Plot(.., {})", pretty_print_range(x_range))),
            },
        }
        str.push('\n');
    });
    str
}

fn pretty_print_expression(node: &ExpressionNode) -> String {
    match node {
        ExpressionNode::Number(f) => format!("{f}"),
        ExpressionNode::Variable(s) => s.to_string(),
        ExpressionNode::BinaryOp { op, left, right } => {
            format!(
                "{}{:?}{}",
                pretty_print_expression(left),
                op,
                pretty_print_expression(right)
            )
        }
        ExpressionNode::UnaryOp { op, right } => {
            format!("{:?}{}", op, pretty_print_expression(right))
        }
        ExpressionNode::FunctionCall { name, args } => {
            let arguments: Vec<String> = args.iter().map(pretty_print_expression).collect();

            format!("{name}({})", arguments.join(", "))
        }
        ExpressionNode::IfExpression {
            condition,
            if_true,
            if_false,
        } => {
            format!(
                "If({}, {}, {})",
                pretty_print_condition(condition),
                pretty_print_expression(if_true),
                pretty_print_expression(if_false)
            )
        }
        ExpressionNode::SumExpression { value, range } => {
            format!(
                "Sum({}, {{ {}, {}, {} }})",
                pretty_print_expression(value),
                range.variable_name,
                pretty_print_expression(&range.lower),
                pretty_print_expression(&range.upper)
            )
        }
    }
}

fn pretty_print_range(node: &Range) -> String {
    format!(
        "{{{}, {}, {}}}",
        pretty_print_expression(&node.value),
        pretty_print_expression(&node.minimum),
        pretty_print_expression(&node.maximum)
    )
}

fn pretty_print_condition(node: &CompareNode) -> String {
    format!(
        "{}{:?}{}",
        pretty_print_expression(&node.left),
        node.op,
        pretty_print_expression(&node.right)
    )
}
