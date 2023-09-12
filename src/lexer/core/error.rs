// Error handling for the lexer

use crate::lexer::Lexer;

#[derive(Debug, Copy, Clone)]
pub enum ErrorKind {
    None,
    UnexpectedCurrentCharacter,
    UnexpectedIndentTypeMismatch,
    UnexpectedEndOfFile,
    UnexpectedLineBreak,
    UnexpectedCharacter,
}

#[derive(Debug, Copy, Clone)]
pub enum ErrorRecovery {
    /// Error is unrecoverable and the file can no longer be parsed
    Unrecoverable,

    /// Error is recoverable - amount of characters to skip is provided
    Recoverable(usize),
}

#[derive(Debug, Copy, Clone)]
pub struct Error {
    recovery: ErrorRecovery,
    pub kind: ErrorKind,
}

impl Error {
    /// Create an error that allows recovery from it
    pub fn recoverable(kind: ErrorKind, characters_to_skip: usize) -> Self {
        Self {
            recovery: ErrorRecovery::Recoverable(characters_to_skip),
            kind,
        }
    }

    /// Create an error incompatible with error recovery
    pub fn unrecoverable(kind: ErrorKind) -> Self {
        Self {
            recovery: ErrorRecovery::Unrecoverable,
            kind,
        }
    }

    /// Create an empty error used for internal purposes
    pub(crate) fn empty() -> Self {
        Self {
            recovery: ErrorRecovery::Unrecoverable,
            kind: ErrorKind::None,
        }
    }
}

impl<'a> Lexer<'a> {
    /// Set the error state to the provided error
    pub(crate) fn set_error(&mut self, error: Error) {
        self.current_error = error;
        match self.current_error.recovery {
            ErrorRecovery::Unrecoverable => {
                // Just panic on unrecoverable errors so we can see the stack trace
                panic!("set_error: lexer encountered unrecoverable error")
            }
            ErrorRecovery::Recoverable(_) => {}
        }
    }

    /// Returns whether or not the error kind is None
    pub fn has_error(&self) -> bool {
        !matches!(self.current_error.kind, ErrorKind::None)
    }

    /// Prepare the error state for the next iteration
    pub(crate) fn reset_error(&mut self) {
        self.current_error.kind = ErrorKind::None;
    }

    /// Get the current error state
    pub fn error(&self) -> Option<&Error> {
        return match self.current_error.kind {
            ErrorKind::None => None,
            _ => Some(&self.current_error)
        };
    }
}

/// Set an error unless the character under the iterator matches the provided pattern
#[macro_export]
macro_rules! set_error_unless {
    ($self:ident, $error:expr, $pattern:pat $(if $guard:expr)? $(,)?) => {
        match $self.peek() {
            $pattern $(if $guard)? => {},
            _ => $self.set_error($error)
        }
    };
}

/// Set an error if the character under the iterator matches the provided pattern
#[macro_export]
macro_rules! set_error_when {
    ($self:ident, $error:expr, $pattern:pat $(if $guard:expr)? $(,)?) => {
        match $self.peek() {
            $pattern $(if $guard)? => $self.set_error($error),
            _ => {}
        }
    };
}