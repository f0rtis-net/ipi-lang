use ipic_parse::{self, create_parse_stream};

fn main() {
    let parser = create_parse_stream("fn main() { let a = 10 + 10; 20; }");
    for expr in parser {
        println!("{:?}", expr);
    }
}
