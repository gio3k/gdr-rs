use crate::ast::{Expr, FunctionBody, Val};

pub struct ForStatement {
    pub iterator: Val,
    pub target: Expr,
    pub body: FunctionBody,
}

pub struct WhileStatement {
    pub condition: Expr,
    pub body: FunctionBody,
}