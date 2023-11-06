use crate::parser::{
    CompareNode, ExpressionNode, PlotFunctionNode, ProgramNode, StatementNode, SumRange, YRange,
};

pub(crate) fn pretty_print(node: &ProgramNode) -> String {
    let mut str = "".to_string();
    node.statements.iter().for_each(|statement| {
        match statement {
            StatementNode::ConstantAssignment { name, value } => {
                str.push_str(&format!("{name} = {}", pretty_print_expression(value)));
            }
            StatementNode::PrintStatement { argument } => {
                str.push_str(&format!("Print({})", pretty_print_expression(argument)));
            }
            StatementNode::Slider {
                name,
                default_value,
                minimum_value,
                maximum_value,
            } => {
                str.push_str(&format!(
                    "{name} = {{ {}, {}, {} }};",
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
            } => {
                let fun_str: Vec<String> = functions.iter().map(pretty_print_function).collect();
                let fun_str = if functions.len() > 1 {
                    format!("[{}]", fun_str.join(", "))
                } else {
                    fun_str.join(", ")
                };
                match y_range {
                    Some(y) => str.push_str(&format!(
                        "Plot({fun_str}, {}, {})",
                        pretty_print_sum_range(x_range),
                        pretty_print_y_range(y)
                    )),
                    None => str.push_str(&format!(
                        "Plot({fun_str}, {})",
                        pretty_print_sum_range(x_range)
                    )),
                }
            }
        }
        str.push('\n');
    });
    str
}

fn pretty_print_function(node: &PlotFunctionNode) -> String {
    let options = &node.options;
    let mut option_list = Vec::new();
    if options.color != "black" {
        option_list.push(format!("color=\"{}\"", options.color));
    }
    if options.width != 1 {
        option_list.push(format!("width={}", options.width));
    }
    if option_list.is_empty() {
        pretty_print_expression(&node.value)
    } else {
        format!(
            "{{ {}, {} }}",
            pretty_print_expression(&node.value),
            option_list.join(", ")
        )
    }
}

fn pretty_print_expression(node: &ExpressionNode) -> String {
    match node {
        ExpressionNode::Number(f) => format!("{f}"),
        ExpressionNode::Variable(s) => s.to_string(),
        ExpressionNode::BinaryOp { op, left, right } => {
            format!(
                "{}{}{}",
                pretty_print_expression(left),
                op,
                pretty_print_expression(right)
            )
        }
        ExpressionNode::UnaryOp { op, right } => {
            format!("{}{}", op, pretty_print_expression(right))
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
                "Sum({}, {})",
                pretty_print_expression(value),
                pretty_print_sum_range(range)
            )
        }
    }
}

fn pretty_print_sum_range(range: &SumRange) -> String {
    format!(
        "{{{}, {}, {}}}",
        range.variable_name,
        pretty_print_expression(&range.lower),
        pretty_print_expression(&range.upper)
    )
}

fn pretty_print_y_range(node: &YRange) -> String {
    format!(
        "{{{}, {}}}",
        pretty_print_expression(&node.minimum),
        pretty_print_expression(&node.maximum)
    )
}

fn pretty_print_condition(node: &CompareNode) -> String {
    format!(
        "{}{}{}",
        pretty_print_expression(&node.left),
        node.op,
        pretty_print_expression(&node.right)
    )
}
