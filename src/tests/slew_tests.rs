use crate::commands::validate_script_command;
use crate::commands::slew::{handle_slew, handle_slew_all};
use crate::types::MetroCommand;
use crate::test_setup;
use super::common::{create_test_scale, create_test_variables, create_test_patterns, create_test_scripts, create_test_counters};
use std::sync::mpsc;

#[test]
fn test_slew_valid_commands() {
    assert!(validate_script_command("SLEW PF 100").is_ok());
    assert!(validate_script_command("SLEW MF 500").is_ok());
    assert!(validate_script_command("SLEW FC 1000").is_ok());
    assert!(validate_script_command("SLEW VOLUME 250").is_ok());
    assert!(validate_script_command("SLEW FB 0").is_ok());
    assert!(validate_script_command("SLEW FQ 10000").is_ok());
}

#[test]
fn test_slew_missing_args() {
    assert!(validate_script_command("SLEW").is_err());
    assert!(validate_script_command("SLEW PF").is_err());
}

#[test]
fn test_slew_with_expressions() {
    // SLEW now accepts expressions, so extra args are valid expression parts
    assert!(validate_script_command("SLEW PF ADD 100 200").is_ok());
    assert!(validate_script_command("SLEW MF MUL 100 2").is_ok());
}

#[test]
fn test_slew_all_valid() {
    assert!(validate_script_command("SLEW.ALL 100").is_ok());
    assert!(validate_script_command("SLEW.ALL 500").is_ok());
    assert!(validate_script_command("SLEW.ALL 0").is_ok());
    assert!(validate_script_command("SLEW.ALL 10000").is_ok());
}

#[test]
fn test_slew_all_missing_args() {
    assert!(validate_script_command("SLEW.ALL").is_err());
}

#[test]
fn test_slew_all_with_expressions() {
    // SLEW.ALL now accepts expressions, so extra args are valid expression parts
    assert!(validate_script_command("SLEW.ALL ADD 100 200").is_ok());
}

#[test]
fn test_slew_various_params() {
    assert!(validate_script_command("SLEW DF 200").is_ok());
    assert!(validate_script_command("SLEW DW 300").is_ok());
    assert!(validate_script_command("SLEW RV 400").is_ok());
    assert!(validate_script_command("SLEW RW 150").is_ok());
    assert!(validate_script_command("SLEW FE 75").is_ok());
    assert!(validate_script_command("SLEW FK 125").is_ok());
    assert!(validate_script_command("SLEW RM 225").is_ok());
    assert!(validate_script_command("SLEW DT 350").is_ok());
}

#[test]
fn test_slew_in_sequence() {
    assert!(validate_script_command("SLEW PF 100; SLEW MF 200").is_ok());
    assert!(validate_script_command("TR; SLEW FC 500; PF 440").is_ok());
}

#[test]
fn test_slew_valid_parameter_names() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let mut counters = create_test_counters();
    let scripts = create_test_scripts();
    let (metro_tx, metro_rx) = mpsc::channel::<MetroCommand>();
    let mut outputs: Vec<String> = Vec::new();

    let parts = vec!["SLEW", "PF", "100"];
    let result = handle_slew(
        &parts,
        &variables,
        &mut patterns,
        &mut counters,
        &scripts,
        0,
        &create_test_scale(),
        &metro_tx,
        1,
        |output: String| outputs.push(output),
    );
    assert!(result.is_ok());
    assert!(outputs.iter().any(|s| s.contains("SET PF SLEW TIME")));

    let msg = metro_rx.try_recv();
    assert!(msg.is_ok());
    if let Ok(MetroCommand::SetParamSlew(param, time)) = msg {
        assert_eq!(param, "pf");
        assert_eq!(time, 0.1);
    }
}

#[test]
fn test_slew_invalid_parameter_name() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let mut counters = create_test_counters();
    let scripts = create_test_scripts();
    let (metro_tx, _metro_rx) = mpsc::channel::<MetroCommand>();
    let mut outputs: Vec<String> = Vec::new();

    let parts = vec!["SLEW", "INVALID", "100"];
    let result = handle_slew(
        &parts,
        &variables,
        &mut patterns,
        &mut counters,
        &scripts,
        0,
        &create_test_scale(),
        &metro_tx,
        1,
        |output: String| outputs.push(output),
    );
    assert!(result.is_ok());
    assert!(outputs.iter().any(|s| s.contains("ERROR: INVALID PARAMETER")));
}

