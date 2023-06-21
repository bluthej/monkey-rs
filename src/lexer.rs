use crate::token::{look_up_ident, Token, Token::*};

pub trait Lex {
    fn tokens(&self) -> Lexer;
}

impl<T: AsRef<str>> Lex for T {
    fn tokens(&self) -> Lexer {
        Lexer::new(self.as_ref())
    }
}

pub struct Lexer<'a> {
    input: &'a str,
    position: usize,
    read_position: usize,
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}

impl<'a> Lexer<'a> {
    pub fn new(input: &str) -> Lexer<'_> {
        Lexer {
            input,
            position: 0,
            read_position: 0,
        }
    }

    fn next_token(&mut self) -> Option<Token<'a>> {
        self.skip_whitespace();
        self.read_char().map(|c| match c {
            '=' => match self.peek_char() {
                Some('=') => {
                    self.read_char();
                    Eq
                }
                _ => Assign,
            },
            '+' => Plus,
            '-' => Minus,
            '!' => match self.peek_char() {
                Some('=') => {
                    self.read_char();
                    NotEq
                }
                _ => Bang,
            },
            '/' => Slash,
            '*' => Asterisk,
            '<' => LT,
            '>' => GT,
            ';' => Semicolon,
            ',' => Comma,
            '(' => LParen,
            ')' => RParen,
            '{' => LBrace,
            '}' => RBrace,
            c if c.is_alphabetic() => look_up_ident(self.read_identifier()),
            c if c.is_ascii_digit() => Int(self.read_number().parse::<usize>().unwrap()),
            _ => Illegal,
        })
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

    fn peek_char(&self) -> Option<char> {
        self.input[self.read_position..]
            .chars()
            .peekable()
            .peek()
            .copied()
    }

    fn skip_whitespace(&mut self) {
        let mut chars = self.input[self.read_position..].chars();
        let mut c = chars.next();
        while c.map_or(false, |c| c.is_whitespace()) {
            self.read_char();
            c = chars.next();
        }
    }

    fn read_identifier(&mut self) -> &'a str {
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

#[cfg(test)]
mod tests {
    use super::*;

    fn test_tokens(input: &str, expected_tokens: &[Token]) {
        let tokens: Vec<_> = input.tokens().collect();
        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn basic_tokens() {
        let input = "=+(){},;";
        let expected_tokens = &[
            Assign, Plus, LParen, RParen, LBrace, RBrace, Comma, Semicolon,
        ];
        test_tokens(input, expected_tokens);
    }

    #[test]
    fn actual_source_code() {
        let input = "let five = 5;
let ten = 10;

let add = fn(x, y) {
    x + y;
};

let result = add(five, ten);";
        let expected_tokens = &[
            Let,
            Ident("five"),
            Assign,
            Int(5),
            Semicolon,
            Let,
            Ident("ten"),
            Assign,
            Int(10),
            Semicolon,
            Let,
            Ident("add"),
            Assign,
            Function,
            LParen,
            Ident("x"),
            Comma,
            Ident("y"),
            RParen,
            LBrace,
            Ident("x"),
            Plus,
            Ident("y"),
            Semicolon,
            RBrace,
            Semicolon,
            Let,
            Ident("result"),
            Assign,
            Ident("add"),
            LParen,
            Ident("five"),
            Comma,
            Ident("ten"),
            RParen,
            Semicolon,
        ];
        test_tokens(input, expected_tokens);
    }

    #[test]
    fn additionnal_tokens() {
        let input = "
!-/*5;
5 < 10 > 5;
        ";
        let expected_tokens = &[
            Bang,
            Minus,
            Slash,
            Asterisk,
            Int(5),
            Semicolon,
            Int(5),
            LT,
            Int(10),
            GT,
            Int(5),
            Semicolon,
        ];
        test_tokens(input, expected_tokens);
    }

    #[test]
    fn if_statement_and_return() {
        let input = "
if (5 < 10) {
    return true;
} else {
    return false;
}
";
        let expected_tokens = &[
            If,
            LParen,
            Int(5),
            LT,
            Int(10),
            RParen,
            LBrace,
            Return,
            True,
            Semicolon,
            RBrace,
            Else,
            LBrace,
            Return,
            False,
            Semicolon,
            RBrace,
        ];
        test_tokens(input, expected_tokens);
    }

    #[test]
    fn two_character_tokens() {
        let input = "
10 == 10;
10 != 9;
";
        let expected_tokens = &[
            Int(10),
            Eq,
            Int(10),
            Semicolon,
            Int(10),
            NotEq,
            Int(9),
            Semicolon,
        ];
        test_tokens(input, expected_tokens);
    }
}
