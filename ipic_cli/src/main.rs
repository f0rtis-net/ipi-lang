use ipic_parse::{self, create_parse_stream};

fn main() {
    let parser = create_parse_stream("((");
    for expr in parser {
        println!("{:?}", expr);
    }
}
