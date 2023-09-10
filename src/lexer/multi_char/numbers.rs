use crate::error_here;
use crate::lexer::{Lexer, LexerError};
use crate::lexer::tokens::{TokenKind, TokenValue};

impl<'a> Lexer<'a> {
    fn parse_float_from_string(&mut self, string: &str) -> Result<f64, LexerError> {
        let string_no_underscores: String = string.chars().filter(|c| *c != '_').collect();

        match string_no_underscores.parse::<f64>() {
            Ok(v) => Ok(v),
            Err(e) => error_here!(self, FloatLiteralParseError, e)
        }
    }

    fn parse_int_from_string(&mut self, string: &str) -> Result<i64, LexerError> {
        let string_no_underscores: String = string.chars().filter(|c| *c != '_').collect();

        match string_no_underscores.parse::<i64>() {
            Ok(v) => Ok(v),
            Err(e) => error_here!(self, IntLiteralParseError, e)
        }
    }


    /// Parse a number (assuming the current character is the start of it)
    pub(crate) fn identify_number(&mut self, is_negative: bool) -> Result<(), LexerError> {
        let start = self.offset();
        let mut is_float: bool = false;

        // Find end of number
        loop {
            match self.see() {
                Some('0'..='9') => {}
                Some('e' | 'E' | '.') => {
                    is_float = true;
                }
                _ => break
            }
            self.advance();
        }

        let end = self.offset();
        let string: String = self.view(start, end).collect();

        return if is_float {
            let mut number = self.parse_float_from_string(string.as_str())?;
            if is_negative {
                number = -number;
            }
            self.insert_token_data(start, end, TokenKind::FloatLiteral, TokenValue::Float(number))
        } else {
            let mut number = self.parse_int_from_string(string.as_str())?;
            if is_negative {
                number = -number;
            }
            self.insert_token_data(start, end, TokenKind::IntegerLiteral, TokenValue::Integer(number))
        };
    }

    /// Parse a number, automatically detecting whether or not it's negative
    pub(crate) fn identify_number_signed(&mut self) -> Result<(), LexerError> {
        let mut is_negative: bool = false;

        match self.see() {
            Some('-') => {
                is_negative = true;
            }
            Some('0'..='9') => {}
            _ => {
                return Err(LexerError::NumberLiteralInvalid(10102301023));
            }
        }

        self.identify_number(is_negative)
    }
}