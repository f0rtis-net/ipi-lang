use super::*;
use expect_test::{expect, Expect};

fn check_lexing(src: &str, expect: Expect) {
    let lexed: String = tokenize(src).toks
    .map(|token| {format!("{:?}\n", token.kind)}).collect();

    expect.assert_eq(&lexed);
}

#[test]
fn test_lexer() {
    check_lexing("let struct return print println imm fn ", 
    expect![[r#"
        LET
        STRUCT
        RETURN
        PRINT
        PRINTLN
        IMMUTABLE
        FUNCTION
    "#]]);
    
    check_lexing(
        "+ - / * . .. -> : ; ",
        expect![[r#"
            PLUS
            MINUS
            SLASH
            STAR
            DOT
            RANGE
            ARROW
            COLON
            SEMICOLON
        "#]]
    );
}