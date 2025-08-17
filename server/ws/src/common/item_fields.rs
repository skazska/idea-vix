use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::api::deserialize::deserialize_some;

/// Shared new-item fields used by boards and packages.
#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct NewItemFields {
    #[validate(length(min = 3, max = 100))]
    pub name: String,
    #[validate(length(max = 500))]
    pub description: Option<String>,
    #[validate(length(max = 255))]
    pub icon: Option<String>,
    pub is_public: Option<bool>,
}

/// Shared patch-item fields used by boards and packages, with nested option semantics.
#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct PatchItemFields {
    #[validate(length(min = 3, max = 100))]
    pub name: Option<String>,
    #[serde(default, deserialize_with = "deserialize_some")]
    #[validate(length(max = 500))]
    pub description: Option<Option<String>>,
    #[serde(default, deserialize_with = "deserialize_some")]
    #[validate(length(max = 255))]
    pub icon: Option<Option<String>>,
    pub is_public: Option<bool>,
}
