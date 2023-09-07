use crate::ast::Node;
use crate::ast::core::literals::Literal;

pub struct FunctionArgument {
    pub name: Literal,
    pub type_hint: Literal,
}

pub struct FunctionDefinition {
    pub identifier: Literal,
    pub type_hint: Literal,
    pub arguments: Vec<Node>,
    pub body: Vec<Node>,
}