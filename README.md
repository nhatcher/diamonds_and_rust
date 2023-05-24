# On diamonds and Rust: creating a compiler form scratch

## Our project: A glorified function plotter

We are going to build a create a programming language a compiler for it. The compiler will get a text file with our script and produce assembly code. In our case the assembly will be WebAssembly. The compiler will also produce some html and javascript code to run the the wasm file properly. We will call this the driver code.

We will:

```bash
$ keithc my_program.keith
```

If successful this will produce a `my_program.wasm` file and a `index.html` file. If you open this with a wbe browser you will be able to run the program. This will be a web app displaying some sliders and a function plot.

If the compilation fails it should show us the correct error message.

## The Keith programming language

Inventing a new programming language is an adventure. You have to come up with a new syntax. The are many things to consider, what problems are you trying to solve? How much is your programming language likely to grow? how fast do yo need it to be? where are those programs going to run?

We need a simple programming language to help us plot functions that depend on a series of variables. The functions might be defined by parts or be defined through sum of other functions.

When plotting those functions we want to be able to change the color and the thickness of each function.

We would like the language to be as simple as possible and use a very mathematical notation. For example a simple function might be defined as `sq(x) = x*x`, this would define function `sq` that computes the square of a number.

We will have a precise grammar in a following section, but let's write some examples of what we want:

Example 1: plot a simple function
```keith
Plot(Sin(x)*x, {x, -1, 1})
```

Example 2: plots two functions
```keith
f(x) = Sin(x)*x
g(x) = Cos(x)*x
Plot([f(x), g(x)], {x, -1, 1})
```

Example 3: plots two functions with different options
```keith
f(x) = Sin(x)*x
g(x) = Cos(x)*x
Plot([{f(x), color="red"}, {g(x), color="blue"}], {x, -1, 1}, {y, -10, 10})
```

Example 4: a more complicated function defined by parts. There is a global variable `b` and _sliding_ variable `a`.
The value of the sliding variable should be accessible from the driver program and it's value could be updated.
```keith
b = 10
a = {5, 1, b}
f(x) = x*x*a
g(x, y) = f(x)*f(y*x)
h(x) = Sum(g(n/b), {n, 1, b})
Plot([{If(x>0, h(x), -h(x)), color="red", width=2}], {x, -5, 5})
```

A sliding variable will be represented by a slider in the html driver code.

### Extensions to the language

What if we have a canvas:
```keith
f(x) = Sin(x)*x
g(x) = Cos(x)*x
canvas = Plot([f(x), g(x)], {x, -1, 1})
```
Then you can draw on the canvas
```keith
DrawText(canvas, {10, 10, "A function"})
```

We might use physical coordinates of logical coordinates

## Parsing the new language

List of tokens:
```rust
enum Token {
    Number(f64),
    Name(String),
    StringLiteral(String),
    // Punctuators
    OpenParenthesis,
    CloseParenthesis,
    OpenBracket,
    CloseBracket,
    OpenBrace,
    CloseBrace,
    Comma,
    SemiColon,
    // Operators
    Plus,
    Minus,
    Times,
    Divide,
    Power,
    // Compare
    Equal, // This is also assignment of function declaration operator
    LessThan,
    GreaterThan,
    LessThanOrEqual,
    GreaterThanOrEqual,
    Illegal(String),
    EoI,
}
```

A lexer is no more difficult than in our previous example.


### The grammar is the blueprints of the parser

Remember that there are such things as parser generators than, given a grammar conveniently written will give you code for the parser (and sometimes the lexer). Of course to do that you need to be very careful about your grammar as the program will not understand many of the subtle details. Depending on the algorithm that the parser generator uses internally you will need to adapt the grammar to meet its idiosyncrasies. When using a parser generator *grammar is code*.

Since we are writing the parser by hand the grammar does not need to be understandable by a computer.
The grammar will be the _blueprints_ of the parser. We should print it and have in front of us at any moment, because any function we write, any object we define will be guided by the grammar.

Without much ado here is a grammar for the Keith programming language:


```
program        => statement (';' statements)*
statement      => slider | variable_definition | function_definition | plot_statement
variable_def   => Name '=' expression
slider         => Name '=' '{' expression, expression, expression '}'

function_def   => Name '(' fn_arguments  ')' '=' expression
fn_arguments   => Name (',' Name)*

expression     => primary operator expression
primary        => number | Name | '(' expression ')' | function_call

function_call  => Name '(' arguments ')' | sum_function | if_function
arguments      => ε | expression (',' expression)*

sum_function   => 'Sum' '(' expression ',' interval  ')'
if_function    => 'If' '(' conditional ',' expression ',' expression   ')'
conditional    => expression comparator expression
comparator     => '=' | '!=' | '<=' | '>='

interval       => '{' Name ',' expression ',' expression '}'
operator       => '+' | '-' | '*' | '/'
plot_statement => 'Plot' '(' function_list ',' range (',' range)? ')'
range          => '{' Name ',' expression ',' expression '}'
function_list  => fn_element | '[' functions ']'
fn_element     => fn_plot (',' fn_plot)*
fn_plot        => '{' expression (',' option)* '}'
fn_option      => Name '=' fn_option_val
fn_option_val  => Number | StringLiteral

// Tokens
Name           => /[A-Za-z]+/
Number         => /^[+-]?[0-9]+(\.[0-9]+)?([Ee][+-][0-9]+)?$/
StringLiteral  => '"'[\"]*"'
```

