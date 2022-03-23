use std::fmt;

use crate::token_types::*;
use crate::object::*;

#[derive(Debug, Clone)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Option<Object>,
    line: usize,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: &str, literal: Option<Object>, line: usize) -> Token {
        Token {
            token_type,
            lexeme: String::from(lexeme),
            literal,
            line,
        }

    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} {} {}", self.token_type, self.lexeme, if let Some(literal) = &self.literal {
            literal.to_string()
        } else {
            "None".to_string()
        })
    }
}
