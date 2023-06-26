pub enum Statement<'a> {
    Let {
        identifier: &'a str,
        value: Expression,
    },
    Return(Expression),
}

pub struct Expression;

pub struct Program<'a> {
    pub statements: Vec<Statement<'a>>,
}
