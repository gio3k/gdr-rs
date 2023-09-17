use crate::core::literal::Literal;
use crate::script::Location;
use crate::stage0::ScriptLexer;

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
    Pass,
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
pub struct Token {
    pub location: Location,
    pub kind: TokenKind,
    pub value: Literal,
}

impl Token {
    /// Create an empty token used for internal purposes
    pub(crate) fn empty() -> Self {
        Self {
            location: Location { start: 0, end: 0 },
            kind: TokenKind::None,
            value: Literal::None,
        }
    }

    pub fn with_symbol_from(&mut self, lexer: &mut ScriptLexer) -> &mut Token {
        let data = lexer.script.slice_to_string(self.location);
        let symbol = lexer.cache_string(data);
        self.value = Literal::Symbol(
            symbol
        );
        self
    }

    pub fn with_value<T>(&mut self, value: T) -> &mut Token
        where Literal: From<T>
    {
        self.value = Literal::from(value);
        self
    }
}

/// Panic unless the token kind matches the pattern.
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

/// Panic if the token kind matches the pattern.
/// This should only be used to make sure there aren't issues with the way the
/// lexer passes from function to function - don't actually use for user code
/// issues!
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

/// Panic unless the token value matches the pattern.
/// This should only be used to make sure there aren't issues with the way the
/// lexer passes from function to function - don't actually use for user code
/// issues!
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

#[macro_export]
macro_rules! cast_token_value {
    ($token:expr, $token_value_type:ident) => {
        match $token.value {
            Literal::$token_value_type(v) => v,
            _ =>
        }
    };
}