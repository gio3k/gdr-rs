use std::num::{ParseFloatError, ParseIntError};
use crate::lexer::core::error::{Error, ErrorKind};
use crate::lexer::Lexer;
use crate::read;
use crate::lexer::core::token::{TokenKind, TokenValue};
use crate::lexer::language::characters::LO_MATH_SPACING;

impl<'a> Lexer<'a> {
    fn parse_float_from_string(&mut self, start: usize, end: usize) -> Result<f64, ParseFloatError> {
        let string_no_underscores: String = self.slice_to_string(start, end)
            .chars()
            .filter(|c| *c != '_')
            .collect();

        match string_no_underscores.parse::<f64>() {
            Ok(v) => Ok(v),
            Err(e) => Err(e)
        }
    }

    fn parse_int_from_string(&mut self, start: usize, end: usize) -> Result<i64, ParseIntError> {
        let string_no_underscores: String = self.slice_to_string(start, end)
            .chars()
            .filter(|c| *c != '_')
            .collect();

        match string_no_underscores.parse::<i64>() {
            Ok(v) => Ok(v),
            Err(e) => Err(e)
        }
    }

    pub fn negative_number_literal(&mut self) -> &mut Self {
        self.number_literal(true);
        self
    }

    pub fn positive_number_literal(&mut self) -> &mut Self {
        self.number_literal(false);
        self
    }

    /// Parses a number literal
    /// Assumes the iterator is on a number or a negative (minus) character
    fn number_literal(&mut self, is_negative: bool) {
        let start = self.offset();
        let mut is_float: bool = false;

        // Find end of number
        read! { self,
            Some('0'..='9') => {},
            Some('e' | 'E' | '.') => {
                is_float = true;
            },
            Some(LO_MATH_SPACING) => {},
            _ => break
        }

        // Read the number as a string
        let end = self.offset();

        if is_float {
            match self.parse_float_from_string(start, end) {
                Ok(v) => {
                    self.set_token_kind(TokenKind::FloatLiteral)
                        .set_token_pos(start, end)
                        .set_token_value(TokenValue::Float(
                            if is_negative { -v } else { v }
                        ));
                }
                Err(e) => self.set_error(Error::recoverable(
                    ErrorKind::FloatParseFailure(e), 1,
                ))
            }
        } else {
            match self.parse_int_from_string(start, end) {
                Ok(v) => {
                    self.set_token_kind(TokenKind::IntegerLiteral)
                        .set_token_pos(start, end)
                        .set_token_value(TokenValue::Integer(
                            if is_negative { -v } else { v }
                        ));
                }
                Err(e) => self.set_error(Error::recoverable(
                    ErrorKind::IntegerParseFailure(e), 1,
                ))
            }
        };
    }
}