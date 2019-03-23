use serde::{Deserialize, Serialize};

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

impl CategoryUpdateRequest {
    /// Checks whether the request is valid, i.e., at least one field is set
    pub fn is_valid(&self) -> bool {
        !(self.name.is_none() && self.parent_id.is_none() && self.color.is_none())
    }
}
