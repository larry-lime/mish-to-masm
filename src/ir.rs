// src/ir.rs
// #![allow(clippy::style)]
#[derive(Debug, Clone)]

pub enum MidenInstruction {
    Proc(String),
    End,
    Push(u64),
    Add,
    Dup(usize),
    // Add more instructions as needed
}

#[derive(Debug, Clone)]
pub struct MidenProgram {
    pub instructions: Vec<MidenInstruction>,
}

impl MidenProgram {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        MidenProgram {
            instructions: Vec::new(),
        }
    }

    pub fn add_instruction(&mut self, instruction: MidenInstruction) {
        self.instructions.push(instruction);
    }

    #[allow(clippy::inherent_to_string)]
    pub fn to_string(&self) -> String {
        let mut output = String::new();
        for instruction in &self.instructions {
            match instruction {
                MidenInstruction::Proc(name) => {
                    output.push_str(&format!("proc.{}\n", name));
                }
                MidenInstruction::End => {
                    output.push_str("end\n");
                }
                MidenInstruction::Push(value) => {
                    output.push_str(&format!("    push.{}\n", value));
                }
                MidenInstruction::Add => {
                    output.push_str("    add\n");
                }
                MidenInstruction::Dup(index) => {
                    output.push_str(&format!("    dup.{}\n", index));
                } // Handle other instructions
            }
        }
        output
    }
}
