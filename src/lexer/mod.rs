use crate::lexer::reader::Reader;
use crate::lexer::tokens::{Token, TokenKind};

pub mod tokens;
pub(crate) mod reader;

#[derive(Debug)]
enum LexerError {
    IndentMismatch
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
        _ => Ok(0),
    }

    loop {
        match reader.next() {
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

pub fn parse(input: &str) -> Result<Vec<Token>, LexerError> {
    let mut reader = Reader::new(input);

    let mut output = Vec::<Token>::new();

    // Keep track of the depth - we'll use this to keep track of scopes
    let mut current_depth: u8 = 0;

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
            }

            '(' => output.push(Token::new_single(reader.pos(), TokenKind::SetStart)),
            ')' => output.push(Token::new_single(reader.pos(), TokenKind::SetEnd)),

            '[' => output.push(Token::new_single(reader.pos(), TokenKind::ArrayStart)),
            ']' => output.push(Token::new_single(reader.pos(), TokenKind::ArrayEnd)),

            '{' => output.push(Token::new_single(reader.pos(), TokenKind::ContainerStart)),
            '}' => output.push(Token::new_single(reader.pos(), TokenKind::ContainerEnd)),

            _ => {}
        }
    }

    Ok(output)
}