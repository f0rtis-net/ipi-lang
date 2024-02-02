use super::*;

fn check_lexing(src: &str) {
    let lexed: String = tokenize(src)
    .map(|token| {format!("{:?}\n", token)}).collect();

    println!("{}", lexed);
}

#[test]
fn test_lexer() {
    check_lexing("+ - / * ");
    check_lexing("10 + 10    - 2 + 4 / 3 )");
    check_lexing("0x19 0x20 0b101 010");
}