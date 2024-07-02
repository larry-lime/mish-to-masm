use move_to_miden_compiler::{ast::Module, codegen, parser};
use std::fs;

fn main() {
    let input = fs::read_to_string("src/example.move").expect("Failed to read input file");
    let ast: Module = parser::parse(&input).expect("Failed to parse input");
    let miden_code = codegen::generate(&ast);
    println!("Generated Miden Assembly:\n{}", miden_code);
}
