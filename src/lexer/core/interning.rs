// String interning support for the lexer

use string_interner::symbol::SymbolU32;
use crate::lexer::Lexer;

impl<'a> Lexer<'a> {
    /// Cache a string and return a symbol for it
    pub fn cache_string<T>(&mut self, string: T) -> SymbolU32
        where
            T: AsRef<str>,
    {
        self.string_interner.get_or_intern(string)
    }

    /// Get a cached string by symbol
    pub fn get_symbol_string(&self, symbol: SymbolU32) -> Option<&str> {
        self.string_interner.resolve(symbol)
    }
    
}