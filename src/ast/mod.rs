#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Expression {
    INTEGER(i32),
    BINARY{l: Box<Expression>, r: Box<Expression>, op: char}
}