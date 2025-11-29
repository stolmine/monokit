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
use crate::types::{MetroCommand, PatternStorage, ScriptStorage, Variables};
use anyhow::Result;
use std::sync::mpsc::Sender;

pub use validate::validate_script_command;

pub fn process_command<F>(
    metro_tx: &Sender<MetroCommand>,
    metro_interval: &mut u64,
    variables: &mut Variables,
    patterns: &mut PatternStorage,
    scripts: &mut ScriptStorage,
    script_index: usize,
    theme: &mut Theme,
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
            variables::handle_variable_a(&parts, variables, patterns, scripts, script_index, output);
        }
        "B" => {
            variables::handle_variable_b(&parts, variables, patterns, scripts, script_index, output);
        }
        "C" => {
            variables::handle_variable_c(&parts, variables, patterns, scripts, script_index, output);
        }
        "D" => {
            variables::handle_variable_d(&parts, variables, patterns, scripts, script_index, output);
        }
        "I" => {
            variables::handle_variable_i(&parts, variables, output);
        }
        "X" => {
            variables::handle_variable_x(&parts, variables, patterns, scripts, script_index, output);
        }
        "Y" => {
            variables::handle_variable_y(&parts, variables, patterns, scripts, script_index, output);
        }
        "Z" => {
            variables::handle_variable_z(&parts, variables, patterns, scripts, script_index, output);
        }
        "T" => {
            variables::handle_variable_t(&parts, variables, patterns, scripts, script_index, output);
        }
        "J" => {
            variables::handle_variable_j(&parts, variables, patterns, scripts, script_index, output)?;
        }
        "K" => {
            variables::handle_variable_k(&parts, variables, patterns, scripts, script_index, output)?;
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
            patterns::handle_pattern(&parts, variables, patterns, scripts, script_index, output)?;
        }
        "PN.L" => {
            patterns::handle_pn_l(&parts, variables, patterns, scripts, script_index, output)?;
        }
        "PN.I" => {
            patterns::handle_pn_i(&parts, variables, patterns, scripts, script_index, output)?;
        }
        "PN.HERE" => {
            patterns::handle_pn_here(&parts, variables, patterns, scripts, script_index, output)?;
        }
        "PN.NEXT" => {
            patterns::handle_pn_next(&parts, variables, patterns, scripts, script_index, output)?;
        }
        "PN.PREV" => {
            patterns::handle_pn_prev(&parts, variables, patterns, scripts, script_index, output)?;
        }
        "PN" => {
            patterns::handle_pn(&parts, variables, patterns, scripts, script_index, output)?;
        }
        "TR" => {
            misc::handle_tr(metro_tx, output)?;
        }
        "VOL" => {
            misc::handle_vol(&parts, metro_tx, output)?;
        }
        "M" => {
            metro_cmds::handle_m(&parts, metro_interval, metro_tx, output)?;
        }
        "M.BPM" => {
            metro_cmds::handle_m_bpm(&parts, metro_interval, metro_tx, output)?;
        }
        "M.ACT" => {
            metro_cmds::handle_m_act(&parts, metro_tx, output)?;
        }
        "M.SCRIPT" => {
            metro_cmds::handle_m_script(&parts, metro_tx, output)?;
        }
        "PF" => {
            synth_params::handle_pf(&parts, variables, patterns, scripts, script_index, metro_tx, output)?;
        }
        "PW" => {
            synth_params::handle_pw(&parts, variables, patterns, scripts, script_index, metro_tx, output)?;
        }
        "MF" => {
            synth_params::handle_mf(&parts, variables, patterns, scripts, script_index, metro_tx, output)?;
        }
        "MW" => {
            synth_params::handle_mw(&parts, variables, patterns, scripts, script_index, metro_tx, output)?;
        }
        "DC" => {
            synth_params::handle_dc(&parts, variables, patterns, scripts, script_index, metro_tx, output)?;
        }
        "DM" => {
            synth_params::handle_dm(&parts, variables, patterns, scripts, script_index, metro_tx, output)?;
        }
        "TK" => {
            synth_params::handle_tk(&parts, variables, patterns, scripts, script_index, metro_tx, output)?;
        }
        "MB" => {
            synth_params::handle_mb(&parts, variables, patterns, scripts, script_index, metro_tx, output)?;
        }
        "MP" => {
            synth_params::handle_mp(&parts, variables, patterns, scripts, script_index, metro_tx, output)?;
        }
        "MD" => {
            synth_params::handle_md(&parts, variables, patterns, scripts, script_index, metro_tx, output)?;
        }
        "MT" => {
            synth_params::handle_mt(&parts, variables, patterns, scripts, script_index, metro_tx, output)?;
        }
        "MA" => {
            synth_params::handle_ma(&parts, variables, patterns, scripts, script_index, metro_tx, output)?;
        }
        "FM" => {
            synth_params::handle_fm(&parts, variables, patterns, scripts, script_index, metro_tx, output)?;
        }
        "AD" => {
            synth_params::handle_ad(&parts, variables, patterns, scripts, script_index, metro_tx, output)?;
        }
        "PD" => {
            synth_params::handle_pd(&parts, variables, patterns, scripts, script_index, metro_tx, output)?;
        }
        "FD" => {
            synth_params::handle_fd(&parts, variables, patterns, scripts, script_index, metro_tx, output)?;
        }
        "PA" => {
            synth_params::handle_pa(&parts, variables, patterns, scripts, script_index, metro_tx, output)?;
        }
        "DD" => {
            synth_params::handle_dd(&parts, variables, patterns, scripts, script_index, metro_tx, output)?;
        }
        "MX" => {
            synth_params::handle_mx(&parts, variables, patterns, scripts, script_index, metro_tx, output)?;
        }
        "MM" => {
            synth_params::handle_mm(&parts, variables, patterns, scripts, script_index, metro_tx, output)?;
        }
        "ME" => {
            synth_params::handle_me(&parts, variables, patterns, scripts, script_index, metro_tx, output)?;
        }
        "FA" => {
            synth_params::handle_fa(&parts, variables, patterns, scripts, script_index, metro_tx, output)?;
        }
        "DA" => {
            synth_params::handle_da(&parts, variables, patterns, scripts, script_index, metro_tx, output)?;
        }
        "RST" => {
            misc::handle_rst(metro_tx, output)?;
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
            random_ops::handle_eith(&parts, variables, patterns, scripts, script_index, output);
        }
        "ADD" | "+" => {
            math_ops::handle_add(&parts, variables, patterns, scripts, script_index, output);
        }
        "SUB" | "-" => {
            math_ops::handle_sub(&parts, variables, patterns, scripts, script_index, output);
        }
        "MUL" | "*" => {
            math_ops::handle_mul(&parts, variables, patterns, scripts, script_index, output);
        }
        "DIV" | "/" => {
            math_ops::handle_div(&parts, variables, patterns, scripts, script_index, output);
        }
        "MOD" | "%" => {
            math_ops::handle_mod(&parts, variables, patterns, scripts, script_index, output);
        }
        "SCRIPT" => {
            return misc::handle_script(&parts);
        }
        "SAVE" => {
            scene_cmds::handle_save(&parts, scripts, patterns, output);
        }
        "LOAD" => {
            scene_cmds::handle_load(&parts, variables, scripts, patterns, output);
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
        _ => {
            output(format!("UNKNOWN COMMAND: {}", cmd));
        }
    }

    Ok(vec![])
}
