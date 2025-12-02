use crate::types::{NotesStorage, PatternStorage, ScriptStorage, Variables};

pub fn handle_save<F>(
    parts: &[&str],
    scripts: &ScriptStorage,
    patterns: &PatternStorage,
    notes: &NotesStorage,
    current_scene_name: &mut Option<String>,
    mut output: F,
) where
    F: FnMut(String),
{
    if parts.len() < 2 {
        output("ERROR: SAVE REQUIRES A SCENE NAME".to_string());
        return;
    }
    let name = parts[1..].join(" ");
    let scene = crate::scene::Scene::from_app_state(scripts, patterns, notes);
    match crate::scene::save_scene(&name, &scene) {
        Ok(()) => {
            *current_scene_name = Some(name.clone());
            output(format!("SAVED SCENE: {}", name));
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
            scene.apply_to_app_state(scripts, patterns, notes);
            *variables = crate::types::Variables::default();
            *current_scene_name = Some(name.clone());
            output(format!("LOADED SCENE: {}", name));
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
    mut output: F,
) where
    F: FnMut(String),
{
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

pub fn handle_delete<F>(
    parts: &[&str],
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
        Ok(()) => output(format!("DELETED SCENE: {}", name)),
        Err(crate::scene::SceneError::NotFound(_)) => {
            output(format!("ERROR: SCENE '{}' NOT FOUND", name));
        }
        Err(e) => output(format!("ERROR: {:?}", e)),
    }
}
