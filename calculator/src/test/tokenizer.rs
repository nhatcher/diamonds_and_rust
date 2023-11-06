use crate::{lexer::Lexer, tokens::Token};

#[test]
fn tokenize_formula() {
    let input_text = "a=-3";
    let mut lexer = Lexer::new(input_text);
    let tokens = [
        Token::Name("a".to_string()),
        Token::Equal,
        Token::Minus,
        Token::Number(3.0),
        Token::EoI,
    ];
    for token in tokens {
        assert_eq!(token, lexer.next_token());
    }
}
