#[derive(Debug, Copy, Clone)]
pub enum TokenType {
    // Single-character tokens
    LeftParenthesis,    // (
    RightParenthesis,   // )
    LeftBrace,          // [
    RightBrace,         // ]
    Comma,              // ,
    Dot,                // .
    Minus,              // -
    Plus,               // +
    Semicolon,          // ;
    Slash,              // /
    Star,             // *

    // One or two character tokens
    Bang,               // !
    BangEqual,          // !=
    Equal,              // =
    EqualEqual,         // ==
    Greater,            // >
    GreaterEqual,       // >=
    Less,               // <
    LessEqual,          // <=

    // Literals
    Identifier,
    String,
    Number,

    // Keywords
    And,
    Struct,
    Else,
    False,
    Fn,
    For,
    If,
    Null,
    Or,
    Print,
    Return,
    Super,
    SelfRef,
    True,
    Var,
    While,

    Eof
}
