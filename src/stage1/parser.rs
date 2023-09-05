use std::io::ErrorKind;
use std::io::ErrorKind::InvalidInput;
use crate::stage1::token::ScriptToken;

pub struct Parser {
    input: Vec<u8>,
    offset: usize,
}

fn is_char_token(c: char) -> bool {
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
        _ => false
    }
}

impl Parser {
    pub(crate) fn new(input: Vec<u8>) -> Parser {
        Parser { input, offset: 0 }
    }

    fn next(&mut self) -> char {
        let x = self.input[self.offset];
        self.offset += 1;
        return x as char;
    }

    fn has_next(&self) -> bool {
        self.offset < self.input.capacity()
    }

    fn peek(&self) -> char {
        return self.input[self.offset] as char;
    }

    fn previous(&self) -> char {
        return self.input[self.offset - 1] as char;
    }

    fn next_string(&mut self) -> Result<Vec<char>, ErrorKind> {
        if !self.has_next() {
            return Err(ErrorKind::UnexpectedEof);
        }

        let mut contents: Vec<char> = Vec::<char>::new();

        loop {
            if !self.has_next() {
                // We're out of characters but the string hasn't been finished
                return Err(ErrorKind::UnexpectedEof);
            }

            let c = self.next();
            match c {
                '"' => break, // End of string

                _ => contents.push(c)
            }
        }

        Ok(contents)
    }

    fn next_identifier(&mut self, skip_previous: bool) -> Result<Vec<char>, ErrorKind> {
        if !self.has_next() {
            return Err(ErrorKind::UnexpectedEof);
        }

        let mut contents: Vec<char> = Vec::<char>::new();

        if !skip_previous {
            // Identifiers can be anything - we want to include the character that made us start looking
            contents.push(self.previous());
        }

        loop {
            if !self.has_next() {
                break; // Just return prematurely if we're at EOF
            }

            // First peek the character - we need to check if it's important to anything else
            let c = self.peek();
            if is_char_token(c) {
                // The character is a token - our identifier is probably complete
                break;
            }

            // Make sure the character isn't a newline or cr
            if c == '\n' || c == '\r' {
                break;
            }

            // This character isn't important, just absorb it
            self.next();

            match c {
                ' ' | '\r' | '\n' => break,
                _ => contents.push(c)
            }
        }

        Ok(contents)
    }

    fn parse_comment(&mut self) -> Result<ScriptToken, ErrorKind> {
        let mut contents: Vec<char> = Vec::<char>::new();
        let mut importance: u8 = 1;

        loop {
            if self.peek() != '#' {
                break;
            }

            importance += 1;
            self.next();
        }

        loop {
            if !self.has_next() {
                break;
            }

            let c = self.next();

            if c == '\n' {
                break;
            }

            contents.push(c);
        }

        Ok(ScriptToken::Comment(contents, importance))
    }

    fn parse_spaced_scope_depth(&mut self) -> u8 {
        let mut depth: u8 = 0;
        let tab_size: u8 = 4;
        let mut spaces: u8 = 1;

        loop {
            match self.peek() {
                ' ' => spaces += 1,

                _ => break
            }

            if spaces == tab_size - 1 {
                depth += 1;
            }

            self.next();
        }

        depth
    }

    fn parse_tabbed_scope_depth(&mut self) -> u8 {
        let mut depth: u8 = 1;

        loop {
            match self.peek() {
                '\t' => depth += 1,

                _ => break
            }

            self.next();
        }

        depth
    }

    fn next_token(&mut self) -> Result<Option<ScriptToken>, ErrorKind> {
        let c = self.next();

        Ok(match c {
            '\n' | '\r' => None,

            '\t' => {
                let depth = self.parse_tabbed_scope_depth();
                if depth != 0 {
                    Some(ScriptToken::ScopeDepth(depth))
                } else {
                    None
                }
            }
            ' ' => {
                let depth = self.parse_spaced_scope_depth();
                if depth != 0 {
                    Some(ScriptToken::ScopeDepth(depth))
                } else {
                    None
                }
            }

            '(' => Some(ScriptToken::SetStart()),
            ')' => Some(ScriptToken::SetEnd()),

            '[' => Some(ScriptToken::ArrayStart()),
            ']' => Some(ScriptToken::ArrayEnd()),

            '{' => Some(ScriptToken::DictStart()),
            '}' => Some(ScriptToken::DictEnd()),

            ':' => Some(ScriptToken::FuncOrTypeHint()),
            '.' => Some(ScriptToken::MemberHint()),

            ',' => Some(ScriptToken::Delimiter()),

            '"' => {
                match self.next_string() {
                    Ok(contents) => Some(ScriptToken::String(contents)),
                    Err(_) => None
                }
            }

            '$' => {
                let result = if self.next() == '"' {
                    // We're a NodePath encased in quotes
                    self.next_string()
                } else {
                    // We're a NodePath without quotes
                    self.next_identifier(false)
                };

                match result {
                    Ok(contents) => Some(ScriptToken::NodePath(contents)),
                    Err(_) => None
                }
            }

            '#' => self.parse_comment().ok(),
            '@' => match self.next_identifier(true) {
                Ok(contents) => Some(ScriptToken::Annotation(contents)),
                Err(_) => None
            }
            _ => match self.next_identifier(false) {
                Ok(contents) => Some(ScriptToken::Identifier(contents)),
                Err(_) => None
            }
        })
    }

    /// Parse script into tokens
    pub fn parse(&mut self) -> Result<Vec<ScriptToken>, ErrorKind> {
        let mut result = Vec::<ScriptToken>::new();

        loop {
            if !self.has_next() {
                break;
            }

            let result: Option<ErrorKind> = match self.next_token() {
                Ok(option) => {
                    match option {
                        None => {}
                        Some(token) => result.push(token)
                    }
                    None
                }
                Err(e) => {
                    Some(e)
                }
            };

            if result.is_some() {
                return Err(result.unwrap());
            }
        }

        Ok(result)
    }
}