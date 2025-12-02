use crate::commands::process_command;
use crate::midi::{MidiConnection, MidiTimingStats};
use crate::theme::Theme;
use crate::types::{Counters, MetroCommand, MetroState, SyncMode};
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use super::common::{create_test_patterns, create_test_scale, create_test_scripts, create_test_variables, create_test_counters};

fn create_test_metro() -> (mpsc::Sender<MetroCommand>, mpsc::Receiver<MetroCommand>, Arc<Mutex<MetroState>>) {
    let (tx, rx) = mpsc::channel();
    let state = Arc::new(Mutex::new(MetroState {
        interval_ms: 500,
        active: false,
        script_index: 0,
    }));
    (tx, rx, state)
}

#[test]
fn test_debug_level_0_blocks_param_output() {
    let mut variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let mut scripts = create_test_scripts();
    let mut counters = create_test_counters();
    let mut scale = create_test_scale();
    let (metro_tx, _metro_rx, _metro_state) = create_test_metro();
    let mut metro_interval = 500u64;
    let mut br_len = 2usize;
    let mut sync_mode = SyncMode::Internal;
    let mut midi_connection: Option<MidiConnection> = None;
    let midi_timing_stats = MidiTimingStats::new();
    let mut theme = Theme::dark();
    let mut debug_level = 0u8;
    let mut activity_hold_ms = 200.0f32;
    let mut show_cpu = false;
    let mut output_messages = Vec::new();

    let result = process_command(
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
        0,
        &mut scale,
        &mut theme,
        &mut debug_level,
        &mut activity_hold_ms,
        &mut show_cpu,
        "PF 440",
        |msg| {
            output_messages.push(msg);
        },
    );

    assert!(result.is_ok());
    assert_eq!(output_messages.len(), 0);
}

#[test]
fn test_debug_level_0_blocks_trigger_output() {
    let mut variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let mut scripts = create_test_scripts();
    let mut counters = create_test_counters();
    let mut scale = create_test_scale();
    let (metro_tx, _metro_rx, _metro_state) = create_test_metro();
    let mut metro_interval = 500u64;
    let mut br_len = 2usize;
    let mut sync_mode = SyncMode::Internal;
    let mut midi_connection: Option<MidiConnection> = None;
    let midi_timing_stats = MidiTimingStats::new();
    let mut theme = Theme::dark();
    let mut debug_level = 0u8;
    let mut activity_hold_ms = 200.0f32;
    let mut show_cpu = false;
    let mut output_messages = Vec::new();

    let result = process_command(
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
        0,
        &mut scale,
        &mut theme,
        &mut debug_level,
        &mut activity_hold_ms,
        &mut show_cpu,
        "TR",
        |msg| {
            output_messages.push(msg);
        },
    );

    assert!(result.is_ok());
    assert_eq!(output_messages.len(), 0);
}

#[test]
fn test_debug_level_0_blocks_print() {
    let mut variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let mut scripts = create_test_scripts();
    let mut counters = create_test_counters();
    let mut scale = create_test_scale();
    let (metro_tx, _metro_rx, _metro_state) = create_test_metro();
    let mut metro_interval = 500u64;
    let mut br_len = 2usize;
    let mut sync_mode = SyncMode::Internal;
    let mut midi_connection: Option<MidiConnection> = None;
    let midi_timing_stats = MidiTimingStats::new();
    let mut theme = Theme::dark();
    let mut debug_level = 0u8;
    let mut activity_hold_ms = 200.0f32;
    let mut show_cpu = false;
    let mut output_messages = Vec::new();

    let result = process_command(
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
        0,
        &mut scale,
        &mut theme,
        &mut debug_level,
        &mut activity_hold_ms,
        &mut show_cpu,
        "PRINT 42",
        |msg| {
            output_messages.push(msg);
        },
    );

    assert!(result.is_ok());
    assert_eq!(output_messages.len(), 0);
}

#[test]
fn test_debug_level_1_allows_print() {
    let mut variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let mut scripts = create_test_scripts();
    let mut counters = create_test_counters();
    let mut scale = create_test_scale();
    let (metro_tx, _metro_rx, _metro_state) = create_test_metro();
    let mut metro_interval = 500u64;
    let mut br_len = 2usize;
    let mut sync_mode = SyncMode::Internal;
    let mut midi_connection: Option<MidiConnection> = None;
    let midi_timing_stats = MidiTimingStats::new();
    let mut theme = Theme::dark();
    let mut debug_level = 1u8;
    let mut activity_hold_ms = 200.0f32;
    let mut show_cpu = false;
    let mut output_messages = Vec::new();

    let result = process_command(
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
        0,
        &mut scale,
        &mut theme,
        &mut debug_level,
        &mut activity_hold_ms,
        &mut show_cpu,
        "PRINT 42",
        |msg| {
            output_messages.push(msg);
        },
    );

    assert!(result.is_ok());
    assert_eq!(output_messages.len(), 1);
    assert_eq!(output_messages[0], "42");
}

