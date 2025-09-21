use std::{sync::Arc};
use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::{db::{DbErr, QueriesIntId, TransactionStarter, Trx, TrxTrait}, error::ModelError, session::session_service::SessionData};


/// Shared role enumeration used across features that implement access control.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Role {
    Owner,
    Manage,
    Edit,
    View,
}

pub const ROLE_OWNER: &str = "owner";
pub const ROLE_MANAGE: &str = "manage";
pub const ROLE_EDIT: &str = "edit";
pub const ROLE_VIEW: &str = "view";

impl From<&String> for Role {
    fn from(role: &String) -> Self {
        match role.to_lowercase().as_str() {
            ROLE_OWNER => Role::Owner,
            ROLE_MANAGE => Role::Manage,
            ROLE_EDIT => Role::Edit,
            ROLE_VIEW => Role::View,
            _ => panic!("Unknown access role: {}", role),
        }
    }
}

impl Into<String> for Role {
    fn into(self) -> String {
        match self {
            Role::Owner => ROLE_OWNER.to_string(),
            Role::Manage => ROLE_MANAGE.to_string(),
            Role::Edit => ROLE_EDIT.to_string(),
            Role::View => ROLE_VIEW.to_string(),
        }
    }
}

impl Serialize for Role {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let role_str: String = self.clone().into();
        serializer.serialize_str(&role_str)
    }
}

impl<'de> Deserialize<'de> for Role {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            ROLE_OWNER => Ok(Role::Owner),
            ROLE_MANAGE => Ok(Role::Manage),
            ROLE_EDIT => Ok(Role::Edit),
            ROLE_VIEW => Ok(Role::View),
            _ => Err(serde::de::Error::custom(format!("Unknown access role: {}", s))),
        }
    }
}

/// Generic access grant request DTO (for HTTP requests): address + role (item_id comes from path parameter).
#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct ItemAccessGrantDto {
    #[validate(length(min = 3, max = 255))]
    pub address: String,
    pub role: Role,
}

/*******
* Item access Service
*/

/// Generic session access role: address + role.
#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct ItemRole<Id: Clone> {
    pub item_id: Id,
    pub address: String,
    pub role: Role,
}

impl<Id: Clone> From<(Id, &ItemRoleDb)> for ItemRole<Id> {
    fn from((id, db): (Id, &ItemRoleDb)) -> Self {
        Self {
            item_id: id,
            address: db.address.clone(),
            role: Role::from(&db.role),
        }
    }
}

/// Validate string role value to be one of allowed non-owner roles for grant/revoke endpoints.
pub fn validate_grant_role(role: &Role) -> Result<(), validator::ValidationError> {
    match role {
        Role::View | Role::Edit | Role::Manage => Ok(()),
        _ => Err(validator::ValidationError::new("invalid_role")),
    }
}

/// Check if any role equals "owner".
pub fn has_owner(roles: &Vec<Role>) -> bool {
    roles.into_iter().any(|r| *r == Role::Owner)
}

/// Check if any role equals "owner" or "manage".
pub fn has_owner_or_manage(roles: &Vec<Role>) -> bool {
    roles.into_iter().any(|r| *r == Role::Owner || *r == Role::Manage)
}

/// Prevent revoking own access; returns BadRequest if target == session.
pub fn ensure_not_self(target_address: &str, session_address: &str) -> Result<(), ModelError> {
    if target_address == session_address {
        Err(ModelError::BadRequest("Cannot manage own access".to_string()))
    } else {
        Ok(())
    }
}

pub trait ItemAccess {
    type SessionData;
    type Id: Clone + Send + Sync; // + Unpin;

