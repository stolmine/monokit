use crate::commands::validate_script_command;

#[test]
fn test_validate_valid_commands() {
    assert!(validate_script_command("TR").is_ok());
    assert!(validate_script_command("PF 440").is_ok());
    assert!(validate_script_command("A 100").is_ok());
    assert!(validate_script_command("P.NEXT").is_ok());
    assert!(validate_script_command("IF A > 5: TR").is_ok());
    assert!(validate_script_command("SCRIPT 1").is_ok());
    assert!(validate_script_command("M.ACT 1").is_ok());
    assert!(validate_script_command("").is_ok());
}

#[test]
fn test_validate_invalid_commands() {
    assert!(validate_script_command("INVALID_CMD").is_err());
    assert!(validate_script_command("NOTREAL 123").is_err());
    assert!(validate_script_command("BADCOMMAND").is_err());
}

#[test]
fn test_validate_commands_with_missing_args() {
    assert!(validate_script_command("VOL").is_err());
    assert!(validate_script_command("PF").is_err());
    assert!(validate_script_command("DC").is_err());
}

#[test]
fn test_validate_control_flow() {
    assert!(validate_script_command("IF A > 0: TR").is_ok());
    assert!(validate_script_command("ELIF A < 10: PF 200").is_ok());
    assert!(validate_script_command("ELSE: RST").is_ok());
    assert!(validate_script_command("PROB 50: TR").is_ok());
    assert!(validate_script_command("EV 4: TR").is_ok());
    assert!(validate_script_command("SKIP 2: PF 100").is_ok());
    assert!(validate_script_command("L 0 10: A I").is_ok());
}

#[test]
fn test_validate_semicolon_commands() {
    assert!(validate_script_command("TR; PF 440; VOL 0.5").is_ok());
}

#[test]
fn test_validate_synth_params_with_expressions() {
    assert!(validate_script_command("PA PN.NEXT 0").is_ok());
    assert!(validate_script_command("PF ADD 100 200").is_ok());
    assert!(validate_script_command("DC MUL A B").is_ok());
    assert!(validate_script_command("TK RND 100").is_ok());
    assert!(validate_script_command("FM PN.HERE 1").is_ok());
    assert!(validate_script_command("AD SUB 1000 T").is_ok());
}

#[test]
fn test_validate_pattern_ops_with_expressions() {
    assert!(validate_script_command("P ADD 5 10 100").is_ok());
    assert!(validate_script_command("PN.NEXT ADD 1 2").is_ok());
    assert!(validate_script_command("PN.L MUL 2 3").is_ok());
    assert!(validate_script_command("PN ADD A B 10 200").is_ok());
}

#[test]
fn test_validate_math_ops_with_expressions() {
    assert!(validate_script_command("ADD PN.NEXT 0 10").is_ok());
    assert!(validate_script_command("MUL RND 100 5").is_ok());
    assert!(validate_script_command("SUB ADD 10 20 5").is_ok());
}

#[test]
fn test_validate_comparison_ops_with_expressions() {
    assert!(validate_script_command("EZ PN.NEXT 0").is_ok());
    assert!(validate_script_command("EQ ADD 1 2 3").is_ok());
    assert!(validate_script_command("GT MUL A 2 10").is_ok());
}

#[test]
fn test_validate_canonical_names() {
    // All canonical forms should be resolved to aliases and validated
    assert!(validate_script_command("POSC.FREQ 440").is_ok());
    assert!(validate_script_command("MOSC.FREQ 880").is_ok());
    assert!(validate_script_command("MBUS.FM 8192").is_ok());
    assert!(validate_script_command("MBUS.AMT 1000").is_ok());
    assert!(validate_script_command("MBUS.TRK 500").is_ok());
    assert!(validate_script_command("FILT.CUT 2000").is_ok());
    assert!(validate_script_command("FILT.RES 8000").is_ok());
    assert!(validate_script_command("DLY.TIME 500").is_ok());
    assert!(validate_script_command("DLY.WET 8192").is_ok());
    assert!(validate_script_command("REV.WET 4096").is_ok());
    assert!(validate_script_command("REV.DEC 12000").is_ok());
    assert!(validate_script_command("DISC.AMT 5000").is_ok());
    assert!(validate_script_command("DISC.MODE 2").is_ok());
    assert!(validate_script_command("LOFI.BIT 8").is_ok());
    assert!(validate_script_command("RING.FRQ 440").is_ok());
    assert!(validate_script_command("COMP.THR 8000").is_ok());
    assert!(validate_script_command("EQ.LOW 3").is_ok());
    assert!(validate_script_command("EQ 2").is_ok()); // EQ (mid Q bandwidth) as command, not comparison
    assert!(validate_script_command("RESO.FRQ 200").is_ok());
    assert!(validate_script_command("OUT.VOL 1").is_ok());
    assert!(validate_script_command("OUT.PAN 0").is_ok());
    assert!(validate_script_command("AENV.DEC 500").is_ok());
    assert!(validate_script_command("PENV.AMT 4").is_ok());
    assert!(validate_script_command("FMEV.AMT 2").is_ok());
    assert!(validate_script_command("ROUT.MP 1").is_ok());
    assert!(validate_script_command("ROUT.MF 1").is_ok());
}

#[test]
fn test_validate_seq_no_space_before_quote() {
    assert!(validate_script_command("SEQ\"C3 E3 G3\"").is_err());
    assert!(validate_script_command("SEQ'C3 E3'").is_err());
    assert!(validate_script_command("PF SEQ\"C3\"").is_err());
}

#[test]
fn test_validate_seq_unclosed_quote() {
    assert!(validate_script_command("SEQ \"C3 E3").is_err());
    assert!(validate_script_command("SEQ 'C3 E3").is_err());
    assert!(validate_script_command("SEQ \"").is_err());
    assert!(validate_script_command("SEQ '").is_err());
}

#[test]
fn test_validate_seq_valid() {
    assert!(validate_script_command("PF SEQ \"C3 E3\"").is_ok());
    assert!(validate_script_command("PA SEQ 'x _ x _'").is_ok());
}

#[test]
fn test_validate_tog_duplicate_values() {
    assert!(validate_script_command("TOG 50 50").is_err());
    assert!(validate_script_command("TOG 100 100").is_err());
    assert!(validate_script_command("TOG 0 0").is_err());
}

#[test]
fn test_validate_tog_valid() {
    assert!(validate_script_command("TOG 50 100").is_ok());
    assert!(validate_script_command("TOG 0 1").is_ok());
    assert!(validate_script_command("TOG A B").is_ok());
}
