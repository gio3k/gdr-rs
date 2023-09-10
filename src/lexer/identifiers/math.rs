use crate::error_here;
use crate::lexer::identifiers::is_valid_identifier_start;
use crate::lexer::{Lexer, LexerError};
use crate::lexer::tokens::TokenKind;

impl<'a> Lexer<'a> {
    /// Figure out what to do with a add token
    pub(crate) fn qualify_math_add(&mut self) -> Result<(), LexerError> {
        match self.advance_and_see() {
            Some('+') => {
                match self.advance_and_see() {
                    Some(c) if is_valid_identifier_start(c) => self.put_token_and_advance(2, TokenKind::MathIncrement),
                    _ => error_here!(self, UnexpectedCharacterInIdentifier)
                }
            }

            Some('=') => {
                match self.advance_and_see() {
                    Some(c) if is_valid_identifier_start(c) => self.put_token_and_advance(2, TokenKind::MathTargetedAdd),
                    _ => error_here!(self, UnexpectedCharacterInIdentifier)
                }
            }

            Some(c) if is_valid_identifier_start(c) => self.put_single_token_and_advance(TokenKind::MathAdd),

            _ => error_here!(self, UnexpectedCharacterInIdentifier)
        }
    }

    /// Figure out what to do with a subtract token
    pub(crate) fn qualify_math_subtract(&mut self) -> Result<(), LexerError> {
        match self.advance_and_see() {
            Some('-') => {
                match self.advance_and_see() {
                    Some(c) if is_valid_identifier_start(c) => self.put_token_and_advance(2, TokenKind::MathDecrement),
                    _ => error_here!(self, UnexpectedCharacterInIdentifier)
                }
            }

            Some('=') => {
                match self.advance_and_see() {
                    Some(c) if is_valid_identifier_start(c) => self.put_token_and_advance(2, TokenKind::MathTargetedSubtract),
                    _ => error_here!(self, UnexpectedCharacterInIdentifier)
                }
            }

            Some(c) if c.is_ascii_digit() => {
                // Read as negative number literal
                self.identify_number(true)
            }

            Some(c) if is_valid_identifier_start(c) => self.put_single_token_and_advance(TokenKind::MathSubtract),

            _ => error_here!(self, UnexpectedCharacterInIdentifier)
        }
    }

    /// Figure out what to do with a divide token
    pub(crate) fn qualify_math_divide(&mut self) -> Result<(), LexerError> {
        match self.advance_and_see() {
            Some('=') => {
                match self.advance_and_see() {
                    Some(c) if is_valid_identifier_start(c) => self.put_token_and_advance(2, TokenKind::MathTargetedDivide),
                    _ => error_here!(self, UnexpectedCharacterInIdentifier)
                }
            }

            Some(c) if is_valid_identifier_start(c) => self.put_single_token_and_advance(TokenKind::MathDivide),

            _ => error_here!(self, UnexpectedCharacterInIdentifier)
        }
    }

    /// Figure out what to do with a multiply token
    pub(crate) fn qualify_math_multiply(&mut self) -> Result<(), LexerError> {
        match self.advance_and_see() {
            Some('=') => {
                match self.advance_and_see() {
                    Some(c) if is_valid_identifier_start(c) => self.put_token_and_advance(2, TokenKind::MathTargetedMultiply),
                    _ => error_here!(self, UnexpectedCharacterInIdentifier)
                }
            }

            Some(c) if is_valid_identifier_start(c) => self.put_single_token_and_advance(TokenKind::MathMultiply),

            _ => error_here!(self, UnexpectedCharacterInIdentifier)
        }
    }
}