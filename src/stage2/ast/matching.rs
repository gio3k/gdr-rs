use crate::stage2::ast::{Expr, FunctionBody};

pub struct MatchCase {
    pub condition: Expr,
    pub body: FunctionBody,
}

pub struct MatchStatement {
    pub target: Expr,
    pub cases: Vec<MatchCase>,
}