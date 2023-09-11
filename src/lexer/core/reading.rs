// Reader functionality for the lexer

use std::iter::{Skip, Take};
use std::str::Chars;
use string_interner::symbol::SymbolU32;
use crate::lexer::Lexer;

impl<'a> Lexer<'a> {
    /// Return the next character without moving the iterator
    pub fn peek(&self) -> Option<char> {
        self.chars.clone().next()
    }

    /// Return the next character and advance the iterator forwards
    pub fn next(&mut self) -> Option<char> {
        self.chars.next()
    }

    /// Return the iterator position
    pub fn offset(&self) -> usize {
        self.source_length - self.chars.as_str().len()
    }

    /// View a slice of data within the provided bounds
    pub fn slice(&self, start: usize, end: usize) -> Skip<Take<Chars<'a>>> {
        self.chars_at_construct_time.clone().take(end).skip(start)
    }

    /// View a slice of data (as a string) within the provided bounds
    pub fn slice_to_string(&self, start: usize, end: usize) -> String {
        self.chars_at_construct_time.clone().take(end).skip(start).collect()
    }

    /// View a slice of data within the provided bounds, cache it, then return the symbol
    pub fn slice_to_string_symbol(&mut self, start: usize, end: usize) -> SymbolU32 {
        self.cache_string(self.slice_to_string(start, end))
    }
}