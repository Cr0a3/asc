#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Token {
    EOF,

    IDENT(String), NUM(String), 

    EQUAL,
    ADD, SUB, MUL, DIV,

    NoToken, Error,
}