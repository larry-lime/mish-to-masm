use crate::ast::*;

pub fn generate(module: &Module) -> String {
    let mut output = String::new();
    for function in &module.functions {
        generate_function(&mut output, function);
    }
    output
}

fn generate_function(output: &mut String, func: &Function) {
    output.push_str(&format!("proc.{}.{}\n", func.name, func.parameters.len()));

    // Generate code for parameters
    for param in &func.parameters {
        output.push_str(&format!(
            "    # Parameter: {} ({})\n",
            param.name,
            type_to_string(&param.typ)
        ));
    }

    // Generate code for function body
    generate_expression(output, &func.body);

    output.push_str("    # Return value is on top of the stack\n");
    output.push_str("end\n\n");
}

fn generate_expression(output: &mut String, expr: &Expression) {
    match expr {
        Expression::Add(left, right) => {
            generate_expression(output, left);
            generate_expression(output, right);
            output.push_str("    add\n");
        }
        Expression::Variable(name) => {
            output.push_str(&format!("    # Load variable {}\n", name));
            // You'll need to implement proper variable handling
        }
        Expression::Literal(value) => {
            output.push_str(&format!("    push.{}\n", value));
        }
    }
}

fn type_to_string(typ: &Type) -> &'static str {
    match typ {
        Type::U64 => "u64",
    }
}
