use phf::{phf_map, Map};

use crate::token::*;
use crate::token_types::*;
use crate::object::*;
use crate::error;

static KEYWORDS: phf::Map<&'static str, TokenType> = phf_map! {
    "and" => TokenType::And,
    "struct" => TokenType::Struct,
    "else" => TokenType::Else,
    "false" => TokenType::False,
    "fn" => TokenType::Fn,
    "if" => TokenType::If,
    "null" => TokenType::Null,
    "or" => TokenType::Or,
    "print" => TokenType::Print,
    "return" => TokenType::Return,
    "self" => TokenType::SelfRef,
    "super" => TokenType::Super,
    "true" => TokenType::True,
    "var" => TokenType::Var,
    "while" => TokenType::While,
};

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
    
    // 
    

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
        if self.current + 1 >= self.source.len() {
            return Some('\0');
        }
        
        self
            .source
            .chars()
            .nth(self.current + 1)
    }

    fn is_numerical(self: &Self, c: char) -> bool {
        return c >= '0' && c <= '9';    
    }

    fn is_alphabetical(self: &Self, c: char) -> bool {
        return (c >= 'a' && c <= 'z') ||
               (c >= 'A' && c <= 'Z') ||
               c == '_';
    }

    fn is_alphanumeric(self: &Self, c: char) -> bool {
        return self.is_numerical(c) || self.is_alphabetical(c);
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
        let next_char = self.source.chars().nth(self.current);
        self.current += 1;
        next_char
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
                            _ = self.next();
                        }
                    } else {
                        self.push_token(TokenType::Slash);
                    };

                },

                // Ignores whitespaces
                ' '  => {},
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
                    if self.is_numerical(c) {
                        self.parse_number();
                    } else if self.is_alphabetical(c) {
                        self.parse_identifier();
                    } else {
                        // TODO(devon): Figure out how you want to handle this at a later point
                        _ = error(self.line, "Unexpected character");
                    }
                }
            }
        }
    }

    fn parse_string(self: &mut Self) {
        while self.peek().unwrap() != '"' && !self.end() {
            if self.peek().unwrap() == '\n' {
                self.line += 1;
            }
            
            _ = self.next();
        }

        if self.end() {
            error(self.line, "Unterminated string.");
            return;
        }

        // The close "
        _ = self.next();

        // Trim the surrounding quotes
        let start_index: usize = self.start + 1;
        //let end_index: usize = self.current - 1;
        let count: usize = self.current - start_index;
        let string: String = self.source.chars().skip(start_index).take(count).collect();

        self.push_token_literal(TokenType::String, Some(Object::Str(string)));
    }

    fn parse_number(self: &mut Self) {
        self.iterate_number();

        if let Some(c) = self.peek() {
            if c == '.' {
                if let Some(nc) = self.peek_next() {
                    if self.is_numerical(nc) {
                        _ = self.next(); // Consume the "."

                        self.iterate_number();
                    }
                }
            }  
        }

        // let string: String = 
        //     self
        //     .source
        //     .chars()
        //     .skip(self.start)
        //     .take(self.current)
        //     .collect();
        let string: String = String::from(&self.source[self.start..self.current]);

        // println!("{:?}, {}, {}", string, self.start, self.current);
        self.push_token_literal(TokenType::Number, 
            Some(
                Object::Number (
                    string
                    .parse::<f64>()
                    .unwrap()
                )
            )
        );
    }

    fn iterate_number(self: &mut Self) {
        let mut end_of_number: bool = false;

        while !end_of_number {
            if let Some(c) = self.peek() {
                if self.is_numerical(c) {
                    _ = self.next();
                    continue;
                }
            }

            end_of_number = true;
        }
    }

    fn parse_identifier(self: &mut Self) {
        let mut end_of_identifer = false;

        while !end_of_identifer {
            if let Some(c) = self.peek() {
                if self.is_alphanumeric(c) {
                    _ = self.next();
                    continue;
                }
            }
            end_of_identifer = true;
        }
        
        let mut tok_type = TokenType::Identifier;
        let text = &self.source[self.start..self.current];

        if let Some(t) = KEYWORDS.get(text) {
            tok_type = *t;
        }

        self.push_token(tok_type);
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

