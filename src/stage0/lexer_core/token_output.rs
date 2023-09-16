use crate::core::literal::Literal;
use crate::script::Location;
use crate::stage0::ScriptLexer;
use crate::stage0::tokens::TokenKind;

impl<'a> ScriptLexer<'a> {
    /// Returns whether or not the token kind is None
    pub fn has_token(&self) -> bool {
        !matches!(self.current_token.kind, TokenKind::None)
    }

    /// End the token here (current iterator position), with the token having the provided size
    pub(crate) fn end_token_here_with_size(&mut self, size: usize) -> &mut Self {
        let end = self.offset();
        self.current_token.location.end = end;
        self.current_token.location.start = end - (size - 1);
        self
    }

    /// End the token here (current iterator position), with the token starting at the provided value
    pub(crate) fn end_token_here(&mut self, start: usize) -> &mut Self {
        let end = self.offset();
        self.current_token.location.end = end;
        self.current_token.location.start = start;
        self
    }

    /// End the token here (current iterator position) as a 1 character token
    pub(crate) fn single_token_here(&mut self) -> &mut Self {
        self.end_token_here_with_size(1)
    }

    /// Set the token position / bounds
    pub(crate) fn set_token_pos(&mut self, location: Location) -> &mut Self {
        self.current_token.location = location;
        self
    }

    /// Set the token position / bounds start
    pub(crate) fn set_token_start(&mut self, start: usize) -> &mut Self {
        self.current_token.location.start = start;
        self
    }

    /// Set the token position / bounds end
    pub(crate) fn set_token_end(&mut self, end: usize) -> &mut Self {
        self.current_token.location.end = end;
        self
    }

    /// Set the token kind
    pub(crate) fn set_token_kind(&mut self, kind: TokenKind) -> &mut Self {
        self.current_token.kind = kind;
        self
    }

    /// Set the token value
    pub(crate) fn set_token_value(&mut self, value: Literal) -> &mut Self {
        self.current_token.value = value;
        self
    }

    /// Make the token value a string based on the token bounds
    pub(crate) fn make_token_symbol(&mut self) -> &mut Self {
        let data = self.script.slice_to_string(self.current_token.location);
        let symbol = self.cache_string(data);
        self.current_token.value = Literal::Symbol(
            symbol
        );
        self
    }

    /// Prepare the token state for the next iteration
    pub(crate) fn reset_output(&mut self) {
        self.current_token.kind = TokenKind::None;
        self.current_token.value = Literal::None;
    }
}