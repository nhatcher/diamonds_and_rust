use std::fmt::Display;

use crate::errors::{LexerError, ParserError, Result};
use crate::{lexer::Lexer, tokens::Token};

#[derive(Debug)]
pub enum Operator {
    Plus,
    Minus,
    Times,
    Divide,
    Power,
}

impl Display for Operator {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Operator::Plus => write!(fmt, "+"),
            Operator::Minus => write!(fmt, "-"),
            Operator::Times => write!(fmt, "*"),
            Operator::Divide => write!(fmt, "/"),
            Operator::Power => write!(fmt, "^"),
        }
    }
}

#[derive(Debug)]
pub enum Comparator {
    Equal,
    LessThan,
    GreaterThan,
    LessThanOrEqual,
    GreaterThanOrEqual,
}

impl Display for Comparator {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Comparator::Equal => write!(fmt, "="),
            Comparator::LessThan => write!(fmt, "<"),
            Comparator::GreaterThan => write!(fmt, ">"),
            Comparator::LessThanOrEqual => write!(fmt, "<="),
            Comparator::GreaterThanOrEqual => write!(fmt, ">="),
        }
    }
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
    pub value: ExpressionNode,
    pub minimum: ExpressionNode,
    pub maximum: ExpressionNode,
}

pub struct PlotFunctionNode {
    pub value: ExpressionNode,
    pub options: Options,
}

pub enum StatementNode {
    ConstantAssignment {
        name: String,
        value: ExpressionNode,
    },
    Slider {
        name: String,
        default_value: ExpressionNode,
        minimum_value: ExpressionNode,
        maximum_value: ExpressionNode,
    },
    FunctionDeclaration {
        name: String,
        arguments: Vec<String>,
        value: ExpressionNode,
    },
    PlotStatement {
        functions: Vec<PlotFunctionNode>,
        x_range: Range,
        y_range: Option<Range>,
    },
}

pub struct CompareNode {
    pub op: Comparator,
    pub left: Box<ExpressionNode>,
    pub right: Box<ExpressionNode>,
}

pub struct SumRange {
    pub variable_name: String,
    pub lower: Box<ExpressionNode>,
    pub upper: Box<ExpressionNode>,
}

pub enum ExpressionNode {
    Number(f64),
    Variable(String),
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
        name: String,
        args: Vec<ExpressionNode>,
    },
    IfExpression {
        condition: CompareNode,
        if_true: Box<ExpressionNode>,
        if_false: Box<ExpressionNode>,
    },
    SumExpression {
        value: Box<ExpressionNode>,
        range: SumRange,
    },
}

#[derive(Debug)]
pub enum UnaryOperator {
    Plus,
    Minus,
}

