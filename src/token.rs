#[derive(Debug, Clone)]
pub enum Token {
    // White space
    Indent,
    NewLine,

    // Single-character tokens.
    LeftParen,
    RightParen,
    Minus,
    Plus,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    // AndAnd
    // PipePipe

    // Literals.
    Identifier(String),
    Number(i32),

    // Keywords.
    Let,
    If,
    Else,
    True,
    False,
    Print,
    Println,
}
