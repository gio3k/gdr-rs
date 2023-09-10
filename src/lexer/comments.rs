use crate::lexer::{Lexer, LexerError};
use crate::lexer::tokens::TokenKind;

impl<'a> Lexer<'a> {
    /// Parse a comment token
    pub(crate) fn parse_generic_comment(&mut self) -> Result<(), LexerError> {
        let start = self.offset();

        loop {
            match self.see() {
                None | Some('\n' | '\r') => {
                    return self.insert_string_filled_token_here(start, TokenKind::LanguageComment);
                }

                _ => {}
            }

            self.advance();
        }
    }
}