pub mod ast;
pub mod codegen;
pub mod ir;
pub mod parser;

use lalrpop_util::lalrpop_mod;
lalrpop_mod!(pub move_to_miden);
