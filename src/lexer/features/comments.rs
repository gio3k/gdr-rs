use crate::lexer::core::error::{Error, ErrorKind};
use crate::lexer::Lexer;
use crate::{read, set_error_unless};
use crate::lexer::core::token::{Token, TokenKind};

impl<'a> Lexer<'a> {
    /// Parses a comment
    /// Assumes the iterator is on a comment start character (#)
    pub fn comment(&mut self) {
        let start = self.offset();

        set_error_unless!(
            self, Error::unrecoverable(ErrorKind::UnexpectedCurrentCharacter),
            Some('#')
        );

        read!{ self,
            (Some('\n' | '\r') | None) => {
                let mut token = Token::new(
                    start,
                    self.offset(),
                    TokenKind::LanguageComment
                );

                self.update_token_value_to_string(&mut token);

                return self.set_token(&token);
            },
            _ => {}
        }
    }
}