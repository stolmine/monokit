use ratatui::{prelude::*, widgets::*};
use crate::types::SPECTRUM_BANDS;

pub fn render_live_page(app: &crate::App, width: usize, height: usize) -> Paragraph<'static> {
    if app.show_grid_view {
        render_grid_view(app, width, height)
    } else {
        render_repl_view(app, height)
    }
}

fn render_repl_view(app: &crate::App, height: usize) -> Paragraph<'static> {
    let visible_lines = if height > 2 { height - 2 } else { 1 };

    // Calculate the window of output to show based on scroll offset
    // output_scroll=0 means show the most recent (bottom), higher values scroll up
    let total_lines = app.output.len();
    let max_scroll = total_lines.saturating_sub(visible_lines);
    let scroll_offset = app.output_scroll.min(max_scroll);

    let end_idx = total_lines.saturating_sub(scroll_offset);
    let start_idx = end_idx.saturating_sub(visible_lines);

    let fg = app.theme.foreground;
    let text: Vec<Line> = app.output[start_idx..end_idx]
        .iter()
        .map(|line| {
            if line.starts_with("ERROR:") || line.starts_with("Error:") {
                let error_text = if line.starts_with("ERROR:") {
                    line[6..].trim()
                } else {
                    line[6..].trim()
                };
                let display_line = format!("  ERROR: {}", error_text.to_uppercase());
                Line::from(Span::styled(display_line, Style::default().fg(app.theme.error)))
            } else {
                Line::from(Span::styled(format!("  {}", line), Style::default().fg(fg)))
            }
        })
        .collect();

    // Show scroll indicator in title if scrolled up
    let title = if scroll_offset > 0 {
        format!(" LIVE [↑{}] ", scroll_offset)
    } else {
        " LIVE ".to_string()
    };

    Paragraph::new(text)
        .style(Style::default().bg(app.theme.background).fg(app.theme.foreground))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(app.theme.border))
                .title(title)
                .title_style(Style::default().fg(app.theme.foreground))
        )
}

