use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Versions {
    #[serde(flatten)]
    pub versions: HashMap<String, String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModEntry {
    pub id: Uuid,
    pub title_slug: String,
    pub display_title: String,
    pub author_slug: String,
    pub display_author: String,
    pub description: String,
    pub versions: Versions,
    pub config_files: Vec<String>,
    pub dependencies: Vec<Uuid>,
}
