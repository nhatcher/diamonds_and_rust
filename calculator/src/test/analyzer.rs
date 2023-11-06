use crate::{parser::Parser, analyzer::analyze_program};

#[test]
fn one_function() {
    let script = "f(x) = Sin(x)*Cos(x)";
    let mut ast = Parser::parse(script).unwrap();
    let symbol_table = analyze_program(&mut ast).unwrap();
    // defines one function
    assert_eq!(symbol_table.functions.len(), 1);
    // Uses two built-in functions
    assert_eq!(symbol_table.builtins.len(), 2);
}

#[test]
fn one_global() {
    let script = "a = 7.0";
    let mut ast = Parser::parse(script).unwrap();
    let symbol_table = analyze_program(&mut ast).unwrap();
    
    // defines one variable
    let globals = &symbol_table.globals;
    assert_eq!(symbol_table.globals.len(), 1);
    let a = globals.first().unwrap();
    assert_eq!(a.name, "a");
    assert!((a.value-7.0).abs() <= f64::EPSILON);
    
    // Uses no built-in functions
    assert_eq!(symbol_table.builtins.len(), 0);
}

#[test]
fn constant_folding() {
    let script = "a = 3*2";
    let mut ast = Parser::parse(script).unwrap();
    let symbol_table = analyze_program(&mut ast).unwrap();
    
    // defines one variable
    let globals = &symbol_table.globals;
    assert_eq!(symbol_table.globals.len(), 1);
    let a = globals.first().unwrap();
    assert_eq!(a.name, "a");
    assert!((a.value-6.0).abs() <= f64::EPSILON);
    
    // Uses no built-in functions
    assert_eq!(symbol_table.builtins.len(), 0);
}

#[test]
fn constant_folding_slider() {
    let script = "\
a = 8
b = {1, 0, 2*a}
";
    let mut ast = Parser::parse(script).unwrap();
    let symbol_table = analyze_program(&mut ast).unwrap();
    
    // defines one variable
    let globals = &symbol_table.globals;
    assert_eq!(symbol_table.globals.len(), 1);
    let a = globals.first().unwrap();
    assert_eq!(a.name, "a");
    assert!((a.value-8.0).abs() <= f64::EPSILON);

    // One slider
    let sliders = &symbol_table.sliders;
    assert_eq!(symbol_table.sliders.len(), 1);
    let b = sliders.first().unwrap();
    assert_eq!(b.name, "b");
    assert!((b.minimum-0.0).abs() <= f64::EPSILON, "Minimum: {}", b.minimum);
    assert!((b.maximum-2.0*8.0).abs() <= f64::EPSILON);
    assert!((b.default-1.0).abs() <= f64::EPSILON);

    // Uses no built-in functions
    assert_eq!(symbol_table.builtins.len(), 0);
}

#[test]
fn slider_in_function() {
    let script = "\
a = {5, 1, 10}
f(x) = x*x*a
";
    let mut ast = Parser::parse(script).unwrap();
    let symbol_table = analyze_program(&mut ast).unwrap();

    assert_eq!(symbol_table.functions.len(), 1);
}