use crate::commands::context::ExecutionContext;
use crate::commands::core::scheduling as delay;
use crate::commands::logging::log_command;
use crate::commands::system::{
    config as config_cmds, display, metro as metro_cmds, midi as midi_cmds,
    preset as preset_cmds, recording, scene as scene_cmds, sc as sc_cmds, triggers, utility,
};
use crate::config;
use anyhow::Result;

pub fn dispatch_system_commands<F>(
    cmd: &str,
    parts: &[&str],
    input: &str,
    ctx: &mut ExecutionContext,
    mut output: F,
) -> Option<Result<Vec<usize>>>
where
    F: FnMut(String),
{
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

    match cmd {
        "BRK" => {
            *script_break = true;
            Some(Ok(vec![]))
        }
        "M" => Some(metro_cmds::handle_m(parts, ctx, output).map(|_| vec![])),
        "M.BPM" => Some(metro_cmds::handle_m_bpm(parts, ctx, output).map(|_| vec![])),
        "M.ACT" => Some(metro_cmds::handle_m_act(parts, ctx, output).map(|_| vec![])),
        "M.SCRIPT" => Some(metro_cmds::handle_m_script(parts, ctx, output).map(|_| vec![])),
        "M.SYNC" => Some(metro_cmds::handle_m_sync(parts, ctx, output).map(|_| vec![])),
        "MIDI.IN" | "MIDI" => Some(midi_cmds::handle_midi_in(parts, ctx, output).map(|_| vec![])),
        "MIDI.DIAG" => Some(midi_cmds::handle_midi_diag(parts, ctx, output).map(|_| vec![])),
        "SC.DIAG" => Some(sc_cmds::handle_sc_diag(parts, ctx, output).map(|_| vec![])),
        "AUDIO.OUT" | "AUDIO" => Some(crate::commands::system::handle_audio_out(parts, metro_tx, audio_devices, output).map(|_| vec![])),
        "RST" => Some(triggers::handle_rst(ctx, output, 0).map(|_| vec![])),
        "SCRIPT" | "$" => {
            log_command(&format!("CMD: {} → DISPATCHED", input.trim()));
            Some(utility::handle_script(parts, variables, patterns, counters, scripts, script_index, scale).map(|_| vec![]))
        }
        "SAVE" => {
            scene_cmds::handle_save(parts, scripts, patterns, notes, current_scene_name, *scramble_enabled, *scramble_mode, *scramble_speed, *scramble_curve, header_scramble, *debug_level, *out_ess, &*ctx.script_mutes, *ctx.confirm_overwrite_scene, &mut *ctx.pending_confirmation, &mut *ctx.scene_modified, &*ctx.sampler_state, output);
            Some(Ok(vec![]))
        }
        "LOAD" => {
            if *load_rst {
                if let Err(e) = triggers::handle_rst(ctx, &mut |_| {}, 1) {
                    return Some(Err(e));
                }
                std::thread::sleep(std::time::Duration::from_millis(160));
            }
            if scene_cmds::handle_load(parts, &mut *ctx.variables, &mut *ctx.scripts, &mut *ctx.patterns, &mut *ctx.notes, &mut *ctx.current_scene_name, *ctx.scramble_enabled, *ctx.scramble_mode, *ctx.scramble_speed, *ctx.scramble_curve, &mut *ctx.header_scramble, *ctx.debug_level, *ctx.out_ess, &mut *ctx.script_mutes, &mut *ctx.scene_modified, &mut *ctx.sampler_state, output) {
                log_command(&format!("CMD: {} → DISPATCHED", input.trim()));
                return Some(Ok(vec![9]));
            }
            Some(Ok(vec![]))
        }
        "LOAD.RST" => {
            config_cmds::handle_load_rst(parts, load_rst, *debug_level, output);
            Some(Ok(vec![]))
        }
        "LOAD.CLR" => {
            config_cmds::handle_load_clr(parts, load_clr, *debug_level, output);
            Some(Ok(vec![]))
        }
        "AUTOLOAD" => {
            config_cmds::handle_autoload(parts, autoload, *debug_level, output);
            Some(Ok(vec![]))
        }
        "SCENES" => {
            scene_cmds::handle_scenes(*debug_level, *out_qry, output);
            Some(Ok(vec![]))
        }
        "DELETE" => {
            scene_cmds::handle_delete(parts, *debug_level, *out_ess, output);
            Some(Ok(vec![]))
        }
        "PSET" => {
            preset_cmds::handle_pset(parts, scripts, *debug_level, *out_ess, output);
            Some(Ok(vec![]))
        }
        "PSET.SAVE" => {
            preset_cmds::handle_pset_save(parts, scripts, *debug_level, *out_ess, output);
            Some(Ok(vec![]))
        }
        "PSET.DEL" => {
            preset_cmds::handle_pset_del(parts, *debug_level, *out_ess, output);
            Some(Ok(vec![]))
        }
        "PSETS" => {
            preset_cmds::handle_psets(*debug_level, *out_qry, output);
            Some(Ok(vec![]))
        }
        "THEME" => {
            utility::handle_theme(parts, theme, color_mode, output);
            Some(Ok(vec![]))
        }
        "VERSION" | "VER" => {
            utility::handle_version(output);
            Some(Ok(vec![]))
        }
        "HELP" => {
            utility::handle_help(output);
            Some(Ok(vec![]))
        }
        "REC" => Some(recording::handle_rec(metro_tx, *debug_level, *out_ess, output).map(|_| vec![])),
        "REC.STOP" => Some(recording::handle_rec_stop(metro_tx, *debug_level, *out_ess, output).map(|_| vec![])),
        "REC.PATH" => Some(recording::handle_rec_path(parts, metro_tx, *debug_level, *out_cfm, output).map(|_| vec![])),
        "PRINT" => {
            utility::handle_print(parts, variables, patterns, counters, scripts, script_index, scale, *debug_level, *out_ess, output);
            Some(Ok(vec![]))
        }
        "DEBUG" => {
            utility::handle_debug(parts, debug_level, out_err, out_ess, out_qry, out_cfm, *debug_level, output);
            Some(Ok(vec![]))
        }
        "CPU" => {
            config_cmds::handle_cpu(parts, show_cpu, *debug_level, output);
            Some(Ok(vec![]))
        }
        "BPM" => {
            config_cmds::handle_bpm(parts, show_bpm, *debug_level, output);
            Some(Ok(vec![]))
        }
        "HEADER" => {
            display::handle_header(parts, header_level, output);
            Some(Ok(vec![]))
        }
        "LIMIT" => Some(display::handle_limit(parts, limiter_enabled, metro_tx, output).map(|_| vec![])),
        "SCOPE.TIME" => Some(display::handle_scope_time(parts, scope_settings, metro_tx, variables, patterns, counters, scripts, script_index, scale, output).map(|_| vec![])),
        "SCOPE.CLR" => {
            display::handle_scope_clr(parts, scope_settings, variables, patterns, counters, scripts, script_index, scale, output);
            Some(Ok(vec![]))
        }
        "SCOPE.MODE" => {
            display::handle_scope_mode(parts, scope_settings, variables, patterns, counters, scripts, script_index, scale, output);
            Some(Ok(vec![]))
        }
        "SCOPE.UNI" => {
            display::handle_scope_uni(parts, scope_settings, variables, patterns, counters, scripts, script_index, scale, output);
            Some(Ok(vec![]))
        }
        "SCOPE.GAIN" | "SCG" => Some(display::handle_scope_gain(parts, scope_settings, metro_tx, variables, patterns, counters, scripts, script_index, scale, output).map(|_| vec![])),
        "SCOPE.RST" | "SCR" => Some(display::handle_scope_rst(scope_settings, metro_tx, output).map(|_| vec![])),
        "NOTE" => {
            display::handle_note(parts, notes, *debug_level, *out_cfm, output);
            Some(Ok(vec![]))
        }
        "NOTE.CLR" => {
            display::handle_note_clr(notes, *debug_level, *out_cfm, output);
            Some(Ok(vec![]))
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
            Some(Ok(vec![]))
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
            Some(Ok(vec![]))
        }
        "HL.SEQ" => {
            display::handle_hl_seq(parts, show_seq_highlight, *debug_level, output);
            Some(Ok(vec![]))
        }
        "METER.HDR" => {
            display::handle_meter_hdr(parts, show_meters_header, *debug_level, output);
            Some(Ok(vec![]))
        }
        "METER.GRID" => {
            display::handle_meter_grid(parts, show_meters_grid, *debug_level, output);
            Some(Ok(vec![]))
        }
        "METER.ASCII" => {
            display::handle_meter_ascii(parts, ascii_meters, *debug_level, output);
            Some(Ok(vec![]))
        }
        "SPECTRUM" => {
            display::handle_spectrum(parts, show_spectrum, *debug_level, output);
            Some(Ok(vec![]))
        }
        "ACTIVITY" => {
            display::handle_activity(parts, show_activity, *debug_level, output);
            Some(Ok(vec![]))
        }
        "GRID" => {
            display::handle_grid(parts, show_grid, *debug_level, output);
            Some(Ok(vec![]))
        }
        "GRID.DEF" => {
            display::handle_grid_def(parts, show_grid_view, *debug_level, output);
            Some(Ok(vec![]))
        }
        "GRID.MODE" => {
            display::handle_grid_mode(parts, grid_mode, *debug_level, output);
            Some(Ok(vec![]))
        }
        "TITLE" => {
            utility::handle_title(parts, title_mode, current_scene_name, *scramble_enabled, *scramble_mode, *scramble_speed, *scramble_curve, header_scramble, output);
            Some(Ok(vec![]))
        }
        "TITLE.TIMER" => {
            utility::handle_title_timer(parts, title_timer_enabled, title_timer_interval_secs, title_timer_last_toggle, title_mode, current_scene_name, *scramble_enabled, *scramble_mode, *scramble_speed, *scramble_curve, header_scramble, variables, patterns, counters, scripts, script_index, scale, output);
            Some(Ok(vec![]))
        }
        "SCRMBL" => {
            utility::handle_scrmbl(parts, scramble_enabled, *debug_level, output);
            Some(Ok(vec![]))
        }
        "SCRMBL.MODE" => {
            utility::handle_scrmbl_mode(parts, scramble_mode, *debug_level, output);
            Some(Ok(vec![]))
        }
        "SCRMBL.SPD" => {
            utility::handle_scrmbl_spd(parts, scramble_speed, *debug_level, output);
            Some(Ok(vec![]))
        }
        "SCRMBL.CRV" => {
            utility::handle_scrmbl_crv(parts, scramble_curve, *debug_level, output);
            Some(Ok(vec![]))
        }
        "CFM.QUIT" => {
            config_cmds::handle_cfm_quit(parts, &mut *ctx.confirm_quit_unsaved, *debug_level, output);
            Some(Ok(vec![]))
        }
        "CFM.SAVE" => {
            config_cmds::handle_cfm_save(parts, &mut *ctx.confirm_overwrite_scene, *debug_level, output);
            Some(Ok(vec![]))
        }
        "OUT.ERR" => {
            config_cmds::handle_out_err(parts, out_err, *debug_level, output);
            Some(Ok(vec![]))
        }
        "OUT.ESS" => {
            config_cmds::handle_out_ess(parts, out_ess, *debug_level, output);
            Some(Ok(vec![]))
        }
        "OUT.QRY" => {
            config_cmds::handle_out_qry(parts, out_qry, *debug_level, output);
            Some(Ok(vec![]))
        }
        "OUT.CFM" => {
            config_cmds::handle_out_cfm(parts, out_cfm, *debug_level, output);
            Some(Ok(vec![]))
        }
        "COMPAT" => {
            utility::handle_compat(terminal_caps, color_mode, output);
            Some(Ok(vec![]))
        }
        "COMPAT.MODE" => {
            utility::handle_compat_mode(parts, ascii_meters, scope_settings, output);
            Some(Ok(vec![]))
        }
        "MUTE" | "MUTE.1" | "MUTE.2" | "MUTE.3" | "MUTE.4" | "MUTE.5" | "MUTE.6" | "MUTE.7" | "MUTE.8" | "MUTE.M" | "MUTE.I" => {
            let script_mutes = &mut *ctx.script_mutes;
            let adjusted_parts = if cmd.starts_with("MUTE.") {
                let script_id = &cmd[5..];
                vec!["MUTE", script_id]
            } else {
                parts.to_vec()
            };
            utility::handle_mute(&adjusted_parts, script_mutes, variables, patterns, counters, scripts, script_index, scale, *debug_level, *out_qry, *out_cfm, output);
            Some(Ok(vec![]))
        }
        "PAGE" => {
            let current_page = &mut *ctx.current_page;
            utility::handle_page(parts, current_page, show_grid_view, *debug_level, *out_cfm, output);
            Some(Ok(vec![]))
        }
        _ => None,
    }
}
