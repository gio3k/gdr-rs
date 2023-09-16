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

    pub(crate) fn reset_token(&mut self) {
        self.token.kind = TokenKind::None;
    }

    pub(crate) fn process(&mut self) {
        assert_token_kind_not!(self.token, TokenKind::None);

        println!("token: {:?}", self.token);

        match self.token.kind {
            TokenKind::IndentTab | TokenKind::IndentSpaces => {
                self.absorb_indents_for_depth_value();
            }
            _ => {
                println!("Unhandled token {:?}", self.token);
                self.absorb();
            }
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

    pub fn process_all(&mut self) {
        self.absorb();

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
    use crate::{assert_token_kind, assert_token_value, Script};
    use crate::lexer::token::{TokenKind, TokenValue};
    use crate::lexer::ScriptLexer;
    use crate::sponge::Sponge;

    #[test]
    fn run() {
        let script = Script::new("\t    var a = 3");
        let mut sponge = Sponge::new_from_data(script);
        sponge.process_all();
    }
}