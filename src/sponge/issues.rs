use crate::ScriptLocation;
use crate::sponge::Sponge;

#[derive(Debug)]
pub enum IssueKindSuggestion {}

#[derive(Debug)]
pub enum IssueKindWarning {
    /// Tabs and spaces are used together
    IndentTypeMismatch
}

#[derive(Debug)]
pub enum IssueKindError {}


#[derive(Debug)]
pub enum IssueKind {
    /// Tiny code / syntax issue - probably code style
    Suggestion(IssueKindSuggestion),

    /// Code / syntax issue that doesn't make the script invalid but should be addressed
    Warning(IssueKindWarning),

    /// Code / syntax issue that makes the script invalid
    Error(IssueKindError),
}

#[derive(Debug)]
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
        println!("Pushed issue {:?}", issue);
        self.issues.push(issue)
    }
}