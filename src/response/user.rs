use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

/// Response containing the requested authentication token
#[derive(Debug, Serialize, Deserialize)]
pub struct TokenResponse {
    /// Authentication token
    pub token: String,
}

/// Response containing details of a User
#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponse {
    /// User's current email address
    pub email: String,
    /// User's current username
    pub username: String,
    /// Creation timestamp of the user's account
    pub created_at: NaiveDateTime,
}
