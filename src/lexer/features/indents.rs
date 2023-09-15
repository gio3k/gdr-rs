use crate::lexer::ScriptLexer;
use crate::{read, assert_peek};
use crate::lexer::token::TokenKind;

impl<'a> ScriptLexer<'a> {
    pub(crate) fn space_indent(&mut self) {
        assert_peek!(self, Some(' '));

        let size = 4;
        let mut count = 0;

        read! { self,
            Some(' ') => {
                count += 1;
                if count >= size {
                    self.set_token_kind(TokenKind::IndentSpaces)
                        .set_token_end(size);
                    return;
                }
            },

            _ => break
        }
    }

    pub(crate) fn tab_indent(&mut self) {
        assert_peek!(self, Some('\t'));

        self.set_token_kind(TokenKind::IndentTab)
            .single_token_here();

        // Move to the next character
        self.next();
    }
}