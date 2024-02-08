#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum NumberBase {
    BINARY = 2,
    OCTAL = 8,
    DECIMAL = 10,
    HEX = 16
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum LiteralKind {
    INT{base: NumberBase, val: String},
    STRING(String)
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum TokenKind {
    LITERAL(LiteralKind),
    IDENT(String),
    LET,
    PLUS,
    MINUS,
    STAR,
    SLASH,
    DECREMENT,
    INCREMENT,
    EQUAL,
    DEQUAL,
    LT, //less than
    MT, //more than
    SEMICOLON,
    COLON,
    LBRACE,
    RBRACE,
    DQUOTE,
    QUOTE,
    AMPERSAND,
    DOT,
    RANGE,
    STRUCT,
    RETURN,
    PRINT,
    PRINTLN,
    IMMUTABLE,
    FUNCTION,
    ARROW,
    EOF
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub pos: (i32, i32) // row | column
}

impl Token {
    pub fn uninited() -> Self {
        Self {
            kind: TokenKind::EOF,
            pos: (0, 0)
        }
    }
}