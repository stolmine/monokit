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

#[test]
fn test_validate_pn_commands_require_pattern_arg() {
    // Query commands - require pattern number
    assert!(validate_script_command("PN.NEXT").is_err());
    assert!(validate_script_command("PN.PREV").is_err());
    assert!(validate_script_command("PN.HERE").is_err());
    assert!(validate_script_command("PN.MIN").is_err());
    assert!(validate_script_command("PN.MAX").is_err());
    assert!(validate_script_command("PN.SUM").is_err());
    assert!(validate_script_command("PN.AVG").is_err());
    assert!(validate_script_command("PN.FND").is_err());

    // State commands - require pattern number
    assert!(validate_script_command("PN.L").is_err());
    assert!(validate_script_command("PN.I").is_err());

    // Manipulation commands - require pattern number
    assert!(validate_script_command("PN.PUSH").is_err());
    assert!(validate_script_command("PN.POP").is_err());
    assert!(validate_script_command("PN.REV").is_err());
    assert!(validate_script_command("PN.ROT").is_err());
    assert!(validate_script_command("PN.SHUF").is_err());
    assert!(validate_script_command("PN.SORT").is_err());
    assert!(validate_script_command("PN.RND").is_err());

    // Math commands - require pattern number and value
    assert!(validate_script_command("PN.ADD").is_err());
    assert!(validate_script_command("PN.SUB").is_err());
    assert!(validate_script_command("PN.MUL").is_err());
    assert!(validate_script_command("PN.DIV").is_err());
    assert!(validate_script_command("PN.MOD").is_err());
    assert!(validate_script_command("PN.SCALE").is_err());

    // Base PN command - requires at least pattern and index
    assert!(validate_script_command("PN").is_err());
    assert!(validate_script_command("PN 0").is_err());
}

#[test]
fn test_validate_pn_commands_with_valid_args() {
    // Query commands with pattern number
    assert!(validate_script_command("PN.NEXT 0").is_ok());
    assert!(validate_script_command("PN.PREV 3").is_ok());
    assert!(validate_script_command("PN.HERE 5").is_ok());
    assert!(validate_script_command("PN.MIN 2").is_ok());

    // State commands with pattern number
    assert!(validate_script_command("PN.L 0").is_ok());
    assert!(validate_script_command("PN.I 1").is_ok());

    // Manipulation commands with pattern number
    assert!(validate_script_command("PN.POP 0").is_ok());
    assert!(validate_script_command("PN.REV 1").is_ok());
    assert!(validate_script_command("PN.RND 2").is_ok());

    // Math commands with pattern and value
    assert!(validate_script_command("PN.PUSH 0 100").is_ok());
    assert!(validate_script_command("PN.ADD 1 50").is_ok());
    assert!(validate_script_command("PN.SCALE 2 0 100").is_ok());

    // Base PN with pattern, index, and optional value
    assert!(validate_script_command("PN 0 0").is_ok());
    assert!(validate_script_command("PN 1 5 200").is_ok());
}

#[test]
fn test_validate_pn_as_expression_without_args() {
    // These should fail - PN.* used as expressions without required args
    // PN.NEXT at end of command with no pattern arg
    assert!(validate_script_command("PF PN.NEXT").is_err());
    assert!(validate_script_command("PA PN.HERE").is_err());

    // PN.NEXT in middle of expression without its arg
    // Note: "ADD PN.NEXT 10" is ambiguous - could be interpreted as PN.NEXT taking 10
    // The simpler case is when PN.* is clearly missing its arg
    assert!(validate_script_command("MUL PN.NEXT PN.HERE").is_err());
}

#[test]
fn test_validate_pn_as_expression_with_args() {
    // These should pass - PN.* used as expressions WITH required args
    assert!(validate_script_command("PF PN.NEXT 0").is_ok());
    assert!(validate_script_command("PA PN.HERE 1").is_ok());
    assert!(validate_script_command("ADD PN.NEXT 0 10").is_ok());
    assert!(validate_script_command("MUL PN.HERE 0 PN.NEXT 1").is_ok());
    assert!(validate_script_command("DC ADD PN.HERE 2 100").is_ok());
}

#[test]
fn test_validate_header_command() {
    // HEADER with no args (query current level)
    assert!(validate_script_command("HEADER").is_ok());

    // HEADER with valid levels 0-4
    assert!(validate_script_command("HEADER 0").is_ok());
    assert!(validate_script_command("HEADER 1").is_ok());
    assert!(validate_script_command("HEADER 2").is_ok());
    assert!(validate_script_command("HEADER 3").is_ok());
    assert!(validate_script_command("HEADER 4").is_ok());

    // HEADER with too many arguments should fail
    assert!(validate_script_command("HEADER 1 2").is_err());
}

