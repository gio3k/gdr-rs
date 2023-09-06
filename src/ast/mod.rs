use crate::ast::class::ClassDefinition;
use crate::ast::func::{FunctionCall, FunctionDefinition};
use crate::ast::iteration::{ForStatement, WhileStatement};
use crate::ast::matching::MatchStatement;
use crate::ast::operations::{ValueAssignment, ValueComparison, ValueOperation};
use crate::ast::variables::VariableDefinition;

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

fn test() {

}