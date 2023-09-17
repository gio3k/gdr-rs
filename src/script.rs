use std::iter::{Skip, Take};
use std::str::Chars;

/// Representation of a script file / contents
#[derive(Copy, Clone)]
pub struct Script<'a> {
    data: &'a str,
    length: usize,
}

impl<'a> Script<'a> {
    pub(crate) fn new(data: &'a str) -> Self {
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
    pub fn slice(&self, location: Location) -> Skip<Take<Chars<'a>>> {
        self.data.chars()
            .clone()
            .take(location.end)
            .skip(location.start)
    }

    /// View a slice of data (as a string) within the provided bounds
    pub fn slice_to_string(&self, location: Location) -> String {
        self.data.chars()
            .clone()
            .take(location.end)
            .skip(location.start)
            .collect()
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Location {
    pub start: usize,
    pub end: usize,
}

impl Location {
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

    pub fn expand_from(a: Location, b: Location) -> Self {
        Self {
            start: if a.start < b.start { a.start } else { b.start },
            end: if a.end > b.end { a.end } else { b.end },
        }
    }

    pub fn expand(&self, other: Location) -> Self {
        Self {
            start: if self.start < other.start { self.start } else { other.start },
            end: if self.end > other.end { self.end } else { other.end },
        }
    }
}