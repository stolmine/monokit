use crate::eval::eval_expression;
use crate::types::{Counters, MetroCommand, PatternStorage, ScaleState, ScriptStorage, Variables, TIER_ERRORS, TIER_ESSENTIAL, TIER_QUERIES, TIER_CONFIRMS};
use anyhow::{Context, Result};
use rand::Rng;
use rosc::OscType;
use std::sync::mpsc::Sender;

pub fn handle_rnd_voice<F>(
    metro_tx: &Sender<MetroCommand>,
    debug_level: u8,
    mut output: F,
) -> Result<()>
where
    F: FnMut(String),
{
    let mut rng = rand::thread_rng();

    let pf = rng.gen_range(50.0..=2000.0);
    let pw = rng.gen_range(0..=2);
    let mf = rng.gen_range(20.0..=1000.0);
    let mw = rng.gen_range(0..=3);
    let fm = rng.gen_range(0..=8000);
    let fb = rng.gen_range(0..=4000);
    let fba = rng.gen_range(0..=8000);
    let fbd = rng.gen_range(10..=2000);

    metro_tx.send(MetroCommand::SendParam("pf".to_string(), OscType::Float(pf)))?;
    metro_tx.send(MetroCommand::SendParam("pw".to_string(), OscType::Int(pw)))?;
    metro_tx.send(MetroCommand::SendParam("mf".to_string(), OscType::Float(mf)))?;
    metro_tx.send(MetroCommand::SendParam("mw".to_string(), OscType::Int(mw)))?;
    metro_tx.send(MetroCommand::SendParam("fm".to_string(), OscType::Int(fm)))?;
    metro_tx.send(MetroCommand::SendParam("fb".to_string(), OscType::Int(fb)))?;
    metro_tx.send(MetroCommand::SendParam("fba".to_string(), OscType::Int(fba)))?;
    metro_tx.send(MetroCommand::SendParam("fbd".to_string(), OscType::Int(fbd)))?;

    if debug_level >= TIER_CONFIRMS {
        output(format!("RND.VOICE: PF={:.1} PW={} MF={:.1} MW={} FM={} FB={}", pf, pw, mf, mw, fm, fb));
    }

    Ok(())
}

pub fn handle_rnd_osc<F>(
    metro_tx: &Sender<MetroCommand>,
    debug_level: u8,
    mut output: F,
) -> Result<()>
where
    F: FnMut(String),
{
    let mut rng = rand::thread_rng();

    let pf = rng.gen_range(50.0..=2000.0);
    let pw = rng.gen_range(0..=2);
    let mf = rng.gen_range(20.0..=1000.0);
    let mw = rng.gen_range(0..=3);

    metro_tx.send(MetroCommand::SendParam("pf".to_string(), OscType::Float(pf)))?;
    metro_tx.send(MetroCommand::SendParam("pw".to_string(), OscType::Int(pw)))?;
    metro_tx.send(MetroCommand::SendParam("mf".to_string(), OscType::Float(mf)))?;
    metro_tx.send(MetroCommand::SendParam("mw".to_string(), OscType::Int(mw)))?;

    if debug_level >= TIER_CONFIRMS {
        output(format!("RND.OSC: PF={:.1} PW={} MF={:.1} MW={}", pf, pw, mf, mw));
    }

    Ok(())
}

pub fn handle_rnd_fm<F>(
    metro_tx: &Sender<MetroCommand>,
    debug_level: u8,
    mut output: F,
) -> Result<()>
where
    F: FnMut(String),
{
    let mut rng = rand::thread_rng();

    let fm = rng.gen_range(0..=8000);
    let fb = rng.gen_range(0..=4000);
    let fba = rng.gen_range(0..=8000);
    let fbd = rng.gen_range(10..=2000);

    metro_tx.send(MetroCommand::SendParam("fm".to_string(), OscType::Int(fm)))?;
    metro_tx.send(MetroCommand::SendParam("fb".to_string(), OscType::Int(fb)))?;
    metro_tx.send(MetroCommand::SendParam("fba".to_string(), OscType::Int(fba)))?;
    metro_tx.send(MetroCommand::SendParam("fbd".to_string(), OscType::Int(fbd)))?;

    if debug_level >= TIER_CONFIRMS {
        output("RANDOMIZED FM".to_string());
    }

    Ok(())
}

