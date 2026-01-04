mod aliases;
pub mod common;
pub mod context;
mod core;
mod gate;
mod patterns;
pub mod randomization;
pub mod registry;
pub mod slew;
mod synth;
mod system;
pub mod validate;
mod validate_expr;

// Re-export OutputDecider so callers can use the trait methods
pub use crate::output::OutputDecider;

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
use crate::utils::split_whitespace_respecting_quotes;
use anyhow::Result;
use chrono;
use std::fs::OpenOptions;
use std::io::Write;
use std::sync::{mpsc::Sender, Arc};

pub use aliases::resolve_alias;

#[cfg(test)]
pub use validate::validate_script_command;

fn log_command(msg: &str) {
    if let Ok(mut file) = OpenOptions::new()
        .create(true)
        .append(true)
        .open("/tmp/monokit_commands.log")
    {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default();
        let secs = timestamp.as_secs();
        let millis = timestamp.subsec_millis();
        let datetime = chrono::DateTime::<chrono::Utc>::from_timestamp(secs as i64, millis * 1_000_000).unwrap_or_default();
        let formatted = datetime.format("%Y-%m-%d %H:%M:%S%.3f");
        let _ = writeln!(file, "[{}] {}", formatted, msg);
    }
}

pub fn process_command<F>(
    ctx: &mut context::ExecutionContext,
    input: &str,
    mut output: F,
) -> Result<Vec<usize>>
where
    F: FnMut(String),
{
    // Extract fields from context for backward compatibility with existing command handlers
    // Use reborrowing for mutable references to avoid moving
    let metro_tx = ctx.metro_tx;
    let metro_interval = &mut *ctx.metro_interval;
    let variables = &mut *ctx.variables;
    let patterns = &mut *ctx.patterns;
    let counters = &mut *ctx.counters;
    let scripts = &mut *ctx.scripts;
    let script_index = ctx.script_index;
    let scale = &mut *ctx.scale;
    let theme = &mut *ctx.theme;
    let debug_level = &mut *ctx.debug_level;
    let activity_hold_ms = &mut *ctx.activity_hold_ms;
    let show_cpu = &mut *ctx.show_cpu;
    let show_bpm = &mut *ctx.show_bpm;
    let header_level = &mut *ctx.header_level;
    let limiter_enabled = &mut *ctx.limiter_enabled;
    let notes = &mut *ctx.notes;
    let load_rst = &mut *ctx.load_rst;
    let load_clr = &mut *ctx.load_clr;
    let vca_mode = &mut *ctx.vca_mode;
    let show_conditional_highlight = &mut *ctx.show_conditional_highlight;
    let scope_settings = &mut *ctx.scope_settings;
    let show_meters_header = &mut *ctx.show_meters_header;
    let show_meters_grid = &mut *ctx.show_meters_grid;
    let show_spectrum = &mut *ctx.show_spectrum;
    let show_activity = &mut *ctx.show_activity;
    let show_grid = &mut *ctx.show_grid;
    let show_grid_view = &mut *ctx.show_grid_view;
    let show_seq_highlight = &mut *ctx.show_seq_highlight;
    let grid_mode = &mut *ctx.grid_mode;
    let eq_state = &mut *ctx.eq_state;
    let current_scene_name = &mut *ctx.current_scene_name;
    let title_mode = &mut *ctx.title_mode;
    let title_timer_enabled = &mut *ctx.title_timer_enabled;
    let title_timer_interval_secs = &mut *ctx.title_timer_interval_secs;
    let title_timer_last_toggle = &mut *ctx.title_timer_last_toggle;
    let out_err = &mut *ctx.out_err;
    let out_ess = &mut *ctx.out_ess;
    let out_qry = &mut *ctx.out_qry;
    let out_cfm = &mut *ctx.out_cfm;
    let audio_devices = ctx.audio_devices;
    let header_scramble = &mut *ctx.header_scramble;
    let scramble_enabled = &mut *ctx.scramble_enabled;
    let scramble_mode = &mut *ctx.scramble_mode;
    let scramble_speed = &mut *ctx.scramble_speed;
    let scramble_curve = &mut *ctx.scramble_curve;
    let ascii_meters = &mut *ctx.ascii_meters;
    let autoload = &mut *ctx.autoload;
    let terminal_caps = ctx.terminal_caps;
    let color_mode = ctx.color_mode;
    let script_break = &mut *ctx.script_break;
    let ev_counters = &mut *ctx.ev_counters;
    let br_len = &mut *ctx.br_len;
    let sync_mode = &mut *ctx.sync_mode;
    let midi_connection = &mut *ctx.midi_connection;
    let midi_timing_stats = ctx.midi_timing_stats;

    let trimmed = input.trim();

    if trimmed.is_empty() {
        return Ok(vec![]);
    }

    let parts_owned = split_whitespace_respecting_quotes(trimmed);
    let parts: Vec<&str> = parts_owned.iter().map(|s| s.as_str()).collect();
    let cmd = parts[0].to_uppercase();
    let original_cmd = cmd.clone();

    // DEBUG: Log before alias resolution
    if cmd.starts_with("CL") {
        log_command(&format!("[DEBUG] [1] Command parsing: original_cmd='{}' (from input '{}')", cmd, trimmed));
    }

    let cmd = resolve_alias(&cmd);

    // DEBUG: Log after alias resolution
    if original_cmd.starts_with("CL") {
        log_command(&format!("[DEBUG] [2] After alias resolution: original='{}' resolved='{}' match={}",
                  original_cmd, cmd, original_cmd != cmd));
    }

    if cmd != original_cmd {
        log_command(&format!("CMD: {} (alias for {})", trimmed, cmd));
    } else {
        log_command(&format!("CMD: {}", trimmed));
    }

    // DEBUG: Log entering match for CL commands
    if cmd.starts_with("CL") {
        log_command(&format!("[DEBUG] [3] Entering match statement: cmd.as_str()='{}'", cmd.as_str()));
    }

    match cmd.as_str() {
        "A" => {
            variables::handle_variable_a(&parts, ctx, output);
        }
        "B" => {
            variables::handle_variable_b(&parts, ctx, output);
        }
        "C" => {
            variables::handle_variable_c(&parts, ctx, output);
        }
        "D" => {
            variables::handle_variable_d(&parts, ctx, output);
        }
        "I" => {
            variables::handle_variable_i(&parts, ctx, output);
        }
        "X" => {
            variables::handle_variable_x(&parts, ctx, output);
        }
        "Y" => {
            variables::handle_variable_y(&parts, ctx, output);
        }
        "Z" => {
            variables::handle_variable_z(&parts, ctx, output);
        }
        "T" => {
            variables::handle_variable_t(&parts, ctx, output);
        }
        "J" => {
            variables::handle_variable_j(&parts, ctx, output)?;
        }
        "K" => {
            variables::handle_variable_k(&parts, ctx, output)?;
        }
        "P.N" => {
            patterns::handle_pattern_n(&parts, ctx, output);
        }
        "P.L" => {
            patterns::handle_pattern_l(&parts, ctx, output);
        }
        "P.I" => {
            patterns::handle_pattern_i(&parts, ctx, output);
        }
        "P.HERE" => {
            patterns::handle_pattern_here(ctx, output);
        }
        "P.NEXT" => {
            patterns::handle_pattern_next(ctx, output);
        }
        "P.PREV" => {
            patterns::handle_pattern_prev(ctx, output);
        }
        "P.PUSH" => {
            patterns::handle_pattern_push(&parts, ctx, output)?;
        }
        "P.POP" => {
            patterns::handle_pattern_pop(ctx, output);
        }
        "P.INS" => {
            patterns::handle_pattern_ins(&parts, ctx, output)?;
        }
        "P.RM" => {
            patterns::handle_pattern_rm(&parts, ctx, output)?;
        }
        "P.REV" => {
            patterns::handle_pattern_rev(ctx, output);
        }
        "P.ROT" => {
            patterns::handle_pattern_rot(&parts, ctx, output)?;
        }
        "P.SHUF" => {
            patterns::handle_pattern_shuf(ctx, output);
        }
        "P.SORT" => {
            patterns::handle_pattern_sort(ctx, output);
        }
        "P.RND" => {
            patterns::handle_pattern_rnd(&parts, ctx, output)?;
        }
        "P.ADD" => {
            patterns::handle_pattern_add(&parts, ctx, output)?;
        }
        "P.SUB" => {
            patterns::handle_pattern_sub(&parts, ctx, output)?;
        }
        "P.MUL" => {
            patterns::handle_pattern_mul(&parts, ctx, output)?;
        }
        "P.DIV" => {
            patterns::handle_pattern_div(&parts, ctx, output)?;
        }
        "P.MOD" => {
            patterns::handle_pattern_mod(&parts, ctx, output)?;
        }
        "P.SCALE" => {
            patterns::handle_pattern_scale(&parts, ctx, output)?;
        }
        "P.MIN" => {
            patterns::handle_pattern_min(ctx, output);
        }
        "P.MAX" => {
            patterns::handle_pattern_max(ctx, output);
        }
        "P.SUM" => {
            patterns::handle_pattern_sum(ctx, output);
        }
        "P.AVG" => {
            patterns::handle_pattern_avg(ctx, output);
        }
        "P.FND" => {
            patterns::handle_pattern_fnd(&parts, ctx, output)?;
        }
        "P" => {
            patterns::handle_pattern(&parts, ctx, output)?;
        }
        "PN.L" => {
            patterns::handle_pn_l(&parts, ctx, output)?;
        }
        "PN.I" => {
            patterns::handle_pn_i(&parts, ctx, output)?;
        }
        "PN.HERE" => {
            patterns::handle_pn_here(&parts, ctx, output)?;
        }
        "PN.NEXT" => {
            patterns::handle_pn_next(&parts, ctx, output)?;
        }
        "PN.PREV" => {
            patterns::handle_pn_prev(&parts, ctx, output)?;
        }
        "PN.PUSH" => {
            patterns::handle_pn_push(&parts, ctx, output)?;
        }
        "PN.POP" => {
            patterns::handle_pn_pop(&parts, ctx, output)?;
        }
        "PN.INS" => {
            patterns::handle_pn_ins(&parts, ctx, output)?;
        }
        "PN.RM" => {
            patterns::handle_pn_rm(&parts, ctx, output)?;
        }
        "PN.REV" => {
            patterns::handle_pn_rev(&parts, ctx, output)?;
        }
        "PN.ROT" => {
            patterns::handle_pn_rot(&parts, ctx, output)?;
        }
        "PN.SHUF" => {
            patterns::handle_pn_shuf(&parts, ctx, output)?;
        }
        "PN.SORT" => {
            patterns::handle_pn_sort(&parts, ctx, output)?;
        }
        "PN.RND" => {
            patterns::handle_pn_rnd(&parts, ctx, output)?;
        }
        "PN.ADD" => {
            patterns::handle_pn_add(&parts, ctx, output)?;
        }
        "PN.SUB" => {
            patterns::handle_pn_sub(&parts, ctx, output)?;
        }
        "PN.MUL" => {
            patterns::handle_pn_mul(&parts, ctx, output)?;
        }
        "PN.DIV" => {
            patterns::handle_pn_div(&parts, ctx, output)?;
        }
        "PN.MOD" => {
            patterns::handle_pn_mod(&parts, ctx, output)?;
        }
        "PN.SCALE" => {
            patterns::handle_pn_scale(&parts, ctx, output)?;
        }
        "PN.MIN" => {
            patterns::handle_pn_min(&parts, ctx, output)?;
        }
        "PN.MAX" => {
            patterns::handle_pn_max(&parts, ctx, output)?;
        }
        "PN.SUM" => {
            patterns::handle_pn_sum(&parts, ctx, output)?;
        }
        "PN.AVG" => {
            patterns::handle_pn_avg(&parts, ctx, output)?;
        }
        "PN.FND" => {
            patterns::handle_pn_fnd(&parts, ctx, output)?;
        }
        "PN" => {
            patterns::handle_pn(&parts, ctx, output)?;
        }
        "PL.DEC" | "PLD" => {
            synth_params::handle_pl_dec(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "PL.ENG" | "PLE" => {
            synth_params::handle_pl_eng(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "PL.FREQ" | "PLF" => {
            synth_params::handle_pl_freq(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "PL.HARM" | "PLH" => {
            synth_params::handle_pl_harm(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "PL.LPG" | "PLL" => {
            synth_params::handle_pl_lpg(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "PL.MORPH" | "PLM" => {
            synth_params::handle_pl_morph(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "PL.TIMB" | "PLT" => {
            synth_params::handle_pl_timb(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "PLTR" => {
            misc::handle_pltr(ctx, output)?;
        }
        "PLV" => {
            synth_params::handle_plv(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "TR" => {
            misc::handle_tr(ctx, output)?;
        }
        "BRK" => {
            *script_break = true;
        }
        "VOL" | "OUT.VOL" => {
            misc::handle_vol(&parts, ctx, output)?;
        }
        "M" => {
            metro_cmds::handle_m(&parts, ctx, output)?;
        }
        "M.BPM" => {
            metro_cmds::handle_m_bpm(&parts, ctx, output)?;
        }
        "M.ACT" => {
            metro_cmds::handle_m_act(&parts, ctx, output)?;
        }
        "M.SCRIPT" => {
            metro_cmds::handle_m_script(&parts, ctx, output)?;
        }
        "M.SYNC" => {
            metro_cmds::handle_m_sync(&parts, ctx, output)?;
        }
        "MIDI.IN" | "MIDI" => {
            midi_cmds::handle_midi_in(&parts, ctx, output)?;
        }
        "MIDI.DIAG" => {
            midi_cmds::handle_midi_diag(&parts, ctx, output)?;
        }
        "SC.DIAG" => {
            sc_cmds::handle_sc_diag(&parts, ctx, output)?;
        }
        "AUDIO.OUT" | "AUDIO" => {
            system::handle_audio_out(&parts, metro_tx, audio_devices, output)?;
        }
        "PF" | "POSC.FREQ" => {
            synth_params::handle_pf(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "PW" | "POSC.WAVE" => {
            synth_params::handle_pw(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "MF" | "MOSC.FREQ" => {
            synth_params::handle_mf(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "MW" | "MOSC.WAVE" => {
            synth_params::handle_mw(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "NW" | "NOISE.WAV" => {
            synth_params::handle_nw(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "NV" | "NOISE.VOL" => {
            synth_params::handle_nv(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "PV" | "PRI.VOL" => {
            synth_params::handle_pv(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "MV" | "MOD.VOL" => {
            synth_params::handle_mv(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "DC" | "DISC.AMT" => {
            synth_params::handle_dc(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "DM" | "DISC.MODE" => {
            synth_params::handle_dm(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "TK" | "MBUS.TRK" => {
            synth_params::handle_tk(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "MB" | "MBUS.AMT" => {
            synth_params::handle_mb(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "MBA" | "MBEV.AMT" => {
            synth_params::handle_mba(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "MBD" | "MBEV.DEC" => {
            synth_params::handle_mbd(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "MP" | "ROUT.MP" => {
            synth_params::handle_mp(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "MD" | "ROUT.MD" => {
            synth_params::handle_md(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "MT" | "ROUT.MT" => {
            synth_params::handle_mt(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "MA" | "ROUT.MA" => {
            synth_params::handle_ma(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "FM" | "MBUS.FM" => {
            synth_params::handle_fm(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "AD" | "AENV.DEC" => {
            synth_params::handle_ad(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "PD" | "PENV.DEC" => {
            synth_params::handle_pd(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "FD" | "FMEV.DEC" => {
            synth_params::handle_fd(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "PA" | "PENV.AMT" => {
            synth_params::handle_pa(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "PAV" => {
            synth_params::handle_pav(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "DD" | "DENV.DEC" => {
            synth_params::handle_dd(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "MX" | "MBUS.MIX" => {
            synth_params::handle_mx(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "MM" | "MBUS.MMX" => {
            synth_params::handle_mm(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "ME" | "MBUS.EMX" => {
            synth_params::handle_me(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "FA" | "FMEV.AMT" => {
            synth_params::handle_fa(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "DA" | "DENV.AMT" => {
            synth_params::handle_da(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "FB" | "MOSC.FB" => {
            synth_params::handle_fb(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "FBA" | "MOSC.FBA" => {
            synth_params::handle_fba(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "FBD" | "FBEV.DEC" => {
            synth_params::handle_fbd(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "RF" | "RESO.FRQ" => {
            synth_params::handle_rf(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "RD" | "RESO.DEC" => {
            synth_params::handle_rd(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "RM" | "RESO.MIX" => {
            synth_params::handle_rm(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "RK" | "RESO.KEY" => {
            synth_params::handle_rk(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "DT" | "DLY.TIME" => {
            synth_params::handle_dt(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "DF" | "DLY.FB" => {
            synth_params::handle_df(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "DLP" | "DLY.LP" => {
            synth_params::handle_dlp(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "DW" | "DLY.WET" => {
            synth_params::handle_dw(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "DS" | "DLY.SYN" => {
            synth_params::handle_ds(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "RV" | "REV.DEC" => {
            synth_params::handle_rv(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "RP" | "REV.PRE" => {
            synth_params::handle_rp(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "RH" | "REV.DMP" => {
            synth_params::handle_rh(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "RW" | "REV.WET" => {
            synth_params::handle_rw(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "D.MODE" | "DLY.MODE" => {
            synth_params::handle_d_mode(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "D.TAIL" | "DLY.TAIL" => {
            synth_params::handle_d_tail(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "R.MODE" | "REV.MODE" => {
            synth_params::handle_r_mode(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "R.TAIL" | "REV.TAIL" => {
            synth_params::handle_r_tail(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "FC" | "FILT.CUT" => {
            synth_params::handle_fc(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "FQ" | "FILT.RES" => {
            synth_params::handle_fq(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "FT" | "FILT.TYP" => {
            synth_params::handle_ft(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "FE" | "FLEV.AMT" => {
            synth_params::handle_fe(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "FED" | "FLEV.DEC" => {
            synth_params::handle_fed(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "FK" | "FILT.KEY" => {
            synth_params::handle_fk(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "MFF" | "MODF.CUT" => {
            synth_params::handle_mff(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "MFQ" | "MODF.RES" => {
            synth_params::handle_mfq(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "MC" | "ROUT.MC" => {
            synth_params::handle_mc(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "MQ" | "ROUT.MQ" => {
            synth_params::handle_mq(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "LB" | "LOFI.BIT" => {
            synth_params::handle_lb(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "LS" | "LOFI.SMP" => {
            synth_params::handle_ls(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "LM" | "LOFI.MIX" => {
            synth_params::handle_lm(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "RGF" | "RING.FRQ" => {
            synth_params::handle_rgf(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "RGW" | "RING.WAV" => {
            synth_params::handle_rgw(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "RGM" | "RING.MIX" => {
            synth_params::handle_rgm(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "CT" | "COMP.THR" => {
            synth_params::handle_ct(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "CR" | "COMP.RAT" => {
            synth_params::handle_cr(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "CA" | "COMP.ATK" => {
            synth_params::handle_ca(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "CL" | "COMP.REL" => {
            synth_params::handle_cl(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "CM" | "COMP.MKP" => {
            synth_params::handle_cm(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "CR.MIX" | "COMP.MIX" => {
            synth_params::handle_cr_mix(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "COMP.AUTO" | "CAU" => {
            synth_params::handle_comp_auto(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "EL" | "EQ.LOW" => {
            synth_params::handle_el(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, eq_state, output)?;
        }
        "ELF" | "EQ.LF" => {
            synth_params::handle_elf(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, eq_state, output)?;
        }
        "EM" | "EQ.MID" => {
            synth_params::handle_em(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, eq_state, output)?;
        }
        "EF" | "EQ.FRQ" => {
            synth_params::handle_ef(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, eq_state, output)?;
        }
        "EQ" => {
            synth_params::handle_eq_param(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, eq_state, output)?;
        }
        "EH" | "EQ.HI" => {
            synth_params::handle_eh(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, eq_state, output)?;
        }
        "EHF" | "EQ.HF" => {
            synth_params::handle_ehf(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, eq_state, output)?;
        }
        "PAN" | "OUT.PAN" => {
            synth_params::handle_pan(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, output)?;
        }
        "VCA" => {
            synth_params::handle_vca(&parts, vca_mode, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_qry, *out_cfm, output)?;
        }
        "VOL.OSC" | "VO" => {
            synth_params::handle_vol_osc(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "VOL.PLA" | "VP" => {
            synth_params::handle_vol_pla(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "VOL.NOS" | "VN" => {
            synth_params::handle_vol_nos(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "VOL.SMP" | "VS" => {
            synth_params::handle_vol_smp(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "PAN.OSC" | "PO" => {
            synth_params::handle_pan_osc(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "PAN.PLA" | "PP" => {
            synth_params::handle_pan_pla(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "PAN.NOS" | "PNN" => {
            synth_params::handle_pan_nos(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "PAN.SMP" | "PS" => {
            synth_params::handle_pan_smp(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "MUTE.OSC" | "MO" => {
            synth_params::handle_mute_osc(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "MUTE.PLA" | "MPL" => {
            synth_params::handle_mute_pla(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "MUTE.NOS" | "MN" => {
            synth_params::handle_mute_nos(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "MUTE.SMP" | "MS" => {
            synth_params::handle_mute_smp(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "KIT" => {
            synth_params::handle_kit(&parts, ctx, output)?;
        }
        "STR" => {
            synth_params::handle_str(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, &mut *ctx.sampler_state, *out_cfm, output)?;
        }
        "S.RATE" | "SR" => {
            synth_params::handle_s_rate(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "S.PITCH" | "SPT" => {
            synth_params::handle_s_pitch(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "S.FINE" | "SFN" => {
            synth_params::handle_s_fine(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "S.DIR" | "SD" => {
            synth_params::handle_s_dir(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "S.LOOP" | "SL" => {
            synth_params::handle_s_loop(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "S.START" | "SST" => {
            synth_params::handle_s_start(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "S.LEN" | "SLE" => {
            synth_params::handle_s_len(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "S.ATK" | "SA" => {
            synth_params::handle_s_atk(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, &mut *ctx.sampler_state, *out_cfm, output)?;
        }
        "S.DEC" | "SDC" => {
            synth_params::handle_s_dec(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, &mut *ctx.sampler_state, *out_cfm, output)?;
        }
        "S.REL" | "SRE" => {
            synth_params::handle_s_rel(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, &mut *ctx.sampler_state, *out_cfm, output)?;
        }
        "S.SUST" | "SSU" => {
            synth_params::handle_s_sust(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "S.VOL" | "SV" => {
            synth_params::handle_s_vol(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "S.PAN" | "SP" => {
            synth_params::handle_s_pan(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "S.FX" | "SFX" => {
            synth_params::handle_s_fx(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "S.RATEMOD" | "SRM" => {
            synth_params::handle_s_ratemod(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "S.PITCHMOD" | "SPM" => {
            synth_params::handle_s_pitchmod(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "SF.CUT" | "SFC" => {
            synth_params::handle_sf_cut(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "SF.RES" | "SFQ" => {
            synth_params::handle_sf_res(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "SF.TYPE" | "SFT" => {
            synth_params::handle_sf_type(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "SF.BITS" | "SFB" => {
            synth_params::handle_sf_bits(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "SF.RATE" | "SFR" => {
            synth_params::handle_sf_rate(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "SF.DECI" | "SFD" => {
            synth_params::handle_sf_deci(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "SF.PROB" | "SFP" => {
            synth_params::handle_sf_prob(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "SF.MULT" | "SFM" => {
            synth_params::handle_sf_mult(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "SF.GLIT" | "SFG" => {
            synth_params::handle_sf_glit(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "BR.LEN" | "BRL" => {
            synth_params::handle_br_len(&parts, *metro_interval, br_len, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "BR.REV" | "BRR" => {
            synth_params::handle_br_rev(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "BR.WIN" | "BRW" => {
            synth_params::handle_br_win(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "BR.MIX" | "BRX" => {
            synth_params::handle_br_mix(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "PS.MODE" | "PSM" => {
            synth_params::handle_ps_mode(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "PS.SEMI" | "PSS" => {
            synth_params::handle_ps_semi(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "PS.GRAIN" | "PSG" => {
            synth_params::handle_ps_grain(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "PS.MIX" | "PSX" => {
            synth_params::handle_ps_mix(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "PS.TARG" | "PST" => {
            synth_params::handle_ps_targ(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "CL.TRIG" | "CLTR" => {
            log_command(&format!("[DEBUG] [4] MATCHED CL.TRIG/CLTR pattern! cmd='{}'", cmd));
            let mut output_vec = vec![];
            log_command("[DEBUG] [5] Calling handle_cl_trig...");
            synth_params::handle_cl_trig(ctx, &mut output_vec)?;
            log_command(&format!("[DEBUG] [6] handle_cl_trig returned, output_vec.len()={}", output_vec.len()));
            for msg in output_vec {
                log_command(&format!("[DEBUG] [7] Outputting message: '{}'", msg));
                output(msg);
            }
            log_command("[DEBUG] [8] CL.TRIG dispatch complete");
        }
        "CL.PITCH" | "CLP" | "CLPT" => {
            synth_params::handle_cl_pitch(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "CL.POS" | "CLO" | "CLPS" => {
            synth_params::handle_cl_pos(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "CL.SIZE" | "CLS" | "CLSZ" => {
            synth_params::handle_cl_size(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "CL.DENS" | "CLD" | "CLDS" => {
            synth_params::handle_cl_dens(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "CL.TEX" | "CLT" | "CLTX" => {
            synth_params::handle_cl_tex(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "CL.WET" | "CLW" => {
            synth_params::handle_cl_wet(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "CL.GAIN" | "CLG" => {
            synth_params::handle_cl_gain(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "CL.SPREAD" | "CLSP" => {
            synth_params::handle_cl_spread(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "CL.RVB" | "CLRV" => {
            synth_params::handle_cl_rvb(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "CL.FB" | "CLF" => {
            synth_params::handle_cl_fb(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "CL.FREEZE" | "CLFZ" => {
            synth_params::handle_cl_freeze(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "CL.MODE" | "CLM" => {
            synth_params::handle_cl_mode(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "CL.LOFI" | "CLLO" => {
            synth_params::handle_cl_lofi(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "SLEW" => {
            slew::handle_slew(&parts, variables, patterns, counters, scripts, script_index, scale, metro_tx, *debug_level, *out_cfm, output)?;
        }
        "SLEW.ALL" => {
            slew::handle_slew_all(&parts, variables, patterns, counters, scripts, script_index, scale, metro_tx, *debug_level, *out_cfm, output)?;
        }
        "AENV.ATK" | "AA" => {
            synth_params::handle_aenv_atk(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "AENV.CRV" | "AC" => {
            synth_params::handle_aenv_crv(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "PENV.ATK" | "PAA" => {
            synth_params::handle_penv_atk(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "PENV.CRV" | "PC" => {
            synth_params::handle_penv_crv(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "FMEV.ATK" | "FAA" => {
            synth_params::handle_fmev_atk(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "FMEV.CRV" => {
            synth_params::handle_fmev_crv(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "DENV.ATK" | "DAA" => {
            synth_params::handle_denv_atk(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "DENV.CRV" => {
            synth_params::handle_denv_crv(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "FBEV.ATK" | "FBAA" => {
            synth_params::handle_fbev_atk(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "FBEV.CRV" | "FBC" => {
            synth_params::handle_fbev_crv(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "FBEV.AMT" => {
            synth_params::handle_fba(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "FLEV.ATK" | "FLAA" => {
            synth_params::handle_flev_atk(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "FLEV.CRV" | "FLC" => {
            synth_params::handle_flev_crv(&parts, variables, patterns, counters, scripts, script_index, metro_tx, *debug_level, scale, *out_cfm, output)?;
        }
        "GATE" => {
            gate::handle_gate(&parts, variables, patterns, counters, scripts, script_index, scale, metro_tx, *debug_level, output)?;
        }
        "AENV.GATE" => {
            gate::handle_aenv_gate(&parts, variables, patterns, counters, scripts, script_index, scale, metro_tx, *debug_level, output)?;
        }
        "PENV.GATE" => {
            gate::handle_penv_gate(&parts, variables, patterns, counters, scripts, script_index, scale, metro_tx, *debug_level, output)?;
        }
        "FMEV.GATE" => {
            gate::handle_fmev_gate(&parts, variables, patterns, counters, scripts, script_index, scale, metro_tx, *debug_level, output)?;
        }
        "DENV.GATE" => {
            gate::handle_denv_gate(&parts, variables, patterns, counters, scripts, script_index, scale, metro_tx, *debug_level, output)?;
        }
        "FBEV.GATE" => {
            gate::handle_fbev_gate(&parts, variables, patterns, counters, scripts, script_index, scale, metro_tx, *debug_level, output)?;
        }
        "FLEV.GATE" => {
            gate::handle_flev_gate(&parts, variables, patterns, counters, scripts, script_index, scale, metro_tx, *debug_level, output)?;
        }
        "RST" => {
            misc::handle_rst(ctx, output, 0)?;
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
        "RND.PL" => {
            randomization::handle_rnd_pl(metro_tx, *debug_level, output)?;
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
            log_command(&format!("CMD: {} → DISPATCHED", trimmed));
            return misc::handle_script(&parts, variables, patterns, counters, scripts, script_index, scale);
        }
        "SAVE" => {
            scene_cmds::handle_save(&parts, scripts, patterns, notes, current_scene_name, *scramble_enabled, *scramble_mode, *scramble_speed, *scramble_curve, header_scramble, *debug_level, *out_ess, &*ctx.script_mutes, *ctx.confirm_overwrite_scene, &mut *ctx.pending_confirmation, &mut *ctx.scene_modified, output);
        }
        "LOAD" => {
            if *load_rst {
                misc::handle_rst(ctx, &mut |_| {}, 1)?;
                // Wait for RST parameter sends to complete before executing Init script
                // RST sends 157 params with 5ms delays (785ms) + buffer for safety
                std::thread::sleep(std::time::Duration::from_millis(160));
            }
            if scene_cmds::handle_load(&parts, &mut *ctx.variables, &mut *ctx.scripts, &mut *ctx.patterns, &mut *ctx.notes, &mut *ctx.current_scene_name, *ctx.scramble_enabled, *ctx.scramble_mode, *ctx.scramble_speed, *ctx.scramble_curve, &mut *ctx.header_scramble, *ctx.debug_level, *ctx.out_ess, &mut *ctx.script_mutes, &mut *ctx.scene_modified, output) {
                log_command(&format!("CMD: {} → DISPATCHED", trimmed));
                return Ok(vec![9]);
            }
        }
        "LOAD.RST" => {
            misc::handle_load_rst(&parts, load_rst, *debug_level, output);
        }
        "LOAD.CLR" => {
            misc::handle_load_clr(&parts, load_clr, *debug_level, output);
        }
        "AUTOLOAD" => {
            misc::handle_autoload(&parts, autoload, *debug_level, output);
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
            misc::handle_theme(&parts, theme, color_mode, output);
        }
        "VERSION" | "VER" => {
            misc::handle_version(output);
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
            misc::handle_debug(&parts, debug_level, out_err, out_ess, out_qry, out_cfm, *debug_level, output);
        }
        "CPU" => {
            misc::handle_cpu(&parts, show_cpu, *debug_level, output);
        }
        "BPM" => {
            misc::handle_bpm(&parts, show_bpm, *debug_level, output);
        }
        "HEADER" => {
            misc::handle_header(&parts, header_level, output);
        }
        "LIMIT" => {
            misc::handle_limit(&parts, limiter_enabled, metro_tx, output)?;
        }
        "SCOPE.TIME" => {
            misc::handle_scope_time(&parts, scope_settings, metro_tx, variables, patterns, counters, scripts, script_index, scale, output)?;
        }
        "SCOPE.CLR" => {
            misc::handle_scope_clr(&parts, scope_settings, variables, patterns, counters, scripts, script_index, scale, output);
        }
        "SCOPE.MODE" => {
            misc::handle_scope_mode(&parts, scope_settings, variables, patterns, counters, scripts, script_index, scale, output);
        }
        "SCOPE.UNI" => {
            misc::handle_scope_uni(&parts, scope_settings, variables, patterns, counters, scripts, script_index, scale, output);
        }
        "SCOPE.GAIN" | "SCG" => {
            misc::handle_scope_gain(&parts, scope_settings, metro_tx, variables, patterns, counters, scripts, script_index, scale, output)?;
        }
        "SCOPE.RST" | "SCR" => {
            misc::handle_scope_rst(scope_settings, metro_tx, output)?;
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
            misc::handle_hl_seq(&parts, show_seq_highlight, *debug_level, output);
        }
        "METER.HDR" => {
            misc::handle_meter_hdr(&parts, show_meters_header, *debug_level, output);
        }
        "METER.GRID" => {
            misc::handle_meter_grid(&parts, show_meters_grid, *debug_level, output);
        }
        "METER.ASCII" => {
            misc::handle_meter_ascii(&parts, ascii_meters, *debug_level, output);
        }
        "SPECTRUM" => {
            misc::handle_spectrum(&parts, show_spectrum, *debug_level, output);
        }
        "ACTIVITY" => {
            misc::handle_activity(&parts, show_activity, *debug_level, output);
        }
        "GRID" => {
            misc::handle_grid(&parts, show_grid, *debug_level, output);
        }
        "GRID.DEF" => {
            misc::handle_grid_def(&parts, show_grid_view, *debug_level, output);
        }
        "GRID.MODE" => {
            misc::handle_grid_mode(&parts, grid_mode, *debug_level, output);
        }
        "TITLE" => {
            misc::handle_title(&parts, title_mode, current_scene_name, *scramble_enabled, *scramble_mode, *scramble_speed, *scramble_curve, header_scramble, output);
        }
        "TITLE.TIMER" => {
            misc::handle_title_timer(&parts, title_timer_enabled, title_timer_interval_secs, title_timer_last_toggle, title_mode, current_scene_name, *scramble_enabled, *scramble_mode, *scramble_speed, *scramble_curve, header_scramble, variables, patterns, counters, scripts, script_index, scale, output);
        }
        "SCRMBL" => {
            misc::handle_scrmbl(&parts, scramble_enabled, *debug_level, output);
        }
        "SCRMBL.MODE" => {
            misc::handle_scrmbl_mode(&parts, scramble_mode, *debug_level, output);
        }
        "SCRMBL.SPD" => {
            misc::handle_scrmbl_spd(&parts, scramble_speed, *debug_level, output);
        }
        "SCRMBL.CRV" => {
            misc::handle_scrmbl_crv(&parts, scramble_curve, *debug_level, output);
        }
        "CFM.QUIT" => {
            misc::handle_cfm_quit(&parts, &mut *ctx.confirm_quit_unsaved, *debug_level, output);
        }
        "CFM.SAVE" => {
            misc::handle_cfm_save(&parts, &mut *ctx.confirm_overwrite_scene, *debug_level, output);
        }
        "OUT.ERR" => {
            misc::handle_out_err(&parts, out_err, *debug_level, output);
        }
        "OUT.ESS" => {
            misc::handle_out_ess(&parts, out_ess, *debug_level, output);
        }
        "OUT.QRY" => {
            misc::handle_out_qry(&parts, out_qry, *debug_level, output);
        }
        "OUT.CFM" => {
            misc::handle_out_cfm(&parts, out_cfm, *debug_level, output);
        }
        "COMPAT" => {
            misc::handle_compat(terminal_caps, color_mode, output);
        }
        "COMPAT.MODE" => {
            misc::handle_compat_mode(&parts, ascii_meters, scope_settings, output);
        }
        "MUTE" | "MUTE.1" | "MUTE.2" | "MUTE.3" | "MUTE.4" | "MUTE.5" | "MUTE.6" | "MUTE.7" | "MUTE.8" | "MUTE.M" | "MUTE.I" => {
            let script_mutes = &mut *ctx.script_mutes;
            let adjusted_parts = if cmd.starts_with("MUTE.") {
                let script_id = &cmd[5..];
                vec!["MUTE", script_id]
            } else {
                parts.clone()
            };
            misc::handle_mute(&adjusted_parts, script_mutes, variables, patterns, counters, scripts, script_index, scale, *debug_level, *out_qry, *out_cfm, output);
        }
        "PAGE" => {
            let current_page = &mut *ctx.current_page;
            misc::handle_page(&parts, current_page, show_grid_view, *debug_level, *out_cfm, output);
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
        "SYNC" => {
            core::sync::handle_sync(patterns, counters, ev_counters, *debug_level, *out_cfm, output);
        }
        "SYNC.SEQ" => {
            core::sync::handle_sync_seq(patterns, *debug_level, *out_cfm, output);
        }
        "SYNC.TOG" => {
            core::sync::handle_sync_tog(patterns, *debug_level, *out_cfm, output);
        }
        "SYNC.PAT" => {
            core::sync::handle_sync_pat(patterns, *debug_level, *out_cfm, output);
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
            // DEBUG: Log unknown commands, especially CL commands
            if cmd.starts_with("CL") {
                log_command(&format!("[DEBUG] [X] UNKNOWN COMMAND HIT! cmd='{}' original_cmd='{}' input='{}'",
                          cmd, original_cmd, trimmed));
                log_command(&format!("[DEBUG] [X] cmd bytes: {:?}", cmd.as_bytes()));
            }
            log_command(&format!("CMD: {} → UNKNOWN", trimmed));
            output(format!("UNKNOWN COMMAND: {}", cmd));
        }
    };

    log_command(&format!("CMD: {} → DISPATCHED", trimmed));

    Ok(vec![])
}
