#![allow(dead_code, clippy::upper_case_acronyms)]

#[derive(Debug, PartialEq)]
enum Token<'a> {
    ILLEGAL,
    EOF,

    // Identifiers + literals
    IDENT(&'a str), // add, foobar, x, y, ...
    INT(usize),     // 1343456

    // Operator
    ASSIGN,
    PLUS,

    // Delimiters
    COMMA,
    SEMICOLON,

    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,

    // Keywords
    FUNCTION,
    LET,
}

use Token::*;

fn look_up_ident(input: &str) -> Token {
    match input {
        "fn" => FUNCTION,
        "let" => LET,
        _ => IDENT(input),
    }
}

struct Lexer<'a> {
    input: &'a str,
    position: usize,
    read_position: usize,
}

impl Lexer<'_> {
    fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        let Some(c) = self.read_char() else {
            return EOF;
        };
        match c {
            '=' => ASSIGN,
            ';' => SEMICOLON,
            '(' => LPAREN,
            ')' => RPAREN,
            ',' => COMMA,
            '+' => PLUS,
            '{' => LBRACE,
            '}' => RBRACE,
            c if c.is_alphabetic() => look_up_ident(self.read_identifier()),
            c if c.is_ascii_digit() => INT(self.read_number().parse::<usize>().unwrap()),
            _ => ILLEGAL,
        }
    }

    fn read_char(&mut self) -> Option<char> {
        self.input[self.read_position..]
            .char_indices()
            .next()
            .map(|(p, c)| {
                self.position = self.read_position;
                self.read_position += p + 1;
                c
            })
    }

    fn skip_whitespace(&mut self) {
        let mut chars = self.input[self.read_position..].chars();
        let mut c = chars.next();
        while c.map_or(false, |c| c.is_whitespace()) {
            self.read_char();
            c = chars.next();
        }
    }

    fn read_identifier(&mut self) -> &str {
        let start = self.position;
        let mut chars = self.input[self.read_position..].chars();
        let mut c = chars.next();
        while c.map_or(false, |c| c.is_alphabetic()) {
            self.read_char();
            c = chars.next();
        }
        let end = self.read_position;
        &self.input[start..end]
    }

    fn read_number(&mut self) -> &str {
        let start = self.position;
        let mut chars = self.input[self.read_position..].chars();
        let mut c = chars.next();
        while c.map_or(false, |c| c.is_ascii_digit()) {
            self.read_char();
            c = chars.next();
        }
        let end = self.read_position;
        &self.input[start..end]
    }
}

fn new(input: &str) -> Lexer<'_> {
    Lexer {
        input,
        position: 0,
        read_position: 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::error::Error;

    type TestResult = Result<(), Box<dyn Error>>;

    #[test]
    fn basic_tokens() -> TestResult {
        let input = "=+(){},;";
        let expected_tokens = [
            ASSIGN, PLUS, LPAREN, RPAREN, LBRACE, RBRACE, COMMA, SEMICOLON, EOF,
        ];

        let mut l = new(input);

        for expected_token in expected_tokens {
            assert_eq!(l.next_token(), expected_token);
        }

        Ok(())
    }

    #[test]
    fn actual_source_code() -> TestResult {
        let input = "let five = 5;
let ten = 10;

let add = fn(x, y) {
    x + y;
};

let result = add(five, ten);";
        let expected_tokens = [
            LET,
            IDENT("five"),
            ASSIGN,
            INT(5),
            SEMICOLON,
            LET,
            IDENT("ten"),
            ASSIGN,
            INT(10),
            SEMICOLON,
            LET,
            IDENT("add"),
            ASSIGN,
            FUNCTION,
            LPAREN,
            IDENT("x"),
            COMMA,
            IDENT("y"),
            RPAREN,
            LBRACE,
            IDENT("x"),
            PLUS,
            IDENT("y"),
            SEMICOLON,
            RBRACE,
            SEMICOLON,
        ];

        let mut l = new(input);

        for expected_token in expected_tokens {
            assert_eq!(l.next_token(), expected_token);
        }

        Ok(())
    }
}
