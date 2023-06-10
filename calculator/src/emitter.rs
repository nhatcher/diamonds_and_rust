use crate::{
    analyzer::SymbolTable,
    parser::{Comparator, ExpressionNode, Operator, ProgramNode, StatementNode, UnaryOperator},
};

use crate::opcodes::*;

// fn encode_leb128(value: u32) -> Vec<u8> {
//     fn encode(i: u32, r: &[u8]) -> Vec<u8> {
//         let b = i & 0x7fu32;
//         let ii = i >> 7;
//         if ii == 0 {
//             [r, &[b as u8]].concat()
//         } else {
//             let r = [r, &[(0x80u32 | b) as u8]].concat();
//             encode(ii, &r)
//         }
//     }
//     encode(value, &[]).to_vec()
// }

// integers must be encoded in a variable length encoding much like utf8 for string called 'leb128'
// https://en.wikipedia.org/wiki/LEB128
// https://webassembly.github.io/spec/core/binary/values.html#integers
fn encode_leb128(mut value: u32) -> Vec<u8> {
    let mut result = vec![];
    loop {
        // byte = low-order 7 bits of value
        let mut byte = value as u8 & 0b0111_1111;
        value >>= 7;
        if value != 0 {
            // set high-order bit of byte
            byte |= 0b1000_0000;
        }
        result.push(byte);
        if value == 0 {
            return result;
        }
    }
}

#[inline(always)]
fn encode_f64(f: f64) -> Vec<u8> {
    encode_f64(f)
}

#[inline(always)]
fn encode_str(str: &str) -> Vec<u8> {
    let a = str.as_bytes();
    let length = a.len() as u32;
    let mut result = Vec::new();
    result.append(&mut encode_leb128(length));
    result.append(&mut a.to_vec());
    result
}

fn emit_type_section(
    symbol_table: &SymbolTable,
    signatures: &mut Vec<u8>,
) -> Result<Vec<u8>, String> {
    let functions = &symbol_table.functions;
    // In Keith all functions return a single f64, the only difference is how many f64 they consume.
    // At least one function (main) has signature () => f64
    signatures.push(0);
    for function in functions {
        let arg_count = function.arg_count;
        if !signatures.contains(&arg_count) {
            signatures.push(arg_count);
        }
    }
    for function in &symbol_table.builtins {
        let arg_count = function.arg_count();
        if !signatures.contains(&arg_count) {
            signatures.push(arg_count);
        }
    }
    signatures.sort();
    let mut signature_vec: Vec<u8> = vec![];
    for arg_count in signatures {
        // It's a function (0x60) and has `arg_count` arguments
        let mut signature_vec = vec![FUNCTION_TYPE_MARKER, *arg_count];
        // all the arguments are f64
        signature_vec.append(&mut vec![F64_TYPE; *arg_count as usize]);
        // return type
        signature_vec.append(&mut vec![0x01, F64_TYPE]);
    }
    let mut result = vec![SECTION_TYPE];
    result.append(&mut encode_leb128(signature_vec.len() as u32));
    result.append(&mut signature_vec);
    Ok(result)
}

fn emit_imports_section(
    root: &ProgramNode,
    symbol_table: &SymbolTable,
    signatures: &[u8],
    constants: &mut Vec<String>,
) -> Result<Vec<u8>, String> {
    let math = encode_str("Math");
    let globals = encode_str("globals");
    let mut imports = Vec::new();
    let mut length = 0;
    // first we import the functions
    for builtin in &symbol_table.builtins {
        imports.append(&mut math.clone());
        imports.append(&mut encode_str(builtin.name()));
        // the import descriptor (it's a function)
        imports.push(FUNCTION_DESCRIPTOR);
        let function_type = signatures
            .iter()
            .position(|&s| s == builtin.arg_count())
            .expect("");
        imports.push(function_type as u8);
        length += 1;
    }
    // then we import the mutable globals (sliders)
    for statement in &root.statements {
        if let StatementNode::Slider { name, .. } = statement {
            imports.append(&mut globals.clone());
            imports.append(&mut encode_str(name));
            // the import descriptor (it's a constant)
            imports.push(CONSTANT_DESCRIPTOR);
            // It's an f64
            imports.push(F64_TYPE);
            // The 'constant' is mutable
            imports.push(CONSTANT_MUTABLE);
            constants.push(name.clone());
            length += 1;
        }
    }
    let mut result = vec![SECTION_IMPORTS];
    result.append(&mut encode_leb128(imports.len() as u32));
    result.append(&mut encode_leb128(length));
    result.append(&mut imports);
    Ok(result)
}

