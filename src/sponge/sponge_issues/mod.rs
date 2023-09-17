use crate::script::Location;
use crate::sponge::Sponge;
use crate::sponge::sponge_issues::error_kind::ErrorKind;
use crate::sponge::sponge_issues::suggestion_kind::SuggestionKind;
use crate::sponge::sponge_issues::warning_kind::WarningKind;

pub mod error_kind;
pub mod warning_kind;
pub mod suggestion_kind;

#[derive(Debug)]
pub enum IssueKind {
    Error(ErrorKind),
    Warning(WarningKind),
    Suggestion(SuggestionKind),
}

#[derive(Debug)]
pub struct SpongeIssue {
    kind: IssueKind,
    location: Location,
}

impl SpongeIssue {
    pub fn new(kind: IssueKind, location: Location) -> Self {
        Self {
            kind,
            location,
        }
    }

    pub fn error(kind: ErrorKind, location: Location) -> Self {
        Self {
            kind: IssueKind::Error(kind),
            location,
        }
    }

    pub fn warning(kind: WarningKind, location: Location) -> Self {
        Self {
            kind: IssueKind::Warning(kind),
            location,
        }
    }

    pub fn suggestion(kind: SuggestionKind, location: Location) -> Self {
        Self {
            kind: IssueKind::Suggestion(kind),
            location,
        }
    }
}

impl<'a> Sponge<'a> {
    pub(crate) fn throw(&mut self, issue: SpongeIssue) {
        println!("throw - {:?}", issue);
        self.issues.push(issue);
    }

    pub(crate) fn throw_here(&mut self, kind: IssueKind) {
        self.throw(SpongeIssue::new(kind, self.token.location))
    }

    pub(crate) fn throw_error_here(&mut self, kind: ErrorKind) {
        self.throw(SpongeIssue::error(kind, self.token.location))
    }

    pub(crate) fn throw_warning_here(&mut self, kind: WarningKind) {
        self.throw(SpongeIssue::warning(kind, self.token.location))
    }

    pub(crate) fn throw_suggestion_here(&mut self, kind: SuggestionKind) {
        self.throw(SpongeIssue::suggestion(kind, self.token.location))
    }
}