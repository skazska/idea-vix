//! Universal workshop storage module: SQLx-backed persistence for workshop items.
//!
//! Scope:
//! - CRUD on workshop item rows (shape, line, rule, layout)
//! - Entity-item association operations (package-item, board-item)
//! - Semantic identifier (slug) lookups
//! - Parametrizable by workshop item type and linking table name
//!
//! Design notes:
//! - Uses explicit transactions where multi-step writes must be atomic
//! - Supports semantic identifier lookups for text-based references
//! - JSON definition storage as TEXT in SQLite
//! - Generic across all workshop item types using parametrization

use std::time::SystemTime;
use sqlx::Error;

use crate::{common::crud::{CrudQueries, QueryLister}, db::{to_unix_timestamp, DbErr, Trx, TrxTrait}};

/***
 * Workshop Item CRUD operations
 */

/// Lister
pub type WorkshopItemLister<'a> = QueryLister<'a>;

/***
 * Backward compatibility type aliases for shapes
 */

pub type ShapeLister<'a> = WorkshopItemLister<'a>;
pub type ShapeDb = WorkshopItemDb;
pub type NewShapeDb<'a> = NewWorkshopItemDb<'a>;
pub type PatchShapeDb<'a> = PatchWorkshopItemDb<'a>;

/***
 * Type aliases for other workshop item types
 */

pub type LineLister<'a> = WorkshopItemLister<'a>;
pub type LineDb = WorkshopItemDb;
pub type NewLineDb<'a> = NewWorkshopItemDb<'a>;
pub type PatchLineDb<'a> = PatchWorkshopItemDb<'a>;

pub type RuleLister<'a> = WorkshopItemLister<'a>;
pub type RuleDb = WorkshopItemDb;
pub type NewRuleDb<'a> = NewWorkshopItemDb<'a>;
pub type PatchRuleDb<'a> = PatchWorkshopItemDb<'a>;

pub type LayoutLister<'a> = WorkshopItemLister<'a>;
pub type LayoutDb = WorkshopItemDb;
pub type NewLayoutDb<'a> = NewWorkshopItemDb<'a>;
pub type PatchLayoutDb<'a> = PatchWorkshopItemDb<'a>;

/***
 * Specific store implementations
 */

/// Shape-specific store for backward compatibility
pub struct ShapeStore {
    inner: WorkshopStore,
}

impl ShapeStore {
    pub fn new() -> Self {
        Self {
            inner: WorkshopStore::new(WorkshopItemType::Shape),
        }
    }

    /// Get a shape by slug for semantic lookups.
    pub async fn get_item_by_slug(&self, slug: &str, trx: &mut Trx) -> Result<ShapeDb, DbErr> {
        self.inner.get_item_by_slug(slug, trx).await
    }
}

impl<'a> CrudQueries<'a> for ShapeStore {
    type Item = ShapeDb;
    type NewItem = NewShapeDb<'a>;
    type PatchItem = PatchShapeDb<'a>;
    type Lister = ShapeLister<'a>;
    type Id = i64;

    async fn add_item(&self, item: &Self::NewItem, trx: &mut Trx) -> Result<Self::Item, DbErr> {
        self.inner.add_item(item, trx).await
    }

    async fn get_items(&self, lister: &Self::Lister, trx: &mut Trx) -> Result<Vec<Self::Item>, DbErr> {
        self.inner.get_items(lister, trx).await
    }

    async fn get_item(&self, id: Self::Id, access: Option<&'a str>, trx: &mut Trx) -> Result<Self::Item, DbErr> {
        self.inner.get_item(id, access, trx).await
    }

    async fn update_item(&self, id: Self::Id, patch: &Self::PatchItem, trx: &mut Trx) -> Result<Self::Item, DbErr> {
        self.inner.update_item(id, patch, trx).await
    }

    async fn delete_item(&self, id: Self::Id, trx: &mut Trx) -> Result<Self::Item, DbErr> {
        self.inner.delete_item(id, trx).await
    }
}

/// Line-specific store
pub struct LineStore {
    inner: WorkshopStore,
}

impl LineStore {
    pub fn new() -> Self {
        Self {
            inner: WorkshopStore::new(WorkshopItemType::Line),
        }
    }

    pub async fn get_item_by_slug(&self, slug: &str, trx: &mut Trx) -> Result<LineDb, DbErr> {
        self.inner.get_item_by_slug(slug, trx).await
    }
}

