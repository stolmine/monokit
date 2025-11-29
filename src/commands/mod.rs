mod counters;
mod math_ops;
mod metro_cmds;
mod misc;
mod patterns;
mod random_ops;
mod scene_cmds;
mod synth_params;
pub mod validate;
mod variables;

use crate::theme::Theme;
use crate::types::{Counters, MetroCommand, PatternStorage, ScriptStorage, Variables};
use anyhow::Result;
use std::sync::mpsc::Sender;

pub use validate::validate_script_command;

pub fn process_command<F>(
    metro_tx: &Sender<MetroCommand>,
    metro_interval: &mut u64,
    br_len: &mut usize,
    variables: &mut Variables,
    patterns: &mut PatternStorage,
    counters: &mut Counters,
    scripts: &mut ScriptStorage,
    script_index: usize,
    theme: &mut Theme,
    debug_level: &mut u8,
    input: &str,
    mut output: F,
) -> Result<Vec<usize>>
where
    F: FnMut(String),
{
    let trimmed = input.trim();

    if trimmed.is_empty() {
        return Ok(vec![]);
    }

    let parts: Vec<&str> = trimmed.split_whitespace().collect();
    let cmd = parts[0].to_uppercase();

    match cmd.as_str() {
        "A" => {
            variables::handle_variable_a(&parts, variables, patterns, counters, scripts, script_index, output);
        }
        "B" => {
            variables::handle_variable_b(&parts, variables, patterns, counters, scripts, script_index, output);
        }
        "C" => {
            variables::handle_variable_c(&parts, variables, patterns, counters, scripts, script_index, output);
        }
        "D" => {
            variables::handle_variable_d(&parts, variables, patterns, counters, scripts, script_index, output);
        }
        "I" => {
            variables::handle_variable_i(&parts, variables, output);
        }
        "X" => {
            variables::handle_variable_x(&parts, variables, patterns, counters, scripts, script_index, output);
        }
        "Y" => {
            variables::handle_variable_y(&parts, variables, patterns, counters, scripts, script_index, output);
        }
        "Z" => {
            variables::handle_variable_z(&parts, variables, patterns, counters, scripts, script_index, output);
        }
        "T" => {
            variables::handle_variable_t(&parts, variables, patterns, counters, scripts, script_index, output);
        }
        "J" => {
            variables::handle_variable_j(&parts, variables, patterns, counters, scripts, script_index, output)?;
        }
        "K" => {
            variables::handle_variable_k(&parts, variables, patterns, counters, scripts, script_index, output)?;
        }
        "P.N" => {
            patterns::handle_pattern_n(&parts, patterns, output);
        }
        "P.L" => {
            patterns::handle_pattern_l(&parts, patterns, output);
        }
        "P.I" => {
            patterns::handle_pattern_i(&parts, patterns, output);
        }
        "P.HERE" => {
            patterns::handle_pattern_here(patterns, output);
        }
        "P.NEXT" => {
            patterns::handle_pattern_next(patterns, output);
        }
        "P.PREV" => {
            patterns::handle_pattern_prev(patterns, output);
        }
        "P" => {
            patterns::handle_pattern(&parts, variables, patterns, counters, scripts, script_index, output)?;
        }
        "PN.L" => {
            patterns::handle_pn_l(&parts, variables, patterns, counters, scripts, script_index, output)?;
        }
        "PN.I" => {
            patterns::handle_pn_i(&parts, variables, patterns, counters, scripts, script_index, output)?;
        }
        "PN.HERE" => {
            patterns::handle_pn_here(&parts, variables, patterns, counters, scripts, script_index, output)?;
        }
        "PN.NEXT" => {
            patterns::handle_pn_next(&parts, variables, patterns, counters, scripts, script_index, output)?;
        }
        "PN.PREV" => {
            patterns::handle_pn_prev(&parts, variables, patterns, counters, scripts, script_index, output)?;
        }
        "PN" => {
            patterns::handle_pn(&parts, variables, patterns, counters, scripts, script_index, output)?;
        }
        "TR" => {
            misc::handle_tr(metro_tx, *debug_level, output)?;
        }
        "VOL" => {
            misc::handle_vol(&parts, metro_tx, *debug_level, output)?;
        }
        "M" => {
            metro_cmds::handle_m(&parts, metro_interval, metro_tx, *debug_level, output)?;
        }
        "M.BPM" => {
            metro_cmds::handle_m_bpm(&parts, metro_interval, metro_tx, *debug_level, output)?;
        }
        "M.ACT" => {
            metro_cmds::handle_m_act(&parts, metro_tx, *debug_level, output)?;
        }
        "M.SCRIPT" => {
            metro_cmds::handle_m_script(&parts, metro_tx, *debug_level, output)?;
        }
        "PF" => {
            synth_params::handle_pf(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, output)?;
        }
        "PW" => {
            synth_params::handle_pw(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, output)?;
        }
        "MF" => {
            synth_params::handle_mf(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, output)?;
        }
        "MW" => {
            synth_params::handle_mw(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, output)?;
        }
        "DC" => {
            synth_params::handle_dc(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, output)?;
        }
        "DM" => {
            synth_params::handle_dm(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, output)?;
        }
        "TK" => {
            synth_params::handle_tk(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, output)?;
        }
        "MB" => {
            synth_params::handle_mb(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, output)?;
        }
        "MP" => {
            synth_params::handle_mp(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, output)?;
        }
        "MD" => {
            synth_params::handle_md(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, output)?;
        }
        "MT" => {
            synth_params::handle_mt(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, output)?;
        }
        "MA" => {
            synth_params::handle_ma(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, output)?;
        }
        "FM" => {
            synth_params::handle_fm(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, output)?;
        }
        "AD" => {
            synth_params::handle_ad(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, output)?;
        }
        "PD" => {
            synth_params::handle_pd(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, output)?;
        }
        "FD" => {
            synth_params::handle_fd(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, output)?;
        }
        "PA" => {
            synth_params::handle_pa(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, output)?;
        }
        "DD" => {
            synth_params::handle_dd(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, output)?;
        }
        "MX" => {
            synth_params::handle_mx(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, output)?;
        }
        "MM" => {
            synth_params::handle_mm(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, output)?;
        }
        "ME" => {
            synth_params::handle_me(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, output)?;
        }
        "FA" => {
            synth_params::handle_fa(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, output)?;
        }
        "DA" => {
            synth_params::handle_da(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, output)?;
        }
        "FB" => {
            synth_params::handle_fb(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, output)?;
        }
        "FBA" => {
            synth_params::handle_fba(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, output)?;
        }
        "FBD" => {
            synth_params::handle_fbd(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, output)?;
        }
        "RF" => {
            synth_params::handle_rf(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, output)?;
        }
        "RD" => {
            synth_params::handle_rd(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, output)?;
        }
        "RM" => {
            synth_params::handle_rm(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, output)?;
        }
        "RK" => {
            synth_params::handle_rk(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, output)?;
        }
        "DT" => {
            synth_params::handle_dt(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, output)?;
        }
        "DF" => {
            synth_params::handle_df(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, output)?;
        }
        "DLP" => {
            synth_params::handle_dlp(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, output)?;
        }
        "DW" => {
            synth_params::handle_dw(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, output)?;
        }
        "DS" => {
            synth_params::handle_ds(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, output)?;
        }
        "RV" => {
            synth_params::handle_rv(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, output)?;
        }
        "RP" => {
            synth_params::handle_rp(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, output)?;
        }
        "RH" => {
            synth_params::handle_rh(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, output)?;
        }
        "RW" => {
            synth_params::handle_rw(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, output)?;
        }
        "D.MODE" => {
            synth_params::handle_d_mode(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, output)?;
        }
        "D.TAIL" => {
            synth_params::handle_d_tail(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, output)?;
        }
        "R.MODE" => {
            synth_params::handle_r_mode(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, output)?;
        }
        "R.TAIL" => {
            synth_params::handle_r_tail(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, output)?;
        }
        "FC" => {
            synth_params::handle_fc(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, output)?;
        }
        "FQ" => {
            synth_params::handle_fq(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, output)?;
        }
        "FT" => {
            synth_params::handle_ft(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, output)?;
        }
        "FE" => {
            synth_params::handle_fe(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, output)?;
        }
        "FED" => {
            synth_params::handle_fed(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, output)?;
        }
        "FK" => {
            synth_params::handle_fk(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, output)?;
        }
        "MF.F" => {
            synth_params::handle_mf_f(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, output)?;
        }
        "LB" => {
            synth_params::handle_lb(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, output)?;
        }
        "LS" => {
            synth_params::handle_ls(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, output)?;
        }
        "LM" => {
            synth_params::handle_lm(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, output)?;
        }
        "RGF" => {
            synth_params::handle_rgf(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, output)?;
        }
        "RGW" => {
            synth_params::handle_rgw(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, output)?;
        }
        "RGM" => {
            synth_params::handle_rgm(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, output)?;
        }
        "CT" => {
            synth_params::handle_ct(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, output)?;
        }
        "CR" => {
            synth_params::handle_cr(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, output)?;
        }
        "CA" => {
            synth_params::handle_ca(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, output)?;
        }
        "CL" => {
            synth_params::handle_cl(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, output)?;
        }
        "CM" => {
            synth_params::handle_cm(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, output)?;
        }
        "EL" => {
            synth_params::handle_el(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, output)?;
        }
        "EM" => {
            synth_params::handle_em(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, output)?;
        }
        "EF" => {
            synth_params::handle_ef(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, output)?;
        }
        "EQ" => {
            synth_params::handle_eq_param(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, output)?;
        }
        "EH" => {
            synth_params::handle_eh(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, output)?;
        }
        "PAN" => {
            synth_params::handle_pan(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, output)?;
        }
        "BR.ACT" => {
            synth_params::handle_br_act(&parts, *metro_interval, br_len, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, output)?;
        }
        "BR.LEN" => {
            synth_params::handle_br_len(&parts, *metro_interval, br_len, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, output)?;
        }
        "BR.REV" => {
            synth_params::handle_br_rev(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, output)?;
        }
        "BR.WIN" => {
            synth_params::handle_br_win(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, output)?;
        }
        "BR.MIX" => {
            synth_params::handle_br_mix(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, output)?;
        }
        "PS.MODE" => {
            synth_params::handle_ps_mode(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, output)?;
        }
        "PS.SEMI" => {
            synth_params::handle_ps_semi(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, output)?;
        }
        "PS.GRAIN" => {
            synth_params::handle_ps_grain(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, output)?;
        }
        "PS.MIX" => {
            synth_params::handle_ps_mix(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, output)?;
        }
        "PS.TARG" => {
            synth_params::handle_ps_targ(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, output)?;
        }
        "RST" => {
            misc::handle_rst(metro_tx, *debug_level, output)?;
        }
        "RND" => {
            random_ops::handle_rnd(&parts, output)?;
        }
        "RRND" => {
            random_ops::handle_rrnd(&parts, output)?;
        }
        "TOSS" => {
            random_ops::handle_toss(output);
        }
        "EITH" => {
            random_ops::handle_eith(&parts, variables, patterns, counters, scripts, script_index, output);
        }
        "TOG" => {
            random_ops::handle_tog(&parts, variables, patterns, counters, scripts, script_index, output);
        }
        "ADD" | "+" => {
            math_ops::handle_add(&parts, variables, patterns, counters, scripts, script_index, output);
        }
        "SUB" | "-" => {
            math_ops::handle_sub(&parts, variables, patterns, counters, scripts, script_index, output);
        }
        "MUL" | "*" => {
            math_ops::handle_mul(&parts, variables, patterns, counters, scripts, script_index, output);
        }
        "DIV" | "/" => {
            math_ops::handle_div(&parts, variables, patterns, counters, scripts, script_index, output);
        }
        "MOD" | "%" => {
            math_ops::handle_mod(&parts, variables, patterns, counters, scripts, script_index, output);
        }
        "MAP" => {
            math_ops::handle_map(&parts, variables, patterns, counters, scripts, script_index, output);
        }
        "SCRIPT" => {
            return misc::handle_script(&parts, variables, patterns, counters, scripts, script_index);
        }
        "SAVE" => {
            scene_cmds::handle_save(&parts, scripts, patterns, output);
        }
        "LOAD" => {
            if scene_cmds::handle_load(&parts, variables, scripts, patterns, output) {
                return Ok(vec![9]);
            }
        }
        "SCENES" => {
            scene_cmds::handle_scenes(output);
        }
        "DELETE" => {
            scene_cmds::handle_delete(&parts, output);
        }
        "THEME" => {
            misc::handle_theme(&parts, theme, output);
        }
        "HELP" => {
            misc::handle_help(output);
        }
        "REC" => {
            misc::handle_rec(metro_tx, *debug_level, output)?;
        }
        "REC.STOP" => {
            misc::handle_rec_stop(metro_tx, *debug_level, output)?;
        }
        "REC.PATH" => {
            misc::handle_rec_path(&parts, metro_tx, *debug_level, output)?;
        }
        "PRINT" => {
            misc::handle_print(&parts, variables, patterns, counters, scripts, script_index, *debug_level, output);
        }
        "DEBUG" => {
            misc::handle_debug(&parts, debug_level, output);
        }
        "N1.RST" => {
            counters::handle_n1_rst(counters, output);
        }
        "N2.RST" => {
            counters::handle_n2_rst(counters, output);
        }
        "N3.RST" => {
            counters::handle_n3_rst(counters, output);
        }
        "N4.RST" => {
            counters::handle_n4_rst(counters, output);
        }
        "N1.MAX" => {
            counters::handle_n1_max(&parts, variables, patterns, counters, scripts, script_index, output);
        }
        "N2.MAX" => {
            counters::handle_n2_max(&parts, variables, patterns, counters, scripts, script_index, output);
        }
        "N3.MAX" => {
            counters::handle_n3_max(&parts, variables, patterns, counters, scripts, script_index, output);
        }
        "N4.MAX" => {
            counters::handle_n4_max(&parts, variables, patterns, counters, scripts, script_index, output);
        }
        _ => {
            output(format!("UNKNOWN COMMAND: {}", cmd));
        }
    }

    Ok(vec![])
}
