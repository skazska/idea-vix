use crate::error::ModelError;

/// Check if any role equals "owner".
pub fn has_owner<'a, I>(roles: I) -> bool
where
    I: IntoIterator<Item = &'a str>,
{
    roles.into_iter().any(|r| r == "owner")
}

/// Check if any role equals "owner" or "manage".
pub fn has_owner_or_manage<'a, I>(roles: I) -> bool
where
    I: IntoIterator<Item = &'a str>,
{
    roles.into_iter().any(|r| r == "owner" || r == "manage")
}

/// Prevent revoking own access; returns BadRequest if target == session.
pub fn ensure_not_self_revoke(target_address: &str, session_address: &str) -> Result<(), ModelError> {
    if target_address == session_address {
        Err(ModelError::BadRequest("Cannot revoke own access".to_string()))
    } else {
        Ok(())
    }
}
