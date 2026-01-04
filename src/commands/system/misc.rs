use crate::commands::context::ExecutionContext;
use crate::config;
use crate::eval::eval_expression;
use crate::output::OutputDecider;
use crate::theme::Theme;
use crate::types::{Counters, MetroCommand, OutputCategory, PatternStorage, ScaleState, ScriptStorage, Variables, TIER_ERRORS, TIER_ESSENTIAL, TIER_CONFIRMS, TIER_QUERIES};
use anyhow::{Context, Result};
use rosc::OscType;
use std::sync::mpsc::Sender;
use std::time::Duration;

macro_rules! define_bool_toggle {
    ($fn_name:ident, $cmd_name:expr, $save_fn:path) => {
        pub fn $fn_name<F>(
            parts: &[&str],
            state: &mut bool,
            debug_level: u8,
            mut output: F,
        ) where
            F: FnMut(String),
        {
            if parts.len() == 1 {
                if debug_level >= TIER_QUERIES {
                    output(format!("{}: {}", $cmd_name, if *state { "ON" } else { "OFF" }));
                }
            } else {
                match parts[1] {
                    "0" => {
                        *state = false;
                        let _ = $save_fn(*state);
                        if debug_level >= TIER_CONFIRMS {
                            output(format!("{}: OFF", $cmd_name));
                        }
                    }
                    "1" => {
                        *state = true;
                        let _ = $save_fn(*state);
                        if debug_level >= TIER_CONFIRMS {
                            output(format!("{}: ON", $cmd_name));
                        }
                    }
                    _ => output(format!("ERROR: {} TAKES 0 (OFF) OR 1 (ON)", $cmd_name)),
                }
            }
        }
    };
    ($fn_name:ident, $cmd_name:expr, $query_fmt:expr, $off_fmt:expr, $on_fmt:expr, $save_fn:path) => {
        pub fn $fn_name<F>(
            parts: &[&str],
            state: &mut bool,
            debug_level: u8,
            mut output: F,
        ) where
            F: FnMut(String),
        {
            if parts.len() == 1 {
                if debug_level >= TIER_QUERIES {
                    output(if *state { $on_fmt.to_string() } else { $off_fmt.to_string() });
                }
            } else {
                match parts[1] {
                    "0" => {
                        *state = false;
                        let _ = $save_fn(*state);
                        if debug_level >= TIER_CONFIRMS {
                            output($off_fmt.to_string());
                        }
                    }
                    "1" => {
                        *state = true;
                        let _ = $save_fn(*state);
                        if debug_level >= TIER_CONFIRMS {
                            output($on_fmt.to_string());
                        }
                    }
                    _ => output(format!("ERROR: {} TAKES 0 (OFF) OR 1 (ON)", $cmd_name)),
                }
            }
        }
    };
}

macro_rules! define_enum_select {
    // Standard variant with labels
    ($fn_name:ident, $cmd_name:expr, $save_fn:path, $err_msg:expr, $(($value:expr, $label:expr)),+ $(,)?) => {
        pub fn $fn_name<F>(
            parts: &[&str],
            state: &mut u8,
            debug_level: u8,
            mut output: F,
        ) where
            F: FnMut(String),
        {
            if parts.len() == 1 {
                if debug_level >= TIER_QUERIES {
                    output(format!("{}: {}", $cmd_name, *state));
                }
            } else {
                let value = parts[1];
                match value {
                    $(
                        stringify!($value) => {
                            *state = $value;
                            let _ = $save_fn(*state);
                            if debug_level >= TIER_CONFIRMS {
                                output(format!("{}: {} ({})", $cmd_name, $value, $label));
                            }
                        }
                    )+
                    _ => output($err_msg.to_string()),
                }
            }
        }
    };
    // Variant without labels - just outputs the numeric value
    ($fn_name:ident, $cmd_name:expr, $save_fn:path, $err_msg:expr, $($value:expr),+ $(,)?) => {
        pub fn $fn_name<F>(
            parts: &[&str],
            state: &mut u8,
            debug_level: u8,
            mut output: F,
        ) where
            F: FnMut(String),
        {
            if parts.len() == 1 {
                if debug_level >= TIER_QUERIES {
                    output(format!("{}: {}", $cmd_name, *state));
                }
            } else if let Ok(val) = parts[1].parse::<u8>() {
                match val {
                    $(
                        $value => {
                            *state = $value;
                            let _ = $save_fn(*state);
                            if debug_level >= TIER_CONFIRMS {
                                output(format!("{}: {}", $cmd_name, $value));
                            }
                        }
                    )+
                    _ => output($err_msg.to_string()),
                }
            } else {
                output($err_msg.to_string());
            }
        }
    };
}

pub fn handle_tr<F>(
    ctx: &mut ExecutionContext,
    mut output: F,
) -> Result<()>
where
    F: FnMut(String),
{
    ctx.metro_tx
        .send(MetroCommand::SendTrigger)
        .context("Failed to send trigger to metro thread")?;
    ctx.output(
        OutputCategory::Confirm,
        "SENT TRIGGER".to_string(),
        &mut output,
    );
    Ok(())
}

pub fn handle_pltr<F>(
    ctx: &mut ExecutionContext,
    mut output: F,
) -> Result<()>
where
    F: FnMut(String),
{
    ctx.metro_tx
        .send(MetroCommand::SendPlaitsTrigger)
        .context("Failed to send Plaits trigger to metro thread")?;
    ctx.output(
        OutputCategory::Confirm,
        "PLAITS TRIGGERED".to_string(),
        &mut output,
    );
    Ok(())
}

pub fn handle_vol<F>(
    parts: &[&str],
    ctx: &mut ExecutionContext,
    mut output: F,
) -> Result<()>
where
    F: FnMut(String),
{
    if parts.len() < 2 {
        output("ERROR: VOL REQUIRES A VALUE (0.0-1.0)".to_string());
        return Ok(());
    }
    let value: f32 = parts[1]
        .parse()
        .context("Failed to parse volume value as float")?;
    if !(0.0..=1.0).contains(&value) {
        output("VOLUME MUST BE BETWEEN 0.0 AND 1.0".to_string());
        return Ok(());
    }
    ctx.metro_tx
        .send(MetroCommand::SendVolume(value))
        .context("Failed to send volume to metro thread")?;
    ctx.output(
        OutputCategory::Confirm,
        format!("SET VOLUME TO {}", value),
        &mut output,
    );
    Ok(())
}

