use crate::commands::randomization::{handle_rnd_voice, handle_rnd_osc, handle_rnd_fm, handle_rnd_mod, handle_rnd_env, handle_rnd_p, handle_rnd_pn, handle_rnd_pall};
use crate::commands::validate_script_command;
use crate::types::MetroCommand;
use rosc::OscType;
use std::sync::mpsc;
use super::common::{create_test_variables, create_test_patterns, create_test_scripts, create_test_counters, create_test_scale};

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

#[test]
fn test_rnd_p_valid_commands() {
    assert!(validate_script_command("RND.P").is_ok());
    assert!(validate_script_command("RND.P 0 100").is_ok());
}

#[test]
fn test_rnd_pn_valid_commands() {
    assert!(validate_script_command("RND.PN 0").is_ok());
    assert!(validate_script_command("RND.PN 0 0 100").is_ok());
}

#[test]
fn test_rnd_pall_valid_commands() {
    assert!(validate_script_command("RND.PALL").is_ok());
    assert!(validate_script_command("RND.PALL 0 127").is_ok());
}

#[test]
fn test_rnd_p_default_range() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let mut counters = create_test_counters();
    let mut outputs: Vec<String> = Vec::new();

    patterns.patterns[0].length = 8;
    patterns.working = 0;

    let parts = vec!["RND.P"];
    let result = handle_rnd_p(&parts, &variables, &mut patterns, &mut counters, &scripts, 0, &create_test_scale(), |output: String| outputs.push(output));
    assert!(result.is_ok());
    assert_eq!(outputs.len(), 1);
    assert!(outputs[0].contains("RANDOMIZED PATTERN 0"));
    assert!(outputs[0].contains("RANGE 0 TO 127"));

    for i in 0..8 {
        assert!(patterns.patterns[0].data[i] >= 0 && patterns.patterns[0].data[i] <= 127);
    }
}

#[test]
fn test_rnd_p_custom_range() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let mut counters = create_test_counters();
    let mut outputs: Vec<String> = Vec::new();

    patterns.patterns[0].length = 8;
    patterns.working = 0;

    let parts = vec!["RND.P", "10", "50"];
    let result = handle_rnd_p(&parts, &variables, &mut patterns, &mut counters, &scripts, 0, &create_test_scale(), |output: String| outputs.push(output));
    assert!(result.is_ok());
    assert_eq!(outputs.len(), 1);
    assert!(outputs[0].contains("RANDOMIZED PATTERN 0"));
    assert!(outputs[0].contains("RANGE 10 TO 50"));

    for i in 0..8 {
        assert!(patterns.patterns[0].data[i] >= 10 && patterns.patterns[0].data[i] <= 50);
    }
}

#[test]
fn test_rnd_pn_default_range() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let mut counters = create_test_counters();
    let mut outputs: Vec<String> = Vec::new();

    patterns.patterns[2].length = 8;

    let parts = vec!["RND.PN", "2"];
    let result = handle_rnd_pn(&parts, &variables, &mut patterns, &mut counters, &scripts, 0, &create_test_scale(), |output: String| outputs.push(output));
    assert!(result.is_ok());
    assert_eq!(outputs.len(), 1);
    assert!(outputs[0].contains("RANDOMIZED PATTERN 2"));
    assert!(outputs[0].contains("RANGE 0 TO 127"));

    for i in 0..8 {
        assert!(patterns.patterns[2].data[i] >= 0 && patterns.patterns[2].data[i] <= 127);
    }
}

#[test]
fn test_rnd_pn_custom_range() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let mut counters = create_test_counters();
    let mut outputs: Vec<String> = Vec::new();

    patterns.patterns[3].length = 8;

    let parts = vec!["RND.PN", "3", "-50", "50"];
    let result = handle_rnd_pn(&parts, &variables, &mut patterns, &mut counters, &scripts, 0, &create_test_scale(), |output: String| outputs.push(output));
    assert!(result.is_ok());
    assert_eq!(outputs.len(), 1);
    assert!(outputs[0].contains("RANDOMIZED PATTERN 3"));
    assert!(outputs[0].contains("RANGE -50 TO 50"));

    for i in 0..8 {
        assert!(patterns.patterns[3].data[i] >= -50 && patterns.patterns[3].data[i] <= 50);
    }
}

