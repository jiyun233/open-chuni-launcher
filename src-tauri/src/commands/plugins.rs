use std::path::PathBuf;

use tauri::AppHandle;

use crate::model::plugin::PluginInfo;
use crate::plugins;

#[tauri::command]
pub async fn list_plugin_dlls(
    app: AppHandle,
    directory: Option<String>,
) -> Result<Vec<PluginInfo>, String> {
    let directory = match directory {
        Some(directory) => PathBuf::from(directory),
        None => plugins::plugins_dir(&app).ok_or_else(|| "未找到内置插件目录".to_string())?,
    };
    tauri::async_runtime::spawn_blocking(move || Ok(plugins::scan_plugins(&directory)))
        .await
        .map_err(|error| format!("内部任务异常：{error}"))?
}