#[test]
fn test_slew_time_range_validation() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let mut counters = create_test_counters();
    let scripts = create_test_scripts();
    let (metro_tx, metro_rx) = mpsc::channel::<MetroCommand>();
    let mut outputs: Vec<String> = Vec::new();

    let parts_valid_min = vec!["SLEW", "PF", "0"];
    let result = handle_slew(
        &parts_valid_min,
        &variables,
        &mut patterns,
        &mut counters,
        &scripts,
        0,
        &create_test_scale(),
        &metro_tx,
        1,
        |output: String| outputs.push(output),
    );
    assert!(result.is_ok());
    let msg = metro_rx.try_recv();
    assert!(msg.is_ok());

    outputs.clear();
    let parts_valid_max = vec!["SLEW", "MF", "10000"];
    let result = handle_slew(
        &parts_valid_max,
        &variables,
        &mut patterns,
        &mut counters,
        &scripts,
        0,
        &create_test_scale(),
        &metro_tx,
        1,
        |output: String| outputs.push(output),
    );
    assert!(result.is_ok());
    let msg = metro_rx.try_recv();
    assert!(msg.is_ok());

    outputs.clear();
    let parts_too_high = vec!["SLEW", "FC", "10001"];
    let result = handle_slew(
        &parts_too_high,
        &variables,
        &mut patterns,
        &mut counters,
        &scripts,
        0,
        &create_test_scale(),
        &metro_tx,
        1,
        |output: String| outputs.push(output),
    );
    assert!(result.is_ok());
    assert!(outputs.iter().any(|s| s.contains("ERROR: SLEW TIME 0-10000 MS")));

    outputs.clear();
    let parts_negative = vec!["SLEW", "FB", "-1"];
    let result = handle_slew(
        &parts_negative,
        &variables,
        &mut patterns,
        &mut counters,
        &scripts,
        0,
        &create_test_scale(),
        &metro_tx,
        1,
        |output: String| outputs.push(output),
    );
    assert!(result.is_ok());
    assert!(outputs.iter().any(|s| s.contains("ERROR: SLEW TIME 0-10000 MS")));
}

#[test]
fn test_slew_all_time_range_validation() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let mut counters = create_test_counters();
    let scripts = create_test_scripts();
    let (metro_tx, metro_rx) = mpsc::channel::<MetroCommand>();
    let mut outputs: Vec<String> = Vec::new();

    let parts_valid = vec!["SLEW.ALL", "500"];
    let result = handle_slew_all(
        &parts_valid,
        &variables,
        &mut patterns,
        &mut counters,
        &scripts,
        0,
        &create_test_scale(),
        &metro_tx,
        1,
        |output: String| outputs.push(output),
    );
    assert!(result.is_ok());
    assert!(outputs.iter().any(|s| s.contains("SET SLEW TIME")));
    let msg = metro_rx.try_recv();
    assert!(msg.is_ok());
    if let Ok(MetroCommand::SetSlewTime(time)) = msg {
        assert_eq!(time, 0.5);
    }

    outputs.clear();
    let parts_invalid = vec!["SLEW.ALL", "15000"];
    let result = handle_slew_all(
        &parts_invalid,
        &variables,
        &mut patterns,
        &mut counters,
        &scripts,
        0,
        &create_test_scale(),
        &metro_tx,
        1,
        |output: String| outputs.push(output),
    );
    assert!(result.is_ok());
    assert!(outputs.iter().any(|s| s.contains("ERROR: SLEW TIME 0-10000 MS")));
}

#[test]
fn test_slew_various_valid_params() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let mut counters = create_test_counters();
    let scripts = create_test_scripts();
    let (metro_tx, metro_rx) = mpsc::channel::<MetroCommand>();

    let valid_params = vec!["pf", "mf", "fc", "fm", "dc", "fb", "fq", "volume", "rv", "rw"];

    for param in valid_params {
        let parts = vec!["SLEW", param, "100"];
        let result = handle_slew(
            &parts,
            &variables,
            &mut patterns,
            &mut counters,
            &scripts,
        0,
        &create_test_scale(),
            &metro_tx,
            0,
            |_| {},
        );
        assert!(result.is_ok());
        let msg = metro_rx.try_recv();
        assert!(msg.is_ok(), "Failed for param: {}", param);
    }
}

#[test]
fn test_slew_canonical_parameter_names() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let mut counters = create_test_counters();
    let scripts = create_test_scripts();
    let (metro_tx, metro_rx) = mpsc::channel::<MetroCommand>();
    let mut outputs: Vec<String> = Vec::new();

    let parts = vec!["SLEW", "REV.WET", "100"];
    let result = handle_slew(
        &parts,
        &variables,
        &mut patterns,
        &mut counters,
        &scripts,
        0,
        &create_test_scale(),
        &metro_tx,
        1,
        |output: String| outputs.push(output),
    );
    assert!(result.is_ok());
    assert!(outputs.iter().any(|s| s.contains("SET REV.WET SLEW TIME")));

    let msg = metro_rx.try_recv();
    assert!(msg.is_ok());
    if let Ok(MetroCommand::SetParamSlew(param, time)) = msg {
        assert_eq!(param, "rw");
        assert_eq!(time, 0.1);
    }
}
