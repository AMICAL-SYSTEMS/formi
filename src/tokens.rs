use alloc::{
    collections::VecDeque,
    string::{String, ToString},
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token {
    Plus,
    Minus,
    Mul,
    Div,
    Mod,
    Dot,
    DotQuote,
    Quote,
    Colon,
    SemiColon,
    Cr,
    Do,
    Loop,
    I,
    Number(u64),
    Word(String),
}

pub type Tokens = VecDeque<Token>;

impl From<&str> for Token {
    fn from(value: &str) -> Self {
        match value {
            "+" => return Token::Plus,
            "-" => return Token::Minus,
            "*" => return Token::Mul,
            "/" => return Token::Div,
            "mod" => return Token::Mod,
            "." => return Token::Dot,
            ".\"" => return Token::DotQuote,
            "\"" => return Token::Quote,
            ":" => return Token::Colon,
            ";" => return Token::SemiColon,
            "cr" => return Token::Cr,
            "DO" => return Token::Do,
            "LOOP" => return Token::Loop,
            "I" => return Token::I,
            _ => match value.parse::<u64>() {
                Ok(num) => return Token::Number(num),
                Err(_) => return Token::Word(value.to_string()),
            },
        }
    }
}
