use crate::lexer::reader::LexerReaderState;

mod reader;

#[derive(Clone, Debug)]
pub enum ScriptToken {
    Identifier(Vec<u8>),
    String(Vec<u8>),
    StringName(Vec<u8>),
    NodePath(Vec<u8>),
    FindUniqueNode(Vec<u8>),
    FindNodePath(Vec<u8>),
    Comment(Vec<u8>, u8),
    Annotation(Vec<u8>),
    FuncOrTypeHint(),

    // Array / dictionary delimiter
    DataDelimiter(),

    // Parent-child / Members
    ExpressionDelimiter(),

    // Depth
    ScopeDepth(u8),

    // Dictionary {}
    DictStart(),
    DictEnd(),

    // Set ()
    SetStart(),
    SetEnd(),

    // Array []
    ArrayStart(),
    ArrayEnd(),
}

pub fn is_char_token(c: u8) -> bool {
    match c {
        b'(' | b')' => true,
        b'{' | b'}' => true,
        b'[' | b']' => true,
        b':' => true,
        b'@' => true,
        b'"' => true,
        b'#' => true,
        b'.' => true,
        b',' => true,
        b'%' => true,
        b'^' => true,
        b'&' => true,
        _ => false
    }
}

#[derive(Debug)]
pub enum LexerError {
    Other(String),
    UnexpectedEof,
    UnexpectedDigit,
    UnexpectedLineBreak,
    InvalidMultiLineStringEnd,
}

fn next_string(state: &mut LexerReaderState) -> Result<Vec<u8>, LexerError> {
    if state.empty() {
        return Err(LexerError::UnexpectedEof);
    }

    let mut is_multiline: bool = false;
    let mut is_apostrophe: bool = false;
    let mut contents: Vec<u8> = Vec::<u8>::new();

    // Let's get the first character of the string
    let c0 = state.next();

    // Check for apostrophe
    if c0 == b'\'' {
        is_apostrophe = true;
    } else if c0 == b'"' {
        // The first value character was a quotation mark - it's likely that this is multi-line
        // We'll check the next character - if it's also a string marker, then it's multi-line
        if state.peek() != b'"' {
            // It's not a string marker - we just have an empty string. Return!
            return Ok(contents);
        }

        // We have a multiline string
        is_multiline = true;
        state.next();
    } else {
        // First character isn't a string marker, add it to contents and move on
        contents.push(c0);
    }

    loop {
        if state.empty() {
            // We're out of characters but the string hasn't been finished
            return Err(LexerError::UnexpectedEof);
        }

        let c = state.next();

        if c == b'\n' && !is_multiline {
            // Error - line break in a normal string
            // Not sure if GDScript allows this?
            return Err(LexerError::UnexpectedLineBreak);
        }

        if c == b'\'' && is_apostrophe {
            break; // End of string!
        }

        if c == b'"' && !is_apostrophe {
            if is_multiline {
                // Let's make sure the end of the string is correct
                if state.next() != b'"' || state.next() != b'"' {
                    return Err(LexerError::InvalidMultiLineStringEnd);
                }
                break;
            } else {
                break; // End of string!
            }
        }

        contents.push(c);
    }

    Ok(contents)
}

fn next_identifier(state: &mut LexerReaderState, skip_previous: bool) -> Result<Vec<u8>, LexerError> {
    if state.empty() {
        return Err(LexerError::UnexpectedEof);
    }

    let mut contents: Vec<u8> = Vec::<u8>::new();

    if !skip_previous {
        // Identifiers can be anything - we want to include the character that made us start looking
        contents.push(state.peek_previous());
    }

    loop {
        if state.empty() {
            break; // Just return prematurely if we're at EOF
        }

        // First peek the character - we need to check if it's important to anything else
        let c = state.peek();
        if is_char_token(c) {
            // The character is a token - our identifier is probably complete
            break;
        }

        // Make sure the character isn't a newline or cr
        if c == b'\n' || c == b'\r' {
            break;
        }

        // This character isn't important, just absorb it
        state.next();

        match c {
            b' ' | b'\r' | b'\n' => break,
            _ => contents.push(c)
        }
    }

    Ok(contents)
}

