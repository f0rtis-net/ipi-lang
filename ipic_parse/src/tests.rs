use super::*;
use expect_test::{expect, Expect};

fn check_parsing(src: &str, expect: Expect) {
    let result: String = construct_expr_stream(src)
    .map(|token| {format!("{:?}\n", token)}).collect();

    expect.assert_eq(&result);
}

#[test]
fn test_parser() {
    check_parsing(
        "let test1 = 10 + 20;",
        expect![[r#"
        LET { name: "test1", val: BINARY { l: INTEGER(10), r: INTEGER(20), op: PLUS } }
        "#]]
    );

    check_parsing(
        "let test_1 = 10 + 20;",
        expect![[r#"
        LET { name: "test_1", val: BINARY { l: INTEGER(10), r: INTEGER(20), op: PLUS } }
        "#]]
    );
}