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

struct Function {}

struct Signature {}

struct ImportedFunction {
    signature: Signature,
    name: String,
}

pub(crate) struct Context<'a> {
    globals: &'a Vec<Global>,
    functions: &'a Vec<Function>,
    locals: &'a Vec<String>,
}

pub(crate) struct ProgramStructure {
    globals: Vec<Global>,
    sliders: Vec<Slider>,
    functions: Vec<Function>,
    imported_functions: Vec<ImportedFunction>,
}

pub(crate) fn analyze_program(program: &mut ProgramNode) -> Result<ProgramStructure> {
    let mut globals = Vec::new();
    let mut sliders = Vec::new();
    let mut functions = Vec::new();
    let mut imported_functions = Vec::new();

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
                analyze_expression(value, &Context{ globals: &globals, functions: &functions, locals: &vec![]})
            }
            StatementNode::PlotStatement {
                functions,
                x_range,
                y_range,
            } => todo!(),
        }
    }
    Ok(ProgramStructure {
        globals,
        sliders,
        functions,
        imported_functions,
    })
}
