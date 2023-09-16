use crate::ScriptLocation;
use crate::sponge::Sponge;

pub enum IssueKindSuggestion {}

pub enum IssueKindWarning {
    /// Tabs and spaces are used together
    IndentTypeMismatch
}

pub enum IssueKindError {}


pub enum IssueKind {
    /// Tiny code / syntax issue - probably code style
    Suggestion(IssueKindSuggestion),

    /// Code / syntax issue that doesn't make the script invalid but should be addressed
    Warning(IssueKindWarning),

    /// Code / syntax issue that makes the script invalid
    Error(IssueKindError),
}

pub struct Issue {
    pub kind: IssueKind,
    pub location: ScriptLocation,
}

impl Issue {
    pub fn suggestion(suggestion_kind: IssueKindSuggestion, location: ScriptLocation) -> Self {
        Self {
            kind: IssueKind::Suggestion(suggestion_kind),
            location,
        }
    }

    pub fn warning(warning_kind: IssueKindWarning, location: ScriptLocation) -> Self {
        Self {
            kind: IssueKind::Warning(warning_kind),
            location,
        }
    }

    pub fn error(error_kind: IssueKindError, location: ScriptLocation) -> Self {
        Self {
            kind: IssueKind::Error(error_kind),
            location,
        }
    }
}


impl<'a> Sponge<'a> {
    pub fn push_issue(&mut self, issue: Issue) {
        self.issues.push(issue)
    }
}