use serde::Serialize;

#[derive(Serialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DisplayModeInfo {
    pub width: u32,
    pub height: u32,
    pub refresh_rate: u32,
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MonitorInfo {
    pub index: u32,
    pub device: String,
    pub name: String,
    pub is_primary: bool,
    pub width: u32,
    pub height: u32,
    pub refresh_rate: u32,
    pub modes: Vec<DisplayModeInfo>,
}
