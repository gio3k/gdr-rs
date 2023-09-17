use crate::assert_token_kind;
use crate::sponge::Sponge;
use crate::sponge::sponge_issues::warning_kind::WarningKind;
use crate::stage0::tokens::TokenKind;

impl<'a> Sponge<'a> {
    pub fn absorb_indents_for_depth(&mut self) -> u32 {
        assert_token_kind!(self.token, TokenKind::IndentTab | TokenKind::IndentSpaces);

        let is_space_based_indenting = match self.token.kind {
            TokenKind::IndentSpaces => true,
            _ => false,
        };

        // Current depth to return
        let mut depth: u32 = 1;

        loop {
            self.scan();

            match self.token.kind {
                TokenKind::IndentTab => {
                    if is_space_based_indenting {
                        self.throw_warning_here(WarningKind::MultipleIndentTypesUsed);
                    }
                    depth += 1;
                }

                TokenKind::IndentSpaces => {
                    if !is_space_based_indenting {
                        self.throw_warning_here(WarningKind::MultipleIndentTypesUsed);
                    }
                    depth += 1;
                }

                _ => break,
            }
        }

        depth
    }
}