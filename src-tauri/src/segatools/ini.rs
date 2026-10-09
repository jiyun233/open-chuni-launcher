use std::path::Path;

use crate::model::segatools::SectionPatch;

pub fn patch_ini(bin_dir: &Path, patches: &[SectionPatch]) -> Result<(), String> {
    let ini_path = bin_dir.join("segatools.ini");
    let content = std::fs::read_to_string(&ini_path).unwrap_or_default();
    let patched = apply_patches(&content, patches);
    std::fs::write(&ini_path, patched)
        .map_err(|error| format!("写入 segatools.ini 失败：{error}"))
}

struct IniSection {
    name: Option<String>,
    body: Vec<String>,
}

fn parse_sections(content: &str) -> Vec<IniSection> {
    let mut sections = vec![IniSection {
        name: None,
        body: Vec::new(),
    }];
    for raw in content.lines() {
        let trimmed = raw.trim();
        if trimmed.starts_with('[') && trimmed.ends_with(']') {
            sections.push(IniSection {
                name: Some(trimmed[1..trimmed.len() - 1].trim().to_lowercase()),
                body: Vec::new(),
            });
        } else {
            sections
                .last_mut()
                .expect("至少存在一个节")
                .body
                .push(raw.to_string());
        }
    }
    sections
}

fn key_of(line: &str) -> Option<String> {
    let eq = line.find('=')?;
    Some(line[..eq].trim().to_lowercase())
}

fn apply_patches(content: &str, patches: &[SectionPatch]) -> String {
    let mut sections = parse_sections(content);

    for patch in patches {
        let managed_keys: Vec<String> =
            patch.entries.iter().map(|entry| entry.key.to_lowercase()).collect();
        let managed_lines: Vec<String> = patch
            .entries
            .iter()
            .map(|entry| format!("{} = {}", entry.key, entry.value))
            .collect();

        if let Some(section) = sections
            .iter_mut()
            .find(|section| section.name.as_deref() == Some(patch.section))
        {
            let kept: Vec<String> = section
                .body
                .iter()
                .filter(|line| key_of(line).map_or(true, |key| !managed_keys.contains(&key)))
                .cloned()
                .collect();
            let mut body = managed_lines.clone();
            body.extend(kept);
            section.body = body;
        } else {
            sections.push(IniSection {
                name: Some(patch.section.to_string()),
                body: managed_lines.clone(),
            });
        }
    }

    let mut output = String::new();
    for section in &sections {
        let mut body = section.body.clone();
        while body.last().map_or(false, |line| line.trim().is_empty()) {
            body.pop();
        }
        if let Some(name) = &section.name {
            output.push_str(&format!("[{name}]\n"));
        }
        for line in &body {
            output.push_str(line);
            output.push('\n');
        }
        if section.name.is_some() {
            output.push('\n');
        }
    }
    output.trim_end_matches('\n').to_string() + "\n"
}
