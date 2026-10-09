use std::sync::Arc;

use tauri::{AppHandle, Manager, State};

use crate::launcher;
use crate::model::launch::{BuiltinPluginSource, DisplayModeRequest, LaunchReport};
use crate::model::segatools::SegatoolsPatch;

use super::types::{LauncherState, TauriSessionEvents};

#[tauri::command]
pub async fn launch_game(
    app: AppHandle,
    game_dir: String,
    dlls: Vec<String>,
    builtin_plugins: Option<Vec<BuiltinPluginSource>>,
    segatools: Option<SegatoolsPatch>,
    display: Option<DisplayModeRequest>,
    launch_timeout_secs: Option<u64>,
) -> Result<LaunchReport, String> {
    if let Some(patch) = &segatools {
        if let Some(keychip) = &patch.keychip {
            // 机台编号不做格式校验，只要不为空即可启动
            if keychip.id.trim().is_empty() {
                return Err("机台编号未填写，无法启动游戏".to_string());
            }
        }
    }
    {
        let state = app.state::<LauncherState>();
        let mut running = state.0.lock().map_err(|_| "内部状态异常".to_string())?;
        if *running {
            return Err("已有游戏会话正在运行".to_string());
        }
        *running = true;
    }

    let session_app = app.clone();
    let result = tauri::async_runtime::spawn_blocking(move || {
        let events = Arc::new(TauriSessionEvents { app: session_app });
        launcher::run_session(
            events,
            &game_dir,
            dlls,
            builtin_plugins,
            segatools,
            display,
            launch_timeout_secs.unwrap_or(15),
        )
    })
    .await
    .map_err(|error| format!("内部任务异常：{error}"))?;

    if let Ok(mut running) = app.state::<LauncherState>().0.lock() {
        *running = false;
    }
    result
}

#[tauri::command]
pub fn is_running(state: State<'_, LauncherState>) -> bool {
    *state.0.lock().unwrap_or_else(|poisoned| poisoned.into_inner())
}

#[tauri::command]
pub fn stop_game() -> Result<(), String> {
    launcher::kill_by_image(launcher::GAME_EXE);
    launcher::kill_by_image(launcher::GAME_INJECTOR_X86);
    launcher::kill_by_image(launcher::AMDAEMON_EXE);
    Ok(())
}
