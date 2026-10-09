use std::path::{Path, PathBuf};
use std::process::Command;

use crate::model::inject::{BuiltInjection, InjectSpec};

fn resolve_dll(bin_dir: &Path, dll: &str) -> PathBuf {
    let path = Path::new(dll);
    if path.is_absolute() {
        path.to_path_buf()
    } else {
        bin_dir.join(path)
    }
}

pub fn build_command(bin_dir: &Path, spec: &InjectSpec) -> BuiltInjection {
    let mut missing_dlls = Vec::new();
    let mut present_dlls = Vec::new();
    for dll in &spec.dlls {
        if resolve_dll(bin_dir, dll).is_file() {
            present_dlls.push(dll.clone());
        } else {
            missing_dlls.push(dll.clone());
        }
    }

    let mut command = Command::new(bin_dir.join(spec.injector));
    command
        .current_dir(bin_dir)
        .arg("-d");
    for dll in &present_dlls {
        command.arg("-k").arg(dll);
    }
    command.arg(&spec.exe).args(&spec.target_args);

    BuiltInjection {
        command,
        missing_dlls,
    }
}