const METER_CHARS: [char; 9] = [' ', '▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];
const METER_CHARS_ASCII: [char; 9] = [' ', '.', 'o', 'O', '0', '@', '#', '#', '#'];

fn level_to_meter_char(level: f32, ascii_mode: bool) -> char {
    let idx = (level.clamp(0.0, 1.0) * 8.0).round() as usize;
    if ascii_mode {
        METER_CHARS_ASCII[idx.min(8)]
    } else {
        METER_CHARS[idx.min(8)]
    }
}

fn vol_to_db(vol: i32) -> String {
    let db = if vol > 0 {
        20.0 * (vol as f32 / 16383.0).log10()
    } else {
        -60.0
    };
    format!("{:>+3.0}", db)
}

fn render_sampler_row(row: usize, app: &crate::App, spans: &mut Vec<Span<'static>>) {
    use std::path::Path;

    // Color scheme: section labels (SAMPLER) use theme.label
    // Param labels use theme.foreground, param values use theme.success
    match row {
        0 => {
            spans.push(Span::styled("KIT ", Style::default().fg(app.theme.foreground)));

            let name_str = if let Some(ref kit_path) = app.sampler_state.kit_path {
                Path::new(kit_path)
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("???")
            } else {
                "(none loaded)"
            };

            let display = if name_str.len() > 25 {
                format!("{}...", &name_str[..22])
            } else {
                format!("{:<26}", name_str)
            };

            spans.push(Span::styled(display, Style::default().fg(app.theme.success)));
        }
        1 => {
            spans.push(Span::styled("SLOT ", Style::default().fg(app.theme.foreground)));

            let slice_info = app.sampler_state.slice_count
                .map(|c| format!("{}SL", c))
                .unwrap_or_default();
            let slice_len = slice_info.len();
            let visual_threshold = 24usize.saturating_sub(slice_len);

            if app.sampler_state.num_slots == 0 {
                spans.push(Span::styled(format!("{:<25}", "--/--"), Style::default().fg(app.theme.success)));
            } else if app.sampler_state.num_slots <= visual_threshold {
                for i in 0..app.sampler_state.num_slots {
                    let (char, color) = if i == app.sampler_state.current_slot {
                        ("●", app.theme.success)
                    } else {
                        ("○", app.theme.secondary)
                    };
                    spans.push(Span::styled(char, Style::default().fg(color)));
                }
                if !slice_info.is_empty() {
                    spans.push(Span::raw(" "));
                    spans.push(Span::styled(slice_info.clone(), Style::default().fg(app.theme.success)));
                }
                let used = app.sampler_state.num_slots + if slice_info.is_empty() { 0 } else { 1 + slice_len };
                let padding = 25usize.saturating_sub(used);
                if padding > 0 {
                    spans.push(Span::raw(" ".repeat(padding)));
                }
            } else {
                let max_index = app.sampler_state.num_slots.saturating_sub(1);
                let numeric_display = if slice_info.is_empty() {
                    format!("{} out of {}", app.sampler_state.current_slot, max_index)
                } else {
                    format!("{} out of {} {}", app.sampler_state.current_slot, max_index, slice_info)
                };
                spans.push(Span::styled(format!("{:<25}", numeric_display), Style::default().fg(app.theme.success)));
            }
        }
        2 => {
            // 9+1+9+1+10 grid: PIT | DIR | LOOP
            let pitch = app.sampler_state.playback.pitch;
            let pitch_val = if pitch >= 0 { format!("+{}", pitch) } else { format!("{}", pitch) };
            spans.push(Span::styled("PIT", Style::default().fg(app.theme.foreground)));
            spans.push(Span::styled(format!("{:>6}", pitch_val), Style::default().fg(app.theme.success)));
            spans.push(Span::raw(" "));

            let dir_char = if app.sampler_state.playback.direction { "◄" } else { "►" };
            spans.push(Span::styled("DIR", Style::default().fg(app.theme.foreground)));
            spans.push(Span::styled(format!("{:>6}", dir_char), Style::default().fg(app.theme.success)));
            spans.push(Span::raw(" "));

            let (loop_char, loop_color) = if app.sampler_state.playback.loop_mode {
                ("●", app.theme.success)
            } else {
                ("○", app.theme.success)
            };
            spans.push(Span::styled("LOOP", Style::default().fg(app.theme.foreground)));
            spans.push(Span::styled(format!("{:>6}", loop_char), Style::default().fg(loop_color)));
        }
        3 => {
            // 9+1+9+1+10 grid: ATK | DEC | REL (values clamped to 4 digits)
            let atk = app.sampler_state.playback.attack.min(9999);
            let dec = app.sampler_state.playback.decay.min(9999);
            let rel = app.sampler_state.playback.release.min(9999);
            spans.push(Span::styled("ATK", Style::default().fg(app.theme.foreground)));
            spans.push(Span::styled(format!("{:>6}", atk), Style::default().fg(app.theme.success)));
            spans.push(Span::raw(" "));
            spans.push(Span::styled("DEC", Style::default().fg(app.theme.foreground)));
            spans.push(Span::styled(format!("{:>6}", dec), Style::default().fg(app.theme.success)));
            spans.push(Span::raw(" "));
            spans.push(Span::styled("REL", Style::default().fg(app.theme.foreground)));
            spans.push(Span::styled(format!("{:>7}", rel), Style::default().fg(app.theme.success)));
        }
        4 => {
            // 9+1+9+1+10 grid: VOL | PAN | MTR (uses mixer_data for consistency with VOL.SMP/PAN.SMP)
            spans.push(Span::styled("VOL", Style::default().fg(app.theme.foreground)));
            spans.push(Span::styled(format!("{:>6}", vol_to_db(app.mixer_data.vol_smp)), Style::default().fg(app.theme.success)));
            spans.push(Span::raw(" "));

            let pan_normalized = (app.mixer_data.pan_smp as f32 / 8192.0).clamp(-1.0, 1.0);
            let pan_value = (pan_normalized * 50.0).round() as i32;
            let pan_display = if pan_value.abs() <= 2 {
                "C".to_string()
            } else if pan_value < 0 {
                format!("L{}", pan_value.abs())
            } else {
                format!("R{}", pan_value)
            };
            spans.push(Span::styled("PAN", Style::default().fg(app.theme.foreground)));
            spans.push(Span::styled(format!("{:>6}", pan_display), Style::default().fg(app.theme.success)));
            spans.push(Span::raw(" "));

            let meter_l = app.voice_meter_data.smp_l;
            let meter_r = app.voice_meter_data.smp_r;
            let l_char = level_to_meter_char(meter_l, app.ascii_meters);
            let r_char = level_to_meter_char(meter_r, app.ascii_meters);
            let meter_color = if meter_l > 0.0 || meter_r > 0.0 { app.theme.success } else { app.theme.secondary };
            let meter_str = format!("{}{}", l_char, r_char);
            spans.push(Span::styled("MTR", Style::default().fg(app.theme.foreground)));
            spans.push(Span::styled(format!("{:>7}", meter_str), Style::default().fg(meter_color)));
        }
        5 => {
            // 9+1+9+1+10 grid: FX | CUT | RES
            let fx_display = match app.sampler_state.playback.fx_routing {
                0 => "DRY",
                1 => "WET",
                _ => "???",
            };
            spans.push(Span::styled("FX", Style::default().fg(app.theme.foreground)));
            spans.push(Span::styled(format!("{:>7}", fx_display), Style::default().fg(app.theme.success)));
            spans.push(Span::raw(" "));

            let cut_normalized = app.sampler_state.fx.filter_cut as f32 / 16383.0;
            let cut_freq = 20.0 + cut_normalized * 19980.0;
            let cut_display = if cut_freq >= 1000.0 {
                format!("{:.1}k", cut_freq / 1000.0)
            } else {
                format!("{:.0}", cut_freq)
            };
            spans.push(Span::styled("CUT", Style::default().fg(app.theme.foreground)));
            spans.push(Span::styled(format!("{:>6}", cut_display), Style::default().fg(app.theme.success)));
            spans.push(Span::raw(" "));

            let res_normalized = (app.sampler_state.fx.filter_res as f32 / 16383.0 * 100.0) as u32;
            let res_str = format!("{}%", res_normalized);
            spans.push(Span::styled("RES", Style::default().fg(app.theme.foreground)));
            spans.push(Span::styled(format!("{:>7}", res_str), Style::default().fg(app.theme.success)));
        }
        _ => {}
    }
}

fn render_fx_row(row: usize, app: &crate::App, spans: &mut Vec<Span<'static>>) {
    match row {
        0 => {
            let lfi_pct = (app.fx_mix_state.lofi_mix as f32 / 16383.0 * 100.0).round() as i32;
            let rng_pct = (app.fx_mix_state.ring_mix as f32 / 16383.0 * 100.0).round() as i32;
            let rso_pct = (app.fx_mix_state.reso_mix as f32 / 16383.0 * 100.0).round() as i32;

            spans.push(Span::styled("LFI", Style::default().fg(app.theme.foreground)));
            spans.push(Span::raw(" "));
            spans.push(Span::styled(format!("{:>3}", lfi_pct), Style::default().fg(app.theme.success)));
            spans.push(Span::raw("  "));

            spans.push(Span::styled("RNG", Style::default().fg(app.theme.foreground)));
            spans.push(Span::raw(" "));
            spans.push(Span::styled(format!("{:>3}", rng_pct), Style::default().fg(app.theme.success)));
            spans.push(Span::raw("  "));

            spans.push(Span::styled("RSO", Style::default().fg(app.theme.foreground)));
            spans.push(Span::raw(" "));
            spans.push(Span::styled(format!("{:>3}", rso_pct), Style::default().fg(app.theme.success)));
            spans.push(Span::raw("     "));
        }
        1 => {
            let cmp_pct = (app.fx_mix_state.comp_mix as f32 / 16383.0 * 100.0).round() as i32;
            let dly_pct = (app.fx_mix_state.delay_wet as f32 / 16383.0 * 100.0).round() as i32;
            let rev_pct = (app.fx_mix_state.reverb_wet as f32 / 16383.0 * 100.0).round() as i32;

            spans.push(Span::styled("CMP", Style::default().fg(app.theme.foreground)));
            spans.push(Span::raw(" "));
            spans.push(Span::styled(format!("{:>3}", cmp_pct), Style::default().fg(app.theme.success)));
            spans.push(Span::raw("  "));

            spans.push(Span::styled("DLY", Style::default().fg(app.theme.foreground)));
            spans.push(Span::raw(" "));
            spans.push(Span::styled(format!("{:>3}", dly_pct), Style::default().fg(app.theme.success)));
            spans.push(Span::raw("  "));

            spans.push(Span::styled("REV", Style::default().fg(app.theme.foreground)));
            spans.push(Span::raw(" "));
            spans.push(Span::styled(format!("{:>3}", rev_pct), Style::default().fg(app.theme.success)));
            spans.push(Span::raw("     "));
        }
        2 => {
            let br_pct = (app.fx_mix_state.beat_rep_mix as f32 / 16383.0 * 100.0).round() as i32;
            let ps_pct = (app.fx_mix_state.pitch_shift_mix as f32 / 16383.0 * 100.0).round() as i32;
            let cld_pct = (app.fx_mix_state.clouds_wet as f32 / 16383.0 * 100.0).round() as i32;

            spans.push(Span::styled("BR ", Style::default().fg(app.theme.foreground)));
            spans.push(Span::raw(" "));
            spans.push(Span::styled(format!("{:>3}", br_pct), Style::default().fg(app.theme.success)));
            spans.push(Span::raw("  "));

            spans.push(Span::styled("PS ", Style::default().fg(app.theme.foreground)));
            spans.push(Span::raw(" "));
            spans.push(Span::styled(format!("{:>3}", ps_pct), Style::default().fg(app.theme.success)));
            spans.push(Span::raw("  "));

            spans.push(Span::styled("CLD", Style::default().fg(app.theme.foreground)));
            spans.push(Span::raw(" "));
            spans.push(Span::styled(format!("{:>3}", cld_pct), Style::default().fg(app.theme.success)));
            spans.push(Span::raw("     "));
        }
        3 => {
            let dec_pct = (app.sampler_state.fx.deci_mix as f32 / 16383.0 * 100.0).round() as i32;
            let gli_pct = (app.sampler_state.fx.glit_mix as f32 / 16383.0 * 100.0).round() as i32;

            spans.push(Span::styled("DEC", Style::default().fg(app.theme.foreground)));
            spans.push(Span::raw(" "));
            spans.push(Span::styled(format!("{:>3}", dec_pct), Style::default().fg(app.theme.success)));
            spans.push(Span::raw("  "));

            spans.push(Span::styled("GLI", Style::default().fg(app.theme.foreground)));
            spans.push(Span::raw(" "));
            spans.push(Span::styled(format!("{:>3}", gli_pct), Style::default().fg(app.theme.success)));
            spans.push(Span::raw("              "));
        }
        4 => {
            spans.push(Span::raw("                              "));
        }
        5 => {
            spans.push(Span::raw("                              "));
        }
        _ => {}
    }
}

fn vol_bar_parts(vol: i32) -> (String, String) {
    let filled_count = ((vol as f32 / 16383.0) * 13.0).round() as usize;
    let empty_count = 13usize.saturating_sub(filled_count);
    ("█".repeat(filled_count), "·".repeat(empty_count))
}

fn pan_numeric(pan: i32) -> String {
    let normalized = (pan as f32 / 8192.0).clamp(-1.0, 1.0);
    let pan_value = (normalized * 50.0).round() as i32;
    if pan_value.abs() <= 2 {
        " C ".to_string()
    } else if pan_value < 0 {
        format!("L{:<2}", pan_value.abs())
    } else {
        format!("R{:<2}", pan_value)
    }
}

fn render_mixer_row(row: usize, app: &crate::App, spans: &mut Vec<Span<'static>>) {

    match row {
        0 => {
            let name_color = app.theme.activity_color(app.trigger_activity, false, app.activity_hold_ms);
            spans.push(Span::styled("OSC", Style::default().fg(name_color)));
            spans.push(Span::raw(" "));

            let (filled, empty) = vol_bar_parts(app.mixer_data.vol_osc);
            spans.push(Span::styled(filled, Style::default().fg(app.theme.success)));
            spans.push(Span::styled(empty, Style::default().fg(app.theme.secondary)));
            spans.push(Span::raw(" "));

            spans.push(Span::styled(vol_to_db(app.mixer_data.vol_osc), Style::default().fg(app.theme.foreground)));
            spans.push(Span::raw(" "));

            spans.push(Span::styled(pan_numeric(app.mixer_data.pan_osc), Style::default().fg(app.theme.foreground)));
            spans.push(Span::raw(" "));

            let meter_l = app.voice_meter_data.osc_l;
            let meter_r = app.voice_meter_data.osc_r;
            let l_char = level_to_meter_char(meter_l, app.ascii_meters);
            let r_char = level_to_meter_char(meter_r, app.ascii_meters);
            let meter_color = if meter_l > 0.0 || meter_r > 0.0 { app.theme.success } else { app.theme.secondary };
            spans.push(Span::styled(format!("{}{}", l_char, r_char), Style::default().fg(meter_color)));
            spans.push(Span::raw(" "));

            let mute_color = if app.mixer_data.mute_osc == 1 {
                app.theme.success
            } else {
                app.theme.secondary
            };
            spans.push(Span::styled("M", Style::default().fg(mute_color)));
        }
        1 => {
            let name_color = app.theme.activity_color(app.plaits_trigger_activity, false, app.activity_hold_ms);
            spans.push(Span::styled("PLA", Style::default().fg(name_color)));
            spans.push(Span::raw(" "));

            let (filled, empty) = vol_bar_parts(app.mixer_data.vol_pla);
            spans.push(Span::styled(filled, Style::default().fg(app.theme.success)));
            spans.push(Span::styled(empty, Style::default().fg(app.theme.secondary)));
            spans.push(Span::raw(" "));

            spans.push(Span::styled(vol_to_db(app.mixer_data.vol_pla), Style::default().fg(app.theme.foreground)));
            spans.push(Span::raw(" "));

            spans.push(Span::styled(pan_numeric(app.mixer_data.pan_pla), Style::default().fg(app.theme.foreground)));
            spans.push(Span::raw(" "));

            let meter_l = app.voice_meter_data.pla_l;
            let meter_r = app.voice_meter_data.pla_r;
            let l_char = level_to_meter_char(meter_l, app.ascii_meters);
            let r_char = level_to_meter_char(meter_r, app.ascii_meters);
            let meter_color = if meter_l > 0.0 || meter_r > 0.0 { app.theme.success } else { app.theme.secondary };
            spans.push(Span::styled(format!("{}{}", l_char, r_char), Style::default().fg(meter_color)));
            spans.push(Span::raw(" "));

            let mute_color = if app.mixer_data.mute_pla == 1 {
                app.theme.success
            } else {
                app.theme.secondary
            };
            spans.push(Span::styled("M", Style::default().fg(mute_color)));
        }
        2 => {
            spans.push(Span::styled("NOS", Style::default().fg(app.theme.secondary)));
            spans.push(Span::raw(" "));

            let (filled, empty) = vol_bar_parts(app.mixer_data.vol_nos);
            spans.push(Span::styled(filled, Style::default().fg(app.theme.success)));
            spans.push(Span::styled(empty, Style::default().fg(app.theme.secondary)));
            spans.push(Span::raw(" "));

            spans.push(Span::styled(vol_to_db(app.mixer_data.vol_nos), Style::default().fg(app.theme.foreground)));
            spans.push(Span::raw(" "));

            spans.push(Span::styled(pan_numeric(app.mixer_data.pan_nos), Style::default().fg(app.theme.foreground)));
            spans.push(Span::raw(" "));

            let meter_l = app.voice_meter_data.nos_l;
            let meter_r = app.voice_meter_data.nos_r;
            let l_char = level_to_meter_char(meter_l, app.ascii_meters);
            let r_char = level_to_meter_char(meter_r, app.ascii_meters);
            let meter_color = if meter_l > 0.0 || meter_r > 0.0 { app.theme.success } else { app.theme.secondary };
            spans.push(Span::styled(format!("{}{}", l_char, r_char), Style::default().fg(meter_color)));
            spans.push(Span::raw(" "));

            let mute_color = if app.mixer_data.mute_nos == 1 {
                app.theme.success
            } else {
                app.theme.secondary
            };
            spans.push(Span::styled("M", Style::default().fg(mute_color)));
        }
        3 => {
            let name_color = app.theme.activity_color(app.sampler_trigger_activity, false, app.activity_hold_ms);
            spans.push(Span::styled("SMP", Style::default().fg(name_color)));
            spans.push(Span::raw(" "));

            let (filled, empty) = vol_bar_parts(app.mixer_data.vol_smp);
            spans.push(Span::styled(filled, Style::default().fg(app.theme.success)));
            spans.push(Span::styled(empty, Style::default().fg(app.theme.secondary)));
            spans.push(Span::raw(" "));

            spans.push(Span::styled(vol_to_db(app.mixer_data.vol_smp), Style::default().fg(app.theme.foreground)));
            spans.push(Span::raw(" "));

            spans.push(Span::styled(pan_numeric(app.mixer_data.pan_smp), Style::default().fg(app.theme.foreground)));
            spans.push(Span::raw(" "));

            let meter_l = app.voice_meter_data.smp_l;
            let meter_r = app.voice_meter_data.smp_r;
            let l_char = level_to_meter_char(meter_l, app.ascii_meters);
            let r_char = level_to_meter_char(meter_r, app.ascii_meters);
            let meter_color = if meter_l > 0.0 || meter_r > 0.0 { app.theme.success } else { app.theme.secondary };
            spans.push(Span::styled(format!("{}{}", l_char, r_char), Style::default().fg(meter_color)));
            spans.push(Span::raw(" "));

            let mute_color = if app.mixer_data.mute_smp == 1 {
                app.theme.success
            } else {
                app.theme.secondary
            };
            spans.push(Span::styled("M", Style::default().fg(mute_color)));
        }
        4 => {
            // CLD - Clouds wet level
            spans.push(Span::styled("CLD ", Style::default().fg(app.theme.foreground)));

            let (filled, empty) = vol_bar_parts(app.fx_mix_state.clouds_wet);
            spans.push(Span::styled(filled, Style::default().fg(app.theme.success)));
            spans.push(Span::styled(empty, Style::default().fg(app.theme.secondary)));
            spans.push(Span::raw("  "));

            let cld_pct = (app.fx_mix_state.clouds_wet as f32 / 16383.0 * 100.0).round() as i32;
            spans.push(Span::styled(format!("{:>3}%", cld_pct), Style::default().fg(app.theme.foreground)));
            spans.push(Span::raw("       "));
        }
        5 => {
            // REV - Reverb wet level
            spans.push(Span::styled("REV ", Style::default().fg(app.theme.foreground)));

            let (filled, empty) = vol_bar_parts(app.fx_mix_state.reverb_wet);
            spans.push(Span::styled(filled, Style::default().fg(app.theme.success)));
            spans.push(Span::styled(empty, Style::default().fg(app.theme.secondary)));
            spans.push(Span::raw("  "));

            let rev_pct = (app.fx_mix_state.reverb_wet as f32 / 16383.0 * 100.0).round() as i32;
            spans.push(Span::styled(format!("{:>3}%", rev_pct), Style::default().fg(app.theme.foreground)));
            spans.push(Span::raw("       "));
        }
        _ => {}
    }
}

fn render_grid_view(app: &crate::App, width: usize, height: usize) -> Paragraph<'static> {
    use crate::types::{GRID_ICONS, GRID_LABELS};

    let mut lines = vec![];

    // Content height (excluding borders)
    let content_height = height.saturating_sub(2);

    // Calculate total content height based on enabled elements
    // Always reserve space for grid (6 rows) so spectrum/meters don't move when grid is hidden
    // + 1 meter label (if grid meters shown)
    // + 1 sampler label (if grid mode 5, always shown)
    // + 2 spectrum rows (if spectrum shown)
    // + 1 spectrum label (if spectrum shown)
    let mut total_content_height = 6;
    // But spectrum and meters still affect layout when toggled
    if app.show_meters_grid {
        total_content_height += 1;
    } else if app.grid_mode == 5 {
        // Sampler label row (shown even without meters)
        total_content_height += 1;
    }
    if app.show_spectrum {
        total_content_height += 3;
    }

    // Calculate vertical padding for centering
    let top_pad_count = content_height.saturating_sub(total_content_height) / 2;

    // Labels (mode 0) need tighter spacing (2-char label + 2 spaces), icons (mode 1) get 3 spaces
    let icon_spacing = if app.grid_mode == 0 { "  " } else { "   " };

    // Pre-compute FX viz content for mode 2 - now stored as structured data
    struct FxVizRow {
        label: String,
        freq: Option<String>,
        bar: Option<String>,
        db: Option<String>,
        q: Option<String>,
    }

    let fx_viz_rows: Vec<FxVizRow> = if app.grid_mode == 2 {
        let mut fx_rows = Vec::new();

        // Helper to format frequency nicely
        let fmt_freq = |f: f32| -> String {
            if f >= 1000.0 { format!("{:.1}k", f / 1000.0) }
            else { format!("{:.0}", f) }
        };

        // Helper to create a gain bar (12 chars): center-balanced, fills left for cut, right for boost
        let gain_bar = |db: f32| -> String {
            let normalized = (db / 24.0).clamp(-1.0, 1.0); // -24 to +24 dB range
            let bars = (normalized.abs() * 6.0).round() as usize; // 0-6 bars each side
            if db >= 0.0 {
                format!("······{}", "█".repeat(bars) + &"·".repeat(6 - bars))
            } else {
                format!("{}{}", "·".repeat(6 - bars) + &"█".repeat(bars), "······")
            }
        };

        // Row 0: Low shelf
        fx_rows.push(FxVizRow {
            label: format!("{:<4}", "LO"),
            freq: Some(format!("{:>5}", fmt_freq(app.eq_state.low_freq))),
            bar: Some(gain_bar(app.eq_state.low_db)),
            db: Some(format!("{:>7}", format!("{:+.1}dB", app.eq_state.low_db))),
            q: Some("   ".to_string()),
        });
        // Row 1: Mid peak
        fx_rows.push(FxVizRow {
            label: format!("{:<4}", "MID"),
            freq: Some(format!("{:>5}", fmt_freq(app.eq_state.mid_freq))),
            bar: Some(gain_bar(app.eq_state.mid_db)),
            db: Some(format!("{:>7}", format!("{:+.1}dB", app.eq_state.mid_db))),
            q: Some("   ".to_string()),
        });
        // Row 2: High shelf
        fx_rows.push(FxVizRow {
            label: format!("{:<4}", "HI"),
            freq: Some(format!("{:>5}", fmt_freq(app.eq_state.high_freq))),
            bar: Some(gain_bar(app.eq_state.high_db)),
            db: Some(format!("{:>7}", format!("{:+.1}dB", app.eq_state.high_db))),
            q: Some("   ".to_string()),
        });
        // Row 3: EQ section label (30 chars)
        fx_rows.push(FxVizRow {
            label: format!("{:<30}", "EQ"),
            freq: None,
            bar: None,
            db: None,
            q: None,
        });
        // Row 4: Input meter + GR value
        let in_bar = level_to_bar(app.compressor_data.input_level, 17, app.ascii_meters);
        let gr_db = app.compressor_data.gain_reduction_db;
        fx_rows.push(FxVizRow {
            label: format!("{:<4}", "IN"),
            freq: Some(in_bar),
            bar: Some(format!("GR{:+5.1}", gr_db)),
            db: None,
            q: None,
        });
        // Row 5: Output meter + Makeup value
        let out_bar = level_to_bar(app.compressor_data.output_level, 17, app.ascii_meters);
        let in_db = if app.compressor_data.input_level > 0.0001 {
            20.0 * app.compressor_data.input_level.log10()
        } else { -60.0 };
        let out_db = if app.compressor_data.output_level > 0.0001 {
            20.0 * app.compressor_data.output_level.log10()
        } else { -60.0 };
        let makeup_db = (out_db - in_db - gr_db).clamp(-20.0, 20.0);
        fx_rows.push(FxVizRow {
            label: format!("{:<4}", "OUT"),
            freq: Some(out_bar),
            bar: Some(format!("MU{:+5.1}", makeup_db)),
            db: None,
            q: None,
        });
        fx_rows
    } else {
        Vec::new()
    };

    // Add top padding for vertical centering
    for _ in 0..top_pad_count {
        lines.push(Line::from(""));
    }

    // Render 6 rows of parameter grid with optional 6-row meters on right
    let meter_rows = 6;

    for row in 0..6 {
        let mut spans = vec![];

        if app.show_grid {
            if app.grid_mode == 2 {
                // FX Viz mode - render with styled spans
                let fx_row = &fx_viz_rows[row];
                if row == 3 {
                    // Row 3: EQ section label
                    spans.push(Span::styled(fx_row.label.clone(), Style::default().fg(app.theme.label)));
                } else if row < 3 {
                    // Rows 0-2: EQ bands (LO/MID/HI)
                    spans.push(Span::styled(fx_row.label.clone(), Style::default().fg(app.theme.foreground)));
                    if let Some(ref freq) = fx_row.freq {
                        spans.push(Span::raw(" "));
                        spans.push(Span::styled(freq.clone(), Style::default().fg(app.theme.success)));
                    }
                    if let Some(ref bar) = fx_row.bar {
                        spans.push(Span::raw(" "));
                        spans.push(Span::styled(bar.clone(), Style::default().fg(app.theme.success)));
                    }
                    if let Some(ref db) = fx_row.db {
                        spans.push(Span::styled(db.clone(), Style::default().fg(app.theme.success)));
                    }
                } else {
                    // Rows 4-5: Compressor (IN/OUT)
                    spans.push(Span::styled(fx_row.label.clone(), Style::default().fg(app.theme.foreground)));
                    if let Some(ref freq) = fx_row.freq {
                        spans.push(Span::raw(" "));
                        spans.push(Span::styled(freq.clone(), Style::default().fg(app.theme.success)));
                    }
                    if let Some(ref bar) = fx_row.bar {
                        spans.push(Span::raw(" "));
                        spans.push(Span::styled(bar.clone(), Style::default().fg(app.theme.success)));
                    }
                }
            } else if app.grid_mode == 3 {
                // Mode 3: Mixer - per-voice levels, pan, mute (styled spans)
                render_mixer_row(row, app, &mut spans);
            } else if app.grid_mode == 4 {
                // Mode 4: FX Viz 2
                render_fx_row(row, app, &mut spans);
            } else if app.grid_mode == 5 {
                // Mode 5: Sampler visualization
                render_sampler_row(row, app, &mut spans);
            } else {
                // Render grid icons or labels based on grid_mode
                for col in 0..8 {
                    let idx = row * 8 + col;
                    let activity = app.param_activity.timestamps[idx];
                    let color = app.theme.activity_color(activity, false, app.activity_hold_ms);

                    if app.grid_mode == 1 {
                        // Icon mode - use scrambled icon if available
                        let icon = if idx < app.grid_scrambles.len() {
                            app.grid_scrambles[idx].current_display.as_str()
                        } else {
                            &GRID_ICONS[idx].to_string()
                        };
                        spans.push(Span::styled(icon.to_string(), Style::default().fg(color)));
                    } else {
                        // Label mode - use scrambled label if available
                        let label = if idx < app.grid_scrambles.len() {
                            app.grid_scrambles[idx].current_display.as_str()
                        } else {
                            GRID_LABELS[idx]
                        };
                        spans.push(Span::styled(label.to_string(), Style::default().fg(color)));
                    }

                    if col < 7 {
                        spans.push(Span::raw(icon_spacing));
                    }
                }
            }
        }

        if app.show_meters_grid {
            if !app.show_grid {
                // Add spacing to align meters when grid is hidden (use mode 0 width = 30 chars)
                spans.push(Span::raw("                              "));  // 30 chars (mode 0 grid width)
            } else if app.grid_mode == 1 {
                // Mode 1 (icons) is 29 chars, mode 0 (labels) is 30 chars - add 1 space to equalize
                spans.push(Span::raw(" "));
            }
            // Modes 2, 3, 4, 5 are exactly 30 chars, no adjustment needed
            // Space before meters
            spans.push(Span::raw("  "));

            // Add 2-char wide L/R meters (full height, matching grid)
            let (l_char, l_color) = get_meter_char_and_color_scaled(app.meter_data.peak_l, row, meter_rows, app.meter_data.clip_l, &app.theme, app.ascii_meters);
            let (r_char, r_color) = get_meter_char_and_color_scaled(app.meter_data.peak_r, row, meter_rows, app.meter_data.clip_r, &app.theme, app.ascii_meters);

            spans.push(Span::styled(format!("{}{}", l_char, l_char), Style::default().fg(l_color)));
            spans.push(Span::raw(" "));
            spans.push(Span::styled(format!("{}{}", r_char, r_char), Style::default().fg(r_color)));
        }

        // Always output 6 rows (empty if both grid and meters hidden) to keep spectrum position fixed
        lines.push(Line::from(spans).alignment(Alignment::Center));
    }

    // Meter/sampler labels row
    if app.show_meters_grid {
        let mut meter_label = vec![];
        // Grid space: show SAMPLER label if mode 5, FX label if mode 4, MIXER label if mode 3, COMP label if mode 2, otherwise blank
        if app.grid_mode == 5 {
            meter_label.push(Span::styled("SAMPLER", Style::default().fg(app.theme.label)));
            meter_label.push(Span::raw("                       "));  // 23 spaces to fill 30 chars
        } else if app.grid_mode == 4 {
            meter_label.push(Span::styled("FX", Style::default().fg(app.theme.label)));
            meter_label.push(Span::raw("                            "));  // 28 spaces to fill 30 chars
        } else if app.grid_mode == 3 {
            meter_label.push(Span::styled("MIXER", Style::default().fg(app.theme.label)));
            meter_label.push(Span::raw("                         "));  // 25 spaces to fill 30 chars
        } else if app.grid_mode == 2 {
            meter_label.push(Span::styled("COMP", Style::default().fg(app.theme.label)));
            meter_label.push(Span::raw("                          "));  // 26 spaces to fill 30 chars
        } else if app.grid_mode == 0 || app.grid_mode == 1 {
            meter_label.push(Span::styled("PARAM ACTIVITY", Style::default().fg(app.theme.label)));
            meter_label.push(Span::raw("                "));  // 16 spaces to fill 30 chars (14 + 16 = 30)
        } else {
            meter_label.push(Span::raw("                              "));  // Grid space (30 chars, matches mode 0)
        }
        meter_label.push(Span::raw("  "));
        meter_label.push(Span::styled("L ", Style::default().fg(app.theme.label)));
        meter_label.push(Span::raw(" "));
        meter_label.push(Span::styled("R ", Style::default().fg(app.theme.label)));
        lines.push(Line::from(meter_label).alignment(Alignment::Center));
    } else if app.grid_mode == 5 {
        // Sampler label row (shown even when meters hidden)
        let mut label_row = vec![];
        label_row.push(Span::styled("SAMPLER", Style::default().fg(app.theme.label)));
        label_row.push(Span::raw("                       "));  // 23 spaces to fill 30 chars
        lines.push(Line::from(label_row).alignment(Alignment::Center));
    } else if app.grid_mode == 4 {
        // FX label row (shown even when meters hidden)
        let mut label_row = vec![];
        label_row.push(Span::styled("FX", Style::default().fg(app.theme.label)));
        label_row.push(Span::raw("                            "));  // 28 spaces to fill 30 chars
        lines.push(Line::from(label_row).alignment(Alignment::Center));
    } else if app.grid_mode == 3 {
        // Mixer label row (shown even when meters hidden)
        let mut label_row = vec![];
        label_row.push(Span::styled("MIXER", Style::default().fg(app.theme.label)));
        label_row.push(Span::raw("                         "));  // 25 spaces to fill 30 chars
        lines.push(Line::from(label_row).alignment(Alignment::Center));
    } else if app.grid_mode == 2 {
        // COMP label row (shown even when meters hidden)
        let mut label_row = vec![];
        label_row.push(Span::styled("COMP", Style::default().fg(app.theme.label)));
        label_row.push(Span::raw("                          "));  // 26 spaces to fill 30 chars
        lines.push(Line::from(label_row).alignment(Alignment::Center));
    } else if app.grid_mode == 0 || app.grid_mode == 1 {
        // Param Activity label row (shown even when meters hidden)
        let mut label_row = vec![];
        label_row.push(Span::styled("PARAM ACTIVITY", Style::default().fg(app.theme.label)));
        label_row.push(Span::raw("                "));  // 16 spaces to fill 30 chars (14 + 16 = 30)
        lines.push(Line::from(label_row).alignment(Alignment::Center));
    }

    // Multi-row spectrum (2 rows tall) - only if spectrum is enabled
    if app.show_spectrum {
        // 2 rows × 8 sub-levels per char = 16 levels of resolution
        // Grid+meters width = 36 chars, spectrum = 30 chars, CPU = 2 chars, spacing = 1 char
        let spectrum_rows = 2;
        let spectrum_chars: [char; 9] = [' ', '▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];

        // CPU percentage and color
        let cpu_percent = app.cpu_data.avg_cpu as u32;
        let cpu_color = if app.cpu_data.avg_cpu >= 80.0 {
            app.theme.error
        } else {
            app.theme.secondary
        };

        for spec_row in 0..spectrum_rows {
            let mut spectrum_spans = vec![];

            for i in 0..SPECTRUM_BANDS {
                // Apply logarithmic scaling for better visual response
                // sqrt gives a moderate curve between linear and full log
                let raw = app.spectrum_data.bands[i];
                let scaled = (raw * 3.0).sqrt().min(1.0);
                let is_clipping = app.spectrum_data.clip[i];

                let (spec_char, spec_color) = get_spectrum_char_for_row(
                    scaled,
                    spec_row,
                    spectrum_rows,
                    &spectrum_chars,
                    is_clipping,
                    &app.theme,
                    app.ascii_meters
                );

                // Double each character for 2-char width
                spectrum_spans.push(Span::styled(
                    format!("{}{}", spec_char, spec_char),
                    Style::default().fg(spec_color)
                ));
            }

            // Right column: CPU percentage on bottom row only (aligned with spectrum bottom)
            // Layout: spectrum (30) + gap (1) + right-aligned percentage (5) = 36
            if spec_row == spectrum_rows - 1 {
                // Bottom row: show CPU percentage, right-aligned
                let cpu_text = format!("{:>5}%", cpu_percent);
                spectrum_spans.push(Span::styled(cpu_text, Style::default().fg(cpu_color)));
            } else {
                // Top row: empty space
                spectrum_spans.push(Span::raw("      "));  // 6 spaces
            }

            lines.push(Line::from(spectrum_spans).alignment(Alignment::Center));
        }

        // Label row: SPECTRUM on left, CPU on right (same row)
        let mut spec_label = vec![];
        spec_label.push(Span::styled("SPECTRUM", Style::default().fg(app.theme.label)));
        // Pad between labels: 36 - 8 (SPECTRUM) - 3 (CPU) = 25 spaces
        spec_label.push(Span::raw("                         "));  // 25 spaces
        spec_label.push(Span::styled("CPU", Style::default().fg(app.theme.label)));
        lines.push(Line::from(spec_label).alignment(Alignment::Center));
    }

    Paragraph::new(lines)
        .style(Style::default().bg(app.theme.background).fg(app.theme.foreground))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(app.theme.border))
                .title(" LIVE ")
                .title_style(Style::default().fg(app.theme.foreground))
        )
}

