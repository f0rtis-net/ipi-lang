use ipic_tokenize::grammar::TokenKind;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Expression {
    INTEGER(i32),
    BINARY{l: Box<Expression>, r: Box<Expression>, op: TokenKind},
    UNARY{obj: Box<Expression>, op: TokenKind},
    LET{name: String, val: Box<Expression>}
}