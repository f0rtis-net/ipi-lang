use ipic_parse;

fn main() {
    let mut expr_stream = ipic_parse::construct_expr_stream("let test1 = 10 + 20;");
    println!("{:?}", expr_stream.next().unwrap());
}
