use std::iter::Peekable;

use ast::Expression;
use ipic_tokenize::{grammar::{NumberBase, TokenKind}, *};
use priority::PriorityLevel;

#[cfg(test)]
mod tests;

mod priority;
pub mod ast;

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

    fn parse_in_brace_expr(
        &mut self, stream: 
        &mut Peekable<impl Iterator<Item = TokenKind>>) -> Expression {

        let result = self.parse_expr(stream, PriorityLevel::LOWEST);
        self.handle_error_type(stream, TokenKind::RBRACE);
        stream.next();
        return result;
    }

    fn parse_prefix(
        &mut self, 
        stream: &mut Peekable<impl Iterator<Item = TokenKind>>, 
        prefix_type: TokenKind) -> Expression {

        match prefix_type {
            TokenKind::DECREMENT => {
                let expr = self.parse_expr(stream, PriorityLevel::PREFIX);
                match expr {
                    _ => unimplemented!("cannot decrement read-only value")
                }
            }
            TokenKind::MINUS => {
                let expr = self.parse_expr(stream, PriorityLevel::PREFIX);

                Expression::UNARY { obj: Box::new(expr), op: TokenKind::MINUS }
            }
            TokenKind::PLUS => self.parse_left(stream), // ignore this format 

            _ => panic!("invalid prefix operator: {:?}", prefix_type)
        }
    }

    fn parse_left(&mut self, stream: &mut Peekable<impl Iterator<Item = TokenKind>>) -> Expression {
        let token = stream.next().unwrap();

        match token {
            TokenKind::LBRACE => self.parse_in_brace_expr(stream),
            TokenKind::INT { base, val } => self.parse_to_num_expr(base, val),
            _ => self.parse_prefix(stream, token)
        }
    }

    fn parse_right(
        &mut self, 
        stream: &mut Peekable<impl Iterator<Item = TokenKind>>, 
        prefix_expr: Expression) -> Expression {

        let token = stream.next().unwrap();
        match token {
            TokenKind::PLUS | TokenKind::MINUS => {
                Expression::BINARY { 
                    l: Box::new(prefix_expr), 
                    r: Box::new(self.parse_expr(stream,  PriorityLevel::SUM_SUB)), 
                    op: token
                }
            }

            TokenKind::STAR | TokenKind::SLASH => {
                Expression::BINARY { 
                    l: Box::new(prefix_expr), 
                    r: Box::new(self.parse_expr(stream,  PriorityLevel::DIV_MUL)), 
                    op: token
                }
            }

            TokenKind::LT | TokenKind::MT => {
                Expression::BINARY { 
                    l: Box::new(prefix_expr), 
                    r: Box::new(self.parse_expr(stream, PriorityLevel::CMP)), 
                    op: token
                }
            }

            TokenKind::DEQUAL => {
                Expression::BINARY { 
                    l: Box::new(prefix_expr), 
                    r: Box::new(self.parse_expr(stream, PriorityLevel::EQUAL)), 
                    op: token
                }
            }

            _ => unimplemented!("Hello from postfix!")
        }
    }

    fn parse_expr(
        &mut self, stream: 
        &mut Peekable<impl Iterator<Item = TokenKind>>, 
        priority: PriorityLevel) -> Expression {

        let mut left_expr = self.parse_left(stream);

        while let Some(tok) = stream.peek() {
            if priority  >= priority::get_tok_priority(tok)  {
                break;
            }

            match tok {
                TokenKind::SEMICOLON => break,
                _ => left_expr = self.parse_right(stream, left_expr),
            };
        }

        left_expr
    }

    fn parse_expr_stmnt(&mut self, stream: &mut Peekable<impl Iterator<Item = TokenKind>>) -> Expression {
        let result  = self.parse_expr(stream, PriorityLevel::LOWEST);
        self.handle_error_type(stream, TokenKind::SEMICOLON); stream.next();
        return result;
    }

    fn handle_error_type(
        &mut self, 
        stream: &mut Peekable<impl Iterator<Item = TokenKind>>, 
        needed: TokenKind) {
            
        let token =  stream.peek().unwrap_or(&TokenKind::EOF);
        match token {
            token if token == &needed => (),
            _ => panic!("Unpredicted token! Waited {:?}, but got {:?}", needed, token)
        };
    }

    fn parse_let_statement(&mut self, stream: &mut Peekable<impl Iterator<Item = TokenKind>>) -> Expression {
        let name = stream.next().unwrap_or(TokenKind::EOF);
        match name {
            TokenKind::IDENT(val) => {
                self.handle_error_type(stream, TokenKind::EQUAL); stream.next();
                Expression::LET { 
                    name: val, 
                    val: Box::new(self.parse_expr_stmnt(stream)) 
                }
            }
            _ => panic!("cannot find val name!")
        }
    }

    pub fn parse_statement(&mut self, stream: &mut Peekable<impl Iterator<Item = TokenKind>>) -> Expression {
        let tok = stream.peek().unwrap();
        match tok {
            TokenKind::LET => {
                stream.next();
                self.parse_let_statement(stream)
            },
            _ => self.parse_expr_stmnt(stream)
        }
    }
}

pub fn construct_expr_stream(input: &str) -> impl Iterator<Item = Expression> + '_ {
    let mut parser = Parser::new();
    let mut lexer = tokenize(input).peekable();

    std::iter::from_fn(move || {
        if let Some(_token) = lexer.peek() {
            let expression = parser.parse_statement(&mut lexer);
            Some(expression)
        } else {
            None
        }
    })
}