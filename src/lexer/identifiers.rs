mod numbers;
mod math;
mod named;

use crate::error_here;
use crate::lexer::{Lexer, LexerError};
use crate::lexer::tokens::TokenKind;

fn is_valid_identifier_character(c: char) -> bool {
    match c {
        '(' | ')' => false,
        '&' | '|' => false,
        '{' | '}' | '[' | ']' => false,
        '=' => false,
        '+' | '-' | '/' | '%' | '^' | '$' | '*' | '@' | '!' | '\\' => false,
        ',' => false,
        '<' | '>' => false,
        ':' => false,
        '#' => false,
        '.' => false,
        '"' | '\'' => false,
        '\r' | '\n' => false,
        _ => true
    }
}

fn is_valid_identifier_start(c: char) -> bool {
    match c {
        '0'..='9' => false, // Don't allow numbers
        c if !is_valid_identifier_character(c) => false,
        _ => true
    }
}

fn is_valid_identifier_body(c: char) -> bool {
    match c {
        ' ' => false, // Don't allow spaces
        c if !is_valid_identifier_character(c) => false,
        _ => true
    }
}

impl<'a> Lexer<'a> {
    fn identify_double_character(&mut self, k1: TokenKind, c2: char, k2: TokenKind) -> Result<(), LexerError> {
        match self.advance_and_see() {
            Some(x) if x == c2 => {
                match self.advance_and_see() {
                    Some(x1) if is_valid_identifier_start(x1) => {
                        // (k1)(k2)
                        self.put_token_and_advance(2, k2)
                    }

                    // (k1)(k2)(?)
                    _ => error_here!(self, UnexpectedCharacterInIdentifier)
                }
            }

            Some(x) if is_valid_identifier_start(x) => {
                self.put_single_token_and_advance(k1)
            }

            // (k1)?
            _ => error_here!(self, UnexpectedCharacterInIdentifier)
        }
    }

    pub(crate) fn identify_multi_character(&mut self) -> Result<(), LexerError> {
        match self.see() {
            Some('>') => self.identify_double_character(
                TokenKind::ComparisonGreaterThan,
                '=',
                TokenKind::ComparisonGreaterThanOrEqualTo,
            ),
            Some('<') => self.identify_double_character(
                TokenKind::ComparisonLesserThan,
                '=',
                TokenKind::ComparisonLesserThanOrEqualTo,
            ),
            Some('=') => self.identify_double_character(
                TokenKind::Assignment,
                '=',
                TokenKind::ComparisonEqualTo,
            ),
            Some('!') => self.identify_double_character(
                TokenKind::NegateExpression,
                '=',
                TokenKind::ComparisonNotEqualTo,
            ),
            Some('+') => self.qualify_math_add(),
            Some('-') => self.qualify_math_subtract(),
            Some('/') => self.qualify_math_divide(),
            Some('*') => self.qualify_math_multiply(),
            Some('0'..='9') => self.identify_number(false),
            Some(c) if is_valid_identifier_start(c) => self.identify_multi_char_named(),
            _ => {
                println!("unhandled char {:?} @ {}", self.see(), self.offset());
                self.advance();
                Ok(())
            }
        }
    }
}