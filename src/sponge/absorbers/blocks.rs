use crate::assert_token_kind;
use crate::core::literal::Literal;
use crate::sponge::crumbs::Statement;
use crate::sponge::Sponge;
use crate::stage0::tokens::TokenKind;

pub struct BlockStatement {
    pub name: Literal,
    pub depth: u32,
    pub body: Vec<Statement>,
}

impl<'a> Sponge<'a> {
    pub fn absorb_indents_for_depth_value(&mut self) -> i32 {
        assert_token_kind!(self.token, TokenKind::IndentTab | TokenKind::IndentSpaces);

        let is_space_based_indenting = match self.token.kind {
            TokenKind::IndentSpaces => true,
            _ => false,
        };

        // Current depth to return
        let mut depth: i32 = 1;

        loop {
            self.absorb();

            match self.token.kind {
                TokenKind::IndentTab => {
                    if is_space_based_indenting {}
                    depth += 1;
                }

                TokenKind::IndentSpaces => {
                    if !is_space_based_indenting {}
                    depth += 1;
                }

                _ => break,
            }
        }

        depth
    }
}