use crate::preset::{factory, Preset, PresetType};
use crate::types::ScriptStorage;

#[test]
fn test_preset_serialization_roundtrip() {
    let preset = Preset {
        version: 1,
        name: "test-preset".to_string(),
        category: "test".to_string(),
        lines: vec![
            "PF 200".to_string(),
            "AD 100".to_string(),
            "TR".to_string(),
        ],
        j: 5,
        k: 10,
        description: "Test preset".to_string(),
    };

    let json = serde_json::to_string(&preset).unwrap();
    let loaded: Preset = serde_json::from_str(&json).unwrap();

    assert_eq!(loaded.version, preset.version);
    assert_eq!(loaded.name, preset.name);
    assert_eq!(loaded.category, preset.category);
    assert_eq!(loaded.lines.len(), 3);
    assert_eq!(loaded.lines[0], "PF 200");
    assert_eq!(loaded.lines[1], "AD 100");
    assert_eq!(loaded.lines[2], "TR");
    assert_eq!(loaded.j, 5);
    assert_eq!(loaded.k, 10);
    assert_eq!(loaded.description, "Test preset");
}

#[test]
fn test_factory_preset_808_kick() {
    let preset = factory::get_factory_preset("808-kick");
    assert!(preset.is_some());

    let preset = preset.unwrap();
    assert_eq!(preset.name, "808-kick");
    assert_eq!(preset.category, "drums");
    assert!(preset.lines.contains(&"PF 55".to_string()));
    assert!(preset.lines.contains(&"PA 4".to_string()));
}

#[test]
fn test_factory_preset_saw_bass() {
    let preset = factory::get_factory_preset("saw-bass");
    assert!(preset.is_some());

    let preset = preset.unwrap();
    assert_eq!(preset.name, "saw-bass");
    assert_eq!(preset.category, "bass");
    assert!(preset.lines.contains(&"PF 65".to_string()));
    assert!(preset.lines.contains(&"PW 2".to_string()));
}

#[test]
fn test_factory_preset_not_found() {
    let preset = factory::get_factory_preset("nonexistent");
    assert!(preset.is_none());
}

#[test]
fn test_list_factory_presets() {
    let presets = factory::list_factory_presets();
    assert!(!presets.is_empty());
    assert!(presets.contains(&"808-kick".to_string()));
    assert!(presets.contains(&"saw-bass".to_string()));
    assert!(presets.contains(&"fm-lead".to_string()));
}

#[test]
fn test_factory_presets_have_valid_lines() {
    let presets = factory::list_factory_presets();
    for name in presets {
        let preset = factory::get_factory_preset(&name).unwrap();
        assert!(!preset.lines.is_empty(), "Preset {} has no lines", name);
        assert!(preset.lines.len() <= 8, "Preset {} has more than 8 lines", name);
    }
}

#[test]
fn test_load_preset_into_script() {
    let mut scripts = ScriptStorage::default();
    let preset = Preset {
        version: 1,
        name: "test".to_string(),
        category: "test".to_string(),
        lines: vec![
            "PF 440".to_string(),
            "AD 200".to_string(),
            "TR".to_string(),
        ],
        j: 42,
        k: 84,
        description: "Test".to_string(),
    };

    let script = &mut scripts.scripts[0];
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

    assert_eq!(script.lines[0], "PF 440");
    assert_eq!(script.lines[1], "AD 200");
    assert_eq!(script.lines[2], "TR");
    assert_eq!(script.lines[3], "");
    assert_eq!(script.j, 42);
    assert_eq!(script.k, 84);
}

#[test]
fn test_get_preset_returns_factory_first() {
    let result = crate::preset::get_preset("808-kick");
    assert!(result.is_ok());

    let (preset, preset_type) = result.unwrap();
    assert_eq!(preset.name, "808-kick");
    assert_eq!(preset_type, PresetType::Factory);
}

#[test]
fn test_sanitize_name() {
    use crate::preset::sanitize_name;
    assert_eq!(sanitize_name("my preset"), "my-preset");
    assert_eq!(sanitize_name("test@#$name"), "test---name");
    assert_eq!(sanitize_name("valid-name_123"), "valid-name_123");
}

#[test]
fn test_all_factory_presets_load() {
    let presets = factory::list_factory_presets();
    for name in presets {
        let result = crate::preset::get_preset(&name);
        assert!(result.is_ok(), "Failed to load preset: {}", name);

        let (preset, preset_type) = result.unwrap();
        assert_eq!(preset_type, PresetType::Factory);
        assert_eq!(preset.name, name);
    }
}
