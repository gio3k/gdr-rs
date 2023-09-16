use crate::{assert_peek, read};
use crate::core::literal::Literal;
use crate::script::Location;
use crate::stage0::ScriptLexer;
use crate::stage0::tokens::TokenKind;

impl<'a> ScriptLexer<'a> {
    /// Parses a string based identifier / keyword
    /// Assumes the iterator is on a comment start character (#)
    pub fn named_item(&mut self) {
        let start = self.offset();

        assert_peek!(self, Some(c) if is_valid_start_for_identifier(c));

        read! { self,
            Some(c) if !is_valid_body_for_identifier(c) => {
                break;
            },
            None => break,
            _ => {}
        }

        let end = self.offset();

        if start == end {
            // This shouldn't be possible unless something's really wrong
            // is_valid_body_for_identifier or is_valid_start_for_identifier are mismatched or wrong
            panic!("Named item character mismatch - something is wrong with is_valid_*_for_identifier");
        }

        let location = Location::new(start, end);

        // Prepare token
        self.set_token_pos(location);

        // Turn into a string
        let word = self.script.slice_to_string(location);
        match word.as_str() {
            "var" => {
                self.set_token_kind(TokenKind::Var);
            }
            "const" => {
                self.set_token_kind(TokenKind::Const);
            }

            "preload" => {
                self.set_token_kind(TokenKind::Preload);
            }

            "func" => {
                self.set_token_kind(TokenKind::Function);
            }
            "return" => {
                self.set_token_kind(TokenKind::Return);
            }

            "for" => {
                self.set_token_kind(TokenKind::For);
            }
            "while" => {
                self.set_token_kind(TokenKind::While);
            }
            "in" => {
                self.set_token_kind(TokenKind::In);
            }

            "if" => {
                self.set_token_kind(TokenKind::If);
            }
            "match" => {
                self.set_token_kind(TokenKind::Match);
            }
            "and" => {
                self.set_token_kind(TokenKind::ComparisonAnd);
            }
            "not" => {
                self.set_token_kind(TokenKind::Not);
            }
            "or" => {
                self.set_token_kind(TokenKind::ComparisonOr);
            }
            "else" => {
                self.set_token_kind(TokenKind::Else);
            }
            "elif" => {
                self.set_token_kind(TokenKind::ElseIf);
            }

            "null" => {
                self.set_token_kind(TokenKind::NullLiteral);
            }
            "false" => {
                self.set_token_kind(TokenKind::BooleanLiteral)
                    .set_token_value(Literal::Boolean(false));
            }
            "true" => {
                self.set_token_kind(TokenKind::BooleanLiteral)
                    .set_token_value(Literal::Boolean(true));
            }

            _ => {
                self.set_token_kind(TokenKind::Identifier)
                    .make_token_symbol();
            }
        }
    }
}

pub fn is_valid_character_for_identifier(c: char) -> bool {
    match c {
        ':' => false,
        ',' => false,
        '(' | ')' => false,
        '[' | ']' => false,
        '{' | '}' => false,
        '<' | '>' | '+' | '-' | '/' | '%' | '^' | '$' | '*' | '@' | '!' | '\\' | '=' => false,
        '.' => false,
        '\r' | '\n' | '\'' | '"' => false,
        _ => true
    }
}

pub fn is_valid_start_for_identifier(c: char) -> bool {
    match c {
        '0'..='9' => false, // Don't allow numbers
        c if !is_valid_character_for_identifier(c) => false,
        _ => true
    }
}

pub fn is_valid_body_for_identifier(c: char) -> bool {
    match c {
        ' ' => false, // Don't allow spaces
        c if !is_valid_character_for_identifier(c) => false,
        _ => true
    }
}
