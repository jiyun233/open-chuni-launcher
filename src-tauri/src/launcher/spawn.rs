use std::path::{Path, PathBuf};

use crate::inject::build_command;
use crate::model::inject::InjectSpec;

use super::process::spawn_logged;
use super::types::{AMDAEMON_EXE, BIN_DIR, GAME_EXE, GAME_INJECTOR_X86};

pub fn resolve_bin_dir(game_root: &Path) -> PathBuf {
    let bin = game_root.join(BIN_DIR);
    if bin.is_dir() {
        bin
    } else {
        game_root.to_path_buf()
    }
}

fn amdaemon_spec() -> InjectSpec {
    InjectSpec {
        injector: "inject_x64.exe",
        exe: AMDAEMON_EXE.to_string(),
        dlls: vec!["chusanamhook.dll".to_string()],
        target_args: [
            "-f",
            "-c",
            "config_common.json",
            "config_server.json",
            "config_client.json",
            "config_cvt.json",
            "config_sp.json",
            "config_hook.json",
        ]
        .map(String::from)
        .to_vec(),
    }
}

fn game_spec(dlls: Vec<String>) -> InjectSpec {
    InjectSpec {
        injector: GAME_INJECTOR_X86,
        exe: GAME_EXE.to_string(),
        dlls,
        target_args: vec![],
    }
}

pub fn spawn_amdaemon(
    bin_dir: &Path,
    on_line: impl Fn(&str) + Send + Sync + Clone + 'static,
) -> Result<(), String> {
    let mut built = build_command(bin_dir, &amdaemon_spec());
    spawn_logged(&mut built.command, "amdaemon", on_line)
        .map_err(|error| format!("启动 amdaemon 失败：{error}"))
}

pub fn spawn_game(
    bin_dir: &Path,
    dlls: Vec<String>,
    on_line: impl Fn(&str) + Send + Sync + Clone + 'static,
) -> Result<Vec<String>, String> {
    let mut built = build_command(bin_dir, &game_spec(dlls));
    spawn_logged(&mut built.command, "game", on_line)
        .map_err(|error| format!("启动游戏失败：{error}"))?;
    Ok(built.missing_dlls)
}
