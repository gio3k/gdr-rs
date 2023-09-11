use std::str::Chars;
use string_interner::backend::StringBackend;
use string_interner::StringInterner;
use string_interner::symbol::SymbolU32;
use crate::lexer::core::error::Error;
use crate::lexer::core::token::Token;

pub mod core;
mod features;

pub struct Lexer<'a> {
    current_error: Error,
    current_token: Token,
    string_interner: StringInterner<StringBackend<SymbolU32>>,
    chars: Chars<'a>,
    chars_at_construct_time: Chars<'a>,
    source_length: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(chars: Chars<'a>) -> Lexer {
        let source_length = chars.as_str().len();
        let chars_at_construct_time = chars.clone();
        Lexer {
            current_error: Error::empty(),
            current_token: Token::empty(),
            string_interner: StringInterner::default(),
            chars,
            chars_at_construct_time,
            source_length,
        }
    }
}