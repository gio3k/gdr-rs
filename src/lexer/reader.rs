pub(crate) struct LexerReaderState<'a> {
    pub offset: usize,
    pub size: usize,
    input: &'a Vec<u8>,
}

impl LexerReaderState<'_> {
    pub fn new(x: &Vec<u8>) -> LexerReaderState {
        LexerReaderState {
            offset: 0,
            size: x.len(),
            input: x,
        }
    }


    /// Return the next character
    pub fn next(&mut self) -> u8 {
        let v = self.input[self.offset];
        self.offset += 1;
        v
    }

    pub fn empty(&self) -> bool {
        !(self.offset < self.size)
    }

    pub fn peek(&self) -> u8 {
        self.input[self.offset]
    }

    pub fn peek_previous(&self) -> u8 {
        if self.offset == 0 {
            panic!("Tried to peek_previous at the beginning of the parser!");
        }
        self.input[self.offset - 1]
    }
}