    /// Add a new access role to an item.
    fn add_access_role(&self, role: ItemRole<Self::Id>, session: &Self::SessionData) -> impl Future<Output=Result<ItemRole<Self::Id>, ModelError>>;
    /// Revoke an access role from an item.
    fn revoke_access_role(&self, role: ItemRole<Self::Id>, session: &Self::SessionData) -> impl Future<Output=Result<ItemRole<Self::Id>, ModelError>>;
    /// Revoke all access roles of an address from a specific item.
    fn revoke_access_roles(&self, item_id: Self::Id, address: String, session: &Self::SessionData) -> impl Future<Output=Result<Vec<ItemRole<Self::Id>>, ModelError>>;
    /// List all access roles for a specific item.
    fn list_access_roles(&self, item_id: Self::Id, session: &Self::SessionData) -> impl Future<Output=Result<Vec<ItemRole<Self::Id>>, ModelError>>;
    /// Get access roles for the current session user on a specific item.
    fn get_my_access_roles(&self, item_id: Self::Id, session: &Self::SessionData) -> impl Future<Output=Result<Vec<Role>, ModelError>>;
}

pub struct CommonItemAccess {
    transaction_starter: Arc<TransactionStarter>,
    queries: Arc<SqliteItemAccessQueries>,
}

impl CommonItemAccess {
    pub fn new(transaction_starter: Arc<TransactionStarter>, queries: Arc<SqliteItemAccessQueries>) -> Self {
        Self { queries, transaction_starter }
    }

    async fn check_am_owner(&self, item_id: i64, session: &SessionData, trx: &mut Trx) -> Result<(), ModelError> {
        let roles = self.queries.get_access_roles(item_id, session, Some(&Vec::from([ROLE_OWNER])), trx).await?;
        if roles.len() == 0 {
            return Err(ModelError::Forbidden("Only owner can manage access".to_string()));
        }

        Ok(())
    }
}

impl ItemAccess for CommonItemAccess {
    type SessionData = SessionData;
    type Id = i64;

    async fn add_access_role(&self, role: ItemRole<Self::Id>, session: &Self::SessionData) -> Result<ItemRole<Self::Id>, ModelError> {
        if validate_grant_role(&role.role).is_err() {
            return Err(ModelError::BadRequest("Invalid role".to_string()));
        }

        let item_id = role.item_id;
        let role_db = ItemRoleDb::from(&role);

        let result = self.transaction_starter.run_in_transaction(async move |trx| {
            self.check_am_owner(item_id, session, trx).await?;
            ensure_not_self(&role.address, &session.address)?;
            let stored = self.queries.add_access_role(item_id, &role_db, trx).await?;
            Ok(stored)
        }).await?;

        Ok(<ItemRole<Self::Id>>::from((item_id, &result)))
    }

    async fn revoke_access_role(&self, role: ItemRole<Self::Id>, session: &SessionData) -> Result<ItemRole<Self::Id>, ModelError> {
        if validate_grant_role(&role.role).is_err() {
            return Err(ModelError::BadRequest("Invalid role".to_string()));
        }

        let item_id = role.item_id;
        let role_db = ItemRoleDb::from(&role);

        let result = self.transaction_starter.run_in_transaction(async move |trx| {
            self.check_am_owner(item_id, session, trx).await?;
            ensure_not_self(&role.address, &session.address)?;
            let revoked = self.queries.revoke_access_role(item_id, &role_db, trx).await?;
            Ok(revoked)
        }).await?;

        Ok(<ItemRole<Self::Id>>::from((role.item_id, &result)))
    }

    async fn revoke_access_roles(&self, item_id: Self::Id, address: String, session: &SessionData) -> Result<Vec<ItemRole<Self::Id>>, ModelError> {
        ensure_not_self(&address, &session.address)?;

        let result = self.transaction_starter.run_in_transaction(async move |trx| {
            self.check_am_owner(item_id, session, trx).await?;
            let revoked = self.queries.revoke_access_roles(item_id, &address, trx).await?;
            Ok(revoked)
        }).await?;

        Ok(result.into_iter().map(|r| ItemRole::from((item_id, &r))).collect())
    }

