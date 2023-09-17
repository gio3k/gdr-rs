use annotations::AnnotationStatement;
use blocks::BlockStatement;
use variables::{
    VariableDeclaration,
    ConstantDeclaration,
    PropertyDeclaration,
};
use crate::sponge::absorbers::statements::functions::FunctionDeclaration;
use crate::sponge::Sponge;
use crate::stage0::tokens::TokenKind;

pub mod blocks;
pub mod annotations;
pub mod variables;
pub mod functions;

pub enum Statement {
    Invalid,
    BlockStatement(Box<BlockStatement>),
    AnnotationStatement(Box<AnnotationStatement>),
    VariableDeclaration(Box<VariableDeclaration>),
    ConstantDeclaration(Box<ConstantDeclaration>),
    PropertyDeclaration(Box<PropertyDeclaration>),
    FunctionDeclaration(Box<FunctionDeclaration>),
}

impl<'a> Sponge<'a> {
    pub fn absorb_statement(&mut self) -> Statement {
        match self.token.kind {
            TokenKind::Var => self.absorb_var_to_statement(),
            TokenKind::Const => self.absorb_const_to_statement(),
            TokenKind::Annotation => self.absorb_annotation_to_statement(),
            _ => {
                Statement::Invalid
            }
        }
    }
}