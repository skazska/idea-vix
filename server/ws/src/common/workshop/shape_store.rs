//! Workshop shape storage module: SQLx-backed persistence for shapes.
//!
//! Scope:
//! - CRUD on `shape` rows
//! - Package-shape association operations
//! - Semantic identifier (slug) lookups
//!
//! Design notes:
//! - Uses explicit transactions where multi-step writes must be atomic
//! - Supports semantic identifier lookups for text-based references
//! - JSON definition storage as TEXT in SQLite

use std::{time::SystemTime};
use sqlx::Error;

use crate::{common::crud::{CrudQueries, QueryLister}, db::{to_unix_timestamp, DbErr, Trx, TrxTrait}};

/***
 * Shape CRUD operations
 */

/// Lister
pub type ShapeLister<'a> = QueryLister<'a>;


/// Database model for a shape row.
#[derive(sqlx::FromRow, Debug, Clone)]
pub struct ShapeDb {
    pub id: i64,
    pub name: String,
    pub slug: String, // Required in our business logic
    pub description: Option<String>,
    pub definition: serde_json::value::Value, // JSON as TEXT
    pub created_at: i64,
    pub updated_at: i64,
}

/// Insert model for a new shape.
#[derive(Debug)]
pub struct NewShapeDb<'a> {
    pub name: &'a str,
    pub slug: &'a str, // required semantic identifier
    pub description: Option<&'a str>,
    pub definition: &'a serde_json::value::Value, // JSON as TEXT
}

/// Update model for an existing shape.
#[derive(Debug)]
pub struct PatchShapeDb<'a> {
    pub name: Option<&'a str>,
    pub description: Option<Option<&'a str>>,
    pub definition: Option<&'a serde_json::value::Value>,
}

/// Database model for package-shape association.
#[derive(sqlx::FromRow, Debug)]
pub struct PackageShapeDb {
    pub package_id: i64,
    pub shape_id: i64,
    pub created_at: i64,
}

/// Storage layer for workshop shapes.
pub struct ShapeStore {}

impl ShapeStore {
    /// Create a new store with the provided database pool.
    pub fn new() -> Self { Self {} }

    // /// Add a shape to workshop.
    // pub async fn add_shape_to_package(&self, item_id: i32, shape_id: i32) -> Result<PackageShapeDb, Error> {
    //     let now = to_unix_timestamp(SystemTime::now()).try_into().unwrap_or(0);

    //     let connection = self.pool.deref();

    //     let result = sqlx::query_as::<_, PackageShapeDb>(
    //         "INSERT INTO package_shape (package_id, shape_id, created_at) VALUES (?1, ?2, ?3)",
    //     )
    //     .bind(package_id)
    //     .bind(shape_id)
    //     .bind(now)
    //     .fetch_one(connection)
    //     .await?;

    //     Ok(result)
    // }

}

impl<'a> CrudQueries<'a> for ShapeStore {
    type Item = ShapeDb;
    type NewItem = NewShapeDb<'a>;
    type PatchItem = PatchShapeDb<'a>;
    type Lister = ShapeLister<'a>;
    type Id = i64;

    /// Create a new shape in the global workshop.
    async fn add_item(&self, shape: &Self::NewItem, trx: &mut Trx) -> Result<Self::Item, DbErr> {
        let now: i64 = to_unix_timestamp(SystemTime::now()).try_into().unwrap_or(0);

        let transaction = trx.get_mut();

        let result = sqlx::query_as::<_, Self::Item>(
            r#"
            INSERT INTO shape (name, slug, description, definition, created_at, updated_at)
            VALUES (?1, ?2, ?3, ?4, ?5, ?5)
            RETURNING id, name, slug, description, definition, created_at, updated_at
            "#
        )
        .bind(shape.name)
        .bind(shape.slug)
        .bind(shape.description)
        .bind(shape.definition)
        .bind(now)
        .fetch_one(&mut **transaction).await;

        result
        // .await {
        //     Ok(row) => row,
        //     Err(err) => {
        //         let _ = transaction.rollback().await;
        //         return Err(err);
        //     }
        // };

        // Commit the transaction
        // match transaction.commit().await {
        //     Ok(_) => Ok(result),
        //     Err(err) => Err(err),
        // }
    }

