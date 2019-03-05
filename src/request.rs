//! Requests that are sent to the backend
//!
//! All requests derive both `serde::Serialize` and `serde::Deserialize` to allow conversion from
//! and to JSON in both backend and frontends.

use chrono::NaiveDateTime;
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

/// Request to create a new wallet
#[derive(Debug, Serialize, Deserialize)]
pub struct WalletCreationRequest {
    /// Name of the wallet to create
    pub name: String,
    /// Type of the wallet to create
    ///
    /// Currently this is a `String` but it might be changed to an `enum` in the future.
    pub wallet_type: String,
    /// Initial balance of the wallet to create
    pub balance: i64,
    /// (Background) Color of the wallet to create
    ///
    /// May be `None` in which case a random color is assigned
    pub color: Option<String>,
}

/// Request to update an existing wallet
///
/// All fields are optional and a `None` value indicates to the backend that this field should not
/// be changed. However at least one value has to be set for the `UserUpdateRequest` to succeed.
///
/// Additionally all fields are marked with `#[serde(default)]` to allow for deserialization from
/// clients that leave out a field if its value is `None`.
#[derive(Debug, Serialize, Deserialize)]
pub struct WalletUpdateRequest {
    /// The new name of the wallet or `None` if the name should remain unchanged
    #[serde(default)]
    pub name: Option<String>,
    /// The new type of the wallet or `None` if the type should remain unchanged
    #[serde(default)]
    pub wallet_type: Option<String>,
    /// The new color of the wallet or `None` if the color should remain unchanged
    #[serde(default)]
    pub color: Option<String>,
}

/// Request to create a new transaction category
#[derive(Debug, Serialize, Deserialize)]
pub struct CategoryCreationRequest {
    /// Name of the category to create
    pub name: String,
    /// Id of the parent category that should contain the new category or `None` if the new
    /// category should be a root category
    #[serde(default)]
    pub parent_id: Option<i64>,
    /// The color of the category to create
    pub color: String,
}

/// Request to update an existing transaction category
///
/// All fields are optional and a `None` value indicates to the backend that this field should not
/// be changed. However at least one value has to be set for the `UserUpdateRequest` to succeed.
///
/// Additionally all fields are marked with `#[serde(default)]` to allow for deserialization from
/// clients that leave out a field if its value is `None`.
#[derive(Debug, Serialize, Deserialize)]
pub struct CategoryUpdateRequest {
    /// The new name of the category or `None` if the name should remain unchanged
    #[serde(default)]
    pub name: Option<String>,
    /// The new id of the parent of the category or `None` if the parent category should remain unchanged
    ///
    /// To make the category a root category set this field to `Some(None)`.
    #[serde(default)]
    pub parent_id: Option<Option<i64>>,
    /// The new color of the category or `None` if the color should remain unchanged
    #[serde(default)]
    pub color: Option<String>,
}

/// Request to create a new transaction in the given wallet
#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionCreationRequest {
    /// Id of the wallet that the new transaction belongs to
    pub wallet_id: i64,
    /// Id of the category that the new transaction belongs to
    pub category_id: i64,
    /// Currency amount that this transaction represents
    ///
    /// As with all balances in Kapitalist the unit of this field should be the smallest in the
    /// currency e.g. Cent for Euro and Dollar.
    pub amount: i64,
    /// Timestamp of the new transaction or `None` if the current time should be used
    #[serde(default)]
    pub ts: Option<NaiveDateTime>,
}

/// Request to update an existing transaction
///
/// All fields are optional and a `None` value indicates to the backend that this field should not
/// be changed. However at least one value has to be set for the `UserUpdateRequest` to succeed.
///
/// Additionally all fields are marked with `#[serde(default)]` to allow for deserialization from
/// clients that leave out a field if its value is `None`.
#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionUpdateRequest {
    /// The new Id of the wallet the transaction belongs to or `None` if the Id should remain
    /// unchanged
    #[serde(default)]
    pub wallet_id: Option<i64>,
    /// The new Id of the category the transaction belongs to or `None` if the Id should remain
    /// unchanged
    #[serde(default)]
    pub category_id: Option<i64>,
    /// The new currency amount the transaction represents or `None` if the amount should remain
    /// unchanged
    #[serde(default)]
    pub amount: Option<i64>,
    /// The new timestamp of the transaction or `None` if the timestamp should remain unchanged
    #[serde(default)]
    pub ts: Option<NaiveDateTime>,
}

impl UserUpdateRequest {
    /// Checks whether the request is valid, i.e., at least one field is set
    pub fn is_valid(&self) -> bool {
        !(self.email.is_none() && self.password.is_none() && self.name.is_none())
    }
}

impl WalletUpdateRequest {
    /// Checks whether the request is valid, i.e., at least one field is set
    pub fn is_valid(&self) -> bool {
        !(self.name.is_none() && self.wallet_type.is_none() && self.color.is_none())
    }
}

impl CategoryUpdateRequest {
    /// Checks whether the request is valid, i.e., at least one field is set
    pub fn is_valid(&self) -> bool {
        !(self.name.is_none() && self.parent_id.is_none() && self.color.is_none())
    }
}

impl TransactionUpdateRequest {
    /// Checks whether the request is valid, i.e., at least one field is set
    pub fn is_valid(&self) -> bool {
        !(self.wallet_id.is_none()
            && self.category_id.is_none()
            && self.amount.is_none()
            && self.ts.is_none())
    }
}
