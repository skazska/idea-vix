use serde::{Deserialize, Serialize};
use validator::Validate;

/// Shared role enumeration used across features that implement access control.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum Role {
    Owner,
    Manage,
    Edit,
    View,
}

impl From<String> for Role {
    fn from(role: String) -> Self {
        match role.as_str() {
            "owner" => Role::Owner,
            "manage" => Role::Manage,
            "edit" => Role::Edit,
            "view" => Role::View,
            _ => panic!("Unknown access role: {}", role),
        }
    }
}

impl From<&str> for Role {
    fn from(role: &str) -> Self {
        match role {
            "owner" => Role::Owner,
            "manage" => Role::Manage,
            "edit" => Role::Edit,
            "view" => Role::View,
            _ => panic!("Unknown access role: {}", role),
        }
    }
}

impl Into<String> for Role {
    fn into(self) -> String {
        match self {
            Role::Owner => "owner".to_string(),
            Role::Manage => "manage".to_string(),
            Role::Edit => "edit".to_string(),
            Role::View => "view".to_string(),
        }
    }
}

/// Wrapper returned by service when reporting an effective role.
#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct RoleOnly {
    pub role: Role,
}

/// Generic access grant request(payload): address + role.
#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct GrantRequest {
    #[validate(length(min = 3, max = 255))]
    pub address: String,
    /// One of: "view", "edit", "manage"
    pub role: String,
}

/// Validate string role value to be one of allowed non-owner roles for grant/revoke endpoints.
pub fn validate_grant_role(role: &str) -> Result<(), validator::ValidationError> {
    match role {
        "view" | "edit" | "manage" => Ok(()),
        _ => Err(validator::ValidationError::new("invalid_role")),
    }
}
