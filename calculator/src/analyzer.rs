// Semantic analysis pass
// Everything that is valid at the parsing level but incorrect should be caught here

// symbol table:
//   globals and lifetimes
//   sliders and lifetimes
//   function and arguments, signatures and function lifetime

use crate::errors::{Result, SemanticError};

use crate::{
    evaluate::evaluate_with_globals,
    parser::{ExpressionNode, ProgramNode, StatementNode},
};

fn is_valid_name(name: &str) -> bool {
    // valid variable names in Keith
    todo!()
}

struct Global {
    name: String,
    value: f64,
}
struct Slider {
    name: String,
    minimum: f64,
    maximum: f64,
}

struct Function {
    name: String,
    arg_count: u32,
}

pub(crate) struct Context<'a> {
    globals: &'a Vec<Global>,
    functions: &'a Vec<Function>,
    locals: &'a Vec<String>,
}

pub(crate) struct SymbolTable {
    globals: Vec<Global>,
    sliders: Vec<Slider>,
    functions: Vec<Function>,
    builtins: Vec<String>,
}

fn is_name_new(name: &str, context: &Context) -> bool {
    todo!()
}

// Returns a list of the sen imported functions
fn analyze_expression(expr: &ExpressionNode, context: &Context) -> Result<Vec<String>>{
    let mut builtins = Vec::new();
    // Check that an expression is valid
    // 1. there are no undefined variables
    // 2. functions are called wit the correct arguments
    match expr {
        ExpressionNode::Number(_) => {},
        ExpressionNode::Variable(name) => {
            // check that variable has not been defined
            if !is_name_new(name, context) {
                return Err(SemanticError{ message: format!("Variable already exist: '{name}'") }.into());
            }
        },
        ExpressionNode::BinaryOp { op: _, left, right } => {
            builtins.append(&mut analyze_expression(left, context)?);
            builtins.append(&mut analyze_expression(right, context)?);
        },
        ExpressionNode::UnaryOp { op: _, right } => builtins.append(&mut analyze_expression(right, context)?),
        ExpressionNode::FunctionCall { name, args } => {
            // We need to check:
            // 1. function exists
            // 2. arguments are correct
            if !is_name_new(name, context) {
                return Err(SemanticError{ message: format!("Variable already exist: '{name}'") }.into());
            }
            // In Keith a function that starts with a capital letter like `Sin` or `Tan` is builtin,
            // functions that start with a lowercase letter are user defined
            let arg_count = args.len();
            if name.chars().next().unwrap().is_uppercase() {
                let fn_f64_to_f64 = vec!["Sin", "Cos", "Tan", "Log"];
                if fn_f64_to_f64.iter().any(|s| s == name) {
                    if arg_count != 1 {
                        return Err(SemanticError{ message: format!("Expected exactly one argument but got {arg_count}") }.into())
                    }
                    builtins.push(name.to_string());
                    builtins.append(&mut analyze_expression(&args[0], context)?);
                    return Ok(builtins);
                }
                let fn_f64xf64_to_f64 = vec!["Atan2", "Pow"];
                if fn_f64xf64_to_f64.iter().any(|s| s == name) {
                    if arg_count != 2 {
                        return Err(SemanticError{ message: format!("Expected exactly one argument but got {arg_count}") }.into())
                    }
                    builtins.push(name.to_string());
                    builtins.append(&mut analyze_expression(&args[0], context)?);
                    builtins.append(&mut analyze_expression(&args[1], context)?);
                    return Ok(builtins);
                }
                return Err(SemanticError{ message: format!("Unrecognized function: '{name}'") }.into());
            }
            // It's a user defined function
            for function in context.functions.iter() {
                if name == &function.name {
                    if arg_count != function.arg_count as usize {
                        return Err(SemanticError{ message: format!("Expected {} arguments but got {}", function.arg_count, arg_count) }.into());            
                    }
                    for arg in args {
                        builtins.append(&mut analyze_expression(arg, context)?);
                    }
                    return Ok(builtins);
                }
            }
            return Err(SemanticError{ message: format!("Undefined function: '{name}'") }.into());

        },
        ExpressionNode::IfExpression { condition, if_true, if_false } => {
            analyze_expression(&condition.left, context)?;
            analyze_expression(&condition.right, context)?;
            analyze_expression(if_true, context)?;
            analyze_expression(if_false, context)?;
        },
        ExpressionNode::SumExpression { value, range } => {
            let name = range.variable_name.clone();
            let mut locals = context.locals.clone();
            locals.push(name);
            let new_context = Context {
                globals: context.globals,
                functions: context.functions,
                locals: &locals,
            };
            analyze_expression(value, &new_context)?;
        },
    };
    Ok(builtins)
}

