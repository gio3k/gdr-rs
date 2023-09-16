use crate::stage0::lexer_features::annotations::FEATURE_ANNOTATION;
use crate::stage0::lexer_features::comments::FEATURE_COMMENT;
use crate::stage0::lexer_features::identifiers::is_valid_start_for_identifier;
use crate::stage0::lexer_features::strings::{FEATURE_SHORT_STRING, FEATURE_STRING};
use crate::stage0::ScriptLexer;
use crate::stage0::tokens::TokenKind;

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

#[macro_export]
macro_rules! read {
    ($self:ident, $($pattern:pat $(if $guard:expr)* => $action:expr),*) => {
        loop {
            match $self.peek() {
                $($pattern $(if $guard)* => $action),*
            }
            $self.next();
        }
    };
}

macro_rules! next_multi_char {
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

impl<'a> ScriptLexer<'a> {
    /// Process the next character from the input data
    pub(crate) fn process_next(&mut self) -> bool {
        self.reset_output();

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
                next_multi_char! { self, ComparisonLesserThan, 1,
                    Some('<') => {
                        next_multi_char! { self, BitwiseLeftShift, 2, }
                    },

                    Some('=') => {
                        next_multi_char! { self, ComparisonLesserThanOrEqualTo, 2, }
                    }
                }
            }

            Some('>') => {
                next_multi_char! { self, ComparisonGreaterThan, 1,
                    Some('>') => {
                        next_multi_char! { self, BitwiseRightShift, 2, }
                    },

                    Some('=') => {
                        next_multi_char! { self, ComparisonGreaterThanOrEqualTo, 2, }
                    }
                }
            }

            Some('=') => {
                next_multi_char! { self, Assignment, 1,
                    Some('=') => {
                        next_multi_char! { self, ComparisonEqualTo, 2, }
                    }
                }
            }

            Some('!') => {
                next_multi_char! { self, NegateExpression, 1,
                    Some('=') => {
                        next_multi_char! { self, ComparisonNotEqualTo, 2, }
                    }
                }
            }

            Some('~') => {
                next_multi_char! { self, BitwiseNot, 1,
                    Some('=') => {
                        next_multi_char! { self, BitwiseTargetedNot, 2, }
                    }
                }
            }

            Some('&') => {
                next_multi_char! { self, BitwiseAnd, 1,
                    Some('&') => {
                        next_multi_char! { self, ComparisonAnd, 2, }
                    },

                    Some('=') => {
                        next_multi_char! { self, BitwiseTargetedAnd, 2, }
                    }
                }
            }

            Some('|') => {
                next_multi_char! { self, BitwiseOr, 1,
                    Some('|') => {
                        next_multi_char! { self, ComparisonOr, 2, }
                    },

                    Some('=') => {
                        next_multi_char! { self, BitwiseTargetedOr, 2, }
                    }
                }
            }

            Some('^') => {
                next_multi_char! { self, BitwiseXor, 1,
                    Some('=') => {
                        next_multi_char! { self, BitwiseTargetedXor, 2, }
                    }
                }
            }

            Some('+') => {
                next_multi_char! { self, MathAdd, 1,
                    Some('+') => {
                        next_multi_char! { self, MathIncrement, 2, }
                    },
                    Some('=') => {
                        next_multi_char! { self, MathTargetedAdd, 2, }
                    }
                }
            }

            Some('-') => {
                next_multi_char! { self, MathSubtract, 1,
                    Some('-') => {
                        next_multi_char! { self, MathDecrement, 2, }
                    },
                    Some('=') => {
                        next_multi_char! { self, MathTargetedSubtract, 2, }
                    },
                    Some('>') => {
                        next_multi_char! { self, TypeArrow, 2, }
                    },
                    Some('0'..='9') => {
                        let start = self.offset();
                        self.negative_number_literal()
                            .set_token_start(start);
                    }
                }
            }

            Some('/') => {
                next_multi_char! { self, MathDivide, 1,
                    Some('=') => {
                        next_multi_char! { self, MathTargetedDivide, 2, }
                    }
                }
            }

            Some('*') => {
                next_multi_char! { self, MathMultiply, 1,
                    Some('=') => {
                        next_multi_char! { self, MathTargetedMultiply, 2, }
                    }
                }
            }

            Some('%') => {
                next_multi_char! { self, MathModulo, 1,
                    Some('=') => {
                        next_multi_char! { self, MathTargetedModulo, 2, }
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
}