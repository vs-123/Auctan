#![allow(warnings, dead_code, unused_must_use)]
use lalrpop;

fn main() {
    lalrpop::Configuration::new()
        .generate_in_source_tree()
        .process();
}
