// Token handling for the lexer
use string_interner::symbol::SymbolU32;
use crate::lexer::ScriptLexer;
use crate::ScriptLocation;

#[derive(Debug, Copy, Clone)]
pub enum TokenKind {
    None,
    Identifier,
    Unknown,
    LineBreak,

    // Indents
    IndentSpaces,
    IndentTab,

    // Core Language Tokens
    Colon,
    Period,
    Comma,

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
    Comment,
    Annotation,
    Preload,
    TypeArrow,

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
    Symbol(SymbolU32),
    Boolean(bool),
}

#[derive(Debug, Copy, Clone)]
pub struct Token {
    pub location: ScriptLocation,
    pub kind: TokenKind,
    pub value: TokenValue,
}

impl Token {
    /// Create an empty token used for internal purposes
    pub(crate) fn empty() -> Self {
        Self {
            location: ScriptLocation { start: 0, end: 0 },
            kind: TokenKind::None,
            value: TokenValue::None,
        }
    }

    pub fn with_symbol_value(&mut self, value: SymbolU32) -> &mut Token {
        self.value = TokenValue::Symbol(
            value
        );
        self
    }

    pub fn with_symbol_from(&mut self, lexer: &mut ScriptLexer) -> &mut Token {
        let data = lexer.script.slice_to_string(self.location);
        let symbol = lexer.cache_string(data);
        self.value = TokenValue::Symbol(
            symbol
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

/// Panic unless the current character matches the pattern.
/// This should only be used to make sure there aren't issues with the way the
/// lexer passes from function to function - don't actually use for user code
/// issues!
#[macro_export]
macro_rules! assert_token_kind {
    ($token:expr, $pattern:pat $(if $guard:expr)? $(,)?) => {
        match $token.kind {
            $pattern $(if $guard)? => {}
            _ => {
                panic!("Unexpected token kind {:?}", $token.kind);
            }
        }
    };
}

#[macro_export]
macro_rules! assert_token_kind_not {
    ($token:expr, $pattern:pat $(if $guard:expr)? $(,)?) => {
        match $token.kind {
            $pattern $(if $guard)? => {
                panic!("Unexpected token kind {:?}", $token.kind);
            }
            _ => {}
        }
    };
}

#[macro_export]
macro_rules! assert_token_value {
    ($token:expr, $pattern:pat $(if $guard:expr)? $(,)?) => {
        match $token.value {
            $pattern $(if $guard)? => {}
            _ => {
                panic!("Unexpected token value {:?}", $token.value);
            }
        }
    };
}

macro_rules! token_value_cast {
    ($token:expr, $token_value_type:ident) => {
        match $token.value {
            TokenValue::$token_value_type(v) => v,
            _ =>
        }
    };
}

impl<'a> ScriptLexer<'a> {
    /// Returns whether or not the token kind is None
    pub fn has_token(&self) -> bool {
        !matches!(self.current_token.kind, TokenKind::None)
    }

    /// End the token here (current iterator position), with the token having the provided size
    pub(crate) fn end_token_here_with_size(&mut self, size: usize) -> &mut Self {
        let end = self.offset();
        self.current_token.location.end = end;
        self.current_token.location.start = end - (size - 1);
        self
    }

    /// End the token here (current iterator position), with the token starting at the provided value
    pub(crate) fn end_token_here(&mut self, start: usize) -> &mut Self {
        let end = self.offset();
        self.current_token.location.end = end;
        self.current_token.location.start = start;
        self
    }

    /// End the token here (current iterator position) as a 1 character token
    pub(crate) fn single_token_here(&mut self) -> &mut Self {
        self.end_token_here_with_size(1)
    }

    /// Set the token position / bounds
    pub(crate) fn set_token_pos(&mut self, location: ScriptLocation) -> &mut Self {
        self.current_token.location = location;
        self
    }

    /// Set the token position / bounds start
    pub(crate) fn set_token_start(&mut self, start: usize) -> &mut Self {
        self.current_token.location.start = start;
        self
    }

    /// Set the token position / bounds end
    pub(crate) fn set_token_end(&mut self, end: usize) -> &mut Self {
        self.current_token.location.end = end;
        self
    }

    /// Set the token kind
    pub(crate) fn set_token_kind(&mut self, kind: TokenKind) -> &mut Self {
        self.current_token.kind = kind;
        self
    }

    /// Set the token value
    pub(crate) fn set_token_value(&mut self, value: TokenValue) -> &mut Self {
        self.current_token.value = value;
        self
    }

    /// Make the token value a string based on the token bounds
    pub(crate) fn make_token_symbol(&mut self) -> &mut Self {
        let data = self.script.slice_to_string(self.current_token.location);
        let symbol = self.cache_string(data);
        self.current_token.value = TokenValue::Symbol(
            symbol
        );
        self
    }

    /// Prepare the token state for the next iteration
    pub(crate) fn reset_token(&mut self) {
        self.current_token.kind = TokenKind::None;
        self.current_token.value = TokenValue::None;
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