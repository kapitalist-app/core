use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

/// Request to create a new transaction in the given wallet
#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionCreationRequest {
    /// Name of the new transaction
    pub name: String,
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
    #[serde(default)]
    /// The new name of the transaction or `None` if the name should remain unchanged
    pub name: Option<String>,
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

impl TransactionUpdateRequest {
    /// Checks whether the request is valid, i.e., at least one field is set
    pub fn is_valid(&self) -> bool {
        !(self.name.is_none()
            && self.wallet_id.is_none()
            && self.category_id.is_none()
            && self.amount.is_none()
            && self.ts.is_none())
    }
}
