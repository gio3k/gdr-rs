use std::iter::{Skip, Take};
use std::str::Chars;

pub struct Reader<'a> {
    input: Option<&'a str>,
    input_length: usize,
    chars: Chars<'a>,
    initial_chars: Chars<'a>,
}

impl<'a> Reader<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input: Some(input),
            input_length: input.len(),
            chars: input.chars(),
            initial_chars: input.chars(),
        }
    }

    fn new_like(reader: &'a Reader) -> Self {
        Self {
            input: None,
            input_length: reader.input_length,
            chars: reader.chars.clone(),
            initial_chars: reader.initial_chars.clone(),
        }
    }

    pub fn see(&self) -> Option<char> {
        return self.chars.clone().next();
    }

    pub fn slice_from(&self, start: usize, end: usize) -> Skip<Take<Chars<'a>>> {
        self.initial_chars.clone().take(end).skip(start)
    }

    pub fn next(&mut self) -> Option<char> {
        self.chars.next()
    }

    pub fn next_back(&mut self) -> Option<char> {
        self.chars.next_back()
    }

    /// Advances by specified amount, returns true if the end of file is hit
    pub fn advance_by(&mut self, amount: usize) -> bool {
        for n in 0..amount {
            match self.next() {
                None => return true,
                Some(_) => continue
            }
        }
        return false;
    }

    pub fn fork(&'a self) -> Self {
        Self::new_like(self)
    }

    pub fn pos(&self) -> usize {
        self.input_length - self.chars.as_str().len()
    }

    pub fn prior_pos(&self) -> usize {
        self.pos() - 1
    }
}