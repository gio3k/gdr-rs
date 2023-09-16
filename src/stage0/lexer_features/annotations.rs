use crate::{assert_peek, read};
use crate::script::Location;
use crate::stage0::ScriptLexer;
use crate::stage0::tokens::TokenKind;

pub const FEATURE_ANNOTATION: char = '@';

impl<'a> ScriptLexer<'a> {
    /// Parses an annotation
    /// Assumes the iterator is on an annotation start character (@)
    pub fn annotation(&mut self) {
        let token_start = self.offset();

        assert_peek!(self, Some(FEATURE_ANNOTATION));

        // Skip the first token
        self.next();
        let data_start = self.offset();

        read! { self,
            Some(' ' | '\n' | '\r' | '(') | None => {
                let end = self.offset();
                self.set_token_kind(TokenKind::Annotation)
                    .set_token_pos(Location::new(data_start, end))
                    .make_token_symbol()
                    .set_token_pos(Location::new(token_start, end));
                break;
            },
            _ => {}
        }
    }
}