use std::iter::{Skip, Take};
use std::str::Chars;
use crate::lexer::tokens::{Token, TokenKind, TokenValue};

pub mod tokens;
mod indents;
mod comments;
mod annotations;
mod strings;

#[derive(Debug)]
pub enum LexerError {
    IndentTypeMismatch,
    InvalidCharacterForToken,
    StringNotTerminated,
}

pub struct Lexer<'a> {
    input: &'a str,
    input_length: usize,

    // Initial iterator state
    input_chars: Chars<'a>,

    // Active iterator state
    chars_x: Chars<'a>,

    // Saved iterator state
    chars_s: Chars<'a>,

    // Resulting token vector
    result: Vec<Token>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            input_length: input.len(),
            input_chars: input.chars(),
            chars_x: input.chars().clone(),
            chars_s: input.chars().clone(),
            result: vec![],
        }
    }

    /// Save the current iterator state
    fn save(&mut self) {
        self.chars_s = self.chars_x.clone()
    }

    /// Restore iterator state from a previous save (.save())
    fn restore(&mut self) {
        self.chars_x = self.chars_s.clone()
    }

    fn see(&self) -> Option<char> {
        self.chars_x.clone().next()
    }

    /// View a slice of data
    fn view(&self, start: usize, end: usize) -> Skip<Take<Chars<'a>>> {
        self.input_chars.clone().take(end).skip(start)
    }

    /// Advances by one
    fn advance(&mut self) -> Option<char> {
        self.chars_x.next()
    }

    /// Rewinds by one
    fn rewind(&mut self) -> Option<char> { self.chars_x.next_back() }

    /// Advances by specified amount, returns true if the end of file is hit
    fn advance_by(&mut self, amount: usize) -> bool {
        for _n in 0..amount {
            match self.chars_x.next() {
                None => return true,
                Some(_) => continue
            }
        }
        false
    }

    fn offset(&self) -> usize {
        self.input_length - self.chars_x.as_str().len()
    }
}

/// Result utils
impl<'a> Lexer<'a> {
    fn push_token(&mut self, token: Token) -> Result<(), LexerError> {
        self.result.push(token);
        Ok(())
    }

    fn push_new_token(&mut self, start: usize, end: usize, kind: TokenKind) -> Result<(), LexerError> {
        self.push_token(
            Token {
                start,
                end,
                kind,
                value: TokenValue::None,
            }
        )
    }

    fn push_new_token_from_here(&mut self, start: usize, kind: TokenKind) -> Result<(), LexerError> {
        self.push_token(
            Token {
                start,
                end: self.offset(),
                kind,
                value: TokenValue::None,
            }
        )
    }

    fn push_new_single_token_from_here(&mut self, kind: TokenKind) -> Result<(), LexerError> {
        let offset = self.offset();
        self.push_token(
            Token {
                start: offset,
                end: offset + 1,
                kind,
                value: TokenValue::None,
            }
        )
    }

    fn advance_new_single_token_from_here(&mut self, kind: TokenKind) -> Result<(), LexerError> {
        self.push_new_single_token_from_here(kind)?;
        self.advance();
        Ok(())
    }

    fn push_new_string(&mut self, start: usize, end: usize, kind: TokenKind) -> Result<(), LexerError> {
        self.push_token(
            Token {
                start,
                end,
                kind,
                value: TokenValue::String(self.view(start, end).collect()),
            }
        )
    }

    fn push_new_string_from_here(&mut self, start: usize, kind: TokenKind) -> Result<(), LexerError> {
        self.push_new_string(start, self.offset(), kind)
    }
}

impl<'a> Lexer<'a> {
    pub fn parse(&mut self) -> Result<&Vec<Token>, LexerError> {
        let mut current_depth: i32 = 0;

        loop {
            match self.see() {
                Some('\n' | '\r') => {
                    // Whitespace
                    self.advance();
                }

                Some('\t' | ' ') => {
                    // Tabs / indents
                    let start = self.offset();
                    let depth = self.current_indent_depth()?;
                    let depth_delta = depth - current_depth;

                    if depth_delta > 0 {
                        self.push_new_token_from_here(start, TokenKind::ScopeEnd)?
                    } else if depth_delta < 0 {
                        self.push_new_token_from_here(start, TokenKind::ScopeStart)?
                    }
                    current_depth = depth;
                }

                Some('(') => {
                    self.advance_new_single_token_from_here(TokenKind::SetStart)?;
                }
                Some(')') => {
                    self.advance_new_single_token_from_here(TokenKind::SetEnd)?;
                }
                Some('[') => {
                    self.advance_new_single_token_from_here(TokenKind::ArrayStart)?;
                }
                Some(']') => {
                    self.advance_new_single_token_from_here(TokenKind::ArrayEnd)?;
                }

                Some('{') => {
                    self.advance_new_single_token_from_here(TokenKind::ContainerStart)?;
                }
                Some('}') => {
                    self.advance_new_single_token_from_here(TokenKind::ContainerEnd)?;
                }

                Some('#') => { self.generic_comment()? }

                Some('@') => { self.generic_annotation()? }

                Some('\'') => { self.string_single_quote_small()? }

                Some('"') => { self.string_with_size_checked()? }

                None => break,

                _ => {
                    self.advance();
                }
            }
        }

        Ok(&self.result)
    }
}