#[test]
fn test_debug_level_1_blocks_params() {
    let mut variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let mut scripts = create_test_scripts();
    let mut counters = create_test_counters();
    let mut scale = create_test_scale();
    let (metro_tx, _metro_rx, _metro_state) = create_test_metro();
    let mut metro_interval = 500u64;
    let mut br_len = 2usize;
    let mut sync_mode = SyncMode::Internal;
    let mut midi_connection: Option<MidiConnection> = None;
    let midi_timing_stats = MidiTimingStats::new();
    let mut theme = Theme::dark();
    let mut debug_level = 1u8;
    let mut activity_hold_ms = 200.0f32;
    let mut show_cpu = false;
    let mut output_messages = Vec::new();

    let result = process_command(
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
        0,
        &mut scale,
        &mut theme,
        &mut debug_level,
        &mut activity_hold_ms,
        &mut show_cpu,
        "PF 440",
        |msg| {
            output_messages.push(msg);
        },
    );

    assert!(result.is_ok());
    assert_eq!(output_messages.len(), 0);
}

#[test]
fn test_debug_level_1_allows_metro_status() {
    let mut variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let mut scripts = create_test_scripts();
    let mut counters = create_test_counters();
    let mut scale = create_test_scale();
    let (metro_tx, _metro_rx, _metro_state) = create_test_metro();
    let mut metro_interval = 500u64;
    let mut br_len = 2usize;
    let mut sync_mode = SyncMode::Internal;
    let mut midi_connection: Option<MidiConnection> = None;
    let midi_timing_stats = MidiTimingStats::new();
    let mut theme = Theme::dark();
    let mut debug_level = 1u8;
    let mut activity_hold_ms = 200.0f32;
    let mut show_cpu = false;
    let mut output_messages = Vec::new();

    let result = process_command(
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
        0,
        &mut scale,
        &mut theme,
        &mut debug_level,
        &mut activity_hold_ms,
        &mut show_cpu,
        "M",
        |msg| {
            output_messages.push(msg);
        },
    );

    assert!(result.is_ok());
    assert_eq!(output_messages.len(), 1);
    assert_eq!(output_messages[0], "METRO INTERVAL: 500MS");
}

#[test]
fn test_debug_level_2_allows_all() {
    let mut variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let mut scripts = create_test_scripts();
    let mut counters = create_test_counters();
    let mut scale = create_test_scale();
    let (metro_tx, _metro_rx, _metro_state) = create_test_metro();
    let mut metro_interval = 500u64;
    let mut br_len = 2usize;
    let mut sync_mode = SyncMode::Internal;
    let mut midi_connection: Option<MidiConnection> = None;
    let midi_timing_stats = MidiTimingStats::new();
    let mut theme = Theme::dark();
    let mut debug_level = 2u8;
    let mut activity_hold_ms = 200.0f32;
    let mut show_cpu = false;
    let mut output_messages = Vec::new();

    let result = process_command(
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
        0,
        &mut scale,
        &mut theme,
        &mut debug_level,
        &mut activity_hold_ms,
        &mut show_cpu,
        "PF 440",
        |msg| {
            output_messages.push(msg);
        },
    );

    assert!(result.is_ok());
    assert_eq!(output_messages.len(), 1);
    assert_eq!(output_messages[0], "SET PRIMARY FREQUENCY TO 440 HZ");

    output_messages.clear();

    let result = process_command(
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
        0,
        &mut scale,
        &mut theme,
        &mut debug_level,
        &mut activity_hold_ms,
        &mut show_cpu,
        "PRINT 42",
        |msg| {
            output_messages.push(msg);
        },
    );

    assert!(result.is_ok());
    assert_eq!(output_messages.len(), 1);
    assert_eq!(output_messages[0], "42");

    output_messages.clear();

    let result = process_command(
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
        0,
        &mut scale,
        &mut theme,
        &mut debug_level,
        &mut activity_hold_ms,
        &mut show_cpu,
        "M",
        |msg| {
            output_messages.push(msg);
        },
    );

    assert!(result.is_ok());
    assert_eq!(output_messages.len(), 1);
    assert_eq!(output_messages[0], "METRO INTERVAL: 500MS");
}
