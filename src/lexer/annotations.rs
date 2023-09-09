use crate::lexer::{Lexer, LexerError};
use crate::lexer::tokens::{Token, TokenKind, TokenValue};

impl<'a> Lexer<'a> {
    pub(crate) fn generic_annotation(&mut self) -> Result<(), LexerError> {
        let start = self.offset();

        loop {
            match self.see() {
                Some(' ' | '\n' | '\r' | '(') | None => {
                    return self.push_new_string_from_here(start, TokenKind::Annotation);
                }

                _ => {
                    self.advance();
                }
            }
        };
    }
}