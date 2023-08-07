use std::fs::read_to_string;

fn main() {
    let buffer = read_to_string("tests/helloworld.js").unwrap();
    println!("{}", buffer);
}