pub fn handle_rnd_mod<F>(
    metro_tx: &Sender<MetroCommand>,
    debug_level: u8,
    mut output: F,
) -> Result<()>
where
    F: FnMut(String),
{
    let mut rng = rand::thread_rng();

    let mb = rng.gen_range(0..=8000);
    let tk = rng.gen_range(0..=8000);
    let mp = rng.gen_range(0..=1);
    let md = rng.gen_range(0..=1);
    let mt = rng.gen_range(0..=1);
    let ma = rng.gen_range(0..=1);

    metro_tx.send(MetroCommand::SendParam("mb".to_string(), OscType::Int(mb)))?;
    metro_tx.send(MetroCommand::SendParam("tk".to_string(), OscType::Int(tk)))?;
    metro_tx.send(MetroCommand::SendParam("mp".to_string(), OscType::Int(mp)))?;
    metro_tx.send(MetroCommand::SendParam("md".to_string(), OscType::Int(md)))?;
    metro_tx.send(MetroCommand::SendParam("mt".to_string(), OscType::Int(mt)))?;
    metro_tx.send(MetroCommand::SendParam("ma".to_string(), OscType::Int(ma)))?;

    if debug_level >= TIER_CONFIRMS {
        output(format!("RANDOMIZED MODULATION MB={} TK={} MP={} MD={} MT={} MA={}", mb, tk, mp, md, mt, ma));
    }

    Ok(())
}

pub fn handle_rnd_env<F>(
    metro_tx: &Sender<MetroCommand>,
    debug_level: u8,
    mut output: F,
) -> Result<()>
where
    F: FnMut(String),
{
    let mut rng = rand::thread_rng();

    let env_atk = rng.gen_range(5..=2000);
    let env_dec = rng.gen_range(5..=2000);
    let env_crv = rng.gen_range(-4.0..=4.0);
    let env_mode = rng.gen_range(0..=2);

    let pa = rng.gen_range(0.0..=8.0);
    let fa = rng.gen_range(0..=8);
    let da = rng.gen_range(0..=8);

    metro_tx.send(MetroCommand::SendParam("env_atk".to_string(), OscType::Int(env_atk)))?;
    metro_tx.send(MetroCommand::SendParam("env_dec".to_string(), OscType::Int(env_dec)))?;
    metro_tx.send(MetroCommand::SendParam("env_crv".to_string(), OscType::Float(env_crv)))?;
    metro_tx.send(MetroCommand::SendParam("env_mode".to_string(), OscType::Int(env_mode)))?;

    metro_tx.send(MetroCommand::SendParam("pa".to_string(), OscType::Float(pa)))?;
    metro_tx.send(MetroCommand::SendParam("fa".to_string(), OscType::Int(fa)))?;
    metro_tx.send(MetroCommand::SendParam("da".to_string(), OscType::Int(da)))?;

    if debug_level >= TIER_CONFIRMS {
        output(format!("RANDOMIZED ENVELOPES ATK={} DEC={} CRV={:.2} PA={:.2} FA={} DA={}", env_atk, env_dec, env_crv, pa, fa, da));
    }

    Ok(())
}

