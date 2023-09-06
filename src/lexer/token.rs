#[derive(Debug)]
pub enum ScriptToken {
    Identifier(Vec<char>),
    String(Vec<char>),
    NodePath(Vec<char>),
    Comment(Vec<char>, u8),
    Annotation(Vec<char>),
    FuncOrTypeHint(),

    // Array / dictionary delimiter
    DataDelimiter(),

    // Parent-child / Members
    ExpressionDelimiter(),

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

pub fn is_char_token(c: char) -> bool {
    match c {
        '(' | ')' => true,
        '{' | '}' => true,
        '[' | ']' => true,
        ':' => true,
        '@' => true,
        '"' => true,
        '#' => true,
        '.' => true,
        ',' => true,
        _ => false
    }
}