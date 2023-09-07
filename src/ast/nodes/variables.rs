use crate::ast::Node;
use crate::ast::core::literals::Literal;

pub struct VariableDefinition {
    pub identifier: Literal,
    pub type_hint: Literal,
    pub value: Option<Box<Node>>,
    pub is_constant: bool,
}