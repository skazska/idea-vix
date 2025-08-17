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

#[cfg(test)]
mod tests {
    use super::*;
    use validator::Validate;

    #[test]
    fn new_item_fields_valid() {
        let item = NewItemFields {
            name: "Valid Name".to_string(),
            description: Some("desc".to_string()),
            icon: Some("icon".to_string()),
            is_public: None,
        };
        assert!(item.validate().is_ok());
    }

    #[test]
    fn new_item_fields_name_too_short() {
        let item = NewItemFields { name: "ab".to_string(), description: None, icon: None, is_public: None };
        assert!(item.validate().is_err());
    }

    #[test]
    fn patch_item_fields_description_length_limits() {
        let ok = PatchItemFields {
            name: None,
            description: Some(Some("x".repeat(500))),
            icon: None,
            is_public: None,
        };
        assert!(ok.validate().is_ok());

        let too_long = PatchItemFields {
            name: None,
            description: Some(Some("x".repeat(501))),
            icon: None,
            is_public: None,
        };
        assert!(too_long.validate().is_err());
    }
}
