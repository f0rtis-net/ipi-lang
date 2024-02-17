use ipic_tokenize::grammar::TokenKind;

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
        TokenKind::SLASH | TokenKind::STAR => PriorityLevel::DIV_MUL,
        TokenKind::PLUS | TokenKind::MINUS => PriorityLevel::SUM_SUB,
        TokenKind::LT | TokenKind::MT => PriorityLevel::CMP,
        TokenKind::DEQUAL => PriorityLevel::EQUAL,
        _ => PriorityLevel::LOWEST,
    }
}