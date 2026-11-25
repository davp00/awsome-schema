//! Errors returned by the shared Surreal client runtime.

/// Failure from SQL construction or a Surreal call.
#[derive(Debug, thiserror::Error, Clone, PartialEq, Eq)]
#[error("{message}")]
pub struct ClientError {
    message: String,
}

impl ClientError {
    /// Build an error from a displayable message.
    #[must_use]
    pub fn new(message: impl Into<String>) -> Self {
        Self { message: message.into() }
    }

    /// Borrow the error text.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }
}

pub(crate) fn err(message: impl Into<String>) -> ClientError {
    ClientError::new(message)
}