pub fn handle_rnd_p<F>(
    parts: &[&str],
    variables: &Variables,
    patterns: &mut PatternStorage,
    counters: &mut Counters,
    scripts: &ScriptStorage,
    script_index: usize,
    scale: &ScaleState,
    debug_level: u8,
    mut output: F,
) -> Result<()>
where
    F: FnMut(String),
{
    use crate::types::TIER_CONFIRMS;

    let (mut min, mut max) = if parts.len() >= 3 {
        let min_val: i16 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index, scale) {
            expr_val
        } else {
            parts[1]
                .parse()
                .context("Failed to parse min value")?
        };
        let max_val: i16 = if let Some((expr_val, _)) = eval_expression(&parts, 2, variables, patterns, counters, scripts, script_index, scale) {
            expr_val
        } else {
            parts[2]
                .parse()
                .context("Failed to parse max value")?
        };
        (min_val, max_val)
    } else {
        (0, 127)
    };
    if min > max {
        std::mem::swap(&mut min, &mut max);
    }
    let pattern = &mut patterns.patterns[patterns.working];
    let mut rng = rand::thread_rng();
    for i in 0..pattern.length {
        pattern.data[i] = rng.gen_range(min..=max);
    }

    if debug_level >= TIER_CONFIRMS {
        output(format!("RANDOMIZED PATTERN {} RANGE {} TO {}", patterns.working, min, max));
    }

    Ok(())
}

pub fn handle_rnd_pn<F>(
    parts: &[&str],
    variables: &Variables,
    patterns: &mut PatternStorage,
    counters: &mut Counters,
    scripts: &ScriptStorage,
    script_index: usize,
    scale: &ScaleState,
    debug_level: u8,
    mut output: F,
) -> Result<()>
where
    F: FnMut(String),
{
    if parts.len() < 2 {
        output("ERROR: RND.PN REQUIRES PATTERN NUMBER (0-5)".to_string());
        return Ok(());
    }
    let pat: usize = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index, scale) {
        if expr_val < 0 || expr_val > 5 {
            output("ERROR: PATTERN NUMBER MUST BE 0-5".to_string());
            return Ok(());
        }
        expr_val as usize
    } else {
        parts[1]
            .parse()
            .context("Failed to parse pattern number")?
    };
    if pat > 5 {
        output("ERROR: PATTERN NUMBER MUST BE 0-5".to_string());
        return Ok(());
    }
    let (mut min, mut max) = if parts.len() >= 4 {
        let min_val: i16 = if let Some((expr_val, _)) = eval_expression(&parts, 2, variables, patterns, counters, scripts, script_index, scale) {
            expr_val
        } else {
            parts[2]
                .parse()
                .context("Failed to parse min value")?
        };
        let max_val: i16 = if let Some((expr_val, _)) = eval_expression(&parts, 3, variables, patterns, counters, scripts, script_index, scale) {
            expr_val
        } else {
            parts[3]
                .parse()
                .context("Failed to parse max value")?
        };
        (min_val, max_val)
    } else {
        (0, 127)
    };
    if min > max {
        std::mem::swap(&mut min, &mut max);
    }
    let pattern = &mut patterns.patterns[pat];
    let mut rng = rand::thread_rng();
    for i in 0..pattern.length {
        pattern.data[i] = rng.gen_range(min..=max);
    }

    use crate::types::TIER_CONFIRMS;
    if debug_level >= TIER_CONFIRMS {
        output(format!("RANDOMIZED PATTERN {} RANGE {} TO {}", pat, min, max));
    }

    Ok(())
}

pub fn handle_rnd_pall<F>(
    parts: &[&str],
    variables: &Variables,
    patterns: &mut PatternStorage,
    counters: &mut Counters,
    scripts: &ScriptStorage,
    script_index: usize,
    scale: &ScaleState,
    debug_level: u8,
    mut output: F,
) -> Result<()>
where
    F: FnMut(String),
{
    let (mut min, mut max) = if parts.len() >= 3 {
        let min_val: i16 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index, scale) {
            expr_val
        } else {
            parts[1]
                .parse()
                .context("Failed to parse min value")?
        };
        let max_val: i16 = if let Some((expr_val, _)) = eval_expression(&parts, 2, variables, patterns, counters, scripts, script_index, scale) {
            expr_val
        } else {
            parts[2]
                .parse()
                .context("Failed to parse max value")?
        };
        (min_val, max_val)
    } else {
        (0, 127)
    };
    if min > max {
        std::mem::swap(&mut min, &mut max);
    }
    let mut rng = rand::thread_rng();
    for pat_idx in 0..6 {
        let pattern = &mut patterns.patterns[pat_idx];
        for i in 0..pattern.length {
            pattern.data[i] = rng.gen_range(min..=max);
        }
    }

    use crate::types::TIER_CONFIRMS;
    if debug_level >= TIER_CONFIRMS {
        output(format!("RANDOMIZED ALL PATTERNS RANGE {} TO {}", min, max));
    }

    Ok(())
}

