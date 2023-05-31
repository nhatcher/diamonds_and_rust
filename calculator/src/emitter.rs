use std::ops::Index;

use crate::{parser::ProgramNode, analyzer::SymbolTable};

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

fn emit_imports_section(symbol_table: &SymbolTable, signatures: &[u8]) -> Result<Vec<u8>, String> {
    let math = encode_str("Math");
    let mut imports = Vec::new();
    for builtin in &symbol_table.builtins {
        imports.append(&mut math.clone());
        imports.append(&mut encode_str(builtin.name()));
        // th import descriptor
        imports.push(0x00);
        let function_type = signatures.iter().position(|&s| s == builtin.arg_count()).expect("");
        imports.push(function_type as u8);
    }
    let length = symbol_table.builtins.len() as u32;
    let mut result = vec![0x02];
    result.append(&mut encode_leb128(length));
    result.append(&mut imports);
    // let mut result = vec![
    //     0x02, 0x55, // import section. Takes 84 bytes (0x55)
    //     0x06, // We import 6 functions
    //     // This is 'Math'. First byte says it's 4 bytes
    //     0x04, 0x4d, 0x61, 0x74, 0x68, // Math
    //     // sin (has three characters, the import descriptor is 0x00 and the type is 0x00)
    //     0x03, 0x73, 0x69, 0x6e, 0x00, 0x00, //
    //     0x04, 0x4d, 0x61, 0x74, 0x68, // Math
    //     0x03, 0x63, 0x6f, 0x73, 0x00, 0x00, // cos
    //     0x04, 0x4d, 0x61, 0x74, 0x68, // Math
    //     0x03, 0x74, 0x61, 0x6e, 0x00, 0x00, // tan
    //     0x04, 0x4d, 0x61, 0x74, 0x68, // Math
    //     0x03, 0x6c, 0x6f, 0x67, 0x00, 0x00, // log
    //     0x04, 0x4d, 0x61, 0x74, 0x68, // Math
    //     0x03, 0x65, 0x78, 0x70, 0x00, 0x00, // exp
    //     0x04, 0x4d, 0x61, 0x74, 0x68, // Math pow
    //     // pow (import descriptor is 0x00 but type is 0x01)
    //     0x03, 0x70, 0x6f, 0x77, 0x00, 0x01,
    // ];
    Ok(result)
}

fn emit_function_section(_node: &ProgramNode) -> Result<Vec<u8>, String> {
    let mut result = vec![0x03];
    Ok(result)
}

fn emit_global_section(_node: &ProgramNode) -> Result<Vec<u8>, String> {
    let mut result = vec![0x06];
    Ok(result)
}

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
    result.append(&mut emit_type_section(symbol_table, &mut signatures)?);
    result.append(&mut emit_imports_section(symbol_table, &signatures)?);
    result.append(&mut emit_function_section(node)?);
    result.append(&mut emit_global_section(node)?);
    result.append(&mut emit_export_section(node)?);
    result.append(&mut emit_code_section(node)?);

    Ok(result)
}
