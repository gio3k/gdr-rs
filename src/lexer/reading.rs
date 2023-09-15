use crate::lexer::ScriptLexer;

impl<'a> ScriptLexer<'a> {
    /// Return the next character without moving the iterator
    pub fn peek(&self) -> Option<char> {
        self.current_iterator.clone().next()
    }

    /// Return the next character and advance the iterator forwards
    pub fn next(&mut self) -> Option<char> {
        self.current_iterator.next()
    }

    /// Return the iterator position
    pub fn offset(&self) -> usize {
        self.script.length - self.current_iterator.as_str().len()
    }
}