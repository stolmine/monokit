use crate::types::{NotesStorage, PatternStorage, ScriptMutes, ScriptStorage, Variables, TIER_ERRORS, TIER_ESSENTIAL, TIER_QUERIES, TIER_CONFIRMS};

pub fn handle_save<F>(
    parts: &[&str],
    scripts: &ScriptStorage,
    patterns: &PatternStorage,
    notes: &NotesStorage,
    current_scene_name: &mut Option<String>,
    scramble_enabled: bool,
    scramble_mode: u8,
    scramble_speed: u8,
    scramble_curve: u8,
    header_scramble: &mut Option<crate::scramble::ScrambleAnimation>,
    debug_level: u8,
    out_ess: bool,
    script_mutes: &ScriptMutes,
    mut output: F,
) where
    F: FnMut(String),
{
    if parts.len() < 2 {
        output("ERROR: SAVE REQUIRES A SCENE NAME".to_string());
        return;
    }
    let name = parts[1..].join(" ");
    let scene = crate::scene::Scene::from_app_state(scripts, patterns, notes, script_mutes);
    match crate::scene::save_scene(&name, &scene) {
        Ok(()) => {
            *current_scene_name = Some(name.clone());
            *header_scramble = if scramble_enabled {
                let mode = crate::scramble::ScrambleMode::from_u8(scramble_mode);
                let curve = crate::scramble::ScrambleCurve::from_u8(scramble_curve);
                Some(crate::scramble::ScrambleAnimation::new_with_options(&name, mode, scramble_speed, curve))
            } else {
                None
            };
            if debug_level >= TIER_ESSENTIAL || out_ess {
                output(format!("SAVED SCENE: {}", name));
            }
        }
        Err(e) => output(format!("ERROR: {:?}", e)),
    }
}

pub fn handle_load<F>(
    parts: &[&str],
    variables: &mut Variables,
    scripts: &mut ScriptStorage,
    patterns: &mut PatternStorage,
    notes: &mut NotesStorage,
    current_scene_name: &mut Option<String>,
    scramble_enabled: bool,
    scramble_mode: u8,
    scramble_speed: u8,
    scramble_curve: u8,
    header_scramble: &mut Option<crate::scramble::ScrambleAnimation>,
    debug_level: u8,
    out_ess: bool,
    script_mutes: &mut ScriptMutes,
    mut output: F,
) -> bool
where
    F: FnMut(String),
{
    if parts.len() < 2 {
        output("ERROR: LOAD REQUIRES A SCENE NAME".to_string());
        return false;
    }
    let name = parts[1..].join(" ");
    match crate::scene::load_scene(&name) {
        Ok(scene) => {
            scene.apply_to_app_state(scripts, patterns, notes, script_mutes);
            *variables = crate::types::Variables::default();
            *current_scene_name = Some(name.clone());
            let _ = crate::config::save_last_scene(&name);
            *header_scramble = if scramble_enabled {
                let mode = crate::scramble::ScrambleMode::from_u8(scramble_mode);
                let curve = crate::scramble::ScrambleCurve::from_u8(scramble_curve);
                Some(crate::scramble::ScrambleAnimation::new_with_options(&name, mode, scramble_speed, curve))
            } else {
                None
            };
            if debug_level >= TIER_ESSENTIAL || out_ess {
                output(format!("LOADED SCENE: {}", name));
            }
            true
        }
        Err(crate::scene::SceneError::NotFound(_)) => {
            output(format!("ERROR: SCENE '{}' NOT FOUND", name));
            false
        }
        Err(e) => {
            output(format!("ERROR: {:?}", e));
            false
        }
    }
}

pub fn handle_scenes<F>(
    debug_level: u8,
    out_qry: bool,
    mut output: F,
) where
    F: FnMut(String),
{
    if debug_level >= TIER_QUERIES || out_qry {
        match crate::scene::list_scenes() {
            Ok(scenes) => {
                if scenes.is_empty() {
                    output("NO SCENES SAVED".to_string());
                } else {
                    output("SCENES:".to_string());
                    for (name, size) in scenes {
                        let size_kb = size as f64 / 1024.0;
                        output(format!("  {} ({:.1} KB)", name, size_kb));
                    }
                }
            }
            Err(e) => output(format!("ERROR: {:?}", e)),
        }
    }
}

pub fn handle_delete<F>(
    parts: &[&str],
    debug_level: u8,
    out_ess: bool,
    mut output: F,
) where
    F: FnMut(String),
{
    if parts.len() < 2 {
        output("ERROR: DELETE REQUIRES A SCENE NAME".to_string());
        return;
    }
    let name = parts[1..].join(" ");
    match crate::scene::delete_scene(&name) {
        Ok(()) => {
            if debug_level >= TIER_ESSENTIAL || out_ess {
                output(format!("DELETED SCENE: {}", name));
            }
        }
        Err(crate::scene::SceneError::NotFound(_)) => {
            output(format!("ERROR: SCENE '{}' NOT FOUND", name));
        }
        Err(e) => output(format!("ERROR: {:?}", e)),
    }
}
