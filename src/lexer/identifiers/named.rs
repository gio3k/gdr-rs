use crate::error_here;
use crate::lexer::{Lexer, LexerError};
use crate::lexer::identifiers::is_valid_identifier_body;
use crate::lexer::tokens::{TokenKind, TokenValue};

impl<'a> Lexer<'a> {
    pub(crate) fn identify_multi_char_named(&mut self) -> Result<(), LexerError> {
        let start = self.offset();

        loop {
            match self.see() {
                Some(c) if !is_valid_identifier_body(c) => {
                    break;
                }
                None => break,
                _ => {}
            }
            self.advance();
        }

        let end = self.offset();

        if start == end {
            // We shouldn't be able to get here at all unless something's really wrong
            return error_here!(self, IdentifierAllowedCharacterMismatch);
        }

        // Identify the token by string
        let string: String = self.view(start, end).collect();
        match string.as_str() {
            "var" => self.insert_token(start, end, TokenKind::Var)?,
            "const" => self.insert_token(start, end, TokenKind::Const)?,
            "func" => self.insert_token(start, end, TokenKind::Function)?,
            "if" => self.insert_token(start, end, TokenKind::If)?,
            "else" => self.insert_token(start, end, TokenKind::Else)?,
            "elif" => self.insert_token(start, end, TokenKind::ElseIf)?,
            "match" => self.insert_token(start, end, TokenKind::Match)?,
            "for" => self.insert_token(start, end, TokenKind::For)?,
            "in" => self.insert_token(start, end, TokenKind::In)?,
            "while" => self.insert_token(start, end, TokenKind::While)?,
            "return" => self.insert_token(start, end, TokenKind::Return)?,
            "not" => self.insert_token(start, end, TokenKind::Not)?,
            "null" => self.insert_token(start, end, TokenKind::NullLiteral)?,
            "false" => self.insert_token_data(start, end, TokenKind::BooleanLiteral, TokenValue::Boolean(false))?,
            "true" => self.insert_token_data(start, end, TokenKind::BooleanLiteral, TokenValue::Boolean(true))?,
            "preload" => self.insert_token(start, end, TokenKind::LanguagePreload)?,

            _ => self.insert_string_token(start, end, TokenKind::Identifier)?,
        }

        self.advance();
        Ok(())
    }
}