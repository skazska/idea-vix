use crate::package::package_types::{NewPackageItem, PatchPackageItem};
use crate::boards::board_types::{NewBoardItem, PatchBoardItem};

#[derive(sqlx::FromRow, Debug)]
pub struct PackageDb {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub icon: Option<String>,
}

#[derive(Debug)]
pub struct NewPackageDb<'a> {
    pub name: &'a str,
    pub description: Option<&'a str>,
    pub icon: Option<&'a str>,
}

impl<'a> From<&'a NewPackageItem> for NewPackageDb<'a> {
    fn from(item: &'a NewPackageItem) -> Self {
        Self {
            name: &item.name,
            description: item.description.as_deref(),
            icon: item.icon.as_deref(),
        }
    }
}

#[derive(Debug)]
pub struct PatchPackageDb<'a> {
    pub name: Option<&'a str>,
    pub description: Option<Option<&'a str>>,
    pub icon: Option<Option<&'a str>>,
}

impl<'a> From<&'a PatchPackageItem> for PatchPackageDb<'a> {
    fn from(item: &'a PatchPackageItem) -> Self {
        Self {
            name: item.name.as_deref(),
            description: item.description.as_ref().map(|d| d.as_deref()),
            icon: item.icon.as_ref().map(|i| i.as_deref()),
        }
    }
}

// Board database models
#[derive(sqlx::FromRow, Debug)]
pub struct BoardDb {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub icon: Option<String>,
}

#[derive(Debug)]
pub struct NewBoardDb<'a> {
    pub name: &'a str,
    pub description: Option<&'a str>,
    pub icon: Option<&'a str>,
}

impl<'a> From<&'a NewBoardItem> for NewBoardDb<'a> {
    fn from(item: &'a NewBoardItem) -> Self {
        Self {
            name: &item.name,
            description: item.description.as_deref(),
            icon: item.icon.as_deref(),
        }
    }
}

#[derive(Debug)]
pub struct PatchBoardDb<'a> {
    pub name: Option<&'a str>,
    pub description: Option<Option<&'a str>>,
    pub icon: Option<Option<&'a str>>,
}

impl<'a> From<&'a PatchBoardItem> for PatchBoardDb<'a> {
    fn from(item: &'a PatchBoardItem) -> Self {
        Self {
            name: item.name.as_deref(),
            description: item.description.as_ref().map(|d| d.as_deref()),
            icon: item.icon.as_ref().map(|i| i.as_deref()),
        }
    }
}
