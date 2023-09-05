#[derive(Debug)]
pub enum Token {
    Identifier(Vec<char>),
    String(Vec<char>),
    Comment(Vec<char>, u8),
    Annotation(Vec<char>),
    FuncOrTypeHint(),

    // Operators
    EqualityOperator(),
    GreaterThanOperator(),
    GreaterThanOrEqualToOperator(),
    LowerThanOperator(),
    LowerThanOrEqualToOperator(),
    MultiplyOperator(),
    AddOperator(),
    NegateOrSubtractOperator(),
    DivideOperator(),
    ModuloOperator(),
    SqrtOperator(),

    // Depth
    ScopeDepth(u8),
    ScopeDepthIncrease(),
    ScopeDepthDecrease(),

    // Set ()
    SetStart(),
    SetEnd(),

    // Array []
    ArrayStart(),
    ArrayEnd(),
}