use crate::assert_token_kind_not;
use crate::script::Script;
use crate::stage0::ScriptLexer;
use crate::stage0::tokens::{Token, TokenKind};

pub mod absorbers;
pub mod sponge_core;
pub mod crumbs;

pub struct Sponge<'a> {
    lexer: ScriptLexer<'a>,

    // Current processing iteration
    /// Current token (for the current iteration)
    token: Token,
}

impl<'a> Sponge<'a> {
    pub fn new(script: Script<'a>) -> Self {
        let lexer = ScriptLexer::new(script);
        Self {
            lexer,
            token: Token::empty(),
        }
    }

    pub(crate) fn reset_token(&mut self) {
        self.token.kind = TokenKind::None;
    }

    /// Returns whether or not the token kind is None
    pub fn has_token(&self) -> bool {
        !matches!(self.token.kind, TokenKind::None)
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
        match self.lexer.scan() {
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