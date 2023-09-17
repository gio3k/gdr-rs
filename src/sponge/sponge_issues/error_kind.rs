use crate::stage0::tokens::TokenKind;

#[derive(Debug)]
pub enum ErrorKind {
    UnexpectedEof,
    UnexpectedIdentifier,
    UnexpectedTopLevelIndent,

    /// Similar to UnexpectedIdentifier - a required token wasn't found
    TokenRequirementNotFound(TokenKind),
    FunctionParameterDefaultsUnsupported,
    FunctionBlockNotFound,
    IndentedBlockNotFound,
}