#[test]
fn test_rnd_pn_invalid_pattern() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let mut counters = create_test_counters();
    let mut outputs: Vec<String> = Vec::new();

    let parts = vec!["RND.PN", "6"];
    let result = handle_rnd_pn(&parts, &variables, &mut patterns, &mut counters, &scripts, 0, &create_test_scale(), |output: String| outputs.push(output));
    assert!(result.is_ok());
    assert_eq!(outputs.len(), 1);
    assert!(outputs[0].contains("ERROR"));
    assert!(outputs[0].contains("0-5"));
}

#[test]
fn test_rnd_pn_missing_pattern_number() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let mut counters = create_test_counters();
    let mut outputs: Vec<String> = Vec::new();

    let parts = vec!["RND.PN"];
    let result = handle_rnd_pn(&parts, &variables, &mut patterns, &mut counters, &scripts, 0, &create_test_scale(), |output: String| outputs.push(output));
    assert!(result.is_ok());
    assert_eq!(outputs.len(), 1);
    assert!(outputs[0].contains("ERROR"));
    assert!(outputs[0].contains("REQUIRES PATTERN NUMBER"));
}

#[test]
fn test_rnd_pall_default_range() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let mut counters = create_test_counters();
    let mut outputs: Vec<String> = Vec::new();

    for i in 0..6 {
        patterns.patterns[i].length = 4;
    }

    let parts = vec!["RND.PALL"];
    let result = handle_rnd_pall(&parts, &variables, &mut patterns, &mut counters, &scripts, 0, &create_test_scale(), |output: String| outputs.push(output));
    assert!(result.is_ok());
    assert_eq!(outputs.len(), 1);
    assert!(outputs[0].contains("RANDOMIZED ALL PATTERNS"));
    assert!(outputs[0].contains("RANGE 0 TO 127"));

    for pat_idx in 0..6 {
        for i in 0..4 {
            assert!(patterns.patterns[pat_idx].data[i] >= 0 && patterns.patterns[pat_idx].data[i] <= 127);
        }
    }
}

#[test]
fn test_rnd_pall_custom_range() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let mut counters = create_test_counters();
    let mut outputs: Vec<String> = Vec::new();

    for i in 0..6 {
        patterns.patterns[i].length = 4;
    }

    let parts = vec!["RND.PALL", "0", "10"];
    let result = handle_rnd_pall(&parts, &variables, &mut patterns, &mut counters, &scripts, 0, &create_test_scale(), |output: String| outputs.push(output));
    assert!(result.is_ok());
    assert_eq!(outputs.len(), 1);
    assert!(outputs[0].contains("RANDOMIZED ALL PATTERNS"));
    assert!(outputs[0].contains("RANGE 0 TO 10"));

    for pat_idx in 0..6 {
        for i in 0..4 {
            assert!(patterns.patterns[pat_idx].data[i] >= 0 && patterns.patterns[pat_idx].data[i] <= 10);
        }
    }
}

#[test]
fn test_rnd_p_with_expression() {
    let mut variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let mut counters = create_test_counters();
    let mut outputs: Vec<String> = Vec::new();

    variables.a = 20;
    variables.b = 80;
    patterns.patterns[0].length = 8;
    patterns.working = 0;

    let parts = vec!["RND.P", "A", "B"];
    let result = handle_rnd_p(&parts, &variables, &mut patterns, &mut counters, &scripts, 0, &create_test_scale(), |output: String| outputs.push(output));
    assert!(result.is_ok());
    assert_eq!(outputs.len(), 1);
    assert!(outputs[0].contains("RANDOMIZED PATTERN 0"));
    assert!(outputs[0].contains("RANGE 20 TO 80"));

    for i in 0..8 {
        assert!(patterns.patterns[0].data[i] >= 20 && patterns.patterns[0].data[i] <= 80);
    }
}

#[test]
fn test_rnd_pn_with_expression() {
    let mut variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let mut counters = create_test_counters();
    let mut outputs: Vec<String> = Vec::new();

    variables.a = 1;
    variables.b = 100;
    variables.c = 200;
    patterns.patterns[1].length = 8;

    let parts = vec!["RND.PN", "A", "B", "C"];
    let result = handle_rnd_pn(&parts, &variables, &mut patterns, &mut counters, &scripts, 0, &create_test_scale(), |output: String| outputs.push(output));
    assert!(result.is_ok());
    assert_eq!(outputs.len(), 1);
    assert!(outputs[0].contains("RANDOMIZED PATTERN 1"));
    assert!(outputs[0].contains("RANGE 100 TO 200"));

    for i in 0..8 {
        assert!(patterns.patterns[1].data[i] >= 100 && patterns.patterns[1].data[i] <= 200);
    }
}
