use crate::stage2::ast_core::func::FunctionDefinition;
use crate::stage2::ast_core::Val;
use crate::stage2::ast_core::variables::VariableDefinition;

pub struct ClassDefinition {
    pub name: Val,
    pub variables: Vec<VariableDefinition>,
    pub functions: Vec<FunctionDefinition>,
}