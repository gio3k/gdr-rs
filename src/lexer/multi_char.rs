mod numbers;
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
        '~' => false,
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
    pub(crate) fn identify_multi_character(&mut self) -> Result<(), LexerError> {
        match self.see() {
            Some('>') => match self.advance_and_see() {
                // >
                Some('>') => {
                    // >>
                    match self.advance_and_see() {
                        Some(c) if is_valid_identifier_start(c) => self.put_token_and_advance(2, TokenKind::BitwiseRightShift),
                        _ => error_here!(self, UnexpectedCharacterInIdentifier)
                    }
                }

                Some('=') => {
                    // >=
                    match self.advance_and_see() {
                        Some(c) if is_valid_identifier_start(c) => self.put_token_and_advance(2, TokenKind::ComparisonGreaterThanOrEqualTo),
                        _ => error_here!(self, UnexpectedCharacterInIdentifier)
                    }
                }

                Some(c) if is_valid_identifier_start(c) => self.put_single_token_and_advance(TokenKind::ComparisonGreaterThan),

                _ => error_here!(self, UnexpectedCharacterInIdentifier)
            }
            Some('<') => match self.advance_and_see() {
                // <
                Some('<') => {
                    // <<
                    match self.advance_and_see() {
                        Some(c) if is_valid_identifier_start(c) => self.put_token_and_advance(2, TokenKind::BitwiseLeftShift),
                        _ => error_here!(self, UnexpectedCharacterInIdentifier)
                    }
                }

                Some('=') => {
                    // <=
                    match self.advance_and_see() {
                        Some(c) if is_valid_identifier_start(c) => self.put_token_and_advance(2, TokenKind::ComparisonLesserThanOrEqualTo),
                        _ => error_here!(self, UnexpectedCharacterInIdentifier)
                    }
                }

                Some(c) if is_valid_identifier_start(c) => self.put_single_token_and_advance(TokenKind::ComparisonLesserThan),

                _ => error_here!(self, UnexpectedCharacterInIdentifier)
            }

            Some('=') => match self.advance_and_see() {
                // =
                Some('=') => {
                    // ==
                    match self.advance_and_see() {
                        Some(c) if is_valid_identifier_start(c) => self.put_token_and_advance(2, TokenKind::ComparisonEqualTo),
                        _ => error_here!(self, UnexpectedCharacterInIdentifier)
                    }
                }

                Some(c) if is_valid_identifier_start(c) => self.put_single_token_and_advance(TokenKind::Assignment),

                _ => error_here!(self, UnexpectedCharacterInIdentifier)
            }
            Some('!') => match self.advance_and_see() {
                // !
                Some('=') => {
                    // !=
                    match self.advance_and_see() {
                        Some(c) if is_valid_identifier_start(c) => self.put_token_and_advance(2, TokenKind::ComparisonNotEqualTo),
                        _ => error_here!(self, UnexpectedCharacterInIdentifier)
                    }
                }

                Some(c) if is_valid_identifier_start(c) => self.put_single_token_and_advance(TokenKind::NegateExpression),

                _ => error_here!(self, UnexpectedCharacterInIdentifier)
            }

            Some('~') => match self.advance_and_see() {
                // ~
                Some('=') => {
                    // ~=
                    match self.advance_and_see() {
                        Some(c) if is_valid_identifier_start(c) => self.put_token_and_advance(2, TokenKind::BitwiseTargetedNot),
                        _ => error_here!(self, UnexpectedCharacterInIdentifier)
                    }
                }

                Some(c) if is_valid_identifier_start(c) => self.put_single_token_and_advance(TokenKind::BitwiseNot),

                _ => error_here!(self, UnexpectedCharacterInIdentifier)
            }
            Some('&') => match self.advance_and_see() {
                // &
                Some('&') => {
                    // &&
                    match self.advance_and_see() {
                        Some(c) if is_valid_identifier_start(c) => self.put_token_and_advance(2, TokenKind::ComparisonAnd),
                        _ => error_here!(self, UnexpectedCharacterInIdentifier)
                    }
                }

                Some('=') => {
                    // &=
                    match self.advance_and_see() {
                        Some(c) if is_valid_identifier_start(c) => self.put_token_and_advance(2, TokenKind::BitwiseTargetedAnd),
                        _ => error_here!(self, UnexpectedCharacterInIdentifier)
                    }
                }

                Some(c) if is_valid_identifier_start(c) => self.put_single_token_and_advance(TokenKind::BitwiseAnd),

                _ => error_here!(self, UnexpectedCharacterInIdentifier)
            }
            Some('|') => match self.advance_and_see() {
                // |
                Some('|') => {
                    // ||
                    match self.advance_and_see() {
                        Some(c) if is_valid_identifier_start(c) => self.put_token_and_advance(2, TokenKind::ComparisonOr),
                        _ => error_here!(self, UnexpectedCharacterInIdentifier)
                    }
                }

                Some('=') => {
                    // |=
                    match self.advance_and_see() {
                        Some(c) if is_valid_identifier_start(c) => self.put_token_and_advance(2, TokenKind::BitwiseTargetedOr),
                        _ => error_here!(self, UnexpectedCharacterInIdentifier)
                    }
                }

                Some(c) if is_valid_identifier_start(c) => self.put_single_token_and_advance(TokenKind::BitwiseOr),

                _ => error_here!(self, UnexpectedCharacterInIdentifier)
            }
            Some('^') => match self.advance_and_see() {
                // ^
                Some('=') => {
                    // ^=
                    match self.advance_and_see() {
                        Some(c) if is_valid_identifier_start(c) => self.put_token_and_advance(2, TokenKind::BitwiseTargetedXor),
                        _ => error_here!(self, UnexpectedCharacterInIdentifier)
                    }
                }

                Some(c) if is_valid_identifier_start(c) => self.put_single_token_and_advance(TokenKind::BitwiseXor),

                _ => error_here!(self, UnexpectedCharacterInIdentifier)
            }

            Some('+') => match self.advance_and_see() {
                // +
                Some('+') => {
                    // ++
                    match self.advance_and_see() {
                        Some(c) if is_valid_identifier_start(c) => self.put_token_and_advance(2, TokenKind::MathIncrement),
                        _ => error_here!(self, UnexpectedCharacterInIdentifier)
                    }
                }

                Some('=') => {
                    // +=
                    match self.advance_and_see() {
                        Some(c) if is_valid_identifier_start(c) => self.put_token_and_advance(2, TokenKind::MathTargetedAdd),
                        _ => error_here!(self, UnexpectedCharacterInIdentifier)
                    }
                }

                Some(c) if is_valid_identifier_start(c) => self.put_single_token_and_advance(TokenKind::MathAdd),

                _ => error_here!(self, UnexpectedCharacterInIdentifier)
            },
            Some('-') => match self.advance_and_see() {
                // -
                Some('-') => {
                    // --
                    match self.advance_and_see() {
                        Some(c) if is_valid_identifier_start(c) => self.put_token_and_advance(2, TokenKind::MathDecrement),
                        _ => error_here!(self, UnexpectedCharacterInIdentifier)
                    }
                }

                Some('=') => {
                    // -=
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
            Some('/') => match self.advance_and_see() {
                // /
                Some('=') => {
                    // /=
                    match self.advance_and_see() {
                        Some(c) if is_valid_identifier_start(c) => self.put_token_and_advance(2, TokenKind::MathTargetedDivide),
                        _ => error_here!(self, UnexpectedCharacterInIdentifier)
                    }
                }

                Some(c) if is_valid_identifier_start(c) => self.put_single_token_and_advance(TokenKind::MathDivide),

                _ => error_here!(self, UnexpectedCharacterInIdentifier)
            }
            Some('*') => match self.advance_and_see() {
                // *
                Some('=') => {
                    // *=
                    match self.advance_and_see() {
                        Some(c) if is_valid_identifier_start(c) => self.put_token_and_advance(2, TokenKind::MathTargetedMultiply),
                        _ => error_here!(self, UnexpectedCharacterInIdentifier)
                    }
                }

                Some(c) if is_valid_identifier_start(c) => self.put_single_token_and_advance(TokenKind::MathMultiply),

                _ => error_here!(self, UnexpectedCharacterInIdentifier)
            }

            Some('%') => match self.advance_and_see() {
                // %
                Some('=') => {
                    // %=
                    match self.advance_and_see() {
                        Some(c) if is_valid_identifier_start(c) => self.put_token_and_advance(2, TokenKind::MathTargetedModulo),
                        _ => error_here!(self, UnexpectedCharacterInIdentifier)
                    }
                }

                Some(c) if is_valid_identifier_start(c) => self.put_single_token_and_advance(TokenKind::MathModulo),

                _ => error_here!(self, UnexpectedCharacterInIdentifier)
            }

            // Positive number handler
            Some('0'..='9') => self.identify_number(false),

            // Multi character named identifier
            Some(c) if is_valid_identifier_start(c) => self.identify_multi_char_named(),
            _ => {
                println!("unhandled char {:?} @ {}", self.see(), self.offset());
                self.advance();
                Ok(())
            }
        }
    }
}