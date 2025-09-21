//! Package storage module: SQLx-backed persistence for packages and access roles.
//!
//! Scope:
//! - CRUD on `package` rows
//! - Visibility-filtered reads based on session address
//! - Access role CRUD in `package_access`
//! - Package shapes CRUD in `package_shape`
//!
//! Design notes:
//! - Uses explicit transactions where multi-step writes must be atomic
//! - Leaves access control decisions to the service layer (except visibility filters)
//! - Patch updates bind only provided fields, supporting NULL to clear values
//
use sqlx::Error;

use crate::{common::crud::{CrudQueries, QueryLister}, db::{DbErr, Trx, TrxTrait}};

/// Storage layer for packages, backed by SQLx + SQLite.
///
/// Handles transactions and low-level SQL, without applying access-control decisions
/// (those are enforced by the service layer where required).
pub struct PackageStore {}

impl PackageStore {
    /// Create a new store using the given pool.
    pub fn new() -> Self {
        Self {}
    }
}

/*******
* Package CRUD operations
*/

/// Lister
pub type PackageLister<'a> = QueryLister<'a>;

/// Database model for a package row.
#[derive(sqlx::FromRow, Debug, Clone)]
pub struct PackageDb {
    pub id: i64,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub is_public: bool,
}

/// Insert model for a new package.
#[derive(Debug)]
pub struct NewPackageDb<'d> {
    pub name: &'d str,
    pub slug: &'d str,
    pub description: Option<&'d str>,
    pub icon: Option<&'d str>,
    pub is_public: bool,
}

/// Update model for an existing package.
///
/// Notes about nested options:
/// - `description: Option<Option<&str>>`: `None` -> not provided; `Some(None)` -> set NULL; `Some(Some(v))` -> set value
/// - `icon` follows the same pattern.
/// - `slug` is not included because slugs cannot be changed after creation
#[derive(Debug)]
pub struct PatchPackageDb<'d> {
    pub name: Option<&'d str>,
    pub description: Option<Option<&'d str>>,
    pub icon: Option<Option<&'d str>>,
    pub is_public: Option<bool>,
}

impl<'d> CrudQueries<'d> for PackageStore {
    type Item = PackageDb;
    type NewItem = NewPackageDb<'d>;
    type PatchItem = PatchPackageDb<'d>;
    type Lister = PackageLister<'d>;
    type Id = i64;

    /// Insert a new package and assign `owner` role to the provided session address.
    /// Uses a transaction to ensure both package row and access row are created.
    async fn add_item(&self, item: &Self::NewItem, trx: &mut Trx) -> Result<Self::Item, DbErr> {
        let transaction = trx.get_mut();

        let result = sqlx::query_as::<_, Self::Item>(
            "INSERT INTO package (name, slug, description, icon, is_public) VALUES (?, ?, ?, ?, ?) RETURNING id, name, slug, description, icon, is_public",
        )
            .bind(item.name)
            .bind(item.slug)
            .bind(item.description)
            .bind(item.icon)
            .bind(item.is_public)
            .fetch_one(&mut **transaction)
            .await;

        result
    }

