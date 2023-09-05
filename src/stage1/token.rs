#[derive(Debug)]
pub enum ScriptToken {
    Identifier(Vec<char>),
    String(Vec<char>),
    Comment(Vec<char>, u8),
    Annotation(Vec<char>),
    FuncOrTypeHint(),

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