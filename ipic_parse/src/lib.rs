
use std::collections::HashMap;
use ast::Node;
use ipic_tokenize::{grammar::{LiteralKind, Token, TokenKind}, tokenize, TokenIterator};
use ipic_tokenize::grammar::{NumberBase, ReservedIDents};
use crate::priority_lvl::{get_tok_priority, PriorityLevel};

mod ast;
mod priority_lvl;

type PrefixFn<'a> = fn(parser: &mut Parser<'a>) -> Option<Node>;
type InfixFn<'a> = fn(parse: &mut Parser<'a>, prefix_node: Node) -> Option<Node>;

struct Parser<'a> {
    stream: TokenIterator<'a>,
    infix_callback: HashMap<TokenKind, InfixFn<'a>>,
    prefix_callback: HashMap<TokenKind, PrefixFn<'a>>,
    peek_tok: Token,
    current_tok: Token
}

impl<'a> Parser<'a> {
    fn new(stream: TokenIterator<'a>) -> Parser<'a> {
        let mut obj = Parser {
            stream,
            infix_callback: HashMap::new(),
            prefix_callback: HashMap::new(),
            peek_tok: Token::uninited(),
            current_tok: Token::uninited()
        };

        if obj.stream.toks.size_hint().0 > 2 {
            obj.bump();
            obj.bump();
        }

        obj.fill_prefix_callbacks();
        obj.fill_inifx_callbacks();

        return obj;
    }

    fn bump(&mut self) -> Option<Token>{
        self.current_tok = self.stream.toks.next()?;
        self.peek_tok = self.stream.toks.peek()?.clone();

        Some(self.current_tok.clone())
    }

    fn fill_prefix_callbacks(&mut self) {
        self.prefix_callback.insert(
            TokenKind::RESERVED(ReservedIDents::LET), Self::parse_var);

        self.prefix_callback.insert(
            TokenKind::LBRACE, Self::parse_in_brace);

        let prefix_ops = vec![
            TokenKind::DECREMENT,
            TokenKind::INCREMENT,
            TokenKind::MINUS,
            TokenKind::PLUS
        ];

        for prefix in prefix_ops {
            self.prefix_callback.insert(prefix, Self::parse_unary);
        }
    }

    fn parse_in_brace(&mut self) -> Option<Node> {
        let result = self.parse_prioritized_expr(PriorityLevel::LOWEST)?;
        self.validate_tok_type(TokenKind::RBRACE);
        return Some(result);
    }

    fn fill_inifx_callbacks(&mut self) {
        let infix_ops = vec![
            TokenKind::STAR,
            TokenKind::SLASH,
            TokenKind::MINUS,
            TokenKind::PLUS
        ];

        for infix in infix_ops {
            self.infix_callback.insert(infix, Self::parse_binary);
        }
    }

    fn validate_tok_type(&mut self, needed: TokenKind) {
        let kind = self.bump().unwrap_or(Token::uninited());

        match kind.kind {
            token if token == needed => (),
            _ => panic!("Unpredicted token! Waited {:?}, but got {:?}", needed, kind)
        };
    }

    fn parse_binary(&mut self, prefix: Node) -> Option<Node> {
        let operator = self.bump()?.kind;

        Some(Node::BINARY {
            lhs: Box::new(prefix),
            rhs: Box::new(self.parse_prioritized_expr(get_tok_priority(&operator))?),
            op: operator
        })
    }

    fn parse_unary(&mut self) -> Option<Node> {
        let op = self.current_tok.kind.clone();
        let expression = self.parse_prioritized_expr(PriorityLevel::PREFIX)?;

        match op {
            TokenKind::DECREMENT | TokenKind::INCREMENT => {
                match expression {
                    _ => unimplemented!("Not valid operator for increment or decrement")
                }
            }
            _ => ()
        };

        Some(Node::UNARY { val: Box::new(expression), op: op.clone() })
    }

    fn parse_to_num_expr(&mut self, base: NumberBase, val: String) -> Node {
        let number = match base {
            NumberBase::BINARY => i32::from_str_radix(val.as_str(), 2),
            NumberBase::OCTAL => i32::from_str_radix(val.as_str(), 8),
            NumberBase::DECIMAL => i32::from_str_radix(val.as_str(), 10),
            NumberBase::HEX => i32::from_str_radix(val.as_str(), 16),
        };

        Node::INTEGER(number.unwrap())
    }

    fn parse_literal(&mut self, literal: LiteralKind) -> Node {
        match literal {
            LiteralKind::INT { base, val } => self.parse_to_num_expr(base, val),
            _ => panic!("error literal type")
        }
    }

    fn parse_prefix(&mut self) -> Option<Node> {
        let tok = self.bump()?;

        let prefix = match tok.kind.clone() {
            TokenKind::LITERAL(lit) => self.parse_literal(lit),
            _ => {
                let callback = self.prefix_callback.get(&tok.kind);
                match callback {
                    Some(func) => func(self)?,
                    _ => panic!("unpredicted token kind for lhs parsing")
                }
            }
        };

        Some(prefix)
    }

    fn parse_prioritized_expr(&mut self, priority: PriorityLevel) -> Option<Node> {
        let mut left = self.parse_prefix()?;

        while let Some(tok) = self.stream.toks.peek() {
            if priority >= get_tok_priority(&tok.kind) {
                break
            }

            if tok.kind == TokenKind::SEMICOLON {
                break
            }

            let callback = self.infix_callback.get(&tok.kind)?;
            left = callback(self, left)?
        }
        Some(left)
    }

    fn parse_statement(&mut self) -> Option<Node> {
        let result = self.parse_prioritized_expr(PriorityLevel::LOWEST)?;
        self.validate_tok_type(TokenKind::SEMICOLON);

        Some(result)
    }

    fn parse_exprs_block(&mut self) -> Option<Node> {
        let mut expressions = Vec::<Node>::new();

        while self.peek_tok.kind != TokenKind::CRBRACE {
            expressions.push(self.parse_statement()?)
        }

        Some(Node::BLOCK(expressions))
    }

    fn parse_function(&mut self) -> Option<Node> {
        let name_tok = self.bump()?;

        let name = match name_tok.kind {
            TokenKind::IDENT(val) => val,
            _ => panic!("unpredicted name token kind.")
        };

        self.validate_tok_type(TokenKind::LBRACE);
        //arguments
        self.validate_tok_type(TokenKind::RBRACE);

        self.validate_tok_type(TokenKind::CLBRACE);

        let fn_body = self.parse_exprs_block()?;

        Some(Node::FUNCTION {
            name,
            arguments: Vec::new(),
            block: Box::new(fn_body),
            ret_type: TokenKind::RESERVED(ReservedIDents::VOID)
        })
    }

    fn parse_var(&mut self) -> Option<Node> {
        let name = self.bump()?;
        match name.kind {
            TokenKind::IDENT(val) => {
                self.validate_tok_type(TokenKind::EQUAL);
                Some(Node::VAL {
                    name: val,
                    expr: Box::new(self.parse_prioritized_expr(PriorityLevel::LOWEST)?)
                })
            }
            _ => panic!("not valid type for value name")
        }
    }

    pub fn parse_node(&mut self) -> Option<Node> {
        let tok = self.bump()?;

        match tok.kind {
            //TokenKind::LET => self.parse_var(), // global variable
            TokenKind::RESERVED(ReservedIDents::FUNCTION) => self.parse_function(),
            _ => panic!("error type of token to parse Node: {:?}", tok)
        }
    }
}

pub fn create_parse_stream(input: &str) -> impl Iterator<Item = Node> + '_ {
    let tok_stream = tokenize(input);
    let mut parser = Parser::new(tok_stream);
    std::iter::from_fn(move || {
        parser.parse_node()
    })
}