impl Display for UnaryOperator {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            UnaryOperator::Plus => write!(fmt, "+"),
            UnaryOperator::Minus => write!(fmt, "-"),
        }
    }
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
                message: format!("Expected {} but got {}", token, self.next_token),
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
            self.expect_token(Token::SemiColon)?;
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
                let mut arguments = Vec::new();
                if let Token::Name(variable) = &self.next_token {
                    arguments.push(variable.to_string());
                    self.advance_tokens();
                } else {
                    self.expect_token(Token::CloseParenthesis)?;
                    let value = self.parse_expression()?;
                    return Ok(StatementNode::FunctionDeclaration {
                        name,
                        arguments: Vec::new(),
                        value,
                    });
                }
                while self.next_token == Token::Comma {
                    self.advance_tokens();
                    if let Token::Name(variable) = &self.next_token {
                        arguments.push(variable.to_string());
                        self.advance_tokens();
                    }
                }
                self.expect_token(Token::CloseParenthesis)?;
                self.expect_token(Token::Equal)?;
                let value = self.parse_expression()?;

                Ok(StatementNode::FunctionDeclaration {
                    name,
                    arguments,
                    value,
                })
            } else if self.next_token == Token::Equal {
                // variable or slider
                self.advance_tokens();
                if self.next_token == Token::OpenBrace {
                    self.advance_tokens();
                    let default_value = self.parse_expression()?;
                    self.expect_token(Token::Comma)?;
                    let minimum_value = self.parse_expression()?;
                    self.expect_token(Token::Comma)?;
                    let maximum_value = self.parse_expression()?;
                    self.expect_token(Token::CloseBrace)?;

                    return Ok(StatementNode::Slider {
                        name,
                        default_value,
                        minimum_value,
                        maximum_value,
                    });
                } else {
                    let value = self.parse_expression()?;
                    return Ok(StatementNode::ConstantAssignment { name, value });
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

    fn parse_name(&mut self) -> Result<String> {
        if let Token::Name(s) = &self.next_token {
            let name = s.to_string();
            self.advance_tokens();
            Ok(name)
        } else {
            Err(ParserError {
                position: self.lexer.get_position(),
                message: format!("Unexpected token '{}'", self.next_token),
            }
            .into())
        }
    }

    fn parse_string_literal(&mut self) -> Result<String> {
        if let Token::StringLiteral(s) = &self.next_token {
            let name = s.to_string();
            self.advance_tokens();
            Ok(name)
        } else {
            Err(ParserError {
                position: self.lexer.get_position(),
                message: format!("Unexpected token '{}'", self.next_token),
            }
            .into())
        }
    }

    fn parse_number(&mut self) -> Result<f64> {
        if let Token::Number(f) = self.next_token {
            self.advance_tokens();
            Ok(f)
        } else {
            Err(ParserError {
                position: self.lexer.get_position(),
                message: format!("Unexpected token '{}'", self.next_token),
            }
            .into())
        }
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
        self.expect_token(Token::OpenBrace)?;
        let value = self.parse_expression()?;

        self.expect_token(Token::Comma)?;
        let minimum = self.parse_expression()?;

        self.expect_token(Token::Comma)?;
        let maximum = self.parse_expression()?;

        self.expect_token(Token::CloseBrace)?;

        Ok(Range {
            value,
            minimum,
            maximum,
        })
    }

    fn add_option(&mut self, options: &mut Options) -> Result<()> {
        match self.parse_name()?.as_str() {
            "color" => {
                self.expect_token(Token::Equal)?;
                let color = self.parse_string_literal()?;
                options.color = color;
                Ok(())
            }
            "width" => {
                self.expect_token(Token::Equal)?;
                let width = self.parse_number()? as u32;
                options.width = width;
                Ok(())
            }
            name => Err(ParserError {
                position: self.lexer.get_position(),
                message: format!("Unexpected option name: '{name}'"),
            }
            .into()),
        }
    }

    fn parse_plot_function(&mut self) -> Result<PlotFunctionNode> {
        if self.next_token == Token::OpenBrace {
            self.advance_tokens();
            let value = self.parse_expression()?;
            let mut options = Options::default();
            if self.next_token == Token::Comma {
                self.advance_tokens();
                self.add_option(&mut options)?;
                while self.next_token == Token::Comma {
                    self.advance_tokens();
                    self.add_option(&mut options)?;
                }
            }
            self.expect_token(Token::CloseBrace)?;
            Ok(PlotFunctionNode { value, options })
        } else {
            let value = self.parse_expression()?;
            Ok(PlotFunctionNode {
                value,
                options: Options::default(),
            })
        }
    }

    fn parse_expression(&mut self) -> Result<ExpressionNode> {
        self.parse_expression_bp(0)
    }

    fn parse_expression_bp(&mut self, min_bp: u8) -> Result<ExpressionNode> {
        let mut lhs = self.parse_primary()?;
        loop {
            let op = match &self.next_token {
                Token::EoI
                | Token::Comma
                | Token::CloseBrace
                | Token::CloseBracket
                | Token::CloseParenthesis
                | Token::SemiColon
                | Token::LessThan
                | Token::LessThanOrEqual
                | Token::Equal
                | Token::GreaterThan
                | Token::GreaterThanOrEqual => break,
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
            let rhs = self.parse_expression_bp(l_bp)?;

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
                if name == "If" {
                    let left = Box::new(self.parse_expression()?);
                    let op = self.parse_comparator()?;
                    let right = Box::new(self.parse_expression()?);

                    self.expect_token(Token::Comma)?;
                    let if_true = Box::new(self.parse_expression()?);

                    self.expect_token(Token::Comma)?;
                    let if_false = Box::new(self.parse_expression()?);

                    self.expect_token(Token::CloseParenthesis)?;

                    Ok(ExpressionNode::IfExpression {
                        condition: CompareNode { op, left, right },
                        if_true,
                        if_false,
                    })
                } else if name == "Sum" {
                    let value = Box::new(self.parse_expression()?);
                    self.expect_token(Token::Comma)?;
                    self.expect_token(Token::OpenBrace)?;
                    let variable_name = self.parse_name()?;
                    self.expect_token(Token::Comma)?;
                    let lower = Box::new(self.parse_expression()?);
                    self.expect_token(Token::Comma)?;
                    let upper = Box::new(self.parse_expression()?);
                    self.expect_token(Token::CloseBrace)?;
                    self.expect_token(Token::CloseParenthesis)?;
                    Ok(ExpressionNode::SumExpression {
                        value,
                        range: SumRange {
                            variable_name,
                            lower,
                            upper,
                        },
                    })
                } else {
                    let mut arguments = Vec::new();
                    arguments.push(self.parse_expression()?);
                    while self.next_token == Token::Comma {
                        self.advance_tokens();
                        arguments.push(self.parse_expression()?);
                    }
                    self.expect_token(Token::CloseParenthesis)?;

                    Ok(ExpressionNode::FunctionCall {
                        name,
                        args: arguments,
                    })
                }
            }
            Token::OpenParenthesis => {
                self.advance_tokens();
                let primary = self.parse_expression()?;
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
            Token::CloseParenthesis
            | Token::StringLiteral(_)
            | Token::OpenBracket
            | Token::CloseBracket
            | Token::OpenBrace
            | Token::CloseBrace
            | Token::Comma
            | Token::Equal
            | Token::NotEqual
            | Token::LessThan
            | Token::GreaterThan
            | Token::LessThanOrEqual
            | Token::GreaterThanOrEqual
            | Token::SemiColon => Err(ParserError {
                position: self.lexer.get_position(),
                message: format!("Unexpected token: '{}'", next_token),
            }
            .into()),
        }
    }

    fn parse_comparator(&mut self) -> Result<Comparator> {
        let result = match self.next_token {
            Token::Equal => Ok(Comparator::Equal),
            Token::LessThan => Ok(Comparator::LessThan),
            Token::LessThanOrEqual => Ok(Comparator::LessThanOrEqual),
            Token::GreaterThan => Ok(Comparator::GreaterThan),
            Token::GreaterThanOrEqual => Ok(Comparator::GreaterThanOrEqual),
            _ => {
                return Err(ParserError {
                    position: self.lexer.get_position(),
                    message: "Unexpected token: ')'".to_string(),
                }
                .into())
            }
        };
        self.advance_tokens();
        result
    }
}
