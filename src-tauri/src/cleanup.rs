#![cfg(windows)]

use std::time::{Duration, Instant};

use sysinfo::{Pid, Process, System};

const STALE_KILL_TIMEOUT: Duration = Duration::from_secs(2);
const STALE_POLL_INTERVAL: Duration = Duration::from_millis(200);

pub fn remove_stale() {
    let markers = StaleMarkers::collect();
    let mut system = match System::new_all() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("初始化系统信息失败: {}", e);
            return;
        }
    };
    if kill_stale(&system, &markers) == 0 {
        return;
    }

    let deadline = Instant::now() + STALE_KILL_TIMEOUT;
    while Instant::now() < deadline {
        system.refresh_all();
        if !has_stale(&system, &markers) {
            return;
        }
        std::thread::sleep(STALE_POLL_INTERVAL);
    }
}

fn kill_stale(system: &System, markers: &StaleMarkers) -> usize {
    let self_pid = Pid::from_u32(std::process::id());
    system
        .processes()
        .iter()
        .filter(|(pid, process)| **pid != self_pid && markers.matches(process))
        .filter(|(_, process)| process.kill())
        .count()
}

fn has_stale(system: &System, markers: &StaleMarkers) -> bool {
    let self_pid = Pid::from_u32(std::process::id());
    system
        .processes()
        .iter()
        .any(|(pid, process)| *pid != self_pid && markers.matches(process))
}

struct StaleMarkers {
    webview_marker: String,
    launcher_exe: String,
}

impl StaleMarkers {
    fn collect() -> Self {
        Self {
            webview_marker: read_identifier(),
            launcher_exe: std::env::current_exe()
                .ok()
                .and_then(|path| Some(path.file_name()?.to_string_lossy().to_lowercase()))
                .unwrap_or_else(|| "open-chunithm-launcher.exe".to_string()),
        }
    }

    fn matches(&self, process: &Process) -> bool {
        let name = process.name().to_string_lossy().to_lowercase();
        let cmdline = process
            .cmd()
            .iter()
            .map(|arg| arg.to_string_lossy())
            .collect::<Vec<_>>()
            .join(" ");
        self.matches_parts(&name, &cmdline)
    }

    fn matches_parts(&self, name: &str, cmdline: &str) -> bool {
        let name = name.to_lowercase();
        if name == self.launcher_exe {
            return true;
        }
        name == "msedgewebview2.exe" && cmdline.to_lowercase().contains(&self.webview_marker)
    }
}

fn read_identifier() -> String {
    serde_json::from_str::<serde_json::Value>(include_str!("../tauri.conf.json"))
        .ok()
        .and_then(|config| Some(config.get("identifier")?.as_str()?.to_string()))
        .unwrap_or_else(|| "cc.moeneko.open-chunithm-launcher".to_string())
}
