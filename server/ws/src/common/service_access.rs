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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn has_owner_true_when_owner_present() {
        assert!(has_owner(["view", "owner"].iter().copied()));
        assert!(!has_owner(["view", "edit"].iter().copied()));
    }

    #[test]
    fn has_owner_or_manage_true_for_owner_or_manage() {
        assert!(has_owner_or_manage(["manage"].iter().copied()));
        assert!(has_owner_or_manage(["owner"].iter().copied()));
        assert!(!has_owner_or_manage(["edit", "view"].iter().copied()));
    }

    #[test]
    fn ensure_not_self_revoke_blocks_same_address() {
        let err = ensure_not_self_revoke("a@b", "a@b").unwrap_err();
        match err { ModelError::BadRequest(_) => {}, _ => panic!("unexpected error type") }
    }

    #[test]
    fn ensure_not_self_revoke_allows_other_address() {
        assert!(ensure_not_self_revoke("x@b", "a@b").is_ok());
    }
}
