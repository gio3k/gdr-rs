use crate::{assert_peek, read};
use crate::script::Location;
use crate::stage0::ScriptLexer;
use crate::stage0::tokens::TokenKind;

const FEATURE_LONG_STRING_AMOUNT: usize = 3;
pub const FEATURE_SHORT_STRING: char = '\'';
pub const FEATURE_STRING: char = '"';


impl<'a> ScriptLexer<'a> {
    /// Read a short string literal
    fn short_string_literal(&mut self) {
        let data_start = self.offset();
        let token_start = data_start - 1;

        read! { self,
            None => {
                let data_end = self.offset();
                self.set_token_kind(TokenKind::StringLiteral)
                    .set_token_pos(Location::new(data_start, data_end))
                    .make_token_symbol()
                    .set_token_pos(Location::new(token_start, data_end));
                break;
            },
            Some('\n') => {
                // Short strings don't support newlines, abort!
                break;
            },
            Some('\'') => {
                let data_end = self.offset();
                let token_end = data_end + 1;
                self.set_token_kind(TokenKind::StringLiteral)
                    .set_token_pos(Location::new(data_start, data_end))
                    .make_token_symbol()
                    .set_token_pos(Location::new(token_start, token_end));
                break;
            },

            _ => {}
        }

        self.next();
    }

    fn generic_string_literal(&mut self) {
        let data_start = self.offset();
        let token_start = data_start - 1;

        read! { self,
            None => {
                let data_end = self.offset();
                self.set_token_kind(TokenKind::StringLiteral)
                    .set_token_pos(Location::new(data_start, data_end))
                    .make_token_symbol()
                    .set_token_pos(Location::new(token_start, data_end));
                break;
            },
            Some('"') => {
                let data_end = self.offset();
                let token_end = data_end + 1;
                self.set_token_kind(TokenKind::StringLiteral)
                    .set_token_pos(Location::new(data_start, data_end))
                    .make_token_symbol()
                    .set_token_pos(Location::new(token_start, token_end));
                break;
            },

            _ => {}
        }

        self.next();
    }

    /// Read a long string literal
    fn long_string_literal(&mut self) {
        let data_start = self.offset();
        let token_start = data_start - FEATURE_LONG_STRING_AMOUNT;

        read! { self,
            None => {
                let data_end = self.offset();
                self.set_token_kind(TokenKind::StringLiteral)
                    .set_token_pos(Location::new(data_start, data_end))
                    .make_token_symbol()
                    .set_token_pos(Location::new(token_start, data_end));
                break;
            },
            Some('"') => {
                let mut is_valid_end = true;
                for _ in 0..FEATURE_LONG_STRING_AMOUNT - 1 {
                    self.next();
                    match self.peek() {
                        Some(FEATURE_STRING) => {},
                        _ => is_valid_end = false,
                    }
                }

                if is_valid_end {
                    let token_end = self.offset();
                    let data_end = token_end - (FEATURE_LONG_STRING_AMOUNT - 1);
                    self.set_token_kind(TokenKind::StringLiteral)
                        .set_token_pos(Location::new(data_start, data_end))
                        .make_token_symbol()
                        .set_token_pos(Location::new(token_start, token_end));
                }
                break;
            },

            _ => {}
        }

        self.next();
    }

    /// Detect the string type and read it to a literal
    pub(crate) fn string_literal(&mut self) {
        assert_peek!(self, Some(FEATURE_SHORT_STRING | FEATURE_STRING));

        match self.peek() {
            Some(FEATURE_SHORT_STRING) => {
                self.next();
                self.short_string_literal();
            }

            Some(FEATURE_STRING) => {
                self.next();
                for _ in 0..FEATURE_LONG_STRING_AMOUNT - 1 {
                    match self.peek() {
                        Some(FEATURE_STRING) => {}
                        _ => {
                            self.generic_string_literal();
                            return;
                        }
                    }
                    self.next();
                }

                // Long string found
                self.long_string_literal();
                return;
            }

            _ => {
                panic!("Logic failure");
            }
        }
    }
}

#[cfg(test)]
mod lexer_tests {
    use crate::{assert_token_kind, assert_token_value};
    use crate::core::literal::Literal;
    use crate::script::Script;
    use crate::stage0::ScriptLexer;
    use crate::stage0::tokens::TokenKind;

    /// Expects 2 tokens - StringLiteral (value: hello, world!) and Identifier (value: abc)
    fn test_case_0(lexer: &mut ScriptLexer) {
        let t0 = lexer.scan()
            .expect("Token shouldn't be None");

        assert_token_kind!(t0, TokenKind::StringLiteral);
        assert_token_value!(t0, Literal::Symbol(s) if s == lexer.cache_string("hello, world!"));

        let t1 = lexer.scan()
            .expect("Token shouldn't be None");

        assert_token_kind!(t1, TokenKind::Identifier);
        assert_token_value!(t1, Literal::Symbol(s) if s == lexer.cache_string("abc"));
    }

    #[test]
    fn short_string_with_identifier_after() {
        let mut lexer = ScriptLexer::new(
            Script::new("'hello, world!'abc")
        );

        test_case_0(&mut lexer);
    }

    #[test]
    fn long_string_with_identifier_after() {
        let mut lexer = ScriptLexer::new(
            Script::new("\"\"\"hello, world!\"\"\"abc")
        );

        test_case_0(&mut lexer);
    }

    #[test]
    fn generic_string_with_identifier_after() {
        let mut lexer = ScriptLexer::new(
            Script::new("\"hello, world!\"abc")
        );

        test_case_0(&mut lexer);
    }

    #[test]
    fn generic_string_with_spaced_identifier_after() {
        let mut lexer = ScriptLexer::new(
            Script::new("\"hello, world!\" abc")
        );

        test_case_0(&mut lexer);
    }

    #[test]
    fn generic_string_with_float_after() {
        let mut lexer = ScriptLexer::new(
            Script::new("\"float >>>\" 11.01")
        );

        let t0 = lexer.scan()
            .expect("Token shouldn't be None");

        assert_token_kind!(t0, TokenKind::StringLiteral);
        assert_token_value!(t0, Literal::Symbol(s) if s == lexer.cache_string("float >>>"));

        let t1 = lexer.scan()
            .expect("Token shouldn't be None");

        assert_token_kind!(t1, TokenKind::FloatLiteral);
        assert_token_value!(t1, Literal::Float(s) if s == 11.01);
    }
}