fn next_comment(state: &mut LexerReaderState) -> Result<ScriptToken, LexerError> {
    let mut contents: Vec<u8> = Vec::<u8>::new();
    let mut importance: u8 = 1;

    loop {
        if state.peek() != b'#' {
            break;
        }

        importance += 1;
        state.next();
    }

    loop {
        if state.empty() {
            break;
        }

        let c = state.next();

        if c == b'\n' {
            break;
        }

        contents.push(c);
    }

    Ok(ScriptToken::Comment(contents, importance))
}

fn read_small_string(state: &mut LexerReaderState) -> Result<Vec<u8>, LexerError> {
    match state.next() {
        b'"' => next_string(state),
        b'\'' => next_string(state),
        _ => next_identifier(state, false)
    }
}

fn parse_spaced_scope_depth(state: &mut LexerReaderState) -> u8 {
    let mut depth: u8 = 0;
    let tab_size: u8 = 4;
    let mut spaces: u8 = 1;

    loop {
        match state.peek() {
            b' ' => spaces += 1,

            _ => break
        }

        if spaces == tab_size - 1 {
            depth += 1;
        }

        state.next();
    }

    depth
}

fn parse_tabbed_scope_depth(state: &mut LexerReaderState) -> u8 {
    let mut depth: u8 = 1;

    loop {
        match state.peek() {
            b'\t' => depth += 1,

            _ => break
        }

        state.next();
    }

    depth
}

pub fn parse(input: &Vec<u8>) -> Result<Vec<ScriptToken>, LexerError> {
    let mut state = LexerReaderState::new(input);

    let mut result: Vec<ScriptToken> = Vec::new();

    loop {
        if state.empty() {
            break;
        }

        let mut error: Option<LexerError> = None;

        match state.next() {
            b'\n' | b'\r' => continue,

            // Indent depth
            b'\t' => {
                let depth = parse_tabbed_scope_depth(&mut state);
                if depth != 0 {
                    result.push(ScriptToken::ScopeDepth(depth))
                }
            }
            b' ' => {
                let depth = parse_spaced_scope_depth(&mut state);
                if depth != 0 {
                    result.push(ScriptToken::ScopeDepth(depth))
                }
            }

            // Sets
            b'(' => result.push(ScriptToken::SetStart()),
            b')' => result.push(ScriptToken::SetEnd()),

            // Arrays
            b'[' => result.push(ScriptToken::ArrayStart()),
            b']' => result.push(ScriptToken::ArrayEnd()),

            // Dictionaries
            b'{' => result.push(ScriptToken::DictStart()),
            b'}' => result.push(ScriptToken::DictEnd()),

            // Language features
            b':' => result.push(ScriptToken::FuncOrTypeHint()),
            b'.' => result.push(ScriptToken::ExpressionDelimiter()),
            b',' => result.push(ScriptToken::DataDelimiter()),

            // Strings
            b'"' => match next_string(&mut state) {
                Ok(data) => result.push(ScriptToken::String(data)),
                Err(e) => error = Some(e)
            },

            // FindNodePath
            b'$' => {
                match read_small_string(&mut state) {
                    Ok(pt) => result.push(ScriptToken::FindNodePath(pt)),
                    Err(e) => error = Some(e)
                }
            }

            // FindUniqueNode
            b'%' => {
                match read_small_string(&mut state) {
                    Ok(pt) => result.push(ScriptToken::FindUniqueNode(pt)),
                    Err(e) => error = Some(e)
                }
            }

            // NodePath
            b'^' => {
                match read_small_string(&mut state) {
                    Ok(pt) => result.push(ScriptToken::NodePath(pt)),
                    Err(e) => error = Some(e)
                }
            }

            // StringName
            b'&' => {
                match read_small_string(&mut state) {
                    Ok(pt) => result.push(ScriptToken::StringName(pt)),
                    Err(e) => error = Some(e)
                }
            }

            // Comments
            b'#' => match next_comment(&mut state) {
                Ok(tk) => result.push(tk),
                Err(e) => error = Some(e)
            }

            // Annotations / attributes
            b'@' => match next_identifier(&mut state, true) {
                Ok(tk) => result.push(ScriptToken::Annotation(tk)),
                Err(e) => error = Some(e)
            }

            // Unknown - probably an identifier
            _ => match next_identifier(&mut state, false) {
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
