use self::grammar::{NumberBase, TokenKind};
use self::cursor::Cursor;

pub mod grammar;
pub mod cursor;

pub fn is_whitespace(symbol: char) -> bool {
    if symbol == ' ' { true } else { false }
}

impl Cursor<'_> {
    pub fn advance_token(&mut self) -> TokenKind {
        let first = match self.bump() {
            Some(symbol) => symbol,
            None => return TokenKind::EOF
        };

        match first {
            '+' => TokenKind::ADD,
            '-' => TokenKind::SUB,
            '/' => TokenKind::DIV,
            '*' => TokenKind::MUL,
            ';' => TokenKind::SEMICOLON,
            first if is_whitespace(first) => self.skip_whitespace(),
            first @ '0'..='9' => self.parse_num(first),
            _ => panic!("undefined type of token: {}", first)
        }
    }

    fn skip_whitespace(&mut self) -> TokenKind {
        self.eat_while(is_whitespace);
        TokenKind::WHITESPACE
    }

    fn parse_num(&mut self, first: char) -> TokenKind {
        let mut base = NumberBase::DECIMAL;
        if first == '0' {
            match self.first() {
                'x' => {
                    base = NumberBase::HEX;
                    self.bump();
                    TokenKind::INT { base: base, val: self.parse_num_to_str() }
                }
                'b' => {
                    base = NumberBase::BINARY;
                    self.bump();
                    TokenKind::INT { base: base, val: self.parse_num_to_str() }
                }
                'o' => {
                    base = NumberBase::OCTAL;
                    self.bump();
                    TokenKind::INT { base: base, val: self.parse_num_to_str() }
                }
                '0'..='9' | '_' => TokenKind::INT { base: base, val: self.parse_num_to_str() },
                _ => TokenKind::INT { base: base, val: "0".to_string() }
            }
        } else {
            let conv = first.to_string();
            TokenKind::INT { base: base, val: (conv + &self.parse_num_to_str()) }
        }
    }

    fn parse_num_to_str(&mut self) -> String {
        let mut result = String::new();

        loop {
            let chr = self.bump().unwrap_or('\0');
            match chr {
                '_' => _ = self.bump(),
                '0'..='9' => result.push(chr),
                _ => break,
            };
        };

        return result;
    }
}

pub fn tokenize(input: &str) -> impl Iterator<Item = TokenKind> + '_ {
    let mut cursor = Cursor::new(input);

    std::iter::from_fn(move || {
        let token = cursor.advance_token();
        if token != TokenKind::EOF {Some(token)} else {None}
    })
}