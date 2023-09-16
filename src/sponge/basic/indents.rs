use crate::assert_token_kind;
use crate::lexer::token::TokenKind;
use crate::sponge::issues::{Issue, IssueKindWarning};
use crate::sponge::Sponge;

impl<'a> Sponge<'a> {
    pub fn absorb_indents_for_depth_value(&mut self) -> i32 {
        assert_token_kind!(self.token, (TokenKind::IndentTab | TokenKind::IndentSpaces));

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
                    if is_space_based_indenting {
                        self.push_issue(
                            Issue::warning(IssueKindWarning::IndentTypeMismatch, self.token.location)
                        )
                    }
                }

                TokenKind::IndentSpaces => {}

                _ => break,
            }
        }

        depth
    }
}