use std::str::Chars;
use string_interner::backend::StringBackend;
use string_interner::StringInterner;
use string_interner::symbol::SymbolU32;
use crate::lexer::core::error::Error;
use crate::lexer::core::token::{Token, TokenKind};
use language::features::annotations::FEATURE_ANNOTATION;
use language::features::comments::FEATURE_COMMENT;
use language::features::strings::{FEATURE_SHORT_STRING, FEATURE_STRING};
use crate::lexer::core::token::TokenValue::Integer;
use crate::lexer::language::characters::{LC_CLOSE_CURLY_BRACKET, LC_CLOSE_ROUND_BRACKET, LC_CLOSE_SQUARE_BRACKET, LC_COLON, LC_COMMA, LC_OPEN_CURLY_BRACKET, LC_OPEN_ROUND_BRACKET, LC_OPEN_SQUARE_BRACKET};

pub mod core;
pub(crate) mod language;

pub struct Lexer<'a> {
    current_error: Error,
    current_token: Token,
    string_interner: StringInterner<StringBackend<SymbolU32>>,
    chars: Chars<'a>,
    chars_at_construct_time: Chars<'a>,
    source_length: usize,
    found_indent_for_current_line: bool,
}

impl<'a> Lexer<'a> {
    pub fn new(chars: Chars<'a>) -> Lexer {
        let source_length = chars.as_str().len();
        let chars_at_construct_time = chars.clone();
        Lexer {
            current_error: Error::empty(),
            current_token: Token::empty(),
            string_interner: StringInterner::default(),
            chars,
            chars_at_construct_time,
            source_length,
            found_indent_for_current_line: false,
        }
    }

    fn handle_line_break(&mut self) {
        self.next();
        self.found_indent_for_current_line = false;
    }

    /// Find and parse the next token from the input data
    pub fn parse(&mut self) -> bool {
        self.reset_error();
        self.reset_token();

        println!("{:?}", self.peek());

        // We need to handle line breaks / indents first
        match self.peek() {
            Some('\n' | '\r') => {
                self.handle_line_break();
                return false;
            }
            Some('\t' | ' ') if (!self.found_indent_for_current_line) => {
                self.indented_scope_depth();
                self.found_indent_for_current_line = true;
                return true;
            }
            Some(_) if (!self.found_indent_for_current_line) => {
                // Text instantly at the start of the newline - no indent
                self.set_token_kind(TokenKind::LanguageIndent)
                    .set_token_value(Integer(0));
                self.found_indent_for_current_line = true;
                return true;
            }
            _ => {}
        }

        match self.peek() {
            // Language features
            Some(FEATURE_ANNOTATION) => self.annotation(),
            Some(FEATURE_COMMENT) => self.comment(),
            Some(FEATURE_STRING | FEATURE_SHORT_STRING) => self.string_literal(),

            // Language core
            Some(LC_COLON) => {
                self.set_token_kind(TokenKind::Colon)
                    .single_token_here();
                self.next();
            }
            Some(LC_COMMA) => {
                self.set_token_kind(TokenKind::Comma)
                    .single_token_here();
                self.next();
            }
            Some(LC_OPEN_ROUND_BRACKET) => {
                self.set_token_kind(TokenKind::BracketRoundOpen)
                    .single_token_here();
                self.next();
            }
            Some(LC_CLOSE_ROUND_BRACKET) => {
                self.set_token_kind(TokenKind::BracketRoundClosed)
                    .single_token_here();
                self.next();
            }
            Some(LC_OPEN_SQUARE_BRACKET) => {
                self.set_token_kind(TokenKind::BracketSquareOpen)
                    .single_token_here();
                self.next();
            }
            Some(LC_CLOSE_SQUARE_BRACKET) => {
                self.set_token_kind(TokenKind::BracketSquareClosed)
                    .single_token_here();
                self.next();
            }
            Some(LC_OPEN_CURLY_BRACKET) => {
                self.set_token_kind(TokenKind::BracketCurlyOpen)
                    .single_token_here();
                self.next();
            }
            Some(LC_CLOSE_CURLY_BRACKET) => {
                self.set_token_kind(TokenKind::BracketCurlyClosed)
                    .single_token_here();
                self.next();
            }
            _ => {
                println!("Unknown character {:?}", self.peek());
                self.next();
            }
        }

        match self.current_token.kind {
            TokenKind::None => false,
            _ => true,
        }
    }
}