    /// Retrieve packages visible to the optional session.
    /// When `session` is `None`, only public packages are returned.
    async fn get_items(&self, lister: &Self::Lister, trx: &mut Trx) -> Result<Vec<Self::Item>, DbErr> {
        let transaction = trx.get_mut();

        let select = Vec::from([String::from("p.id"), String::from("p.name"), String::from("p.slug"), String::from("p.description"), String::from("p.icon"), String::from("p.is_public")]);
        let from = Vec::from([String::from("package p")]);
        let mut where_clauses: Vec<String> = Vec::new();
        let mut paging = Vec::new();

        if let Some(filter) = &lister.filter {
            if let Some(_access) = &filter.access {
                where_clauses.push(String::from("(p.is_public = 1 OR EXISTS (SELECT 1 FROM package_access_roles pa WHERE pa.package_id = p.id AND pa.address = ?))"));
            } else {
                where_clauses.push(String::from("p.is_public = 1"));
            }

            if let Some(_search) = &filter.search {
                where_clauses.push(String::from("(p.name LIKE ? OR p.description LIKE ?)"));
            }

            if let Some(ids) = &filter.ids {
                let ids = ids.iter().map(|id| id.to_string()).collect::<Vec<_>>().join(", ");
                where_clauses.push(format!(" AND p.id IN ({ids})"));
                // where_clause.push_str(" AND p.id IN (");
                // where_clause.push_str(&ids.iter().map(|_| "?").collect::<Vec<_>>().join(", "));
                // where_clause.push_str(")");
            }
        }

        paging.push(format!(" LIMIT {}", lister.pager.limit));
        if let Some(offset) = lister.pager.offset {
            paging.push(format!(" OFFSET {}", offset));
        }

        let mut sql = format!(
            "SELECT {} FROM {}{}",
            select.join(", "),
            from.join(", "),
            if where_clauses.is_empty() {
                String::new()
            } else {
                format!(" WHERE {}", where_clauses.join(" AND "))
            }
        );

        for p in paging {
            sql.push_str(&p);
        }

        let q = sqlx::query_as::<_, Self::Item>(&sql);

        let q = match &lister.filter {
            Some(filter) => {
                let q = match &filter.access {
                    Some(access) => q.bind(access),
                    None => q,
                };

                let q = match &filter.search {
                    Some(search) => {
                        let search = format!("%{}%", search);
                        let q = q.bind(search.clone());
                        let q = q.bind(search);
                        q
                    }
                    None => q,
                };

                // if let Some(ids) = filter.ids {
                //     for id in ids {
                //         let q = q.bind(id);
                //     }
                // }
                q
            }
            None => q,
        };

        let result = q.fetch_all(&mut **transaction).await;

        result
    }

    /// Retrieve a single package by id if visible to the optional session.
    async fn get_item(&self, id: Self::Id, access: Option<&'d str>, trx: &mut Trx) -> Result<Self::Item, DbErr> {
        let transaction = trx.get_mut();
        // FIXME: impl needs to be lifetimed to set in lifetimed types,
        //               impl lifetime must apply to something but types...
        //               so added lifetime to trait, but...  
        //               trait defined lifetime applies methods too....
        //               so each method needs params of own lifetime....
        //               actually each method needs own lifetime to uniform their params lifetimes and `transaction` param is important here...
        //               so cannot pass local references of one method when call another...
        //               like this:
        // let ids = vec![id];
        // let lister = QueryLister {
        //     filter: Some(QueryFilter {
        //         access: access.map(|s| s),
        //         filter: None,
        //         ids: Some(ids.as_ref()),
        //         search: None,
        //     }),
        //     pager: QueryPager { limit: 1, offset: None },
        // };
        // let items = self.get_items(&lister, transaction).await?;

        // let item = items.get(0).cloned().ok_or(Error::RowNotFound)?;

        // Ok(item)

        let select = Vec::from([String::from("p.id"), String::from("p.name"), String::from("p.slug"), String::from("p.description"), String::from("p.icon"), String::from("p.is_public")]);
        let from = Vec::from([String::from("package p")]);
        let mut where_clauses: Vec<String> = Vec::new();

        if let Some(_access) = access {
            where_clauses.push(String::from("(p.is_public = 1 OR EXISTS (SELECT 1 FROM package_access_roles pa WHERE pa.package_id = p.id AND pa.address = ?))"));
        } else {
            where_clauses.push(String::from("p.is_public = 1"));
        }

        where_clauses.push(String::from("p.id = ?"));

        let sql = format!(
            "SELECT {} FROM {}{}",
            select.join(", "),
            from.join(", "),
            if where_clauses.is_empty() {
                String::new()
            } else {
                format!(" WHERE {}", where_clauses.join(" AND "))
            }
        );

        let q = sqlx::query_as::<_, Self::Item>(&sql);

        let q = match access {
            Some(access) => q.bind(access).bind(id),
            None => q.bind(id),
        };

        let result = q.fetch_one(&mut **transaction).await;

        result
    }