fn emit_function_section(root: &ProgramNode, signatures: &[u8]) -> Result<Vec<u8>, String> {
    let mut function_count = 0;
    let mut bytes = Vec::new();
    for statement in &root.statements {
        if let StatementNode::FunctionDeclaration { arguments, .. } = statement {
            function_count += 1;
            let arg_count = arguments.len() as u8;
            let function_type_index = signatures.iter().position(|&s| s == arg_count).expect("");
            bytes.push(function_type_index as u8);
        }
    }
    let mut result = vec![SECTION_FUNCTION];
    let bytes_count = bytes.len() as u32;
    result.append(&mut encode_leb128(bytes_count));
    result.append(&mut encode_leb128(function_count));
    result.append(&mut bytes);
    Ok(result)
}

fn emit_memory_section(_node: &ProgramNode) -> Result<Vec<u8>, String> {
    let mut bytes = vec![0x01, LIMITS_FLAG_NO_MAX, 0x01]; // one memory, one initial page
    let mut result = vec![SECTION_MEMORY];
    let bytes_count = bytes.len() as u32;
    result.append(&mut encode_leb128(bytes_count));
    result.append(&mut bytes);
    Ok(result)
}

fn emit_export_section(_node: &ProgramNode) -> Result<Vec<u8>, String> {
    // The export section consists of a single function draw(width, height) and the memory
    let mut bytes = vec![0x02]; // two exports
    bytes.append(&mut encode_str("memory"));
    bytes.push(MEMORY_EXPORT_KIND);
    bytes.push(0x00);

    bytes.append(&mut encode_str("redraw"));
    bytes.push(FUNCTION_EXPORT_KIND);
    // TODO: redraw function index
    bytes.push(0x00);
    
    let mut result = vec![SECTION_EXPORT];
    let bytes_count = bytes.len() as u32;
    result.append(&mut encode_leb128(bytes_count));
    result.append(&mut bytes);
    Ok(result)
}

