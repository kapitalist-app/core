use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

/// Response containing details of a Category
#[derive(Debug, Serialize, Deserialize)]
pub struct CategoryResponse {
    /// (Database) Id of the Category
    ///
    /// Clients need this to reference the Wallet in a request
    pub id: i64,
    /// (Database) Id of the parent Category (may be `None`)
    pub parent_id: Option<i64>,
    /// (Database) Id of the User which created the Category or `None` if it is a default Category
    pub user_id: Option<i64>,
    /// Name of the Category
    pub name: String,
    /// Color of the Category. Mainly used for charts and reports
    pub color: String,
    /// Timestamp when the Category was created
    pub created_at: NaiveDateTime,
}
