use crate::lexer::{Lexer, LexerError};
use crate::lexer::tokens::TokenKind;

impl<'a> Lexer<'a> {
    pub(crate) fn parse_annotation(&mut self) -> Result<(), LexerError> {
        let start = self.offset();

        loop {
            match self.see() {
                Some(' ' | '\n' | '\r' | '(') | None => {
                    return self.insert_string_filled_token_here(start, TokenKind::LanguageAnnotation);
                }

                _ => {
                    self.advance();
                }
            }
        };
    }
}