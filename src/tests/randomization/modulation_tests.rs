use crate::commands::randomization::{handle_rnd_mod, handle_rnd_env};
use crate::commands::validate_script_command;
use crate::types::MetroCommand;
use rosc::OscType;
use std::sync::mpsc;

#[test]
fn test_rnd_mod_valid_commands() {
    assert!(validate_script_command("RND.MOD").is_ok());
}

#[test]
fn test_rnd_env_valid_commands() {
    assert!(validate_script_command("RND.ENV").is_ok());
}

#[test]
fn test_rnd_mod_sends_all_params() {
    let (metro_tx, metro_rx) = mpsc::channel::<MetroCommand>();
    let mut outputs: Vec<String> = Vec::new();

    let result = handle_rnd_mod(&metro_tx, 2, |output: String| outputs.push(output));
    assert!(result.is_ok());

    let mut param_count = 0;
    let mut found_params = std::collections::HashSet::new();

    while let Ok(msg) = metro_rx.try_recv() {
        if let MetroCommand::SendParam(param, _) = msg {
            found_params.insert(param);
            param_count += 1;
        }
    }

    assert_eq!(param_count, 6);
    assert!(found_params.contains("mb"));
    assert!(found_params.contains("tk"));
    assert!(found_params.contains("mp"));
    assert!(found_params.contains("md"));
    assert!(found_params.contains("mt"));
    assert!(found_params.contains("ma"));
}

#[test]
fn test_rnd_env_sends_all_params() {
    let (metro_tx, metro_rx) = mpsc::channel::<MetroCommand>();
    let mut outputs: Vec<String> = Vec::new();

    let result = handle_rnd_env(&metro_tx, 2, |output: String| outputs.push(output));
    assert!(result.is_ok());

    let mut param_count = 0;
    let mut found_params = std::collections::HashSet::new();

    while let Ok(msg) = metro_rx.try_recv() {
        if let MetroCommand::SendParam(param, _) = msg {
            found_params.insert(param);
            param_count += 1;
        }
    }

    assert_eq!(param_count, 7);
    assert!(found_params.contains("env_atk"));
    assert!(found_params.contains("env_dec"));
    assert!(found_params.contains("env_crv"));
    assert!(found_params.contains("env_mode"));
    assert!(found_params.contains("pa"));
    assert!(found_params.contains("fa"));
    assert!(found_params.contains("da"));
}

#[test]
fn test_rnd_mod_param_ranges() {
    let (metro_tx, metro_rx) = mpsc::channel::<MetroCommand>();

    for _ in 0..10 {
        let result = handle_rnd_mod(&metro_tx, 0, |_| {});
        assert!(result.is_ok());

        while let Ok(msg) = metro_rx.try_recv() {
            if let MetroCommand::SendParam(param, value) = msg {
                match param.as_str() {
                    "mb" => {
                        if let OscType::Int(v) = value {
                            assert!(v >= 0 && v <= 8000, "MB out of range: {}", v);
                        }
                    }
                    "tk" => {
                        if let OscType::Int(v) = value {
                            assert!(v >= 0 && v <= 8000, "TK out of range: {}", v);
                        }
                    }
                    "mp" | "md" | "mt" | "ma" => {
                        if let OscType::Int(v) = value {
                            assert!(v >= 0 && v <= 1, "{} out of range: {}", param.to_uppercase(), v);
                        }
                    }
                    _ => panic!("Unexpected param: {}", param),
                }
            }
        }
    }
}

#[test]
fn test_rnd_env_param_ranges() {
    let (metro_tx, metro_rx) = mpsc::channel::<MetroCommand>();

    for _ in 0..10 {
        let result = handle_rnd_env(&metro_tx, 0, |_| {});
        assert!(result.is_ok());

        while let Ok(msg) = metro_rx.try_recv() {
            if let MetroCommand::SendParam(param, value) = msg {
                match param.as_str() {
                    "env_atk" | "env_dec" => {
                        if let OscType::Int(v) = value {
                            assert!(v >= 5 && v <= 2000, "{} out of range: {}", param.to_uppercase(), v);
                        }
                    }
                    "env_crv" => {
                        if let OscType::Float(v) = value {
                            assert!(v >= -4.0 && v <= 4.0, "ENV_CRV out of range: {}", v);
                        }
                    }
                    "env_mode" => {
                        if let OscType::Int(v) = value {
                            assert!(v >= 0 && v <= 2, "ENV_MODE out of range: {}", v);
                        }
                    }
                    "pa" => {
                        if let OscType::Float(v) = value {
                            assert!(v >= 0.0 && v <= 8.0, "PA out of range: {}", v);
                        }
                    }
                    "fa" | "da" => {
                        if let OscType::Int(v) = value {
                            assert!(v >= 0 && v <= 8, "{} out of range: {}", param.to_uppercase(), v);
                        }
                    }
                    _ => panic!("Unexpected param: {}", param),
                }
            }
        }
    }
}

#[test]
fn test_rnd_mod_produces_output_in_debug_mode() {
    let (metro_tx, _metro_rx) = mpsc::channel::<MetroCommand>();
    let mut outputs: Vec<String> = Vec::new();

    let result = handle_rnd_mod(&metro_tx, 2, |output: String| outputs.push(output));
    assert!(result.is_ok());
    assert_eq!(outputs.len(), 1);
    assert!(outputs[0].contains("RANDOMIZED MODULATION"));
    assert!(outputs[0].contains("MB="));
    assert!(outputs[0].contains("TK="));
    assert!(outputs[0].contains("MP="));
}

#[test]
fn test_rnd_env_produces_output_in_debug_mode() {
    let (metro_tx, _metro_rx) = mpsc::channel::<MetroCommand>();
    let mut outputs: Vec<String> = Vec::new();

    let result = handle_rnd_env(&metro_tx, 2, |output: String| outputs.push(output));
    assert!(result.is_ok());
    assert_eq!(outputs.len(), 1);
    assert!(outputs[0].contains("RANDOMIZED ENVELOPES"));
    assert!(outputs[0].contains("ATK="));
    assert!(outputs[0].contains("PA="));
}

#[test]
fn test_rnd_mod_no_output_in_silent_mode() {
    let (metro_tx, _metro_rx) = mpsc::channel::<MetroCommand>();
    let mut outputs: Vec<String> = Vec::new();

    let result = handle_rnd_mod(&metro_tx, 0, |output: String| outputs.push(output));
    assert!(result.is_ok());
    assert_eq!(outputs.len(), 0);
}

#[test]
fn test_rnd_env_no_output_in_silent_mode() {
    let (metro_tx, _metro_rx) = mpsc::channel::<MetroCommand>();
    let mut outputs: Vec<String> = Vec::new();

    let result = handle_rnd_env(&metro_tx, 0, |output: String| outputs.push(output));
    assert!(result.is_ok());
    assert_eq!(outputs.len(), 0);
}
