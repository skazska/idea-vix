/// Shape commons service for managing linked shapes in the workshop (CRUD).

use std::sync::Arc;

use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{common::workshop::shape_store::{NewShapeDb, PatchShapeDb, ShapeDb, ShapeStore}, db::TransactionStarter};


lazy_static! {
    static ref SLUG_REGEX: Regex = Regex::new(r"^[a-z][a-z0-9-]*$").unwrap();
}

/// API representation of a shape.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Shape {
    pub id: i64,
    pub name: String,
    pub slug: String, // semantic identifier for text-based references
    pub description: Option<String>,
    pub definition: serde_json::Value,
    pub created_at: i64,
    pub updated_at: i64,
}

/// Request model for creating a new shape.
#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct NewShapeItem {
    #[validate(length(min = 3, max = 100))]
    pub name: String,
    #[validate(length(min = 3, max = 100))]
    #[validate(regex(path = "*SLUG_REGEX"))]
    pub slug: String,
    #[validate(length(max = 500))]
    pub description: Option<String>,
    pub definition: serde_json::Value,
}

/// Request model for updating an existing shape.
#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct PatchShapeItem {
    #[validate(length(min = 3, max = 100))]
    pub name: Option<String>,
    #[validate(length(max = 500))]
    pub description: Option<Option<String>>,
    pub definition: Option<serde_json::Value>,
}

impl<'a> From<&'a NewShapeItem> for NewShapeDb<'a> {
    fn from(item: &'a NewShapeItem) -> Self {
        Self {
            name: &item.name,
            slug: &item.slug,
            description: item.description.as_deref(),
            definition: &item.definition,
        }
    }
}

impl<'a> From<&'a PatchShapeItem> for PatchShapeDb<'a> {
    fn from(item: &'a PatchShapeItem) -> Self {
        Self {
            name: item.name.as_deref(),
            description: item.description.as_ref().map(|opt| opt.as_deref()),
            definition: item.definition.as_ref(),
        }
    }
}

impl From<ShapeDb> for Shape {
    fn from(db: ShapeDb) -> Self {
        Self {
            id: db.id,
            name: db.name,
            slug: db.slug,
            description: db.description,
            definition: db.definition,
            created_at: db.created_at.try_into().unwrap_or(0),
            updated_at: db.updated_at.try_into().unwrap_or(0),
        }
    }
}



/// Request model for adding a shape to a package.
#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct AddShapeToPackageItem {
    pub shape_id: i32,
    #[validate(length(max = 100))]
    pub name: Option<String>, // optional display name override
}

/// Request model for updating a package-shape association.
#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct UpdatePackageShapeItem {
    #[validate(length(max = 100))]
    pub name: Option<String>,
}





pub struct LinkedShapeService {
    transaction_starter: Arc<TransactionStarter>,
    store: Arc<ShapeStore>,
}

impl LinkedShapeService {
    /// Create a new LinkedShapeService with the given ShapeStore.
    pub fn new(transaction_starter: Arc<TransactionStarter>, store: Arc<ShapeStore>) -> Self {
        Self { transaction_starter, store }
    }

    // Add methods for managing linked shapes here.
}