fn get_spectrum_char_for_row(
    level: f32,
    row: usize,
    total_rows: usize,
    chars: &[char; 9],
    is_clipping: bool,
    theme: &crate::theme::Theme,
    ascii_mode: bool
) -> (char, ratatui::style::Color) {
    const ASCII_CHARS: [char; 9] = [' ', '.', ':', '-', '=', '+', '#', '#', '#'];
    let active_chars = if ascii_mode { &ASCII_CHARS } else { chars };

    // Row 0 = top, row (total_rows-1) = bottom
    // Each row covers 1/total_rows of the range
    let row_height = 1.0 / total_rows as f32;
    let row_bottom = (total_rows - 1 - row) as f32 * row_height;
    let row_top = row_bottom + row_height;

    let color = if is_clipping { theme.error } else { theme.success };

    if level >= row_top {
        // Full block - level is above this row
        (active_chars[8], color)
    } else if level > row_bottom {
        // Partial block - level is within this row
        let fill = (level - row_bottom) / row_height;
        let char_idx = ((fill * 8.0).round() as usize).clamp(1, 8);
        (active_chars[char_idx], color)
    } else {
        // Empty
        (' ', theme.secondary)
    }
}

fn get_meter_char_and_color_scaled(
    peak: f32,
    meter_row: usize,
    total_rows: usize,
    is_clipping: bool,
    theme: &crate::theme::Theme,
    ascii_mode: bool
) -> (char, ratatui::style::Color) {
    const METER_CHARS: [char; 9] = [' ', '▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];
    const METER_CHARS_ASCII: [char; 9] = [' ', '.', ':', '-', '=', '+', '#', '#', '#'];
    let active_chars = if ascii_mode { &METER_CHARS_ASCII } else { &METER_CHARS };

    // Each row covers 1/total_rows of the range
    let row_height = 1.0 / total_rows as f32;
    let row_bottom = (total_rows - 1 - meter_row) as f32 * row_height;
    let row_top = row_bottom + row_height;

    let color = if is_clipping { theme.error } else { theme.success };

    if peak >= row_top {
        (active_chars[8], color)
    } else if peak > row_bottom {
        let fill = (peak - row_bottom) / row_height;
        let char_idx = ((fill * 8.0).round() as usize).clamp(1, 8);
        (active_chars[char_idx], color)
    } else {
        (' ', theme.secondary)
    }
}

fn level_to_bar(level: f32, width: usize, ascii_mode: bool) -> String {
    let filled = (level * width as f32).round() as usize;
    let empty = width.saturating_sub(filled);
    if ascii_mode {
        format!("{}{}", "#".repeat(filled), ".".repeat(empty))
    } else {
        format!("{}{}", "█".repeat(filled), "·".repeat(empty))
    }
}