impl<'a> CrudQueries<'a> for LineStore {
    type Item = LineDb;
    type NewItem = NewLineDb<'a>;
    type PatchItem = PatchLineDb<'a>;
    type Lister = LineLister<'a>;
    type Id = i64;

    async fn add_item(&self, item: &Self::NewItem, trx: &mut Trx) -> Result<Self::Item, DbErr> {
        self.inner.add_item(item, trx).await
    }

    async fn get_items(&self, lister: &Self::Lister, trx: &mut Trx) -> Result<Vec<Self::Item>, DbErr> {
        self.inner.get_items(lister, trx).await
    }

    async fn get_item(&self, id: Self::Id, access: Option<&'a str>, trx: &mut Trx) -> Result<Self::Item, DbErr> {
        self.inner.get_item(id, access, trx).await
    }

    async fn update_item(&self, id: Self::Id, patch: &Self::PatchItem, trx: &mut Trx) -> Result<Self::Item, DbErr> {
        self.inner.update_item(id, patch, trx).await
    }

    async fn delete_item(&self, id: Self::Id, trx: &mut Trx) -> Result<Self::Item, DbErr> {
        self.inner.delete_item(id, trx).await
    }
}

/// Rule-specific store
pub struct RuleStore {
    inner: WorkshopStore,
}

impl RuleStore {
    pub fn new() -> Self {
        Self {
            inner: WorkshopStore::new(WorkshopItemType::Rule),
        }
    }

    pub async fn get_item_by_slug(&self, slug: &str, trx: &mut Trx) -> Result<RuleDb, DbErr> {
        self.inner.get_item_by_slug(slug, trx).await
    }
}

impl<'a> CrudQueries<'a> for RuleStore {
    type Item = RuleDb;
    type NewItem = NewRuleDb<'a>;
    type PatchItem = PatchRuleDb<'a>;
    type Lister = RuleLister<'a>;
    type Id = i64;

    async fn add_item(&self, item: &Self::NewItem, trx: &mut Trx) -> Result<Self::Item, DbErr> {
        self.inner.add_item(item, trx).await
    }

    async fn get_items(&self, lister: &Self::Lister, trx: &mut Trx) -> Result<Vec<Self::Item>, DbErr> {
        self.inner.get_items(lister, trx).await
    }

    async fn get_item(&self, id: Self::Id, access: Option<&'a str>, trx: &mut Trx) -> Result<Self::Item, DbErr> {
        self.inner.get_item(id, access, trx).await
    }

    async fn update_item(&self, id: Self::Id, patch: &Self::PatchItem, trx: &mut Trx) -> Result<Self::Item, DbErr> {
        self.inner.update_item(id, patch, trx).await
    }

    async fn delete_item(&self, id: Self::Id, trx: &mut Trx) -> Result<Self::Item, DbErr> {
        self.inner.delete_item(id, trx).await
    }
}

/// Layout-specific store
pub struct LayoutStore {
    inner: WorkshopStore,
}

impl LayoutStore {
    pub fn new() -> Self {
        Self {
            inner: WorkshopStore::new(WorkshopItemType::Layout),
        }
    }

    pub async fn get_item_by_slug(&self, slug: &str, trx: &mut Trx) -> Result<LayoutDb, DbErr> {
        self.inner.get_item_by_slug(slug, trx).await
    }
}

impl<'a> CrudQueries<'a> for LayoutStore {
    type Item = LayoutDb;
    type NewItem = NewLayoutDb<'a>;
    type PatchItem = PatchLayoutDb<'a>;
    type Lister = LayoutLister<'a>;
    type Id = i64;

    async fn add_item(&self, item: &Self::NewItem, trx: &mut Trx) -> Result<Self::Item, DbErr> {
        self.inner.add_item(item, trx).await
    }

    async fn get_items(&self, lister: &Self::Lister, trx: &mut Trx) -> Result<Vec<Self::Item>, DbErr> {
        self.inner.get_items(lister, trx).await
    }

    async fn get_item(&self, id: Self::Id, access: Option<&'a str>, trx: &mut Trx) -> Result<Self::Item, DbErr> {
        self.inner.get_item(id, access, trx).await
    }

    async fn update_item(&self, id: Self::Id, patch: &Self::PatchItem, trx: &mut Trx) -> Result<Self::Item, DbErr> {
        self.inner.update_item(id, patch, trx).await
    }

    async fn delete_item(&self, id: Self::Id, trx: &mut Trx) -> Result<Self::Item, DbErr> {
        self.inner.delete_item(id, trx).await
    }
}