fn emit_code_for_expression(
    node: &ExpressionNode,
    symbol_table: &SymbolTable,
    arguments: &[String],
    functions: &[String],
) -> Result<Vec<u8>, String> {
    let mut result = Vec::new();
    match node {
        ExpressionNode::Number(f) => {
            result.push(INSTR_F64_CONST);
            result.append(&mut encode_f64(*f));
        }
        ExpressionNode::Variable(name) => {
            if let Some(index) = arguments.iter().position(|s| s == name) {
                // It's an arguments
                result.push(INSTR_LOCAL_GET);
                result.push(index as u8);
            } else if let Some(index) = symbol_table.globals.iter().position(|s| &s.name == name) {
                // It's a global, we look up it's value
                let f = symbol_table.globals[index].value;
                result.push(INSTR_F64_CONST);
                result.append(&mut encode_f64(f));
            } else if let Some(index) = symbol_table.sliders.iter().position(|s| &s.name == name) {
                // It's a slider
                result.push(INSTR_GLOBAL_GET);
                result.push(index as u8);
            } else {
                return Err(format!("Unrecognized variable name '{name}'"));
            };
        }
        ExpressionNode::BinaryOp { op, left, right } => {
            let mut lhs = emit_code_for_expression(left, symbol_table, arguments, functions)?;
            let mut rhs = emit_code_for_expression(right, symbol_table, arguments, functions)?;
            result.append(&mut lhs);
            result.append(&mut rhs);
            match op {
                Operator::Plus => result.push(INSTR_F64_ADD),
                Operator::Minus => result.push(INSTR_F64_SUB),
                Operator::Times => result.push(INSTR_F64_MUL),
                Operator::Divide => result.push(INSTR_F64_DIV),
                Operator::Power => {
                    result.push(INSTR_FUNCTION_CALL);
                    if let Some(function_index) = functions.iter().position(|s| s == "Pow") {
                        result.push(function_index as u8);
                    } else {
                        return Err("Error".to_string());
                    }
                }
            };
        }
        ExpressionNode::UnaryOp { op, right } => {
            match op {
                UnaryOperator::Plus => result.append(&mut emit_code_for_expression(
                    right,
                    symbol_table,
                    arguments,
                    functions,
                )?),
                UnaryOperator::Minus => {
                    result.append(&mut emit_code_for_expression(
                        right,
                        symbol_table,
                        arguments,
                        functions,
                    )?);
                    result.push(INSTR_F64_CONST);
                    let minus_one: f64 = -1.0;
                    result.append(&mut minus_one.to_le_bytes().to_vec());
                    result.push(INSTR_F64_MUL);
                }
            };
        }
        ExpressionNode::FunctionCall { name, args } => {
            for arg in args {
                result.append(&mut emit_code_for_expression(
                    arg,
                    symbol_table,
                    arguments,
                    functions,
                )?);
            }
            result.push(INSTR_FUNCTION_CALL);
            if let Some(function_index) = functions.iter().position(|s| s == name) {
                result.push(function_index as u8);
            } else {
                return Err("Error".to_string());
            }
        }
        ExpressionNode::IfExpression {
            condition,
            if_true,
            if_false,
        } => {
            result.append(&mut emit_code_for_expression(
                &condition.left,
                symbol_table,
                arguments,
                functions,
            )?);
            result.append(&mut emit_code_for_expression(
                &condition.right,
                symbol_table,
                arguments,
                functions,
            )?);
            let op = match condition.op {
                Comparator::Equal => INSTR_F64_EQ,
                Comparator::NotEqual => INSTR_F64_NE,
                Comparator::LessThan => INSTR_F64_LT,
                Comparator::GreaterThan => INSTR_F64_GT,
                Comparator::LessThanOrEqual => INSTR_F64_LE,
                Comparator::GreaterThanOrEqual => INSTR_F64_GE,
            };
            result.push(op);
            result.push(INSTR_BLOCK_IF);
            // In Keith If always return a double
            result.push(F64_TYPE);
            result.append(&mut emit_code_for_expression(
                if_true,
                symbol_table,
                arguments,
                functions,
            )?);
            result.push(INSTR_BLOCK_ELSE);
            result.append(&mut emit_code_for_expression(
                if_false,
                symbol_table,
                arguments,
                functions,
            )?);
            result.push(EXPRESSION_END);
        }
        ExpressionNode::SumExpression { value, range } => {
            let lower = if let ExpressionNode::Number(x) = *range.lower {
                x
            } else {
                return Err("Expecting number at this point".to_string());
            };
            let upper = if let ExpressionNode::Number(x) = *range.upper {
                x
            } else {
                return Err("Expecting number at this point".to_string());
            };

            // We define two local variables:
            // * 'i' a counter of type i32
            // * 'result' the total sum, an f64
            result.push(0x02);
            result.push(0x01);
            result.push(I32_TYPE);
            result.push(0x01);
            result.push(F64_TYPE);

            // i = 0
            result.push(INSTR_I32_CONST);
            result.push(0x00);
            result.push(INSTR_LOCAL_SET);
            // The local 0x00 is the function argument
            result.push(0x01);

            // result = lower
            result.push(INSTR_F64_CONST);
            result.append(&mut lower.to_le_bytes().to_vec());
            result.push(INSTR_LOCAL_SET);
            result.push(0x02);

            result.push(INSTR_BLOCK_LOOP);
            result.push(INSTR_VOID);

            // i < range.max
            result.push(INSTR_LOCAL_GET);
            result.push(0x01);
            // cast i into f64
            result.push(INSTR_F64_CONVERT_I32_S);
            result.push(INSTR_F64_CONST);
            result.append(&mut upper.to_le_bytes().to_vec());
            result.push(INSTR_F64_LT);

            result.push(INSTR_BLOCK_IF);
            result.push(INSTR_VOID);

            // result += value
            result.append(&mut emit_code_for_expression(
                value,
                symbol_table,
                arguments,
                functions,
            )?);
            result.push(INSTR_LOCAL_GET);
            result.push(0x02);
            result.push(INSTR_F64_ADD);

            // i++
            result.push(INSTR_LOCAL_GET);
            result.push(0x01);
            result.push(INSTR_I32_CONST);
            result.push(0x01);
            result.push(INSTR_F64_ADD);

            // break 1, continue loop
            result.push(INSTR_BR);
            // break depth
            result.push(0x01);

            result.push(EXPRESSION_END);
            result.push(EXPRESSION_END);

            // return result
            result.push(INSTR_LOCAL_GET);
            result.push(0x02);
        }
    };
    Ok(result)
}

