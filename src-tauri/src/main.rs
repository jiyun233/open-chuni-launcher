// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


fn main() {
    #[cfg(windows)]
    {
        if std::env::consts::ARCH != "x86_64" {
            show_error_popup("本启动器仅支持在 Windows x86_64 下运行");
            std::process::exit(1);
        }
        open_chunithm_launcher_lib::run()
    }

    #[cfg(not(windows))]
    {
        show_error_popup("本启动器仅支持在 Windows x86_64 下运行");
        std::process::exit(1);
    }
}

fn show_error_popup(msg: &str) {
    #[cfg(windows)]
    {
        use windows::core::HSTRING;
        use windows::Win32::UI::WindowsAndMessaging::{
            MessageBoxW, MB_ICONERROR, MB_OK,
        };
        unsafe {
            MessageBoxW(
                None,
                &HSTRING::from(msg),
                &HSTRING::from("启动错误"),
                MB_OK | MB_ICONERROR,
            );
        }
    }
    #[cfg(target_os = "macos")]
    {
        // 调 osascript 弹窗（简单粗暴）
        let _ = std::process::Command::new("osascript")
            .args(["-e", &format!(
                "display dialog \"{}\" buttons {{\"OK\"}} default button \"OK\" with icon stop",
                msg.replace('"', "\\\"")
            )])
            .status();
    }
    #[cfg(all(unix, not(target_os = "macos")))]
    {
        // 有 zenity 就用，没有就退化为 stderr
        let ok = std::process::Command::new("zenity")
            .args(["--error", "--text", msg])
            .status()
            .is_ok();
        if !ok {
            eprintln!("{}", msg);
        }
    }
}