/// Database model for a workshop item row (shape, line, rule, layout).
#[derive(sqlx::FromRow, Debug, Clone)]
pub struct WorkshopItemDb {
    pub id: i64,
    pub name: String,
    pub slug: String, // Required semantic identifier
    pub description: Option<String>,
    pub definition: serde_json::value::Value, // JSON as TEXT
    pub created_at: i64,
    pub updated_at: i64,
}

/// Insert model for a new workshop item.
#[derive(Debug)]
pub struct NewWorkshopItemDb<'a> {
    pub name: &'a str,
    pub slug: &'a str, // required semantic identifier
    pub description: Option<&'a str>,
    pub definition: &'a serde_json::value::Value, // JSON as TEXT
}

/// Update model for an existing workshop item.
#[derive(Debug)]
pub struct PatchWorkshopItemDb<'a> {
    pub name: Option<&'a str>,
    pub description: Option<Option<&'a str>>,
    pub definition: Option<&'a serde_json::value::Value>,
}

/// Database model for entity-workshop-item association.
#[derive(sqlx::FromRow, Debug)]
pub struct EntityWorkshopItemDb {
    pub item_id: i64, // entity id (package_id or board_id)
    pub workshop_item_id: i64, // workshop item id (shape_id, line_id, rule_id, layout_id)
    pub origin_id: Option<i64>, // package id for board imports, None for package own items
    pub name: Option<String>, // board-specific override name
    pub created_at: i64,
}

/// Enum to identify workshop item types.
#[derive(Debug, Clone)]
pub enum WorkshopItemType {
    Shape,
    Line,
    Rule,
    Layout,
}

impl WorkshopItemType {
    /// Returns the table name for this workshop item type.
    pub fn table_name(&self) -> &'static str {
        match self {
            WorkshopItemType::Shape => "shape",
            WorkshopItemType::Line => "line",
            WorkshopItemType::Rule => "rule",
            WorkshopItemType::Layout => "layout",
        }
    }

    /// Returns the column name for the workshop item ID in linking tables.
    pub fn id_column_name(&self) -> &'static str {
        match self {
            WorkshopItemType::Shape => "shape_id",
            WorkshopItemType::Line => "line_id",
            WorkshopItemType::Rule => "rule_id",
            WorkshopItemType::Layout => "layout_id",
        }
    }
}

/// Universal storage layer for workshop items, parametrized by item type.
pub struct WorkshopStore {
    item_type: WorkshopItemType,
}

impl WorkshopStore {
    /// Create a new store for the specified workshop item type.
    pub fn new(item_type: WorkshopItemType) -> Self { 
        Self { item_type } 
    }

