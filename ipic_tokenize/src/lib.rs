mod cursor;
pub mod grammar;
use std::iter::Peekable;
use self::grammar::*;
use self::cursor::*;

#[cfg(test)]
mod tests;

pub fn is_whitespace(symbol: char) -> bool {
    match symbol {
        ' ' => true,
        _ => false
    }
}

fn is_id_continue(c: char) -> bool {
    unicode_xid::UnicodeXID::is_xid_continue(c)
}

fn is_id(symbol: char) -> bool {
    symbol == '_' || unicode_xid::UnicodeXID::is_xid_start(symbol)
}

impl Cursor<'_> {
    pub fn advance_token(&mut self) -> Token {
        let first = match self.bump() {
            Some(symbol) => symbol,
            None => return Token{kind: TokenKind::EOF, pos:(self.line, self.col)}
        };

        if is_whitespace(first) {
            self.skip_whitespace();
            return self.advance_token();
        }

        if first == '\n' {
            self.line += 1;
            return self.advance_token();
        }

        if first == '/' {
            if self.first() == '/' {
                self.skip_comment_line(); 
                return self.advance_token();
            }
        }

        let kind = match first {
            '+' => match self.first() {
                '+' => { self.bump(); TokenKind::INCREMENT },
                _ => TokenKind::PLUS
            },
            '-' => match self.first() {
                '-' => { self.bump(); TokenKind::DECREMENT },
                '>' => { self.bump(); TokenKind::ARROW }
                _ => TokenKind::MINUS
            },
            '/' => TokenKind::SLASH,
            '=' => match self.first() {
                '=' => { self.bump(); TokenKind::DEQUAL },
                _ => TokenKind::EQUAL
            },
            '.' => match self.first() {
                '.' => { self.bump(); TokenKind::RANGE }
                _ => TokenKind::DOT
            },
            '>' => TokenKind::MT,
            '<' => TokenKind::LT,
            '*' => TokenKind::STAR,
            ':' => TokenKind::COLON,
            ';' => TokenKind::SEMICOLON,
            '(' => TokenKind::LBRACE,
            ')' => TokenKind::RBRACE,
            '{' => TokenKind::CLBRACE,
            '}' => TokenKind::CRBRACE,
            '"' => TokenKind::DQUOTE,
            '\'' => TokenKind::QUOTE,
            '&' => TokenKind::AMPERSAND,
            first @ '0'..='9' => self.parse_num(first),
            first if is_id(first) => self.parse_id(first),

            _ => panic!("undefined token at line: {} | col: {}", self.line, self.col)
        };

        Token{kind, pos:(self.line, self.col)}
    }

    fn parse_id(&mut self, symbol: char) -> TokenKind {
        let first = symbol;
        let mut result = String::from(first);

        loop {
            let chr = self.first();

            if !is_id_continue(chr) {
                break;
            }

            self.bump();

            result.push(chr);
        }
        
        match result.as_str() {
            "let" => TokenKind::RESERVED(ReservedIDents::LET),
            "struct" => TokenKind::RESERVED(ReservedIDents::STRUCT),
            "return" => TokenKind::RETURN,
            "print" => TokenKind::PRINT,
            "println" => TokenKind::PRINTLN,
            "imm" => TokenKind::IMMUTABLE,
            "fn" => TokenKind::RESERVED(ReservedIDents::FUNCTION),
            _ => TokenKind::IDENT(result)
        }
    }

    fn skip_comment_line(&mut self) {
        self.eat_while(|c| c != '\n');
    }

    fn skip_whitespace(&mut self) {
        self.eat_while(is_whitespace);
    }

    fn parse_num(&mut self, first: char) -> TokenKind {
        let mut base = NumberBase::DECIMAL;
        if first == '0' {
            let lit_kind = match self.first() {
                'x' => {
                    base = NumberBase::HEX;
                    self.bump();
                    LiteralKind::INT { base, val: self.parse_hex_num_to_str() }
                }
                'b' => {
                    base = NumberBase::BINARY;
                    self.bump();
                    LiteralKind::INT { base, val: self.parse_num_to_str() }
                }
                'o' => {
                    base = NumberBase::OCTAL;
                    self.bump();
                    LiteralKind::INT { base, val: self.parse_num_to_str() }
                }
                '0'..='9' | '_' => LiteralKind::INT { base: base, val: self.parse_num_to_str() },
                _ => LiteralKind::INT { base, val: "0".to_string() }
            };

            TokenKind::LITERAL(lit_kind)
        } else {
            let conv = first.to_string();
            TokenKind::LITERAL(LiteralKind::INT { base, val: (conv + &self.parse_num_to_str()) })
        }
    }

    fn parse_num_to_str(&mut self) -> String {
        let mut result = String::new();

        loop {
            let chr = self.first();
            match chr {
                '_' => _ = self.bump(),
                '0'..='9' => result.push(self.bump().unwrap()),
                _ => break,
            };
        };

        return result;
    }

    fn parse_hex_num_to_str(&mut self) -> String {
        let mut result = String::new();

        loop {
            let chr = self.first();
            match chr {
                '_' => _ = self.bump(),
                '0'..='9' | 'a'..='f' | 'A'..='F' => result.push(self.bump().unwrap()),
                _ => break,
            };
        };

        return result;
    }
}

pub struct TokenIterator<'a> {
    pub toks: Peekable<Box<dyn Iterator<Item = Token> + 'a>>
}

impl<'a> TokenIterator<'a> {
    fn new(tokens: Box<dyn Iterator<Item = Token> + 'a>) -> Self {
        TokenIterator { toks: tokens.peekable() }
    }
}

pub fn tokenize(input: &str) -> TokenIterator {
    let mut cursor = Cursor::new(input);

    let iterator = std::iter::from_fn(move || {
        let token = cursor.advance_token();
        if token.kind != TokenKind::EOF {Some(token)} else {None}
    }).peekable();

    TokenIterator::new(Box::new(iterator))
}




