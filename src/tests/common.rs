use crate::types::{Counters, MetroCommand, PatternStorage, ScaleState, ScriptStorage, Variables};
use std::sync::mpsc::{self, Receiver, Sender};

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

pub fn create_test_counters() -> Counters {
    Counters::default()
}

pub fn create_test_scale() -> ScaleState {
    ScaleState::default()
}

pub fn create_test_metro_tx() -> (Sender<MetroCommand>, Receiver<MetroCommand>) {
    mpsc::channel::<MetroCommand>()
}

#[macro_export]
macro_rules! test_setup {
    () => {{
        let variables = $crate::tests::common::create_test_variables();
        let mut patterns = $crate::tests::common::create_test_patterns();
        let scripts = $crate::tests::common::create_test_scripts();
        let mut counters = $crate::tests::common::create_test_counters();
        let scale = $crate::tests::common::create_test_scale();
        (variables, patterns, scripts, counters, scale)
    }};
    (mut) => {{
        let mut variables = $crate::tests::common::create_test_variables();
        let mut patterns = $crate::tests::common::create_test_patterns();
        let scripts = $crate::tests::common::create_test_scripts();
        let mut counters = $crate::tests::common::create_test_counters();
        let scale = $crate::tests::common::create_test_scale();
        (variables, patterns, scripts, counters, scale)
    }};
}
