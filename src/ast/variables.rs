use crate::ast::{Expr, Val};

pub struct VariableDefinition {
    pub identifier: Val,
    pub type_hint: Val,
    pub value: Expr,
    pub constant: Val,
}

pub struct VariableRef {
    pub identifier: Val,
}