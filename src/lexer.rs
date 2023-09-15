use std::str::Chars;
use string_interner::backend::StringBackend;
use string_interner::StringInterner;
use string_interner::symbol::SymbolU32;
use crate::lexer::token::{Token, TokenKind};
use crate::lexer::features::annotations::FEATURE_ANNOTATION;
use crate::lexer::features::comments::FEATURE_COMMENT;
use crate::lexer::features::identifiers::is_valid_start_for_identifier;
use crate::lexer::features::strings::{FEATURE_SHORT_STRING, FEATURE_STRING};
use crate::Script;

pub mod token;
pub(crate) mod reading;
pub(crate) mod features;
pub mod interning;

pub struct ScriptLexer<'a> {
    /// The script being read
    script: Script<'a>,

    /// String interner
    string_interner: StringInterner<StringBackend<SymbolU32>>,

    // Current state, etc...
    /// Current token after last processing iteration
    current_token: Token,

    /// Current iterator after last processing iteration
    current_iterator: Chars<'a>,

    indents_handled_for_current_line: bool,
    newline_handled_for_current_line: bool,

    /// Current line number, starting from 0
    line_number: usize,

    /// Offset / location of the current line
    line_offset: usize,
}

macro_rules! multi_char_match {
    ($self:ident, $token:ident, $token_size:literal, $($pattern:pat $(if $guard:expr)* => $action:expr),*) => {
        $self.next();
        match $self.peek() {
            $($pattern $(if $guard)* => $action),*
            None => {
                // EOF - complete this token
                $self.set_token_kind(TokenKind::$token)
                    .end_token_here_with_size($token_size);
            }

            Some(__any__) if is_valid_start_for_identifier(__any__) => {
                // Valid identifier start is ahead - complete this token
                $self.set_token_kind(TokenKind::$token)
                    .end_token_here_with_size($token_size);
            }

            // Something unidentifiable was found, we need to handle that
            _ => {
                $self.set_token_kind(TokenKind::Unknown)
                    .end_token_here_with_size($token_size);
            }
        };
    };
}

/// Panic unless the current character matches the pattern.
/// This should only be used to make sure there aren't issues with the way the
/// lexer passes from function to function - don't actually use for user code
/// issues!
#[macro_export]
macro_rules! assert_peek {
    ($self:ident, $pattern:pat $(if $guard:expr)? $(,)?) => {
        let __c__ = $self.peek();
        match __c__ {
            $pattern $(if $guard)? => {}
            _ => {
                panic!(
                    "Unexpected character {:?} on line {}, character {} (offset {})",
                    $self.peek(), $self.line_number, $self.offset() - $self.line_offset,
                    $self.offset()
                );
            }
        }
    };
}

#[macro_export]
macro_rules! assert_peek_not {
    ($self:ident, $pattern:pat $(if $guard:expr)? $(,)?) => {
        let __c__ = $self.peek();
        match __c__ {
            $pattern $(if $guard)? => {
                panic!(
                    "Unexpected character {:?} on line {}, character {} (offset {})",
                    $self.peek(), $self.line_number, $self.offset() - $self.line_offset,
                    $self.offset()
                );
            }
            _ => {}
        }
    };
}

impl<'a> ScriptLexer<'a> {
    pub fn new(script: Script<'a>) -> Self {
        Self {
            script,
            string_interner: StringInterner::default(),
            current_token: Token::empty(),
            current_iterator: script.iterator(),
            indents_handled_for_current_line: false,
            newline_handled_for_current_line: false,
            line_number: 0,
            line_offset: 0,
        }
    }

