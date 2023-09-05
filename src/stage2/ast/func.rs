use crate::stage2::ast::{Expr, FunctionBody, Val};

pub struct FunctionArgument {
    pub identifier: Val::Identifier,
    pub default: Expr, // can this be anything (Expr) or just Vals?
}

pub struct FunctionDefinition {
    pub name: Val::Identifier,
    pub arguments: Vec<FunctionArgument>,
    pub body: FunctionBody,
}

pub struct FunctionCall {
    pub function: Val::Identifier,
    pub target: Expr,
    pub arguments: Vec<Expr>,
}

pub struct ReturnStatement {
    pub value: Expr,
}