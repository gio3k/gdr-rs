use std::marker::PhantomData;

pub struct ParserState<TA, TB> {
    pub(crate) offset: usize,
    pub(crate) size: usize,
    pub(crate) pa: PhantomData<TA>,
    pub(crate) pb: PhantomData<TB>
}

impl ParserState<u8, char> {
    /// Return the next character
    pub fn next(&mut self, input: &Vec<u8>) -> char {
        let v = input[self.offset];
        self.offset += 1;
        return v as char;
    }

    pub fn empty(&self) -> bool {
        !(self.offset < self.size)
    }

    pub fn peek(&self, input: &Vec<u8>) -> char {
        return input[self.offset] as char;
    }

    pub fn peek_previous(&self, input: &Vec<u8>) -> char {
        if self.offset == 0 {
            panic!("Tried to peek_previous at the beginning of the parser!");
        }
        return input[self.offset - 1] as char;
    }
}