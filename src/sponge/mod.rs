use crate::{assert_token_kind, assert_token_kind_not};
use crate::script::Script;
use crate::sponge::absorbers::expressions::Expression;
use crate::sponge::absorbers::statements::Statement;
use crate::sponge::sponge_issues::error_kind::ErrorKind;
use crate::sponge::sponge_issues::SpongeIssue;
use crate::stage0::ScriptLexer;
use crate::stage0::tokens::{Token, TokenKind};

pub mod absorbers;
pub mod sponge_core;
pub mod sponge_issues;

pub struct Sponge<'a> {
    lexer: ScriptLexer<'a>,

    // Current processing iteration
    /// Current token (for the current iteration)
    token: Token,

    /// Issues detected while absorbing
    issues: Vec<SpongeIssue>,
}

impl<'a> Sponge<'a> {
    pub fn new(script: Script<'a>) -> Self {
        let lexer = ScriptLexer::new(script);
        Self {
            lexer,
            token: Token::empty(),
            issues: vec![],
        }
    }

    pub(crate) fn reset_token(&mut self) {
        self.token.kind = TokenKind::None;
    }

    /// Returns whether or not the token kind is None
    pub fn has_token(&self) -> bool {
        !matches!(self.token.kind, TokenKind::None)
    }

    /// Processes the current token.
    pub(crate) fn process(&mut self) {
        assert_token_kind_not!(self.token, TokenKind::None);

        println!("token: {:?}", self.token);

        match self.token.kind {
            TokenKind::IndentTab | TokenKind::IndentSpaces => {
                // Unexpected indent found
                // Throw error, then absorb the indents
                self.throw_error_here(ErrorKind::UnexpectedIdentifier);
                self.throw_error_here(ErrorKind::UnexpectedTopLevelIndent);
                self.absorb_indents_for_depth();
            }

            TokenKind::LineBreak => {
                // Skip line break
                self.scan();
            }

            _ => {
                println!("Unhandled token {:?}", self.token);
                self.scan();
            }
        }
    }

    /// Absorbs the next token from the lexer without processing it.
    pub(crate) fn scan(&mut self) {
        match self.lexer.scan() {
            None => self.reset_token(),
            Some(v) => {
                self.token = v;
            }
        }
    }

    pub fn process_all(&mut self) {
        self.scan();

        loop {
            self.process();
            if matches!(self.token.kind, TokenKind::None) {
                break;
            }
        }
    }
}

#[cfg(test)]
mod sponge_tests {
    use crate::{assert_token_kind, assert_token_value};
    use crate::core::literal::Literal;
    use crate::script::Script;
    use crate::stage0::ScriptLexer;
    use crate::stage0::tokens::TokenKind;

    #[test]
    fn float() {
        let script = Script::new("");
    }
}