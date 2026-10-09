use std::io::{BufRead, BufReader, Read};
use std::process::{Command, Stdio};

use super::types::OPENSSL_IA32CAP;

fn stylize(command: &mut Command) {
    command.env("OPENSSL_ia32cap", OPENSSL_IA32CAP);
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x0800_0000;
        command.creation_flags(CREATE_NO_WINDOW);
    }
}

pub fn kill_by_image(image: &str) {
    let mut command = Command::new("taskkill");
    command.args(["/F", "/IM", image]);
    stylize(&mut command);
    let _ = command.output();
}

pub fn process_exists(image: &str) -> bool {
    let mut command = Command::new("tasklist");
    command.args(["/FI", &format!("IMAGENAME eq {image}"), "/NH"]);
    stylize(&mut command);
    let Ok(output) = command.output() else {
        return false;
    };
    String::from_utf8_lossy(&output.stdout)
        .to_lowercase()
        .contains(&image.to_lowercase())
}

pub(super) fn spawn_logged(
    command: &mut Command,
    source: &'static str,
    on_line: impl Fn(&str) + Send + Sync + Clone + 'static,
) -> std::io::Result<()> {
    command.stdout(Stdio::piped()).stderr(Stdio::piped());
    let mut child = command.spawn()?;

    let stdout = child.stdout.take().map(BufReader::new);
    let stderr = child.stderr.take().map(BufReader::new);
    if let Some(stream) = stdout {
        let on_line = on_line.clone();
        std::thread::spawn(move || forward_lines(stream, source, &on_line));
    }
    if let Some(stream) = stderr {
        std::thread::spawn(move || forward_lines(stream, source, &on_line));
    }
    Ok(())
}

fn forward_lines<R: Read>(stream: R, source: &str, on_line: &impl Fn(&str)) {
    let mut reader = BufReader::new(stream);
    let mut buffer = Vec::new();
    loop {
        buffer.clear();
        match reader.read_until(b'\n', &mut buffer) {
            Ok(0) | Err(_) => break,
            Ok(_) => {
                let text = String::from_utf8_lossy(&buffer);
                let line = text.trim_end_matches(['\n', '\r']);
                if !line.is_empty() {
                    on_line(&format!("[{source}] {line}"));
                }
            }
        }
    }
}
