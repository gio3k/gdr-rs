use crate::ast::func::FunctionDefinition;
use crate::ast::Val;
use crate::ast::variables::VariableDefinition;

pub struct ClassDefinition {
    pub name: Val,
    pub variables: Vec<VariableDefinition>,
    pub functions: Vec<FunctionDefinition>,
}