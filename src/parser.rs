use crate::ast::*;
use crate::move_to_miden::ModuleParser;

pub fn parse(input: &str) -> Result<Module, String> {
    let parser = ModuleParser::new();
    parser.parse(input).map_err(|e| e.to_string())
}
