use crate::lexer::core::error::{Error, ErrorKind};
use crate::lexer::Lexer;
use crate::{read, set_error_unless};
use crate::lexer::core::token::{TokenKind, TokenValue};
use crate::lexer::language::characters::{LC_CLOSE_CURLY_BRACKET, LC_CLOSE_ROUND_BRACKET, LC_CLOSE_SQUARE_BRACKET, LC_COLON, LC_COMMA, LC_OPEN_CURLY_BRACKET, LC_OPEN_ROUND_BRACKET, LC_OPEN_SQUARE_BRACKET};
use crate::lexer::language::keywords::{LKW_BRANCH_AND, LKW_BRANCH_ELSE, LKW_BRANCH_ELSE_IF, LKW_BRANCH_IF, LKW_BRANCH_MATCH, LKW_BRANCH_NOT, LKW_BRANCH_OR, LKW_CONSTANT, LKW_FUNCTION_RETURN, LKW_FUNCTION_STATEMENT, LKW_GD_PRELOAD, LKW_LITERAL_BOOLEAN_FALSE, LKW_LITERAL_BOOLEAN_TRUE, LKW_LITERAL_NULL, LKW_LOOP_FOR, LKW_LOOP_IN, LKW_LOOP_WHILE, LKW_VARIABLE};

impl<'a> Lexer<'a> {
    /// Parses a string based identifier / keyword
    /// Assumes the iterator is on a comment start character (#)
    pub fn named_item(&mut self) {
        let start = self.offset();

        set_error_unless!(
            self, Error::unrecoverable(ErrorKind::UnexpectedCurrentCharacter),
            Some(c) if is_valid_start_for_identifier(c)
        );

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
            self.set_error(
                Error::unrecoverable(ErrorKind::NamedItemCharacterIdentificationMismatch)
            );
        }

        // Prepare token
        self.set_token_pos(start, end);

        // Turn into a string
        let word = self.slice_to_string(start, end);
        match word.as_str() {
            LKW_VARIABLE => {
                self.set_token_kind(TokenKind::Var);
            }
            LKW_CONSTANT => {
                self.set_token_kind(TokenKind::Const);
            }

            LKW_GD_PRELOAD => {
                self.set_token_kind(TokenKind::LanguagePreload);
            }

            LKW_FUNCTION_STATEMENT => {
                self.set_token_kind(TokenKind::Function);
            }
            LKW_FUNCTION_RETURN => {
                self.set_token_kind(TokenKind::Return);
            }

            LKW_LOOP_FOR => {
                self.set_token_kind(TokenKind::For);
            }
            LKW_LOOP_WHILE => {
                self.set_token_kind(TokenKind::While);
            }
            LKW_LOOP_IN => {
                self.set_token_kind(TokenKind::In);
            }

            LKW_BRANCH_IF => {
                self.set_token_kind(TokenKind::If);
            }
            LKW_BRANCH_MATCH => {
                self.set_token_kind(TokenKind::Match);
            }
            LKW_BRANCH_AND => {
                self.set_token_kind(TokenKind::ComparisonAnd);
            }
            LKW_BRANCH_NOT => {
                self.set_token_kind(TokenKind::Not);
            }
            LKW_BRANCH_OR => {
                self.set_token_kind(TokenKind::ComparisonOr);
            }
            LKW_BRANCH_ELSE => {
                self.set_token_kind(TokenKind::Else);
            }
            LKW_BRANCH_ELSE_IF => {
                self.set_token_kind(TokenKind::ElseIf);
            }

            LKW_LITERAL_NULL => {
                self.set_token_kind(TokenKind::NullLiteral);
            }
            LKW_LITERAL_BOOLEAN_FALSE => {
                self.set_token_kind(TokenKind::BooleanLiteral)
                    .set_token_value(TokenValue::Boolean(false));
            }
            LKW_LITERAL_BOOLEAN_TRUE => {
                self.set_token_kind(TokenKind::BooleanLiteral)
                    .set_token_value(TokenValue::Boolean(true));
            }

            _ => {
                self.set_token_kind(TokenKind::Identifier)
                    .make_token_value_string();
            }
        }
    }
}

pub fn is_valid_character_for_identifier(c: char) -> bool {
    match c {
        LC_COLON => false,
        LC_COMMA => false,
        LC_OPEN_ROUND_BRACKET | LC_CLOSE_ROUND_BRACKET => false,
        LC_OPEN_SQUARE_BRACKET | LC_CLOSE_SQUARE_BRACKET => false,
        LC_OPEN_CURLY_BRACKET | LC_CLOSE_CURLY_BRACKET => false,
        '<' | '>' | '+' | '-' | '/' | '%' | '^' | '$' | '*' | '@' | '!' | '\\' | '=' => false,
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
