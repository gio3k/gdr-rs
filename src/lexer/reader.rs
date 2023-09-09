use std::str::Chars;

pub struct Reader<'a> {
    input: Option<&'a str>,
    input_length: usize,
    chars: Chars<'a>,
}

impl Reader {
    pub fn new(input: &str) -> Self {
        Self {
            input: Some(input),
            input_length: input.len(),
            chars: input.chars(),
        }
    }

    fn new_like(reader: &Reader) -> Self {
        Self {
            input: None,
            input_length: reader.len,
            chars: reader.chars.clone(),
        }
    }

    pub fn see(&self) -> Option<char> {
        return self.chars.clone().next();
    }

    pub fn next(&mut self) -> Option<char> {
        self.chars.next()
    }

    pub fn fork(&self) -> Self {
        Self::new_like(self)
    }

    pub fn pos(&self) -> usize {
        self.input_length - self.chars.as_str().len()
    }
}