use crate::commands::randomization::{handle_rnd_voice, handle_rnd_osc, handle_rnd_fm};
use crate::commands::validate_script_command;
use crate::types::MetroCommand;
use rosc::OscType;
use std::sync::mpsc;

#[test]
fn test_rnd_voice_valid_commands() {
    assert!(validate_script_command("RND.VOICE").is_ok());
}

#[test]
fn test_rnd_osc_valid_commands() {
    assert!(validate_script_command("RND.OSC").is_ok());
}

#[test]
fn test_rnd_fm_valid_commands() {
    assert!(validate_script_command("RND.FM").is_ok());
}

#[test]
fn test_rnd_voice_sends_all_params() {
    let (metro_tx, metro_rx) = mpsc::channel::<MetroCommand>();
    let mut outputs: Vec<String> = Vec::new();

    let result = handle_rnd_voice(&metro_tx, 2, |output: String| outputs.push(output));
    assert!(result.is_ok());

    let mut param_count = 0;
    let mut found_params = std::collections::HashSet::new();

    while let Ok(msg) = metro_rx.try_recv() {
        if let MetroCommand::SendParam(param, _) = msg {
            found_params.insert(param);
            param_count += 1;
        }
    }

    assert_eq!(param_count, 8);
    assert!(found_params.contains("pf"));
    assert!(found_params.contains("pw"));
    assert!(found_params.contains("mf"));
    assert!(found_params.contains("mw"));
    assert!(found_params.contains("fm"));
    assert!(found_params.contains("fb"));
    assert!(found_params.contains("fba"));
    assert!(found_params.contains("fbd"));
}

#[test]
fn test_rnd_osc_sends_osc_params() {
    let (metro_tx, metro_rx) = mpsc::channel::<MetroCommand>();
    let mut outputs: Vec<String> = Vec::new();

    let result = handle_rnd_osc(&metro_tx, 2, |output: String| outputs.push(output));
    assert!(result.is_ok());

    let mut param_count = 0;
    let mut found_params = std::collections::HashSet::new();

    while let Ok(msg) = metro_rx.try_recv() {
        if let MetroCommand::SendParam(param, _) = msg {
            found_params.insert(param);
            param_count += 1;
        }
    }

    assert_eq!(param_count, 4);
    assert!(found_params.contains("pf"));
    assert!(found_params.contains("pw"));
    assert!(found_params.contains("mf"));
    assert!(found_params.contains("mw"));
}

#[test]
fn test_rnd_fm_sends_fm_params() {
    let (metro_tx, metro_rx) = mpsc::channel::<MetroCommand>();
    let mut outputs: Vec<String> = Vec::new();

    let result = handle_rnd_fm(&metro_tx, 2, |output: String| outputs.push(output));
    assert!(result.is_ok());

    let mut param_count = 0;
    let mut found_params = std::collections::HashSet::new();

    while let Ok(msg) = metro_rx.try_recv() {
        if let MetroCommand::SendParam(param, _) = msg {
            found_params.insert(param);
            param_count += 1;
        }
    }

    assert_eq!(param_count, 4);
    assert!(found_params.contains("fm"));
    assert!(found_params.contains("fb"));
    assert!(found_params.contains("fba"));
    assert!(found_params.contains("fbd"));
}

#[test]
fn test_rnd_voice_param_ranges() {
    let (metro_tx, metro_rx) = mpsc::channel::<MetroCommand>();

    for _ in 0..10 {
        let result = handle_rnd_voice(&metro_tx, 0, |_| {});
        assert!(result.is_ok());

        while let Ok(msg) = metro_rx.try_recv() {
            if let MetroCommand::SendParam(param, value) = msg {
                match param.as_str() {
                    "pf" => {
                        if let OscType::Float(v) = value {
                            assert!(v >= 50.0 && v <= 2000.0, "PF out of range: {}", v);
                        }
                    }
                    "pw" => {
                        if let OscType::Int(v) = value {
                            assert!(v >= 0 && v <= 2, "PW out of range: {}", v);
                        }
                    }
                    "mf" => {
                        if let OscType::Float(v) = value {
                            assert!(v >= 20.0 && v <= 1000.0, "MF out of range: {}", v);
                        }
                    }
                    "mw" => {
                        if let OscType::Int(v) = value {
                            assert!(v >= 0 && v <= 3, "MW out of range: {}", v);
                        }
                    }
                    "fm" => {
                        if let OscType::Int(v) = value {
                            assert!(v >= 0 && v <= 8000, "FM out of range: {}", v);
                        }
                    }
                    "fb" => {
                        if let OscType::Int(v) = value {
                            assert!(v >= 0 && v <= 4000, "FB out of range: {}", v);
                        }
                    }
                    "fba" => {
                        if let OscType::Int(v) = value {
                            assert!(v >= 0 && v <= 8000, "FBA out of range: {}", v);
                        }
                    }
                    "fbd" => {
                        if let OscType::Int(v) = value {
                            assert!(v >= 10 && v <= 2000, "FBD out of range: {}", v);
                        }
                    }
                    _ => panic!("Unexpected param: {}", param),
                }
            }
        }
    }
}