    /// Find and process the next token from the input data
    fn process_next_token(&mut self) -> bool {
        self.reset_token();

        match self.peek() {
            Some('\n' | '\r') if self.newline_handled_for_current_line => {
                self.indents_handled_for_current_line = false;
                self.next();
                return false;
            }

            Some('\n' | '\r') if !self.newline_handled_for_current_line => {
                self.line_number += 1;
                self.line_offset = self.offset();
                self.indents_handled_for_current_line = false;
                self.newline_handled_for_current_line = true;
                self.set_token_kind(TokenKind::LineBreak)
                    .single_token_here();
                self.next();
            }

            Some('\t') if !self.indents_handled_for_current_line => {
                self.tab_indent();
            }

            Some(' ') if !self.indents_handled_for_current_line => {
                self.space_indent();
            }

            _ => {
                self.indents_handled_for_current_line = true;
                self.newline_handled_for_current_line = false;
            }
        }

        if self.has_token() {
            return true;
        }

        match self.peek() {
            // Non-indent whitespace
            Some(' ') => {
                self.next();
            }

            // Language features
            Some(FEATURE_ANNOTATION) => self.annotation(),
            Some(FEATURE_COMMENT) => self.comment(),
            Some(FEATURE_STRING | FEATURE_SHORT_STRING) => self.string_literal(),

            // Language core
            Some(':') => {
                self.set_token_kind(TokenKind::Colon)
                    .single_token_here();
                self.next();
            }
            Some('.') => {
                self.set_token_kind(TokenKind::Period)
                    .single_token_here();
                self.next();
            }
            Some(',') => {
                self.set_token_kind(TokenKind::Comma)
                    .single_token_here();
                self.next();
            }
            Some('(') => {
                self.set_token_kind(TokenKind::BracketRoundOpen)
                    .single_token_here();
                self.next();
            }
            Some(')') => {
                self.set_token_kind(TokenKind::BracketRoundClosed)
                    .single_token_here();
                self.next();
            }
            Some('[') => {
                self.set_token_kind(TokenKind::BracketSquareOpen)
                    .single_token_here();
                self.next();
            }
            Some(']') => {
                self.set_token_kind(TokenKind::BracketSquareClosed)
                    .single_token_here();
                self.next();
            }
            Some('{') => {
                self.set_token_kind(TokenKind::BracketCurlyOpen)
                    .single_token_here();
                self.next();
            }
            Some('}') => {
                self.set_token_kind(TokenKind::BracketCurlyClosed)
                    .single_token_here();
                self.next();
            }

            Some('<') => {
                multi_char_match! { self, ComparisonLesserThan, 1,
                    Some('<') => {
                        multi_char_match! { self, BitwiseLeftShift, 2, }
                    },

                    Some('=') => {
                        multi_char_match! { self, ComparisonLesserThanOrEqualTo, 2, }
                    }
                }
            }

            Some('>') => {
                multi_char_match! { self, ComparisonGreaterThan, 1,
                    Some('>') => {
                        multi_char_match! { self, BitwiseRightShift, 2, }
                    },

                    Some('=') => {
                        multi_char_match! { self, ComparisonGreaterThanOrEqualTo, 2, }
                    }
                }
            }

            Some('=') => {
                multi_char_match! { self, Assignment, 1,
                    Some('=') => {
                        multi_char_match! { self, ComparisonEqualTo, 2, }
                    }
                }
            }

            Some('!') => {
                multi_char_match! { self, NegateExpression, 1,
                    Some('=') => {
                        multi_char_match! { self, ComparisonNotEqualTo, 2, }
                    }
                }
            }

            Some('~') => {
                multi_char_match! { self, BitwiseNot, 1,
                    Some('=') => {
                        multi_char_match! { self, BitwiseTargetedNot, 2, }
                    }
                }
            }

            Some('&') => {
                multi_char_match! { self, BitwiseAnd, 1,
                    Some('&') => {
                        multi_char_match! { self, ComparisonAnd, 2, }
                    },

                    Some('=') => {
                        multi_char_match! { self, BitwiseTargetedAnd, 2, }
                    }
                }
            }

            Some('|') => {
                multi_char_match! { self, BitwiseOr, 1,
                    Some('|') => {
                        multi_char_match! { self, ComparisonOr, 2, }
                    },

                    Some('=') => {
                        multi_char_match! { self, BitwiseTargetedOr, 2, }
                    }
                }
            }

            Some('^') => {
                multi_char_match! { self, BitwiseXor, 1,
                    Some('=') => {
                        multi_char_match! { self, BitwiseTargetedXor, 2, }
                    }
                }
            }

            Some('+') => {
                multi_char_match! { self, MathAdd, 1,
                    Some('+') => {
                        multi_char_match! { self, MathIncrement, 2, }
                    },
                    Some('=') => {
                        multi_char_match! { self, MathTargetedAdd, 2, }
                    }
                }
            }

            Some('-') => {
                multi_char_match! { self, MathSubtract, 1,
                    Some('-') => {
                        multi_char_match! { self, MathDecrement, 2, }
                    },
                    Some('=') => {
                        multi_char_match! { self, MathTargetedSubtract, 2, }
                    },
                    Some('>') => {
                        multi_char_match! { self, TypeArrow, 2, }
                    },
                    Some('0'..='9') => {
                        let start = self.offset();
                        self.negative_number_literal()
                            .set_token_start(start);
                    }
                }
            }

            Some('/') => {
                multi_char_match! { self, MathDivide, 1,
                    Some('=') => {
                        multi_char_match! { self, MathTargetedDivide, 2, }
                    }
                }
            }

            Some('*') => {
                multi_char_match! { self, MathMultiply, 1,
                    Some('=') => {
                        multi_char_match! { self, MathTargetedMultiply, 2, }
                    }
                }
            }

            Some('%') => {
                multi_char_match! { self, MathModulo, 1,
                    Some('=') => {
                        multi_char_match! { self, MathTargetedModulo, 2, }
                    }
                }
            }

            Some('0'..='9') => {
                self.positive_number_literal();
            }

            _ => {
                // This might be a named item
                self.named_item();
            }
        }

        match self.current_token.kind {
            TokenKind::None => false,
            _ => true,
        }
    }

    /// Parse until a new token is found - returns None when there are no tokens left.
    pub fn parse(&mut self) -> Option<Token> {
        loop {
            if self.peek() == None {
                return None;
            }

            let result = self.process_next_token();

            if result == false {
                continue;
            }

            return Some(self.current_token);
        }
    }
}
