use std::iter::{Skip, Take};
use std::str::Chars;

pub mod lexer;
pub mod sponge;

/// Representation of a script file / contents
#[derive(Copy, Clone)]
pub struct Script<'a> {
    data: &'a str,
    length: usize,
}

impl<'a> Script<'a> {
    fn new(data: &'a str) -> Self {
        Self {
            data,
            length: data.len(),
        }
    }

    pub fn iterator(&self) -> Chars<'a> {
        self.data.chars()
            .clone()
    }

    pub fn length(&self) -> usize {
        self.length
    }

    /// View a slice of data within the provided bounds
    pub fn slice(&self, location: ScriptLocation) -> Skip<Take<Chars<'a>>> {
        self.data.chars()
            .clone()
            .take(location.end)
            .skip(location.start)
    }

    /// View a slice of data (as a string) within the provided bounds
    pub fn slice_to_string(&self, location: ScriptLocation) -> String {
        self.data.chars()
            .clone()
            .take(location.end)
            .skip(location.start)
            .collect()
    }
}

#[derive(Debug, Copy, Clone)]
pub struct ScriptLocation {
    pub start: usize,
    pub end: usize,
}

impl ScriptLocation {
    pub fn new(start: usize, end: usize) -> Self {
        Self {
            start,
            end,
        }
    }

    pub fn single(v: usize) -> Self {
        Self {
            start: v,
            end: v,
        }
    }
}