pub fn handle_rnd_fx<F>(
    metro_tx: &Sender<MetroCommand>,
    debug_level: u8,
    mut output: F,
) -> Result<()>
where
    F: FnMut(String),
{
    let mut rng = rand::thread_rng();

    // Filter
    let fc = rng.gen_range(200.0..=8000.0);
    metro_tx.send(MetroCommand::SendParam("fc".to_string(), OscType::Float(fc)))?;
    let fq = rng.gen_range(0..=8000);
    metro_tx.send(MetroCommand::SendParam("fq".to_string(), OscType::Int(fq)))?;
    let ft = rng.gen_range(0..=13);
    metro_tx.send(MetroCommand::SendParam("ft".to_string(), OscType::Int(ft)))?;
    let fe = rng.gen_range(0..=8000);
    metro_tx.send(MetroCommand::SendParam("fe".to_string(), OscType::Int(fe)))?;

    // Lo-Fi
    let lb = rng.gen_range(4..=16);
    metro_tx.send(MetroCommand::SendParam("lb".to_string(), OscType::Int(lb)))?;
    let ls = rng.gen_range(2000..=48000);
    metro_tx.send(MetroCommand::SendParam("ls".to_string(), OscType::Int(ls)))?;
    let lm = rng.gen_range(0..=8000);
    metro_tx.send(MetroCommand::SendParam("lm".to_string(), OscType::Int(lm)))?;

    // Ring Mod
    let rgf = rng.gen_range(20.0..=2000.0);
    metro_tx.send(MetroCommand::SendParam("rgf".to_string(), OscType::Float(rgf)))?;
    let rgw = rng.gen_range(0..=3);
    metro_tx.send(MetroCommand::SendParam("rgw".to_string(), OscType::Int(rgw)))?;
    let rgm = rng.gen_range(0..=8000);
    metro_tx.send(MetroCommand::SendParam("rgm".to_string(), OscType::Int(rgm)))?;

    // Resonator
    let rf = rng.gen_range(50.0..=2000.0);
    metro_tx.send(MetroCommand::SendParam("rf".to_string(), OscType::Float(rf)))?;
    let rd = rng.gen_range(50..=2000);
    metro_tx.send(MetroCommand::SendParam("rd".to_string(), OscType::Int(rd)))?;
    let rm = rng.gen_range(0..=8000);
    metro_tx.send(MetroCommand::SendParam("rm".to_string(), OscType::Int(rm)))?;

    // Delay
    let dt = rng.gen_range(50..=500);
    metro_tx.send(MetroCommand::SendParam("dt".to_string(), OscType::Int(dt)))?;
    let df = rng.gen_range(0..=10000);
    metro_tx.send(MetroCommand::SendParam("df".to_string(), OscType::Int(df)))?;
    let dlp = rng.gen_range(1000.0..=15000.0);
    metro_tx.send(MetroCommand::SendParam("dlp".to_string(), OscType::Float(dlp)))?;
    let dw = rng.gen_range(0..=8000);
    metro_tx.send(MetroCommand::SendParam("dw".to_string(), OscType::Int(dw)))?;

    // EQ
    let el = rng.gen_range(-12..=12);
    metro_tx.send(MetroCommand::SendParam("el".to_string(), OscType::Int(el)))?;
    let em = rng.gen_range(-12..=12);
    metro_tx.send(MetroCommand::SendParam("em".to_string(), OscType::Int(em)))?;
    let eh = rng.gen_range(-12..=12);
    metro_tx.send(MetroCommand::SendParam("eh".to_string(), OscType::Int(eh)))?;

    // Compressor (favor wet side for parallel compression)
    let cr_mix = rng.gen_range(8192..=16383);
    metro_tx.send(MetroCommand::SendParam("cr_mix".to_string(), OscType::Int(cr_mix)))?;

    // Reverb
    let rv = rng.gen_range(0..=10000);
    metro_tx.send(MetroCommand::SendParam("rv".to_string(), OscType::Int(rv)))?;
    let rp = rng.gen_range(0..=50);
    metro_tx.send(MetroCommand::SendParam("rp".to_string(), OscType::Int(rp)))?;
    let rh = rng.gen_range(0..=10000);
    metro_tx.send(MetroCommand::SendParam("rh".to_string(), OscType::Int(rh)))?;
    let rw = rng.gen_range(0..=8000);
    metro_tx.send(MetroCommand::SendParam("rw".to_string(), OscType::Int(rw)))?;

    // MiClouds Granular Effect
    let cl_pitch = rng.gen_range(0..=16383);
    metro_tx.send(MetroCommand::SendParam("cl_pitch".to_string(), OscType::Int(cl_pitch)))?;
    let cl_pos = rng.gen_range(0..=16383);
    metro_tx.send(MetroCommand::SendParam("cl_pos".to_string(), OscType::Int(cl_pos)))?;
    let cl_size = rng.gen_range(4096..=12288);  // Bias toward medium grain sizes
    metro_tx.send(MetroCommand::SendParam("cl_size".to_string(), OscType::Int(cl_size)))?;
    let cl_dens = rng.gen_range(4096..=12288);  // Bias toward medium density
    metro_tx.send(MetroCommand::SendParam("cl_dens".to_string(), OscType::Int(cl_dens)))?;
    let cl_tex = rng.gen_range(0..=16383);
    metro_tx.send(MetroCommand::SendParam("cl_tex".to_string(), OscType::Int(cl_tex)))?;
    let cl_wet = rng.gen_range(0..=8192);  // Max 50% wet to avoid overwhelming mix
    metro_tx.send(MetroCommand::SendParam("cl_wet".to_string(), OscType::Int(cl_wet)))?;
    // Keep gain at unity for safety
    metro_tx.send(MetroCommand::SendParam("cl_gain".to_string(), OscType::Int(8192)))?;
    let cl_spread = rng.gen_range(0..=16383);
    metro_tx.send(MetroCommand::SendParam("cl_spread".to_string(), OscType::Int(cl_spread)))?;
    let cl_rvb = rng.gen_range(0..=8192);  // Max 50% internal reverb
    metro_tx.send(MetroCommand::SendParam("cl_rvb".to_string(), OscType::Int(cl_rvb)))?;
    let cl_fb = rng.gen_range(0..=6553);  // Max ~40% feedback to be safe
    metro_tx.send(MetroCommand::SendParam("cl_fb".to_string(), OscType::Int(cl_fb)))?;
    // Don't randomize freeze - leave at 0
    metro_tx.send(MetroCommand::SendParam("cl_freeze".to_string(), OscType::Int(0)))?;
    let cl_mode = rng.gen_range(0..=3);
    metro_tx.send(MetroCommand::SendParam("cl_mode".to_string(), OscType::Int(cl_mode)))?;
    let cl_lofi = rng.gen_range(0..=4096);  // Light lo-fi effect
    metro_tx.send(MetroCommand::SendParam("cl_lofi".to_string(), OscType::Int(cl_lofi)))?;

    use crate::types::TIER_CONFIRMS;
    if debug_level >= TIER_CONFIRMS {
        output("RANDOMIZED FX".to_string());
    }

    Ok(())
}

