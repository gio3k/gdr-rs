use string_interner::symbol::SymbolU32;

#[derive(Debug, Copy, Clone)]
pub enum Literal {
    None,
    Float(f64),
    Integer(i64),
    Symbol(SymbolU32),
    Boolean(bool),
}

impl From<f64> for Literal {
    fn from(value: f64) -> Self {
        Literal::Float(value)
    }
}

impl From<i64> for Literal {
    fn from(value: i64) -> Self {
        Literal::Integer(value)
    }
}

impl From<SymbolU32> for Literal {
    fn from(value: SymbolU32) -> Self {
        Literal::Symbol(value)
    }
}

impl From<bool> for Literal {
    fn from(value: bool) -> Self {
        Literal::Boolean(value)
    }
}