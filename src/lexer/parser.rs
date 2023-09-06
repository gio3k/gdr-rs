use crate::lexer::{is_char_token, ScriptToken};
use crate::lexer::reader::ReaderState;

#[derive(Debug)]
pub enum LexerError {
    Other(String),
    UnexpectedEof,
    UnexpectedDigit,
}

fn next_string(input: &Vec<u8>, state: &mut ReaderState) -> Result<Vec<char>, LexerError> {
    if state.empty() {
        return Err(LexerError::UnexpectedEof);
    }

    let mut contents: Vec<char> = Vec::<char>::new();

    loop {
        if state.empty() {
            // We're out of characters but the string hasn't been finished
            return Err(LexerError::UnexpectedEof);
        }

        let c = state.next(input);
        match c {
            '"' => break, // End of string

            _ => contents.push(c)
        }
    }

    Ok(contents)
}

fn next_identifier(input: &Vec<u8>, state: &mut ReaderState, skip_previous: bool) -> Result<Vec<char>, LexerError> {
    if state.empty() {
        return Err(LexerError::UnexpectedEof);
    }

    let mut contents: Vec<char> = Vec::<char>::new();

    if !skip_previous {
        // Identifiers can be anything - we want to include the character that made us start looking
        contents.push(state.peek_previous(input));
    }

    loop {
        if state.empty() {
            break; // Just return prematurely if we're at EOF
        }

        // First peek the character - we need to check if it's important to anything else
        let c = state.peek(input);
        if is_char_token(c) {
            // The character is a token - our identifier is probably complete
            break;
        }

        // Make sure the character isn't a newline or cr
        if c == '\n' || c == '\r' {
            break;
        }

        // This character isn't important, just absorb it
        state.next(input);

        match c {
            ' ' | '\r' | '\n' => break,
            _ => contents.push(c)
        }
    }

    Ok(contents)
}

fn next_comment(input: &Vec<u8>, state: &mut ReaderState) -> Result<ScriptToken, LexerError> {
    let mut contents: Vec<char> = Vec::<char>::new();
    let mut importance: u8 = 1;

    loop {
        if state.peek(input) != '#' {
            break;
        }

        importance += 1;
        state.next(input);
    }

    loop {
        if state.empty() {
            break;
        }

        let c = state.next(input);

        if c == '\n' {
            break;
        }

        contents.push(c);
    }

    Ok(ScriptToken::Comment(contents, importance))
}

fn parse_spaced_scope_depth(input: &Vec<u8>, state: &mut ReaderState) -> u8 {
    let mut depth: u8 = 0;
    let tab_size: u8 = 4;
    let mut spaces: u8 = 1;

    loop {
        match state.peek(input) {
            ' ' => spaces += 1,

            _ => break
        }

        if spaces == tab_size - 1 {
            depth += 1;
        }

        state.next(input);
    }

    depth
}

fn parse_tabbed_scope_depth(input: &Vec<u8>, state: &mut ReaderState) -> u8 {
    let mut depth: u8 = 1;

    loop {
        match state.peek(input) {
            '\t' => depth += 1,

            _ => break
        }

        state.next(input);
    }

    depth
}

pub fn parse(input: &Vec<u8>) -> Result<Vec<ScriptToken>, LexerError> {
    let mut state = ReaderState {
        offset: 0,
        size: input.len(),
    };

    let mut result: Vec<ScriptToken> = Vec::new();

    loop {
        if state.empty() {
            break;
        }

        let mut error: Option<LexerError> = None;

        match state.next(input) {
            '\n' | '\r' => continue,

            // Indent depth
            '\t' => {
                let depth = parse_tabbed_scope_depth(input, &mut state);
                if depth != 0 {
                    result.push(ScriptToken::ScopeDepth(depth))
                }
            }
            ' ' => {
                let depth = parse_spaced_scope_depth(input, &mut state);
                if depth != 0 {
                    result.push(ScriptToken::ScopeDepth(depth))
                }
            }

            // Sets
            '(' => result.push(ScriptToken::SetStart()),
            ')' => result.push(ScriptToken::SetEnd()),

            // Arrays
            '[' => result.push(ScriptToken::ArrayStart()),
            ']' => result.push(ScriptToken::ArrayEnd()),

            // Dictionaries
            '{' => result.push(ScriptToken::DictStart()),
            '}' => result.push(ScriptToken::DictEnd()),

            // Language features
            ':' => result.push(ScriptToken::FuncOrTypeHint()),
            '.' => result.push(ScriptToken::ExpressionDelimiter()),
            ',' => result.push(ScriptToken::DataDelimiter()),

            // Strings
            '"' => match next_string(input, &mut state) {
                Ok(data) => result.push(ScriptToken::String(data)),
                Err(e) => error = Some(e)
            },

            // NodePath
            '$' => {
                let data = if state.next(input) == '"' {
                    next_string(input, &mut state)
                } else {
                    next_identifier(input, &mut state, false)
                };
                match data {
                    Ok(pt) => result.push(ScriptToken::NodePath(pt)),
                    Err(e) => error = Some(e)
                }
            }

            // Comments
            '#' => match next_comment(input, &mut state) {
                Ok(tk) => result.push(tk),
                Err(e) => error = Some(e)
            }

            // Annotations / attributes
            '@' => match next_identifier(input, &mut state, true) {
                Ok(tk) => result.push(ScriptToken::Annotation(tk)),
                Err(e) => error = Some(e)
            }

            // Unknown - probably an identifier
            _ => match next_identifier(input, &mut state, false) {
                Ok(tk) => result.push(ScriptToken::Identifier(tk)),
                Err(e) => error = Some(e)
            },
        }

        if error.is_some() {
            return Err(error.unwrap());
        }
    }

    Ok(result)
}