pub(crate) fn analyze_program(program: &mut ProgramNode) -> Result<SymbolTable> {
    let mut globals = Vec::new();
    let mut sliders = Vec::new();
    let mut functions = Vec::new();
    let mut builtins = Vec::new();

    let mut seen_names: Vec<String> = Vec::new();

    for statement in program.statements.iter_mut() {
        match statement {
            StatementNode::ConstantAssignment { name, value } => {
                // We need to check
                // 1. It has not been used before
                // 2. It can be evaluated using all constants defined before
                if seen_names.contains(name) {
                    return Err(SemanticError {
                        message: format!("Variable has already been defined '{name}'"),
                    }
                    .into());
                }
                *value = ExpressionNode::Number(evaluate_with_globals(
                    value,
                    &Context {
                        globals: &globals,
                        functions: &functions,
                        locals: &vec![]
                    },
                )?);
                seen_names.push(name.clone());
            }
            StatementNode::Slider {
                name,
                default_value,
                minimum_value,
                maximum_value,
            } => {
                if seen_names.contains(name) {
                    return Err(SemanticError {
                        message: format!("Variable has already been defined '{name}'"),
                    }
                    .into());
                }
                *default_value = ExpressionNode::Number(evaluate_with_globals(
                    default_value,
                    &Context {
                        globals: &globals,
                        functions: &functions,
                        locals: &vec![],
                    },
                )?);
                *minimum_value = ExpressionNode::Number(evaluate_with_globals(
                    minimum_value,
                    &Context {
                        globals: &globals,
                        functions: &functions,
                        locals: &vec![]
                    },
                )?);
                *maximum_value = ExpressionNode::Number(evaluate_with_globals(
                    maximum_value,
                    &Context {
                        globals: &globals,
                        functions: &functions,
                        locals: &vec![]
                    },
                )?);
                seen_names.push(name.clone());
            }
            StatementNode::FunctionDeclaration {
                name,
                arguments,
                value,
            } => {
                if seen_names.contains(name) {
                    return Err(SemanticError {
                        message: format!("Variable has already been defined '{name}'"),
                    }
                    .into());
                }
                seen_names.push(name.clone());
                builtins.append(&mut analyze_expression(value, &Context{ globals: &globals, functions: &functions, locals: arguments})?);
            }
            StatementNode::PlotStatement {
                functions: function_list,
                x_range,
                y_range,
            } => {
                let name = x_range.variable_name.clone();
                let context = &Context {
                    globals: &globals,
                    functions: &functions,
                    locals: &vec![name],
                };
                for function in function_list {
                    builtins.append(&mut analyze_expression(&function.value, context)?);
                }
                if let Some(range) = y_range {
                    let context = &Context {
                        globals: &globals,
                        functions: &functions,
                        locals: &vec![]
                    };
                    let minimum = evaluate_with_globals(&range.minimum, context)?;
                    let maximum = evaluate_with_globals(&range.maximum, context)?;  
                    range.minimum = ExpressionNode::Number(minimum);
                    range.maximum = ExpressionNode::Number(maximum);  
                }
            },
        }
    }
    Ok(SymbolTable {
        globals,
        sliders,
        functions,
        builtins,
    })
}
