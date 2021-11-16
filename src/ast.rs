#[derive(Clone, Debug, std::cmp::PartialEq, std::cmp::Eq)]
pub enum Type {
    Num,
    None,
    Str,
    Identifier,
    Ptr(Box<Type>),
    Invalid,
    Block,
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
    Eq(Box<Node>, Box<Node>),
    NotEq(Box<Node>, Box<Node>),
    Gt(Box<Node>, Box<Node>),
    Lt(Box<Node>, Box<Node>),

    Assign(Box<Node>, Box<Node>),
    Print(Box<Node>),
    Comment(String),
    Block(Vec<Node>),
    Proc(Box<Node>, Box<Node>),
    Call(Box<Node>),
    If(Box<Node>, Box<Node>),
    IfElse(Box<Node>, Box<Node>, Box<Node>),
}
