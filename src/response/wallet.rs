use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

/// Response containing details of a Wallet
#[derive(Debug, Serialize, Deserialize)]
pub struct WalletResponse {
    /// (Database) Id of the Wallet
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
