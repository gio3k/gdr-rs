use crate::stage2::ast::class::ClassDefinition;
use crate::stage2::ast::func::{FunctionCall, FunctionDefinition};
use crate::stage2::ast::iteration::{ForStatement, WhileStatement};
use crate::stage2::ast::matching::MatchStatement;
use crate::stage2::ast::operations::{ValueComparison, ValueOperation, ValueAssignment};
use crate::stage2::ast::variables::{VariableDefinition};

mod operations;
mod variables;
mod func;
mod iteration;
mod class;
mod matching;

type FunctionBody = Vec<Expr>;

pub enum Expr {
    ValueOperation(ValueOperation),
    ValueComparison(ValueComparison),
    ValueAssignment(ValueAssignment),
    VariableDefinition(VariableDefinition),
    FunctionDefinition(FunctionDefinition),
    ClassDefinition(ClassDefinition),
    ForStatement(ForStatement),
    WhileStatement(WhileStatement),
    MatchStatement(MatchStatement),
    FunctionCall(FunctionCall),
}

pub enum Val {
    Identifier(String),
    Float(f64),
    Integer(i64),
    Boolean(bool),
}