#[derive(Debug)]
pub enum ScriptToken {
    Identifier(Vec<char>),
    String(Vec<char>),
    NodePath(Vec<char>),
    Comment(Vec<char>, u8),
    Annotation(Vec<char>),
    FuncOrTypeHint(),

    // Parent-child / Members
    MemberHint(),

    // Depth
    ScopeDepth(u8),

    // Dictionary {}
    DictStart(),
    DictEnd(),

    // Set ()
    SetStart(),
    SetEnd(),

    // Array []
    ArrayStart(),
    ArrayEnd(),
}