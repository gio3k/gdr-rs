use crate::lexer::core::error::{Error, ErrorKind};
use crate::lexer::Lexer;
use crate::{read, set_error_unless};
use crate::lexer::core::token::{Token, TokenKind};

impl<'a> Lexer<'a> {
    /// Parses an annotation
    /// Assumes the iterator is on an annotation start character (@)
    pub fn annotation(&mut self) {
        let start = self.offset();

        set_error_unless!(
            self, Error::unrecoverable(ErrorKind::UnexpectedCurrentCharacter),
            Some('@')
        );

        read! { self,
            Some(' ' | '\n' | '\r' | '(') => {
                let mut token = Token::new(
                    start,
                    self.offset(),
                    TokenKind::LanguageAnnotation
                );

                self.update_token_value_to_string(&mut token);

                self.set_token(&token);
            },
            _ => {}
        }
    }
}