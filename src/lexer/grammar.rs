#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum NumberBase {
    BINARY = 2,
    OCTAL = 8,
    DECIMAL = 10,
    HEX = 16
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum TokenKind {
    IDENT(String),
    INT{base: NumberBase, val: String},
    ADD,
    SUB,
    MUL,
    DIV,
    DECREMENT,
    INCREMENT,
    SEMICOLON,
    LBRACE,
    RBRACE,
    EOF
}