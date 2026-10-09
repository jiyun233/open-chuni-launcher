use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct LaunchReport {
    pub missing_dlls: Vec<String>,
}

#[derive(Deserialize)]
pub struct BuiltinPluginSource {
    pub dll: String,
    pub source: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DisplayModeRequest {
    pub monitor: u32,
    pub width: u32,
    pub height: u32,
    pub refresh_rate: u32,
}
