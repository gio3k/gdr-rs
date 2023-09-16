pub(crate) mod lexer_core;
pub(crate) mod lexer_features;
mod processing;
pub mod tokens;

use std::str::Chars;
use string_interner::backend::StringBackend;
use string_interner::StringInterner;
use string_interner::symbol::SymbolU32;
use crate::script::Script;
use crate::stage0::tokens::Token;

pub struct ScriptLexer<'a> {
    /// The script being read
    pub(crate) script: Script<'a>,

    /// String interner
    pub(crate) string_interner: StringInterner<StringBackend<SymbolU32>>,

    // Current state, etc...
    /// Current token after last processing iteration
    pub(crate) current_token: Token,

    /// Current iterator after last processing iteration
    pub(crate) current_iterator: Chars<'a>,

    indents_handled_for_current_line: bool,
    newline_handled_for_current_line: bool,

    /// Current line number, starting from 0
    line_number: usize,

    /// Offset / location of the current line
    line_offset: usize,
}

impl<'a> ScriptLexer<'a> {
    pub fn new(script: Script<'a>) -> Self {
        Self {
            script,
            string_interner: StringInterner::default(),
            current_token: Token::empty(),
            current_iterator: script.iterator(),
            indents_handled_for_current_line: false,
            newline_handled_for_current_line: false,
            line_number: 0,
            line_offset: 0,
        }
    }

    /// Parse until a new token is found - returns None when there are no tokens left.
    pub fn scan(&mut self) -> Option<Token> {
        loop {
            if self.peek() == None {
                return None;
            }

            let result = self.process_next();

            if result == false {
                continue;
            }

            return Some(self.current_token);
        }
    }
}
