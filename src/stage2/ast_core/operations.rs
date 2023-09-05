use crate::stage2::ast_core::{Expr, Val};

pub enum OperationType {
    Add,
    Subtract,
    Multiply,
    Divide,
}

pub enum ComparisonType {
    EqualTo,
    LowerThan,
    GreaterThan,
    EqualToOrLowerThan,
    EqualToOrGreaterThan,
}

pub struct ValueOperation {
    pub operation: OperationType,
    pub a: Expr,
    pub b: Expr,
}

pub struct ValueComparison {
    pub comparison: ComparisonType,
    pub a: Expr,
    pub b: Expr,
}

pub struct ValueAssignment {
    pub target: Expr,
    pub value: Expr,
}