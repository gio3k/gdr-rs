use crate::assert_token_kind;
use crate::core::literal::Literal;
use crate::sponge::absorbers::statements::Statement;
use crate::sponge::crumbs::Expression;
use crate::sponge::Sponge;
use crate::sponge::sponge_core::node::Node;
use crate::stage0::tokens::TokenKind;

pub struct VariableDeclaration {
    pub node: Node,
    pub name: Literal,
    pub type_hint: Literal,
    pub default: Expression,
}

pub struct ConstantDeclaration {
    pub node: Node,
    pub name: Literal,
    pub type_hint: Literal,
    pub value: Expression,
}

pub struct PropertyDeclaration {
    pub node: Node,
    pub name: Literal,
    pub type_hint: Literal,
    pub getter: Option<Expression>,
    pub setter: Option<Expression>,
}

impl<'a> Sponge<'a> {
    pub fn absorb_const_to_statement(&mut self) -> Statement {
        assert_token_kind!(self.token, TokenKind::Const);
    }

    pub fn absorb_var_to_statement(&mut self) -> Statement {
        assert_token_kind!(self.token, TokenKind::Var);
    }
}