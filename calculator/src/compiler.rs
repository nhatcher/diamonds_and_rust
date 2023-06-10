use crate::{
    parser::{Function, Node, ParserError, UnaryOperator},
    tokens::Operator,
};


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

pub fn compile(node: &Node) -> Result<Vec<u8>, ParserError> {
    let mut result = vec![
        0x00, 0x61, 0x73, 0x6d, // module header
        0x01, 0x00, 0x00, 0x00, // module version
        /* section type */
        0x01, 0x10, // section type. Takes 16 bytes (0x10)
        0x03, // There are three function types
        0x60, 0x01, 0x7c, 0x01, 0x7c, // (f64) => f64, for sin, cos, tan, log, exp
        0x60, 0x02, 0x7c, 0x7c, 0x01, 0x7c, // (f64, f64) => f64, for pow
        0x60, 0x00, 0x01, 0x7c, // () => f64, for main
        /* section import */
        0x02, 0x55, // import section. Takes 84 bytes (0x55)
        0x06, // We import 6 functions
        // This is 'imports'. First byte says it's seven bytes
        0x07, 0x69, 0x6d, 0x70, 0x6f, 0x72, 0x74, 0x73, // imports
        // sin (has three characters, the import descriptor is 0x00 and the type is 0x00)
        0x03, 0x73, 0x69, 0x6e, 0x00, 0x00, //
        0x07, 0x69, 0x6d, 0x70, 0x6f, 0x72, 0x74, 0x73, // imports
        0x03, 0x63, 0x6f, 0x73, 0x00, 0x00, // cos
        0x07, 0x69, 0x6d, 0x70, 0x6f, 0x72, 0x74, 0x73, // imports
        0x03, 0x74, 0x61, 0x6e, 0x00, 0x00, // tan
        0x07, 0x69, 0x6d, 0x70, 0x6f, 0x72, 0x74, 0x73, // imports
        0x03, 0x6c, 0x6f, 0x67, 0x00, 0x00, // log
        0x07, 0x69, 0x6d, 0x70, 0x6f, 0x72, 0x74, 0x73, // imports
        0x03, 0x65, 0x78, 0x70, 0x00, 0x00, // exp
        0x07, 0x69, 0x6d, 0x70, 0x6f, 0x72, 0x74, 0x73, // imports pow
        // pow (import descriptor is 0x00 but type is 0x01)
        0x03, 0x70, 0x6f, 0x77, 0x00, 0x01,
        /* function section. There is only one function and it is of type 0x02 () => f64 */
        0x03, 0x02, 0x01, 0x02,
        /*  Export section. */
        // Has 8 bytes, 1 exported symbol ('main') is a function (0x00) and the function index is 0x06
        0x07, 0x08, 0x01, 0x04, 0x6d, 0x61, 0x69, 0x6e, 0x00, 0x06, //
        /* body */
        0x0a, // function body
    ];
    let body = &mut compile_node(node)?;
    let l = body.len() as u32;
    result.append(&mut encode_leb128(l + 4));
    // function count
    result.push(0x01);
    result.append(&mut encode_leb128(l + 2));
    // We have no local variables
    result.push(0x00);
    result.append(body);
    // end
    result.push(0x0b);

    Ok(result)
}

pub fn compile_node(node: &Node) -> Result<Vec<u8>, ParserError> {
    match node {
        Node::Number(f) => {
            let mut result = vec![0x44];
            result.append(&mut encode_f64(f));
            Ok(result)
        }
        Node::Variable(s) => {
            if s == "PI" {
                let f = std::f64::consts::PI;
                let mut result = vec![0x44];
                result.append(&mut encode_f64(f));
                Ok(result)
            } else {
                Err(ParserError {
                    position: 0,
                    message: format!("Unknown constant: {s}"),
                })
            }
        }
        Node::Function { index, arg } => {
            let mut argument = compile_node(arg)?;
            let mut result = vec![];
            result.append(&mut argument);
            result.push(0x10);
            match index {
                Function::Sin => result.push(0x00),
                Function::Cos => result.push(0x01),
                Function::Tan => result.push(0x02),
                Function::Log => result.push(0x03),
                Function::Exp => result.push(0x04),
                Function::Compile => unreachable!(),
            }
            Ok(result)
        }
        Node::BinaryOp { op, left, right } => {
            let mut lhs = compile_node(left)?;
            let mut rhs = compile_node(right)?;
            let mut result = vec![];
            result.append(&mut lhs);
            result.append(&mut rhs);
            match op {
                Operator::Plus => result.push(0xa0),

                Operator::Minus => result.push(0xa1),
                Operator::Times => result.push(0xa2),
                Operator::Divide => result.push(0xa3),
                Operator::Power => {
                    result.push(0x10);
                    // pow is function number 5
                    result.push(0x05);
                }
            };
            Ok(result)
        }
        Node::UnaryOp { op, right } => match op {
            UnaryOperator::Plus => Ok(compile_node(right)?),
            UnaryOperator::Minus => {
                let mut result = compile_node(right)?;
                result.push(0x44);
                let minus_one: f64 = -1.0;
                result.append(&mut minus_one.to_le_bytes().to_vec());
                result.push(0xa2);
                Ok(result)
            }
        },
    }
}
