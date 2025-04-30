use crate::ast;
use crate::ast::{Node, Type};
use std::collections::HashMap;

pub struct Interpreter {
    code: Vec<Node>,
    source_code: Vec<String>,
    line_number: usize,
    defined_procedures: HashMap<String, Node>,
    defined_variables: HashMap<String, (Type, String)>,
}

impl Interpreter {
    pub fn interpret(source_code: Vec<String>, code: Vec<Node>) {
        let mut interpreter = Interpreter {
            code: code.clone(),
            source_code,
            line_number: 0,
            defined_variables: HashMap::new(),
            defined_procedures: HashMap::new(),
        };
        for (line_number, item) in code.iter().enumerate() {
            interpreter.line_number = line_number;
            interpreter.interpret_node(item.clone());
        }
        println!(
            "\n\nDEBUGGING:\nVARIABLES: {:?}\nPROCEDURES: {:?}",
            interpreter.defined_variables, interpreter.defined_procedures
        );
    }

    fn error(msg: String) {
        println!("{}", msg);
        std::process::exit(1);
    }

    fn get_variable(&mut self, var: String) -> (Type, String) {
        let mut output = self.defined_variables.get(&var).unwrap().clone();

        let identifier_regex = regex::Regex::new(r"[a-zA-Z_][a-zA-Z_0-9]*").unwrap();

        if output.0 == Type::Identifier {
            output = self.get_variable(output.1.clone());
        }

        output
    }

