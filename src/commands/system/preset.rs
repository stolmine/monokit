use crate::preset::{self, Preset, PresetType};
use crate::types::ScriptStorage;

pub fn handle_pset<F>(
    parts: &[&str],
    scripts: &mut ScriptStorage,
    mut output: F,
) where
    F: FnMut(String),
{
    if parts.len() < 3 {
        output("ERROR: PSET NEEDS SCRIPT# AND NAME".to_string());
        return;
    }

    let script_num = match parts[1].parse::<usize>() {
        Ok(n) if n >= 1 && n <= 8 => n - 1,
        _ => {
            output("ERROR: SCRIPT NUMBER MUST BE 1-8".to_string());
            return;
        }
    };

    let name = parts[2..].join(" ").to_lowercase();

    match preset::get_preset(&name) {
        Ok((preset, preset_type)) => {
            let script = &mut scripts.scripts[script_num];

            for (i, line) in preset.lines.iter().enumerate() {
                if i < 8 {
                    script.lines[i] = line.clone();
                }
            }

            for i in preset.lines.len()..8 {
                script.lines[i] = String::new();
            }

            script.j = preset.j;
            script.k = preset.k;

            let type_marker = match preset_type {
                PresetType::Factory => "[F]",
                PresetType::User => "[U]",
            };
            output(format!("LOADED PRESET {} {} INTO SCRIPT {}", type_marker, name, script_num + 1));
        }
        Err(preset::PresetError::NotFound(_)) => {
            output(format!("ERROR: PRESET '{}' NOT FOUND", name));
        }
        Err(e) => {
            output(format!("ERROR: {:?}", e));
        }
    }
}

pub fn handle_pset_save<F>(
    parts: &[&str],
    scripts: &ScriptStorage,
    mut output: F,
) where
    F: FnMut(String),
{
    if parts.len() < 3 {
        output("ERROR: PSET.SAVE NEEDS SCRIPT# AND NAME".to_string());
        return;
    }

    let script_num = match parts[1].parse::<usize>() {
        Ok(n) if n >= 1 && n <= 8 => n - 1,
        _ => {
            output("ERROR: SCRIPT NUMBER MUST BE 1-8".to_string());
            return;
        }
    };

    let name = parts[2..].join(" ").to_lowercase();

    let script = &scripts.scripts[script_num];
    let lines: Vec<String> = script.lines.iter()
        .take_while(|line| !line.is_empty())
        .cloned()
        .collect();

    let preset = Preset {
        version: 1,
        name: name.clone(),
        category: "user".to_string(),
        lines,
        j: script.j,
        k: script.k,
        description: format!("User preset from script {}", script_num + 1),
    };

    match preset::save_preset(&name, &preset) {
        Ok(()) => output(format!("SAVED PRESET: {}", name)),
        Err(e) => output(format!("ERROR: {:?}", e)),
    }
}

pub fn handle_pset_del<F>(
    parts: &[&str],
    mut output: F,
) where
    F: FnMut(String),
{
    if parts.len() < 2 {
        output("ERROR: PSET.DEL REQUIRES A PRESET NAME".to_string());
        return;
    }

    let name = parts[1..].join(" ").to_lowercase();

    if preset::factory::get_factory_preset(&name).is_some() {
        output(format!("ERROR: CANNOT DELETE FACTORY PRESET '{}'", name));
        return;
    }

    match preset::delete_preset(&name) {
        Ok(()) => output(format!("DELETED PRESET: {}", name)),
        Err(preset::PresetError::NotFound(_)) => {
            output(format!("ERROR: PRESET '{}' NOT FOUND", name));
        }
        Err(e) => output(format!("ERROR: {:?}", e)),
    }
}

pub fn handle_psets<F>(
    mut output: F,
) where
    F: FnMut(String),
{
    let factory_presets = preset::factory::list_factory_presets();
    let user_presets = preset::list_user_presets().unwrap_or_default();

    if factory_presets.is_empty() && user_presets.is_empty() {
        output("NO PRESETS AVAILABLE".to_string());
        return;
    }

    output("PRESETS:".to_string());

    if !factory_presets.is_empty() {
        output("".to_string());
        output("FACTORY:".to_string());
        for name in factory_presets {
            if let Some(preset) = preset::factory::get_factory_preset(&name) {
                output(format!("  [F] {} - {}", name, preset.description));
            }
        }
    }

    if !user_presets.is_empty() {
        output("".to_string());
        output("USER:".to_string());
        for (name, _size) in user_presets {
            output(format!("  [U] {}", name));
        }
    }
}
