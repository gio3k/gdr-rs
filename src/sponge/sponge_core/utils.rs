use crate::sponge::Sponge;
use crate::stage0::tokens::TokenKind;

impl<'a> Sponge<'a> {
    /// Absorb / scan tokens until a non-line break is found
    pub fn skip_line_breaks(&mut self) {
        loop {
            match self.token.kind {
                TokenKind::LineBreak => {
                    self.scan();
                }

                _ => {
                    return;
                }
            }
        }
    }
}