use string_interner::symbol::SymbolU32;

#[derive(Debug, Copy, Clone)]
pub enum Literal {
    None,
    Float(f64),
    Integer(i64),
    Symbol(SymbolU32),
    Boolean(bool),
}