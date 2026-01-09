use crate::commands::context::ExecutionContext;
use crate::commands::synth as synth_params;
use crate::commands::system::triggers;
use anyhow::Result;

pub fn dispatch_synth_commands<F>(
    cmd: &str,
    parts: &[&str],
    ctx: &mut ExecutionContext,
    mut output: F,
) -> Option<Result<Vec<usize>>>
where
    F: FnMut(String),
{
    let metro_tx = ctx.metro_tx;
    let variables = &mut *ctx.variables;
    let patterns = &mut *ctx.patterns;
    let counters = &mut *ctx.counters;
    let scripts = &mut *ctx.scripts;
    let script_index = ctx.script_index;
    let scale = &mut *ctx.scale;
    let debug_level = *ctx.debug_level;
    let out_cfm = *ctx.out_cfm;
    let out_qry = *ctx.out_qry;
    let eq_state = &mut *ctx.eq_state;
    let mixer_data = &mut *ctx.mixer_data;
    let fx_mix_state = &mut *ctx.fx_mix_state;
    let vca_mode = &mut *ctx.vca_mode;
    let metro_interval = &*ctx.metro_interval;
    let br_len = &mut *ctx.br_len;

    match cmd {
        "PL.DEC" | "PLD" => Some(synth_params::handle_pl_dec(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_cfm, output).map(|_| vec![])),
        "PL.ENG" | "PLE" => Some(synth_params::handle_pl_eng(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_cfm, output).map(|_| vec![])),
        "PL.FREQ" | "PLF" => Some(synth_params::handle_pl_freq(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_cfm, output).map(|_| vec![])),
        "PL.HARM" | "PLH" => Some(synth_params::handle_pl_harm(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_cfm, output).map(|_| vec![])),
        "PL.LPG" | "PLL" => Some(synth_params::handle_pl_lpg(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_cfm, output).map(|_| vec![])),
        "PL.MORPH" | "PLM" => Some(synth_params::handle_pl_morph(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_cfm, output).map(|_| vec![])),
        "PL.TIMB" | "PLT" => Some(synth_params::handle_pl_timb(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_cfm, output).map(|_| vec![])),
        "PLTR" => Some(triggers::handle_pltr(ctx, output).map(|_| vec![])),
        "PLV" => Some(synth_params::handle_plv(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_cfm, output).map(|_| vec![])),
        "TR" => Some(triggers::handle_tr(ctx, output).map(|_| vec![])),
        "VOL" | "OUT.VOL" => Some(triggers::handle_vol(parts, ctx, output).map(|_| vec![])),
        "PF" | "POSC.FREQ" => Some(synth_params::handle_pf(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_cfm, output).map(|_| vec![])),
        "PW" | "POSC.WAVE" => Some(synth_params::handle_pw(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_cfm, output).map(|_| vec![])),
        "MF" | "MOSC.FREQ" => Some(synth_params::handle_mf(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_cfm, output).map(|_| vec![])),
        "MW" | "MOSC.WAVE" => Some(synth_params::handle_mw(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_cfm, output).map(|_| vec![])),
        "NW" | "NOISE.WAV" => Some(synth_params::handle_nw(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_cfm, output).map(|_| vec![])),
        "NV" | "NOISE.VOL" => Some(synth_params::handle_nv(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_cfm, output).map(|_| vec![])),
        "PV" | "PRI.VOL" => Some(synth_params::handle_pv(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_cfm, output).map(|_| vec![])),
        "MV" | "MOD.VOL" => Some(synth_params::handle_mv(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_cfm, output).map(|_| vec![])),
        "DC" | "DISC.AMT" => Some(synth_params::handle_dc(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_cfm, output).map(|_| vec![])),
        "DM" | "DISC.MODE" => Some(synth_params::handle_dm(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_cfm, output).map(|_| vec![])),
        "TK" | "MBUS.TRK" => Some(synth_params::handle_tk(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_cfm, output).map(|_| vec![])),
        "MB" | "MBUS.AMT" => Some(synth_params::handle_mb(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_cfm, output).map(|_| vec![])),
        "MBA" | "MBEV.AMT" => Some(synth_params::handle_mba(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_cfm, output).map(|_| vec![])),
        "MBD" | "MBEV.DEC" => Some(synth_params::handle_mbd(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_cfm, output).map(|_| vec![])),
        "MP" | "ROUT.MP" => Some(synth_params::handle_mp(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_cfm, output).map(|_| vec![])),
        "MD" | "ROUT.MD" => Some(synth_params::handle_md(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_cfm, output).map(|_| vec![])),
        "MT" | "ROUT.MT" => Some(synth_params::handle_mt(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_cfm, output).map(|_| vec![])),
        "MA" | "ROUT.MA" => Some(synth_params::handle_ma(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_cfm, output).map(|_| vec![])),
        "FM" | "MBUS.FM" => Some(synth_params::handle_fm(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_cfm, output).map(|_| vec![])),
        "AD" | "AENV.DEC" => Some(synth_params::handle_ad(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_cfm, output).map(|_| vec![])),
        "PD" | "PENV.DEC" => Some(synth_params::handle_pd(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_cfm, output).map(|_| vec![])),
        "FD" | "FMEV.DEC" => Some(synth_params::handle_fd(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_cfm, output).map(|_| vec![])),
        "PA" | "PENV.AMT" => Some(synth_params::handle_pa(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_cfm, output).map(|_| vec![])),
        "PAV" => Some(synth_params::handle_pav(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_cfm, output).map(|_| vec![])),
        "DD" | "DENV.DEC" => Some(synth_params::handle_dd(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_cfm, output).map(|_| vec![])),
        "MX" | "MBUS.MIX" => Some(synth_params::handle_mx(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_cfm, output).map(|_| vec![])),
        "MM" | "MBUS.MMX" => Some(synth_params::handle_mm(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_cfm, output).map(|_| vec![])),
        "ME" | "MBUS.EMX" => Some(synth_params::handle_me(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_cfm, output).map(|_| vec![])),
        "FA" | "FMEV.AMT" => Some(synth_params::handle_fa(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_cfm, output).map(|_| vec![])),
        "DA" | "DENV.AMT" => Some(synth_params::handle_da(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_cfm, output).map(|_| vec![])),
        "FB" | "MOSC.FB" => Some(synth_params::handle_fb(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_cfm, output).map(|_| vec![])),
        "FBA" | "MOSC.FBA" => Some(synth_params::handle_fba(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_cfm, output).map(|_| vec![])),
        "FBD" | "FBEV.DEC" => Some(synth_params::handle_fbd(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_cfm, output).map(|_| vec![])),
        "RF" | "RESO.FRQ" => Some(synth_params::handle_rf(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_cfm, output).map(|_| vec![])),
        "RD" | "RESO.DEC" => Some(synth_params::handle_rd(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_cfm, output).map(|_| vec![])),
        "RM" | "RESO.MIX" => Some(synth_params::handle_rm(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_cfm, fx_mix_state, output).map(|_| vec![])),
        "RK" | "RESO.KEY" => Some(synth_params::handle_rk(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_cfm, output).map(|_| vec![])),
        "DT" | "DLY.TIME" => Some(synth_params::handle_dt(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_cfm, output).map(|_| vec![])),
        "DF" | "DLY.FB" => Some(synth_params::handle_df(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_cfm, output).map(|_| vec![])),
        "DLP" | "DLY.LP" => Some(synth_params::handle_dlp(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_cfm, output).map(|_| vec![])),
        "DW" | "DLY.WET" => Some(synth_params::handle_dw(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_cfm, fx_mix_state, output).map(|_| vec![])),
        "DS" | "DLY.SYN" => Some(synth_params::handle_ds(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_cfm, output).map(|_| vec![])),
        "RV" | "REV.DEC" => Some(synth_params::handle_rv(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_cfm, output).map(|_| vec![])),
        "RP" | "REV.PRE" => Some(synth_params::handle_rp(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_cfm, output).map(|_| vec![])),
        "RH" | "REV.DMP" => Some(synth_params::handle_rh(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_cfm, output).map(|_| vec![])),
        "RW" | "REV.WET" => Some(synth_params::handle_rw(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_cfm, fx_mix_state, output).map(|_| vec![])),
        "D.MODE" | "DLY.MODE" => Some(synth_params::handle_d_mode(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_cfm, output).map(|_| vec![])),
        "D.TAIL" | "DLY.TAIL" => Some(synth_params::handle_d_tail(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_cfm, output).map(|_| vec![])),
        "R.MODE" | "REV.MODE" => Some(synth_params::handle_r_mode(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_cfm, output).map(|_| vec![])),
        "R.TAIL" | "REV.TAIL" => Some(synth_params::handle_r_tail(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_cfm, output).map(|_| vec![])),
        "FC" | "FILT.CUT" => Some(synth_params::handle_fc(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_cfm, output).map(|_| vec![])),
        "FQ" | "FILT.RES" => Some(synth_params::handle_fq(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_cfm, output).map(|_| vec![])),
        "FT" | "FILT.TYP" => Some(synth_params::handle_ft(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_cfm, output).map(|_| vec![])),
        "FE" | "FLEV.AMT" => Some(synth_params::handle_fe(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_cfm, output).map(|_| vec![])),
        "FED" | "FLEV.DEC" => Some(synth_params::handle_fed(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_cfm, output).map(|_| vec![])),
        "FK" | "FILT.KEY" => Some(synth_params::handle_fk(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_cfm, output).map(|_| vec![])),
        "MFF" | "MODF.CUT" => Some(synth_params::handle_mff(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_cfm, output).map(|_| vec![])),
        "MFQ" | "MODF.RES" => Some(synth_params::handle_mfq(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_cfm, output).map(|_| vec![])),
        "MC" | "ROUT.MC" => Some(synth_params::handle_mc(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_cfm, output).map(|_| vec![])),
        "MQ" | "ROUT.MQ" => Some(synth_params::handle_mq(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_cfm, output).map(|_| vec![])),
        "LB" | "LOFI.BIT" => Some(synth_params::handle_lb(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_cfm, output).map(|_| vec![])),
        "LS" | "LOFI.SMP" => Some(synth_params::handle_ls(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_cfm, output).map(|_| vec![])),
        "LM" | "LOFI.MIX" => Some(synth_params::handle_lm(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_cfm, fx_mix_state, output).map(|_| vec![])),
        "RGF" | "RING.FRQ" => Some(synth_params::handle_rgf(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_cfm, output).map(|_| vec![])),
        "RGW" | "RING.WAV" => Some(synth_params::handle_rgw(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_cfm, output).map(|_| vec![])),
        "RGM" | "RING.MIX" => Some(synth_params::handle_rgm(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_cfm, fx_mix_state, output).map(|_| vec![])),
        "CT" | "COMP.THR" => Some(synth_params::handle_ct(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_cfm, output).map(|_| vec![])),
        "CR" | "COMP.RAT" => Some(synth_params::handle_cr(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_cfm, output).map(|_| vec![])),
        "CA" | "COMP.ATK" => Some(synth_params::handle_ca(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_cfm, output).map(|_| vec![])),
        "CL" | "COMP.REL" => Some(synth_params::handle_cl(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_cfm, output).map(|_| vec![])),
        "CM" | "COMP.MKP" => Some(synth_params::handle_cm(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_cfm, output).map(|_| vec![])),
        "CR.MIX" | "COMP.MIX" => Some(synth_params::handle_cr_mix(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_cfm, fx_mix_state, output).map(|_| vec![])),
        "COMP.AUTO" | "CAU" => Some(synth_params::handle_comp_auto(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_cfm, output).map(|_| vec![])),
        "EL" | "EQ.LOW" => Some(synth_params::handle_el(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_cfm, eq_state, output).map(|_| vec![])),
        "ELF" | "EQ.LF" => Some(synth_params::handle_elf(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_cfm, eq_state, output).map(|_| vec![])),
        "EM" | "EQ.MID" => Some(synth_params::handle_em(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_cfm, eq_state, output).map(|_| vec![])),
        "EF" | "EQ.FRQ" => Some(synth_params::handle_ef(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_cfm, eq_state, output).map(|_| vec![])),
        "EQ" => Some(synth_params::handle_eq_param(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_cfm, eq_state, output).map(|_| vec![])),
        "EH" | "EQ.HI" => Some(synth_params::handle_eh(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_cfm, eq_state, output).map(|_| vec![])),
        "EHF" | "EQ.HF" => Some(synth_params::handle_ehf(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_cfm, eq_state, output).map(|_| vec![])),
        "PAN" | "OUT.PAN" => Some(synth_params::handle_pan(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, output).map(|_| vec![])),
        "VCA" => Some(synth_params::handle_vca(parts, vca_mode, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_qry, out_cfm, output).map(|_| vec![])),
        "VOL.OSC" | "VO" => Some(synth_params::handle_vol_osc(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_cfm, mixer_data, output).map(|_| vec![])),
        "VOL.PLA" | "VP" => Some(synth_params::handle_vol_pla(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_cfm, mixer_data, output).map(|_| vec![])),
        "VOL.NOS" | "VN" => Some(synth_params::handle_vol_nos(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_cfm, mixer_data, output).map(|_| vec![])),
        "VOL.SMP" | "VS" => Some(synth_params::handle_vol_smp(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_cfm, mixer_data, output).map(|_| vec![])),
        "PAN.OSC" | "PO" => Some(synth_params::handle_pan_osc(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_cfm, mixer_data, output).map(|_| vec![])),
        "PAN.PLA" | "PP" => Some(synth_params::handle_pan_pla(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_cfm, mixer_data, output).map(|_| vec![])),
        "PAN.NOS" | "PNN" => Some(synth_params::handle_pan_nos(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_cfm, mixer_data, output).map(|_| vec![])),
        "PAN.SMP" | "PS" => Some(synth_params::handle_pan_smp(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_cfm, mixer_data, output).map(|_| vec![])),
        "MUTE.OSC" | "MO" => Some(synth_params::handle_mute_osc(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_cfm, mixer_data, output).map(|_| vec![])),
        "MUTE.PLA" | "MPL" => Some(synth_params::handle_mute_pla(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_cfm, mixer_data, output).map(|_| vec![])),
        "MUTE.NOS" | "MN" => Some(synth_params::handle_mute_nos(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_cfm, mixer_data, output).map(|_| vec![])),
        "MUTE.SMP" | "MS" => Some(synth_params::handle_mute_smp(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_cfm, mixer_data, output).map(|_| vec![])),
        "KIT" => Some(synth_params::handle_kit(parts, ctx, output).map(|_| vec![])),
        "STR" => Some(synth_params::handle_str(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, &mut *ctx.sampler_state, out_cfm, output).map(|_| vec![])),
        "KIT.LEN" | "KL" => {
            synth_params::handle_kit_len(&*ctx.sampler_state, debug_level, out_qry, output);
            Some(Ok(vec![]))
        }
        "KIT.INFO" => {
            synth_params::handle_kit_info(&*ctx.sampler_state, debug_level, out_qry, output);
            Some(Ok(vec![]))
        }
        "S.RATE" | "SR" => Some(synth_params::handle_s_rate(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, &mut *ctx.sampler_state, out_cfm, output).map(|_| vec![])),
        "S.PITCH" | "SPT" => Some(synth_params::handle_s_pitch(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, &mut *ctx.sampler_state, out_cfm, output).map(|_| vec![])),
        "S.FINE" | "SFN" => Some(synth_params::handle_s_fine(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, &mut *ctx.sampler_state, out_cfm, output).map(|_| vec![])),
        "S.DIR" | "SD" => Some(synth_params::handle_s_dir(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, &mut *ctx.sampler_state, out_cfm, output).map(|_| vec![])),
        "S.LOOP" | "SL" => Some(synth_params::handle_s_loop(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, &mut *ctx.sampler_state, out_cfm, output).map(|_| vec![])),
        "S.START" | "SST" => Some(synth_params::handle_s_start(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, &mut *ctx.sampler_state, out_cfm, output).map(|_| vec![])),
        "S.LEN" | "SLE" => Some(synth_params::handle_s_len(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, &mut *ctx.sampler_state, out_cfm, output).map(|_| vec![])),
        "S.SLICE" | "SSLC" => Some(synth_params::handle_s_slice(parts, variables, patterns, counters, scripts, script_index, debug_level, scale, &mut *ctx.sampler_state, out_cfm, output).map(|_| vec![])),
        "S.ONSET" | "SONS" => Some(synth_params::handle_s_onset(parts, &mut *ctx.sampler_state, debug_level, out_cfm, output).map(|_| vec![])),
        "S.ONSET.MIN" | "SOMIN" => Some(synth_params::handle_s_onset_min(parts, &mut *ctx.sampler_state, debug_level, out_cfm, output).map(|_| vec![])),
        "S.ATK" | "SA" => Some(synth_params::handle_s_atk(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, &mut *ctx.sampler_state, out_cfm, output).map(|_| vec![])),
        "S.DEC" | "SDC" => Some(synth_params::handle_s_dec(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, &mut *ctx.sampler_state, out_cfm, output).map(|_| vec![])),
        "S.REL" | "SRE" => Some(synth_params::handle_s_rel(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, &mut *ctx.sampler_state, out_cfm, output).map(|_| vec![])),
        "S.SUST" | "SSU" => Some(synth_params::handle_s_sust(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, &mut *ctx.sampler_state, out_cfm, output).map(|_| vec![])),
        "S.FX" | "SFX" => Some(synth_params::handle_s_fx(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, &mut *ctx.sampler_state, out_cfm, output).map(|_| vec![])),
        "S.RATEMOD" | "SRM" => Some(synth_params::handle_s_ratemod(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, &mut *ctx.sampler_state, out_cfm, output).map(|_| vec![])),
        "S.PITCHMOD" | "SPM" => Some(synth_params::handle_s_pitchmod(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, &mut *ctx.sampler_state, out_cfm, output).map(|_| vec![])),
        "SF.CUT" | "SFC" => Some(synth_params::handle_sf_cut(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, &mut *ctx.sampler_state, out_cfm, output).map(|_| vec![])),
        "SF.RES" | "SFQ" => Some(synth_params::handle_sf_res(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, &mut *ctx.sampler_state, out_cfm, output).map(|_| vec![])),
        "SF.TYPE" | "SFT" => Some(synth_params::handle_sf_type(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, &mut *ctx.sampler_state, out_cfm, output).map(|_| vec![])),
        "SF.BITS" | "SFB" => Some(synth_params::handle_sf_bits(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, &mut *ctx.sampler_state, out_cfm, output).map(|_| vec![])),
        "SF.RATE" | "SFR" => Some(synth_params::handle_sf_rate(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, &mut *ctx.sampler_state, out_cfm, output).map(|_| vec![])),
        "SF.DECI" | "SFD" => Some(synth_params::handle_sf_deci(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, &mut *ctx.sampler_state, out_cfm, output).map(|_| vec![])),
        "SRINGS.PIT" | "SRRP" => Some(synth_params::handle_srings_pit(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, &mut *ctx.sampler_state, out_cfm, output).map(|_| vec![])),
        "SRINGS.STRC" | "SRRS" => Some(synth_params::handle_srings_strc(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, &mut *ctx.sampler_state, out_cfm, output).map(|_| vec![])),
        "SRINGS.BRIT" | "SRRB" => Some(synth_params::handle_srings_brit(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, &mut *ctx.sampler_state, out_cfm, output).map(|_| vec![])),
        "SRINGS.DAMP" | "SRRD" => Some(synth_params::handle_srings_damp(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, &mut *ctx.sampler_state, out_cfm, output).map(|_| vec![])),
        "SRINGS.POS" | "SRRO" => Some(synth_params::handle_srings_pos(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, &mut *ctx.sampler_state, out_cfm, output).map(|_| vec![])),
        "SRINGS.MODE" | "SRRM" => Some(synth_params::handle_srings_mode(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, &mut *ctx.sampler_state, out_cfm, output).map(|_| vec![])),
        "SRINGS.WET" | "SRRW" => Some(synth_params::handle_srings_wet(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, &mut *ctx.sampler_state, out_cfm, output).map(|_| vec![])),
        "SF.CUTMOD" | "SFCM" => Some(synth_params::handle_sf_cutmod(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, &mut *ctx.sampler_state, out_cfm, output).map(|_| vec![])),
        "SF.RESMOD" | "SFQM" => Some(synth_params::handle_sf_resmod(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, &mut *ctx.sampler_state, out_cfm, output).map(|_| vec![])),
        "BR.LEN" | "BRL" => Some(synth_params::handle_br_len(parts, *metro_interval, br_len, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_cfm, output).map(|_| vec![])),
        "BR.REV" | "BRR" => Some(synth_params::handle_br_rev(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_cfm, output).map(|_| vec![])),
        "BR.WIN" | "BRW" => Some(synth_params::handle_br_win(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_cfm, output).map(|_| vec![])),
        "BR.MIX" | "BRX" => Some(synth_params::handle_br_mix(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_cfm, fx_mix_state, output).map(|_| vec![])),
        "PS.MODE" | "PSM" => Some(synth_params::handle_ps_mode(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_cfm, output).map(|_| vec![])),
        "PS.SEMI" | "PSS" => Some(synth_params::handle_ps_semi(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_cfm, output).map(|_| vec![])),
        "PS.GRAIN" | "PSG" => Some(synth_params::handle_ps_grain(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_cfm, output).map(|_| vec![])),
        "PS.MIX" | "PSX" => Some(synth_params::handle_ps_mix(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_cfm, fx_mix_state, output).map(|_| vec![])),
        "PS.TARG" | "PST" => Some(synth_params::handle_ps_targ(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_cfm, output).map(|_| vec![])),
        "CL.TRIG" | "CLTR" => {
            use crate::commands::logging::log_command;
            log_command(&format!("[DEBUG] [4] MATCHED CL.TRIG/CLTR pattern! cmd='{}'", cmd));
            let mut output_vec = vec![];
            log_command("[DEBUG] [5] Calling handle_cl_trig...");
            match synth_params::handle_cl_trig(ctx, &mut output_vec) {
                Ok(_) => {
                    log_command(&format!("[DEBUG] [6] handle_cl_trig returned, output_vec.len()={}", output_vec.len()));
                    for msg in output_vec {
                        log_command(&format!("[DEBUG] [7] Outputting message: '{}'", msg));
                        output(msg);
                    }
                    log_command("[DEBUG] [8] CL.TRIG dispatch complete");
                    Some(Ok(vec![]))
                }
                Err(e) => Some(Err(e)),
            }
        }
        "CL.PITCH" | "CLP" | "CLPT" => Some(synth_params::handle_cl_pitch(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_cfm, output).map(|_| vec![])),
        "CL.POS" | "CLO" | "CLPS" => Some(synth_params::handle_cl_pos(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_cfm, output).map(|_| vec![])),
        "CL.SIZE" | "CLS" | "CLSZ" => Some(synth_params::handle_cl_size(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_cfm, output).map(|_| vec![])),
        "CL.DENS" | "CLD" | "CLDS" => Some(synth_params::handle_cl_dens(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_cfm, output).map(|_| vec![])),
        "CL.TEX" | "CLT" | "CLTX" => Some(synth_params::handle_cl_tex(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_cfm, output).map(|_| vec![])),
        "CL.WET" | "CLW" => Some(synth_params::handle_cl_wet(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_cfm, fx_mix_state, output).map(|_| vec![])),
        "CL.GAIN" | "CLG" => Some(synth_params::handle_cl_gain(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_cfm, output).map(|_| vec![])),
        "CL.SPREAD" | "CLSP" => Some(synth_params::handle_cl_spread(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_cfm, output).map(|_| vec![])),
        "CL.RVB" | "CLRV" => Some(synth_params::handle_cl_rvb(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_cfm, output).map(|_| vec![])),
        "CL.FB" | "CLF" => Some(synth_params::handle_cl_fb(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_cfm, output).map(|_| vec![])),
        "CL.FREEZE" | "CLFZ" => Some(synth_params::handle_cl_freeze(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_cfm, output).map(|_| vec![])),
        "CL.MODE" | "CLM" => Some(synth_params::handle_cl_mode(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_cfm, output).map(|_| vec![])),
        "CL.LOFI" | "CLLO" => Some(synth_params::handle_cl_lofi(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_cfm, output).map(|_| vec![])),
        "AENV.ATK" | "AA" => Some(synth_params::handle_aenv_atk(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_cfm, output).map(|_| vec![])),
        "AENV.CRV" | "AC" => Some(synth_params::handle_aenv_crv(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_cfm, output).map(|_| vec![])),
        "PENV.ATK" | "PAA" => Some(synth_params::handle_penv_atk(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_cfm, output).map(|_| vec![])),
        "PENV.CRV" | "PC" => Some(synth_params::handle_penv_crv(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_cfm, output).map(|_| vec![])),
        "FMEV.ATK" | "FAA" => Some(synth_params::handle_fmev_atk(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_cfm, output).map(|_| vec![])),
        "FMEV.CRV" => Some(synth_params::handle_fmev_crv(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_cfm, output).map(|_| vec![])),
        "DENV.ATK" | "DAA" => Some(synth_params::handle_denv_atk(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_cfm, output).map(|_| vec![])),
        "DENV.CRV" => Some(synth_params::handle_denv_crv(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_cfm, output).map(|_| vec![])),
        "FBEV.ATK" | "FBAA" => Some(synth_params::handle_fbev_atk(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_cfm, output).map(|_| vec![])),
        "FBEV.CRV" | "FBC" => Some(synth_params::handle_fbev_crv(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_cfm, output).map(|_| vec![])),
        "FBEV.AMT" => Some(synth_params::handle_fba(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_cfm, output).map(|_| vec![])),
        "FLEV.ATK" | "FLAA" => Some(synth_params::handle_flev_atk(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_cfm, output).map(|_| vec![])),
        "FLEV.CRV" | "FLC" => Some(synth_params::handle_flev_crv(parts, variables, patterns, counters, scripts, script_index, metro_tx, debug_level, scale, out_cfm, output).map(|_| vec![])),
        _ => None,
    }
}