#[test]
fn test_validate_cpu_command() {
    assert!(validate_script_command("CPU").is_ok());
    assert!(validate_script_command("CPU 0").is_ok());
    assert!(validate_script_command("CPU 1").is_ok());
    assert!(validate_script_command("CPU 1 2").is_err());
}

#[test]
fn test_validate_bpm_command() {
    assert!(validate_script_command("BPM").is_ok());
    assert!(validate_script_command("BPM 1").is_ok());
    assert!(validate_script_command("BPM 0").is_ok());
    assert!(validate_script_command("BPM 1 2").is_err());
}

#[test]
fn test_validate_counter_commands() {
    assert!(validate_script_command("N1").is_ok());
    assert!(validate_script_command("N2").is_ok());
    assert!(validate_script_command("N3").is_ok());
    assert!(validate_script_command("N4").is_ok());
    assert!(validate_script_command("N1 1").is_err());

    assert!(validate_script_command("N1.MAX").is_ok());
    assert!(validate_script_command("N1.MAX 10").is_ok());
    assert!(validate_script_command("N1.MAX 10 20").is_err());

    assert!(validate_script_command("N2.MIN").is_ok());
    assert!(validate_script_command("N2.MIN 5").is_ok());
    assert!(validate_script_command("N2.MIN 5 10").is_err());
}

#[test]
fn test_validate_ui_toggle_commands() {
    assert!(validate_script_command("METER.HDR").is_ok());
    assert!(validate_script_command("METER.HDR 1").is_ok());
    assert!(validate_script_command("METER.HDR 0 1").is_err());

    assert!(validate_script_command("METER.GRID").is_ok());
    assert!(validate_script_command("METER.GRID 1").is_ok());

    assert!(validate_script_command("ACTIVITY").is_ok());
    assert!(validate_script_command("ACTIVITY 0").is_ok());

    assert!(validate_script_command("GRID").is_ok());
    assert!(validate_script_command("GRID 1").is_ok());

    assert!(validate_script_command("GRID.DEF").is_ok());
    assert!(validate_script_command("GRID.DEF 0").is_ok());

    assert!(validate_script_command("GRID.MODE").is_ok());
    assert!(validate_script_command("GRID.MODE 1").is_ok());

    assert!(validate_script_command("HL.COND").is_ok());
    assert!(validate_script_command("HL.COND 1").is_ok());

    assert!(validate_script_command("HL.SEQ").is_ok());
    assert!(validate_script_command("HL.SEQ 0").is_ok());

    assert!(validate_script_command("SPECTRUM").is_ok());
    assert!(validate_script_command("SPECTRUM 1").is_ok());
}

#[test]
fn test_validate_system_commands() {
    assert!(validate_script_command("FLASH").is_ok());
    assert!(validate_script_command("FLASH 500").is_ok());
    assert!(validate_script_command("FLASH 100 200").is_err());

    assert!(validate_script_command("TITLE").is_ok());
    assert!(validate_script_command("TITLE 1").is_ok());
    assert!(validate_script_command("TITLE 1 2").is_err());

    assert!(validate_script_command("LIMIT").is_ok());
    assert!(validate_script_command("LIMIT 1").is_ok());
    assert!(validate_script_command("LIMIT 1 0").is_err());
}

#[test]
fn test_validate_diag_commands() {
    assert!(validate_script_command("SC.DIAG 1").is_ok());
    assert!(validate_script_command("SC.DIAG 1 2").is_ok());
    assert!(validate_script_command("SC.DIAG 1 2 3").is_err());

    assert!(validate_script_command("MIDI.DIAG 1").is_ok());
    assert!(validate_script_command("MIDI.DIAG 1 0").is_ok());
    assert!(validate_script_command("MIDI.DIAG 1 2 3").is_err());
}

#[test]
fn test_validate_metro_commands() {
    assert!(validate_script_command("M.ACT").is_ok());
    assert!(validate_script_command("M.ACT 1").is_ok());
    assert!(validate_script_command("M.ACT 1 2").is_err());

    assert!(validate_script_command("M.SCRIPT").is_ok());
    assert!(validate_script_command("M.SCRIPT 5").is_ok());
    assert!(validate_script_command("M.SCRIPT 1 2").is_err());
}

#[test]
fn test_validate_q_root_command() {
    assert!(validate_script_command("Q.ROOT").is_ok());
    assert!(validate_script_command("Q.ROOT 60").is_ok());
    assert!(validate_script_command("Q.ROOT 60 70").is_err());
}
