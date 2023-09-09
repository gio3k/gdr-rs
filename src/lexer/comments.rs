use crate::lexer::{Lexer, LexerError};
use crate::lexer::tokens::{Token, TokenKind, TokenValue};

impl<'a> Lexer<'a> {
    /// Parse a comment token
    pub(crate) fn generic_comment(&mut self) -> Result<(), LexerError> {
        let start = self.offset();

        loop {
            match self.see() {
                None | Some('\n' | '\r') => {
                    return self.push_new_string_from_here(start, TokenKind::Comment);
                }

                None | _ => {}
            }

            self.advance();
        }
    }
}