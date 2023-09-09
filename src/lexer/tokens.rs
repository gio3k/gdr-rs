pub enum TokenKind {
    Identifier,
    Comment,
    String,
    LongString,

    ScopeStart,
    ScopeEnd,

    SetStart,
    SetEnd,

    ArrayStart,
    ArrayEnd,

    ContainerStart,
    ContainerEnd,

    // and, &&
    ConditionAnd,

    // or, ||
    ConditionOr,
}

pub enum TokenValue {
    None,
    Float(f64),
    Integer(i64),
    String(String),
}

#[derive(Debug)]
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

    pub fn new_single(start: usize, kind: TokenKind) -> Self {
        Self {
            start,
            end: start + 1,
            kind,
            value: TokenValue::None,
        }
    }
}