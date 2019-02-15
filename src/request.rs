use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserCreationRequest {
    pub email: String,
    pub password: String,
    #[serde(default)]
    pub name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserUpdateRequest {
    #[serde(default)]
    pub email: Option<String>,
    #[serde(default)]
    pub password: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WalletCreationRequest {
    pub name: String,
    pub wallet_type: String,
    pub balance: i64,
    pub color: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WalletUpdateRequest {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub wallet_type: Option<String>,
    #[serde(default)]
    pub color: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CategoryCreationRequest {
    pub name: String,
    #[serde(default)]
    pub parent_id: Option<i64>,
    pub color: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CategoryUpdateRequest {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub parent_id: Option<Option<i64>>,
    #[serde(default)]
    pub color: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionCreationRequest {
    pub wallet_id: i64,
    pub category_id: i64,
    pub amount: i64,
    #[serde(default)]
    pub ts: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionUpdateRequest {
    #[serde(default)]
    pub wallet_id: Option<i64>,
    #[serde(default)]
    pub category_id: Option<i64>,
    #[serde(default)]
    pub amount: Option<i64>,
    #[serde(default)]
    pub ts: Option<NaiveDateTime>,
}
