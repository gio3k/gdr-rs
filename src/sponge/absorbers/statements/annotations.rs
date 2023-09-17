use crate::assert_token_kind;
use crate::core::literal::Literal;
use crate::script::Location;
use crate::sponge::absorbers::statements::Statement;
use crate::sponge::Sponge;
use crate::stage0::tokens::TokenKind;

pub struct AnnotationStatement {
    pub location: Location,
    pub name: Literal,
    pub target: Box<Statement>,
}

impl<'a> Sponge<'a> {
    pub fn absorb_annotation_to_statement(&mut self) -> Statement {
        assert_token_kind!(self.token, TokenKind::Annotation);
    }
}