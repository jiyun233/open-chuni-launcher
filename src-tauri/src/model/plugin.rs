use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize, Default)]
#[serde(default)]
pub struct PluginManifest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub version: Option<String>,
    pub author: Option<String>,
}

#[derive(Deserialize, Default)]
#[serde(default)]
pub struct PluginLocaleOverrides {
    pub name: Option<String>,
    pub description: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PluginInfo {
    pub file: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    pub path: String,
    pub icon_data_url: Option<String>,
    pub has_manifest: bool,
}
