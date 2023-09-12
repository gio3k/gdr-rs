use crate::lexer::core::error::{Error, ErrorKind};
use crate::lexer::Lexer;
use crate::{read, set_error_unless};
use crate::lexer::core::token::{Token, TokenKind, TokenValue};

pub const FEATURE_ANNOTATION: char = '@';

impl<'a> Lexer<'a> {
    /// Parses an annotation
    /// Assumes the iterator is on an annotation start character (@)
    pub fn annotation(&mut self) {
        let start = self.offset();

        set_error_unless!(
            self, Error::unrecoverable(ErrorKind::UnexpectedCurrentCharacter),
            Some(FEATURE_ANNOTATION)
        );

        read! { self,
            (Some(' ' | '\n' | '\r' | '(') | None) => {
                self.set_token_kind(TokenKind::LanguageAnnotation)
                    .end_token_here(start)
                    .make_token_value_string();
                break;
            },
            _ => {}
        }
    }
}