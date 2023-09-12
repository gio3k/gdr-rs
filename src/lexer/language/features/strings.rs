use crate::lexer::core::error::{Error, ErrorKind};
use crate::lexer::Lexer;
use crate::{read, set_error_unless, set_error_when};
use crate::lexer::core::token::{Token, TokenKind, TokenValue};

pub const FEATURE_LONG_STRING: char = '"';
const FEATURE_LONG_STRING_TOKEN_OFFSET: usize = 3;
pub const FEATURE_SHORT_STRING: char = '\'';
pub const FEATURE_STRING: char = '"';


impl<'a> Lexer<'a> {
    /// Parses a long string (""")
    /// Assumes the iterator is on the first character of text
    fn string_with_multiline_support(&mut self) {
        set_error_when!(
            self, Error::unrecoverable(ErrorKind::UnexpectedCurrentCharacter),
            Some(FEATURE_SHORT_STRING | FEATURE_LONG_STRING)
        );

        let data_start = self.offset();
        let token_start = data_start - FEATURE_LONG_STRING_TOKEN_OFFSET;

        read! { self,
            Some('\n') => {
                return self.set_error(Error::recoverable(ErrorKind::UnexpectedLineBreak, 1));
            },

            Some(FEATURE_LONG_STRING) => {
                self.next();

                match self.peek() {
                    Some(FEATURE_LONG_STRING) => {
                        self.next();

                        match self.peek() {
                            Some(FEATURE_LONG_STRING) => {
                                // Long string end complete
                                let data_end = self.offset();
                                let token_end = data_end + FEATURE_LONG_STRING_TOKEN_OFFSET;

                                self.set_token_kind(TokenKind::StringLiteral)
                                    .set_token_pos(data_start, data_end)
                                    .make_token_value_string()
                                    .set_token_pos(token_start, token_end);
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
            Some(FEATURE_SHORT_STRING | FEATURE_STRING)
        );

        let data_start = self.offset();
        let token_start = data_start - 1;

        read! { self,
            Some('\n') => {
                return self.set_error(Error::recoverable(ErrorKind::UnexpectedLineBreak, 1));
            },

            Some(FEATURE_SHORT_STRING) => {
                let data_end = self.offset();
                let token_end = data_end + 1;
                self.set_token_kind(TokenKind::StringLiteral)
                    .set_token_pos(data_start, data_end)
                    .make_token_value_string()
                    .set_token_pos(token_start, token_end);
            },

            _ => {}
        }
    }

    /// Parses a string (")
    /// Assumes the iterator is on the first character of text
    fn generic_string(&mut self) {
        set_error_when!(
            self, Error::unrecoverable(ErrorKind::UnexpectedCurrentCharacter),
            Some(FEATURE_SHORT_STRING | FEATURE_STRING)
        );

        let data_start = self.offset();
        let token_start = data_start - 1;

        read! { self,
             Some(FEATURE_STRING) => {
                let data_end = self.offset();
                let token_end = data_end + 1;
                self.set_token_kind(TokenKind::StringLiteral)
                    .set_token_pos(data_start, data_end)
                    .make_token_value_string()
                    .set_token_pos(token_start, token_end);
            },

            _ => {}
        }
    }

    /// Parses a string, automatically detecting the type
    /// Assumes the iterator is on a valid string starter
    pub fn string_literal(&mut self) {
        set_error_unless!(
            self, Error::unrecoverable(ErrorKind::UnexpectedCurrentCharacter),
            Some(FEATURE_SHORT_STRING | FEATURE_STRING)
        );

        match self.peek() {
            Some(FEATURE_SHORT_STRING) => {
                self.next();
                self.short_string();
            }

            Some(FEATURE_STRING) => {
                self.next();

                // Let's see if this is a long string or normal string
                match self.peek() {
                    Some(FEATURE_STRING) => {
                        // This is either the end of a normal string or the 2nd character of a long string
                        match self.peek() {
                            Some(FEATURE_STRING) => {
                                // Another quote, this is a long string
                                self.next();
                                self.string_with_multiline_support();
                            }

                            Some(_) => {
                                // This is data, we just finished an empty normal string
                                self.set_token_kind(TokenKind::StringLiteral)
                                    .set_token_value(TokenValue::None)
                                    .single_token_here();
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
