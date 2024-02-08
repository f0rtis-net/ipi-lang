
use std::collections::HashMap;
use ast::Node;
use ipic_tokenize::{grammar::{LiteralKind, Token, TokenKind}, tokenize, TokenIterator};
mod ast;

type PrefixFn<'a> = fn(parser: &mut Parser<'a>) -> Node;
type InfixFn<'a> = fn(parse: &mut Parser<'a>, prefix_node: Node) -> Node;

struct Parser<'a> {
    stream: TokenIterator<'a>,
    infix_callback: HashMap<TokenKind, InfixFn<'a>>,
    prefix_callback: HashMap<TokenKind, PrefixFn<'a>>
}

impl<'a> Parser<'a> {
    fn new(stream: TokenIterator<'a>) -> Parser<'a> {
        let mut obj = Parser {
            stream,
            infix_callback: HashMap::new(),
            prefix_callback: HashMap::new()
        };

        obj.register_prefixes();

        return obj;
    }

    fn register_prefixes(&mut self) {
        self.prefix_callback.insert(
            TokenKind::LBRACE, Self::self_test
        ); 

        
        let prefix_ops = vec![
            TokenKind::DECREMENT, 
            TokenKind::INCREMENT, 
            TokenKind::MINUS,
            TokenKind::PLUS
            ];

        for prefix in prefix_ops {
            self.prefix_callback.insert(prefix, Self::parse_unary_prefix);
        }
    }

    fn parse_unary_prefix(&mut self) -> Node {
        let operator = self.stream.toks.peek().unwrap().clone();
        let expression = self.parse_statement().unwrap();
        Node::UNARY { val: Box::new(expression), op: operator.kind }
    }

    fn self_test(&mut self) -> Node {
        println!("{:?}",self.stream.toks.next());
        Node::INTEGER(10)
    }

    fn parse_literal(&mut self, literal: LiteralKind) -> Node {
        match literal {
            LiteralKind::INT { base, val } => Node::INTEGER(10),
            _ => Node::INTEGER(10)
        }
    }

    fn parse_expr_statement(&mut self) -> Node {
        unimplemented!("")
    }

    fn parse_ret_statement(&mut self) -> Node {
        unimplemented!("")
    }

    pub fn parse_statement(&mut self) -> Option<Node> {
        let tok = self.stream.toks.peek()?;

        match tok.kind.clone() {
            TokenKind::LITERAL(lit) => Some(self.parse_literal(lit)),
            _ => {
                let callback = self.prefix_callback.get(&tok.kind)?;
                Some(callback(self))
            }
        }
    }
}

pub fn create_parse_stream(input: &str) -> impl Iterator<Item = Node> + '_ {
    let tok_stream = tokenize(input);
    let mut parser = Parser::new(tok_stream);
    std::iter::from_fn(move || {
        parser.parse_statement() 
    })
}