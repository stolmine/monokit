use crate::commands::validate_script_command;
use crate::commands::delay::{handle_del, handle_del_clr, handle_del_x, handle_del_r};
use crate::types::MetroCommand;
use crate::test_setup;
use super::common::{create_test_scale, create_test_variables, create_test_patterns, create_test_scripts, create_test_counters};
use std::sync::mpsc;

#[test]
fn test_del_parses_delay_time() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let mut counters = create_test_counters();
    let scripts = create_test_scripts();
    let (metro_tx, metro_rx) = mpsc::channel::<MetroCommand>();
    let mut outputs: Vec<String> = Vec::new();

    let input = "DEL 500: TR";
    let parts = vec!["DEL", "500:"];
    let result = handle_del(
        &parts,
        input,
        &variables,
        &mut patterns,
        &mut counters,
        &scripts,
        0,
        &metro_tx,
        &create_test_scale(),
        1,
        |output: String| outputs.push(output),
    );
    assert!(result.is_ok());
    assert!(outputs.iter().any(|s| s.contains("SCHEDULED COMMAND IN 500MS")));

    let msg = metro_rx.try_recv();
    assert!(msg.is_ok());
    if let Ok(MetroCommand::ScheduleDelayed(cmd, delay_ms, _script_index)) = msg {
        assert_eq!(cmd, "TR");
        assert_eq!(delay_ms, 500);
    }
}

#[test]
fn test_del_requires_colon() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let mut counters = create_test_counters();
    let scripts = create_test_scripts();
    let (metro_tx, _metro_rx) = mpsc::channel::<MetroCommand>();
    let mut outputs: Vec<String> = Vec::new();

    let input = "DEL 500 TR";
    let parts = vec!["DEL", "500"];
    let result = handle_del(
        &parts,
        input,
        &variables,
        &mut patterns,
        &mut counters,
        &scripts,
        0,
        &metro_tx,
        &create_test_scale(),
        1,
        |output: String| outputs.push(output),
    );
    assert!(result.is_ok());
    assert!(outputs.iter().any(|s| s.contains("ERROR: DEL REQUIRES FORMAT")));
}

#[test]
fn test_del_max_16000ms() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let mut counters = create_test_counters();
    let scripts = create_test_scripts();
    let (metro_tx, _metro_rx) = mpsc::channel::<MetroCommand>();
    let mut outputs: Vec<String> = Vec::new();

    let input = "DEL 16001: TR";
    let parts = vec!["DEL", "16001:"];
    let result = handle_del(
        &parts,
        input,
        &variables,
        &mut patterns,
        &mut counters,
        &scripts,
        0,
        &metro_tx,
        &create_test_scale(),
        1,
        |output: String| outputs.push(output),
    );
    assert!(result.is_ok());
    assert!(outputs.iter().any(|s| s.contains("ERROR: DELAY TIME EXCEEDS MAXIMUM OF 16000MS")));
}

#[test]
fn test_del_clr_clears_buffer() {
    let (metro_tx, metro_rx) = mpsc::channel::<MetroCommand>();
    let mut outputs: Vec<String> = Vec::new();

    let result = handle_del_clr(
        &metro_tx,
        1,
        |output: String| outputs.push(output),
    );
    assert!(result.is_ok());
    assert!(outputs.iter().any(|s| s.contains("CLEARED ALL DELAYED COMMANDS")));

    let msg = metro_rx.try_recv();
    assert!(msg.is_ok());
    assert!(matches!(msg.unwrap(), MetroCommand::ClearDelayed));
}

#[test]
fn test_del_x_parses_count_and_ms() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let mut counters = create_test_counters();
    let scripts = create_test_scripts();
    let (metro_tx, metro_rx) = mpsc::channel::<MetroCommand>();
    let mut outputs: Vec<String> = Vec::new();

    let input = "DEL.X 4 100: TR";
    let parts = vec!["DEL.X", "4", "100:"];
    let result = handle_del_x(
        &parts,
        input,
        &variables,
        &mut patterns,
        &mut counters,
        &scripts,
        0,
        &metro_tx,
        &create_test_scale(),
        1,
        |output: String| outputs.push(output),
    );
    assert!(result.is_ok());
    assert!(outputs.iter().any(|s| s.contains("SCHEDULED COMMAND 4 TIMES AT 100MS INTERVALS")));

    let msg = metro_rx.try_recv();
    assert!(msg.is_ok());
    if let Ok(MetroCommand::ScheduleRepeated(cmd, count, interval_ms, _script_index)) = msg {
        assert_eq!(cmd, "TR");
        assert_eq!(count, 4);
        assert_eq!(interval_ms, 100);
    }
}

#[test]
fn test_del_r_parses_count_and_ms() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let mut counters = create_test_counters();
    let scripts = create_test_scripts();
    let (metro_tx, metro_rx) = mpsc::channel::<MetroCommand>();
    let mut outputs: Vec<String> = Vec::new();

    let input = "DEL.R 3 200: PF 440";
    let parts = vec!["DEL.R", "3", "200:"];
    let result = handle_del_r(
        &parts,
        input,
        &variables,
        &mut patterns,
        &mut counters,
        &scripts,
        0,
        &metro_tx,
        &create_test_scale(),
        1,
        |output: String| outputs.push(output),
    );
    assert!(result.is_ok());

    let msg1 = metro_rx.try_recv();
    assert!(msg1.is_ok());
    if let Ok(MetroCommand::ScheduleDelayed(cmd, delay_ms, _script_index)) = msg1 {
        assert_eq!(cmd, "PF 440");
        assert_eq!(delay_ms, 0);
    }

    let msg2 = metro_rx.try_recv();
    assert!(msg2.is_ok());
    if let Ok(MetroCommand::ScheduleRepeated(cmd, count, interval_ms, _script_index)) = msg2 {
        assert_eq!(cmd, "PF 440");
        assert_eq!(count, 2);
        assert_eq!(interval_ms, 200);
    }
}

#[test]
fn test_del_with_expression() {
    let mut variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let mut counters = create_test_counters();
    let scripts = create_test_scripts();
    let (metro_tx, metro_rx) = mpsc::channel::<MetroCommand>();
    let mut outputs: Vec<String> = Vec::new();

    variables.a = 100;

    let input = "DEL MUL A 10: TR";
    let parts = vec!["DEL", "MUL"];
    let result = handle_del(
        &parts,
        input,
        &variables,
        &mut patterns,
        &mut counters,
        &scripts,
        0,
        &metro_tx,
        &create_test_scale(),
        1,
        |output: String| outputs.push(output),
    );
    assert!(result.is_ok());
    assert!(outputs.iter().any(|s| s.contains("SCHEDULED COMMAND IN 1000MS")));

    let msg = metro_rx.try_recv();
    assert!(msg.is_ok());
    if let Ok(MetroCommand::ScheduleDelayed(cmd, delay_ms, _script_index)) = msg {
        assert_eq!(cmd, "TR");
        assert_eq!(delay_ms, 1000);
    }
}

#[test]
fn test_del_valid_commands() {
    assert!(validate_script_command("DEL 100: TR").is_ok());
    assert!(validate_script_command("DEL 500: PF 440").is_ok());
    assert!(validate_script_command("DEL 1000: A 100").is_ok());
    assert!(validate_script_command("DEL.CLR").is_ok());
    assert!(validate_script_command("DEL.X 4 100: TR").is_ok());
    assert!(validate_script_command("DEL.R 3 200: TR").is_ok());
}
