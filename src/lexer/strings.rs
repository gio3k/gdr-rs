use crate::error_here;
use crate::lexer::{Lexer, LexerError};
use crate::lexer::tokens::TokenKind;

impl<'a> Lexer<'a> {
    pub(crate) fn parse_string_with_size_checked(&mut self) -> Result<(), LexerError> {
        let mut is_long_string: bool = true;

        // Skip the first character so we don't instantly stop
        self.advance();

        // We need to figure out if the string is a long string (""")
        // Save the current iterator state
        self.save();
        // Check if the next 2 characters are also quotes
        for _n in 0..2 {
            if let Some(c) = self.see() {
                if c != '"' {
                    // We found a non-quote character - it's not a long string!
                    is_long_string = false;
                }
            }
            self.advance();
        }
        self.restore();

        if is_long_string {
            self.advance_by(2);
        }

        let start = self.offset();

        // Let's find the end of the string too
        loop {
            match self.see() {
                Some('"') => {
                    if !is_long_string {
                        // We're not a long string - just return
                        self.insert_string_filled_token_here(start, TokenKind::StringLiteral)?;
                        self.advance();
                        return Ok(());
                    }

                    let end = self.offset();

                    // We're a long string - let's see if this is the end
                    let mut is_suitable_long_string_end = true;
                    for _ in 0..3 {
                        if let Some(c) = self.see() {
                            if c != '"' {
                                is_suitable_long_string_end = false;
                            }
                        }
                        self.advance();
                    }

                    if is_suitable_long_string_end {
                        return self.insert_string_token(start, end, TokenKind::StringLiteral);
                    }

                    self.advance();
                }
                None => return error_here!(self, StringNotTerminated),
                _ => {
                    self.advance();
                }
            }
        }
    }

    pub(crate) fn parse_string_single_quote_small(&mut self) -> Result<(), LexerError> {
        // Skip the first character so we don't instantly stop
        self.advance();

        let start = self.offset();

        loop {
            match self.see() {
                Some('\'') => {
                    self.insert_string_filled_token_here(start, TokenKind::StringLiteral)?;
                    self.advance();
                    return Ok(());
                }
                Some('\n') | None => return error_here!(self, StringNotTerminated),
                _ => {
                    self.advance();
                }
            }
        }
    }
}