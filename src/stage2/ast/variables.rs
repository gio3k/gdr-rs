use crate::stage2::ast::{Expr, Val};

pub struct VariableDefinition {
    pub identifier: Val::Identifier,
    pub type_hint: Val::Identifier,
    pub value: Expr,
    pub constant: Val::Boolean,
}

pub struct VariableRef {
    pub identifier: Val::Identifier
}