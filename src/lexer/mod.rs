use crate::lexer::reader::ReaderState;

mod reader;

#[derive(Clone, Debug)]
pub enum ScriptToken {
    Identifier(Vec<char>),
    String(Vec<char>),
    StringName(Vec<char>),
    NodePath(Vec<char>),
    FindUniqueNode(Vec<char>),
    FindNodePath(Vec<char>),
    Comment(Vec<char>, u8),
    Annotation(Vec<char>),
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

pub fn is_char_token(c: char) -> bool {
    match c {
        '(' | ')' => true,
        '{' | '}' => true,
        '[' | ']' => true,
        ':' => true,
        '@' => true,
        '"' => true,
        '#' => true,
        '.' => true,
        ',' => true,
        '%' => true,
        '^' => true,
        '&' => true,
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

fn next_string(input: &Vec<u8>, state: &mut ReaderState) -> Result<Vec<char>, LexerError> {
    if state.empty() {
        return Err(LexerError::UnexpectedEof);
    }

    let mut is_multiline: bool = false;
    let mut is_apostrophe: bool = false;
    let mut contents: Vec<char> = Vec::<char>::new();

    // Let's get the first character of the string
    let c0 = state.next(&input);

    // Check for apostrophe
    if c0 == '\'' {
        is_apostrophe = true;
    } else if c0 == '"' {
        // The first value character was a quotation mark - it's likely that this is multi-line
        // We'll check the next character - if it's also a string marker, then it's multi-line
        if state.peek(&input) != '"' {
            // It's not a string marker - we just have an empty string. Return!
            return Ok(contents);
        }

        // We have a multiline string
        is_multiline = true;
        state.next(&input);
    } else {
        // First character isn't a string marker, add it to contents and move on
        contents.push(c0);
    }

    loop {
        if state.empty() {
            // We're out of characters but the string hasn't been finished
            return Err(LexerError::UnexpectedEof);
        }

        let c = state.next(input);

        if c == '\n' && !is_multiline {
            // Error - line break in a normal string
            // Not sure if GDScript allows this?
            return Err(LexerError::UnexpectedLineBreak);
        }

        if c == '\'' && is_apostrophe {
            break; // End of string!
        }

        if c == '"' && !is_apostrophe {
            if is_multiline {
                // Let's make sure the end of the string is correct
                if state.next(&input) != '"' || state.next(&input) != '"' {
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

fn read_small_string(input: &Vec<u8>, state: &mut ReaderState) -> Result<Vec<char>, LexerError> {
    match state.next(&input) {
        '"' => next_string(&input, state),
        '\'' => next_string(&input, state),
        _ => next_identifier(&input, state, false)
    }
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

            // FindNodePath
            '$' => {
                match read_small_string(input, &mut state) {
                    Ok(pt) => result.push(ScriptToken::FindNodePath(pt)),
                    Err(e) => error = Some(e)
                }
            }

            // FindUniqueNode
            '%' => {
                match read_small_string(input, &mut state) {
                    Ok(pt) => result.push(ScriptToken::FindUniqueNode(pt)),
                    Err(e) => error = Some(e)
                }
            }

            // NodePath
            '^' => {
                match read_small_string(input, &mut state) {
                    Ok(pt) => result.push(ScriptToken::NodePath(pt)),
                    Err(e) => error = Some(e)
                }
            }

            // StringName
            '&' => {
                match read_small_string(input, &mut state) {
                    Ok(pt) => result.push(ScriptToken::StringName(pt)),
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
