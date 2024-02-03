use super::grammar::TokenKind;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum PriorityLevel {
    LOWEST = 0,
    EQUAL,
    CMP,
    SUM_SUB,
    DIV_MUL,
    PREFIX,
    LBRACE,
}

pub fn get_tok_priority(token: &TokenKind) -> PriorityLevel {
    match token {
        TokenKind::LBRACE => PriorityLevel::LBRACE,
        TokenKind::DIV | TokenKind::MUL => PriorityLevel::DIV_MUL,
        TokenKind::ADD | TokenKind::SUB => PriorityLevel::SUM_SUB,
        _ => PriorityLevel::LOWEST,
    }
}