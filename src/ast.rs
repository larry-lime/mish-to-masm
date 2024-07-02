// src/ast.rs
#[derive(Debug, PartialEq)]
pub enum Type {
    U64,
}

#[derive(Debug, PartialEq)]
pub struct Parameter {
    pub name: String,
    pub typ: Type,
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    Add(Box<Expression>, Box<Expression>),
    Variable(String),
    Literal(u64),
}

#[derive(Debug, PartialEq)]
pub struct Function {
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub return_type: Type,
    pub body: Expression,
}

#[derive(Debug, PartialEq)]
pub struct Module {
    pub functions: Vec<Function>,
}