pub fn handle_rst<F>(
    ctx: &mut ExecutionContext,
    mut output: F,
    delay_ms: u64,
) -> Result<()>
where
    F: FnMut(String),
{
    *ctx.vca_mode = true;
    ctx.metro_tx.send(MetroCommand::SendParam("pf".to_string(), OscType::Float(131.0)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("pw".to_string(), OscType::Int(0)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("mf".to_string(), OscType::Float(262.0)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("mw".to_string(), OscType::Int(0)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }

    ctx.metro_tx.send(MetroCommand::SendParam("dc".to_string(), OscType::Int(0)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("dm".to_string(), OscType::Int(0)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("tk".to_string(), OscType::Int(0)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("mb".to_string(), OscType::Int(0)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("mba".to_string(), OscType::Int(0)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("mbd".to_string(), OscType::Int(100)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("mp".to_string(), OscType::Int(0)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("md".to_string(), OscType::Int(0)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("mt".to_string(), OscType::Int(0)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("ma".to_string(), OscType::Int(0)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("fm".to_string(), OscType::Int(0)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("mx".to_string(), OscType::Int(0)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("mm".to_string(), OscType::Int(0)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("me".to_string(), OscType::Int(0)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }

    ctx.metro_tx.send(MetroCommand::SendParam("fb".to_string(), OscType::Int(0)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("fba".to_string(), OscType::Int(0)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("fbd".to_string(), OscType::Int(100)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }

    ctx.metro_tx.send(MetroCommand::SendParam("ad".to_string(), OscType::Int(100)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("pd".to_string(), OscType::Int(10)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("fd".to_string(), OscType::Int(10)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("dd".to_string(), OscType::Int(10)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("pa".to_string(), OscType::Float(0.0)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("fa".to_string(), OscType::Int(0)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("da".to_string(), OscType::Int(0)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }

    ctx.metro_tx.send(MetroCommand::SendParam("fc".to_string(), OscType::Float(10000.0)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("fq".to_string(), OscType::Int(0)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("ft".to_string(), OscType::Int(0)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("fe".to_string(), OscType::Int(0)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("fed".to_string(), OscType::Int(100)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("fk".to_string(), OscType::Int(0)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("mf_f".to_string(), OscType::Int(0)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("mf_q".to_string(), OscType::Int(0)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }

    ctx.metro_tx.send(MetroCommand::SendParam("rf".to_string(), OscType::Float(131.0)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("rd".to_string(), OscType::Int(100)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("rm".to_string(), OscType::Int(0)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("rk".to_string(), OscType::Int(0)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }

    ctx.metro_tx.send(MetroCommand::SendParam("dt".to_string(), OscType::Int(250)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("df".to_string(), OscType::Int(0)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("dlp".to_string(), OscType::Int(5000)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("dw".to_string(), OscType::Int(0)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("ds".to_string(), OscType::Int(0)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("dmode".to_string(), OscType::Int(2)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("dtail".to_string(), OscType::Int(1)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }

    ctx.metro_tx.send(MetroCommand::SendParam("rv".to_string(), OscType::Int(0)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("rp".to_string(), OscType::Int(0)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("rh".to_string(), OscType::Int(8000)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("rw".to_string(), OscType::Int(0)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("rmode".to_string(), OscType::Int(2)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("rtail".to_string(), OscType::Int(1)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }

    ctx.metro_tx.send(MetroCommand::SendParam("lb".to_string(), OscType::Int(16)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("ls".to_string(), OscType::Int(48000)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("lm".to_string(), OscType::Int(0)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }

    ctx.metro_tx.send(MetroCommand::SendParam("rgf".to_string(), OscType::Float(131.0)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("rgw".to_string(), OscType::Int(0)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("rgm".to_string(), OscType::Int(0)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }

    ctx.metro_tx.send(MetroCommand::SendParam("ct".to_string(), OscType::Int(8192)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("cr".to_string(), OscType::Int(1)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("ca".to_string(), OscType::Int(10)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("cl".to_string(), OscType::Int(100)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("cm".to_string(), OscType::Int(0)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("cr_mix".to_string(), OscType::Int(16383)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }

    ctx.metro_tx.send(MetroCommand::SendParam("el".to_string(), OscType::Int(0)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("elf".to_string(), OscType::Float(200.0)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("em".to_string(), OscType::Int(0)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("ef".to_string(), OscType::Float(1000.0)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("eq".to_string(), OscType::Float(1.0)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("eh".to_string(), OscType::Int(0)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("ehf".to_string(), OscType::Float(4000.0)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }

    ctx.metro_tx.send(MetroCommand::SendParam("nw".to_string(), OscType::Int(0)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("np".to_string(), OscType::Int(0)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("nm".to_string(), OscType::Int(0)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("nv".to_string(), OscType::Int(0)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("pv".to_string(), OscType::Int(16383)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("mv".to_string(), OscType::Int(0)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }

    ctx.metro_tx.send(MetroCommand::SendParam("pn".to_string(), OscType::Int(0)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("t_gate".to_string(), OscType::Int(0)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("vca_mode".to_string(), OscType::Int(1)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }

    ctx.metro_tx.send(MetroCommand::SendParam("br_len".to_string(), OscType::Int(250)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("br_rev".to_string(), OscType::Int(0)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("br_win".to_string(), OscType::Int(5)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("br_mix".to_string(), OscType::Int(0)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }

    ctx.metro_tx.send(MetroCommand::SendParam("ps_mode".to_string(), OscType::Int(0)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("ps_semi".to_string(), OscType::Int(0)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("ps_grain".to_string(), OscType::Int(20)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("ps_mix".to_string(), OscType::Int(0)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("ps_targ".to_string(), OscType::Int(0)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }

    ctx.metro_tx.send(MetroCommand::SendParam("pitch".to_string(), OscType::Float(131.0)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("detune".to_string(), OscType::Int(0)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("engine".to_string(), OscType::Int(0)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("harmonics".to_string(), OscType::Int(8192)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("timbre".to_string(), OscType::Int(8192)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("morph".to_string(), OscType::Int(8192)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("decay".to_string(), OscType::Int(8192)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("lpg".to_string(), OscType::Int(8192)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("plv".to_string(), OscType::Int(0)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("pav".to_string(), OscType::Int(0)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }

    ctx.metro_tx.send(MetroCommand::SendParam("cl_pitch".to_string(), OscType::Int(8192)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("cl_pos".to_string(), OscType::Int(8192)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("cl_size".to_string(), OscType::Int(8192)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("cl_dens".to_string(), OscType::Int(8192)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("cl_tex".to_string(), OscType::Int(8192)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("cl_wet".to_string(), OscType::Int(0)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("cl_gain".to_string(), OscType::Int(8192)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("cl_spread".to_string(), OscType::Int(8192)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("cl_rvb".to_string(), OscType::Int(0)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("cl_fb".to_string(), OscType::Int(0)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("cl_freeze".to_string(), OscType::Int(0)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("cl_mode".to_string(), OscType::Int(0)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("cl_lofi".to_string(), OscType::Int(0)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }

    ctx.metro_tx.send(MetroCommand::SendVolume(1.0))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }

    ctx.metro_tx.send(MetroCommand::SendParam("slew_time".to_string(), OscType::Int(0)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("slew_pf".to_string(), OscType::Int(-1)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("slew_mf".to_string(), OscType::Int(-1)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("slew_fc".to_string(), OscType::Int(-1)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("slew_fm".to_string(), OscType::Int(-1)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("slew_mx".to_string(), OscType::Int(-1)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("slew_dc".to_string(), OscType::Int(-1)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("slew_fb".to_string(), OscType::Int(-1)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("slew_fq".to_string(), OscType::Int(-1)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("slew_fk".to_string(), OscType::Int(-1)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("slew_fe".to_string(), OscType::Int(-1)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("slew_rf".to_string(), OscType::Int(-1)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("slew_rm".to_string(), OscType::Int(-1)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("slew_dt".to_string(), OscType::Int(-1)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("slew_df".to_string(), OscType::Int(-1)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("slew_dw".to_string(), OscType::Int(-1)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("slew_rv".to_string(), OscType::Int(-1)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("slew_rw".to_string(), OscType::Int(-1)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("slew_volume".to_string(), OscType::Int(-1)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("slew_pn".to_string(), OscType::Int(-1)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("slew_lb".to_string(), OscType::Int(-1)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("slew_ls".to_string(), OscType::Int(-1)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("slew_lm".to_string(), OscType::Int(-1)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("slew_rgf".to_string(), OscType::Int(-1)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("slew_rgm".to_string(), OscType::Int(-1)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("slew_ct".to_string(), OscType::Int(-1)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("slew_cm".to_string(), OscType::Int(-1)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("slew_el".to_string(), OscType::Int(-1)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("slew_em".to_string(), OscType::Int(-1)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("slew_eh".to_string(), OscType::Int(-1)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("slew_ef".to_string(), OscType::Int(-1)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }

    ctx.metro_tx.send(MetroCommand::SendParam("env_atk".to_string(), OscType::Int(1)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("env_crv".to_string(), OscType::Int(-4)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("aenv_atk".to_string(), OscType::Int(-1)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("penv_atk".to_string(), OscType::Int(-1)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("fmev_atk".to_string(), OscType::Int(-1)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("denv_atk".to_string(), OscType::Int(-1)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("fbev_atk".to_string(), OscType::Int(-1)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("flev_atk".to_string(), OscType::Int(-1)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("aenv_crv".to_string(), OscType::Int(-100)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("penv_crv".to_string(), OscType::Int(-100)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("fmev_crv".to_string(), OscType::Int(-100)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("denv_crv".to_string(), OscType::Int(-100)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("fbev_crv".to_string(), OscType::Int(-100)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("flev_crv".to_string(), OscType::Int(-100)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }

    ctx.metro_tx.send(MetroCommand::SendParam("s_rate".to_string(), OscType::Int(8192)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("s_pitch".to_string(), OscType::Int(0)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("s_fine".to_string(), OscType::Int(0)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("s_direction".to_string(), OscType::Int(0)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("s_loop".to_string(), OscType::Int(0)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("s_startFrame".to_string(), OscType::Int(0)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("s_endFrame".to_string(), OscType::Int(-1)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("s_atk".to_string(), OscType::Int(0)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("s_dec".to_string(), OscType::Int(8192)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("s_rel".to_string(), OscType::Int(1000)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("s_sust".to_string(), OscType::Int(0)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("s_volume".to_string(), OscType::Int(8192)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }

    ctx.metro_tx.send(MetroCommand::SendParam("sf_cut".to_string(), OscType::Int(16383)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("sf_res".to_string(), OscType::Int(0)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("sf_type".to_string(), OscType::Int(0)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("sf_bits".to_string(), OscType::Int(0)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("sf_rate".to_string(), OscType::Int(16383)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("sf_deci".to_string(), OscType::Int(0)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("sf_prob".to_string(), OscType::Int(0)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("sf_mult".to_string(), OscType::Int(8192)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("sf_glit".to_string(), OscType::Int(0)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }

    ctx.metro_tx.send(MetroCommand::SendParam("vol_osc".to_string(), OscType::Int(16383)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("vol_pla".to_string(), OscType::Int(16383)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("vol_nos".to_string(), OscType::Int(16383)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("vol_smp".to_string(), OscType::Int(16383)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("pan_osc".to_string(), OscType::Int(0)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("pan_pla".to_string(), OscType::Int(0)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("pan_nos".to_string(), OscType::Int(0)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("pan_smp".to_string(), OscType::Int(0)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("mute_osc".to_string(), OscType::Int(0)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("mute_pla".to_string(), OscType::Int(0)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("mute_nos".to_string(), OscType::Int(0)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }
    ctx.metro_tx.send(MetroCommand::SendParam("mute_smp".to_string(), OscType::Int(0)))?;
    if delay_ms > 0 {
        std::thread::sleep(Duration::from_millis(delay_ms));
    }

    ctx.output(
        OutputCategory::Essential,
        "RESET TO DEFAULTS".to_string(),
        &mut output,
    );
    Ok(())
}

pub fn handle_script(
    parts: &[&str],
    variables: &Variables,
    patterns: &mut PatternStorage,
    counters: &mut Counters,
    scripts: &ScriptStorage,
    script_index: usize,
    scale: &ScaleState,
) -> Result<Vec<usize>> {
    if parts.len() < 2 {
        return Err(anyhow::anyhow!("SCRIPT REQUIRES NUMBER 1-8"));
    }
    let num: usize = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index, scale) {
        if expr_val < 1 || expr_val > 8 {
            return Err(anyhow::anyhow!("SCRIPT NUMBER MUST BE 1-8"));
        }
        expr_val as usize
    } else {
        parts[1]
            .parse()
            .context("Failed to parse script number")?
    };
    if num < 1 || num > 8 {
        return Err(anyhow::anyhow!("SCRIPT NUMBER MUST BE 1-8"));
    }
    Ok(vec![num - 1])
}

pub fn handle_theme<F>(
    parts: &[&str],
    theme: &mut Theme,
    color_mode: crate::types::ColorMode,
    mut output: F,
) where
    F: FnMut(String),
{
    if parts.len() == 1 {
        match config::load_config() {
            Ok(cfg) => {
                let available = config::list_themes(&cfg);
                output(format!("CURRENT THEME: {}", theme.name.to_uppercase()));
                output("AVAILABLE THEMES:".to_string());
                for theme_name in available {
                    output(format!("  {}", theme_name.to_uppercase()));
                }
            }
            Err(_) => {
                output(format!("CURRENT THEME: {}", theme.name.to_uppercase()));
                output("AVAILABLE THEMES:".to_string());
                output("  DARK".to_string());
                output("  LIGHT".to_string());
                output("  SYSTEM".to_string());
            }
        }
    } else {
        let name = parts[1];
        match config::load_config() {
            Ok(cfg) => {
                match config::load_theme_by_name(name, &cfg) {
                    Ok(new_theme) => {
                        let theme_name = new_theme.name.clone();
                        *theme = if color_mode == crate::types::ColorMode::Color256 {
                            new_theme.to_256_color()
                        } else {
                            new_theme
                        };
                        if let Err(e) = config::save_theme_mode(&theme_name) {
                            output(format!("WARNING: FAILED TO SAVE THEME: {:?}", e));
                        }
                        output(format!("SWITCHED TO {} THEME", theme_name.to_uppercase()));
                    }
                    Err(e) => {
                        output(e.to_string().to_uppercase());
                    }
                }
            }
            Err(e) => {
                output(format!("FAILED TO LOAD CONFIG: {:?}", e));
            }
        }
    }
}

pub fn handle_version<F>(
    mut output: F,
) where
    F: FnMut(String),
{
    output(format!("MONOKIT v{}", env!("CARGO_PKG_VERSION")));
}

pub fn handle_help<F>(
    mut output: F,
) where
    F: FnMut(String),
{
    output("=== MONOKIT COMMANDS ===".to_string());
    output("".to_string());
    output("TRIGGER: TR".to_string());
    output("VOLUME:  VOL <0.0-1.0>".to_string());
    output("RESET:   RST".to_string());
    output("".to_string());
    output("-- REPL UTILITIES --".to_string());
    output("CLEAR / CLR     CLEAR OUTPUT HISTORY".to_string());
    output("DEBUG <0-5>  SET VERBOSITY LEVEL".to_string());
    output("HEADER       SHOW CURRENT HEADER LEVEL".to_string());
    output("HEADER <0-4>    SET HEADER VERBOSITY".to_string());
    output("                0=NAV ONLY, 1=+METERS, 2=+C/P".to_string());
    output("                3=FULL NAV, 4=+CPU (DEFAULT)".to_string());
    output("PRINT \"TEXT\"    OUTPUT LITERAL STRING".to_string());
    output("PRINT <EXPR>    EVALUATE AND PRINT EXPRESSION".to_string());
    output("".to_string());
    output("-- OSCILLATORS --".to_string());
    output("PF <HZ>     PRIMARY FREQ (20-20000)".to_string());
    output("PW <0-2>    PRIMARY WAVE (SIN/TRI/SAW)".to_string());
    output("MF <HZ>     MOD FREQ".to_string());
    output("MW <0-2>    MOD WAVE".to_string());
    output("".to_string());
    output("-- FM & DISCONTINUITY --".to_string());
    output("FM <0-16383>  FM INDEX".to_string());
    output("FA <0-16383>  FM ENV AMOUNT".to_string());
    output("FD <MS>       FM ENV DECAY".to_string());
    output("DC <0-16383>  DISCONTINUITY".to_string());
    output("DA <0-16383>  DC ENV AMOUNT".to_string());
    output("DD <MS>       DC ENV DECAY".to_string());
    output("DM <0-6>  DC MODE (FOLD/TANH/SOFT/HARD...)".to_string());
    output("".to_string());
    output("-- ENVELOPES --".to_string());
    output("AD <MS>       AMP DECAY".to_string());
    output("PD <MS>       PITCH DECAY".to_string());
    output("PA <0-16>     PITCH ENV AMOUNT".to_string());
    output("".to_string());
    output("-- MOD BUS --".to_string());
    output("MB <0-16383>  MOD BUS AMOUNT".to_string());
    output("MP/MD/MT/MA <0|1>  ROUTING TOGGLES".to_string());
    output("TK <0-16383>  TRACKING".to_string());
    output("".to_string());
    output("-- MIX --".to_string());
    output("MX <0-16383>  MIX AMOUNT".to_string());
    output("MM/ME <0|1>   MIX ROUTING".to_string());
    output("".to_string());
    output("-- METRO --".to_string());
    output("M             SHOW INTERVAL".to_string());
    output("M <MS>        SET INTERVAL".to_string());
    output("M.BPM <BPM>   SET BPM".to_string());
    output("M.ACT <0|1>     START/STOP".to_string());
    output("M.SCRIPT <1-8> SET SCRIPT TO CALL ON EACH TICK".to_string());
    output("".to_string());
    output("-- SCRIPTS --".to_string());
    output("SCRIPT <1-8>  EXECUTE STORED SCRIPT".to_string());
    output("".to_string());
    output("-- SCENES --".to_string());
    output("SAVE <NAME>   SAVE CURRENT STATE".to_string());
    output("LOAD <NAME>   LOAD SAVED STATE".to_string());
    output("AUTOLOAD <0|1> AUTO-LOAD LAST SCENE AT START".to_string());
    output("SCENES        LIST ALL SCENES".to_string());
    output("DELETE <NAME> DELETE A SCENE".to_string());
    output("".to_string());
    output("-- THEME --".to_string());
    output("THEME            SHOW CURRENT".to_string());
    output("THEME DARK       DARK MODE".to_string());
    output("THEME LIGHT      LIGHT MODE".to_string());
    output("THEME SYSTEM     USE OS PREFERENCE".to_string());
    output("THEME CUSTOM     USE CUSTOM THEME".to_string());
    output("".to_string());
    output("-- RECORDING --".to_string());
    output("REC              START RECORDING".to_string());
    output("REC.STOP         STOP RECORDING".to_string());
    output("REC.PATH <PATH>  SET CUSTOM PATH PREFIX".to_string());
}

pub fn handle_rec<F>(
    metro_tx: &Sender<MetroCommand>,
    debug_level: u8,
    out_ess: bool,
    mut output: F,
) -> Result<()>
where
    F: FnMut(String),
{
    // Get current working directory
    let cwd = std::env::current_dir()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_else(|_| ".".to_string());

    metro_tx
        .send(MetroCommand::StartRecording(cwd))
        .context("Failed to send recording command")?;
    if debug_level >= TIER_ESSENTIAL || out_ess {
        output("RECORDING STARTED".to_string());
    }
    Ok(())
}

pub fn handle_rec_stop<F>(
    metro_tx: &Sender<MetroCommand>,
    debug_level: u8,
    out_ess: bool,
    mut output: F,
) -> Result<()>
where
    F: FnMut(String),
{
    metro_tx
        .send(MetroCommand::StopRecording)
        .context("Failed to send stop recording command")?;
    if debug_level >= TIER_ESSENTIAL || out_ess {
        output("RECORDING STOPPED".to_string());
    }
    Ok(())
}

pub fn handle_rec_path<F>(
    parts: &[&str],
    metro_tx: &Sender<MetroCommand>,
    debug_level: u8,
    out_cfm: bool,
    mut output: F,
) -> Result<()>
where
    F: FnMut(String),
{
    if parts.len() < 2 {
        output("REC.PATH REQUIRES A PATH PREFIX".to_string());
        return Ok(());
    }

    let path = parts[1].to_string();
    metro_tx
        .send(MetroCommand::SetRecordingPath(path.clone()))
        .context("Failed to send recording path")?;
    if debug_level >= TIER_CONFIRMS || out_cfm {
        output(format!("SET RECORDING PATH PREFIX TO: {}", path.to_uppercase()));
    }
    Ok(())
}

pub fn handle_print<F>(
    parts: &[&str],
    variables: &Variables,
    patterns: &mut PatternStorage,
    counters: &mut Counters,
    scripts: &ScriptStorage,
    script_index: usize,
    scale: &ScaleState,
    debug_level: u8,
    out_ess: bool,
    mut output: F,
) where
    F: FnMut(String),
{
    if parts.len() < 2 {
        output("ERROR: PRINT REQUIRES AT LEAST 1 ARGUMENT".to_string());
        return;
    }

    if parts[1].starts_with('"') || parts[1].starts_with('\'') {
        let joined = parts[1..].join(" ");
        let quote_char = if parts[1].starts_with('"') { '"' } else { '\'' };

        if joined.starts_with(quote_char) && joined.ends_with(quote_char) && joined.len() > 1 {
            let literal = &joined[1..joined.len() - 1];
            if debug_level >= TIER_ESSENTIAL || out_ess {
                output(literal.to_string());
            }
        } else {
            output("UNTERMINATED STRING LITERAL".to_string());
        }
    } else {
        if let Some((result, _)) = eval_expression(parts, 1, variables, patterns, counters, scripts, script_index, scale) {
            if debug_level >= TIER_ESSENTIAL || out_ess {
                output(format!("{}", result));
            }
        } else {
            output("FAILED TO EVALUATE EXPRESSION".to_string());
        }
    }
}

/// Custom DEBUG handler that sets debug_level AND synchronizes out_* flags
pub fn handle_debug<F>(
    parts: &[&str],
    debug_level: &mut u8,
    out_err: &mut bool,
    out_ess: &mut bool,
    out_qry: &mut bool,
    out_cfm: &mut bool,
    current_level: u8,
    mut output: F,
) where
    F: FnMut(String),
{
    const LABELS: &[(u8, &str)] = &[
        (0, "SILENT"),
        (1, "ERRORS"),
        (2, "ESSENTIAL"),
        (3, "QUERIES"),
        (4, "CONFIRMS"),
        (5, "VERBOSE"),
    ];

    if parts.len() == 1 {
        // Query mode
        if current_level >= TIER_QUERIES {
            output(format!("DEBUG: {}", *debug_level));
        }
    } else {
        match parts[1].parse::<u8>() {
            Ok(val) if val <= 5 => {
                *debug_level = val;
                let _ = config::save_debug_level(val);

                // Synchronize out_* flags based on tier
                // TIER_ERRORS=1, TIER_ESSENTIAL=2, TIER_QUERIES=3, TIER_CONFIRMS=4
                *out_err = val >= 1;
                *out_ess = val >= 2;
                *out_qry = val >= 3;
                *out_cfm = val >= 4;

                // Save out_* flags
                let _ = config::save_out_err(*out_err);
                let _ = config::save_out_ess(*out_ess);
                let _ = config::save_out_qry(*out_qry);
                let _ = config::save_out_cfm(*out_cfm);

                if current_level >= TIER_CONFIRMS {
                    let label = LABELS.iter().find(|(v, _)| *v == val).map(|(_, l)| *l).unwrap_or("UNKNOWN");
                    output(format!("DEBUG: {} ({})", val, label));
                }
            }
            _ => output("ERROR: DEBUG TAKES 0-5".to_string()),
        }
    }
}

define_bool_toggle!(handle_cpu, "CPU", "CPU DISPLAY: {}", "CPU DISPLAY: OFF", "CPU DISPLAY: ON", config::save_show_cpu);

define_bool_toggle!(handle_bpm, "BPM", "BPM: {}", "BPM: OFF", "BPM: ON", config::save_show_bpm);

pub fn handle_header<F>(
    parts: &[&str],
    header_level: &mut u8,
    mut output: F,
) where
    F: FnMut(String),
{
    if parts.len() == 1 {
        output(format!("HEADER LEVEL: {}", header_level));
    } else {
        let value = parts[1];
        match value {
            "0" => {
                *header_level = 0;
                let _ = config::save_header_level(*header_level);
                output("HEADER LEVEL: 0 (NAV ONLY)".to_string());
            }
            "1" => {
                *header_level = 1;
                let _ = config::save_header_level(*header_level);
                output("HEADER LEVEL: 1 (NAV + METERS)".to_string());
            }
            "2" => {
                *header_level = 2;
                let _ = config::save_header_level(*header_level);
                output("HEADER LEVEL: 2 (NAV + H|P + METERS)".to_string());
            }
            "3" => {
                *header_level = 3;
                let _ = config::save_header_level(*header_level);
                output("HEADER LEVEL: 3 (FULL NAV + H|P + METERS)".to_string());
            }
            "4" => {
                *header_level = 4;
                let _ = config::save_header_level(*header_level);
                output("HEADER LEVEL: 4 (FULL NAV + H|P + METERS + CPU)".to_string());
            }
            _ => {
                output("ERROR: HEADER TAKES 0-4".to_string());
            }
        }
    }
}

pub fn handle_limit<F>(
    parts: &[&str],
    limiter_enabled: &mut bool,
    metro_tx: &Sender<MetroCommand>,
    mut output: F,
) -> Result<()>
where
    F: FnMut(String),
{
    if parts.len() == 1 {
        output(format!("LIMITER: {}", if *limiter_enabled { 1 } else { 0 }));
    } else {
        let value = parts[1];
        match value {
            "0" => {
                *limiter_enabled = false;
                metro_tx
                    .send(MetroCommand::SendParam("limit".to_string(), OscType::Int(0)))
                    .context("Failed to send limiter param")?;
                let _ = config::save_limiter_enabled(*limiter_enabled);
                output("LIMITER: OFF".to_string());
            }
            "1" => {
                *limiter_enabled = true;
                metro_tx
                    .send(MetroCommand::SendParam("limit".to_string(), OscType::Int(1)))
                    .context("Failed to send limiter param")?;
                let _ = config::save_limiter_enabled(*limiter_enabled);
                output("LIMITER: ON".to_string());
            }
            _ => {
                output("ERROR: LIMIT TAKES 0 (OFF) OR 1 (ON)".to_string());
            }
        }
    }
    Ok(())
}

define_bool_toggle!(handle_load_rst, "LOAD.RST", "LOAD.RST: {}", "LOAD.RST: OFF (PERSIST PARAMS)", "LOAD.RST: ON (RESET BEFORE LOAD)", config::save_load_rst);

define_bool_toggle!(handle_load_clr, "LOAD.CLR", "LOAD.CLR: {}", "LOAD.CLR: OFF", "LOAD.CLR: ON (CLEAR ON LOAD)", config::save_load_clr);

define_bool_toggle!(handle_autoload, "AUTOLOAD", "AUTOLOAD: {}", "AUTOLOAD: OFF", "AUTOLOAD: ON (LOAD LAST SCENE)", config::save_autoload);

pub fn handle_scope_time<F>(
    parts: &[&str],
    scope_settings: &mut crate::types::ScopeSettings,
    metro_tx: &Sender<MetroCommand>,
    variables: &Variables,
    patterns: &mut PatternStorage,
    counters: &mut Counters,
    scripts: &ScriptStorage,
    script_index: usize,
    scale: &ScaleState,
    mut output: F,
) -> Result<()>
where
    F: FnMut(String),
{
    if parts.len() == 1 {
        output(format!("SCOPE.TIME: {}MS", scope_settings.timespan_ms));
    } else {
        let value = if let Some((val, _)) = eval_expression(parts, 1, variables, patterns, counters, scripts, script_index, scale) {
            val as u32
        } else {
            parts[1].parse().context("Failed to parse timespan value")?
        };

        if value < 5 || value > 500 {
            output("SCOPE.TIME MUST BE 5-500 MS".to_string());
            return Ok(());
        }

        scope_settings.timespan_ms = value;

        metro_tx
            .send(MetroCommand::SendScopeRate(value as f32))
            .context("Failed to send scope rate")?;

        let _ = config::save_scope_settings(scope_settings);

        output(format!("SCOPE.TIME: {}MS", value));
    }
    Ok(())
}

pub fn handle_scope_clr<F>(
    parts: &[&str],
    scope_settings: &mut crate::types::ScopeSettings,
    variables: &Variables,
    patterns: &mut PatternStorage,
    counters: &mut Counters,
    scripts: &ScriptStorage,
    script_index: usize,
    scale: &ScaleState,
    mut output: F,
) where
    F: FnMut(String),
{
    if parts.len() == 1 {
        output(format!("SCOPE.CLR: {}", scope_settings.color_mode.name()));
    } else {
        let input = parts[1];

        if let Some(mode) = crate::types::ScopeColorMode::from_str(input) {
            scope_settings.color_mode = mode;
            let _ = config::save_scope_settings(scope_settings);
            output(format!("SCOPE.CLR: {}", mode.name()));
        } else if let Some((val, _)) = eval_expression(parts, 1, variables, patterns, counters, scripts, script_index, scale) {
            if val >= 0 && val <= 8 {
                scope_settings.color_mode = crate::types::ScopeColorMode::from_u8(val as u8);
                let _ = config::save_scope_settings(scope_settings);
                output(format!("SCOPE.CLR: {} ({})", val, scope_settings.color_mode.name()));
            } else {
                output("ERROR: SCOPE.CLR NUMBER MUST BE 0-8".to_string());
            }
        } else if let Ok(val) = input.parse::<i16>() {
            if val >= 0 && val <= 8 {
                scope_settings.color_mode = crate::types::ScopeColorMode::from_u8(val as u8);
                let _ = config::save_scope_settings(scope_settings);
                output(format!("SCOPE.CLR: {} ({})", val, scope_settings.color_mode.name()));
            } else {
                output("ERROR: SCOPE.CLR NUMBER MUST BE 0-8".to_string());
            }
        } else {
            output("ERROR: INVALID COLOR NAME OR NUMBER".to_string());
            output("VALID: FOREGROUND SECONDARY HIGHLIGHT_BG".to_string());
            output("       HIGHLIGHT_FG BORDER ERROR ACCENT".to_string());
            output("       SUCCESS LABEL OR 0-8".to_string());
        }
    }
}

pub fn handle_scope_mode<F>(
    parts: &[&str],
    scope_settings: &mut crate::types::ScopeSettings,
    variables: &Variables,
    patterns: &mut PatternStorage,
    counters: &mut Counters,
    scripts: &ScriptStorage,
    script_index: usize,
    scale: &ScaleState,
    mut output: F,
) where
    F: FnMut(String),
{
    if parts.len() == 1 {
        let mode_name = match scope_settings.display_mode {
            1 => "BLOCK",
            2 => "LINE",
            3 => "DOT",
            4 => "QUADRANT",
            _ => "BRAILLE",
        };
        output(format!("SCOPE.MODE: {} ({})", scope_settings.display_mode, mode_name));
    } else {
        let value = if let Some((val, _)) = eval_expression(parts, 1, variables, patterns, counters, scripts, script_index, scale) {
            val
        } else {
            parts[1].parse().unwrap_or(-1)
        };

        match value {
            0 => {
                scope_settings.display_mode = 0;
                let _ = config::save_scope_settings(scope_settings);
                output("SCOPE.MODE: 0 (BRAILLE)".to_string());
            }
            1 => {
                scope_settings.display_mode = 1;
                let _ = config::save_scope_settings(scope_settings);
                output("SCOPE.MODE: 1 (BLOCK)".to_string());
            }
            2 => {
                scope_settings.display_mode = 2;
                let _ = config::save_scope_settings(scope_settings);
                output("SCOPE.MODE: 2 (LINE)".to_string());
            }
            3 => {
                scope_settings.display_mode = 3;
                let _ = config::save_scope_settings(scope_settings);
                output("SCOPE.MODE: 3 (DOT)".to_string());
            }
            4 => {
                scope_settings.display_mode = 4;
                let _ = config::save_scope_settings(scope_settings);
                output("SCOPE.MODE: 4 (QUADRANT)".to_string());
            }
            _ => {
                output("ERROR: SCOPE.MODE TAKES 0-4".to_string());
            }
        }
    }
}

pub fn handle_scope_uni<F>(
    parts: &[&str],
    scope_settings: &mut crate::types::ScopeSettings,
    variables: &Variables,
    patterns: &mut PatternStorage,
    counters: &mut Counters,
    scripts: &ScriptStorage,
    script_index: usize,
    scale: &ScaleState,
    mut output: F,
) where
    F: FnMut(String),
{
    if parts.len() == 1 {
        output(format!("SCOPE.UNI: {}", if scope_settings.unipolar { 1 } else { 0 }));
    } else {
        let value = if let Some((val, _)) = eval_expression(parts, 1, variables, patterns, counters, scripts, script_index, scale) {
            val
        } else {
            parts[1].parse().unwrap_or(-1)
        };

        match value {
            0 => {
                scope_settings.unipolar = false;
                let _ = config::save_scope_settings(scope_settings);
                output("SCOPE.UNI: 0 (BIPOLAR)".to_string());
            }
            1 => {
                scope_settings.unipolar = true;
                let _ = config::save_scope_settings(scope_settings);
                output("SCOPE.UNI: 1 (UNIPOLAR)".to_string());
            }
            _ => {
                output("ERROR: SCOPE.UNI TAKES 0-1".to_string());
            }
        }
    }
}

pub fn handle_scope_gain<F>(
    parts: &[&str],
    scope_settings: &mut crate::types::ScopeSettings,
    metro_tx: &Sender<MetroCommand>,
    variables: &Variables,
    patterns: &mut PatternStorage,
    counters: &mut Counters,
    scripts: &ScriptStorage,
    script_index: usize,
    scale: &ScaleState,
    mut output: F,
) -> Result<()>
where
    F: FnMut(String),
{
    if parts.len() == 1 {
        output(format!("SCOPE.GAIN: {}", scope_settings.gain));
    } else {
        let value = if let Some((val, _)) = eval_expression(parts, 1, variables, patterns, counters, scripts, script_index, scale) {
            val
        } else {
            parts[1].parse().context("Failed to parse gain value")?
        };

        if value < 0 || value > 16383 {
            output("SCOPE.GAIN MUST BE 0-16383".to_string());
            return Ok(());
        }

        scope_settings.gain = value as u16;

        let gain_f32 = value as f32 / 8192.0;
        metro_tx
            .send(MetroCommand::SendParam("scopeGain".to_string(), OscType::Float(gain_f32)))
            .context("Failed to send scope gain")?;

        let _ = config::save_scope_settings(scope_settings);

        output(format!("SCOPE.GAIN: {}", value));
    }
    Ok(())
}

pub fn handle_scope_rst<F>(
    scope_settings: &mut crate::types::ScopeSettings,
    metro_tx: &Sender<MetroCommand>,
    mut output: F,
) -> Result<()>
where
    F: FnMut(String),
{
    scope_settings.timespan_ms = 100;
    scope_settings.color_mode = crate::types::ScopeColorMode::Success;
    scope_settings.display_mode = 0;
    scope_settings.unipolar = false;
    scope_settings.gain = 8192;

    metro_tx
        .send(MetroCommand::SendScopeRate(100.0))
        .context("Failed to send scope rate")?;

    metro_tx
        .send(MetroCommand::SendParam("scopeGain".to_string(), OscType::Float(1.0)))
        .context("Failed to send scope gain")?;

    let _ = config::save_scope_settings(scope_settings);

    output("SCOPE RESET TO DEFAULTS".to_string());
    Ok(())
}

pub fn handle_note<F>(
    parts: &[&str],
    notes: &mut crate::types::NotesStorage,
    debug_level: u8,
    out_cfm: bool,
    mut output: F,
) where
    F: FnMut(String),
{
    if parts.len() < 2 {
        output("ERROR: NOTE REQUIRES QUOTED TEXT".to_string());
        return;
    }

    let joined = parts[1..].join(" ");

    let text = if (joined.starts_with('"') && joined.ends_with('"')) ||
                  (joined.starts_with('\'') && joined.ends_with('\'')) {
        if joined.len() < 2 {
            output("NOTE TEXT MUST BE QUOTED".to_string());
            return;
        }
        joined[1..joined.len()-1].to_string()
    } else {
        output("NOTE TEXT MUST BE QUOTED".to_string());
        return;
    };

    let mut found_empty = false;
    for i in 0..8 {
        if notes.lines[i].is_empty() {
            notes.lines[i] = text.clone();
            found_empty = true;
            if debug_level >= TIER_CONFIRMS || out_cfm {
                output(format!("NOTE ADDED TO LINE {}", i + 1));
            }
            break;
        }
    }

    if !found_empty {
        output("NOTES PAGE FULL (8 LINES MAX)".to_string());
    }
}

pub fn handle_note_clr<F>(
    notes: &mut crate::types::NotesStorage,
    debug_level: u8,
    out_cfm: bool,
    mut output: F,
) where
    F: FnMut(String),
{
    for i in 0..8 {
        notes.lines[i].clear();
    }
    if debug_level >= TIER_CONFIRMS || out_cfm {
        output("NOTES CLEARED".to_string());
    }
}

define_bool_toggle!(handle_meter_hdr, "METER.HDR", "HEADER METERS: {}", "HEADER METERS: OFF", "HEADER METERS: ON", config::save_show_meters_header);

define_bool_toggle!(handle_meter_grid, "METER.GRID", "GRID METERS: {}", "GRID METERS: OFF", "GRID METERS: ON", config::save_show_meters_grid);

define_bool_toggle!(handle_meter_ascii, "METER.ASCII", "ASCII METERS: {}", "ASCII METERS: OFF", "ASCII METERS: ON", config::save_ascii_meters);

define_bool_toggle!(handle_spectrum, "SPECTRUM", "SPECTRUM: {}", "SPECTRUM: OFF", "SPECTRUM: ON", config::save_show_spectrum);

define_bool_toggle!(handle_activity, "ACTIVITY", "ACTIVITY: {}", "ACTIVITY: OFF", "ACTIVITY: ON", config::save_show_activity);

define_bool_toggle!(handle_grid, "GRID", "GRID: {}", "GRID: OFF", "GRID: ON", config::save_show_grid);

define_bool_toggle!(handle_grid_def, "GRID.DEF", "GRID.DEF: {}", "GRID.DEF: 0 (REPL)", "GRID.DEF: 1 (GRID)", config::save_show_grid_view);

define_bool_toggle!(handle_hl_seq, "HL.SEQ", "SEQ HIGHLIGHT: {}", "SEQ HIGHLIGHT: OFF", "SEQ HIGHLIGHT: ON", config::save_show_seq_highlight);

define_enum_select!(
    handle_grid_mode,
    "GRID.MODE",
    config::save_grid_mode,
    "ERROR: GRID.MODE TAKES 0-5",
    (0, "LABELS"),
    (1, "ICONS"),
    (2, "FX VIZ"),
    (3, "MIXER"),
    (4, "FX VIZ 2"),
    (5, "SAMPLER"),
);

pub fn handle_title<F>(
    parts: &[&str],
    title_mode: &mut u8,
    current_scene_name: &Option<String>,
    scramble_enabled: bool,
    scramble_mode: u8,
    scramble_speed: u8,
    scramble_curve: u8,
    header_scramble: &mut Option<crate::scramble::ScrambleAnimation>,
    mut output: F,
) where
    F: FnMut(String),
{
    if parts.len() == 1 {
        let mode_name = if *title_mode == 0 { "MONOKIT" } else { "SCENE" };
        output(format!("TITLE: {} ({})", title_mode, mode_name));
    } else {
        let value = parts[1];
        match value {
            "0" => {
                *title_mode = 0;
                let _ = config::save_title_mode(*title_mode);
                *header_scramble = if scramble_enabled {
                    let mode = crate::scramble::ScrambleMode::from_u8(scramble_mode);
                    let curve = crate::scramble::ScrambleCurve::from_u8(scramble_curve);
                    Some(crate::scramble::ScrambleAnimation::new_with_options("MONOKIT", mode, scramble_speed, curve))
                } else {
                    None
                };
                output("TITLE: 0 (MONOKIT)".to_string());
            }
            "1" => {
                *title_mode = 1;
                let _ = config::save_title_mode(*title_mode);
                let text = current_scene_name.as_ref().map(|s| s.as_str()).unwrap_or("[UNSAVED]");
                *header_scramble = if scramble_enabled {
                    let mode = crate::scramble::ScrambleMode::from_u8(scramble_mode);
                    let curve = crate::scramble::ScrambleCurve::from_u8(scramble_curve);
                    Some(crate::scramble::ScrambleAnimation::new_with_options(text, mode, scramble_speed, curve))
                } else {
                    None
                };
                output("TITLE: 1 (SCENE NAME)".to_string());
            }
            _ => {
                output("ERROR: TITLE TAKES 0 (MONOKIT) OR 1 (SCENE)".to_string());
            }
        }
    }
}

define_bool_toggle!(handle_out_err, "OUT.ERR", config::save_out_err);

define_bool_toggle!(handle_out_ess, "OUT.ESS", config::save_out_ess);

define_bool_toggle!(handle_out_qry, "OUT.QRY", config::save_out_qry);

define_bool_toggle!(handle_out_cfm, "OUT.CFM", config::save_out_cfm);

define_bool_toggle!(handle_scrmbl, "SCRMBL", config::save_scramble_enabled);

define_enum_select!(
    handle_scrmbl_mode,
    "SCRMBL.MODE",
    config::save_scramble_mode,
    "ERROR: SCRMBL.MODE TAKES 0-3",
    (0, "REGULAR"),
    (1, "SMASH"),
    (2, "ROLLING"),
    (3, "OVERSHOOT"),
);

define_enum_select!(
    handle_scrmbl_spd,
    "SCRMBL.SPD",
    config::save_scramble_speed,
    "ERROR: SCRMBL.SPD TAKES 1-10",
    1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
);

define_enum_select!(
    handle_scrmbl_crv,
    "SCRMBL.CRV",
    config::save_scramble_curve,
    "ERROR: SCRMBL.CRV TAKES 0 (LINEAR) OR 1 (SETTLE)",
    (0, "LINEAR"),
    (1, "SETTLE"),
);

pub fn handle_title_timer<F>(
    parts: &[&str],
    title_timer_enabled: &mut bool,
    title_timer_interval_secs: &mut u16,
    title_timer_last_toggle: &mut Option<std::time::Instant>,
    title_mode: &mut u8,
    current_scene_name: &Option<String>,
    scramble_enabled: bool,
    scramble_mode: u8,
    scramble_speed: u8,
    scramble_curve: u8,
    header_scramble: &mut Option<crate::scramble::ScrambleAnimation>,
    variables: &Variables,
    patterns: &mut PatternStorage,
    counters: &mut Counters,
    scripts: &ScriptStorage,
    script_index: usize,
    scale: &ScaleState,
    mut output: F,
) where
    F: FnMut(String),
{
    if parts.len() == 1 {
        if *title_timer_enabled {
            output(format!("TITLE.TIMER: ON ({}S)", *title_timer_interval_secs));
        } else {
            output("TITLE.TIMER: OFF".to_string());
        }
    } else {
        let value: i16 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index, scale) {
            expr_val
        } else {
            parts[1].parse().unwrap_or(-1)
        };
        match value {
            0 => {
                *title_timer_enabled = false;
                *title_timer_last_toggle = None;
                let _ = config::save_title_timer_enabled(false);
                output("TITLE.TIMER: OFF".to_string());
            }
            1 => {
                if parts.len() < 3 {
                    output("TITLE.TIMER 1 REQUIRES SECONDS (1-1800)".to_string());
                    return;
                }
                let secs: i16 = if let Some((expr_val, _)) = eval_expression(&parts, 2, variables, patterns, counters, scripts, script_index, scale) {
                    expr_val
                } else {
                    parts[2].parse().unwrap_or(-1)
                };
                if secs < 1 || secs > 1800 {
                    output("TITLE.TIMER SECONDS MUST BE 1-1800".to_string());
                    return;
                }
                let secs_u16 = secs as u16;
                *title_timer_enabled = true;
                *title_timer_interval_secs = secs_u16;
                *title_timer_last_toggle = Some(std::time::Instant::now());
                let _ = config::save_title_timer_enabled(true);
                let _ = config::save_title_timer_interval_secs(secs_u16);

                let text = if *title_mode == 0 {
                    "MONOKIT"
                } else {
                    current_scene_name.as_ref().map(|s| s.as_str()).unwrap_or("[UNSAVED]")
                };
                *header_scramble = if scramble_enabled {
                    let mode = crate::scramble::ScrambleMode::from_u8(scramble_mode);
                    let curve = crate::scramble::ScrambleCurve::from_u8(scramble_curve);
                    Some(crate::scramble::ScrambleAnimation::new_with_options(text, mode, scramble_speed, curve))
                } else {
                    None
                };

                output(format!("TITLE.TIMER: ON ({}S)", secs_u16));
            }
            _ => {
                output("ERROR: TITLE.TIMER TAKES 0 (OFF) OR 1 <SECONDS>".to_string());
            }
        }
    }
}

pub fn handle_compat<F>(
    terminal_caps: &crate::terminal::TerminalCapabilities,
    color_mode: crate::types::ColorMode,
    mut output: F,
) where
    F: FnMut(String),
{
    let term = terminal_caps.term_program.as_deref().unwrap_or("unknown");
    let color = match color_mode {
        crate::types::ColorMode::TrueColor => "TRUECOLOR (24-BIT)",
        crate::types::ColorMode::Color256 => "256-COLOR",
    };
    output(format!("TERMINAL: {}", term.to_uppercase()));
    output(format!("COLOR MODE: {}", color));
    if !terminal_caps.true_color {
        output("TIP: USE ITERM2 FOR BEST RESULTS".to_string());
    }
}

pub fn handle_compat_mode<F>(
    parts: &[&str],
    ascii_meters: &mut bool,
    scope_settings: &mut crate::types::ScopeSettings,
    mut output: F,
) where
    F: FnMut(String),
{
    if parts.len() == 1 {
        let mode = if *ascii_meters { 1 } else { 0 };
        output(format!("COMPAT.MODE: {}", mode));
        return;
    }

    match parts[1] {
        "0" => {
            *ascii_meters = false;
            let _ = config::save_ascii_meters(false);
            output("COMPAT.MODE: 0 (FULL)".to_string());
        }
        "1" => {
            *ascii_meters = true;
            scope_settings.display_mode = 1;
            let _ = config::save_ascii_meters(true);
            let _ = config::save_scope_settings(scope_settings);
            output("COMPAT.MODE: 1 (BASIC)".to_string());
        }
        _ => output("ERROR: COMPAT.MODE TAKES 0 OR 1".to_string()),
    }
}

fn parse_script_id(id_str: &str) -> Option<usize> {
    let upper = id_str.to_uppercase();
    match upper.as_str() {
        "M" => Some(8),
        "I" => Some(9),
        _ => {
            if let Ok(num) = id_str.parse::<usize>() {
                if num >= 1 && num <= 8 {
                    Some(num - 1)
                } else {
                    None
                }
            } else {
                None
            }
        }
    }
}

fn script_id_label(index: usize) -> String {
    match index {
        0..=7 => format!("{}", index + 1),
        8 => "M".to_string(),
        9 => "I".to_string(),
        _ => format!("{}", index),
    }
}

pub fn handle_mute<F>(
    parts: &[&str],
    script_mutes: &mut crate::types::ScriptMutes,
    variables: &Variables,
    patterns: &mut PatternStorage,
    counters: &mut Counters,
    scripts: &ScriptStorage,
    script_index: usize,
    scale: &ScaleState,
    debug_level: u8,
    out_qry: bool,
    out_cfm: bool,
    mut output: F,
) where
    F: FnMut(String),
{
    use crate::eval::eval_expression;
    use crate::types::{TIER_CONFIRMS, TIER_QUERIES};

    if parts.len() == 1 {
        if debug_level >= TIER_QUERIES || out_qry {
            output("SCRIPT MUTES:".to_string());
            for i in 0..10 {
                let label = script_id_label(i);
                let status = if script_mutes.muted[i] { "MUTED" } else { "ACTIVE" };
                output(format!("  {}: {}", label, status));
            }
        }
        return;
    }

    let Some(index) = parse_script_id(parts[1]) else {
        output("ERROR: INVALID SCRIPT ID (USE 1-8, M, I)".to_string());
        return;
    };

    if parts.len() == 2 {
        script_mutes.muted[index] = !script_mutes.muted[index];
        let label = script_id_label(index);
        let status = if script_mutes.muted[index] { "MUTED" } else { "ACTIVE" };
        if debug_level >= TIER_CONFIRMS || out_cfm {
            output(format!("SCRIPT {}: {}", label, status));
        }
        return;
    }

    if let Some((val, _)) = eval_expression(&parts, 2, variables, patterns, counters, scripts, script_index, scale) {
        script_mutes.muted[index] = val != 0;
        let label = script_id_label(index);
        let status = if script_mutes.muted[index] { "MUTED" } else { "ACTIVE" };
        if debug_level >= TIER_CONFIRMS || out_cfm {
            output(format!("SCRIPT {}: {}", label, status));
        }
    } else {
        output("ERROR: INVALID MUTE VALUE EXPRESSION".to_string());
    }
}

pub fn handle_page<F>(
    parts: &[&str],
    current_page: &mut crate::types::Page,
    show_grid_view: &mut bool,
    debug_level: u8,
    out_cfm: bool,
    mut output: F,
) where
    F: FnMut(String),
{
    use crate::types::{Page, TIER_CONFIRMS};

    if parts.len() < 2 {
        output("ERROR: PAGE REQUIRES PAGE NAME OR NUMBER".to_string());
        return;
    }

    let page_arg = parts[1].to_uppercase();
    let target_page = match page_arg.as_str() {
        "LIVE" | "L" => Page::Live,
        "HELP" | "H" => Page::Help,
        "GRID" | "G" => {
            *show_grid_view = true;
            Page::Live
        }
        "1" => Page::Script1,
        "2" => Page::Script2,
        "3" => Page::Script3,
        "4" => Page::Script4,
        "5" => Page::Script5,
        "6" => Page::Script6,
        "7" => Page::Script7,
        "8" => Page::Script8,
        "M" => Page::Metro,
        "I" => Page::Init,
        "P" => Page::Pattern,
        "V" => Page::Variables,
        "N" => Page::Notes,
        "S" => Page::Scope,
        _ => {
            output(format!("ERROR: INVALID PAGE \"{}\"", page_arg));
            return;
        }
    };

    *current_page = target_page;

    if debug_level >= TIER_CONFIRMS || out_cfm {
        output(format!("PAGE: {}", current_page.name()));
    }
}

define_bool_toggle!(handle_cfm_quit, "CFM.QUIT", "CFM.QUIT: {}", "CFM.QUIT: OFF", "CFM.QUIT: ON (CONFIRM QUIT)", config::save_confirm_quit_unsaved);

define_bool_toggle!(handle_cfm_save, "CFM.SAVE", "CFM.SAVE: {}", "CFM.SAVE: OFF", "CFM.SAVE: ON (CONFIRM OVERWRITE)", config::save_confirm_overwrite_scene);
