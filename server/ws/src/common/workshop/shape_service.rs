//! Workshop shape service module: business logic for managing shapes.
//!
//! Responsibilities:
//! - Validate and transform API payloads to storage models
//! - Enforce access control for shape operations
//! - Convert storage models to API responses
//! - Delegate persistence to the storage layer (`ShapeStore`)

use crate::error::ModelError;
use crate::session::session_service::SessionData;
use crate::common::workshop::shape_store::{GetBy, NewShapeDb, PatchShapeDb, ShapeDb, ShapeStore};
use serde::{Deserialize, Serialize};
use validator::{Validate};
use lazy_static::lazy_static;
use regex::Regex;

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




/// Service layer encapsulating business logic for workshop shapes.
pub struct ShapeService {
    items_store: ShapeStore,
}

impl ShapeService {
    /// Create a new `ShapeService` with the given store.
    pub fn new(shape_store: ShapeStore) -> Self {
        Self { items_store: shape_store }
    }

    /// Returns the created shape or an error.
    /// No access control here use from 
    pub async fn add_item(&self, item: &NewShapeItem, _session: &SessionData) -> Result<Shape, ModelError> {
        // Attempt to create the shape
        let result = self.items_store.add_item(item.into()).await?;

        Ok(result.into())
    }

    /// Get a shape by its unique ID.
    pub async fn get_item(&self, id: i32) -> Result<Shape, ModelError> {
        let item = self.items_store.get_item(GetBy::Id(id)).await?;

        Ok(item.into())
    }

    /// Get a shape by its unique slug.
    pub async fn get_shape_by_slug(&self, slug: &str) -> Result<Shape, ModelError> {
        let item = self.items_store.get_item(GetBy::Slug(slug.to_string())).await?;

        Ok(item.into())
    }

    /// Update an existing shape.
    /// No access control here use from package or board context
    pub async fn update_item(&self, id: i32, item: &PatchShapeItem, _session: &SessionData) -> Result<Shape, ModelError> {
        let result = self.items_store.update_item(id, item.into()).await?;

        Ok(result.into())
    }


    /// List all shapes in the global workshop catalog.
    pub async fn get_items(&self, ids: &Vec<i64>) -> Result<Vec<Shape>, ModelError> {
        let items = self.items_store.get_items(ids).await?;

        Ok(items.into_iter().map(|item| item.into()).collect())
    }

}
