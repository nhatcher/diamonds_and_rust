// Semantic analysis pass
// Everything that is valid at the parsing level but incorrect should be caught here

// symbol table:
//   globals and lifetimes
//   sliders and lifetimes
//   function and arguments, signatures and function lifetime

use crate::builtins::{Builtin, get_builtin_by_name};
use crate::errors::{Result, SemanticError};

use crate::{
    evaluate::evaluate_in_context,
    parser::{ExpressionNode, ProgramNode, StatementNode},
};


pub struct Global {
    pub name: String,
    pub value: f64,
}
pub struct Slider {
    pub name: String,
    pub minimum: f64,
    pub maximum: f64,
    pub default: f64,
}

pub struct Function {
    pub name: String,
    pub arg_count: u8,
}

pub(crate) struct Context<'a> {
    globals: &'a Vec<Global>,
    functions: &'a Vec<Function>,
    locals: &'a Vec<String>,
}

pub(crate) struct SymbolTable {
    pub globals: Vec<Global>,
    pub sliders: Vec<Slider>,
    pub functions: Vec<Function>,
    pub builtins: Vec<Builtin>,
}

fn is_name_new(name: &str, context: &Context) -> bool {
    todo!()
}

// Returns a list of the sen imported functions
fn analyze_expression(expr: &ExpressionNode, context: &Context) -> Result<Vec<Builtin>>{
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
            // NOTE: We could substitute globals here for their value, but we will do that when emitting code instead.
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
            let arg_count = args.len() as u8;

            if name.chars().next().unwrap().is_uppercase() {
                if let Some(builtin) = get_builtin_by_name(name) {
                    if arg_count != builtin.arg_count() {
                        return Err(SemanticError{ message: format!("Expected exactly one argument but got {arg_count}") }.into())
                    }
                    builtins.push(builtin);
                    for arg in args {
                        builtins.append(&mut analyze_expression(arg, context)?);
                    }
                    return Ok(builtins);
                }
                return Err(SemanticError{ message: format!("Unrecognized function: '{name}'") }.into());
            }
            // It's a user defined function
            for function in context.functions.iter() {
                if name == &function.name {
                    if arg_count != function.arg_count {
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
                let f = evaluate_in_context(
                    value,
                    &Context {
                        globals: &globals,
                        functions: &functions,
                        locals: &vec![]
                    },
                )?;
                *value = ExpressionNode::Number(f);
                globals.push(Global { name: name.clone(), value: f });
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
                let default = evaluate_in_context(
                    default_value,
                    &Context {
                        globals: &globals,
                        functions: &functions,
                        locals: &vec![],
                    },
                )?;
                let minimum = evaluate_in_context(
                    minimum_value,
                    &Context {
                        globals: &globals,
                        functions: &functions,
                        locals: &vec![]
                    },
                )?;
                let maximum = evaluate_in_context(
                    maximum_value,
                    &Context {
                        globals: &globals,
                        functions: &functions,
                        locals: &vec![]
                    },
                )?;
                sliders.push(Slider { name: name.clone(), minimum, maximum, default });
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
                functions.push(Function { name: name.clone(), arg_count: arguments.len() as u8 })
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
                    let minimum = evaluate_in_context(&range.minimum, context)?;
                    let maximum = evaluate_in_context(&range.maximum, context)?;  
                    range.minimum = ExpressionNode::Number(minimum);
                    range.maximum = ExpressionNode::Number(maximum);  
                }
            },
            StatementNode::PrintStatement { argument } => todo!(),
        }
    }
    Ok(SymbolTable {
        globals,
        sliders,
        functions,
        builtins,
    })
}