    /// List all shapes.
    /// No access control here use from service layer.
    async fn get_items(&self, lister: &Self::Lister, trx: &mut Trx) -> Result<Vec<Self::Item>, DbErr> {
        let transaction = trx.get_mut();

        let select = Vec::from([String::from("s.id"), String::from("s.name"), String::from("s.slug"), String::from("s.description"), String::from("s.definition"), String::from("s.created_at"), String::from("s.updated_at")]);
        let from = Vec::from([String::from("shape s")]);
        let mut where_clauses = Vec::new();
        let mut paging = Vec::new();

        if let Some(filter) = &lister.filter {
            // if let Some(_access) = &filter.access {
            //     where_clauses.push(String::from("(b.is_public = 1 OR EXISTS (SELECT 1 FROM package_access pa WHERE pa.package_id = p.id AND pa.address = ?))"));
            // } else {
            //     where_clauses.push(String::from("p.is_public = 1"));
            // }

            if let Some(_search) = &filter.search {
                where_clauses.push(format!("s.name LIKE ?"));
            }

            if let Some(ids) = &filter.ids {
                if !ids.is_empty() {
                    let ids_str: String = ids.iter().map(|id| id.to_string()).collect::<Vec<_>>().join(",");
                    where_clauses.push(format!("s.id IN ({})", ids_str));
                    // where_clause.push_str(" AND p.id IN (");
                    // where_clause.push_str(&ids.iter().map(|_| "?").collect::<Vec<_>>().join(", "));
                    // where_clause.push_str(")");
                }
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
                // let q = match &filter.access {
                //     Some(access) => q.bind(access),
                //     None => q,
                // };

                let q = match &filter.search {
                    Some(search) => {
                        let search_pattern = format!("%{}%", search);
                        q.bind(search_pattern)
                    },
                    None => q,
                };

                q
            },
            None => q,
        };

        let results = q.fetch_all(&mut **transaction).await;

        results
    }

    /// Get a shape by id.
    async fn get_item(&self, id: Self::Id, _access: Option<&'a str>, trx: &mut Trx) -> Result<Self::Item, DbErr> {
        let transaction = trx.get_mut();

        // FIXME: impl needs to be lifetimed to set in lifetimed types,
        //               impl lifetime must apply to something but types...
        //               so added lifetime to trait, but...  
        //               trait defined lifetime applies methods too....
        //               so each method needs params of own lifetime....
        //               actually each method needs own lifetime to uniform their params lifetimes and `transaction` param is important here...
        //               so cannot pass local references of one method when call another...   
        //               like this:
        // let items = self.get_items(&Self::Lister {
        //     filter: Some(crate::common::crud::QueryFilter {
        //         access: access.map(|s| s.to_string()),
        //         filter: None,
        //         ids: Some(Vec::from([id])),
        //         search: None,
        //     }),
        //     pager: QueryPager { limit: 1, offset: None },
        // }, transaction).await?;

        // let item = items.get(0).cloned().ok_or(Error::RowNotFound)?;

        // Ok(item)

        let select = Vec::from([String::from("s.id"), String::from("s.name"), String::from("s.slug"), String::from("s.description"), String::from("s.definition"), String::from("s.created_at"), String::from("s.updated_at")]);
        let from = Vec::from([String::from("shape s")]);
        let where_clauses = Vec::from([String::from("s.id = ?1")]);

        let sql = format!(
            "SELECT {} FROM {} WHERE {}",
            select.join(", "),
            from.join(", "),
            where_clauses.join(" AND ")
        );

        let q = sqlx::query_as::<_, Self::Item>(&sql).bind(id);
        let result = q.fetch_one(&mut **transaction).await;

        result
    }

    /// Update an existing shape.
    async fn update_item(&self, id: Self::Id, patch: &Self::PatchItem, trx: &mut Trx) -> Result<Self::Item, DbErr> {
        let transaction = trx.get_mut();
        let now = to_unix_timestamp(SystemTime::now()).try_into().unwrap_or(0);

        let set = crate::sqlx_build_set!(
            patch.name.is_some() => "name = ?",
            patch.description.is_some() => "description = ?",
            patch.definition.is_some() => "definition = ?",
            true => "updated_at = ?"
        );

        if set.eq("updated_at = ?") {
            return Err(Error::InvalidArgument("No fields to update".into()));
        }

        let query = format!(
            "UPDATE shape SET {} WHERE id = ? RETURNING id, name, slug, description, definition, created_at, updated_at",
            set
        );

        let q = sqlx::query_as::<_, Self::Item>(&query);

        let q = match patch.name {
            Some(name) => q.bind(name),
            None => q,
        };

        let q = match patch.description {
            Some(description) => q.bind(description),
            None => q,
        };

        let q = match patch.definition {
            Some(definition) => q.bind(definition),
            None => q,
        };

        let result = q.bind(now).bind(id).fetch_one(&mut **transaction).await;

        result
    }

    /// Delete a shape by id.
    async fn delete_item(&self, id: Self::Id, trx: &mut Trx) -> Result<Self::Item, DbErr> {
        let transaction = trx.get_mut();
        let result = sqlx::query_as::<_, Self::Item>("DELETE FROM shape WHERE id = ? RETURNING id, name, slug, description, definition, created_at, updated_at")
            .bind(id)
            .fetch_one(&mut **transaction)
            .await;

        result
    }
}
