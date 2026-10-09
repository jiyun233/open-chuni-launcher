use crate::display;
use crate::model::display::MonitorInfo;

#[tauri::command]
pub async fn list_monitors() -> Result<Vec<MonitorInfo>, String> {
    tauri::async_runtime::spawn_blocking(display::list_monitors)
        .await
        .map_err(|error| format!("内部任务异常：{error}"))
}
