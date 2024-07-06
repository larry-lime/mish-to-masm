use move_compiler::{codegen, parser};
use std::fs;

fn main() {
    // Get input from file
    let input = fs::read_to_string("tests/move/functions.move").expect("Failed to read input file");
    let ast = parser::parse(&input).expect("Failed to parse input");
    let miden_code = codegen::generate(&ast);
    println!("Generated Miden Assembly:\n{}", miden_code);
}
