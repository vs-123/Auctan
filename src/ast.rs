#[derive(Clone, Debug, std::cmp::PartialEq, std::cmp::Eq)]
pub enum Type {
    Num,
    None,
    Str,
    Identifier,
    Ptr(Box<Type>),
    Invalid,
}

pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Eq,
    Neq,
    Lt,
    Gt,
    Le,
    Ge,
    And,
    Or,
    Not,
}

#[derive(Clone, Debug)]
pub enum Node {
    Num(String),
    Str(String),
    Identifier(String),
    Add(Box<Node>, Box<Node>),
    Sub(Box<Node>, Box<Node>),
    Mul(Box<Node>, Box<Node>),
    Div(Box<Node>, Box<Node>),
    Assign(Box<Node>, Box<Node>),
    Print(Box<Node>),
    Comment(String),
}
