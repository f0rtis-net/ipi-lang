mod lexer;
mod parser;
mod ast;

fn main() {
    let mut parser = parser::Parser::new();
    for e in parser.decode_text("0xa + 20; 0xb + 0xa;") {
        println!("expression: {:?}", e);
    }
}
