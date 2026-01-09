use crate::commands::validate_script_command;
use crate::commands::resolve_alias;

// Tests for resolve_alias: converts short alias to canonical long form
// e.g., "PF" -> "POSC.FREQ", "SR" -> "S.RATE"

#[test]
fn test_alias_resolution_posc_freq() {
    // Alias resolves to canonical form
    assert_eq!(resolve_alias("PF"), "POSC.FREQ");
    // Canonical stays unchanged
    assert_eq!(resolve_alias("POSC.FREQ"), "POSC.FREQ");
}

#[test]
fn test_alias_resolution_filt_cut() {
    assert_eq!(resolve_alias("FC"), "FILT.CUT");
    assert_eq!(resolve_alias("FILT.CUT"), "FILT.CUT");
}

#[test]
fn test_alias_resolution_mosc_freq() {
    assert_eq!(resolve_alias("MF"), "MOSC.FREQ");
    assert_eq!(resolve_alias("MOSC.FREQ"), "MOSC.FREQ");
}

#[test]
fn test_alias_resolution_disc_amt() {
    assert_eq!(resolve_alias("DC"), "DISC.AMT");
    assert_eq!(resolve_alias("DISC.AMT"), "DISC.AMT");
}

#[test]
fn test_alias_resolution_unknown_command() {
    let result = resolve_alias("UNKNOWN.CMD");
    assert_eq!(result, "UNKNOWN.CMD");
}

#[test]
fn test_alias_resolution_short_commands_no_alias() {
    // Some short commands ARE aliases
    assert_eq!(resolve_alias("VOL"), "OUT.VOL");
    assert_eq!(resolve_alias("PAN"), "OUT.PAN");
    // TR has no canonical, stays unchanged
    assert_eq!(resolve_alias("TR"), "TR");
}

#[test]
fn test_alias_resolution_reverb() {
    assert_eq!(resolve_alias("RV"), "REV.DEC");
    assert_eq!(resolve_alias("RW"), "REV.WET");
}

#[test]
fn test_alias_resolution_delay() {
    assert_eq!(resolve_alias("DT"), "DLY.TIME");
    assert_eq!(resolve_alias("DF"), "DLY.FB");
}

#[test]
fn test_alias_resolution_sampler() {
    // Sampler aliases
    assert_eq!(resolve_alias("SR"), "S.RATE");
    assert_eq!(resolve_alias("SPT"), "S.PITCH");
    assert_eq!(resolve_alias("SFN"), "S.FINE");
    assert_eq!(resolve_alias("SD"), "S.DIR");
    assert_eq!(resolve_alias("SL"), "S.LOOP");
}

#[test]
fn test_env_atk_valid() {
    assert!(validate_script_command("ENV.ATK 100").is_ok());
    assert!(validate_script_command("ENV.ATK 1").is_ok());
    assert!(validate_script_command("ENV.ATK 10000").is_ok());
}

#[test]
fn test_env_atk_missing_arg() {
    assert!(validate_script_command("ENV.ATK").is_err());
}

#[test]
fn test_env_atk_too_many_args() {
    assert!(validate_script_command("ENV.ATK 100 200").is_err());
}

#[test]
fn test_env_dec_valid() {
    assert!(validate_script_command("ENV.DEC 500").is_ok());
    assert!(validate_script_command("ENV.DEC 1").is_ok());
    assert!(validate_script_command("ENV.DEC 10000").is_ok());
}

#[test]
fn test_env_dec_missing_arg() {
    assert!(validate_script_command("ENV.DEC").is_err());
}

#[test]
fn test_env_dec_too_many_args() {
    assert!(validate_script_command("ENV.DEC 500 1000").is_err());
}

#[test]
fn test_env_crv_valid() {
    assert!(validate_script_command("ENV.CRV 0").is_ok());
    assert!(validate_script_command("ENV.CRV -8").is_ok());
    assert!(validate_script_command("ENV.CRV 8").is_ok());
    assert!(validate_script_command("ENV.CRV 4.5").is_ok());
    assert!(validate_script_command("ENV.CRV -2.3").is_ok());
}

