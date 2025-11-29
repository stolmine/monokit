use crate::scene::{sanitize_name, scene_path, Scene, ScenePattern, SceneScript};
use super::common::{create_test_patterns, create_test_scripts};

#[test]
fn test_scene_serialization_roundtrip() {
    let scene = Scene {
        version: 1,
        scripts: vec![SceneScript {
            lines: vec![
                "PF 200".to_string(),
                "TR".to_string(),
                "".to_string(),
                "".to_string(),
                "".to_string(),
                "".to_string(),
                "".to_string(),
                "".to_string(),
            ],
            j: 0,
            k: 0,
        }],
        patterns: vec![ScenePattern {
            data: vec![100, 200, 300, 400],
            length: 4,
            index: 0,
        }],
        pattern_working: 0,
    };

    let json = serde_json::to_string(&scene).unwrap();
    let loaded: Scene = serde_json::from_str(&json).unwrap();

    assert_eq!(loaded.version, scene.version);
    assert_eq!(loaded.pattern_working, scene.pattern_working);
    assert_eq!(loaded.scripts.len(), 1);
    assert_eq!(loaded.scripts[0].lines[0], "PF 200");
    assert_eq!(loaded.scripts[0].lines[1], "TR");
    assert_eq!(loaded.patterns.len(), 1);
    assert_eq!(loaded.patterns[0].data[0], 100);
    assert_eq!(loaded.patterns[0].data[1], 200);
    assert_eq!(loaded.patterns[0].length, 4);
}

#[test]
fn test_scene_from_app_state() {
    let mut scripts = create_test_scripts();
    let mut patterns = create_test_patterns();

    scripts.scripts[0].lines[0] = "PF 100".to_string();
    scripts.scripts[0].lines[1] = "TR".to_string();
    scripts.scripts[0].j = 5;
    scripts.scripts[0].k = 10;

    patterns.patterns[0].data[0] = 100;
    patterns.patterns[0].data[1] = 200;
    patterns.patterns[0].data[2] = 300;
    patterns.patterns[0].length = 3;
    patterns.patterns[0].index = 1;
    patterns.working = 2;

    let scene = Scene::from_app_state(&scripts, &patterns);

    assert_eq!(scene.version, 1);
    assert_eq!(scene.scripts.len(), 10);
    assert_eq!(scene.scripts[0].lines[0], "PF 100");
    assert_eq!(scene.scripts[0].lines[1], "TR");
    assert_eq!(scene.scripts[0].j, 5);
    assert_eq!(scene.scripts[0].k, 10);
    assert_eq!(scene.patterns.len(), 6);
    assert_eq!(scene.patterns[0].data[0], 100);
    assert_eq!(scene.patterns[0].data[1], 200);
    assert_eq!(scene.patterns[0].data[2], 300);
    assert_eq!(scene.patterns[0].length, 3);
    assert_eq!(scene.patterns[0].index, 1);
    assert_eq!(scene.pattern_working, 2);
}

#[test]
fn test_scene_apply_to_app_state() {
    let scene = Scene {
        version: 1,
        scripts: vec![SceneScript {
            lines: vec![
                "A 10".to_string(),
                "B 20".to_string(),
                "".to_string(),
                "".to_string(),
                "".to_string(),
                "".to_string(),
                "".to_string(),
                "".to_string(),
            ],
            j: 42,
            k: 84,
        }],
        patterns: vec![ScenePattern {
            data: vec![111, 222, 333, 444],
            length: 4,
            index: 2,
        }],
        pattern_working: 1,
    };

    let mut scripts = create_test_scripts();
    let mut patterns = create_test_patterns();

    scene.apply_to_app_state(&mut scripts, &mut patterns);

    assert_eq!(scripts.scripts[0].lines[0], "A 10");
    assert_eq!(scripts.scripts[0].lines[1], "B 20");
    assert_eq!(scripts.scripts[0].j, 42);
    assert_eq!(scripts.scripts[0].k, 84);
    assert_eq!(patterns.patterns[0].data[0], 111);
    assert_eq!(patterns.patterns[0].data[1], 222);
    assert_eq!(patterns.patterns[0].data[2], 333);
    assert_eq!(patterns.patterns[0].data[3], 444);
    assert_eq!(patterns.patterns[0].length, 4);
    assert_eq!(patterns.patterns[0].index, 2);
    assert_eq!(patterns.working, 1);
}

#[test]
fn test_sanitize_name_spaces() {
    assert_eq!(sanitize_name("my scene"), "my-scene");
}

#[test]
fn test_sanitize_name_special_chars() {
    assert_eq!(sanitize_name("test@#$name"), "test---name");
}

#[test]
fn test_sanitize_name_alphanumeric() {
    assert_eq!(sanitize_name("test-name_123"), "test-name_123");
}

#[test]
fn test_scene_path_ends_with_json() {
    let path = scene_path("test");
    assert!(path.to_string_lossy().ends_with(".json"));
}

#[test]
fn test_scene_path_uses_sanitized_name() {
    let path = scene_path("my test scene");
    let path_str = path.to_string_lossy();
    assert!(path_str.ends_with("my-test-scene.json"));
}
