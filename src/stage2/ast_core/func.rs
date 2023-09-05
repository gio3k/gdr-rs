use crate::stage2::ast_core::{Expr, FunctionBody, Val};

pub struct FunctionArgument {
    pub identifier: Val,
    pub default: Expr, // can this be anything (Expr) or just Vals?
}

pub struct FunctionDefinition {
    pub name: Val,
    pub arguments: Vec<FunctionArgument>,
    pub body: FunctionBody,
}

pub struct FunctionCall {
    pub function: Val,
    pub target: Expr,
    pub parameters: Vec<Expr>,
}

pub struct ReturnStatement {
    pub value: Expr,
}