    async fn list_access_roles(&self, item_id: Self::Id, session: &SessionData) -> Result<Vec<ItemRole<Self::Id>>, ModelError> {
        let roles = self.transaction_starter.run_in_transaction(async move |trx| {
            self.check_am_owner(item_id, session, trx).await?;
            let rows = self.queries.list_access_roles(item_id, trx).await?;
            Ok(rows)
        }).await?;

        let roles = roles.into_iter().map(|r| ItemRole::from((item_id, &r))).collect();

        println!("list_access_roles roles: {:?}", roles);

        Ok(roles)
    }

    async fn get_my_access_roles(&self, item_id: Self::Id, session: &SessionData) -> Result<Vec<Role>, ModelError> {
        let result = self.transaction_starter.run_in_transaction(async move |trx| {
            let roles = self.queries.get_access_roles(item_id, session, None, trx).await?;
            Ok(roles)
        }).await?;

        Ok(result.into_iter().map(|r| Role::from(&r.role)).collect())
    }
}

/*******
* Item access DB operations
*/

/// Item role Db record with role field only
#[derive(sqlx::FromRow, Debug)]
pub struct RoleOnly {
    pub role: String,
}

/// Item Role Db Record
#[derive(sqlx::FromRow, Debug)]
pub struct ItemRoleDb {
    pub address: String,
    pub role: String,
}

impl<Id: Clone> From<&ItemRole<Id>> for ItemRoleDb {
    fn from(role: &ItemRole<Id>) -> Self {
        Self {
            address: role.address.clone(),
            role: role.role.clone().into(),
        }
    }
}

pub trait ItemAccessQueries<'t> {
    type Id: Clone + Send + Sync; // + Unpin;

    /// Fetch all roles of session.address for a specific item  (for access check).
    fn get_access_roles(&self, item_id: Self::Id, session: &SessionData, roles: Option<&Vec<&str>>, trx: &mut Trx) -> impl Future<Output = Result<Vec<RoleOnly>, DbErr>>;
    /// List roles for a specific item.
    fn list_access_roles(&self, item_id: Self::Id, trx: &mut Trx) -> impl Future<Output = Result<Vec<ItemRoleDb>, DbErr>>;
    /// Grant address role to a specific item.
    fn add_access_role(&self, item_id: Self::Id, role: &ItemRoleDb, trx: &mut Trx) -> impl Future<Output = Result<ItemRoleDb, DbErr>>;
    /// Revoke all roles of address from a specific item.
    fn revoke_access_roles(&self, item_id: Self::Id, address: &str, trx: &mut Trx) -> impl Future<Output = Result<Vec<ItemRoleDb>, DbErr>>;
    /// Revoke a specific role of address from a specific item.
    fn revoke_access_role(&self, item_id: Self::Id, role: &ItemRoleDb, trx: &mut Trx) -> impl Future<Output = Result<ItemRoleDb, DbErr>>;
}

trait ItemAccessQueriesMeta {
    fn get_table(&self) -> &str;
    fn get_id_attr(&self) -> &str;
}

pub struct SqliteItemAccessQueries {
    table: String,
    item_id_attr: String,
}

impl SqliteItemAccessQueries {
    pub fn new(table: &str, item_id_attr: &str) -> Self {
        Self { table: table.to_string(), item_id_attr: item_id_attr.to_string() }
    }
}

impl ItemAccessQueriesMeta for SqliteItemAccessQueries {
    fn get_table(&self) -> &str {
        &self.table
    }

    fn get_id_attr(&self) -> &str {
        &self.item_id_attr
    }
    
}

impl<'d> QueriesIntId<'d> for SqliteItemAccessQueries {
    type Id = i64;

    fn get_id(&self, id: Self::Id) -> impl sqlx::Encode<'d, sqlx::Sqlite> + sqlx::Type<sqlx::Sqlite> + 'd {
        id
    }
}

impl<'t> ItemAccessQueries<'t> for SqliteItemAccessQueries {
    type Id = i64;

