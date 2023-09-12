use crate::lexer::language::characters::{LC_CLOSE_CURLY_BRACKET, LC_CLOSE_ROUND_BRACKET, LC_CLOSE_SQUARE_BRACKET, LC_COLON, LC_COMMA, LC_OPEN_CURLY_BRACKET, LC_OPEN_ROUND_BRACKET, LC_OPEN_SQUARE_BRACKET};

pub mod characters;
pub mod keywords;
pub(crate) mod features;

pub fn is_valid_character_for_identifier(c: char) -> bool {
    match c {
        LC_COLON => false,
        LC_COMMA => false,
        LC_OPEN_ROUND_BRACKET | LC_CLOSE_ROUND_BRACKET => false,
        LC_OPEN_SQUARE_BRACKET | LC_CLOSE_SQUARE_BRACKET => false,
        LC_OPEN_CURLY_BRACKET | LC_CLOSE_CURLY_BRACKET => false,
        '<' | '>' | '+' | '-' | '/' | '%' | '^' | '$' | '*' | '@' | '!' | '\\' | '=' => false,
        _ => true
    }
}

pub fn is_valid_start_for_identifier(c: char) -> bool {
    match c {
        '0'..='9' => false, // Don't allow numbers
        c if !is_valid_character_for_identifier(c) => false,
        _ => true
    }
}

pub fn is_valid_body_for_identifier(c: char) -> bool {
    match c {
        ' ' => false, // Don't allow spaces
        c if !is_valid_character_for_identifier(c) => false,
        _ => true
    }
}
