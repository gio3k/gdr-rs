use crate::stage2::ast::func::FunctionDefinition;
use crate::stage2::ast::Val;
use crate::stage2::ast::variables::VariableDefinition;

pub struct ClassDefinition {
    pub name: Val::Identifier,
    pub variables: Vec<VariableDefinition>,
    pub functions: Vec<FunctionDefinition>,
}