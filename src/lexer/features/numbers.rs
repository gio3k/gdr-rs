use std::num::{ParseFloatError, ParseIntError};
use crate::lexer::ScriptLexer;
use crate::{read, ScriptLocation};
use crate::lexer::token::TokenKind;
use crate::literals::Literal;

impl<'a> ScriptLexer<'a> {
    fn parse_float_from_string(&mut self, location: ScriptLocation) -> Result<f64, ParseFloatError> {
        let string_no_underscores: String = self.script.slice_to_string(location)
            .chars()
            .filter(|c| *c != '_')
            .collect();

        match string_no_underscores.parse::<f64>() {
            Ok(v) => Ok(v),
            Err(e) => Err(e)
        }
    }

    fn parse_int_from_string(&mut self, location: ScriptLocation) -> Result<i64, ParseIntError> {
        let string_no_underscores: String = self.script.slice_to_string(location)
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
            Some('_') => {},
            _ => break
        }

        // Read the number as a string
        let end = self.offset();
        let location = ScriptLocation::new(start, end);

        if is_float {
            match self.parse_float_from_string(location) {
                Ok(v) => {
                    self.set_token_kind(TokenKind::FloatLiteral)
                        .set_token_pos(location)
                        .set_token_value(Literal::Float(
                            if is_negative { -v } else { v }
                        ));
                }
                Err(e) => {
                    println!("Got parse error {:?} parsing token", e);
                    self.set_token_kind(TokenKind::Unknown)
                        .set_token_pos(location);
                }
            }
        } else {
            match self.parse_int_from_string(location) {
                Ok(v) => {
                    self.set_token_kind(TokenKind::IntegerLiteral)
                        .set_token_pos(location)
                        .set_token_value(Literal::Integer(
                            if is_negative { -v } else { v }
                        ));
                }
                Err(e) => {
                    println!("Got parse error {:?} parsing token", e);
                    self.set_token_kind(TokenKind::Unknown)
                        .set_token_pos(location);
                }
            }
        };
    }
}

#[cfg(test)]
mod lexer_tests {
    use crate::{assert_token_kind, assert_token_value, Script};
    use crate::lexer::token::TokenKind;
    use crate::lexer::ScriptLexer;
    use crate::literals::Literal;

    #[test]
    fn float() {
        let mut lexer = ScriptLexer::new(
            Script::new("123.03")
        );

        let t0 = lexer.parse()
            .expect("Token shouldn't be None");
        assert_token_kind!(t0, TokenKind::FloatLiteral);
        assert_token_value!(t0, Literal::Float(v) if v == 123.03);
    }

    #[test]
    fn integer() {
        let mut lexer = ScriptLexer::new(
            Script::new("123")
        );

        let t0 = lexer.parse()
            .expect("Token shouldn't be None");
        assert_token_kind!(t0, TokenKind::IntegerLiteral);
        assert_token_value!(t0, Literal::Integer(123));
    }
}