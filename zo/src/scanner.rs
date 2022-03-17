
use crate::token::*;
use crate::token_types::*;
use crate::object::*;
use crate::error;

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: &str) -> Scanner {
        Scanner { 
            source: String::from(source),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }
    
    // --- Immutables
    fn end(self: &Self) -> bool {
        self.current >= self.source.len()
    }

    fn peek(self: &Self) -> Option<char> {
        if self.end() {
            return Some('\0');
        }

        self.source.chars().nth(self.current)
    }

    fn peek_next(self: &Self) -> Option<char> {
        if self.end() {
            return Some('\0');
        }
        
        self.source.chars().nth(self.current + 1)
    }

    // --- Mutables
    pub fn parse(self: &mut Self) -> Vec<Token> {

        while !self.end() {
            self.start = self.current;
            self.parse_token();
        }
        

        self.tokens.push(Token::new(
            TokenType::Eof,
            "",
            None,
            self.line,
        ));

        self.tokens.clone()
    }


    fn next(self: &mut Self) -> Option<char> {
        self.current += 1;
        self.source.chars().nth(self.current)
    }


    fn expected(self: &mut Self, expected_char: char) -> bool {
        if self.end() {
            return false
        }
       
        if let Some(next_char) = self.peek_next() {

            if next_char != expected_char {
                return false
            }
        }

        self.current += 1;
        true
    }

    fn parse_token(self: &mut Self) {
        let character = self.next();

        if let Some(c) = character {
            match c {
                '(' => self.push_token(TokenType::LeftParenthesis),
                ')' => self.push_token(TokenType::RightParenthesis),
                '{' => self.push_token(TokenType::LeftBrace),
                '}' => self.push_token(TokenType::RightBrace),
                ',' => self.push_token(TokenType::Comma),
                '.' => self.push_token(TokenType::Dot),
                '-' => self.push_token(TokenType::Minus),
                '+' => self.push_token(TokenType::Plus),
                ';' => self.push_token(TokenType::Semicolon),
                '*' => self.push_token(TokenType::Star),
                
                // Operators
                '!' => {
                    let token_type: TokenType = if self.expected('=') { 
                        TokenType::BangEqual
                    } else {
                        TokenType::Bang
                    };

                    self.push_token(token_type);
                },
                '=' => {
                    let token_type: TokenType = if self.expected('=') { 
                        TokenType::EqualEqual
                    } else {
                        TokenType::Equal
                    };

                    self.push_token(token_type);
                },
                '<' => {
                    let token_type: TokenType = if self.expected('=') { 
                        TokenType::LessEqual
                    } else {
                        TokenType::Less
                    };

                    self.push_token(token_type);
                },
                '>' => {
                    let token_type: TokenType = if self.expected('=') { 
                        TokenType::GreaterEqual
                    } else {
                        TokenType::Greater
                    };

                    self.push_token(token_type);
                },
                '/' => {
                    if self.expected('/') { 
                        // Comment
                        while self.peek().unwrap() != '\n' && !self.end() {
                            self.next();
                        }
                    } else {
                        self.push_token(TokenType::Slash);
                    };

                },

                // Ignores whitespaces
                ' ' => {},
                '\r' => {},
                '\t' => {},

                // Increase the line count
                '\n' => {
                    self.line += 1;
                },

                // String
                '"' => {
                    self.parse_string();
                },


                _ => {
                    // TODO(devon): Figure out how you want to handle this at a later point
                    _ = error(self.line, "Unexpected character");
                }
            }
        }
    }

    fn parse_string(self: &mut Self) {
        while self.peek().unwrap() != '"' && !self.end() {
            if self.peek().unwrap() == '\n' {
                self.line += 1;
            }
            
            self.next();
        }

        if self.end() {
            error(self.line, "Unterminated string.");
            return;
        }

        // The close "
        self.next();

        // Trim the surrounding quotes
        let start_index: usize = self.start + 1;
        //let end_index: usize = self.current - 1;
        let count: usize = self.current - start_index;

        let string: String = self.source.chars().skip(start_index).take(count).collect();
        self.push_token_literal(TokenType::String, Some(Object::Str(string)));
    }

    fn push_token(self: &mut Self, token: TokenType) {
        self.push_token_literal(token, None);
    }

    fn push_token_literal(self: &mut Self, token_type: TokenType, literal: Option<Object>) {
        let text = &self.source[self.start..self.current];
        self.tokens.push(Token::new(
            token_type,
            text,
            literal,
            self.line
        ));
    }

}
