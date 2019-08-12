use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

/// Response containing details of a Transaction
#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionResponse {
    /// (Database) Id of the Transaction
    ///
    /// Clients need this to reference the Transaction in a request
    pub id: i64,
    /// Human readable name of the Transaction
    pub name: String,
    /// (Database) Id of the Wallet this transaction belongs to
    pub wallet_id: i64,
    /// (Database) Id of the Category this transaction belongs to
    pub category_id: i64,
    /// Amount of money the Transaction represents (in the smallest currency unit)
    pub amount: i64,
    /// Timestamp when the Transaction was/will be executed
    pub ts: NaiveDateTime,
}