#[test]
fn test_rnd_osc_param_ranges() {
    let (metro_tx, metro_rx) = mpsc::channel::<MetroCommand>();

    for _ in 0..10 {
        let result = handle_rnd_osc(&metro_tx, 0, |_| {});
        assert!(result.is_ok());

        while let Ok(msg) = metro_rx.try_recv() {
            if let MetroCommand::SendParam(param, value) = msg {
                match param.as_str() {
                    "pf" => {
                        if let OscType::Float(v) = value {
                            assert!(v >= 50.0 && v <= 2000.0, "PF out of range: {}", v);
                        }
                    }
                    "pw" => {
                        if let OscType::Int(v) = value {
                            assert!(v >= 0 && v <= 2, "PW out of range: {}", v);
                        }
                    }
                    "mf" => {
                        if let OscType::Float(v) = value {
                            assert!(v >= 20.0 && v <= 1000.0, "MF out of range: {}", v);
                        }
                    }
                    "mw" => {
                        if let OscType::Int(v) = value {
                            assert!(v >= 0 && v <= 3, "MW out of range: {}", v);
                        }
                    }
                    _ => panic!("Unexpected param: {}", param),
                }
            }
        }
    }
}

#[test]
fn test_rnd_fm_param_ranges() {
    let (metro_tx, metro_rx) = mpsc::channel::<MetroCommand>();

    for _ in 0..10 {
        let result = handle_rnd_fm(&metro_tx, 0, |_| {});
        assert!(result.is_ok());

        while let Ok(msg) = metro_rx.try_recv() {
            if let MetroCommand::SendParam(param, value) = msg {
                match param.as_str() {
                    "fm" => {
                        if let OscType::Int(v) = value {
                            assert!(v >= 0 && v <= 8000, "FM out of range: {}", v);
                        }
                    }
                    "fb" => {
                        if let OscType::Int(v) = value {
                            assert!(v >= 0 && v <= 4000, "FB out of range: {}", v);
                        }
                    }
                    "fba" => {
                        if let OscType::Int(v) = value {
                            assert!(v >= 0 && v <= 8000, "FBA out of range: {}", v);
                        }
                    }
                    "fbd" => {
                        if let OscType::Int(v) = value {
                            assert!(v >= 10 && v <= 2000, "FBD out of range: {}", v);
                        }
                    }
                    _ => panic!("Unexpected param: {}", param),
                }
            }
        }
    }
}

#[test]
fn test_rnd_voice_produces_output_in_debug_mode() {
    let (metro_tx, _metro_rx) = mpsc::channel::<MetroCommand>();
    let mut outputs: Vec<String> = Vec::new();

    let result = handle_rnd_voice(&metro_tx, 2, |output: String| outputs.push(output));
    assert!(result.is_ok());
    assert_eq!(outputs.len(), 1);
    assert!(outputs[0].contains("RANDOMIZED VOICE"));
    assert!(outputs[0].contains("PF="));
    assert!(outputs[0].contains("MF="));
    assert!(outputs[0].contains("FM="));
    assert!(outputs[0].contains("FB="));
}

#[test]
fn test_rnd_osc_produces_output_in_debug_mode() {
    let (metro_tx, _metro_rx) = mpsc::channel::<MetroCommand>();
    let mut outputs: Vec<String> = Vec::new();

    let result = handle_rnd_osc(&metro_tx, 2, |output: String| outputs.push(output));
    assert!(result.is_ok());
    assert_eq!(outputs.len(), 1);
    assert!(outputs[0].contains("RANDOMIZED OSCILLATORS"));
}

#[test]
fn test_rnd_fm_produces_output_in_debug_mode() {
    let (metro_tx, _metro_rx) = mpsc::channel::<MetroCommand>();
    let mut outputs: Vec<String> = Vec::new();

    let result = handle_rnd_fm(&metro_tx, 2, |output: String| outputs.push(output));
    assert!(result.is_ok());
    assert_eq!(outputs.len(), 1);
    assert!(outputs[0].contains("RANDOMIZED FM"));
}

#[test]
fn test_rnd_voice_no_output_in_silent_mode() {
    let (metro_tx, _metro_rx) = mpsc::channel::<MetroCommand>();
    let mut outputs: Vec<String> = Vec::new();

    let result = handle_rnd_voice(&metro_tx, 0, |output: String| outputs.push(output));
    assert!(result.is_ok());
    assert_eq!(outputs.len(), 0);
}
