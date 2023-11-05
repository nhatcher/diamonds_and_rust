use crate::{analyzer::analyze_program, parser::Parser, pretty_print::pretty_print};

#[test]
fn pretty_prints_assign() {
    let script = "a = -3;\n";
    let mut program = Parser::parse(script).unwrap();
    assert_eq!(script, pretty_print(&program));
    let _symbol_table = analyze_program(&mut program).unwrap();
    assert_eq!(script, pretty_print(&program));
}

#[test]
fn parses_example() {
    let script = "\
f(x) = Sin(x)*x;
g(x) = Cos(x)*x;
Plot([f(x), g(x)], {x, -1, 1});\n";
    let mut program = Parser::parse(script).unwrap();
    let _symbol_table = analyze_program(&mut program).unwrap();
    let script_formatted = pretty_print(&program);
    assert_eq!(script, script_formatted);
}
