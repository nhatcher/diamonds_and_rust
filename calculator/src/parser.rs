use std::{error, fmt::Display};

use crate::{lexer::Lexer, tokens::Token};

#[derive(Debug)]
pub enum Function {
    Cos,
    Sin,
    Tan,
    Log,
    Exp,
    Compile,
}

#[derive(Debug)]
pub enum Operator {
    Plus,
    Minus,
    Times,
    Divide,
    Power,
}
pub enum Comparator {
    Equal,
    LessThan,
    GreaterThan,
    LessThanOrEqual,
    GreaterThanOrEqual,
}

pub struct Options {
    color: String,
    width: u32,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            color: "black".to_string(),
            width: 1,
        }
    }
}

pub struct ProgramNode {
    pub statements: Vec<StatementNode>,
}

pub struct Range {
    value: ExpressionNode,
    minimum: ExpressionNode,
    maximum: ExpressionNode,
}

pub struct PlotFunctionNode {
    value: ExpressionNode,
    options: Options,
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

pub struct CompareNode {
    op: Comparator,
    left: Box<ExpressionNode>,
    right: Box<ExpressionNode>,
}

pub enum ExpressionNode {
    Number(f64),
    Variable(String),
    Function {
        index: Function,
        arg: Box<ExpressionNode>,
    },
    BinaryOp {
        op: Operator,
        left: Box<ExpressionNode>,
        right: Box<ExpressionNode>,
    },
    UnaryOp {
        op: UnaryOperator,
        right: Box<ExpressionNode>,
    },
    FunctionCall {
        index: Function,
        args: Vec<ExpressionNode>,
    },
    IfExpression {
        condition: CompareNode,
        if_true: Box<ExpressionNode>,
        if_false: Box<ExpressionNode>,
    },
}

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug)]
pub struct ParserError {
    pub position: usize,
    pub message: String,
}

impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl error::Error for ParserError {}

#[derive(Debug)]
pub struct LexerError {
    pub position: usize,
    pub message: String,
}

impl Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl error::Error for LexerError {}

#[derive(Debug)]
pub enum UnaryOperator {
    Plus,
    Minus,
}

pub struct Parser {
    lexer: Lexer,
    next_token: Token,
    peek_token: Token,
}

fn binding_power(op: &Operator) -> u8 {
    match op {
        Operator::Plus => 1,
        Operator::Minus => 1,
        Operator::Times => 3,
        Operator::Divide => 3,
        Operator::Power => 5,
    }
}

impl Parser {
    pub fn parse(input_text: &str) -> Result<ProgramNode> {
        let mut lexer = Lexer::new(input_text);
        let next_token = lexer.next_token();
        let peek_token = lexer.next_token();
        let mut parser = Parser {
            lexer,
            next_token,
            peek_token,
        };
        parser.parse_root()
    }

    fn expect_token(&mut self, token: Token) -> Result<()> {
        if self.next_token != token {
            return Err(ParserError {
                position: self.lexer.get_position(),
                message: format!("Expected {} but got {}", self.next_token, token),
            }
            .into());
        }
        self.advance_tokens();
        Ok(())
    }

    fn parse_root(&mut self) -> Result<ProgramNode> {
        let mut statements = Vec::new();
        while self.next_token != Token::EoI {
            statements.push(self.parse_statement()?);
        }
        Ok(ProgramNode { statements })
    }

    fn parse_statement(&mut self) -> Result<StatementNode> {
        let next_token = self.next_token.clone();
        if let Token::Name(name) = next_token {
            self.advance_tokens();
            if name == "Plot" {
                self.expect_token(Token::OpenParenthesis)?;
                return self.parse_plot_statement();
            }
            if self.next_token == Token::OpenParenthesis {
                // function definition
                self.advance_tokens();
                self.parse_function_definition()
            } else if self.next_token == Token::Equal {
                // variable or slider
                self.advance_tokens();
                if self.next_token == Token::OpenBrace {
                    self.parse_slider()
                } else {
                    let expression = self.parse_expression(0)?;
                    return Ok(StatementNode::ConstantAssignment {
                        name,
                        value: Box::new(expression),
                    });
                }
            } else {
                Err(ParserError {
                    position: self.lexer.get_position(),
                    message: format!("Unexpected token '{}'", self.next_token),
                }
                .into())
            }
        } else {
            Err(ParserError {
                position: self.lexer.get_position(),
                message: format!("Unexpected token '{}'", self.next_token),
            }
            .into())
        }
    }

