use std::fmt;

#[derive(PartialEq, Clone, Debug)]
pub enum Token {
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
    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
    LessThanOrEqual,
    GreaterThanOrEqual,
    Illegal(String),
    EoI,
}

impl fmt::Display for Token {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Illegal(s) => write!(fmt, "Illegal Token: {}", s),
            Token::EoI => write!(fmt, "End Of Input"),
            Token::Number(f) => write!(fmt, "{}", f),
            Token::Name(s) => write!(fmt, "{}", s),
            Token::OpenParenthesis => write!(fmt, "("),
            Token::CloseParenthesis => write!(fmt, ")"),
            Token::Plus => write!(fmt, "+"),
            Token::Minus => write!(fmt, "-"),
            Token::Times => write!(fmt, "*"),
            Token::Divide => write!(fmt, "/"),
            Token::Power => write!(fmt, "^"),
            Token::OpenBracket => write!(fmt, "["),
            Token::CloseBracket => write!(fmt, "]"),
            Token::OpenBrace => write!(fmt, "{{"),
            Token::CloseBrace => write!(fmt, "}}"),
            Token::Comma => write!(fmt, ","),
            Token::SemiColon => write!(fmt, ";"),
            Token::Equal => write!(fmt, "="),
            Token::LessThan => write!(fmt, "<"),
            Token::GreaterThan => write!(fmt, "^>"),
            Token::LessThanOrEqual => write!(fmt, "<="),
            Token::GreaterThanOrEqual => write!(fmt, ">="),
            Token::StringLiteral(s) => write!(fmt, "{}", s),
            Token::NotEqual => write!(fmt, "!="),
        }
    }
}
