pub mod config;
pub mod issues;
pub(crate) mod basic;

use crate::lexer::ScriptLexer;
use crate::lexer::token::{Token, TokenKind};
use crate::{assert_token_kind, assert_token_kind_not, Script};
use crate::sponge::config::SpongeConfig;
use crate::sponge::issues::Issue;

pub struct Sponge<'a> {
    lexer: ScriptLexer<'a>,

    /// Configuration
    config: SpongeConfig,

    // Current processing iteration
    /// Current token (for the current iteration)
    token: Token,

    /// Issues found while parsing the script
    issues: Vec<Issue>,
}

impl<'a> Sponge<'a> {
    pub fn new_from_data(script: Script<'a>) -> Self {
        let lexer = ScriptLexer::new(script);
        Self {
            lexer,
            config: SpongeConfig {},
            token: Token::empty(),
            issues: vec![],
        }
    }

    pub(crate) fn reset_token(&mut self) {}

    pub(crate) fn process(&mut self) {
        assert_token_kind_not!(self.token, TokenKind::None);

        match self.token.kind {
            TokenKind::IndentTab | TokenKind::IndentSpaces => {
                self.absorb_indents_for_depth_value();
            }
            _ => {}
        }
    }

    /// Absorbs the next token from the lexer.
    pub(crate) fn absorb(&mut self) {
        match self.lexer.parse() {
            None => self.reset_token(),
            Some(v) => {
                self.token = v;
            }
        }
    }
}