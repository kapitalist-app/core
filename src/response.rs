use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct VersionResponse {
    pub version: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenResponse {
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    error: String,
}

impl ErrorResponse {
    pub fn new<S>(err: S) -> ErrorResponse
    where
        S: Into<String>,
    {
        ErrorResponse { error: err.into() }
    }

    pub fn internal_server_error() -> ErrorResponse {
        ErrorResponse::new("Internal server error")
    }

    pub fn not_implemented() -> ErrorResponse {
        ErrorResponse::new("Not implemented yet")
    }

    pub fn unauthorized() -> ErrorResponse {
        ErrorResponse::new("Unauthorized")
    }
}
