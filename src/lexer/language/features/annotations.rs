use crate::lexer::Lexer;
use crate::{lexer_expect, read};
use crate::lexer::core::token::TokenKind;

pub const FEATURE_ANNOTATION: char = '@';

impl<'a> Lexer<'a> {
    /// Parses an annotation
    /// Assumes the iterator is on an annotation start character (@)
    pub fn annotation(&mut self) {
        let token_start = self.offset();

        lexer_expect!(self, Some(FEATURE_ANNOTATION));

        // Skip the first token
        self.next();
        let data_start = self.offset();

        read! { self,
            Some(' ' | '\n' | '\r' | '(') | None => {
                let end = self.offset();
                self.set_token_kind(TokenKind::Annotation)
                    .set_token_pos(data_start, end)
                    .make_token_symbol()
                    .set_token_pos(token_start, end);
                break;
            },
            _ => {}
        }
    }
}