    /// Update a package row. Access is validated by the service layer.
    ///
    /// Notes:
    /// - Builds the SQL `SET` clause with `crate::sqlx_build_set!`, adding columns only
    ///   for fields that are `Some(..)`.
    /// - Bind values must follow the exact same order as the `SET` fragments above.
    ///   We bind each provided field in sequence, then bind `id` for the `WHERE`.
    /// - For `Option<Option<&str>>` fields like `description` and `icon`:
    ///   - `Some(Some(v))` sets the value to `v`.
    ///   - `Some(None)` sets the column to `NULL`.
    ///   - `None` means "do not update this column".
    /// - If no fields are provided, returns `Error::InvalidArgument`.
    async fn update_item(&self, id: Self::Id, item: &Self::PatchItem, trx: &mut Trx) -> Result<Self::Item, DbErr> {
        let transaction = trx.get_mut();
        let set = crate::sqlx_build_set!(
            item.name.is_some() => "name = ?",
            item.description.is_some() => "description = ?",
            item.icon.is_some() => "icon = ?",
            item.is_public.is_some() => "is_public = ?"
        );

        if set.is_empty() {
            return Err(Error::InvalidArgument(
                "No fields to update in package item".to_string(),
            ));
        }
        let query = format!(
            "UPDATE package SET {} WHERE id = ? RETURNING id, name, slug, description, icon, is_public",
            set
        );

        let q = sqlx::query_as::<_, Self::Item>(&query);
        let q = match item.name {
            Some(name) => q.bind(name),
            _ => q,
        };
        // Bind Option<&str> correctly when provided, including NULL
        let q = match item.description {
            Some(desc_opt) => q.bind(desc_opt),
            None => q,
        };
        let q = match item.icon {
            Some(icon_opt) => q.bind(icon_opt),
            None => q,
        };
        let q = match item.is_public {
            Some(is_public) => q.bind(is_public),
            None => q,
        };

        // let item_id: i32 = id.try_into().map_err(|_| Error::InvalidArgument("id is too big. Unsupported by DB".into()))?;

        let result = q
            .bind(id)
            .fetch_one(&mut **transaction)
            .await;
            //  {
            //     Ok(row) => row,
            //     Err(err) => {
            //         let _ = transaction.rollback().await;
            //         return Err(err);
            //     }
            // };

        result

        // match transaction.commit().await {
        //     Ok(_) => Ok(result),
        //     Err(err) => Err(err),
        // }
    }

    /// Delete a package row and its access entries within a transaction.
    async fn delete_item(&self, id: Self::Id, trx: &mut Trx) -> Result<Self::Item, DbErr> {
        let transaction = trx.get_mut();

        let result = sqlx::query_as::<_, Self::Item>("DELETE FROM package WHERE id = ? RETURNING id, name, slug, description, icon, is_public")
            .bind(id)
            .fetch_one(&mut **transaction)
            .await;
            //  {
            //     Ok(row) => row,
            //     Err(err) => {
            //         let _ = transaction.rollback().await;
            //         return Err(err);
            //     }
            // };

        result

        // match sqlx::query("DELETE FROM package_access WHERE package_id = ?")
        //     .bind(id)
        //     .execute(&mut *transaction)
        //     .await {
        //         Ok(_) => {}
        //         Err(err) => {
        //             let _ = transaction.rollback().await;
        //             return Err(err);
        //         }
        //     };

        // match transaction.commit().await {
        //     Ok(_) => Ok(result),
        //     Err(err) => Err(err),
        // }
    }

    // /// List all shapes for a package id.
    // pub async fn list_shapes(&self, package_id: i32) -> Result<Vec<PackageShapeDb>, Error> {
    //     let connection = self.pool.deref();

    //     let rows = sqlx::query_as::<_, PackageShapeDb>(
    //         "SELECT id, package_id, name, description FROM package_shape WHERE package_id = ? ORDER BY name"
    //     )
    //     .bind(package_id)
    //     .fetch_all(connection)
    //     .await?;

    //     Ok(rows)
    // }
}
