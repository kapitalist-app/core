//! Requests that are sent to the backend
//!
//! All requests derive both `serde::Serialize` and `serde::Deserialize` to allow conversion from
//! and to JSON in both backend and frontends.

mod category;
mod transaction;
mod user;
mod wallet;

pub use crate::request::category::*;
pub use crate::request::transaction::*;
pub use crate::request::user::*;
pub use crate::request::wallet::*;