    /// Get the table name for this store's workshop item type.
    pub fn table_name(&self) -> &'static str {
        self.item_type.table_name()
    }

    /// Get the ID column name for this store's workshop item type in linking tables.
    pub fn id_column_name(&self) -> &'static str {
        self.item_type.id_column_name()
    }

    /// Link a workshop item to an entity (package or board).
    pub async fn link_item_to_entity(
        &self,
        entity_table: &str, // "package" or "board" 
        entity_id: i64,
        workshop_item_id: i64,
        origin_id: Option<i64>,
        name_override: Option<&str>,
        trx: &mut Trx,
    ) -> Result<EntityWorkshopItemDb, DbErr> {
        let now: i64 = to_unix_timestamp(SystemTime::now()).try_into().unwrap_or(0);
        let transaction = trx.get_mut();

        let link_table = format!("{}_{}", entity_table, self.table_name());
        let item_id_col = format!("{}_id", entity_table);
        let workshop_id_col = self.id_column_name();

        // Determine if the link table has a name column (board tables have name, package tables don't)
        let has_name_col = entity_table == "board";
        
        let sql = if has_name_col {
            format!(
                "INSERT INTO {} ({}, {}, origin_id, name, created_at) VALUES (?1, ?2, ?3, ?4, ?5) RETURNING item_id as item_id, {} as workshop_item_id, origin_id, name, created_at",
                link_table, item_id_col, workshop_id_col, workshop_id_col
            )
        } else {
            format!(
                "INSERT INTO {} ({}, {}, origin_id, created_at) VALUES (?1, ?2, ?3, ?4) RETURNING item_id as item_id, {} as workshop_item_id, origin_id, NULL as name, created_at",
                link_table, item_id_col, workshop_id_col, workshop_id_col
            )
        };

        let mut query = sqlx::query_as::<_, EntityWorkshopItemDb>(&sql)
            .bind(entity_id)
            .bind(workshop_item_id)
            .bind(origin_id);

        if has_name_col {
            query = query.bind(name_override);
        }

        let result = query.bind(now).fetch_one(&mut **transaction).await;

        result
    }

    /// Unlink a workshop item from an entity.
    pub async fn unlink_item_from_entity(
        &self,
        entity_table: &str,
        entity_id: i64,
        workshop_item_id: i64,
        origin_id: Option<i64>,
        trx: &mut Trx,
    ) -> Result<EntityWorkshopItemDb, DbErr> {
        let transaction = trx.get_mut();

        let link_table = format!("{}_{}", entity_table, self.table_name());
        let item_id_col = format!("{}_id", entity_table);
        let workshop_id_col = self.id_column_name();

        let sql = format!(
            "DELETE FROM {} WHERE {} = ?1 AND {} = ?2 AND origin_id IS ?3 RETURNING item_id as item_id, {} as workshop_item_id, origin_id, name, created_at",
            link_table, item_id_col, workshop_id_col, workshop_id_col
        );

        let result = sqlx::query_as::<_, EntityWorkshopItemDb>(&sql)
            .bind(entity_id)
            .bind(workshop_item_id)
            .bind(origin_id)
            .fetch_one(&mut **transaction)
            .await;

        result
    }

    /// List workshop items linked to an entity.
    pub async fn list_entity_items(
        &self,
        entity_table: &str,
        entity_id: i64,
        lister: &WorkshopItemLister<'_>,
        trx: &mut Trx,
    ) -> Result<Vec<WorkshopItemDb>, DbErr> {
        let transaction = trx.get_mut();

        let link_table = format!("{}_{}", entity_table, self.table_name());
        let item_id_col = format!("{}_id", entity_table);
        let workshop_id_col = self.id_column_name();
        let item_table = self.table_name();

        let select = vec![
            format!("i.id"),
            format!("i.name"),
            format!("i.slug"),
            format!("i.description"),
            format!("i.definition"),
            format!("i.created_at"),
            format!("i.updated_at"),
        ];

        let from = vec![
            format!("{} i", item_table),
            format!("JOIN {} l ON i.id = l.{}", link_table, workshop_id_col),
        ];

        let mut where_clauses = vec![format!("l.{} = ?", item_id_col)];

        if let Some(filter) = &lister.filter {
            if let Some(_search) = &filter.search {
                where_clauses.push(format!("i.name LIKE ?"));
            }

            if let Some(ids) = &filter.ids {
                if !ids.is_empty() {
                    let ids_str: String = ids.iter().map(|id| id.to_string()).collect::<Vec<_>>().join(",");
                    where_clauses.push(format!("i.id IN ({})", ids_str));
                }
            }
        }

        let mut paging = Vec::new();
        paging.push(format!(" LIMIT {}", lister.pager.limit));
        if let Some(offset) = lister.pager.offset {
            paging.push(format!(" OFFSET {}", offset));
        }

        let mut sql = format!(
            "SELECT {} FROM {} WHERE {}",
            select.join(", "),
            from.join(" "),
            where_clauses.join(" AND ")
        );

        for p in paging {
            sql.push_str(&p);
        }

        let mut q = sqlx::query_as::<_, WorkshopItemDb>(&sql).bind(entity_id);

        if let Some(filter) = &lister.filter {
            if let Some(search) = &filter.search {
                let search_pattern = format!("%{}%", search);
                q = q.bind(search_pattern);
            }
        }

        let results = q.fetch_all(&mut **transaction).await;

        results
    }
}

impl<'a> CrudQueries<'a> for WorkshopStore {
    type Item = WorkshopItemDb;
    type NewItem = NewWorkshopItemDb<'a>;
    type PatchItem = PatchWorkshopItemDb<'a>;
    type Lister = WorkshopItemLister<'a>;
    type Id = i64;

    /// Create a new workshop item in the global workshop.
    async fn add_item(&self, item: &Self::NewItem, trx: &mut Trx) -> Result<Self::Item, DbErr> {
        let now: i64 = to_unix_timestamp(SystemTime::now()).try_into().unwrap_or(0);
        let transaction = trx.get_mut();
        
        let table = self.table_name();
        let sql = format!(
            "INSERT INTO {} (name, slug, description, definition, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?5) RETURNING id, name, slug, description, definition, created_at, updated_at",
            table
        );

        let result = sqlx::query_as::<_, Self::Item>(&sql)
            .bind(item.name)
            .bind(item.slug)
            .bind(item.description)
            .bind(item.definition)
            .bind(now)
            .fetch_one(&mut **transaction).await;

        result
    }

