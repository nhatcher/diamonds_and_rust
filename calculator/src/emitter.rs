use crate::{parser::{ProgramNode, StatementNode}, analyzer::SymbolTable};

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

fn encode_str(str: &str) -> Vec<u8> {
    todo!()
}

fn emit_type_section(symbol_table: &SymbolTable, signatures: &mut Vec<u8>) -> Result<Vec<u8>, String> {
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
        let mut signature_vec = vec![0x60, *arg_count];
        // all the arguments are f64 (0x07)
        signature_vec.append(&mut vec![0x07; *arg_count as usize]);
        // return type 
        signature_vec.append(&mut vec![0x01, 0x07]);
    }
    let mut result = vec![0x01];
    result.append(&mut encode_leb128(signature_vec.len() as u32));
    result.append(&mut signature_vec);
    Ok(result)
}

fn emit_imports_section(root: &ProgramNode, symbol_table: &SymbolTable, signatures: &[u8], constants: &mut Vec<String>) -> Result<Vec<u8>, String> {
    let math = encode_str("Math");
    let globals = encode_str("globals");
    let mut imports = Vec::new();
    let mut length = 0;
    // first we import the functions
    for builtin in &symbol_table.builtins {
        imports.append(&mut math.clone());
        imports.append(&mut encode_str(builtin.name()));
        // the import descriptor (it's a function)
        imports.push(0x00);
        let function_type = signatures.iter().position(|&s| s == builtin.arg_count()).expect("");
        imports.push(function_type as u8);
        length += 1;
    }
    // then we import the mutable globals (sliders)
    for statement in &root.statements {
        if let StatementNode::Slider { name, .. } = statement {
            imports.append(&mut globals.clone());
            imports.append(&mut encode_str(name));
            // the import descriptor (it's a constant)
            imports.push(0x03);
            // It's an f64
            imports.push(0x7c);
            // The 'constant' is mutable
            imports.push(0x01);
            constants.push(name.clone());
            length += 1;
        }
    }
    let mut result = vec![0x02];
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
    let mut result = vec![0x03];
    let bytes_count = bytes.len() as u32;
    result.append(&mut encode_leb128(bytes_count));
    result.append(&mut encode_leb128(function_count));
    result.append(&mut bytes);
    Ok(result)
}

// fn emit_global_section(root: &ProgramNode, constants: &mut Vec<String>) -> Result<Vec<u8>, String> {
//     let mut bytes = Vec::new();
//     for statement in &root.statements {
//         match statement {
//             StatementNode::ConstantAssignment { name, value } => {
//                 bytes.append(&mut vec![0x7c, 0x00]);
//                 constants.push(name.clone());
//             }
//             StatementNode::Slider { name, default_value, minimum_value: _, maximum_value: _ } => {
//                 bytes.append(&mut vec![0x7c, 0x01]);
//                 constants.push(name.clone());
//             }
//             _ => {
//                 // noop
//             }
            
//         }
//     }
//     let mut result = vec![0x06];
//     result.append(&mut bytes);
//     Ok(result)
// }

fn emit_export_section(_node: &ProgramNode) -> Result<Vec<u8>, String> {
    let mut result = vec![0x07];
    Ok(result)
}

fn emit_code_section(_node: &ProgramNode) -> Result<Vec<u8>, String> {
    let mut result = vec![0x0a];
    Ok(result)
}

pub(crate) fn emit_code(node: &ProgramNode, symbol_table: &SymbolTable) -> Result<Vec<u8>, String> {
    let mut result = vec![
        0x00, 0x61, 0x73, 0x6d, // module header
        0x01, 0x00, 0x00, 0x00, // module version
    ];
    let mut signatures = Vec::new();
    let mut constants = Vec::new();
    result.append(&mut emit_type_section(symbol_table, &mut signatures)?);
    result.append(&mut emit_imports_section(node, symbol_table, &signatures, &mut constants)?);
    result.append(&mut emit_function_section(node, &signatures)?);
    result.append(&mut emit_export_section(node)?);
    result.append(&mut emit_code_section(node)?);

    Ok(result)
}