    async fn get_access_roles(&self, item_id: Self::Id, session: &SessionData, roles: Option<&Vec<&str>>, trx: &mut Trx) -> Result<Vec<RoleOnly>, DbErr> {
        let transaction = trx.get_mut();
        let mut sql = format!("SELECT role FROM {} WHERE {} = ? AND address = ?", self.get_table(), self.get_id_attr());
        if let Some(roles) = roles {
            if !roles.is_empty() {
                let placeholders = roles.iter().map(|_| "?").collect::<Vec<&str>>().join(", ");
                sql.push_str(&format!(" AND role IN ({})", placeholders));
            }
        }

        let q = sqlx::query_as::<_, RoleOnly>(&sql)
            .bind(self.get_id(item_id))
            .bind(&session.address);

        let q = match roles {
            Some(roles) => {
                let mut q = q;
                for role in roles {
                    q = q.bind(role);
                }
                q
            },
            None => q,
        };

        let access = q.fetch_all(&mut **transaction).await;

        access
    }

    async fn list_access_roles(&self, item_id: Self::Id, trx: &mut Trx) -> Result<Vec<ItemRoleDb>, DbErr> {
        let transaction = trx.get_mut();
        let rows = sqlx::query_as::<_, ItemRoleDb>(
            &format!("SELECT address, role FROM {} WHERE {} = ? ORDER BY address", self.get_table(), self.get_id_attr())
        )
        .bind(self.get_id(item_id))
        .fetch_all(&mut **transaction)
        .await;
println!("list_access_roles rows: {:?}", rows);
        rows
    }

    async fn add_access_role(&self, item_id: Self::Id, role: &ItemRoleDb, trx: &mut Trx) -> Result<ItemRoleDb, DbErr> {
        let transaction = trx.get_mut();
        let result = sqlx::query_as::<_, ItemRoleDb>(
            &format!("INSERT INTO {} ({}, address, role) VALUES (?, ?, ?) RETURNING address, role", self.get_table(), self.get_id_attr())
        )
            .bind(self.get_id(item_id))
            .bind(&role.address)
            .bind(&role.role)
            .fetch_one(&mut **transaction)
            .await;

        result
    }

    async fn revoke_access_roles(&self, item_id: Self::Id, address: &str, trx: &mut Trx) -> Result<Vec<ItemRoleDb>, DbErr> {
        let transaction = trx.get_mut();
        let result = sqlx::query_as::<_, ItemRoleDb>(
            &format!("DELETE FROM {} WHERE {} = ? AND address = ? RETURNING address, role", self.get_table(), self.get_id_attr())
        )
            .bind(self.get_id(item_id))
            .bind(address)
            .fetch_all(&mut **transaction)
            .await;

        result
    }

    async fn revoke_access_role(&self, item_id: Self::Id, role: &ItemRoleDb, trx: &mut Trx) -> Result<ItemRoleDb, DbErr> {
        let transaction = trx.get_mut();
        let result = sqlx::query_as::<_, ItemRoleDb>(
            &format!("DELETE FROM {} WHERE {} = ? AND address = ? AND role = ? RETURNING address, role", self.get_table(), self.get_id_attr())
        )
            .bind(self.get_id(item_id))
            .bind(&role.address)
            .bind(&role.role)
            .fetch_one(&mut **transaction)
            .await;

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn has_owner_true_when_owner_present() {
        assert!(has_owner(&Vec::from([Role::View, Role::Owner])));
        assert!(!has_owner(&Vec::from([Role::View, Role::Edit])));
    }

    #[test]
    fn has_owner_or_manage_true_for_owner_or_manage() {
        assert!(has_owner_or_manage(&Vec::from([Role::Manage])));
        assert!(has_owner_or_manage(&Vec::from([Role::Owner])));
        assert!(!has_owner_or_manage(&Vec::from([Role::Edit, Role::View])));
    }

    #[test]
    fn ensure_not_self_revoke_blocks_same_address() {
        let err = ensure_not_self("a@b", "a@b").unwrap_err();
        match err { ModelError::BadRequest(_) => {}, _ => panic!("unexpected error type") }
    }

    #[test]
    fn ensure_not_self_revoke_allows_other_address() {
        assert!(ensure_not_self("x@b", "a@b").is_ok());
    }
}
