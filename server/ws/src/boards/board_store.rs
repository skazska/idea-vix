//! Board storage module: SQLx-backed persistence for boards and access roles.
//!
//! Scope:
//! - CRUD on `board` rows
//! - Visibility-filtered reads based on session address
//!
//! Design notes:
//! - Uses explicit transactions where multi-step writes must be atomic
//! - Leaves access control decisions to the service layer (except visibility filters)
//! - Patch updates bind only provided fields, supporting NULL to clear values

use crate::{common::crud::{CrudQueries, QueryLister}, db::{DbErr, Trx, TrxTrait}};

/// Storage layer for boards, backed by SQLx + SQLite.
///
/// Handles transactions and low-level SQL, without applying access-control decisions
/// (those are enforced by the service layer where required).
pub struct BoardStore {}

impl BoardStore {
    /// Create a new store using the given pool.
    pub fn new() -> Self {
        Self {}
    }
}

/*******
* Board CRUD operations
*/

/// Lister
pub type BoardLister<'a> = QueryLister<'a>;

// Board database content model
#[derive(sqlx::FromRow, Debug)]
pub struct BoardDb {
    pub id: i64,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub is_public: bool,
}

/// New board database model
#[derive(Debug)]
pub struct NewBoardDb<'a> {
    pub name: &'a str,
    pub slug: &'a str,
    pub description: Option<&'a str>,
    pub icon: Option<&'a str>,
    pub is_public: bool,
}


/// Patch board database model
#[derive(Debug)]
pub struct PatchBoardDb<'a> {
    pub name: Option<&'a str>,
    pub slug: Option<&'a str>,
    pub description: Option<Option<&'a str>>,
    pub icon: Option<Option<&'a str>>,
    pub is_public: Option<bool>,
}

impl<'d> CrudQueries<'d> for BoardStore {
    type Item = BoardDb;
    type NewItem = NewBoardDb<'d>;
    type PatchItem = PatchBoardDb<'d>;
    type Lister = BoardLister<'d>;
    type Id = i64;

    /// Insert a new board and assign `owner` role to the provided session address.
    /// Uses a transaction to ensure both board row and access row are created.
    async fn add_item(&self, item: &Self::NewItem, trx: &mut Trx) -> Result<Self::Item, DbErr> {
        let transaction = trx.get_mut();

        let result = sqlx::query_as::<_, Self::Item>(
            "INSERT INTO board (name, slug, description, icon, is_public) VALUES (?, ?, ?, ?, ?) RETURNING id, name, slug, description, icon, is_public",
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

    /// Retrieve boards visible to the optional session.
    /// When `session` is `None`, only public boards are returned.
    async fn get_items(&self, lister: &Self::Lister, trx: &mut Trx) -> Result<Vec<Self::Item>, DbErr> {
        let transaction = trx.get_mut();

        let select = Vec::from([String::from("b.id"), String::from("b.name"), String::from("b.slug"), String::from("b.description"), String::from("b.icon"), String::from("b.is_public")]);
        let from = Vec::from([String::from("board b")]);
        let mut where_clauses: Vec<String> = Vec::new();
        let mut paging = Vec::new();

        if let Some(filter) = &lister.filter {
            if let Some(_access) = &filter.access {
                where_clauses.push(String::from("(b.is_public = 1 OR EXISTS (SELECT 1 FROM board_access_roles bar WHERE bar.board_id = b.id AND bar.address = ?))"));
            } else {
                where_clauses.push(String::from("b.is_public = 1"));
            }

            if let Some(_search) = &filter.search {
                where_clauses.push(String::from("(b.name LIKE ? OR b.description LIKE ?)"));
            }

            if let Some(ids) = &filter.ids {
                let ids = ids.iter().map(|id| id.to_string()).collect::<Vec<_>>().join(", ");
                where_clauses.push(format!(" AND b.id IN ({ids})"));
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

                match &filter.search {
                    Some(search) => {
                        let pattern = format!("%{}%", search);
                        q.bind(pattern.clone()).bind(pattern)
                    }
                    None => q,
                }
            }
            None => q,
        };

        let result = q.fetch_all(&mut **transaction).await;

        result
    }

    /// Retrieve a single board by its ID with access control
    async fn get_item(&self, item_id: Self::Id, access: Option<&'d str>, trx: &mut Trx) -> Result<Self::Item, DbErr> {
        let transaction = trx.get_mut();

        let query = if let Some(address) = access {
            sqlx::query_as::<_, Self::Item>(
                "SELECT b.id, b.name, b.slug, b.description, b.icon, b.is_public
                 FROM board b
                 LEFT JOIN board_access_roles bar ON b.id = bar.board_id
                 WHERE b.id = ? AND (b.is_public = 1 OR (bar.address = ? AND bar.role IN ('owner', 'manage', 'edit', 'view')))",
            )
            .bind(item_id)
            .bind(address)
        } else {
            sqlx::query_as::<_, Self::Item>(
                "SELECT id, name, slug, description, icon, is_public FROM board WHERE id = ? AND is_public = 1",
            )
            .bind(item_id)
        };

        let result = query.fetch_one(&mut **transaction).await;

        result
    }

    /// Updates a board in the store. Access is validated by the service layer.
    async fn update_item(&self, id: Self::Id, item: &Self::PatchItem, trx: &mut Trx) -> Result<Self::Item, DbErr> {
        let transaction = trx.get_mut();

        let mut query_parts = Vec::new();

        if item.name.is_some() {
            query_parts.push("name = ?");
        }
        if item.slug.is_some() {
            query_parts.push("slug = ?");
        }
        if item.description.is_some() {
            query_parts.push("description = ?");
        }
        if item.icon.is_some() {
            query_parts.push("icon = ?");
        }
        if item.is_public.is_some() {
            query_parts.push("is_public = ?");
        }

        if query_parts.is_empty() {
            // Nothing to update, fetch the current item
            return sqlx::query_as::<_, Self::Item>("SELECT id, name, slug, description, icon, is_public FROM board WHERE id = ?")
                .bind(id)
                .fetch_one(&mut **transaction)
                .await;
        }

        let query_str = format!(
            "UPDATE board SET {} WHERE id = ? RETURNING id, name, slug, description, icon, is_public",
            query_parts.join(", ")
        );

        let mut query = sqlx::query_as::<_, Self::Item>(&query_str);

        // Bind values in the order they appear in the SET clause
        if let Some(name) = item.name {
            query = query.bind(name);
        }
        if let Some(slug) = item.slug {
            query = query.bind(slug);
        }
        if let Some(description) = item.description {
            query = query.bind(description);
        }
        if let Some(icon) = item.icon {
            query = query.bind(icon);
        }
        if let Some(is_public) = item.is_public {
            query = query.bind(is_public);
        }

        // Bind the ID for the WHERE clause
        query = query.bind(id);

        let result = query.fetch_one(&mut **transaction).await;

        result
    }

    /// Delete a board by its ID. Access validation is performed by the service layer.
    async fn delete_item(&self, id: Self::Id, trx: &mut Trx) -> Result<Self::Item, DbErr> {
        let transaction = trx.get_mut();

        let result = sqlx::query_as::<_, Self::Item>("DELETE FROM board WHERE id = ? RETURNING id, name, description, icon, is_public, slug")
            .bind(id)
            .fetch_one(&mut **transaction)
            .await;

        result
    }
}