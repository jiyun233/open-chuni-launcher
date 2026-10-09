use std::path::PathBuf;
use std::sync::Arc;
use std::time::{Duration, Instant};

use crate::display;
use crate::model::launch::{BuiltinPluginSource, DisplayModeRequest, LaunchReport};
use crate::model::segatools::SegatoolsPatch;
use crate::segatools::patch_ini;

use super::process::{kill_by_image, process_exists};
use super::spawn::{resolve_bin_dir, spawn_amdaemon, spawn_game};
use super::types::{AMDAEMON_EXE, GAME_EXE, GAME_INJECTOR_X86};

pub trait SessionEvent: Send + Sync {
    fn log(&self, message: &str);
    fn running(&self, running: bool);
}

pub fn run_session(
    events: Arc<dyn SessionEvent>,
    game_root: &str,
    dlls: Vec<String>,
    builtin_plugins: Option<Vec<BuiltinPluginSource>>,
    segatools: Option<SegatoolsPatch>,
    display: Option<DisplayModeRequest>,
    launch_timeout_secs: u64,
) -> Result<LaunchReport, String> {
    let game_root = PathBuf::from(game_root);
    if !game_root.is_dir() {
        return Err(format!("游戏根目录不存在：{}", game_root.display()));
    }
    let bin_dir = resolve_bin_dir(&game_root);
    if !bin_dir.join(GAME_INJECTOR_X86).is_file() {
        return Err(format!(
            "游戏目录结构不正确：{} 中未找到 {}",
            bin_dir.display(),
            GAME_INJECTOR_X86
        ));
    }

    if let Some(builtin_plugins) = builtin_plugins {
        for plugin in builtin_plugins {
            let target = bin_dir.join(&plugin.dll);
            let source = PathBuf::from(&plugin.source);
            if source != target {
                std::fs::copy(&source, &target).map_err(|error| {
                    format!("无法安装内置插件 {}：{error}", plugin.dll)
                })?;
            }
        }
    }

    if let Some(patch) = &segatools {
        patch_ini(&bin_dir, &patch.sections())?;
        events.log("已将配置写入 segatools.ini");
    }

    events.log("正在清理残留的游戏进程…");
    kill_by_image(GAME_EXE);
    kill_by_image(GAME_INJECTOR_X86);
    kill_by_image(AMDAEMON_EXE);

    let mut display_guard: Option<display::DisplayGuard> = match &display {
        Some(request) => match display::apply_mode(
            request.monitor,
            request.width,
            request.height,
            request.refresh_rate,
        ) {
            Ok(Some(guard)) => {
                let refresh = if request.refresh_rate > 0 {
                    format!("{} Hz", request.refresh_rate)
                } else {
                    "当前刷新率".to_string()
                };
                events.log(&format!(
                    "已将显示器 {} 切换到 {}×{} @ {}，游戏结束后自动恢复",
                    request.monitor, request.width, request.height, refresh
                ));
                Some(guard)
            }
            Ok(None) => None,
            Err(error) => {
                events.log(&format!("警告：切换显示模式失败，已按原模式继续：{error}"));
                None
            }
        },
        None => None,
    };

    events.log("正在启动 amdaemon…");
    let events_amdaemon = events.clone();
    spawn_amdaemon(&bin_dir, move |line| events_amdaemon.log(line))?;

    events.log("正在启动游戏…");
    let events_game = events.clone();
    let missing_dlls = match spawn_game(&bin_dir, dlls, move |line| events_game.log(line)) {
        Ok(missing_dlls) => missing_dlls,
        Err(error) => {
            kill_by_image(AMDAEMON_EXE);
            return Err(error);
        }
    };

    events.running(true);

    events.log("等待游戏进程出现…");
    let deadline = Instant::now() + Duration::from_secs(launch_timeout_secs.max(1));
    loop {
        if process_exists(GAME_EXE) {
            break;
        }
        if Instant::now() >= deadline {
            kill_by_image(GAME_EXE);
            kill_by_image(GAME_INJECTOR_X86);
            kill_by_image(AMDAEMON_EXE);
            events.running(false);
            return Err("等待游戏进程出现超时，请检查游戏目录与注入配置".to_string());
        }
        std::thread::sleep(Duration::from_millis(500));
    }

    events.log("游戏运行中，等待退出…");
    while process_exists(GAME_EXE) {
        std::thread::sleep(Duration::from_millis(1000));
    }

    events.log("正在清理 amdaemon…");
    kill_by_image(AMDAEMON_EXE);

    if let Some(guard) = display_guard.as_mut() {
        events.log("正在恢复显示器显示模式…");
        guard.restore();
        events.log("显示器显示模式已恢复");
    }
    drop(display_guard);

    if !missing_dlls.is_empty() {
        events.log(&format!(
            "以下 DLL 文件不存在，已跳过：{}",
            missing_dlls.join(", ")
        ));
    }
    events.log("游戏进程已全部结束");
    events.running(false);

    Ok(LaunchReport { missing_dlls })
}
