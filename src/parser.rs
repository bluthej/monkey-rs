#![allow(unused)]

use crate::{
    ast::{Expression, Identifier, Program, Statement},
    lexer::Lexer,
    token::Token,
};
use std::{iter::Peekable, mem::discriminant};

struct Parser<'a> {
    l: Peekable<Lexer<'a>>,
    errors: Vec<String>,
}

impl<'a> Parser<'a> {
    fn new(l: Lexer<'a>) -> Self {
        Self {
            l: l.peekable(),
            errors: vec![],
        }
    }

    fn parse_program(&mut self) -> Program<'a> {
        let mut statements = vec![];
        while self.l.peek().is_some() {
            if let Some(stmt) = self.parse_statement() {
                statements.push(stmt);
            }
        }
        Program { statements }
    }

    fn parse_statement(&mut self) -> Option<Statement<'a>> {
        match self.l.next() {
            Some(Token::LET) => self.parse_let_statement(),
            Some(Token::RETURN) => self.parse_return_statement(),
            _ => None,
        }
    }

    fn parse_let_statement(&mut self) -> Option<Statement<'a>> {
        if !self.expect_peek(&Token::IDENT("")) {
            return None;
        }
        let Some(Token::IDENT(value)) = self.l.next() else {
            unreachable!("Peeked first");
        };
        if !self.expect_peek(&Token::ASSIGN) {
            return None;
        }
        self.l.find(|token| token == &Token::SEMICOLON);
        let identifier = Identifier { value };
        let value = Expression;
        Some(Statement::LetStatement { identifier, value })
    }

    fn expect_peek(&mut self, expected: &Token) -> bool {
        let actual_is_expected = match self.l.peek() {
            Some(actual) => discriminant(actual) == discriminant(expected),
            None => false,
        };
        if !actual_is_expected {
            self.peek_error(expected);
        }
        actual_is_expected
    }

    fn peek_error(&mut self, expected: &Token) {
        let msg = format!(
            "expected next token to be {:?}, got {:?} instead",
            Some(expected),
            self.l.peek()
        );
        self.errors.push(msg);
    }

    fn parse_return_statement(&mut self) -> Option<Statement<'a>> {
        self.l.find(|token| token == &Token::SEMICOLON);
        let value = Expression;
        Some(Statement::ReturnStatement { value })
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::Lex;

    use super::*;

    #[test]
    fn let_statement() {
        let input = "
let x = 5;
let y = 10;
let foobar = 838383;
";

        let mut p = Parser::new(input.tokens());
        let program = p.parse_program();

        check_parser_errors(&p);

        assert_eq!(
            program.statements.len(),
            3,
            "program.statements does not contain 3 statements, got: {}",
            program.statements.len()
        );

        let expected_identifiers = ["x", "y", "foobar"];
        for (statement, expected_identifier) in program.statements.iter().zip(expected_identifiers)
        {
            let Statement::LetStatement {
                identifier: name, ..
            } = statement else {
                panic!("The input should only contain let statements");
            };
            println!("{}", name.value);
            println!("{}", expected_identifier);
            assert_eq!(name.value, expected_identifier);
        }
    }

    #[test]
    #[should_panic]
    fn wrong_let_statement() {
        let input = "
let x 5;
let = 10;
let 838383;
";

        let mut p = Parser::new(input.tokens());
        let program = p.parse_program();

        check_parser_errors(&p);
    }

    fn check_parser_errors(p: &Parser) {
        if p.errors.is_empty() {
            return;
        }
        println!("parser has {} errors", p.errors.len());
        for error in &p.errors {
            eprintln!("{}", error);
        }
        panic!()
    }

    #[test]
    fn return_statement() {
        let input = "
return 5;
return 10;
return 993322;
";

        let mut p = Parser::new(input.tokens());
        let program = p.parse_program();

        check_parser_errors(&p);

        assert_eq!(
            program.statements.len(),
            3,
            "program.statements does not contain 3 statements, got: {}",
            program.statements.len()
        );

        for statement in program.statements {
            if !matches!(statement, Statement::ReturnStatement { .. }) {
                panic!("The input should only contain return statements");
            }
        }
    }
}
