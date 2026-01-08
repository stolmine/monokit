use crate::commands::context::ExecutionContext;
use crate::commands::OutputDecider;
use crate::types::{MetroCommand, OutputCategory};
use anyhow::{Context, Result};
use rosc::OscType;
use std::time::Duration;

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
    ctx.metro_tx.send(MetroCommand::SendParam("s_volume".to_string(), OscType::Int(16383)))?; // Max - use vol_smp for actual control
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

    *ctx.sampler_state = crate::types::SamplerState::default();
    crate::eval::KIT_SLOTS.store(0, std::sync::atomic::Ordering::Relaxed);
    *ctx.mixer_data = crate::types::MixerData::default();

    ctx.output(
        OutputCategory::Essential,
        "RESET TO DEFAULTS".to_string(),
        &mut output,
    );
    Ok(())
}
