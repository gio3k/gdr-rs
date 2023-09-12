use crate::lexer::core::error::{Error, ErrorKind};
use crate::lexer::Lexer;
use crate::{read, set_error_unless};
use crate::lexer::core::token::{Token, TokenKind, TokenValue};

impl<'a> Lexer<'a> {
    /// Parses indents - they're used for scope depth
    /// Assumes the iterator is on a tab or space character
    pub fn indented_scope_depth(&mut self) {
        let mut is_spaces: bool = false;
        let mut is_tabs: bool = false;
        let mut count: i64 = 0;
        let start = self.offset();

        set_error_unless!(
            self, Error::unrecoverable(ErrorKind::UnexpectedCurrentCharacter),
            Some('\t' | ' ')
        );

        read! { self,
            Some('\t') => {
                if is_spaces {
                    // We're expecting spaces and we just found a tab, error!
                    return self.set_error(Error::recoverable(ErrorKind::UnexpectedIndentTypeMismatch, 1));
                }

                is_tabs = true;
                count += 1;
            },

            Some(' ') => {
                if is_tabs {
                    // We're expecting tabs and we just found a space, error!
                    return self.set_error(Error::recoverable(ErrorKind::UnexpectedIndentTypeMismatch, 1));
                }

                is_spaces = true;
                count += 1;
            },

            _ => break
        }

        self.set_token_kind(TokenKind::LanguageIndent)
            .end_token_here(start);

        return if is_spaces {
            self.set_token_value(
                TokenValue::Integer(count / 4)
            );
        } else if is_tabs {
            self.set_token_value(
                TokenValue::Integer(count)
            );
        } else {
            self.set_error(Error::unrecoverable(ErrorKind::UnexpectedIndentTypeMismatch))
        };
    }
}