fn emit_code_section(
    root: &ProgramNode,
    symbol_table: &SymbolTable,
    functions: &[String],
) -> Result<Vec<u8>, String> {
    // Has the code for all the functions
    let mut bytes = Vec::new();
    let mut function_count = 0;
    for statement in &root.statements {
        if let StatementNode::FunctionDeclaration {
            arguments,
            name,
            value,
        } = statement
        {
            // function size
            let mut function_bytes = Vec::new();
            function_bytes.append(&mut emit_code_for_expression(
                value,
                symbol_table,
                arguments,
                functions,
            )?);
            // end
            function_bytes.push(EXPRESSION_END);
            bytes.append(&mut encode_leb128(function_bytes.len() as u32));
            bytes.append(&mut function_bytes);
            function_count += 1;
        }
    }
    let mut plot_function_count = 0;
    // Now we define the function main(width, height)
    for statement in &root.statements {
        if let StatementNode::PlotStatement { functions: plot_functions, x_range, y_range } = statement {
            let arguments = &vec![x_range.variable_name.to_string()];
            for function in plot_functions {
                let mut function_bytes = Vec::new();
                function_bytes.append(&mut emit_code_for_expression(
                    &function.value,
                    symbol_table,
                    arguments,
                    functions,
                )?);
                // end
                function_bytes.push(EXPRESSION_END);
                bytes.append(&mut encode_leb128(function_bytes.len() as u32));
                bytes.append(&mut function_bytes);
                plot_function_count += 1;
            }
            {
                // last function, the main function
                let mut function_bytes = Vec::new();
                let width = 1.0;
                let x0 = if let ExpressionNode::Number(x0) = *x_range.lower {
                    x0
                } else {
                    return Err("Expected number at this point".to_string());
                };
                let x1 = if let ExpressionNode::Number(x1) = *x_range.upper {
                    x1
                } else {
                    return Err("Expected number at this point".to_string());
                };
                // local variable step

                /*let step = (x1 - x0) / width;
                let n = (width / step).ceil() as i32;
                for i in 0..n {

                }*/

                function_bytes.push(EXPRESSION_END);
                bytes.append(&mut encode_leb128(function_bytes.len() as u32));
                bytes.append(&mut function_bytes);
                plot_function_count += 1;
            }

        }
    }
    let mut result = vec![SECTION_CODE];
    let bytes_count = bytes.len() as u32;
    result.append(&mut encode_leb128(bytes_count));
    result.append(&mut encode_leb128(function_count+plot_function_count));
    result.append(&mut bytes);
    Ok(result)
}

pub(crate) fn emit_code(node: &ProgramNode, symbol_table: &SymbolTable) -> Result<Vec<u8>, String> {
    let mut result = vec![
        0x00, 0x61, 0x73, 0x6d, // module header
        0x01, 0x00, 0x00, 0x00, // module version
    ];
    let mut signatures = Vec::new();
    let mut constants = Vec::new();
    let mut functions = Vec::new();
    for function in &symbol_table.builtins {
        functions.push(function.name().to_string());
    }
    for function in &symbol_table.functions {
        functions.push(function.name.clone());
    }
    result.append(&mut emit_type_section(symbol_table, &mut signatures)?);
    result.append(&mut emit_imports_section(
        node,
        symbol_table,
        &signatures,
        &mut constants,
    )?);
    result.append(&mut emit_function_section(node, &signatures)?);
    result.append(&mut emit_memory_section(node)?);
    result.append(&mut emit_export_section(node)?);
    result.append(&mut emit_code_section(node, symbol_table, &functions)?);

    Ok(result)
}
