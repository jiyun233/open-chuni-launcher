mod cleanup;
mod commands;
mod display;
mod inject;
mod launcher;
mod model;
mod plugins;
mod process_guard;
mod segatools;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    process_guard::setup();
    #[cfg(windows)]
    cleanup::remove_stale();

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .manage(commands::LauncherState::default())
        .invoke_handler(tauri::generate_handler![
            commands::game::launch_game,
            commands::game::stop_game,
            commands::game::is_running,
            commands::plugins::list_plugin_dlls,
            commands::display::list_monitors
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
