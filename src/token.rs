#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Token {
    EOF,

    IDENT(String), NUM(String), 

    EQUAL,
    ADD, SUB, MUL, DIV,

    RAX, RBX, RCX, RDX,
    EAX, EBX, ECX, EDX,
     AX,  BX,  CX,  DX,

    LBracket, RBracket,

    NoToken, Error,
}