#[test]
fn test_env_crv_missing_arg() {
    assert!(validate_script_command("ENV.CRV").is_err());
}

#[test]
fn test_env_crv_too_many_args() {
    assert!(validate_script_command("ENV.CRV 2 4").is_err());
}

#[test]
fn test_env_mode_valid() {
    assert!(validate_script_command("ENV.MODE 0").is_ok());
    assert!(validate_script_command("ENV.MODE 1").is_ok());
    assert!(validate_script_command("ENV.MODE 2").is_ok());
}

#[test]
fn test_env_mode_missing_arg() {
    assert!(validate_script_command("ENV.MODE").is_err());
}

#[test]
fn test_env_mode_too_many_args() {
    assert!(validate_script_command("ENV.MODE 1 2").is_err());
}

#[test]
fn test_aenv_atk_valid() {
    assert!(validate_script_command("AENV.ATK 50").is_ok());
    assert!(validate_script_command("AENV.ATK 1").is_ok());
    assert!(validate_script_command("AENV.ATK 10000").is_ok());
}

#[test]
fn test_aenv_atk_missing_arg() {
    assert!(validate_script_command("AENV.ATK").is_err());
}

#[test]
fn test_aenv_atk_too_many_args() {
    assert!(validate_script_command("AENV.ATK 50 100").is_err());
}

#[test]
fn test_aenv_crv_valid() {
    assert!(validate_script_command("AENV.CRV 2").is_ok());
    assert!(validate_script_command("AENV.CRV -4").is_ok());
}

#[test]
fn test_aenv_crv_missing_arg() {
    assert!(validate_script_command("AENV.CRV").is_err());
}


#[test]
fn test_penv_atk_valid() {
    assert!(validate_script_command("PENV.ATK 75").is_ok());
}

#[test]
fn test_penv_atk_missing_arg() {
    assert!(validate_script_command("PENV.ATK").is_err());
}

#[test]
fn test_penv_crv_valid() {
    assert!(validate_script_command("PENV.CRV -3").is_ok());
}

#[test]
fn test_penv_crv_missing_arg() {
    assert!(validate_script_command("PENV.CRV").is_err());
}


#[test]
fn test_fmev_atk_valid() {
    assert!(validate_script_command("FMEV.ATK 200").is_ok());
}

#[test]
fn test_fmev_atk_missing_arg() {
    assert!(validate_script_command("FMEV.ATK").is_err());
}

#[test]
fn test_fmev_crv_valid() {
    assert!(validate_script_command("FMEV.CRV 1").is_ok());
}

#[test]
fn test_fmev_crv_missing_arg() {
    assert!(validate_script_command("FMEV.CRV").is_err());
}


#[test]
fn test_denv_atk_valid() {
    assert!(validate_script_command("DENV.ATK 150").is_ok());
}

#[test]
fn test_denv_atk_missing_arg() {
    assert!(validate_script_command("DENV.ATK").is_err());
}

#[test]
fn test_denv_crv_valid() {
    assert!(validate_script_command("DENV.CRV 5").is_ok());
}

#[test]
fn test_denv_crv_missing_arg() {
    assert!(validate_script_command("DENV.CRV").is_err());
}


#[test]
fn test_fbev_atk_valid() {
    assert!(validate_script_command("FBEV.ATK 300").is_ok());
}

#[test]
fn test_fbev_atk_missing_arg() {
    assert!(validate_script_command("FBEV.ATK").is_err());
}

#[test]
fn test_fbev_crv_valid() {
    assert!(validate_script_command("FBEV.CRV -6").is_ok());
}

#[test]
fn test_fbev_crv_missing_arg() {
    assert!(validate_script_command("FBEV.CRV").is_err());
}


#[test]
fn test_flev_atk_valid() {
    assert!(validate_script_command("FLEV.ATK 250").is_ok());
}

#[test]
fn test_flev_atk_missing_arg() {
    assert!(validate_script_command("FLEV.ATK").is_err());
}

#[test]
fn test_flev_crv_valid() {
    assert!(validate_script_command("FLEV.CRV 3").is_ok());
}

