use std::num::ParseFloatError;
use crate::ast::{Node, SyntaxError};
use crate::ast::reader::TokenReaderState;
use crate::lexer::ScriptToken;

pub enum Literal {
    Identifier(String),
    Float(f64),
    Integer(i64),
    Boolean(bool),
}

fn read_number(data: &Vec<char>, state: &mut TokenReaderState) -> Result<Literal, SyntaxError> {
    let mut buffer: Vec<char> = Vec::new();
    let mut is_float: bool = false;

    // Append all of data
    buffer.extend(data);

    if !state.empty() {
        // There's more data to read - go until we're not a number anymore
        loop {
            match state.next() {
                ScriptToken::Identifier(d0) => {
                    // If this identifier starts with a number, add to the buffer
                    if d0.len() == 0 {
                        panic!("Empty identifier encountered reading number");
                    }
                    buffer.extend(d0);
                }
                ScriptToken::ExpressionDelimiter() => {
                    is_float = true;
                }
                _ => {
                    break;
                }
            }
        }
    }

    // todo: find a better way to do this
    // Turn buffer into a string
    let string_buffer: String = buffer.iter().collect();

    return if is_float {
        match string_buffer.parse::<f64>() {
            Ok(x) => Ok(Literal::Float(x)),
            Err(e) => Err(SyntaxError::InvalidFloatLiteral(e))
        }
    } else {
        match string_buffer.parse::<i64>() {
            Ok(x) => Ok(Literal::Integer(x)),
            Err(e) => Err(SyntaxError::InvalidIntLiteral(e))
        }
    };
}

fn read_literal(state: &mut TokenReaderState) -> Result<Literal, SyntaxError> {
    let token = state.peek();

    match token {
        _ => Err(SyntaxError::UnexpectedTokenForLiteral(token.clone())),

        ScriptToken::Identifier(data) => {
            if data.len() == 0 {
                // todo: handle this nicely
                panic!("Identifier token was empty!");
            }

            // We have the data from the identifier token - move on
            state.next();

            // First, let's see if we have a number
            if data[0].is_digit(10) {
                // We have a number - create a literal from it
                return match read_number(data, state, tokens) {
                    Ok(literal) => Ok(literal),
                    Err(e) => Err(e),
                };
            }

            // See if we have a boolean
            // todo: find a better way to do this!
            let data_string: String = data.clone().iter().collect();
            if data_string == "true" {
                return Ok(Literal::Boolean(true));
            }

            if data_string == "false" {
                return Ok(Literal::Boolean(false));
            }

            // Just return as an identifier...
            return Ok(Literal::Identifier(data_string));
        }
    }
}