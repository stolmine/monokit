use crate::commands::validate_script_command;
use crate::commands::resolve_alias;

#[test]
fn test_alias_resolution_posc_freq() {
    let result = resolve_alias("POSC.FREQ");
    assert_eq!(result, "PF");
}

#[test]
fn test_alias_resolution_filt_cut() {
    let result = resolve_alias("FILT.CUT");
    assert_eq!(result, "FC");
}

#[test]
fn test_alias_resolution_mosc_freq() {
    let result = resolve_alias("MOSC.FREQ");
    assert_eq!(result, "MF");
}

#[test]
fn test_alias_resolution_disc_amt() {
    let result = resolve_alias("DISC.AMT");
    assert_eq!(result, "DC");
}

#[test]
fn test_alias_resolution_unknown_command() {
    let result = resolve_alias("UNKNOWN.CMD");
    assert_eq!(result, "UNKNOWN.CMD");
}

#[test]
fn test_alias_resolution_short_commands_no_alias() {
    assert_eq!(resolve_alias("VOL"), "VOL");
    assert_eq!(resolve_alias("PAN"), "PAN");
    assert_eq!(resolve_alias("TR"), "TR");
}

#[test]
fn test_alias_resolution_reverb() {
    assert_eq!(resolve_alias("REV.DEC"), "RV");
    assert_eq!(resolve_alias("REV.WET"), "RW");
}

#[test]
fn test_alias_resolution_delay() {
    assert_eq!(resolve_alias("DLY.TIME"), "DT");
    assert_eq!(resolve_alias("DLY.FB"), "DF");
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
fn test_aenv_mode_valid() {
    assert!(validate_script_command("AENV.MODE 0").is_ok());
    assert!(validate_script_command("AENV.MODE 1").is_ok());
}

#[test]
fn test_aenv_mode_missing_arg() {
    assert!(validate_script_command("AENV.MODE").is_err());
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
fn test_penv_mode_valid() {
    assert!(validate_script_command("PENV.MODE 2").is_ok());
}

#[test]
fn test_penv_mode_missing_arg() {
    assert!(validate_script_command("PENV.MODE").is_err());
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
fn test_fmev_mode_valid() {
    assert!(validate_script_command("FMEV.MODE 1").is_ok());
}

#[test]
fn test_fmev_mode_missing_arg() {
    assert!(validate_script_command("FMEV.MODE").is_err());
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
fn test_denv_mode_valid() {
    assert!(validate_script_command("DENV.MODE 0").is_ok());
}

#[test]
fn test_denv_mode_missing_arg() {
    assert!(validate_script_command("DENV.MODE").is_err());
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
fn test_fbev_mode_valid() {
    assert!(validate_script_command("FBEV.MODE 2").is_ok());
}

#[test]
fn test_fbev_mode_missing_arg() {
    assert!(validate_script_command("FBEV.MODE").is_err());
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
fn test_flev_mode_valid() {
    assert!(validate_script_command("FLEV.MODE 1").is_ok());
}

#[test]
fn test_flev_mode_missing_arg() {
    assert!(validate_script_command("FLEV.MODE").is_err());
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
    assert!(validate_script_command("ENV.ATK 100; ENV.DEC 500; ENV.MODE 1").is_ok());
    assert!(validate_script_command("AENV.ATK 50; PENV.ATK 75; GATE 200").is_ok());
}
