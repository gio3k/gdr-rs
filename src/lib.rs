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
        self.data.chars().clone()
    }

    pub fn length(&self) -> usize {
        self.length
    }

    /// View a slice of data within the provided bounds
    pub fn slice(&self, start: usize, end: usize) -> Skip<Take<Chars<'a>>> {
        self.data.chars().clone().take(end).skip(start)
    }

    /// View a slice of data (as a string) within the provided bounds
    pub fn slice_to_string(&self, start: usize, end: usize) -> String {
        self.data.chars().clone().take(end).skip(start).collect()
    }
}

pub struct ScriptLocation {
    pub start: usize,
    pub end: usize,
}

