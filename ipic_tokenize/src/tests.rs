use super::*;
use expect_test::{expect, Expect};

fn check_lexing(src: &str, expect: Expect) {
    let lexed: String = tokenize(src)
    .map(|token| {format!("{:?}\n", token)}).collect();

    expect.assert_eq(&lexed);
}

#[test]
fn test_lexer() {
    check_lexing("let", 
    expect![[r#"
        LET
    "#]]);
    
    check_lexing(
        "+ - / * ",
        expect![[r#"
            PLUS
            MINUS
            SLASH
            STAR
        "#]]
    );

    /*check_lexing("+ - / * ");
    check_lexing("10 + 10    - 2 + 4 / 3 )");
    check_lexing("0x19 0x20 0b101 010");
    check_lexing("let");*/
}