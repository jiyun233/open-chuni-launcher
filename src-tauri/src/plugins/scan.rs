use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use base64::Engine as _;
use serde_json::Value;
use tauri::{AppHandle, Manager};

use crate::model::plugin::{PluginInfo, PluginLocaleOverrides, PluginManifest};

pub fn plugins_dir(app: &AppHandle) -> Option<PathBuf> {
    if let Ok(resource_dir) = app.path().resource_dir() {
        let dir = resource_dir.join("plugins");
        if dir.is_dir() {
            return Some(dir);
        }
    }
    let dev_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("plugins");
    dev_dir.is_dir().then_some(dev_dir)
}

pub fn scan_plugins(dir: &Path) -> Vec<PluginInfo> {
    let Ok(entries) = fs::read_dir(dir) else {
        return Vec::new();
    };

    let mut plugins: Vec<PluginInfo> = entries
        .flatten()
        .filter(|entry| entry.path().is_dir())
        .filter_map(|entry| read_plugin_folder(&entry.path()))
        .collect();

    plugins.sort_by(|a, b| a.file.to_lowercase().cmp(&b.file.to_lowercase()));
    plugins
}

fn read_plugin_folder(folder: &Path) -> Option<PluginInfo> {
    let dll_path = find_dll(folder)?;
    let dll_file = dll_path.file_name()?.to_string_lossy().to_string();
    let stem = dll_path.file_stem()?.to_string_lossy().to_string();

    let manifest_path = folder.join(format!("{stem}.json"));
    let manifest_text = fs::read_to_string(&manifest_path).ok();
    let manifest: PluginManifest = manifest_text
        .as_deref()
        .and_then(|text| serde_json::from_str(text).ok())
        .unwrap_or_default();

    let (locale_names, locale_descriptions) = read_locale_overrides(folder, &stem);

    let icon_path = manifest
        .icon
        .as_deref()
        .map(|icon| folder.join(icon))
        .or_else(|| find_default_icon(folder, &stem))
        .and_then(|icon_path| icon_to_data_url(&icon_path));

    Some(PluginInfo {
        file: dll_file,
        name: build_localized_value(manifest.name, &locale_names, Some(stem.clone())),
        description: build_localized_value(manifest.description, &locale_descriptions, None),
        version: manifest.version,
        author: manifest.author,
        path: dll_path.to_string_lossy().to_string(),
        icon_data_url: icon_path,
        has_manifest: manifest_text.is_some(),
    })
}

fn find_dll(folder: &Path) -> Option<PathBuf> {
    let mut dlls: Vec<PathBuf> = fs::read_dir(folder)
        .ok()?
        .flatten()
        .map(|entry| entry.path())
        .filter(|path| {
            path.is_file()
                && path
                    .extension()
                    .is_some_and(|ext| ext.eq_ignore_ascii_case("dll"))
        })
        .collect();
    dlls.sort_by(|a, b| a.file_name().cmp(&b.file_name()));

    let folder_name = folder.file_name()?.to_string_lossy().to_lowercase();
    let preferred = dlls.iter().position(|path| {
        path.file_stem()
            .is_some_and(|stem| stem.to_string_lossy().to_lowercase() == folder_name)
    });
    match preferred {
        Some(index) => Some(dlls.remove(index)),
        None => dlls.into_iter().next(),
    }
}

fn read_locale_overrides(
    folder: &Path,
    stem: &str,
) -> (BTreeMap<String, String>, BTreeMap<String, String>) {
    let mut names = BTreeMap::new();
    let mut descriptions = BTreeMap::new();

    let Ok(entries) = fs::read_dir(folder) else {
        return (names, descriptions);
    };
    let prefix = format!("{stem}.");
    for entry in entries.flatten() {
        let file = entry.file_name().to_string_lossy().to_string();
        let Some(suffix) = file.strip_prefix(&prefix).and_then(|rest| rest.strip_suffix(".json"))
        else {
            continue;
        };
        let is_locale_id = suffix.starts_with(|c: char| c.is_ascii_alphabetic())
            && suffix
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || c == '-');
        if !is_locale_id {
            continue;
        }

        let Ok(text) = fs::read_to_string(entry.path()) else {
            continue;
        };
        let Ok(overrides) = serde_json::from_str::<PluginLocaleOverrides>(&text) else {
            continue;
        };
        if let Some(name) = overrides.name {
            names.insert(suffix.to_string(), name);
        }
        if let Some(description) = overrides.description {
            descriptions.insert(suffix.to_string(), description);
        }
    }
    (names, descriptions)
}

fn build_localized_value(
    base: Option<String>,
    overrides: &BTreeMap<String, String>,
    fallback: Option<String>,
) -> Option<Value> {
    let base = base.filter(|text| !text.trim().is_empty());
    if overrides.is_empty() {
        return base.or(fallback).map(Value::String);
    }

    let mut map = serde_json::Map::new();
    if let Some(text) = base {
        map.insert("en-US".to_string(), Value::String(text));
    }
    for (locale, text) in overrides {
        map.insert(locale.clone(), Value::String(text.clone()));
    }
    Some(Value::Object(map))
}

fn find_default_icon(folder: &Path, stem: &str) -> Option<PathBuf> {
    ["png", "jpg", "jpeg", "svg"]
        .iter()
        .map(|ext| folder.join(format!("{stem}.{ext}")))
        .find(|path| path.is_file())
}

fn icon_to_data_url(path: &Path) -> Option<String> {
    let bytes = fs::read(path).ok()?;
    let mime = match path.extension()?.to_string_lossy().to_lowercase().as_str() {
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "gif" => "image/gif",
        "svg" => "image/svg+xml",
        "ico" => "image/x-icon",
        _ => return None,
    };
    Some(format!(
        "data:{mime};base64,{}",
        base64::engine::general_purpose::STANDARD.encode(bytes)
    ))
}
