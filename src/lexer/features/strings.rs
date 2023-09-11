use crate::lexer::core::error::{Error, ErrorKind};
use crate::lexer::Lexer;
use crate::{read, set_error_unless, set_error_when};
use crate::lexer::core::token::{Token, TokenKind, TokenValue};

impl<'a> Lexer<'a> {
    /// Parses a long string (""")
    /// Assumes the iterator is on the first character of text
    fn string_with_multiline_support(&mut self) {
        set_error_when!(
            self, Error::unrecoverable(ErrorKind::UnexpectedCurrentCharacter),
            Some('"')
        );

        let data_start = self.offset();
        let token_start = data_start - 3;

        read! { self,
            Some('\n') => {
                return self.set_error(Error::recoverable(ErrorKind::UnexpectedLineBreak, 1));
            },

            Some('"') => {
                self.next();

                match self.peek() {
                    Some('"') => {
                        self.next();

                        match self.peek() {
                            Some('"') => {
                                // Long string end complete
                                let data_end = self.offset();
                                let token_end = data_end + 3;
                                let mut token = Token::new(
                                    token_start,
                                    token_end,
                                    TokenKind::StringLiteral
                                );

                                token.value = TokenValue::String(
                                    self.slice_to_string_symbol(
                                        data_start, data_end
                                    )
                                );

                                return self.set_token(&token);
                            }

                            None => self.set_error(Error::recoverable(ErrorKind::UnexpectedEndOfFile, 1)),

                            _ => {}
                        }
                    }

                    None => self.set_error(Error::recoverable(ErrorKind::UnexpectedEndOfFile, 1)),

                    _ => {}
                }

            },

            _ => {}
        }
    }

    /// Parses a short string (')
    /// Assumes the iterator is on the first character of text
    fn short_string(&mut self) {
        set_error_when!(
            self, Error::unrecoverable(ErrorKind::UnexpectedCurrentCharacter),
            Some('\'' | '"')
        );

        let data_start = self.offset();
        let token_start = data_start - 1;

        read! { self,
            Some('\n') => {
                return self.set_error(Error::recoverable(ErrorKind::UnexpectedLineBreak, 1));
            },

            Some('\'') => {
                let data_end = self.offset();
                let token_end = data_end + 1;
                let mut token = Token::new(
                    token_start,
                    token_end,
                    TokenKind::StringLiteral
                );

                token.value = TokenValue::String(
                    self.slice_to_string_symbol(
                        data_start, data_end
                    )
                );

                return self.set_token(&token);
            },

            _ => {}
        }
    }

    /// Parses a string (")
    /// Assumes the iterator is on the first character of text
    fn generic_string(&mut self) {
        set_error_when!(
            self, Error::unrecoverable(ErrorKind::UnexpectedCurrentCharacter),
            Some('\'' | '"')
        );

        let data_start = self.offset();
        let token_start = data_start - 1;

        read! { self,
             Some('"') => {
                let data_end = self.offset();
                let token_end = data_end + 1;
                let mut token = Token::new(
                    token_start,
                    token_end,
                    TokenKind::StringLiteral
                );

                token.value = TokenValue::String(
                    self.slice_to_string_symbol(
                        data_start, data_end
                    )
                );

                return self.set_token(&token);
            },

            _ => {}
        }
    }

    /// Parses a string, automatically detecting the type
    /// Assumes the iterator is on a valid string starter
    pub fn string_literal(&mut self) {
        set_error_unless!(
            self, Error::unrecoverable(ErrorKind::UnexpectedCurrentCharacter),
            Some('"' | '\'')
        );

        match self.peek() {
            Some('\'') => {
                self.next();
                self.short_string();
            }

            Some('"') => {
                self.next();

                // Let's see if this is a long string or normal string
                match self.peek() {
                    Some('"') => {
                        // This is either the end of a normal string or the 2nd character of a long string
                        match self.peek() {
                            Some('"') => {
                                // Another quote, this is a long string
                                self.next();
                                self.string_with_multiline_support();
                            }

                            Some(_) => {
                                // This is data, we just finished an empty normal string
                                return self.set_token(
                                    &Token::new(
                                        self.offset() - 1,
                                        self.offset(),
                                        TokenKind::StringLiteral,
                                    )
                                );
                            }

                            None => {
                                // End of file, string incomplete
                                return self.set_error(Error::unrecoverable(ErrorKind::UnexpectedEndOfFile));
                            }
                        }
                    }

                    Some(_) => {
                        // This is data - this is a normal string
                        return self.generic_string();
                    }

                    None => {
                        // End of file, string incomplete
                        return self.set_error(Error::unrecoverable(ErrorKind::UnexpectedEndOfFile));
                    }
                }
            }

            _ => {
                return self.set_error(Error::unrecoverable(ErrorKind::UnexpectedCurrentCharacter));
            }
        }
    }
}
