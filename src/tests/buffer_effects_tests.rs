use crate::commands::validate_script_command;

#[test]
fn test_br_act_valid() {
    assert!(validate_script_command("BR.ACT 0").is_ok());
    assert!(validate_script_command("BR.ACT 1").is_ok());
}

#[test]
fn test_br_len_valid() {
    assert!(validate_script_command("BR.LEN 0").is_ok());
    assert!(validate_script_command("BR.LEN 3").is_ok());
    assert!(validate_script_command("BR.LEN 7").is_ok());
}

#[test]
fn test_br_rev_valid() {
    assert!(validate_script_command("BR.REV 0").is_ok());
    assert!(validate_script_command("BR.REV 1").is_ok());
}

#[test]
fn test_br_win_valid() {
    assert!(validate_script_command("BR.WIN 1").is_ok());
    assert!(validate_script_command("BR.WIN 25").is_ok());
    assert!(validate_script_command("BR.WIN 50").is_ok());
}

#[test]
fn test_br_mix_valid() {
    assert!(validate_script_command("BR.MIX 0").is_ok());
    assert!(validate_script_command("BR.MIX 8192").is_ok());
    assert!(validate_script_command("BR.MIX 16383").is_ok());
}

#[test]
fn test_br_commands_with_expressions() {
    assert!(validate_script_command("BR.MIX ADD 100 200").is_ok());
    assert!(validate_script_command("BR.LEN SUB 5 2").is_ok());
    assert!(validate_script_command("BR.WIN MUL 10 2").is_ok());
    assert!(validate_script_command("BR.ACT RND 2").is_ok());
}

#[test]
fn test_br_commands_missing_args() {
    assert!(validate_script_command("BR.ACT").is_err());
    assert!(validate_script_command("BR.LEN").is_err());
    assert!(validate_script_command("BR.REV").is_err());
    assert!(validate_script_command("BR.WIN").is_err());
    assert!(validate_script_command("BR.MIX").is_err());
}

#[test]
fn test_ps_mode_valid() {
    assert!(validate_script_command("PS.MODE 0").is_ok());
    assert!(validate_script_command("PS.MODE 1").is_ok());
}

#[test]
fn test_ps_semi_valid() {
    assert!(validate_script_command("PS.SEMI 0").is_ok());
    assert!(validate_script_command("PS.SEMI 12").is_ok());
    assert!(validate_script_command("PS.SEMI -12").is_ok());
    assert!(validate_script_command("PS.SEMI 24").is_ok());
    assert!(validate_script_command("PS.SEMI -24").is_ok());
}

#[test]
fn test_ps_grain_valid() {
    assert!(validate_script_command("PS.GRAIN 5").is_ok());
    assert!(validate_script_command("PS.GRAIN 50").is_ok());
    assert!(validate_script_command("PS.GRAIN 100").is_ok());
}

#[test]
fn test_ps_mix_valid() {
    assert!(validate_script_command("PS.MIX 0").is_ok());
    assert!(validate_script_command("PS.MIX 8192").is_ok());
    assert!(validate_script_command("PS.MIX 16383").is_ok());
}

#[test]
fn test_ps_targ_valid() {
    assert!(validate_script_command("PS.TARG 0").is_ok());
    assert!(validate_script_command("PS.TARG 1").is_ok());
}

#[test]
fn test_ps_commands_with_expressions() {
    assert!(validate_script_command("PS.SEMI SUB 0 12").is_ok());
    assert!(validate_script_command("PS.MIX ADD 8000 192").is_ok());
    assert!(validate_script_command("PS.GRAIN MUL 10 5").is_ok());
    assert!(validate_script_command("PS.MODE RND 2").is_ok());
}

#[test]
fn test_ps_semi_negative_with_expressions() {
    assert!(validate_script_command("PS.SEMI SUB 0 12").is_ok());
    assert!(validate_script_command("PS.SEMI SUB A 24").is_ok());
    assert!(validate_script_command("PS.SEMI MUL -1 12").is_ok());
}

#[test]
fn test_ps_commands_missing_args() {
    assert!(validate_script_command("PS.MODE").is_err());
    assert!(validate_script_command("PS.SEMI").is_err());
    assert!(validate_script_command("PS.GRAIN").is_err());
    assert!(validate_script_command("PS.MIX").is_err());
    assert!(validate_script_command("PS.TARG").is_err());
}

#[test]
fn test_br_and_ps_combined_usage() {
    assert!(validate_script_command("BR.ACT 1; PS.SEMI 12; PS.MIX 16383").is_ok());
    assert!(validate_script_command("BR.LEN 4; BR.MIX 8192; PS.TARG 1").is_ok());
}
