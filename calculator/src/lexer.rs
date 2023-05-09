use crate::tokens::Token;

enum NumberParseState {
    State1,
    State2,
    State3,
    State4,
    State5,
    State6,
    State7,
    State8,
}

pub struct Lexer {
    input_chars: Vec<char>,
    previous_position: usize,
    position: usize,
}

impl Lexer {
    pub fn new(input_text: &str) -> Lexer {
        let input_chars: Vec<char> = input_text.chars().collect();
        Lexer {
            input_chars,
            previous_position: 0,
            position: 0,
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.previous_position = self.position;
        self.consume_whitespace();
        match self.read_next_char() {
            Some(ch) => match ch {
                '+' => Token::Plus,
                '-' => Token::Minus,
                '*' => Token::Times,
                '/' => Token::Divide,
                '^' => Token::Power,
                ',' => Token::Comma,
                '=' => Token::Equal,
                ';' => Token::SemiColon,
                '(' => Token::OpenParenthesis,
                ')' => Token::CloseParenthesis,
                '[' => Token::OpenBracket,
                ']' => Token::CloseBracket,
                '{' => Token::OpenBrace,
                '}' => Token::CloseBrace,
                '<' => {
                    let next_char = self.peek_char();
                    if next_char == Some(&'=') {
                        self.position += 1;
                        Token::LessThanOrEqual
                    } else {
                        Token::LessThan
                    }
                }
                '>' => {
                    if self.peek_char() == Some(&'=') {
                        self.position += 1;
                        Token::GreaterThanOrEqual
                    } else {
                        Token::GreaterThan
                    }
                }
                '!' => {
                    if self.peek_char() == Some(&'=') {
                        self.position += 1;
                        Token::NotEqual
                    } else {
                        Token::Illegal("Unexpected character. Expected '='".to_string())
                    }
                }
                '0'..='9' | '.' => {
                    self.position -= 1;
                    self.read_number()
                }
                'A'..='Z' | 'a'..='z' => {
                    self.position -= 1;
                    self.read_name()
                }
                '"' => {
                    let text = self.read_name();
                    if self.read_next_char() != Some(&'"') {
                        Token::Illegal("Unexpected character. Expected '\"'".to_string())
                    } else {
                        Token::StringLiteral(text.to_string())
                    }
                }
                _ => Token::Illegal(format!("Unexpected character: '{}'", ch)),
            },
            None => Token::EoI,
        }
    }

    pub fn get_position(&self) -> usize {
        self.position
    }

    fn read_name(&mut self) -> Token {
        // A valid function name starts with an upper letter and it is followed
        // by [a-z][A-Z]_[0-9]
        let position = self.position;
        while let Some(ch) = self.read_next_char() {
            match ch {
                'A'..='Z' | 'a'..='z' | '0'..='9' | '_' => {}
                _ => {
                    self.position -= 1;
                    break;
                }
            }
        }

        let name: String = self.input_chars[position..self.position].iter().collect();
        Token::Name(name)
    }

    fn read_next_char(&mut self) -> Option<&char> {
        let next_char = self.input_chars.get(self.position);
        if next_char.is_some() {
            self.position += 1;
        }
        next_char
    }

    fn peek_char(&self) -> Option<&char> {
        self.input_chars.get(self.position)
    }

    fn consume_whitespace(&mut self) {
        while let Some(&char) = self.input_chars.get(self.position) {
            if !char.is_whitespace() {
                break;
            }
            self.position += 1;
        }
    }

    // We use a standard parser.
    // The state of the art: https://arxiv.org/abs/2101.11408
    //                         digit                       digit                                  digit
    //    +-----+-----+       +------+                   +-----+             +-----+-----+       +-----+
    //    |     +     |       |      |                   |     |             |     +     |       |     |
    //    |           |       |      |                   |     v             |           v       |     v
    // +--+--+     +--+--+    |   +--+--+     +-----+    |  +--+--+      +---+-+      +--+--+    |  +--+--+
    // |  1  |     |  2  |    +---+  3  |  .  |  4  |    +--+  5  |      |  6  |      |  7  |    +--+  8  |
    // |     |     |     +------->+     +-----+     +-------+     +----->+     |      |     +------>+     |
    // ++-+--+     +--+--+ digit  +-+-+-+     +-----+ digit +-----+  E   +-+-+-+      +--+--+       +-----+
    //  | |           ^             | |                                    ^ |           ^
    //  | |     -     |             | |                                    | |     -     |
    //  | +-----+-----+             | |                                    | +-----+-----+
    //  |                           | |                                    |
    //  |         digit             | |                 E                  |
    //  +---------------------------+ +------------------------------------+
    fn read_number(&mut self) -> Token {
        let mut state = NumberParseState::State1;
        let start = self.position;
        let mut accept = true;
        while accept {
            if let Some(&c) = self.peek_char() {
                match state {
                    NumberParseState::State1 => {
                        if c.is_ascii_digit() {
                            state = NumberParseState::State3;
                        } else if c == '-' || c == '+' {
                            state = NumberParseState::State2;
                        } else {
                            return Token::Illegal(format!("Expecting digit or + or -, got {}", c));
                        }
                    }
                    NumberParseState::State2 => {
                        if c.is_ascii_digit() {
                            state = NumberParseState::State3;
                        } else {
                            return Token::Illegal(format!("Expecting digit got  {}", c));
                        }
                    }
                    NumberParseState::State3 => {
                        // Accepting state
                        if c == '.' {
                            state = NumberParseState::State4;
                        } else if c == 'E' || c == 'e' {
                            state = NumberParseState::State6;
                        } else if !c.is_ascii_digit() {
                            accept = false;
                        }
                    }
                    NumberParseState::State4 => {
                        if c.is_ascii_digit() {
                            state = NumberParseState::State5;
                        } else {
                            return Token::Illegal(format!("Expecting digit got  {}", c));
                        }
                    }
                    NumberParseState::State5 => {
                        // Accepting state
                        if c == 'e' || c == 'E' {
                            state = NumberParseState::State6;
                        } else if !c.is_ascii_digit() {
                            accept = false;
                        }
                    }
                    NumberParseState::State6 => {
                        if c == '+' || c == '-' || c.is_ascii_digit() {
                            state = NumberParseState::State7;
                        } else {
                            return Token::Illegal(format!("Expecting '+'or '-' got  {}", c));
                        }
                    }
                    NumberParseState::State7 => {
                        if c.is_ascii_digit() {
                            state = NumberParseState::State8;
                        } else {
                            return Token::Illegal(format!("Expecting digit got  {}", c));
                        }
                    }
                    NumberParseState::State8 => {
                        // Accepting state
                        if !c.is_ascii_digit() {
                            accept = false;
                        }
                    }
                }
                if accept {
                    self.position += 1;
                }
            } else {
                break;
            }
        }
        let str: String = self.input_chars[start..self.position].iter().collect();
        Token::Number(str.parse::<f64>().unwrap())
    }
}
