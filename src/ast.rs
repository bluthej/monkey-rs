pub enum Statement<'a> {
    LetStatement {
        identifier: Identifier<'a>,
        value: Expression,
    },
    ReturnStatement {
        value: Expression,
    },
}

pub struct Identifier<'a> {
    pub value: &'a str,
}

pub struct Expression;

pub struct Program<'a> {
    pub statements: Vec<Statement<'a>>,
}
