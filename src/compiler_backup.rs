use crate::ast::{Node, Type};
use crate::ast;
use regex::Regex;
use std::collections::HashMap;

fn error(msg: String) {
    println!("{}", msg);
    std::process::exit(1);
}

fn get_type(t: Node) -> ast::Type {
    if let Node::Type(x) = t {
        x
    } else {
        ast::Type::Invalid
    }
}

fn compile_node(
    line_number: usize,
    defined_functions: &mut HashMap<String, (String, String)>,
    defined_variables: &mut HashMap<String, (String, String)>,
    node: &Node,
) -> String {
    let mut compiled_output = String::new();

    match node {
        Node::Int(x) => {
            compiled_output += &x;
        }

        Node::Str(x) => {
            // compiled_output += &x.replace("\\s", " ");
            compiled_output += &x.replace("\\s", " ");
        }

        Node::Name(x) => {
            compiled_output += &x;
        }

        Node::Op(op, a, b) => {
            compiled_output += &format!(
                "{} {} {}",
                {
                    match op.to_string().as_str() {
                        "+" => {
                            "add"
                        }

                        "-" => {
                            "sub"
                        }

                        "*" => {
                            "mul"
                        }

                        "/" => {
                            "div"
                        }

                        "%" => {
                            "mod"
                        }

                        _ => {""}
                    }
                },
                compile_node(
                    line_number,
                    defined_functions,
                    defined_variables,
                    node
                ),
                compile_node(
                    line_number,
                    defined_functions,
                    defined_variables,
                    node
                )
            );
        }

        Node::Type(x) => {
            // compiled_output += &compile_type(x);
        }

        Node::Assign(name, input_type, val) => {
            let variable_value = (
                    compile_node(
                        line_number,
                        defined_functions,
                        defined_variables,
                        input_type
                    ),
                    compile_node(
                        line_number,
                        defined_functions,
                        defined_variables,
                        val
                    )
                );
            defined_variables.insert(
                name.to_string(),
                variable_value
            );
        }

        Node::Return(e) => {

        }

        Node::Block(all) => {
            for item in all.iter() {
                compiled_output += &compile_node(
                        line_number,
                        defined_functions,
                        defined_variables,
                        item
                    );
            }
        }

        Node::Import(module) => {
            // compiled_output += &format!("#include \"{}.h\"\n", module.replace("\"", ""));
            // compile_node(std::fs::read_to_string(module))
            // TODO //
        }

        Node::Label(name, body) => {
            let mut name = name;
            if name == "main" {
                let temp_name = ".ENTRY".to_string();
                let name = &temp_name;
            }

            compiled_output += &format!("label {}\n", name);
            compiled_output += &format!("{}", compile_node(
                line_number,
                defined_functions,
                defined_variables,
                node
            ))
        }

        Node::Print(to_print) => {
            compiled_output += &format!();
        }

        Node::If(e, body, other) => {

        }

        Node::While(e, body) => {

        }

        _ => {}
    }

    compiled_output
}

pub fn compile(ast: &Vec<Node>) -> String {
    let mut compiled_output = String::from("");
    let mut defined_functions: HashMap<String, (String, String)> = HashMap::new();
    let mut defined_variables: HashMap<String, (String, String)> = HashMap::new();

    for (line_number, item) in ast.iter().enumerate() {
        compiled_output += &compile_node(
            line_number,
            &mut defined_functions,
            &mut defined_variables,
            item
        );
    }

    compiled_output
}
