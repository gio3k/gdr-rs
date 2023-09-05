use crate::stage2::ast::{Expr, FunctionBody, Val};

pub struct ForStatement {
    pub iterator: Val::Identifier,
    pub target: Expr,
    pub body: FunctionBody,
}

pub struct WhileStatement {
    pub condition: Expr,
    pub body: FunctionBody,
}