Every line is a production rule. Everything to the left of the `=>` is a non terminal and will written with snake_case. Terminals, elements that do not have a production rule are tokens and are normally described with a regular expression. The theory of finite automata that we described in our previous post deals with them.
Elements in quotes like `'('` or `Sum` are also tokens. We use the pipe `|` to indicate alternatives. Parenthesis group elements and the `*` operator means there could be any number of them.

For instance the production rule:

```
arguments => ε | expression (',' expression)*
```

The arguments of a function could be the empty set (no arguments) or an expression followed by any number of `',' expression`. We use a question mark `?` to mean that the group can be present or not.

The first on the agenda is to write a `struct` for each production rule that has just one alternative and an `enum` for those that have more than one. For instance:

```rust
pub struct ProgramNode {
    pub statements: Vec<StatementNode>,
}

pub enum StatementNode {
    ConstantAssignment {
        name: String,
        value: Box<ExpressionNode>,
    },
    Slider {
        name: String,
        default_value: f64,
        minimum_value: f64,
        maximum_value: f64,
    },
    FunctionDeclaration {
        arguments: Vec<String>,
        value: Box<ExpressionNode>,
    },
    PlotStatement {
        functions: Vec<PlotFunctionNode>,
        x_range: Range,
        y_range: Option<Range>,
    },
}
```

You can see the whole list (here)[].

## Semantic analysis

Once the AST has been build, we need to make sure the program makes sense. In this stage we will test for things we can't check at parse time (or are more difficult to test at parse time) like undefined variables or variables defined twice.

In our case the semantic analysis pass will also extract the set of functions we use and their signatures that we will need at a later stage when emitting code.

## Binary WebAssembly, the theoretical minimum

Since our target language is WebAssembly we need to know a little bit about the language.
There are two formats for WebAssembly, a text format and a binary format. We are interested here about the binary format although there is, of course a close one to one relation between them.

The binary encoding of a WebAssembly module is composed of a header and 13 sections.
If a section is not present it is assumed to be empty.

Only a few are important for us:

* The _type section_ where we define all the signatures.
* The _function section_ when we declare the types for each function. 
* The _export section_ where we declare the functions that we export and will be usable from JavaScript.
* The _import section_ were we declare all functions that will be imported from the JSvaScript runtime.
* The _code section_ where we write all the code  for the functions

We need to understand how to write bytes for each section.

### The header

It is composed of 8 bytes. The first four are the the magic bytes `\0asm` and the second four indicate the version that for us wil be 1:

```
0x00, 0x61, 0x73, 0x6d,
0x01, 0x00, 0x00, 0x00,
```

By the way if you like to see the hex code of a string you can use this small utility in you JavaScript console:


```javascript
const jsStringToBytes = (s) => {
    const textEncoder = new TextEncoder();
    const bytes = textEncoder.encode(s);
    const r = [];
    for (var i = 0; i < bytes.length; i++) {
        r.push(`0x${bytes[i].toString(16).padStart(2, '0')}`);
    }
    return r;
}
```

For example:

```javascript
> jsStringToBytes('\0asm');
(4) ['0x00', '0x61', '0x73', '0x6d']
```

### Common things to all sections

A section starts with a byte that identifies the section. The byte is a number (u8) in between 0 and 12.