    fn get_string(string: String) -> String {
        // Remove the first \" and last \" and then return it using regex
        let string_regex = regex::Regex::new(r#"(")(.*)(")"#).unwrap();
        string_regex.replace_all(&string, "$2").to_string()
    }

    fn get_type_and_value(&mut self, node: Node) -> (ast::Type, String) {
        match node {
            Node::Num(num) => return (ast::Type::Num, num),
            Node::Str(le_str) => {
                return (
                    ast::Type::Str,
                    Interpreter::get_string(le_str).replace("\\n", "\n"),
                );
            } // `le` doesn't signify anything here
            Node::Identifier(ident) => return (ast::Type::Identifier, ident),
            Node::Add(left_node, right_node) => {
                // Check if left and right are numbers
                let (mut left_type, mut left) = self.get_type_and_value(*left_node);
                let (mut right_type, mut right) = self.get_type_and_value(*right_node);

                // Checking left
                if left_type == ast::Type::Identifier {
                    let (this_left_type, left_value) = self.get_variable(left);
                    left_type = this_left_type;
                    left = left_value;
                }

                if left_type == ast::Type::Str {
                    // left = Interpreter::get_string(left);
                } else if left_type != ast::Type::Num {
                    Interpreter::error(format!(
                        "Code:\n{} | {}\nProblem: `{}` is not a number.",
                        self.line_number, self.source_code[self.line_number], left
                    ));
                }

                // Checking right
                if right_type == ast::Type::Identifier {
                    let (this_right_type, right_value) = self.get_variable(right);
                    right_type = this_right_type;
                    right = right_value;
                }

                if right_type == ast::Type::Str {
                    // right = Interpreter::get_string(right);
                } else if right_type != ast::Type::Num {
                    Interpreter::error(format!(
                        "Code:\n{} | {}\nProblem: `{}` is not a number.",
                        self.line_number, self.source_code[self.line_number], right
                    ));
                }

                // Checking if both are same type
                if left_type != right_type {
                    Interpreter::error(format!(
                        "Code:\n{} | {}\nProblem: `{}` and `{}` are not the same type.",
                        self.line_number, self.source_code[self.line_number], left, right
                    ));
                }
                // Check if both are numbers
                if left_type == ast::Type::Num && right_type == ast::Type::Num {
                    return (
                        ast::Type::Num,
                        format!(
                            "{}",
                            left.parse::<f64>().unwrap() + right.parse::<f64>().unwrap()
                        ),
                    );
                } else if left_type == ast::Type::Str && right_type == ast::Type::Str {
                    // Check if both are strings
                    return (ast::Type::Str, format!("{}{}", left, right));
                }

                return (ast::Type::Invalid, "".to_string());
            }

            Node::Sub(left_node, right_node) => {
                // Check if left and right are numbers
                let (mut left_type, mut left) = self.get_type_and_value(*left_node);
                let (mut right_type, mut right) = self.get_type_and_value(*right_node);

                // Checking left
                if left_type == ast::Type::Identifier {
                    let (this_left_type, left_value) = self.get_variable(left);
                    left_type = this_left_type;
                    left = left_value;
                }
                if left_type != ast::Type::Num {
                    Interpreter::error(format!(
                        "Code:\n{} | {}\nProblem: `{}` is not a number.",
                        self.line_number, self.source_code[self.line_number], left
                    ));
                }

                // Checking right
                if right_type == ast::Type::Identifier {
                    let (this_right_type, right_value) = self.get_variable(right);
                    right_type = this_right_type;
                    right = right_value;
                }

                if right_type != ast::Type::Num {
                    Interpreter::error(format!(
                        "Code:\n{} | {}\nProblem: `{}` is not a number.",
                        self.line_number, self.source_code[self.line_number], right
                    ));
                }

                // Check if both are numbers
                if left_type == ast::Type::Num && right_type == ast::Type::Num {
                    return (
                        ast::Type::Num,
                        format!(
                            "{}",
                            left.parse::<f64>().unwrap() - right.parse::<f64>().unwrap()
                        ),
                    );
                }

                return (ast::Type::Invalid, "".to_string());
            }

            Node::Mul(left_node, right_node) => {
                // Check if left and right are numbers
                let (mut left_type, mut left) = self.get_type_and_value(*left_node);
                let (mut right_type, mut right) = self.get_type_and_value(*right_node);

                // Checking left
                if left_type == ast::Type::Identifier {
                    let (this_left_type, left_value) = self.get_variable(left);
                    left_type = this_left_type;
                    left = left_value;
                }

                if left_type != ast::Type::Num {
                    Interpreter::error(format!(
                        "Code:\n{} | {}\nProblem: `{}` is not a number.",
                        self.line_number, self.source_code[self.line_number], left
                    ));
                }

                // Checking right
                if right_type == ast::Type::Identifier {
                    let (this_right_type, right_value) = self.get_variable(right);
                    right_type = this_right_type;
                    right = right_value;
                }

                if right_type != ast::Type::Num {
                    Interpreter::error(format!(
                        "Code:\n{} | {}\nProblem: `{}` is not a number.",
                        self.line_number, self.source_code[self.line_number], right
                    ));
                }

                // Check if both are numbers
                if left_type == ast::Type::Num && right_type == ast::Type::Num {
                    return (
                        ast::Type::Num,
                        format!(
                            "{}",
                            left.parse::<f64>().unwrap() * right.parse::<f64>().unwrap()
                        ),
                    );
                }

                return (ast::Type::Invalid, "".to_string());
            }

            Node::Div(left_node, right_node) => {
                // Check if left and right are numbers
                let (mut left_type, mut left) = self.get_type_and_value(*left_node);
                let (mut right_type, mut right) = self.get_type_and_value(*right_node);

                // Checking left
                if left_type == ast::Type::Identifier {
                    let (this_left_type, left_value) = self.get_variable(left);
                    left_type = this_left_type;
                    left = left_value;
                }

                if left_type != ast::Type::Num {
                    Interpreter::error(format!(
                        "Code:\n{} | {}\nProblem: `{}` is not a number.",
                        self.line_number, self.source_code[self.line_number], left
                    ));
                }

                // Checking right
                if right_type == ast::Type::Identifier {
                    let (this_right_type, right_value) = self.get_variable(right);
                    right_type = this_right_type;
                    right = right_value;
                }

                if right_type != ast::Type::Num {
                    Interpreter::error(format!(
                        "Code:\n{} | {}\nProblem: `{}` is not a number.",
                        self.line_number, self.source_code[self.line_number], right
                    ));
                }

                // Check if both are numbers
                if left_type == ast::Type::Num && right_type == ast::Type::Num {
                    return (
                        ast::Type::Num,
                        format!(
                            "{}",
                            left.parse::<f64>().unwrap() / right.parse::<f64>().unwrap()
                        ),
                    );
                }

                return (ast::Type::Invalid, "".to_string());
            }

            Node::Eq(left, right) => {
                let (mut left_type, mut left) = self.get_type_and_value(*left);
                let (mut right_type, mut right) = self.get_type_and_value(*right);

                // Checking left
                if left_type == ast::Type::Identifier {
                    let (this_left_type, left_value) = self.get_variable(left);
                    left_type = this_left_type;
                    left = left_value;
                }

                // Checking right
                if right_type == ast::Type::Identifier {
                    let (this_right_type, right_value) = self.get_variable(right);
                    right_type = this_right_type;
                    right = right_value;
                }

                if left == right {
                    return (ast::Type::Num, "1".to_string());
                } else {
                    return (ast::Type::Num, "0".to_string());
                }

                return (ast::Type::Invalid, "".to_string());
            }

            Node::NotEq(left, right) => {
                let (mut left_type, mut left) = self.get_type_and_value(*left);
                let (mut right_type, mut right) = self.get_type_and_value(*right);

                // Checking left
                if left_type == ast::Type::Identifier {
                    let (this_left_type, left_value) = self.get_variable(left);
                    left_type = this_left_type;
                    left = left_value;
                }

                // Checking right
                if right_type == ast::Type::Identifier {
                    let (this_right_type, right_value) = self.get_variable(right);
                    right_type = this_right_type;
                    right = right_value;
                }

                if left != right {
                    return (ast::Type::Num, "1".to_string());
                } else {
                    return (ast::Type::Num, "0".to_string());
                }

                return (ast::Type::Invalid, "".to_string());
            }

            Node::Gt(left, right) => {
                let (mut left_type, mut left) = self.get_type_and_value(*left);
                let (mut right_type, mut right) = self.get_type_and_value(*right);

                // Checking left
                if left_type == ast::Type::Identifier {
                    let (this_left_type, left_value) = self.get_variable(left);
                    left_type = this_left_type;
                    left = left_value;
                }

                // Checking right
                if right_type == ast::Type::Identifier {
                    let (this_right_type, right_value) = self.get_variable(right);
                    right_type = this_right_type;
                    right = right_value;
                }

                if left_type != ast::Type::Num {
                    Interpreter::error(format!(
                        "Code:\n{} | {}\nProblem: `{}` is not a number.",
                        self.line_number, self.source_code[self.line_number], left
                    ));
                }

                if right_type != ast::Type::Num {
                    Interpreter::error(format!(
                        "Code:\n{} | {}\nProblem: `{}` is not a number.",
                        self.line_number, self.source_code[self.line_number], right
                    ));
                }

                if left.parse::<f64>().unwrap() > right.parse::<f64>().unwrap() {
                    return (ast::Type::Num, "1".to_string());
                } else {
                    return (ast::Type::Num, "0".to_string());
                }

                return (ast::Type::Invalid, "".to_string());
            }

            Node::Lt(left, right) => {
                let (mut left_type, mut left) = self.get_type_and_value(*left);
                let (mut right_type, mut right) = self.get_type_and_value(*right);

                // Checking left
                if left_type == ast::Type::Identifier {
                    let (this_left_type, left_value) = self.get_variable(left);
                    left_type = this_left_type;
                    left = left_value;
                }

                // Checking right
                if right_type == ast::Type::Identifier {
                    let (this_right_type, right_value) = self.get_variable(right);
                    right_type = this_right_type;
                    right = right_value;
                }

                if left_type != ast::Type::Num {
                    Interpreter::error(format!(
                        "Code:\n{} | {}\nProblem: `{}` is not a number.",
                        self.line_number, self.source_code[self.line_number], left
                    ));
                }

                if right_type != ast::Type::Num {
                    Interpreter::error(format!(
                        "Code:\n{} | {}\nProblem: `{}` is not a number.",
                        self.line_number, self.source_code[self.line_number], right
                    ));
                }

                if left.parse::<f64>().unwrap() < right.parse::<f64>().unwrap() {
                    return (ast::Type::Num, "1".to_string());
                } else {
                    return (ast::Type::Num, "0".to_string());
                }

                return (ast::Type::Invalid, "".to_string());
            }

            Node::Block(nodes) => {
                let mut result = (ast::Type::Block, "".to_string());
                return result; // No value for block
            }

            _ => return (ast::Type::Invalid, String::new()),
        }
    }

    fn run_procedure(&mut self, proc_name: Node) {
        let (proc_type, proc_value) = self.get_type_and_value(proc_name);
        if proc_type != ast::Type::Identifier {
            Interpreter::error(format!(
                "Code:\n{} | {}\nProblem: `{}` is not a procedure.",
                self.line_number, self.source_code[self.line_number], proc_value
            ));
        }

        let proc_name = proc_value;

        // Check if procedure exists
        if !self.defined_procedures.contains_key(&proc_name) {
            Interpreter::error(format!(
                "Code:\n{} | {}\nProblem: `{}` is not a procedure.",
                self.line_number, self.source_code[self.line_number], proc_name
            ));
        }

        let procedure = self.defined_procedures.get(&proc_name).unwrap();

        if let ast::Node::Block(nodes) = procedure.clone() {
            for node in nodes {
                self.interpret_node(node);
            }
        }
    }

    fn interpret_node(&mut self, node: Node) -> String {
        let mut compiled_output = String::new();

        match node {
            Node::Assign(identifier, value) => {
                let (mut identifier_type, mut identifier_value) =
                    self.get_type_and_value(*identifier);
                let (mut value_type, mut value) = self.get_type_and_value(*value);

                // Checking identifier
                if identifier_type != ast::Type::Identifier {
                    Interpreter::error(format!(
                        "Code:\n{} | {}\nProblem: `{}` is not a valid identifier.",
                        self.source_code[self.line_number], self.line_number, identifier_value
                    ));
                }

                // Checking value
                if !vec![ast::Type::Num, ast::Type::Str, ast::Type::Identifier]
                    .contains(&value_type)
                {
                    Interpreter::error(format!(
                        "Code:\n{} | {}\nProblem: `{}` is not a valid value.",
                        self.source_code[self.line_number], self.line_number, value
                    ));
                }
                if value_type == ast::Type::Identifier {
                    let (this_value_type, value_value) = self.get_variable(value);
                    value_type = this_value_type;
                    value = value_value;
                }

                // Assign the value to the variable
                self.defined_variables.insert(
                    identifier_value.to_string(),
                    (value_type, value.to_string()),
                );
            }

            Node::Print(value) => {
                let (mut value_type, mut value) = self.get_type_and_value(*value);

                // Checking value
                if value_type == ast::Type::Invalid {
                    Interpreter::error(format!(
                        "Code:\n{} | {}\nProblem: `{}` is not of a valid type.",
                        self.line_number, self.source_code[self.line_number], value
                    ));
                }

                if value_type == ast::Type::Identifier {
                    let (this_value_type, value_value) = self.get_variable(value);
                    value_type = this_value_type;
                    value = value_value;
                }

                if value_type == ast::Type::Str {
                    // value = Interpreter::get_string(value);
                }

                // Print the value
                print!("{}", value);
            }

            Node::Comment(comment) => {
                // Do nothing
            }

            Node::Proc(name, nodes) => {
                let (mut name_type, name) = self.get_type_and_value(*name);

                if name_type != ast::Type::Identifier {
                    Interpreter::error(format!(
                        "Code:\n{} | {}\nProblem: `{}` is not a valid identifier.",
                        self.line_number, self.source_code[self.line_number], name
                    ));
                }

                let (mut procedure_type, mut procedure) = self.get_type_and_value(*nodes.clone());

                if procedure_type != ast::Type::Block {
                    Interpreter::error(format!(
                        "Code:\n{} | {}\nProblem: `{}` is not a valid block.",
                        self.line_number, self.source_code[self.line_number], procedure
                    ));
                }

                // Add the procedure to the defined procedures
                self.defined_procedures.insert(name.to_string(), *nodes);
            }

            Node::Call(name) => {
                let (mut name_type, name_value) = self.get_type_and_value(*name.clone());

                if name_type != ast::Type::Identifier {
                    Interpreter::error(format!(
                        "Code:\n{} | {}\nProblem: `{}` is not a valid identifier.",
                        self.line_number, self.source_code[self.line_number], name_value
                    ));
                }

                // Check if the procedure exists
                if !self.defined_procedures.contains_key(&name_value) {
                    Interpreter::error(format!(
                        "Code:\n{} | {}\nProblem: `{}` is not a defined procedure.",
                        self.line_number, self.source_code[self.line_number], name_value
                    ));
                }

                // Run the procedure
                self.run_procedure(*name.clone());
            }

            Node::If(condition, nodes) => {
                let (mut condition_type, mut condition_value) = self.get_type_and_value(*condition);

                if condition_type != ast::Type::Num {
                    Interpreter::error(format!(
                        "Code:\n{} | {}\nProblem: `{}` is not a valid condition.",
                        self.line_number, self.source_code[self.line_number], condition_value
                    ));
                }

                if condition_value == "0" {
                    // Do nothing
                } else {
                    if let ast::Node::Block(nodes) = *nodes.clone() {
                        for node in nodes {
                            compiled_output += &self.interpret_node(node);
                        }
                    }
                }
            }

            Node::IfElse(condition, nodes, else_nodes) => {
                let (mut condition_type, mut condition_value) = self.get_type_and_value(*condition);

                if condition_type != ast::Type::Num {
                    Interpreter::error(format!(
                        "Code:\n{} | {}\nProblem: `{}` is not a valid condition.",
                        self.line_number, self.source_code[self.line_number], condition_value
                    ));
                }

                if condition_value == "0" {
                    if let ast::Node::Block(else_nodes) = *nodes.clone() {
                        for node in else_nodes {
                            compiled_output += &self.interpret_node(node);
                        }
                    }
                } else {
                    if let ast::Node::Block(nodes) = *nodes.clone() {
                        for node in nodes {
                            compiled_output += &self.interpret_node(node);
                        }
                    }
                }
            }

            _ => {}
        }

        compiled_output
    }
}
