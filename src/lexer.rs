use crate::token::{look_up_ident, Token, Token::*};

pub struct Lexer<'a> {
    input: &'a str,
    position: usize,
    read_position: usize,
}

impl Lexer<'_> {
    pub fn new(input: &str) -> Lexer<'_> {
        Lexer {
            input,
            position: 0,
            read_position: 0,
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        let Some(c) = self.read_char() else {
        return EOF;
    };
        match c {
            '=' => match self.peek_char() {
                Some('=') => {
                    self.read_char();
                    EQ
                }
                _ => ASSIGN,
            },
            '+' => PLUS,
            '-' => MINUS,
            '!' => match self.peek_char() {
                Some('=') => {
                    self.read_char();
                    NOT_EQ
                }
                _ => BANG,
            },
            '/' => SLASH,
            '*' => ASTERISK,
            '<' => LT,
            '>' => GT,
            ';' => SEMICOLON,
            ',' => COMMA,
            '(' => LPAREN,
            ')' => RPAREN,
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

#[cfg(test)]
mod tests {
    use super::*;

    fn test_tokens(input: &str, expected_tokens: &[Token]) {
        let mut l = Lexer::new(input);

        for expected_token in expected_tokens {
            assert_eq!(&l.next_token(), expected_token);
        }
    }

    #[test]
    fn basic_tokens() {
        let input = "=+(){},;";
        let expected_tokens = &[
            ASSIGN, PLUS, LPAREN, RPAREN, LBRACE, RBRACE, COMMA, SEMICOLON, EOF,
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
        test_tokens(input, expected_tokens);
    }

    #[test]
    fn additionnal_tokens() {
        let input = "
!-/*5;
5 < 10 > 5;
        ";
        let expected_tokens = &[
            BANG,
            MINUS,
            SLASH,
            ASTERISK,
            INT(5),
            SEMICOLON,
            INT(5),
            LT,
            INT(10),
            GT,
            INT(5),
            SEMICOLON,
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
            IF,
            LPAREN,
            INT(5),
            LT,
            INT(10),
            RPAREN,
            LBRACE,
            RETURN,
            TRUE,
            SEMICOLON,
            RBRACE,
            ELSE,
            LBRACE,
            RETURN,
            FALSE,
            SEMICOLON,
            RBRACE,
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
            INT(10),
            EQ,
            INT(10),
            SEMICOLON,
            INT(10),
            NOT_EQ,
            INT(9),
            SEMICOLON,
        ];
        test_tokens(input, expected_tokens);
    }
}