    /// List all workshop items.
    /// No access control here use from service layer.
    async fn get_items(&self, lister: &Self::Lister, trx: &mut Trx) -> Result<Vec<Self::Item>, DbErr> {
        let transaction = trx.get_mut();
        
        let table = self.table_name();
        let alias = &table[0..1]; // Use first letter as alias (s for shape, l for line, etc.)

        let select = vec![
            format!("{}.id", alias),
            format!("{}.name", alias),
            format!("{}.slug", alias),
            format!("{}.description", alias),
            format!("{}.definition", alias),
            format!("{}.created_at", alias),
            format!("{}.updated_at", alias),
        ];
        
        let from = vec![format!("{} {}", table, alias)];
        let mut where_clauses = Vec::new();

        if let Some(filter) = &lister.filter {
            if let Some(_search) = &filter.search {
                where_clauses.push(format!("{}.name LIKE ?", alias));
            }

            if let Some(ids) = &filter.ids {
                if !ids.is_empty() {
                    let ids_str: String = ids.iter().map(|id| id.to_string()).collect::<Vec<_>>().join(",");
                    where_clauses.push(format!("{}.id IN ({})", alias, ids_str));
                }
            }
        }

        let mut paging = Vec::new();
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

        let mut q = sqlx::query_as::<_, Self::Item>(&sql);

        if let Some(filter) = &lister.filter {
            if let Some(search) = &filter.search {
                let search_pattern = format!("%{}%", search);
                q = q.bind(search_pattern);
            }
        }

        let results = q.fetch_all(&mut **transaction).await;

        results
    }

    /// Get a workshop item by id.
    async fn get_item(&self, id: Self::Id, _access: Option<&'a str>, trx: &mut Trx) -> Result<Self::Item, DbErr> {
        let transaction = trx.get_mut();
        
        let table = self.table_name();
        let alias = &table[0..1]; // Use first letter as alias
        
        let select = vec![
            format!("{}.id", alias),
            format!("{}.name", alias),
            format!("{}.slug", alias),
            format!("{}.description", alias),
            format!("{}.definition", alias),
            format!("{}.created_at", alias),
            format!("{}.updated_at", alias),
        ];
        
        let from = vec![format!("{} {}", table, alias)];
        let where_clauses = vec![format!("{}.id = ?1", alias)];

        let sql = format!(
            "SELECT {} FROM {} WHERE {}",
            select.join(", "),
            from.join(", "),
            where_clauses.join(" AND ")
        );

        let result = sqlx::query_as::<_, Self::Item>(&sql)
            .bind(id)
            .fetch_one(&mut **transaction)
            .await;

        result
    }

    /// Update an existing workshop item.
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

        let table = self.table_name();
        let sql = format!(
            "UPDATE {} SET {} WHERE id = ? RETURNING id, name, slug, description, definition, created_at, updated_at",
            table, set
        );

        let mut q = sqlx::query_as::<_, Self::Item>(&sql);

        if let Some(name) = patch.name {
            q = q.bind(name);
        }

        if let Some(description) = patch.description {
            q = q.bind(description);
        }

        if let Some(definition) = patch.definition {
            q = q.bind(definition);
        }

        let result = q.bind(now).bind(id).fetch_one(&mut **transaction).await;

        result
    }

    /// Delete a workshop item by id.
    async fn delete_item(&self, id: Self::Id, trx: &mut Trx) -> Result<Self::Item, DbErr> {
        let transaction = trx.get_mut();
        
        let table = self.table_name();
        let sql = format!(
            "DELETE FROM {} WHERE id = ? RETURNING id, name, slug, description, definition, created_at, updated_at",
            table
        );
        
        let result = sqlx::query_as::<_, Self::Item>(&sql)
            .bind(id)
            .fetch_one(&mut **transaction)
            .await;

        result
    }
}

impl WorkshopStore {
    /// Get a workshop item by slug for semantic lookups.
    pub async fn get_item_by_slug(&self, slug: &str, trx: &mut Trx) -> Result<WorkshopItemDb, DbErr> {
        let transaction = trx.get_mut();
        
        let table = self.table_name();
        let sql = format!(
            "SELECT id, name, slug, description, definition, created_at, updated_at FROM {} WHERE slug = ?",
            table
        );
        
        let result = sqlx::query_as::<_, WorkshopItemDb>(&sql)
            .bind(slug)
            .fetch_one(&mut **transaction)
            .await;

        result
    }
}
