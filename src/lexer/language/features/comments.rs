use crate::lexer::core::error::{Error, ErrorKind};
use crate::lexer::Lexer;
use crate::{read, set_error_unless};
use crate::lexer::core::token::{Token, TokenKind};

pub const FEATURE_COMMENT: char = '#';

impl<'a> Lexer<'a> {
    /// Parses a comment
    /// Assumes the iterator is on a comment start character (#)
    pub fn comment(&mut self) {
        let start = self.offset();

        set_error_unless!(
            self, Error::unrecoverable(ErrorKind::UnexpectedCurrentCharacter),
            Some(FEATURE_COMMENT)
        );

        read! { self,
            (Some('\n' | '\r') | None) => {
                self.set_token_kind(TokenKind::LanguageComment)
                    .end_token_here(start)
                    .make_token_symbol();
                break;
            },
            _ => {}
        }
    }
}