use crate::ScriptLocation;

pub enum IssueKind {}

pub enum IssueSeverity {
    /// Tiny code / syntax issue - probably code style
    Suggestion,

    /// Code / syntax issue that doesn't make the script invalid but should be addressed
    Warning,

    /// Code / syntax issue that makes the script invalid
    Error,
}

pub struct Issue {
    pub kind: IssueKind,
    pub location: ScriptLocation,
}