#[test]
fn test_flev_crv_missing_arg() {
    assert!(validate_script_command("FLEV.CRV").is_err());
}


#[test]
fn test_gate_valid() {
    assert!(validate_script_command("GATE 0").is_ok());
    assert!(validate_script_command("GATE 500").is_ok());
    assert!(validate_script_command("GATE 10000").is_ok());
}

#[test]
fn test_gate_missing_arg() {
    assert!(validate_script_command("GATE").is_err());
}

#[test]
fn test_gate_too_many_args() {
    assert!(validate_script_command("GATE 500 1000").is_err());
}

#[test]
fn test_aenv_gate_valid() {
    assert!(validate_script_command("AENV.GATE 100").is_ok());
    assert!(validate_script_command("AENV.GATE 0").is_ok());
    assert!(validate_script_command("AENV.GATE 10000").is_ok());
}

#[test]
fn test_aenv_gate_missing_arg() {
    assert!(validate_script_command("AENV.GATE").is_err());
}

#[test]
fn test_aenv_gate_too_many_args() {
    assert!(validate_script_command("AENV.GATE 100 200").is_err());
}

#[test]
fn test_penv_gate_valid() {
    assert!(validate_script_command("PENV.GATE 200").is_ok());
}

#[test]
fn test_penv_gate_missing_arg() {
    assert!(validate_script_command("PENV.GATE").is_err());
}

#[test]
fn test_fmev_gate_valid() {
    assert!(validate_script_command("FMEV.GATE 150").is_ok());
}

#[test]
fn test_fmev_gate_missing_arg() {
    assert!(validate_script_command("FMEV.GATE").is_err());
}

#[test]
fn test_denv_gate_valid() {
    assert!(validate_script_command("DENV.GATE 250").is_ok());
}

#[test]
fn test_denv_gate_missing_arg() {
    assert!(validate_script_command("DENV.GATE").is_err());
}

#[test]
fn test_fbev_gate_valid() {
    assert!(validate_script_command("FBEV.GATE 300").is_ok());
}

#[test]
fn test_fbev_gate_missing_arg() {
    assert!(validate_script_command("FBEV.GATE").is_err());
}

#[test]
fn test_flev_gate_valid() {
    assert!(validate_script_command("FLEV.GATE 175").is_ok());
}

#[test]
fn test_flev_gate_missing_arg() {
    assert!(validate_script_command("FLEV.GATE").is_err());
}

#[test]
fn test_all_envelope_commands_in_sequence() {
    assert!(validate_script_command("ENV.ATK 100; ENV.DEC 500; ENV.CRV 2").is_ok());
    assert!(validate_script_command("AENV.ATK 50; PENV.ATK 75; GATE 200").is_ok());
}

#[test]
fn test_aa_alias_valid() {
    assert!(validate_script_command("AA 100").is_ok());
}

#[test]
fn test_aa_alias_missing_arg() {
    assert!(validate_script_command("AA").is_err());
}

#[test]
fn test_paa_alias_valid() {
    assert!(validate_script_command("PAA 75").is_ok());
}

#[test]
fn test_faa_alias_valid() {
    assert!(validate_script_command("FAA 200").is_ok());
}

#[test]
fn test_daa_alias_valid() {
    assert!(validate_script_command("DAA 150").is_ok());
}

#[test]
fn test_fbaa_alias_valid() {
    assert!(validate_script_command("FBAA 300").is_ok());
}

#[test]
fn test_flaa_alias_valid() {
    assert!(validate_script_command("FLAA 250").is_ok());
}

#[test]
fn test_ac_alias_valid() {
    assert!(validate_script_command("AC 2").is_ok());
}

#[test]
fn test_pc_alias_valid() {
    assert!(validate_script_command("PC -3").is_ok());
}

#[test]
fn test_fbc_alias_valid() {
    assert!(validate_script_command("FBC -6").is_ok());
}

#[test]
fn test_flc_alias_valid() {
    assert!(validate_script_command("FLC 3").is_ok());
}
