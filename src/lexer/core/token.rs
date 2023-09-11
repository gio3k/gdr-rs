// Token handling for the lexer

use string_interner::symbol::SymbolU32;
use crate::lexer::Lexer;

#[derive(Debug, Copy, Clone)]
pub enum TokenKind {
    None,
    Identifier,

    // Literals
    FloatLiteral,
    IntegerLiteral,
    StringLiteral,
    BooleanLiteral,
    NullLiteral,

    // Comparisons
    ComparisonGreaterThan,
    ComparisonGreaterThanOrEqualTo,

    ComparisonLesserThan,
    ComparisonLesserThanOrEqualTo,

    ComparisonEqualTo,
    ComparisonNotEqualTo,

    ComparisonAnd,
    ComparisonOr,

    BitwiseNot,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    BitwiseTargetedNot,
    BitwiseTargetedAnd,
    BitwiseTargetedOr,
    BitwiseTargetedXor,
    BitwiseLeftShift,
    BitwiseRightShift,

    // Math and Operations
    NegateExpression,
    Assignment,

    MathAdd,
    MathSubtract,
    MathDivide,
    MathMultiply,
    MathModulo,
    MathTargetedAdd,
    MathTargetedSubtract,
    MathTargetedDivide,
    MathTargetedMultiply,
    MathTargetedModulo,
    MathIncrement,
    MathDecrement,

    // Language words / statements
    Var,
    Const,
    Function,
    If,
    Else,
    ElseIf,
    Match,
    For,
    In,
    While,
    Return,
    Not,

    // Core Language Features
    LanguageComment,
    LanguageAnnotation,
    LanguagePreload,
    LanguageTypeArrow,
    LanguageIndent,

    // Core Language Tokens
    Colon,
    Period,
    Comma,

    // Brackets
    BracketRoundOpen,
    BracketRoundClosed,
    BracketSquareOpen,
    BracketSquareClosed,
    BracketCurlyOpen,
    BracketCurlyClosed,
}

#[derive(Debug, Copy, Clone)]
pub enum TokenValue {
    None,
    Float(f64),
    Integer(i64),
    String(SymbolU32),
    Boolean(bool),
}

#[derive(Debug, Copy, Clone)]
pub struct Token {
    pub start: usize,
    pub end: usize,
    pub kind: TokenKind,
    pub value: TokenValue,
}

impl Token {
    pub fn new(start: usize, end: usize, kind: TokenKind) -> Self {
        Self {
            start,
            end,
            kind,
            value: TokenValue::None,
        }
    }

    pub fn single(offset: usize, kind: TokenKind) -> Self {
        Self {
            start: offset,
            end: offset + 1,
            kind,
            value: TokenValue::None,
        }
    }

    /// Create an empty token used for internal purposes
    pub(crate) fn empty() -> Self {
        Self {
            start: 0,
            end: 0,
            kind: TokenKind::None,
            value: TokenValue::None,
        }
    }

    pub fn with_string_value(&mut self, value: SymbolU32) -> &mut Token {
        self.value = TokenValue::String(
            value
        );
        self
    }

    pub fn with_string_from(&mut self, lexer: &mut Lexer) -> &mut Token {
        self.value = TokenValue::String(
            lexer.slice_to_string_symbol(self.start, self.end)
        );
        self
    }

    pub fn with_int_value(&mut self, value: i64) -> &mut Token {
        self.value = TokenValue::Integer(
            value
        );
        self
    }

    pub fn with_float_value(&mut self, value: f64) -> &mut Token {
        self.value = TokenValue::Float(
            value
        );
        self
    }

    pub fn with_bool_value(&mut self, value: bool) -> &mut Token {
        self.value = TokenValue::Boolean(
            value
        );
        self
    }
}

impl<'a> Lexer<'a> {
    /// Set the token state to the provided token data
    pub(crate) fn set_token(&mut self, token: &Token) {
        self.current_token = *token;
    }

    /// Prepare the token state for the next iteration
    pub(crate) fn reset_token(&mut self) {
        self.current_token.kind = TokenKind::None;
    }

    /// Get the current token state
    pub fn token(&self) -> Option<&Token> {
        return match self.current_token.kind {
            TokenKind::None => None,
            _ => Some(&self.current_token)
        };
    }
}

/// Runs a match loop - matches then advances until the loop is broken
#[macro_export]
macro_rules! read {
    ($self:ident, $($pattern:pat $(if $guard:expr)* => $action:expr),*) => {
        loop {
            match $self.peek() {
                $($pattern $(if $guard)* => $action),*
            }
            $self.next();
        }
    };
}