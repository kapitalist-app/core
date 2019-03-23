use serde::{Deserialize, Serialize};

/// Generic error response with an error message indicating what went wrong
///
/// Clients should use the error message in combination with the HTTP return type to determine how
/// to handle the error.
#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    /// Message describing the error
    pub error: String,
}

impl ErrorResponse {
    /// Construct a new `ErrorResponse` from a given error message
    pub fn new<S>(err: S) -> Self
    where
        S: Into<String>,
    {
        Self { error: err.into() }
    }

    /// Utility function to return a generic "Internal server error"
    ///
    /// This is usually used in production to hide potentially sensitive error details.
    pub fn internal_server_error() -> Self {
        Self::new("Internal server error")
    }

    /// Utility function to return a generic "Not implemented yet" error
    pub fn not_implemented() -> Self {
        Self::new("Not implemented yet")
    }

    /// Utility function to return a generic "Unauthorized" error
    pub fn unauthorized() -> Self {
        Self::new("Unauthorized")
    }
}
