use std::io::ErrorKind;
use crate::lexer::ParseError;
use crate::lexer::token::{is_char_token, ScriptToken};

pub struct Parser {
    input: Vec<u8>,
    offset: usize,
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

    fn next_string(&mut self) -> Result<Vec<char>, ParseError> {
        if !self.has_next() {
            return Err(ParseError::UnexpectedEof);
        }

        let mut contents: Vec<char> = Vec::<char>::new();

        loop {
            if !self.has_next() {
                // We're out of characters but the string hasn't been finished
                return Err(ParseError::UnexpectedEof);
            }

            let c = self.next();
            match c {
                '"' => break, // End of string

                _ => contents.push(c)
            }
        }

        Ok(contents)
    }

    fn next_identifier(&mut self, skip_previous: bool) -> Result<Vec<char>, ParseError> {
        if !self.has_next() {
            return Err(ParseError::UnexpectedEof);
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

    fn parse_comment(&mut self) -> Result<ScriptToken, ParseError> {
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

    fn next_token(&mut self) -> Result<Option<ScriptToken>, ParseError> {
        let c = self.next();

        match c {
            '\n' | '\r' => Ok(None),

            '\t' => {
                let depth = self.parse_tabbed_scope_depth();
                if depth != 0 {
                    Ok(Some(ScriptToken::ScopeDepth(depth)))
                } else {
                    Ok(None)
                }
            }
            ' ' => {
                let depth = self.parse_spaced_scope_depth();
                if depth != 0 {
                    Ok(Some(ScriptToken::ScopeDepth(depth)))
                } else {
                    Ok(None)
                }
            }

            '(' => Ok(Some(ScriptToken::SetStart())),
            ')' => Ok(Some(ScriptToken::SetEnd())),

            '[' => Ok(Some(ScriptToken::ArrayStart())),
            ']' => Ok(Some(ScriptToken::ArrayEnd())),

            '{' => Ok(Some(ScriptToken::DictStart())),
            '}' => Ok(Some(ScriptToken::DictEnd())),

            ':' => Ok(Some(ScriptToken::FuncOrTypeHint())),
            '.' => Ok(Some(ScriptToken::ExpressionDelimiter())),

            ',' => Ok(Some(ScriptToken::DataDelimiter())),

            '"' => {
                match self.next_string() {
                    Ok(contents) => Ok(Some(ScriptToken::String(contents))),
                    Err(e) => Err(e)
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
                    Ok(contents) => Ok(Some(ScriptToken::NodePath(contents))),
                    Err(e) => Err(e)
                }
            }

            '#' => match self.parse_comment() {
                Ok(token) => Ok(Some(token)),
                Err(e) => Err(e)
            }
            '@' => match self.next_identifier(true) {
                Ok(contents) => Ok(Some(ScriptToken::Annotation(contents))),
                Err(e) => Err(e)
            }
            _ => match self.next_identifier(false) {
                Ok(contents) => Ok(Some(ScriptToken::Identifier(contents))),
                Err(e) => Err(e)
            }
        }
    }

    /// Parse script into tokens
    pub fn parse(&mut self) -> Result<Vec<ScriptToken>, ParseError> {
        let mut result = Vec::<ScriptToken>::new();

        loop {
            if !self.has_next() {
                break;
            }

            let result: Option<ParseError> = match self.next_token() {
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