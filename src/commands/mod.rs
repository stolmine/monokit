mod aliases;
pub mod common;
mod core;
mod gate;
mod patterns;
pub mod randomization;
pub mod slew;
mod synth;
mod system;
pub mod validate;

// Re-export from core module
use core::{counters, math_ops, random_ops, variables};
pub use core::scale;
pub use core::scheduling as delay;

// Re-export from system module
use system::{metro as metro_cmds, midi as midi_cmds, misc, preset as preset_cmds, scene as scene_cmds, sc as sc_cmds};

// Re-export from synth module
use synth as synth_params;

use crate::config;
use crate::midi::{MidiConnection, MidiTimingStats};
use crate::theme::Theme;
use crate::types::{Counters, MetroCommand, PatternStorage, ScaleState, ScriptStorage, SyncMode, Variables};
use anyhow::Result;
use std::sync::{mpsc::Sender, Arc};

pub use aliases::resolve_alias;
pub use validate::validate_script_command;

pub fn process_command<F>(
    metro_tx: &Sender<MetroCommand>,
    metro_interval: &mut u64,
    br_len: &mut usize,
    sync_mode: &mut SyncMode,
    midi_connection: &mut Option<MidiConnection>,
    midi_timing_stats: &Arc<MidiTimingStats>,
    variables: &mut Variables,
    patterns: &mut PatternStorage,
    counters: &mut Counters,
    scripts: &mut ScriptStorage,
    script_index: usize,
    scale: &mut ScaleState,
    theme: &mut Theme,
    debug_level: &mut u8,
    activity_hold_ms: &mut f32,
    show_cpu: &mut bool,
    show_bpm: &mut bool,
    header_level: &mut u8,
    limiter_enabled: &mut bool,
    notes: &mut crate::types::NotesStorage,
    load_rst: &mut bool,
    show_conditional_highlight: &mut bool,
    scope_timespan_ms: &mut u32,
    scope_color_mode: &mut u8,
    scope_display_mode: &mut u8,
    scope_unipolar: &mut bool,
    show_meters_header: &mut bool,
    show_meters_grid: &mut bool,
    show_spectrum: &mut bool,
    show_activity: &mut bool,
    show_grid: &mut bool,
    show_grid_view: &mut bool,
    show_seq_highlight: &mut bool,
    grid_mode: &mut u8,
    current_scene_name: &mut Option<String>,
    title_mode: &mut u8,
    out_err: &mut bool,
    out_ess: &mut bool,
    out_qry: &mut bool,
    out_cfm: &mut bool,
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
    let cmd = resolve_alias(&cmd);

    match cmd.as_str() {
        "A" => {
            variables::handle_variable_a(&parts, variables, patterns, counters, scripts, script_index, scale, *debug_level, *out_qry, *out_cfm, output);
        }
        "B" => {
            variables::handle_variable_b(&parts, variables, patterns, counters, scripts, script_index, scale, *debug_level, *out_qry, *out_cfm, output);
        }
        "C" => {
            variables::handle_variable_c(&parts, variables, patterns, counters, scripts, script_index, scale, *debug_level, *out_qry, *out_cfm, output);
        }
        "D" => {
            variables::handle_variable_d(&parts, variables, patterns, counters, scripts, script_index, scale, *debug_level, *out_qry, *out_cfm, output);
        }
        "I" => {
            variables::handle_variable_i(&parts, variables, *debug_level, *out_err, *out_qry, *out_cfm, output);
        }
        "X" => {
            variables::handle_variable_x(&parts, variables, patterns, counters, scripts, script_index, scale, *debug_level, *out_qry, *out_cfm, output);
        }
        "Y" => {
            variables::handle_variable_y(&parts, variables, patterns, counters, scripts, script_index, scale, *debug_level, *out_qry, *out_cfm, output);
        }
        "Z" => {
            variables::handle_variable_z(&parts, variables, patterns, counters, scripts, script_index, scale, *debug_level, *out_qry, *out_cfm, output);
        }
        "T" => {
            variables::handle_variable_t(&parts, variables, patterns, counters, scripts, script_index, scale, *debug_level, *out_qry, *out_cfm, output);
        }
        "J" => {
            variables::handle_variable_j(&parts, variables, patterns, counters, scripts, script_index, scale, *debug_level, *out_err, *out_qry, *out_cfm, output)?;
        }
        "K" => {
            variables::handle_variable_k(&parts, variables, patterns, counters, scripts, script_index, scale, *debug_level, *out_err, *out_qry, *out_cfm, output)?;
        }
        "P.N" => {
            patterns::handle_pattern_n(&parts, patterns, *debug_level, *out_err, *out_qry, *out_cfm, output);
        }
        "P.L" => {
            patterns::handle_pattern_l(&parts, patterns, *debug_level, *out_err, *out_qry, *out_cfm, output);
        }
        "P.I" => {
            patterns::handle_pattern_i(&parts, patterns, *debug_level, *out_err, *out_qry, *out_cfm, output);
        }
        "P.HERE" => {
            patterns::handle_pattern_here(patterns, *debug_level, *out_err, *out_qry, *out_cfm, output);
        }
        "P.NEXT" => {
            patterns::handle_pattern_next(patterns, *debug_level, *out_err, *out_qry, *out_cfm, output);
        }
        "P.PREV" => {
            patterns::handle_pattern_prev(patterns, *debug_level, *out_err, *out_qry, *out_cfm, output);
        }
        "P.PUSH" => {
            patterns::handle_pattern_push(&parts, variables, patterns, counters, scripts, script_index, scale, *debug_level, *out_err, *out_qry, *out_cfm, output)?;
        }
        "P.POP" => {
            patterns::handle_pattern_pop(patterns, *debug_level, *out_err, *out_qry, *out_cfm, output);
        }
        "P.INS" => {
            patterns::handle_pattern_ins(&parts, variables, patterns, counters, scripts, script_index, scale, *debug_level, *out_err, *out_qry, *out_cfm, output)?;
        }
        "P.RM" => {
            patterns::handle_pattern_rm(&parts, variables, patterns, counters, scripts, script_index, scale, *debug_level, *out_err, *out_qry, *out_cfm, output)?;
        }
        "P.REV" => {
            patterns::handle_pattern_rev(patterns, *debug_level, *out_err, *out_qry, *out_cfm, output);
        }
        "P.ROT" => {
            patterns::handle_pattern_rot(&parts, variables, patterns, counters, scripts, script_index, scale, *debug_level, *out_err, *out_qry, *out_cfm, output)?;
        }
        "P.SHUF" => {
            patterns::handle_pattern_shuf(patterns, *debug_level, *out_err, *out_qry, *out_cfm, output);
        }
        "P.SORT" => {
            patterns::handle_pattern_sort(patterns, *debug_level, *out_err, *out_qry, *out_cfm, output);
        }
        "P.RND" => {
            patterns::handle_pattern_rnd(&parts, variables, patterns, counters, scripts, script_index, scale, *debug_level, *out_err, *out_qry, *out_cfm, output)?;
        }
        "P.ADD" => {
            patterns::handle_pattern_add(&parts, variables, patterns, counters, scripts, script_index, scale, *debug_level, *out_err, *out_qry, *out_cfm, output)?;
        }
        "P.SUB" => {
            patterns::handle_pattern_sub(&parts, variables, patterns, counters, scripts, script_index, scale, *debug_level, *out_err, *out_qry, *out_cfm, output)?;
        }
        "P.MUL" => {
            patterns::handle_pattern_mul(&parts, variables, patterns, counters, scripts, script_index, scale, *debug_level, *out_err, *out_qry, *out_cfm, output)?;
        }
        "P.DIV" => {
            patterns::handle_pattern_div(&parts, variables, patterns, counters, scripts, script_index, scale, *debug_level, *out_err, *out_qry, *out_cfm, output)?;
        }
        "P.MOD" => {
            patterns::handle_pattern_mod(&parts, variables, patterns, counters, scripts, script_index, scale, *debug_level, *out_err, *out_qry, *out_cfm, output)?;
        }
        "P.SCALE" => {
            patterns::handle_pattern_scale(&parts, variables, patterns, counters, scripts, script_index, scale, *debug_level, *out_err, *out_qry, *out_cfm, output)?;
        }
        "P.MIN" => {
            patterns::handle_pattern_min(patterns, *debug_level, *out_err, *out_qry, *out_cfm, output);
        }
        "P.MAX" => {
            patterns::handle_pattern_max(patterns, *debug_level, *out_err, *out_qry, *out_cfm, output);
        }
        "P.SUM" => {
            patterns::handle_pattern_sum(patterns, *debug_level, *out_err, *out_qry, *out_cfm, output);
        }
        "P.AVG" => {
            patterns::handle_pattern_avg(patterns, *debug_level, *out_err, *out_qry, *out_cfm, output);
        }
        "P.FND" => {
            patterns::handle_pattern_fnd(&parts, variables, patterns, counters, scripts, script_index, scale, *debug_level, *out_err, *out_qry, *out_cfm, output)?;
        }
        "P" => {
            patterns::handle_pattern(&parts, variables, patterns, counters, scripts, script_index, scale, *debug_level, *out_err, *out_qry, *out_cfm, output)?;
        }
        "PN.L" => {
            patterns::handle_pn_l(&parts, variables, patterns, counters, scripts, script_index, scale, *debug_level, *out_err, *out_qry, *out_cfm, output)?;
        }
        "PN.I" => {
            patterns::handle_pn_i(&parts, variables, patterns, counters, scripts, script_index, scale, *debug_level, *out_err, *out_qry, *out_cfm, output)?;
        }
        "PN.HERE" => {
            patterns::handle_pn_here(&parts, variables, patterns, counters, scripts, script_index, scale, *debug_level, *out_err, *out_qry, *out_cfm, output)?;
        }
        "PN.NEXT" => {
            patterns::handle_pn_next(&parts, variables, patterns, counters, scripts, script_index, scale, *debug_level, *out_err, *out_qry, *out_cfm, output)?;
        }
        "PN.PREV" => {
            patterns::handle_pn_prev(&parts, variables, patterns, counters, scripts, script_index, scale, *debug_level, *out_err, *out_qry, *out_cfm, output)?;
        }
        "PN.PUSH" => {
            patterns::handle_pn_push(&parts, variables, patterns, counters, scripts, script_index, scale, *debug_level, *out_err, *out_qry, *out_cfm, output)?;
        }
        "PN.POP" => {
            patterns::handle_pn_pop(&parts, variables, patterns, counters, scripts, script_index, scale, *debug_level, *out_err, *out_qry, *out_cfm, output)?;
        }
        "PN.INS" => {
            patterns::handle_pn_ins(&parts, variables, patterns, counters, scripts, script_index, scale, *debug_level, *out_err, *out_qry, *out_cfm, output)?;
        }
        "PN.RM" => {
            patterns::handle_pn_rm(&parts, variables, patterns, counters, scripts, script_index, scale, *debug_level, *out_err, *out_qry, *out_cfm, output)?;
        }
        "PN.REV" => {
            patterns::handle_pn_rev(&parts, variables, patterns, counters, scripts, script_index, scale, *debug_level, *out_err, *out_qry, *out_cfm, output)?;
        }
        "PN.ROT" => {
            patterns::handle_pn_rot(&parts, variables, patterns, counters, scripts, script_index, scale, *debug_level, *out_err, *out_qry, *out_cfm, output)?;
        }
        "PN.SHUF" => {
            patterns::handle_pn_shuf(&parts, variables, patterns, counters, scripts, script_index, scale, *debug_level, *out_err, *out_qry, *out_cfm, output)?;
        }
        "PN.SORT" => {
            patterns::handle_pn_sort(&parts, variables, patterns, counters, scripts, script_index, scale, *debug_level, *out_err, *out_qry, *out_cfm, output)?;
        }
        "PN.RND" => {
            patterns::handle_pn_rnd(&parts, variables, patterns, counters, scripts, script_index, scale, *debug_level, *out_err, *out_qry, *out_cfm, output)?;
        }
        "PN.ADD" => {
            patterns::handle_pn_add(&parts, variables, patterns, counters, scripts, script_index, scale, *debug_level, *out_err, *out_qry, *out_cfm, output)?;
        }
        "PN.SUB" => {
            patterns::handle_pn_sub(&parts, variables, patterns, counters, scripts, script_index, scale, *debug_level, *out_err, *out_qry, *out_cfm, output)?;
        }
        "PN.MUL" => {
            patterns::handle_pn_mul(&parts, variables, patterns, counters, scripts, script_index, scale, *debug_level, *out_err, *out_qry, *out_cfm, output)?;
        }
        "PN.DIV" => {
            patterns::handle_pn_div(&parts, variables, patterns, counters, scripts, script_index, scale, *debug_level, *out_err, *out_qry, *out_cfm, output)?;
        }
        "PN.MOD" => {
            patterns::handle_pn_mod(&parts, variables, patterns, counters, scripts, script_index, scale, *debug_level, *out_err, *out_qry, *out_cfm, output)?;
        }
        "PN.SCALE" => {
            patterns::handle_pn_scale(&parts, variables, patterns, counters, scripts, script_index, scale, *debug_level, *out_err, *out_qry, *out_cfm, output)?;
        }
        "PN.MIN" => {
            patterns::handle_pn_min(&parts, variables, patterns, counters, scripts, script_index, scale, *debug_level, *out_err, *out_qry, *out_cfm, output)?;
        }
        "PN.MAX" => {
            patterns::handle_pn_max(&parts, variables, patterns, counters, scripts, script_index, scale, *debug_level, *out_err, *out_qry, *out_cfm, output)?;
        }
        "PN.SUM" => {
            patterns::handle_pn_sum(&parts, variables, patterns, counters, scripts, script_index, scale, *debug_level, *out_err, *out_qry, *out_cfm, output)?;
        }
        "PN.AVG" => {
            patterns::handle_pn_avg(&parts, variables, patterns, counters, scripts, script_index, scale, *debug_level, *out_err, *out_qry, *out_cfm, output)?;
        }
        "PN.FND" => {
            patterns::handle_pn_fnd(&parts, variables, patterns, counters, scripts, script_index, scale, *debug_level, *out_err, *out_qry, *out_cfm, output)?;
        }
        "PN" => {
            patterns::handle_pn(&parts, variables, patterns, counters, scripts, script_index, scale, *debug_level, *out_err, *out_qry, *out_cfm, output)?;
        }
        "TR" => {
            misc::handle_tr(metro_tx, *debug_level, *out_cfm, output)?;
        }
        "VOL" => {
            misc::handle_vol(&parts, metro_tx, *debug_level, *out_cfm, output)?;
        }
        "M" => {
            metro_cmds::handle_m(&parts, metro_interval, metro_tx, *debug_level, *out_qry, *out_ess, output)?;
        }
        "M.BPM" => {
            metro_cmds::handle_m_bpm(&parts, metro_interval, metro_tx, *debug_level, *out_ess, output)?;
        }
        "M.ACT" => {
            metro_cmds::handle_m_act(&parts, metro_tx, *debug_level, *out_ess, output)?;
        }
        "M.SCRIPT" => {
            metro_cmds::handle_m_script(&parts, metro_tx, *debug_level, *out_ess, output)?;
        }
        "M.SYNC" => {
            metro_cmds::handle_m_sync(&parts, sync_mode, metro_tx, *debug_level, *out_qry, *out_ess, output)?;
        }
        "MIDI.IN" | "MIDI" => {
            midi_cmds::handle_midi_in(&parts, metro_tx, midi_connection, midi_timing_stats, output)?;
        }
        "MIDI.DIAG" => {
            midi_cmds::handle_midi_diag(&parts, metro_tx, midi_timing_stats, output)?;
        }
        "SC.DIAG" => {
            sc_cmds::handle_sc_diag(&parts, metro_tx, output)?;
        }
        "PF" => {
            synth_params::handle_pf(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "PW" => {
            synth_params::handle_pw(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "MF" => {
            synth_params::handle_mf(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "MW" => {
            synth_params::handle_mw(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "DC" => {
            synth_params::handle_dc(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, output)?;
        }
        "DM" => {
            synth_params::handle_dm(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "TK" => {
            synth_params::handle_tk(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "MB" => {
            synth_params::handle_mb(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "MP" => {
            synth_params::handle_mp(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "MD" => {
            synth_params::handle_md(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "MT" => {
            synth_params::handle_mt(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "MA" => {
            synth_params::handle_ma(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "FM" => {
            synth_params::handle_fm(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "AD" => {
            synth_params::handle_ad(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, output)?;
        }
        "PD" => {
            synth_params::handle_pd(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, output)?;
        }
        "FD" => {
            synth_params::handle_fd(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, output)?;
        }
        "PA" => {
            synth_params::handle_pa(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, output)?;
        }
        "DD" => {
            synth_params::handle_dd(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "MX" => {
            synth_params::handle_mx(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "MM" => {
            synth_params::handle_mm(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "ME" => {
            synth_params::handle_me(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "FA" => {
            synth_params::handle_fa(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, output)?;
        }
        "DA" => {
            synth_params::handle_da(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, output)?;
        }
        "FB" => {
            synth_params::handle_fb(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "FBA" => {
            synth_params::handle_fba(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "FBD" => {
            synth_params::handle_fbd(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "RF" => {
            synth_params::handle_rf(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "RD" => {
            synth_params::handle_rd(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "RM" => {
            synth_params::handle_rm(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "RK" => {
            synth_params::handle_rk(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "DT" => {
            synth_params::handle_dt(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "DF" => {
            synth_params::handle_df(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "DLP" => {
            synth_params::handle_dlp(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "DW" => {
            synth_params::handle_dw(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "DS" => {
            synth_params::handle_ds(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "RV" => {
            synth_params::handle_rv(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "RP" => {
            synth_params::handle_rp(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "RH" => {
            synth_params::handle_rh(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "RW" => {
            synth_params::handle_rw(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "D.MODE" => {
            synth_params::handle_d_mode(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "D.TAIL" => {
            synth_params::handle_d_tail(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "R.MODE" => {
            synth_params::handle_r_mode(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "R.TAIL" => {
            synth_params::handle_r_tail(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "FC" => {
            synth_params::handle_fc(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "FQ" => {
            synth_params::handle_fq(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "FT" => {
            synth_params::handle_ft(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "FE" => {
            synth_params::handle_fe(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "FED" => {
            synth_params::handle_fed(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "FK" => {
            synth_params::handle_fk(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "MF.F" => {
            synth_params::handle_mf_f(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "LB" => {
            synth_params::handle_lb(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "LS" => {
            synth_params::handle_ls(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "LM" => {
            synth_params::handle_lm(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "RGF" => {
            synth_params::handle_rgf(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "RGW" => {
            synth_params::handle_rgw(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "RGM" => {
            synth_params::handle_rgm(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "CT" => {
            synth_params::handle_ct(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "CR" => {
            synth_params::handle_cr(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "CA" => {
            synth_params::handle_ca(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "CL" => {
            synth_params::handle_cl(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "CM" => {
            synth_params::handle_cm(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "EL" => {
            synth_params::handle_el(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "EM" => {
            synth_params::handle_em(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "EF" => {
            synth_params::handle_ef(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "EQ" => {
            synth_params::handle_eq_param(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "EH" => {
            synth_params::handle_eh(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "PAN" => {
            synth_params::handle_pan(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, output)?;
        }
        "BR.ACT" => {
            synth_params::handle_br_act(&parts, *metro_interval, br_len, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "BR.LEN" => {
            synth_params::handle_br_len(&parts, *metro_interval, br_len, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "BR.REV" => {
            synth_params::handle_br_rev(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "BR.WIN" => {
            synth_params::handle_br_win(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "BR.MIX" => {
            synth_params::handle_br_mix(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "PS.MODE" => {
            synth_params::handle_ps_mode(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "PS.SEMI" => {
            synth_params::handle_ps_semi(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "PS.GRAIN" => {
            synth_params::handle_ps_grain(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "PS.MIX" => {
            synth_params::handle_ps_mix(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "PS.TARG" => {
            synth_params::handle_ps_targ(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "SLEW" => {
            slew::handle_slew(&parts, variables, patterns, counters, scripts, script_index, scale, metro_tx, *debug_level, output)?;
        }
        "SLEW.ALL" => {
            slew::handle_slew_all(&parts, variables, patterns, counters, scripts, script_index, scale, metro_tx, *debug_level, output)?;
        }
        "AENV.ATK" | "AA" => {
            synth_params::handle_aenv_atk(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, output)?;
        }
        "AENV.CRV" | "AC" => {
            synth_params::handle_aenv_crv(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, output)?;
        }
        "PENV.ATK" | "PAA" => {
            synth_params::handle_penv_atk(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, output)?;
        }
        "PENV.CRV" | "PC" => {
            synth_params::handle_penv_crv(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, output)?;
        }
        "FMEV.ATK" | "FAA" => {
            synth_params::handle_fmev_atk(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, output)?;
        }
        "FMEV.CRV" => {
            synth_params::handle_fmev_crv(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, output)?;
        }
        "DENV.ATK" | "DAA" => {
            synth_params::handle_denv_atk(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, output)?;
        }
        "DENV.CRV" => {
            synth_params::handle_denv_crv(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, output)?;
        }
        "FBEV.ATK" | "FBAA" => {
            synth_params::handle_fbev_atk(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, output)?;
        }
        "FBEV.CRV" | "FBC" => {
            synth_params::handle_fbev_crv(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, output)?;
        }
        "FBEV.AMT" => {
            synth_params::handle_fba(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "FLEV.ATK" | "FLAA" => {
            synth_params::handle_flev_atk(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, output)?;
        }
        "FLEV.CRV" | "FLC" => {
            synth_params::handle_flev_crv(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, output)?;
        }
        "RST" => {
            misc::handle_rst(metro_tx, *debug_level, *out_ess, output)?;
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
            random_ops::handle_eith(&parts, variables, patterns, counters, scripts, script_index, scale, output);
        }
        "TOG" => {
            random_ops::handle_tog(&parts, variables, patterns, counters, scripts, script_index, scale, output);
        }
        "RND.VOICE" => {
            randomization::handle_rnd_voice(metro_tx, *debug_level, output)?;
        }
        "RND.OSC" => {
            randomization::handle_rnd_osc(metro_tx, *debug_level, output)?;
        }
        "RND.FM" => {
            randomization::handle_rnd_fm(metro_tx, *debug_level, output)?;
        }
        "RND.MOD" => {
            randomization::handle_rnd_mod(metro_tx, *debug_level, output)?;
        }
        "RND.ENV" => {
            randomization::handle_rnd_env(metro_tx, *debug_level, output)?;
        }
        "RND.P" => {
            randomization::handle_rnd_p(&parts, variables, patterns, counters, scripts, script_index, scale, *debug_level, output)?;
        }
        "RND.PN" => {
            randomization::handle_rnd_pn(&parts, variables, patterns, counters, scripts, script_index, scale, *debug_level, output)?;
        }
        "RND.PALL" => {
            randomization::handle_rnd_pall(&parts, variables, patterns, counters, scripts, script_index, scale, *debug_level, output)?;
        }
        "RND.FX" => {
            randomization::handle_rnd_fx(metro_tx, *debug_level, output)?;
        }
        "RND.FILT" => {
            randomization::handle_rnd_filt(metro_tx, *debug_level, output)?;
        }
        "RND.DLY" => {
            randomization::handle_rnd_dly(metro_tx, *debug_level, output)?;
        }
        "RND.VERB" => {
            randomization::handle_rnd_verb(metro_tx, *debug_level, output)?;
        }
        "ADD" | "+" => {
            math_ops::handle_add(&parts, variables, patterns, counters, scripts, script_index, scale, output);
        }
        "SUB" | "-" => {
            math_ops::handle_sub(&parts, variables, patterns, counters, scripts, script_index, scale, output);
        }
        "MUL" | "*" => {
            math_ops::handle_mul(&parts, variables, patterns, counters, scripts, script_index, scale, output);
        }
        "DIV" | "/" => {
            math_ops::handle_div(&parts, variables, patterns, counters, scripts, script_index, scale, output);
        }
        "MOD" | "%" => {
            math_ops::handle_mod(&parts, variables, patterns, counters, scripts, script_index, scale, output);
        }
        "MAP" => {
            math_ops::handle_map(&parts, variables, patterns, counters, scripts, script_index, scale, output);
        }
        "SCRIPT" | "$" => {
            return misc::handle_script(&parts, variables, patterns, counters, scripts, script_index, scale);
        }
        "SAVE" => {
            scene_cmds::handle_save(&parts, scripts, patterns, notes, current_scene_name, *debug_level, *out_ess, output);
        }
        "LOAD" => {
            if *load_rst {
                misc::handle_rst(metro_tx, *debug_level, *out_ess, &mut |_| {})?;
            }
            if scene_cmds::handle_load(&parts, variables, scripts, patterns, notes, current_scene_name, *debug_level, *out_ess, output) {
                return Ok(vec![9]);
            }
        }
        "LOAD.RST" => {
            misc::handle_load_rst(&parts, load_rst, output);
        }
        "SCENES" => {
            scene_cmds::handle_scenes(*debug_level, *out_qry, output);
        }
        "DELETE" => {
            scene_cmds::handle_delete(&parts, *debug_level, *out_ess, output);
        }
        "PSET" => {
            preset_cmds::handle_pset(&parts, scripts, *debug_level, *out_ess, output);
        }
        "PSET.SAVE" => {
            preset_cmds::handle_pset_save(&parts, scripts, *debug_level, *out_ess, output);
        }
        "PSET.DEL" => {
            preset_cmds::handle_pset_del(&parts, *debug_level, *out_ess, output);
        }
        "PSETS" => {
            preset_cmds::handle_psets(*debug_level, *out_qry, output);
        }
        "THEME" => {
            misc::handle_theme(&parts, theme, output);
        }
        "HELP" => {
            misc::handle_help(output);
        }
        "REC" => {
            misc::handle_rec(metro_tx, *debug_level, *out_ess, output)?;
        }
        "REC.STOP" => {
            misc::handle_rec_stop(metro_tx, *debug_level, *out_ess, output)?;
        }
        "REC.PATH" => {
            misc::handle_rec_path(&parts, metro_tx, *debug_level, *out_ess, output)?;
        }
        "PRINT" => {
            misc::handle_print(&parts, variables, patterns, counters, scripts, script_index, scale, *debug_level, *out_qry, output);
        }
        "DEBUG" => {
            misc::handle_debug(&parts, debug_level, output);
        }
        "CPU" => {
            misc::handle_cpu(&parts, show_cpu, output);
        }
        "BPM" => {
            misc::handle_bpm(&parts, show_bpm, output);
        }
        "HEADER" => {
            misc::handle_header(&parts, header_level, output);
        }
        "LIMIT" => {
            misc::handle_limit(&parts, limiter_enabled, metro_tx, *debug_level, *out_qry, *out_cfm, output)?;
        }
        "SCOPE.TIME" => {
            misc::handle_scope_time(&parts, scope_timespan_ms, scope_color_mode, scope_display_mode, scope_unipolar, metro_tx, variables, patterns, counters, scripts, script_index, scale, *debug_level, *out_qry, output)?;
        }
        "SCOPE.CLR" => {
            misc::handle_scope_clr(&parts, scope_timespan_ms, scope_color_mode, scope_display_mode, scope_unipolar, variables, patterns, counters, scripts, script_index, scale, *debug_level, *out_cfm, output);
        }
        "SCOPE.MODE" => {
            misc::handle_scope_mode(&parts, scope_timespan_ms, scope_color_mode, scope_display_mode, scope_unipolar, variables, patterns, counters, scripts, script_index, scale, *debug_level, *out_cfm, output);
        }
        "SCOPE.UNI" => {
            misc::handle_scope_uni(&parts, scope_timespan_ms, scope_color_mode, scope_display_mode, scope_unipolar, variables, patterns, counters, scripts, script_index, scale, *debug_level, *out_cfm, output);
        }
        "NOTE" => {
            misc::handle_note(&parts, notes, *debug_level, *out_cfm, output);
        }
        "NOTE.CLR" => {
            misc::handle_note_clr(notes, *debug_level, *out_cfm, output);
        }
        "FLASH" => {
            if parts.len() < 2 {
                output(format!("FLASH HOLD: {}ms", *activity_hold_ms as u32));
            } else if let Ok(val) = parts[1].parse::<u32>() {
                *activity_hold_ms = val as f32;
                let _ = config::save_activity_hold_ms(val);
                output(format!("FLASH HOLD: {}ms", val));
            } else {
                output("ERROR: FLASH <MS> (0-1000)".to_string());
            }
        }
        "HL.COND" => {
            if parts.len() == 1 {
                output(format!("COND HIGHLIGHT: {}", if *show_conditional_highlight { 1 } else { 0 }));
            } else {
                match parts[1] {
                    "0" => {
                        *show_conditional_highlight = false;
                        let _ = config::save_show_conditional_highlight(*show_conditional_highlight);
                        output("COND HIGHLIGHT: OFF".to_string());
                    }
                    "1" => {
                        *show_conditional_highlight = true;
                        let _ = config::save_show_conditional_highlight(*show_conditional_highlight);
                        output("COND HIGHLIGHT: ON".to_string());
                    }
                    _ => {
                        output("ERROR: HL.COND 0|1".to_string());
                    }
                }
            }
        }
        "HL.SEQ" => {
            misc::handle_hl_seq(&parts, show_seq_highlight, output);
        }
        "METER.HDR" => {
            misc::handle_meter_hdr(&parts, show_meters_header, output);
        }
        "METER.GRID" => {
            misc::handle_meter_grid(&parts, show_meters_grid, output);
        }
        "SPECTRUM" => {
            misc::handle_spectrum(&parts, show_spectrum, output);
        }
        "ACTIVITY" => {
            misc::handle_activity(&parts, show_activity, output);
        }
        "GRID" => {
            misc::handle_grid(&parts, show_grid, output);
        }
        "GRID.DEF" => {
            misc::handle_grid_def(&parts, show_grid_view, output);
        }
        "GRID.MODE" => {
            misc::handle_grid_mode(&parts, grid_mode, output);
        }
        "TITLE" => {
            misc::handle_title(&parts, title_mode, output);
        }
        "OUT.ERR" => {
            misc::handle_out_err(&parts, out_err, output);
        }
        "OUT.ESS" => {
            misc::handle_out_ess(&parts, out_ess, output);
        }
        "OUT.QRY" => {
            misc::handle_out_qry(&parts, out_qry, output);
        }
        "OUT.CFM" => {
            misc::handle_out_cfm(&parts, out_cfm, output);
        }
        "N1" => {
            counters::handle_n1(counters, *debug_level, *out_qry, output);
        }
        "N2" => {
            counters::handle_n2(counters, *debug_level, *out_qry, output);
        }
        "N3" => {
            counters::handle_n3(counters, *debug_level, *out_qry, output);
        }
        "N4" => {
            counters::handle_n4(counters, *debug_level, *out_qry, output);
        }
        "N1.RST" => {
            counters::handle_n1_rst(counters, *debug_level, *out_cfm, output);
        }
        "N2.RST" => {
            counters::handle_n2_rst(counters, *debug_level, *out_cfm, output);
        }
        "N3.RST" => {
            counters::handle_n3_rst(counters, *debug_level, *out_cfm, output);
        }
        "N4.RST" => {
            counters::handle_n4_rst(counters, *debug_level, *out_cfm, output);
        }
        "N1.MAX" => {
            counters::handle_n1_max(&parts, variables, patterns, counters, scripts, script_index, scale, *debug_level, *out_err, *out_cfm, output);
        }
        "N2.MAX" => {
            counters::handle_n2_max(&parts, variables, patterns, counters, scripts, script_index, scale, *debug_level, *out_err, *out_cfm, output);
        }
        "N3.MAX" => {
            counters::handle_n3_max(&parts, variables, patterns, counters, scripts, script_index, scale, *debug_level, *out_err, *out_cfm, output);
        }
        "N4.MAX" => {
            counters::handle_n4_max(&parts, variables, patterns, counters, scripts, script_index, scale, *debug_level, *out_err, *out_cfm, output);
        }
        "N1.MIN" => {
            counters::handle_n1_min(&parts, variables, patterns, counters, scripts, script_index, scale, *debug_level, *out_cfm, output);
        }
        "N2.MIN" => {
            counters::handle_n2_min(&parts, variables, patterns, counters, scripts, script_index, scale, *debug_level, *out_cfm, output);
        }
        "N3.MIN" => {
            counters::handle_n3_min(&parts, variables, patterns, counters, scripts, script_index, scale, *debug_level, *out_cfm, output);
        }
        "N4.MIN" => {
            counters::handle_n4_min(&parts, variables, patterns, counters, scripts, script_index, scale, *debug_level, *out_cfm, output);
        }
        "DEL" => {
            delay::handle_del(&parts, input, variables, patterns, counters, scripts, script_index, metro_tx, scale, *debug_level, output)?;
        }
        "DEL.CLR" => {
            delay::handle_del_clr(metro_tx, *debug_level, *out_ess, output)?;
        }
        "DEL.X" => {
            delay::handle_del_x(&parts, input, variables, patterns, counters, scripts, script_index, metro_tx, scale, *debug_level, output)?;
        }
        "DEL.R" => {
            delay::handle_del_r(&parts, input, variables, patterns, counters, scripts, script_index, metro_tx, scale, *debug_level, output)?;
        }
        "Q.ROOT" => {
            scale::handle_q_root(&parts, variables, patterns, counters, scripts, script_index, scale, *debug_level, output);
        }
        "Q.SCALE" => {
            scale::handle_q_scale(&parts, variables, patterns, counters, scripts, script_index, scale, *debug_level, output);
        }
        "Q.BIT" => {
            scale::handle_q_bit(&parts, variables, patterns, counters, scripts, script_index, scale, *debug_level, output);
        }
        _ => {
            output(format!("UNKNOWN COMMAND: {}", cmd));
        }
    }

    Ok(vec![])
}
