use crate::parser::ProgramNode;

fn emit_type_section(_node: &ProgramNode) -> Result<Vec<u8>, String> {
    let mut result = vec![
        0x01, 0x10, // section type. Takes 16 bytes (0x10)
        0x03, // There are three function types
        0x60, 0x01, 0x7c, 0x01, 0x7c, // (f64) => f64, for sin, cos, tan, log, exp
        0x60, 0x02, 0x7c, 0x7c, 0x01, 0x7c, // (f64, f64) => f64, for pow
        0x60, 0x00, 0x01, 0x7c, // () => f64, for main
    ];
    Ok(result)
}

fn emit_imports_section(_node: &ProgramNode) -> Result<Vec<u8>, String> {
    let mut result = vec![
        0x02, 0x55, // import section. Takes 84 bytes (0x55)
        0x06, // We import 6 functions
        // This is 'Math'. First byte says it's 4 bytes
        0x04, 0x4d, 0x61, 0x74, 0x68, // Math
        // sin (has three characters, the import descriptor is 0x00 and the type is 0x00)
        0x03, 0x73, 0x69, 0x6e, 0x00, 0x00, //
        0x04, 0x4d, 0x61, 0x74, 0x68, // Math
        0x03, 0x63, 0x6f, 0x73, 0x00, 0x00, // cos
        0x04, 0x4d, 0x61, 0x74, 0x68, // Math
        0x03, 0x74, 0x61, 0x6e, 0x00, 0x00, // tan
        0x04, 0x4d, 0x61, 0x74, 0x68, // Math
        0x03, 0x6c, 0x6f, 0x67, 0x00, 0x00, // log
        0x04, 0x4d, 0x61, 0x74, 0x68, // Math
        0x03, 0x65, 0x78, 0x70, 0x00, 0x00, // exp
        0x04, 0x4d, 0x61, 0x74, 0x68, // Math pow
        // pow (import descriptor is 0x00 but type is 0x01)
        0x03, 0x70, 0x6f, 0x77, 0x00, 0x01,
    ];
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

pub(crate) fn emit_code(node: &ProgramNode) -> Result<Vec<u8>, String> {
    let mut result = vec![
        0x00, 0x61, 0x73, 0x6d, // module header
        0x01, 0x00, 0x00, 0x00, // module version
    ];
    result.append(&mut emit_type_section(node)?);
    result.append(&mut emit_imports_section(node)?);
    result.append(&mut emit_function_section(node)?);
    result.append(&mut emit_global_section(node)?);
    result.append(&mut emit_export_section(node)?);
    result.append(&mut emit_code_section(node)?);

    Ok(result)
}
