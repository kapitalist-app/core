//! Responses that are returned by the backend.
//!
//! All responses derive both `serde::Serialize` and `serde::Deserialize` to allow conversion from
//! and to JSON in both backend and frontends.

// XXX: Include more responses that currently live in the backend

mod error;

mod category;
mod user;
mod wallet;

pub use crate::response::error::*;

pub use crate::response::category::*;
pub use crate::response::user::*;
pub use crate::response::wallet::*;

use serde::{Deserialize, Serialize};

/// Response containing the version of the backend
///
/// Clients usually use this to conditionally enable a feature if the backend's version is high
/// enough to support it.
#[derive(Debug, Serialize, Deserialize)]
pub struct VersionResponse {
    /// Version of the backend in the form MAJOR.MINOR.PATCH
    pub version: String,
}
