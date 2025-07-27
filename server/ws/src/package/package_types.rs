use crate::db::models::{ PackageDb };
use serde::{Deserialize, Deserializer, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Package {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub icon: Option<String>,
}


impl From<PackageDb> for Package {
    fn from(item: PackageDb) -> Self {
        Self {
            id: item.id,
            name: item.name,
            description: item.description,
            icon: item.icon,
        }
    }
}

// impl PackageItem {
//     pub fn from_db(item: PackageDb) -> Self {
//         Self {
//             id: item.id,
//             name: item.name,
//             description: item.description,
//             icon: item.icon,
//         }
//     }
// }

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct NewPackageItem {
    #[validate(length(min = 3, max = 100))]
    pub name: String,
    #[validate(length(max = 500))]
    pub description: Option<String>,
    #[validate(length(max = 255))]
    pub icon: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct PatchPackageItem {
    #[validate(length(min = 3, max = 100))]
    pub name: Option<String>,
    #[serde(default, deserialize_with = "deserialize_some")]
    #[validate(length(max = 500))]
    pub description: Option<Option<String>>,
    #[serde(default, deserialize_with = "deserialize_some")]
    #[validate(length(max = 255))]
    pub icon: Option<Option<String>>,
}

// Any value that is present is considered Some value, including null.
fn deserialize_some<'de, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
    where T: Deserialize<'de>,
          D: Deserializer<'de>
{
    Deserialize::deserialize(deserializer).map(Some)
}

// impl PatchPackageItem {
//     pub fn to_db(&self) -> DbNewPackageItem {
//         DbNewPackageItem {
//             name: self.name.as_deref(),
//             description: self.description.as_ref().and_then(|d| d.as_deref()),
//             icon: self.icon.as_ref().and_then(|i| i.as_deref()),
//         }
//     }
    
// }