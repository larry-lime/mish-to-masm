use crate::ast::*;
use crate::ir::{MidenInstruction, MidenProgram};
use std::collections::HashMap;

pub fn generate(module: &Module) -> String {
    let mut program = MidenProgram::new();
    for function in &module.functions {
        generate_function(&mut program, function);
    }
    program.to_string()
}

fn generate_function(program: &mut MidenProgram, func: &Function) {
    program.add_instruction(MidenInstruction::Proc(func.name.clone()));

    let mut vars = HashMap::new();

    for (index, param) in func.parameters.iter().enumerate() {
        vars.insert(param.name.clone(), index);
    }

    generate_expression(program, &func.body, &mut vars);

    program.add_instruction(MidenInstruction::End);
}

fn generate_expression(
    program: &mut MidenProgram,
    expr: &Expression,
    vars: &mut HashMap<String, usize>,
) {
    match expr {
        Expression::Add(left, right) => {
            generate_expression(program, left, vars);
            generate_expression(program, right, vars);
            program.add_instruction(MidenInstruction::Add);
        }
        Expression::Variable(name) => {
            if let Some(&index) = vars.get(name) {
                program.add_instruction(MidenInstruction::Push(index as u64));
            } else {
                panic!("Variable not found: {}", name);
            }
        }
        Expression::Literal(value) => {
            program.add_instruction(MidenInstruction::Push(*value));
        }
    }
}