Next there is an unsigned integer (u32) identifying the size of the section. Traditionally a u32 takes four bytes, but the WebAssembly identification uses a variable length encoding for the integers called [LEB128](https://en.wikipedia.org/wiki/LEB128) or little endian base 128.
The smart thing about this is that small integers (up to 127) are encoded with a single byte.
Variable length is also used in unicode: utf-8.

Note that each section is a list of elements. The next byte(s) are another u32 specifying the number of elements.

In diverse situations we will need to deal with different number types.
The types of those numbers are also encoded as bytes:

```
0x7F => i32
0x7E => i64
0x7D => f32
0x7C => f64
```

The specification does not have specific types for the unsigned values, the precise interpretation of the bit pattern is left for the assembly instructions.

A vector in WebAssembly is encoded with a u32 leb128 encoded number and then each element.

### 0x01: Type section

In this section we encode all the signatures of all the functions appearing in the module. This are both the imported functions and the functions defined in the module. The signature of a function is the types of its arguments and the type of the returned value.

For instance the signature of the `Sin` function is `f64 -> 64` takes a double and returns a double, while the signature of the `Pow` functions is `(f64, f64) -> f64`, takes two doubles and returns a double.

A function type will be encoded by `0x60` a vector or number types for the arguments and a vector of number types for the returned variables.

An example type section would be:

```
// section type. Takes 16 bytes (0x10)
0x01, 0x10,
// There are three function types
0x03,
// (f64) => f64: one argument of type f64 and one return value of type f64
0x60, 0x01, 0x7c, 0x01, 0x7c,
// (f64, f64) => f64, two arguments and returns one value
0x60, 0x02, 0x7c, 0x7c, 0x01, 0x7c,
// () => f64, has no arguments and returns a f64
0x60, 0x00, 0x01, 0x7c,
```

There you go! We have encoded our first section!

### 0x02: Import section

In this second section we declare the names and types of the functions we import from the runtime. The section is again a vector.

Imports are namespaced. So an import would be (name1, name2 `0x00`, type index). Both names should be `utf8` bytes (encoded with the `to_hex` function above, for instance).

For example if we want to import from `lib` (`[0x6c, 0x69, 0x62]`) a function called `sin` (`[0x73, 0x69, 0x6e]`) the full line would be something like:

```
0x6c, 0x69, 0x62, 0x73, 0x69, 0x6e, 0x00, 0x00
```

The first `0x00` is saying that the import object is a function. Could also have been a table (`0x01`) a memory (`0x02`) or a global (`0x03`). The second `0x00` is saying that the types of the function is the first one of the types in the type section.


### 0x03: Function section

In this section we declare the types of each function we define in the code section. It's basically a list of types.

For instance:

```
0x03, 0x02, 0x01, 0x02
```

First byte is the section number, second is the number of bytes in the body of the section, third says we only define one function and the last byte says that the function is of type (`0x02`) the third type declared in the type section.

### 0x06: Global section

This is a list of the global variables. The runtime will have read and write access to these variables.

Each global has a value type (f64 always in our case or `0x7c`) a byte that says whether the variable is mutable or not `0x00` says it's mutable, `0x01` says it's constant.

### 0x07: Export section

Here we declare all functions that we export. For instance let's say we export just one function called `main`:

```
// section number 7, the body has 8 bytes
0x07, 0x08,
// There is only 1 function we export
0x01,
// the name has four bytes: 'main'
0x04, 0x6d, 0x61, 0x69, 0x6e,
// It's a function we are exporting
0x00, 
// The index is 6
0x06,
```

### 0x0A: Code section

This last section is actually the most important, where all of our code lives.
It is composed of a list of functions. Instead of telling you all the rules to get a general function let me lay down a couple of examples.

Let's define the function `twisted(x, y) = x+y*3` of signature `(f64, f64) => f64`. Takes two doubles and returns a double. Remember that the signature has already been defined in the functions section.

WebAssembly is a [stack machine](https://en.wikipedia.org/wiki/Stack_machine), meaning about the only two things we can do is push bytes onto stack and pop bytes from the stack.

Like in most [stack oriented programming languages](https://en.wikipedia.org/wiki/Stack-oriented_programming) operations are introduced in [Reverse Polish Notation](https://en.wikipedia.org/wiki/Reverse_Polish_notation) (RPN). The operation `2+3` will be entered as:

```
PUSH 2.0
PUSH 3.0
ADD
```

In our case all those will be bytes, of course. The operation `PUSH number` is `0x44` followed by the little endian byte encoding of the number.


```
0x0a, 0x12, 0x01
// Size of the function body in bytes. We do not use local variables
0x10, 0x00,
// get the first argument
0x20, 0x00,
// get the second argument
0x20, 0x01
// We are going to introduce an f64
0x44,
// All f64 numbers are encoded using 8 bytes. The number 3.0 in particular is
0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x08, 0x40
// multiply the last 2
// 0xa2
// finally add them
// 0xa0
```

There are opcodes for all types of operations. We have learned two binary opcodes `0x0a2` for multiplying two `f64` and `0xa0` two add two `f64`. but in a given assembly language there are hundreds, maybe thousands of binary instructions. there are different instructions to add/multiply/divide different types. Plus operations to convert one to another, trunc them, find the remaining of a division, tets comparisons...

Let's do a function that given a number `x` computes `add_powers(x) = Sum(x^n, {n, 1, 100})`. We will need to learn two new things, making a loop and local variables

### WebAssembly cheat sheet

Section codes

Type codes

Operation codes

## The one with the compiler

## Putting it altogether. A solid frontend


## Exercises

1. Add comments. Note that comments are ignored by the compiler. You can add single line comments or multiline comments. Maybe C style /* and closed by */ or C++ // or like Smalltalk just using quotes or Perl/Python style using `#` at the beginning, your call!
2. Create a vscode plugin for Keith!

## References

* [WebAssembly binary specification](https://webassembly.github.io/spec/core/binary/index.html)

* https://www.h-schmidt.net/FloatConverter/IEEE754.html
* https://webassembly.github.io/wabt/demo/wat2wasm/
* https://www.ibm.com/docs/en/aix/7.2?topic=types-double-precision-floating-point
