### The one with the compiler

We can go the extra mile and build a full compiler from the AST. To create a compiler, we need to choose a target architecture, which is sometimes x86_64, arm64, or something else. However, compilers usually target an intermediate language.

These days, people typically target the [LLVM](https://llvm.org/) infrastructure. For a beautiful introduction, see the [Kaleidoscope example](https://llvm.org/docs/tutorial/). This approach allows you not only to compile to various targets but also to apply all sorts of advanced optimizations to your assembly code.

In this case, we'll target WebAssembly. Writing assembly code can be a dull task, full of obscure opcodes and tricky specifications, but there are few things more rewarding for an engineer.

Assembly code is basically a sequence of operations ("op codes") and associated parameters. For instance, in WebAssembly, you can call a function with the op code `0x10` followed by the function index.

WebAssembly is a stack machine. To pushadd a number onto the stack, use the op codeenter `0x44` followed by the number, encoded as per the [WebAssembly spec](https://webassembly.github.io/spec/core/binary/values.html). In Rust, we are fortunate to have `number.to_le_bytes()` for this purpose. To multiply two numbers, pushadd them both onto the stack, and then add the code `0xa2`. All the codes for every operation in WebAssembly are in the [full specification](https://webassembly.github.io/spec/core/binary/index.html).

One issue we will encounter while targeting WebAssembly is the lack of built-in functions for `sin` and other common mathematical operations. We have two options to address this. We could provide our own implementation, but that would result in a significant amount of assembly-generated code. Alternatively, we can import these functions from the environment, in this case, the browser. We will choose the latter option. If you are curious about the specifics, you can examine the source code. For our purposes, it means that the `sin`, `cos`, `tan`, `log`, `exp`, and `pow` functions referred to in our WebAssembly code via the function indices 0, 1, 2, 3, 4, and 5, respectively.

The core of the algorithm is:

```rust
use crate::{
    parser::{Function, Node, ParserError, UnaryOperator},
    tokens::Operator,
};

// integers must be encoded in a variable length encoding much like utf8 for string called 'leb128'
fn encode_leb128(value: u32) -> Vec<u8> {
    todo!()
}

pub fn compile(node: &Node) -> Result<Vec<u8>, ParserError> {
    let mut result = todo!();
    let body= &mut compile_node(node)?;
    let l = body.len() as u32;
    result.append(&mut encode_leb128(l+4));
    // function count
    result.push(0x01);
    result.append(&mut encode_leb128(l+2));
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
            result.append(&mut f.to_le_bytes().to_vec());
            Ok(result)
        },
        Node::Variable(s) => {
            if s == "PI" {
                let f = std::f64::consts::PI;
                let mut result = vec![0x44];
                result.append(&mut f.to_le_bytes().to_vec());
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
                    result.push(0x05);
                }
            };
            Ok(result)
        }
        Node::UnaryOp { op, right } => match op {
            UnaryOperator::Plus => Ok(compile_node(right)?),
            UnaryOperator::Minus => {
                // we add -1 and multiply
                let mut result = compile_node(right)?;
                result.push(0x44);
                let minus_one: f64 = -1.0;
                result.append(&mut minus_one.to_le_bytes().to_vec());
                result.push(0xa2);
                Ok(result)
            },
        },
    }
}
```

To test the compiler, run `cargo run`. When prompted, enter `Compile(-2.3*Sin(9)*Exp(3.1)/(1+3.2^4.5))` or any other expression you prefer. This process will create a file called `main.wasm`. To run this, you will need a browser environment and must provide all the necessary functions. We've provided all the necessary scaffolding to run `main.wasm` in theYou can find these in the `index.html` file.

Simply run `python -m http.server` in the calculator folder and navigate to `http://localhost:8000` on your browser. You will find the result of the evaluation in the JavaScript console.

That's all from me. Now, go ahead and create things that nobody ever thought were possible.
