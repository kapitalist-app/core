use serde::{Deserialize, Serialize};

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
    pub balance: Option<i64>,
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

impl WalletUpdateRequest {
    /// Checks whether the request is valid, i.e., at least one field is set
    pub fn is_valid(&self) -> bool {
        !(self.name.is_none() && self.wallet_type.is_none() && self.color.is_none())
    }
}
