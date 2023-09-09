use std::fs::read;
use crate::lexer::reader::Reader;
use crate::lexer::tokens::{Token, TokenKind, TokenValue};

pub mod tokens;
pub(crate) mod reader;

#[derive(Debug)]
pub enum LexerError {
    IndentMismatch,
    InvalidCharacterForToken,
}

pub fn get_indent_size(c0: char, reader: &mut Reader) -> Result<i8, LexerError> {
    let mut tab_indent: i8 = 0;
    let mut is_tab: bool = false;
    let mut space_count: i8 = 0;

    match c0 {
        '\t' => {
            tab_indent += 1;
            is_tab = true
        }

        ' ' => space_count += 1,
        _ => return Ok(0),
    }

    loop {
        match reader.see() {
            None => break,
            Some(c) => {
                match c {
                    '\t' => {
                        tab_indent += 1;
                        is_tab = true
                    }

                    ' ' => space_count += 1,
                    _ => break
                }
                reader.next();
            }
        }
    }

    let tab_size: i8 = 4;

    if space_count >= tab_size && is_tab {
        // Indent type mismatch?
        return Err(LexerError::IndentMismatch);
    }

    return if is_tab {
        Ok(tab_indent)
    } else {
        Ok(space_count / tab_size)
    };
}

/// Absorb the comment and return a comment token
/// Keeps the # comment tokens
fn absorb_comment(reader: &mut Reader) -> Result<Option<Token>, LexerError> {
    let start = reader.prior_pos();

    loop {
        let char = match reader.see() {
            None => return Ok(None),
            Some(c) => c
        };

        match char {
            '\n' | '\r' => {
                let end = reader.pos();
                return Ok(
                    Some(
                        Token {
                            start,
                            end,
                            kind: TokenKind::Comment,
                            value: TokenValue::String(reader.slice_from(start, end).collect()),
                        }
                    )
                );
            }

            _ => {
                reader.next();
            }
        }
    }
}

fn absorb_annotation(reader: &mut Reader) -> Result<Option<Token>, LexerError> {
    let start = reader.pos();

    loop {
        let char = match reader.see() {
            None => return Ok(None),
            Some(c) => c
        };

        match char {
            ' ' | '\n' | '\r' | '(' => {
                let end = reader.pos();
                return Ok(
                    Some(
                        Token {
                            start,
                            end,
                            kind: TokenKind::Annotation,
                            value: TokenValue::String(reader.slice_from(start, end).collect()),
                        }
                    )
                );
            }
            _ => {
                reader.next();
            }
        }
    }
}

fn absorb_small_string(reader: &mut Reader) -> Result<Option<Token>, LexerError> {
    let start = reader.pos();

    loop {
        let char = match reader.see() {
            None => return Ok(None),
            Some(c) => c
        };

        match char {
            '\'' => {
                let end = reader.pos();
                reader.next(); // We don't want to read the string start again - skip it!
                return Ok(
                    Some(
                        Token {
                            start,
                            end,
                            kind: TokenKind::String,
                            value: TokenValue::String(reader.slice_from(start, end).collect()),
                        }
                    )
                );
            }
            _ => {
                reader.next();
            }
        }
    }
}

fn absorb_string(reader: &mut Reader) -> Result<Option<Token>, LexerError> {
    let mut reader_fork = reader.fork();
    let mut is_long_string: bool = false;

    // We need to figure out if the string is a long string (""")
    if let Some(c1) = reader_fork.next() {
        if c1 == '"' {
            if let Some(c2) = reader_fork.next() {
                if c2 == '"' {
                    // We have a long string!
                    is_long_string = true;
                    reader.advance_by(2);
                }
            }
        }
    }

    let start = reader.pos();
    let mut end: usize = 0;

    // Let's find the end of the string too
    loop {
        let char = match reader.next() {
            None => break,
            Some(c) => c
        };

        if char == '"' {
            end = reader.prior_pos();
            if !is_long_string {
                // Not a long string - one ending quote is fine
                break;
            } else {
                // Long string - let's see if the ending is correct
                if let Some(c1) = reader.next() {
                    if c1 == '"' {
                        if let Some(c2) = reader.next() {
                            break;
                        }
                    }
                }
            }
        }
    }

    let data: String = reader.slice_from(start, end).collect();
    Ok(
        Some(
            if is_long_string {
                Token {
                    start,
                    end,
                    kind: TokenKind::LongString,
                    value: TokenValue::String(data),
                }
            } else {
                Token {
                    start,
                    end,
                    kind: TokenKind::String,
                    value: TokenValue::String(data),
                }
            }
        )
    )
}

pub fn parse(input: &str) -> Result<Vec<Token>, LexerError> {
    let mut reader = Reader::new(input);

    let mut output = Vec::<Token>::new();

    // Keep track of the depth - we'll use this to keep track of scopes
    let mut current_depth: i8 = 0;

    loop {
        let char = match reader.next() {
            None => break,
            Some(c) => c
        };

        match char {
            '\n' | '\r' => continue,
            '\t' | ' ' => {
                // Handle indents
                let start = reader.pos();
                let depth = get_indent_size(char, &mut reader)?;
                let depth_delta = depth - current_depth;
                let end = reader.pos();
                if depth_delta > 0 {
                    output.push(Token::new(start, end, TokenKind::ScopeEnd));
                } else if depth_delta < 0 {
                    output.push(Token::new(start, end, TokenKind::ScopeStart));
                }
                current_depth = depth;
            }

            '(' => output.push(Token::new_single(&reader, TokenKind::SetStart)),
            ')' => output.push(Token::new_single(&reader, TokenKind::SetEnd)),

            '[' => output.push(Token::new_single(&reader, TokenKind::ArrayStart)),
            ']' => output.push(Token::new_single(&reader, TokenKind::ArrayEnd)),

            '{' => output.push(Token::new_single(&reader, TokenKind::ContainerStart)),
            '}' => output.push(Token::new_single(&reader, TokenKind::ContainerEnd)),

            '#' => {
                match absorb_comment(&mut reader)? {
                    None => {}
                    Some(token) => output.push(token)
                }
            }

            '@' => {
                match absorb_annotation(&mut reader)? {
                    None => {}
                    Some(token) => output.push(token)
                }
            }

            '\'' => {
                match absorb_small_string(&mut reader)? {
                    None => {}
                    Some(token) => output.push(token)
                }
            }

            '"' => {
                match absorb_string(&mut reader)? {
                    None => {}
                    Some(token) => output.push(token)
                }
            }

            _ => {}
        }
    }

    Ok(output)
}