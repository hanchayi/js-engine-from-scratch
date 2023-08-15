extern crate engine;
use std::fs::read_to_string;
use engine::exec;

fn main() {
    let buffer = read_to_string("tests/js/test.js").unwrap();
    exec(buffer);
}
