use ipic_tokenize::grammar::TokenKind;

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Node {
    INTEGER(i32),
    BINARY{lhs: Box<Node>, rhs: Box<Node>, op: TokenKind},
    UNARY{val: Box<Node>, op: TokenKind}
}