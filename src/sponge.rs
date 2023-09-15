pub mod config;
pub mod issues;

use std::str::Chars;
use crate::lexer::ScriptLexer;
use crate::lexer::token::Token;
use crate::Script;
use crate::sponge::config::SpongeConfig;

pub struct Sponge<'a> {
    lexer: ScriptLexer<'a>,

    /// Configuration
    config: SpongeConfig,

    // Current processing iteration
    /// Current token (for the current iteration)
    token: Option<Token>,
}

impl<'a> Sponge<'a> {
    pub fn new_from_data(script: Script<'a>) -> Self {
        let lexer = ScriptLexer::new(script);
        Self {
            lexer,
            config: SpongeConfig {},
            token: None,
        }
    }

    /// Absorbs the next token from the lexer.
    pub(crate) fn absorb(&mut self) {
        self.token = self.lexer.proceed();
    }
}