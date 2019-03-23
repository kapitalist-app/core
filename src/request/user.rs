use serde::{Deserialize, Serialize};

/// Request to create a new user account
#[derive(Debug, Serialize, Deserialize)]
pub struct UserCreationRequest {
    /// Email address that the user entered
    pub email: String,
    /// Password that the user entered
    pub password: String,
    /// The name that the user entered
    ///
    /// May be `None` in which case the email is used as the name.
    #[serde(default)]
    pub name: Option<String>,
}

/// Request to update an existing user account
///
/// All fields are optional and a `None` value indicates to the backend that this field should not
/// be changed. However at least one value has to be set for the `UserUpdateRequest` to succeed.
///
/// Additionally all fields are marked with `#[serde(default)]` to allow for deserialization from
/// clients that leave out a field if its value is `None`.
#[derive(Debug, Serialize, Deserialize)]
pub struct UserUpdateRequest {
    /// The new email address of the user or `None` if the email address should remain unchanged
    #[serde(default)]
    pub email: Option<String>,
    /// The new password of the user or `None` if the password should remain unchanged
    #[serde(default)]
    pub password: Option<String>,
    /// The new name of the user or `None` if the name should remain unchanged
    #[serde(default)]
    pub name: Option<String>,
}

/// Request for an authentication token
///
/// As the backend uses tokens for all API resources a user must first obtain a token from email
/// and password before performing any action.
#[derive(Debug, Serialize, Deserialize)]
pub struct TokenRequest {
    /// The email address of the user that wishes to authenticate
    pub email: String,
    /// The password of the user that wishes to authenticate
    pub password: String,
}

impl UserCreationRequest {
    /// Construct a new `UserCreationRequest` with all fields set
    pub fn new(email: String, password: String, name: Option<String>) -> Self {
        Self {
            email,
            password,
            name,
        }
    }

    /// Construct a new `UserCreationRequest` without giving a name
    pub fn without_name(email: String, password: String) -> Self {
        Self::new(email, password, None)
    }
}

impl UserUpdateRequest {
    /// Checks whether the request is valid, i.e., at least one field is set
    pub fn is_valid(&self) -> bool {
        !(self.email.is_none() && self.password.is_none() && self.name.is_none())
    }
}
