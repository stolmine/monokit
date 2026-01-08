use crate::commands::context::ExecutionContext;
use crate::commands::core::{counters, math_ops, random_ops, scale, scheduling as delay, variables};
use crate::commands::{gate, patterns, randomization, slew};
use anyhow::Result;

pub fn dispatch_core_commands<F>(
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
    let out_err = *ctx.out_err;
    let out_qry = *ctx.out_qry;
    let out_cfm = *ctx.out_cfm;
    let ev_counters = &mut *ctx.ev_counters;

    match cmd {
        "A" => {
            variables::handle_variable_a(parts, ctx, output);
            Some(Ok(vec![]))
        }
        "B" => {
            variables::handle_variable_b(parts, ctx, output);
            Some(Ok(vec![]))
        }
        "C" => {
            variables::handle_variable_c(parts, ctx, output);
            Some(Ok(vec![]))
        }
        "D" => {
            variables::handle_variable_d(parts, ctx, output);
            Some(Ok(vec![]))
        }
        "I" => {
            variables::handle_variable_i(parts, ctx, output);
            Some(Ok(vec![]))
        }
        "X" => {
            variables::handle_variable_x(parts, ctx, output);
            Some(Ok(vec![]))
        }
        "Y" => {
            variables::handle_variable_y(parts, ctx, output);
            Some(Ok(vec![]))
        }
        "Z" => {
            variables::handle_variable_z(parts, ctx, output);
            Some(Ok(vec![]))
        }
        "T" => {
            variables::handle_variable_t(parts, ctx, output);
            Some(Ok(vec![]))
        }
        "J" => Some(variables::handle_variable_j(parts, ctx, output).map(|_| vec![])),
        "K" => Some(variables::handle_variable_k(parts, ctx, output).map(|_| vec![])),
        "P.N" => {
            patterns::handle_pattern_n(parts, ctx, output);
            Some(Ok(vec![]))
        }
        "P.L" => {
            patterns::handle_pattern_l(parts, ctx, output);
            Some(Ok(vec![]))
        }
        "P.I" => {
            patterns::handle_pattern_i(parts, ctx, output);
            Some(Ok(vec![]))
        }
        "P.HERE" => {
            patterns::handle_pattern_here(ctx, output);
            Some(Ok(vec![]))
        }
        "P.NEXT" => {
            patterns::handle_pattern_next(ctx, output);
            Some(Ok(vec![]))
        }
        "P.PREV" => {
            patterns::handle_pattern_prev(ctx, output);
            Some(Ok(vec![]))
        }
        "P.PUSH" => Some(patterns::handle_pattern_push(parts, ctx, output).map(|_| vec![])),
        "P.POP" => {
            patterns::handle_pattern_pop(ctx, output);
            Some(Ok(vec![]))
        }
        "P.INS" => { Some(patterns::handle_pattern_ins(parts, ctx, output).map(|_| vec![])) }
        "P.RM" => { Some(patterns::handle_pattern_rm(parts, ctx, output).map(|_| vec![])) }
        "P.REV" => {
            patterns::handle_pattern_rev(ctx, output);
            Some(Ok(vec![]))
        }
        "P.ROT" => { Some(patterns::handle_pattern_rot(parts, ctx, output).map(|_| vec![])) }
        "P.SHUF" => {
            patterns::handle_pattern_shuf(ctx, output);
            Some(Ok(vec![]))
        }
        "P.SORT" => {
            patterns::handle_pattern_sort(ctx, output);
            Some(Ok(vec![]))
        }
        "P.RND" => { Some(patterns::handle_pattern_rnd(parts, ctx, output).map(|_| vec![])) }
        "P.ADD" => { Some(patterns::handle_pattern_add(parts, ctx, output).map(|_| vec![])) }
        "P.SUB" => { Some(patterns::handle_pattern_sub(parts, ctx, output).map(|_| vec![])) }
        "P.MUL" => { Some(patterns::handle_pattern_mul(parts, ctx, output).map(|_| vec![])) }
        "P.DIV" => { Some(patterns::handle_pattern_div(parts, ctx, output).map(|_| vec![])) }
        "P.MOD" => { Some(patterns::handle_pattern_mod(parts, ctx, output).map(|_| vec![])) }
        "P.SCALE" => { Some(patterns::handle_pattern_scale(parts, ctx, output).map(|_| vec![])) }
        "P.MIN" => {
            patterns::handle_pattern_min(ctx, output);
            Some(Ok(vec![]))
        }
        "P.MAX" => {
            patterns::handle_pattern_max(ctx, output);
            Some(Ok(vec![]))
        }
        "P.SUM" => {
            patterns::handle_pattern_sum(ctx, output);
            Some(Ok(vec![]))
        }
        "P.AVG" => {
            patterns::handle_pattern_avg(ctx, output);
            Some(Ok(vec![]))
        }
        "P.FND" => { Some(patterns::handle_pattern_fnd(parts, ctx, output).map(|_| vec![])) }
        "P" => { Some(patterns::handle_pattern(parts, ctx, output).map(|_| vec![])) }
        "PN.L" => { Some(patterns::handle_pn_l(parts, ctx, output).map(|_| vec![])) }
        "PN.I" => { Some(patterns::handle_pn_i(parts, ctx, output).map(|_| vec![])) }
        "PN.HERE" => { Some(patterns::handle_pn_here(parts, ctx, output).map(|_| vec![])) }
        "PN.NEXT" => { Some(patterns::handle_pn_next(parts, ctx, output).map(|_| vec![])) }
        "PN.PREV" => { Some(patterns::handle_pn_prev(parts, ctx, output).map(|_| vec![])) }
        "PN.PUSH" => { Some(patterns::handle_pn_push(parts, ctx, output).map(|_| vec![])) }
        "PN.POP" => { Some(patterns::handle_pn_pop(parts, ctx, output).map(|_| vec![])) }
        "PN.INS" => { Some(patterns::handle_pn_ins(parts, ctx, output).map(|_| vec![])) }
        "PN.RM" => { Some(patterns::handle_pn_rm(parts, ctx, output).map(|_| vec![])) }
        "PN.REV" => { Some(patterns::handle_pn_rev(parts, ctx, output).map(|_| vec![])) }
        "PN.ROT" => { Some(patterns::handle_pn_rot(parts, ctx, output).map(|_| vec![])) }
        "PN.SHUF" => { Some(patterns::handle_pn_shuf(parts, ctx, output).map(|_| vec![])) }
        "PN.SORT" => { Some(patterns::handle_pn_sort(parts, ctx, output).map(|_| vec![])) }
        "PN.RND" => { Some(patterns::handle_pn_rnd(parts, ctx, output).map(|_| vec![])) }
        "PN.ADD" => { Some(patterns::handle_pn_add(parts, ctx, output).map(|_| vec![])) }
        "PN.SUB" => { Some(patterns::handle_pn_sub(parts, ctx, output).map(|_| vec![])) }
        "PN.MUL" => { Some(patterns::handle_pn_mul(parts, ctx, output).map(|_| vec![])) }
        "PN.DIV" => { Some(patterns::handle_pn_div(parts, ctx, output).map(|_| vec![])) }
        "PN.MOD" => { Some(patterns::handle_pn_mod(parts, ctx, output).map(|_| vec![])) }
        "PN.SCALE" => { Some(patterns::handle_pn_scale(parts, ctx, output).map(|_| vec![])) }
        "PN.MIN" => { Some(patterns::handle_pn_min(parts, ctx, output).map(|_| vec![])) }
        "PN.MAX" => { Some(patterns::handle_pn_max(parts, ctx, output).map(|_| vec![])) }
        "PN.SUM" => { Some(patterns::handle_pn_sum(parts, ctx, output).map(|_| vec![])) }
        "PN.AVG" => { Some(patterns::handle_pn_avg(parts, ctx, output).map(|_| vec![])) }
        "PN.FND" => { Some(patterns::handle_pn_fnd(parts, ctx, output).map(|_| vec![])) }
        "PN" => { Some(patterns::handle_pn(parts, ctx, output).map(|_| vec![])) }
        "RND" => {
            random_ops::handle_rnd(parts, output);
            Some(Ok(vec![]))
        }
        "RRND" => {
            random_ops::handle_rrnd(parts, output);
            Some(Ok(vec![]))
        }
        "TOSS" => {
            random_ops::handle_toss(output);
            Some(Ok(vec![]))
        }
        "EITH" => {
            random_ops::handle_eith(parts, variables, patterns, counters, scripts, script_index, scale, output);
            Some(Ok(vec![]))
        }
        "TOG" => {
            random_ops::handle_tog(parts, variables, patterns, counters, scripts, script_index, scale, output);
            Some(Ok(vec![]))
        }
        "RND.VOICE" => { Some(randomization::handle_rnd_voice(metro_tx, debug_level, output).map(|_| vec![])) }
        "RND.OSC" => { Some(randomization::handle_rnd_osc(metro_tx, debug_level, output).map(|_| vec![])) }
        "RND.FM" => { Some(randomization::handle_rnd_fm(metro_tx, debug_level, output).map(|_| vec![])) }
        "RND.MOD" => { Some(randomization::handle_rnd_mod(metro_tx, debug_level, output).map(|_| vec![])) }
        "RND.ENV" => { Some(randomization::handle_rnd_env(metro_tx, debug_level, output).map(|_| vec![])) }
        "RND.P" => { Some(randomization::handle_rnd_p(parts, variables, patterns, counters, scripts, script_index, scale, debug_level, output).map(|_| vec![])) }
        "RND.PN" => { Some(randomization::handle_rnd_pn(parts, variables, patterns, counters, scripts, script_index, scale, debug_level, output).map(|_| vec![])) }
        "RND.PALL" => { Some(randomization::handle_rnd_pall(parts, variables, patterns, counters, scripts, script_index, scale, debug_level, output).map(|_| vec![])) }
        "RND.PL" => { Some(randomization::handle_rnd_pl(metro_tx, debug_level, output).map(|_| vec![])) }
        "RND.FX" => { Some(randomization::handle_rnd_fx(metro_tx, debug_level, output).map(|_| vec![])) }
        "RND.FILT" => { Some(randomization::handle_rnd_filt(metro_tx, debug_level, output).map(|_| vec![])) }
        "RND.DLY" => { Some(randomization::handle_rnd_dly(metro_tx, debug_level, output).map(|_| vec![])) }
        "RND.VERB" => { Some(randomization::handle_rnd_verb(metro_tx, debug_level, output).map(|_| vec![])) }
        "ADD" | "+" => {
            math_ops::handle_add(parts, variables, patterns, counters, scripts, script_index, scale, output);
            Some(Ok(vec![]))
        }
        "SUB" | "-" => {
            math_ops::handle_sub(parts, variables, patterns, counters, scripts, script_index, scale, output);
            Some(Ok(vec![]))
        }
        "MUL" | "*" => {
            math_ops::handle_mul(parts, variables, patterns, counters, scripts, script_index, scale, output);
            Some(Ok(vec![]))
        }
        "DIV" | "/" => {
            math_ops::handle_div(parts, variables, patterns, counters, scripts, script_index, scale, output);
            Some(Ok(vec![]))
        }
        "MOD" | "%" => {
            math_ops::handle_mod(parts, variables, patterns, counters, scripts, script_index, scale, output);
            Some(Ok(vec![]))
        }
        "MAP" => {
            math_ops::handle_map(parts, variables, patterns, counters, scripts, script_index, scale, output);
            Some(Ok(vec![]))
        }
        "SLEW" => { Some(slew::handle_slew(parts, variables, patterns, counters, scripts, script_index, scale, metro_tx, debug_level, out_cfm, output).map(|_| vec![])) }
        "SLEW.ALL" => { Some(slew::handle_slew_all(parts, variables, patterns, counters, scripts, script_index, scale, metro_tx, debug_level, out_cfm, output).map(|_| vec![])) }
        "GATE" => {
            gate::handle_gate(parts, variables, patterns, counters, scripts, script_index, scale, metro_tx, debug_level, output);
            Some(Ok(vec![]))
        }
        "AENV.GATE" => {
            gate::handle_aenv_gate(parts, variables, patterns, counters, scripts, script_index, scale, metro_tx, debug_level, output);
            Some(Ok(vec![]))
        }
        "PENV.GATE" => {
            gate::handle_penv_gate(parts, variables, patterns, counters, scripts, script_index, scale, metro_tx, debug_level, output);
            Some(Ok(vec![]))
        }
        "FMEV.GATE" => {
            gate::handle_fmev_gate(parts, variables, patterns, counters, scripts, script_index, scale, metro_tx, debug_level, output);
            Some(Ok(vec![]))
        }
        "DENV.GATE" => {
            gate::handle_denv_gate(parts, variables, patterns, counters, scripts, script_index, scale, metro_tx, debug_level, output);
            Some(Ok(vec![]))
        }
        "FBEV.GATE" => {
            gate::handle_fbev_gate(parts, variables, patterns, counters, scripts, script_index, scale, metro_tx, debug_level, output);
            Some(Ok(vec![]))
        }
        "FLEV.GATE" => {
            gate::handle_flev_gate(parts, variables, patterns, counters, scripts, script_index, scale, metro_tx, debug_level, output);
            Some(Ok(vec![]))
        }
        "N1" => {
            counters::handle_n1(counters, debug_level, out_qry, output);
            Some(Ok(vec![]))
        }
        "N2" => {
            counters::handle_n2(counters, debug_level, out_qry, output);
            Some(Ok(vec![]))
        }
        "N3" => {
            counters::handle_n3(counters, debug_level, out_qry, output);
            Some(Ok(vec![]))
        }
        "N4" => {
            counters::handle_n4(counters, debug_level, out_qry, output);
            Some(Ok(vec![]))
        }
        "N1.RST" => {
            counters::handle_n1_rst(counters, debug_level, out_cfm, output);
            Some(Ok(vec![]))
        }
        "N2.RST" => {
            counters::handle_n2_rst(counters, debug_level, out_cfm, output);
            Some(Ok(vec![]))
        }
        "N3.RST" => {
            counters::handle_n3_rst(counters, debug_level, out_cfm, output);
            Some(Ok(vec![]))
        }
        "N4.RST" => {
            counters::handle_n4_rst(counters, debug_level, out_cfm, output);
            Some(Ok(vec![]))
        }
        "N1.MAX" => {
            counters::handle_n1_max(parts, variables, patterns, counters, scripts, script_index, scale, debug_level, out_err, out_cfm, output);
            Some(Ok(vec![]))
        }
        "N2.MAX" => {
            counters::handle_n2_max(parts, variables, patterns, counters, scripts, script_index, scale, debug_level, out_err, out_cfm, output);
            Some(Ok(vec![]))
        }
        "N3.MAX" => {
            counters::handle_n3_max(parts, variables, patterns, counters, scripts, script_index, scale, debug_level, out_err, out_cfm, output);
            Some(Ok(vec![]))
        }
        "N4.MAX" => {
            counters::handle_n4_max(parts, variables, patterns, counters, scripts, script_index, scale, debug_level, out_err, out_cfm, output);
            Some(Ok(vec![]))
        }
        "N1.MIN" => {
            counters::handle_n1_min(parts, variables, patterns, counters, scripts, script_index, scale, debug_level, out_cfm, output);
            Some(Ok(vec![]))
        }
        "N2.MIN" => {
            counters::handle_n2_min(parts, variables, patterns, counters, scripts, script_index, scale, debug_level, out_cfm, output);
            Some(Ok(vec![]))
        }
        "N3.MIN" => {
            counters::handle_n3_min(parts, variables, patterns, counters, scripts, script_index, scale, debug_level, out_cfm, output);
            Some(Ok(vec![]))
        }
        "N4.MIN" => {
            counters::handle_n4_min(parts, variables, patterns, counters, scripts, script_index, scale, debug_level, out_cfm, output);
            Some(Ok(vec![]))
        }
        "SYNC" => {
            crate::commands::core::sync::handle_sync(patterns, counters, ev_counters, debug_level, out_cfm, output);
            Some(Ok(vec![]))
        }
        "SYNC.SEQ" => {
            crate::commands::core::sync::handle_sync_seq(patterns, debug_level, out_cfm, output);
            Some(Ok(vec![]))
        }
        "SYNC.TOG" => {
            crate::commands::core::sync::handle_sync_tog(patterns, debug_level, out_cfm, output);
            Some(Ok(vec![]))
        }
        "SYNC.PAT" => {
            crate::commands::core::sync::handle_sync_pat(patterns, debug_level, out_cfm, output);
            Some(Ok(vec![]))
        }
        "DEL" => {
            let input = parts.join(" ");
            Some(delay::handle_del(parts, &input, variables, patterns, counters, scripts, script_index, metro_tx, scale, debug_level, output).map(|_| vec![]))
        }
        "DEL.CLR" => {
            Some(delay::handle_del_clr(metro_tx, debug_level, *ctx.out_ess, output).map(|_| vec![]))
        }
        "DEL.X" => {
            let input = parts.join(" ");
            Some(delay::handle_del_x(parts, &input, variables, patterns, counters, scripts, script_index, metro_tx, scale, debug_level, output).map(|_| vec![]))
        }
        "DEL.R" => {
            let input = parts.join(" ");
            Some(delay::handle_del_r(parts, &input, variables, patterns, counters, scripts, script_index, metro_tx, scale, debug_level, output).map(|_| vec![]))
        }
        "Q.ROOT" => {
            scale::handle_q_root(parts, variables, patterns, counters, scripts, script_index, scale, debug_level, output);
            Some(Ok(vec![]))
        }
        "Q.SCALE" => {
            scale::handle_q_scale(parts, variables, patterns, counters, scripts, script_index, scale, debug_level, output);
            Some(Ok(vec![]))
        }
        "Q.BIT" => {
            scale::handle_q_bit(parts, variables, patterns, counters, scripts, script_index, scale, debug_level, output);
            Some(Ok(vec![]))
        }
        _ => None,
    }
}