pub fn handle_rnd_filt<F>(
    metro_tx: &Sender<MetroCommand>,
    debug_level: u8,
    mut output: F,
) -> Result<()>
where
    F: FnMut(String),
{
    let mut rng = rand::thread_rng();

    let fc = rng.gen_range(200.0..=8000.0);
    metro_tx.send(MetroCommand::SendParam("fc".to_string(), OscType::Float(fc)))?;

    let fq = rng.gen_range(0..=8000);
    metro_tx.send(MetroCommand::SendParam("fq".to_string(), OscType::Int(fq)))?;

    let ft = rng.gen_range(0..=13);
    metro_tx.send(MetroCommand::SendParam("ft".to_string(), OscType::Int(ft)))?;

    let fe = rng.gen_range(0..=8000);
    metro_tx.send(MetroCommand::SendParam("fe".to_string(), OscType::Int(fe)))?;

    use crate::types::TIER_CONFIRMS;
    if debug_level >= TIER_CONFIRMS {
        output("RANDOMIZED FILTER".to_string());
    }

    Ok(())
}

pub fn handle_rnd_dly<F>(
    metro_tx: &Sender<MetroCommand>,
    debug_level: u8,
    mut output: F,
) -> Result<()>
where
    F: FnMut(String),
{
    let mut rng = rand::thread_rng();

    let dt = rng.gen_range(50..=500);
    metro_tx.send(MetroCommand::SendParam("dt".to_string(), OscType::Int(dt)))?;

    let df = rng.gen_range(0..=10000);
    metro_tx.send(MetroCommand::SendParam("df".to_string(), OscType::Int(df)))?;

    let dlp = rng.gen_range(1000.0..=15000.0);
    metro_tx.send(MetroCommand::SendParam("dlp".to_string(), OscType::Float(dlp)))?;

    let dw = rng.gen_range(0..=8000);
    metro_tx.send(MetroCommand::SendParam("dw".to_string(), OscType::Int(dw)))?;

    use crate::types::TIER_CONFIRMS;
    if debug_level >= TIER_CONFIRMS {
        output("RANDOMIZED DELAY".to_string());
    }

    Ok(())
}

