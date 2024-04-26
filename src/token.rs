#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Token {
    EOF,

    IDENT(String),
    STR(String),
    NUM(String),

    EQUAL,
    ADD,
    SUB,
    MUL,
    DIV,

    LBracket,
    RBracket,

    NoToken,
    Error,
}
