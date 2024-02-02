mod lexer;
fn main() {
    let lexer = lexer::tokenize("12 0x16 0b11 + - / * ");

    for e in lexer {
        println!("token: {:?}", e);
    }
}
