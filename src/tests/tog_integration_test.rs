use crate::commands::process_command;
use crate::midi::{MidiConnection, MidiTimingStats};
use crate::types::{Counters, MetroCommand, PatternStorage, ScaleState, ScriptStorage, SyncMode, Variables};
use std::sync::mpsc;

#[test]
fn test_tog_command_integration() {
    let (metro_tx, _metro_rx) = mpsc::channel::<MetroCommand>();
    let mut metro_interval = 500u64;
    let mut br_len = 2usize;
    let mut sync_mode = SyncMode::Internal;
    let mut midi_connection: Option<MidiConnection> = None;
    let midi_timing_stats = MidiTimingStats::new();
    let mut variables = Variables::default();
    let mut patterns = PatternStorage::default();
    let mut counters = Counters::default();
    let mut scripts = ScriptStorage::default();
    let mut scale = ScaleState::default();
    let mut theme = crate::theme::Theme::default();
    let mut debug_level = 0u8;
    let mut activity_hold_ms = 200.0f32;
    let script_index = 0;

    let mut outputs = Vec::new();
    let output = |s: String| {
        outputs.push(s);
    };

    let _ = process_command(
        &metro_tx,
        &mut metro_interval,
        &mut br_len,
        &mut sync_mode,
        &mut midi_connection,
        &midi_timing_stats,
        &mut variables,
        &mut patterns,
        &mut counters,
        &mut scripts,
        script_index,
        &mut scale,
        &mut theme,
        &mut debug_level,
        &mut activity_hold_ms,
        "TOG 100 200",
        output,
    );
    assert_eq!(outputs.last().unwrap(), "100");
    outputs.clear();

    let output = |s: String| {
        outputs.push(s);
    };
    let _ = process_command(
        &metro_tx,
        &mut metro_interval,
        &mut br_len,
        &mut sync_mode,
        &mut midi_connection,
        &midi_timing_stats,
        &mut variables,
        &mut patterns,
        &mut counters,
        &mut scripts,
        script_index,
        &mut scale,
        &mut theme,
        &mut debug_level,
        &mut activity_hold_ms,
        "TOG 100 200",
        output,
    );
    assert_eq!(outputs.last().unwrap(), "200");
    outputs.clear();

    let output = |s: String| {
        outputs.push(s);
    };
    let _ = process_command(
        &metro_tx,
        &mut metro_interval,
        &mut br_len,
        &mut sync_mode,
        &mut midi_connection,
        &midi_timing_stats,
        &mut variables,
        &mut patterns,
        &mut counters,
        &mut scripts,
        script_index,
        &mut scale,
        &mut theme,
        &mut debug_level,
        &mut activity_hold_ms,
        "TOG 100 200",
        output,
    );
    assert_eq!(outputs.last().unwrap(), "100");
}

#[test]
fn test_tog_with_variable_assignment() {
    let (metro_tx, _metro_rx) = mpsc::channel::<MetroCommand>();
    let mut metro_interval = 500u64;
    let mut br_len = 2usize;
    let mut sync_mode = SyncMode::Internal;
    let mut midi_connection: Option<MidiConnection> = None;
    let midi_timing_stats = MidiTimingStats::new();
    let mut variables = Variables::default();
    let mut patterns = PatternStorage::default();
    let mut counters = Counters::default();
    let mut scripts = ScriptStorage::default();
    let mut scale = ScaleState::default();
    let mut theme = crate::theme::Theme::default();
    let mut debug_level = 0u8;
    let mut activity_hold_ms = 200.0f32;
    let script_index = 0;

    let mut outputs = Vec::new();
    let output = |s: String| {
        outputs.push(s);
    };

    let _ = process_command(
        &metro_tx,
        &mut metro_interval,
        &mut br_len,
        &mut sync_mode,
        &mut midi_connection,
        &midi_timing_stats,
        &mut variables,
        &mut patterns,
        &mut counters,
        &mut scripts,
        script_index,
        &mut scale,
        &mut theme,
        &mut debug_level,
        &mut activity_hold_ms,
        "A TOG 0 1",
        output,
    );
    assert_eq!(variables.a, 0);

    let output = |s: String| {
        outputs.push(s);
    };
    let _ = process_command(
        &metro_tx,
        &mut metro_interval,
        &mut br_len,
        &mut sync_mode,
        &mut midi_connection,
        &midi_timing_stats,
        &mut variables,
        &mut patterns,
        &mut counters,
        &mut scripts,
        script_index,
        &mut scale,
        &mut theme,
        &mut debug_level,
        &mut activity_hold_ms,
        "A TOG 0 1",
        output,
    );
    assert_eq!(variables.a, 1);

    let output = |s: String| {
        outputs.push(s);
    };
    let _ = process_command(
        &metro_tx,
        &mut metro_interval,
        &mut br_len,
        &mut sync_mode,
        &mut midi_connection,
        &midi_timing_stats,
        &mut variables,
        &mut patterns,
        &mut counters,
        &mut scripts,
        script_index,
        &mut scale,
        &mut theme,
        &mut debug_level,
        &mut activity_hold_ms,
        "A TOG 0 1",
        output,
    );
    assert_eq!(variables.a, 0);
}
