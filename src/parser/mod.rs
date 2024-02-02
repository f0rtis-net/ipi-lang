use std::iter::Peekable;

use crate::ast::*;
use crate::lexer::*;
use self::grammar::NumberBase;
use self::grammar::TokenKind;

pub struct Parser {}

impl Parser {
    pub fn new() -> Parser {
        Parser { }
    }

    fn parse_to_num_expr(&mut self, base: NumberBase, val: String) -> Expression {
        let number = match base {
            NumberBase::BINARY => i32::from_str_radix(val.as_str(), 2),
            NumberBase::OCTAL => i32::from_str_radix(val.as_str(), 8),
            NumberBase::DECIMAL => i32::from_str_radix(val.as_str(), 10),
            NumberBase::HEX => i32::from_str_radix(val.as_str(), 16),
        };

        Expression::INTEGER(number.unwrap())
    }

    fn parse_prefix(&mut self, stream: &mut Peekable<impl Iterator<Item = TokenKind>>) -> Expression {
        let token = stream.next().unwrap();

        match token {
            TokenKind::INT { base, val } => self.parse_to_num_expr(base, val),
            _ => unimplemented!("Hello from prefix!")
        }
    }

    fn parse_postfix(&mut self, stream: &mut Peekable<impl Iterator<Item = TokenKind>>, prefix_expr: Expression) -> Expression {
        let token = stream.next().unwrap();
        match token {
            TokenKind::ADD => {
                Expression::BINARY { 
                    l: Box::new(prefix_expr), 
                    r: Box::new(self.parse_expr(stream)), 
                    op: '+' 
                }
            }

            _ => unimplemented!("Hello from postfix!")
        }
    }

    fn parse_expr(&mut self, stream: &mut Peekable<impl Iterator<Item = TokenKind>>) -> Expression {
        let mut left_expr = self.parse_prefix(stream);

        while let Some(tok) = stream.peek() {
            match tok {
                TokenKind::SEMICOLON => break,
                _ => left_expr = self.parse_postfix(stream, left_expr),
            };
        }

        left_expr
    }

    fn parse_expr_stmnt(&mut self, stream: &mut Peekable<impl Iterator<Item = TokenKind>>) -> Expression {
        let result  = self.parse_expr(stream);

        match stream.next().unwrap() {
            TokenKind::SEMICOLON => result,
            _ => panic!("unpredicted token on expression end"),
        }
    }
    
    pub fn decode_text(&mut self, str: &str) -> Vec<Expression> {
        let mut expressions = Vec::<Expression>::new();
        let mut stream = tokenize(str).peekable();
    
        while let Some(token) = stream.peek() {
            expressions.push(self.parse_expr_stmnt(&mut stream));
        }

        expressions
    }
}