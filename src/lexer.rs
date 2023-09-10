use std::iter::{Skip, Take};
use std::num::{ParseFloatError, ParseIntError};
use std::str::Chars;
use crate::lexer::tokens::{Token, TokenKind, TokenValue};

pub mod tokens;
mod indents;
mod comments;
mod annotations;
mod strings;
mod multi_char;

#[macro_export]
macro_rules! error_here {
    ($self:ident, $e:ident $( , $args:expr )*) => {
        Err(LexerError::$e($self.offset(), $( $args ),*))
    }
}

#[derive(Debug)]
pub enum LexerError {
    IndentTypeMismatch(usize),
    InvalidCharacterForToken(usize),
    StringNotTerminated(usize),
    IncompleteIdentifier(usize),
    UnexpectedCharacterInIdentifier(usize),
    IdentifierAllowedCharacterMismatch(usize),
    NumberLiteralInvalid(usize),
    FloatLiteralParseError(usize, ParseFloatError),
    IntLiteralParseError(usize, ParseIntError),
}

pub struct Lexer<'a> {
    input_length: usize,

    // Initial iterator state
    input_chars: Chars<'a>,

    // Active iterator state
    chars_x: Chars<'a>,

    // Saved iterator state
    chars_s: Chars<'a>,

    // Resulting token vector
    result: Vec<Token>,

    // Current scope depth
    depth: i32,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input_length: input.len(),
            input_chars: input.chars(),
            chars_x: input.chars().clone(),
            chars_s: input.chars().clone(),
            result: vec![],
            depth: 0,
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

    fn advance_and_see(&mut self) -> Option<char> {
        self.advance();
        self.see()
    }

    /// View a slice of data
    fn view(&self, start: usize, end: usize) -> Skip<Take<Chars<'a>>> {
        self.input_chars.clone().take(end).skip(start)
    }

    /// Advances by one
    fn advance(&mut self) -> Option<char> {
        self.chars_x.next()
    }

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
    fn insert_token_data(&mut self, start: usize, end: usize, kind: TokenKind, value: TokenValue) -> Result<(), LexerError> {
        self.result.push(Token {
            start,
            end,
            kind,
            value,
        });
        Ok(())
    }

    fn insert_token(&mut self, start: usize, end: usize, kind: TokenKind) -> Result<(), LexerError> {
        self.result.push(Token {
            start,
            end,
            kind,
            value: TokenValue::None,
        });
        Ok(())
    }

    fn insert_token_here(&mut self, start: usize, kind: TokenKind) -> Result<(), LexerError> {
        let end = self.offset();
        self.result.push(Token {
            start,
            end,
            kind,
            value: TokenValue::None,
        });
        Ok(())
    }

    // Add a token to the result list, assuming the current character is the last character of the token
    fn put_token(&mut self, size: usize, kind: TokenKind) -> Result<(), LexerError> {
        let start = self.offset();
        let end = start + size;
        self.result.push(Token {
            start,
            end,
            kind,
            value: TokenValue::None,
        });
        Ok(())
    }

    fn put_token_and_advance(&mut self, size: usize, kind: TokenKind) -> Result<(), LexerError> {
        self.put_token(size, kind)?;
        self.advance();
        Ok(())
    }

    // Add a token to the result list, assuming the current character is the one and only character
    fn put_single_token(&mut self, kind: TokenKind) -> Result<(), LexerError> {
        self.put_token(1, kind)
    }

    fn put_single_token_and_advance(&mut self, kind: TokenKind) -> Result<(), LexerError> {
        self.put_single_token(kind)?;
        self.advance();
        Ok(())
    }

    fn insert_string_token(&mut self, start: usize, end: usize, kind: TokenKind) -> Result<(), LexerError> {
        let value = TokenValue::String(self.view(start, end).collect());
        self.result.push(Token {
            start,
            end,
            kind,
            value,
        });
        Ok(())
    }

    fn insert_string_filled_token_here(&mut self, start: usize, kind: TokenKind) -> Result<(), LexerError> {
        let end = self.offset();
        self.insert_string_token(start, end, kind)
    }
}

impl<'a> Lexer<'a> {
    fn parse_next(&mut self, c: char) -> Result<(), LexerError> {
        match c {
            '\n' | '\r' => {
                // Whitespace
                self.advance();
            }

            '\t' | ' ' => {
                // Tabs / indents
                let start = self.offset();
                let depth = self.parse_current_indent_depth()?;
                let depth_delta = depth - self.depth;

                if depth_delta > 0 {
                    self.insert_token_here(start, TokenKind::PopScope)?
                } else if depth_delta < 0 {
                    self.insert_token_here(start, TokenKind::PushScope)?
                }
                self.depth = depth;
            }

            // Single character tokens
            ':' => self.put_single_token_and_advance(TokenKind::Colon)?,
            '.' => self.put_single_token_and_advance(TokenKind::Period)?,
            ',' => self.put_single_token_and_advance(TokenKind::Comma)?,
            '(' => self.put_single_token_and_advance(TokenKind::PushSet)?,
            ')' => self.put_single_token_and_advance(TokenKind::PopSet)?,
            '[' => self.put_single_token_and_advance(TokenKind::PushArray)?,
            ']' => self.put_single_token_and_advance(TokenKind::PopArray)?,
            '{' => self.put_single_token_and_advance(TokenKind::PushContainer)?,
            '}' => self.put_single_token_and_advance(TokenKind::PopContainer)?,

            '#' => { self.parse_generic_comment()? }

            '@' => { self.parse_annotation()? }

            '\'' => { self.parse_string_single_quote_small()? }

            '"' => { self.parse_string_with_size_checked()? }

            _ => { self.identify_multi_character()?; }
        }

        Ok(())
    }

    pub fn parse(&mut self) -> Result<&Vec<Token>, LexerError> {
        loop {
            match self.see() {
                None => break,
                Some(c) => {
                    match self.parse_next(c) {
                        Ok(_) => {}
                        Err(e) => {
                            println!("Encountered parse error {:?} - adjusting and moving on", e);
                            self.advance();
                        }
                    }
                }
            }
        }

        Ok(&self.result)
    }
}