pub fn handle_rnd_verb<F>(
    metro_tx: &Sender<MetroCommand>,
    debug_level: u8,
    mut output: F,
) -> Result<()>
where
    F: FnMut(String),
{
    let mut rng = rand::thread_rng();

    let rv = rng.gen_range(0..=10000);
    metro_tx.send(MetroCommand::SendParam("rv".to_string(), OscType::Int(rv)))?;

    let rp = rng.gen_range(0..=50);
    metro_tx.send(MetroCommand::SendParam("rp".to_string(), OscType::Int(rp)))?;

    let rh = rng.gen_range(0..=10000);
    metro_tx.send(MetroCommand::SendParam("rh".to_string(), OscType::Int(rh)))?;

    let rw = rng.gen_range(0..=8000);
    metro_tx.send(MetroCommand::SendParam("rw".to_string(), OscType::Int(rw)))?;

    use crate::types::TIER_CONFIRMS;
    if debug_level >= TIER_CONFIRMS {
        output("RANDOMIZED REVERB".to_string());
    }

    Ok(())
}

pub fn handle_rnd_pl<F>(
    metro_tx: &Sender<MetroCommand>,
    debug_level: u8,
    mut output: F,
) -> Result<()>
where
    F: FnMut(String),
{
    use crate::types::PLAITS_NODE_ID;
    let mut rng = rand::thread_rng();

    let engine = rng.gen_range(0..=15);
    let harmonics = rng.gen_range(0..=16383);
    let timbre = rng.gen_range(0..=16383);
    let morph = rng.gen_range(0..=16383);
    let decay = rng.gen_range(0..=16383);
    let lpg = rng.gen_range(0..=16383);

    // Scale 0-16383 values to 0.0-1.0 floats for SynthDef
    metro_tx.send(MetroCommand::SendParam("engine".to_string(), OscType::Int(engine)))?;
    metro_tx.send(MetroCommand::SendParam("harmonics".to_string(), OscType::Float(harmonics as f32 / 16383.0)))?;
    metro_tx.send(MetroCommand::SendParam("timbre".to_string(), OscType::Float(timbre as f32 / 16383.0)))?;
    metro_tx.send(MetroCommand::SendParam("morph".to_string(), OscType::Float(morph as f32 / 16383.0)))?;
    metro_tx.send(MetroCommand::SendParam("decay".to_string(), OscType::Float(decay as f32 / 16383.0)))?;
    metro_tx.send(MetroCommand::SendParam("lpg".to_string(), OscType::Float(lpg as f32 / 16383.0)))?;


    Ok(())
}