    fn parse_function_definition(&mut self) -> Result<StatementNode> {
        todo!()
    }

    fn parse_variable_definition(&mut self) -> Result<StatementNode> {
        todo!()
    }

    fn parse_slider(&mut self) -> Result<StatementNode> {
        todo!()
    }

    fn parse_plot_statement(&mut self) -> Result<StatementNode> {
        let mut functions = Vec::new();

        // function or a list of functions
        if self.next_token == Token::OpenBracket {
            // list of functions
            self.advance_tokens();
            functions.push(self.parse_plot_function()?);
            while self.next_token == Token::Comma {
                self.advance_tokens();
                functions.push(self.parse_plot_function()?);
            }
            self.expect_token(Token::CloseBracket)?;
        } else {
            // just one function
            functions.push(self.parse_plot_function()?);
        }

        self.expect_token(Token::Comma)?;

        let x_range = self.parse_range()?;

        // y-range
        let y_range = if self.next_token == Token::Comma {
            self.advance_tokens();
            Some(self.parse_range()?)
        } else {
            None
        };

        self.expect_token(Token::CloseParenthesis)?;

        Ok(StatementNode::PlotStatement {
            functions,
            x_range,
            y_range,
        })
    }

    fn parse_range(&mut self) -> Result<Range> {
        todo!()
    }

    fn add_option(&mut self, options: &mut Options) -> Result<()> {
        if let Token::Name(name) = &self.next_token {
            if name == "color" {
                self.advance_tokens();
                if let Token::StringLiteral(color) = self.next_token.clone() {
                    self.advance_tokens();
                    options.color = color;
                    return Ok(());
                }
            } else if name == "width" {
                self.advance_tokens();
                if let Token::Number(width) = self.next_token.clone() {
                    options.width = width as u32;
                    return Ok(());
                }
            }
        }
        Err(ParserError {
            position: self.lexer.get_position(),
            message: format!("Unexpected token '{}'", self.next_token),
        }
        .into())
    }

    fn parse_plot_function(&mut self) -> Result<PlotFunctionNode> {
        if self.next_token == Token::OpenBrace {
            self.advance_tokens();
            let value = self.parse_expression(0)?;
            let mut options = Options::default();
            if self.next_token == Token::Comma {
                self.advance_tokens();
                self.add_option(&mut options)?;
                while self.next_token == Token::Comma {
                    self.advance_tokens();
                    self.add_option(&mut options)?;
                }
            }
            Ok(PlotFunctionNode { value, options })
        } else {
            let value = self.parse_expression(0)?;
            Ok(PlotFunctionNode {
                value,
                options: Options::default(),
            })
        }
    }

    fn parse_expression(&mut self, min_bp: u8) -> Result<ExpressionNode> {
        let mut lhs = self.parse_primary()?;
        loop {
            let op = match &self.next_token {
                Token::EoI => break,
                Token::CloseParenthesis => break,
                Token::Plus => Operator::Plus,
                Token::Minus => Operator::Minus,
                Token::Times => Operator::Times,
                Token::Divide => Operator::Divide,
                Token::Power => Operator::Power,
                unexpected => {
                    return Err(ParserError {
                        position: self.lexer.get_position(),
                        message: format!(
                            "Expected operator: +, -,*, /, ^ or EoI, ) but found '{}'",
                            unexpected
                        ),
                    }
                    .into())
                }
            };

            let l_bp = binding_power(&op);
            if l_bp < min_bp {
                break;
            }

            self.advance_tokens();
            let rhs = self.parse_expression(l_bp)?;

            lhs = ExpressionNode::BinaryOp {
                op,
                left: Box::new(lhs),
                right: Box::new(rhs),
            };
        }
        Ok(lhs)
    }

