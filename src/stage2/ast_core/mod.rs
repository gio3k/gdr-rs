use crate::stage2::ast_core::class::ClassDefinition;
use crate::stage2::ast_core::func::{FunctionCall, FunctionDefinition};
use crate::stage2::ast_core::iteration::{ForStatement, WhileStatement};
use crate::stage2::ast_core::matching::MatchStatement;
use crate::stage2::ast_core::operations::{ValueComparison, ValueOperation, ValueAssignment};
use crate::stage2::ast_core::variables::{VariableDefinition};

mod operations;
mod variables;
mod func;
mod iteration;
mod class;
mod matching;
mod script;

type FunctionBody = Vec<Expr>;

pub enum Expr {
    ValueOperation(Box<ValueOperation>),
    ValueComparison(Box<ValueComparison>),
    ValueAssignment(Box<ValueAssignment>),
    VariableDefinition(Box<VariableDefinition>),
    FunctionDefinition(FunctionDefinition),
    ClassDefinition(ClassDefinition),
    ForStatement(Box<ForStatement>),
    WhileStatement(Box<WhileStatement>),
    MatchStatement(Box<MatchStatement>),
    FunctionCall(Box<FunctionCall>),
}

pub enum Val {
    Identifier(String),
    Float(f64),
    Integer(i64),
    Boolean(bool),
}