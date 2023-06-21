#[derive(Debug, PartialEq)]
pub enum Token<'a> {
    Illegal,
    EoF,

    // Identifiers + literals
    Ident(&'a str), // add, foobar, x, y, ...
    Int(usize),     // 1343456

    // Operator
    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,

    LT,
    GT,

    Eq,
    NotEq,

    // Delimiters
    Comma,
    Semicolon,

    LParen,
    RParen,
    LBrace,
    RBrace,

    // Keywords
    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,
}

use Token::*;

pub fn look_up_ident(input: &str) -> Token {
    match input {
        "fn" => Function,
        "let" => Let,
        "true" => True,
        "false" => False,
        "if" => If,
        "else" => Else,
        "return" => Return,
        _ => Ident(input),
    }
}
