use crate::types::{PatternStorage, ScriptStorage, Variables};

pub fn create_test_variables() -> Variables {
    Variables {
        a: 0,
        b: 0,
        c: 0,
        d: 0,
        i: 0,
        x: 0,
        y: 0,
        z: 0,
        t: 0,
    }
}

pub fn create_test_patterns() -> PatternStorage {
    PatternStorage::default()
}

pub fn create_test_scripts() -> ScriptStorage {
    ScriptStorage::default()
}
