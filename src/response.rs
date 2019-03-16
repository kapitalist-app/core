//! Response types that are returned from the backend.
//!
//! All responses derive both `serde::Serialize` and `serde::Deserialize` to allow conversion from
//! and to JSON in both backend and frontends.

// XXX: Include more responses that currently live in the backend

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

/// Response containing the version of the backend
///
/// Clients usually use this to conditionally enable a feature if the backend's version is high
/// enough to support it.
#[derive(Debug, Serialize, Deserialize)]
pub struct VersionResponse {
    /// Version of the backend in the form MAJOR.MINOR.PATCH
    pub version: String,
}

/// Response containing the requested authentication token
#[derive(Debug, Serialize, Deserialize)]
pub struct TokenResponse {
    /// Authentication token
    pub token: String,
}

/// Response containing details of a User
#[derive(Debug, Deserialize, Serialize)]
pub struct UserResponse {
    /// User's current email address
    pub email: String,
    /// User's current username
    pub username: String,
    /// Creation timestamp of the user's account
    pub created_at: NaiveDateTime,
}

/// Response containing details of a Wallet
#[derive(Debug, Deserialize, Serialize)]
pub struct WalletResponse {
    /// (Database) Id of the wallet
    ///
    /// Clients need this to reference the Wallet in a request
    pub id: i64,
    /// User chosen name of the Wallet
    pub name: String,
    // XXX: Make this an enum
    /// Type of the Wallet, e.g., credit-card, checking account etc.
    pub wallet_type: String,
    /// Current balance of the Wallet
    pub current_balance: i64,
    /// User chosen color of the wallet to represent it in client applications
    pub color: String,
    /// Timestamp when the Wallet was created
    pub created_at: NaiveDateTime,
}

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
