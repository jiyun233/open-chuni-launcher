use std::sync::Mutex;

use tauri::{AppHandle, Emitter};

use crate::launcher::SessionEvent;

#[derive(Default)]
pub struct LauncherState(pub Mutex<bool>);

pub struct TauriSessionEvents {
    pub app: AppHandle,
}

impl SessionEvent for TauriSessionEvents {
    fn log(&self, message: &str) {
        let _ = self.app.emit("launch://log", message.to_string());
    }

    fn running(&self, running: bool) {
        let _ = self.app.emit("launch://state", running);
    }
}
