use crate::lexer::ScriptLexer;
use crate::{assert_peek, read};
use crate::lexer::token::TokenKind;

pub const FEATURE_COMMENT: char = '#';

impl<'a> ScriptLexer<'a> {
    /// Parses a comment
    /// Assumes the iterator is on a comment start character (#)
    pub fn comment(&mut self) {
        let start = self.offset();

        assert_peek!(self, Some(FEATURE_COMMENT));

        read! { self,
            Some('\n' | '\r') | None => {
                self.set_token_kind(TokenKind::Comment)
                    .end_token_here(start)
                    .make_token_symbol();
                break;
            },
            _ => {}
        }
    }
}