#![allow(warnings, dead_code, unused_must_use)]
#![feature(box_into_inner)]
use std::env::args;
use std::fs::{self};
use std::process::Command;
pub mod ast;
mod compiler;
mod parser;

fn main() {
    let mut args = args();

    args.next().unwrap();

    let input_file = args.next().unwrap();

    let code = fs::read_to_string(input_file.clone()).unwrap();
    match parser::ProgramParser::new().parse(&code) {
        Ok(ast) => {
            compiler::Interpreter::interpret(code.lines().map(String::from).collect(), ast);
            // println!("{}", out);
        }
        Err(e) => {
            // let e = e.to_string().replace("r#\"[a-zA-Z_][a-zA-Z_0-9]*\"#", "a name");
            println!("Error: {}", e);
        }
    }
}