    fn advance_tokens(&mut self) {
        self.next_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    fn parse_primary(&mut self) -> Result<ExpressionNode> {
        let next_token = self.next_token.clone();
        match next_token {
            Token::Illegal(s) => Err(Box::new(LexerError {
                position: self.lexer.get_position(),
                message: s,
            })),
            Token::EoI => Err(Box::new(ParserError {
                position: self.lexer.get_position(),
                message: "Unexpected end of Input".to_string(),
            })),
            Token::Number(value) => {
                self.advance_tokens();
                Ok(ExpressionNode::Number(value))
            }
            Token::Plus => {
                self.advance_tokens();
                let primary = self.parse_primary()?;
                Ok(ExpressionNode::UnaryOp {
                    op: UnaryOperator::Plus,
                    right: Box::new(primary),
                })
            }
            Token::Minus => {
                self.advance_tokens();
                let primary = self.parse_primary()?;
                Ok(ExpressionNode::UnaryOp {
                    op: UnaryOperator::Minus,
                    right: Box::new(primary),
                })
            }
            Token::Times => Err(ParserError {
                position: self.lexer.get_position(),
                message: "Unexpected token: '*'".to_string(),
            }
            .into()),
            Token::Power => Err(ParserError {
                position: self.lexer.get_position(),
                message: "Unexpected token: '^'".to_string(),
            }
            .into()),
            Token::Divide => Err(ParserError {
                position: self.lexer.get_position(),
                message: "Unexpected token: '/'".to_string(),
            }
            .into()),
            Token::Name(name) => {
                self.advance_tokens();
                if self.next_token != Token::OpenParenthesis {
                    return Ok(ExpressionNode::Variable(name));
                }
                self.advance_tokens();
                let argument = self.parse_expression(0)?;

                if self.next_token != Token::CloseParenthesis {
                    return Err(ParserError {
                        position: self.lexer.get_position(),
                        message: "Expecting: ')'".to_string(),
                    }
                    .into());
                }
                self.advance_tokens();

                let index = match name.as_str() {
                    "Cos" => Function::Cos,
                    "Sin" => Function::Sin,
                    "Tan" => Function::Tan,
                    "Log" => Function::Log,
                    "Exp" => Function::Exp,
                    "Compile" => Function::Compile,
                    _ => {
                        return Err(ParserError {
                            position: self.lexer.get_position(),
                            message: format!("Invalid function name: '{}'", name),
                        }
                        .into());
                    }
                };
                Ok(ExpressionNode::Function {
                    index,
                    arg: Box::new(argument),
                })
            }
            Token::OpenParenthesis => {
                self.advance_tokens();
                let primary = self.parse_expression(0)?;
                if self.next_token != Token::CloseParenthesis {
                    return Err(ParserError {
                        position: self.lexer.get_position(),
                        message: "Expecting: ')'".to_string(),
                    }
                    .into());
                }
                self.advance_tokens();
                Ok(primary)
            }
            Token::CloseParenthesis => Err(ParserError {
                position: self.lexer.get_position(),
                message: "Unexpected token: ')'".to_string(),
            }
            .into()),
            Token::StringLiteral(_) => todo!(),
            Token::OpenBracket => todo!(),
            Token::CloseBracket => todo!(),
            Token::OpenBrace => todo!(),
            Token::CloseBrace => todo!(),
            Token::Comma => todo!(),
            Token::Equal => todo!(),
            Token::NotEqual => todo!(),
            Token::LessThan => todo!(),
            Token::GreaterThan => todo!(),
            Token::LessThanOrEqual => todo!(),
            Token::GreaterThanOrEqual => todo!(),
        }
    }
}
