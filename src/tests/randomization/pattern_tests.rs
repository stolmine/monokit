use crate::commands::randomization::{handle_rnd_p, handle_rnd_pn, handle_rnd_pall};
use crate::commands::validate_script_command;
use crate::tests::common::{create_test_variables, create_test_patterns, create_test_scripts, create_test_counters, create_test_scale};

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
