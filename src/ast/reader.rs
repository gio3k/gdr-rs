use crate::lexer::ScriptToken;

pub(crate) struct TokenReaderState {
    pub offset: usize,
    pub size: usize,
    input: Vec<ScriptToken>,
}

impl TokenReaderState {
    pub fn new(x: Vec<ScriptToken>) -> TokenReaderState {
        TokenReaderState {
            offset: 0,
            size: x.len(),
            input: x,
        }
    }

    /// Return the next character
    pub fn next(&mut self) -> &ScriptToken {
        let v = &self.input[self.offset];
        self.offset += 1;
        return v;
    }

    pub fn next_no_return(&mut self) {
        self.offset += 1;
    }

    pub fn empty(&self) -> bool {
        !(self.offset < self.size)
    }

    pub fn peek(&self) -> &ScriptToken {
        return &self.input[self.offset];
    }

    pub fn peek_previous(&self) -> &ScriptToken {
        if self.offset == 0 {
            panic!("Tried to peek_previous at the beginning of the parser!");
        }
        return &self.input[self.offset - 1];
    }
}