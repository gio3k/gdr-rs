use crate::lexer::core::error::{Error, ErrorKind};
use crate::lexer::Lexer;
use crate::{read, set_error_unless};
use crate::lexer::core::token::{Token, TokenKind, TokenValue};

pub const FEATURE_ANNOTATION: char = '@';

impl<'a> Lexer<'a> {
    /// Parses an annotation
    /// Assumes the iterator is on an annotation start character (@)
    pub fn annotation(&mut self) {
        let token_start = self.offset();

        set_error_unless!(
            self, Error::unrecoverable(ErrorKind::UnexpectedCurrentCharacter),
            Some(FEATURE_ANNOTATION)
        );

        // Skip the first token
        self.next();
        let data_start = self.offset();

        read! { self,
            (Some(' ' | '\n' | '\r' | '(') | None) => {
                let end = self.offset();
                self.set_token_kind(TokenKind::LanguageAnnotation)
                    .set_token_pos(data_start, end)
                    .make_token_symbol()
                    .set_token_pos(token_start, end);
                break;
            },
            _ => {}
        }
    }
}