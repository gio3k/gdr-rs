mod token;
pub mod parser;

#[derive(Debug)]
pub enum ParseError {
    Unknown = 0,
    UnexpectedEof,
    UnexpectedDigit,
}