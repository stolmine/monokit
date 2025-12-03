use anyhow::Result;

use super::resolve_alias;

/// Validates PN.* expression fragments that require pattern arguments.
/// Returns the number of tokens consumed, or None if this is not a PN.* expression.
fn validate_pn_expression(parts: &[&str], start_idx: usize) -> Result<Option<usize>> {
    if start_idx >= parts.len() {
        return Ok(None);
    }

    let part = parts[start_idx].to_uppercase();
    let part = resolve_alias(&part);

    // Check for PN.* commands that require pattern number
    match part.as_str() {
        "PN.HERE" | "PN.NEXT" | "PN.PREV" | "PN.POP" | "PN.REV" | "PN.SHUF" | "PN.SORT" |
        "PN.MIN" | "PN.MAX" | "PN.SUM" | "PN.AVG" | "PN.RND" => {
            // These require exactly 1 argument (pattern number)
            if start_idx + 1 >= parts.len() {
                return Err(anyhow::anyhow!("{} NEEDS PAT NUM", part));
            }
            // Recursively validate the argument
            if let Some(consumed) = validate_pn_expression(parts, start_idx + 1)? {
                Ok(Some(1 + consumed))
            } else {
                Ok(Some(2)) // Command + its argument
            }
        }
        "PN.L" | "PN.I" => {
            // These require at least 1 argument (pattern number)
            if start_idx + 1 >= parts.len() {
                return Err(anyhow::anyhow!("{} REQUIRES AT LEAST 1 ARGUMENT", part));
            }
            // Recursively validate the argument
            if let Some(consumed) = validate_pn_expression(parts, start_idx + 1)? {
                Ok(Some(1 + consumed))
            } else {
                Ok(Some(2)) // Command + at least first argument
            }
        }
        "PN.PUSH" | "PN.ADD" | "PN.SUB" | "PN.MUL" | "PN.DIV" | "PN.MOD" | "PN.FND" | "PN.ROT" => {
            // These require 2 arguments (pattern number and value/amount)
            if start_idx + 2 >= parts.len() {
                return Err(anyhow::anyhow!("{} NEEDS PAT NUM AND VAL", part));
            }
            // Recursively validate both arguments
            let mut consumed = 1; // The command itself
            if let Some(arg1_consumed) = validate_pn_expression(parts, start_idx + 1)? {
                consumed += arg1_consumed;
            } else {
                consumed += 1;
            }
            if let Some(arg2_consumed) = validate_pn_expression(parts, start_idx + consumed)? {
                consumed += arg2_consumed;
            } else {
                consumed += 1;
            }
            Ok(Some(consumed))
        }
        "PN.SCALE" => {
            // Requires 3 arguments (pattern, min, max)
            if start_idx + 3 >= parts.len() {
                return Err(anyhow::anyhow!("PN.SCALE NEEDS PAT, MIN, MAX"));
            }
            // Recursively validate all three arguments
            let mut consumed = 1; // The command itself
            for _ in 0..3 {
                if let Some(arg_consumed) = validate_pn_expression(parts, start_idx + consumed)? {
                    consumed += arg_consumed;
                } else {
                    consumed += 1;
                }
            }
            Ok(Some(consumed))
        }
        "PN" => {
            // Requires at least 2 arguments (pattern and index)
            if start_idx + 2 >= parts.len() {
                return Err(anyhow::anyhow!("PN requires at least 2 arguments"));
            }
            // Recursively validate both arguments
            let mut consumed = 1; // The command itself
            if let Some(arg1_consumed) = validate_pn_expression(parts, start_idx + 1)? {
                consumed += arg1_consumed;
            } else {
                consumed += 1;
            }
            if let Some(arg2_consumed) = validate_pn_expression(parts, start_idx + consumed)? {
                consumed += arg2_consumed;
            } else {
                consumed += 1;
            }
            Ok(Some(consumed))
        }
        _ => Ok(None)
    }
}

/// Validates all PN.* expressions in the argument list
fn validate_all_pn_expressions(parts: &[&str], start_idx: usize) -> Result<()> {
    let mut idx = start_idx;
    while idx < parts.len() {
        if let Some(consumed) = validate_pn_expression(parts, idx)? {
            idx += consumed;
        } else {
            idx += 1;
        }
    }
    Ok(())
}

pub fn validate_script_command(cmd: &str) -> Result<()> {
    let trimmed = cmd.trim();
    if trimmed.is_empty() {
        return Ok(());
    }

    let upper = trimmed.to_uppercase();

    // Check for SEQ" or SEQ' anywhere in command (no space before quote)
    if upper.contains("SEQ\"") || upper.contains("SEQ'") {
        return Err(anyhow::anyhow!("SEQ REQUIRES SPACE BEFORE QUOTE"));
    }

    // Early SEQ validation - must come before general command parsing
    if upper.starts_with("SEQ") {
        // Check for proper SEQ command format
        if let Some(seq_pos) = upper.find("SEQ ") {
            let after_seq_start = seq_pos + 4;
            if after_seq_start < trimmed.len() {
                let remaining = &trimmed[after_seq_start..];
                let remaining_trimmed = remaining.trim_start();

                // Check if pattern is empty
                if remaining_trimmed.is_empty() {
                    return Err(anyhow::anyhow!("SEQ REQUIRES A QUOTED PATTERN"));
                }

                // Check for proper quote structure
                if remaining_trimmed.starts_with('"') {
                    if !remaining_trimmed.ends_with('"') || remaining_trimmed.len() == 1 {
                        return Err(anyhow::anyhow!("SEQ HAS UNCLOSED QUOTE"));
                    }
                    // Check for empty pattern
                    if remaining_trimmed.len() == 2 {
                        return Err(anyhow::anyhow!("SEQ REQUIRES A QUOTED PATTERN"));
                    }
                    // Check for empty alternation or random choice
                    let inner = &remaining_trimmed[1..remaining_trimmed.len()-1];
                    if inner.contains("<>") {
                        return Err(anyhow::anyhow!("SEQ HAS EMPTY ALTERNATION <>"));
                    }
                    if inner.contains("{}") {
                        return Err(anyhow::anyhow!("SEQ HAS EMPTY RANDOM CHOICE {{}}"));
                    }
                } else if remaining_trimmed.starts_with('\'') {
                    if !remaining_trimmed.ends_with('\'') || remaining_trimmed.len() == 1 {
                        return Err(anyhow::anyhow!("SEQ HAS UNCLOSED QUOTE"));
                    }
                    // Check for empty pattern
                    if remaining_trimmed.len() == 2 {
                        return Err(anyhow::anyhow!("SEQ REQUIRES A QUOTED PATTERN"));
                    }
                    // Check for empty alternation or random choice
                    let inner = &remaining_trimmed[1..remaining_trimmed.len()-1];
                    if inner.contains("<>") {
                        return Err(anyhow::anyhow!("SEQ HAS EMPTY ALTERNATION <>"));
                    }
                    if inner.contains("{}") {
                        return Err(anyhow::anyhow!("SEQ HAS EMPTY RANDOM CHOICE {{}}"));
                    }
                } else {
                    // No quote at all
                    return Err(anyhow::anyhow!("SEQ REQUIRES A QUOTED PATTERN"));
                }
            } else {
                return Err(anyhow::anyhow!("SEQ REQUIRES A QUOTED PATTERN"));
            }
        }

        // If all validation passes, SEQ is valid
        return Ok(());
    }

    if trimmed.contains(':') {
        let colon_pos = trimmed.find(':').unwrap();
        let prefix = trimmed[..colon_pos].trim().to_uppercase();
        if prefix.starts_with("IF ")
            || prefix.starts_with("ELIF ")
            || prefix == "ELSE"
            || prefix.starts_with("PROB ")
            || prefix.starts_with("EV ")
            || prefix.starts_with("SKIP ")
            || prefix.starts_with("L ")
            || prefix.starts_with("DEL ")
            || prefix.starts_with("DEL.X ")
            || prefix.starts_with("DEL.R ")
        {
            return Ok(());
        }
    }

    if trimmed.to_uppercase().starts_with("L ") {
        return Ok(());
    }

    if trimmed.contains(';') {
        return Ok(());
    }

    let parts: Vec<&str> = trimmed.split_whitespace().collect();
    if parts.is_empty() {
        return Ok(());
    }

    // Resolve canonical names to aliases before validation
    let command = resolve_alias(&parts[0].to_uppercase());
    let argc = parts.len() - 1;

    match command.as_str() {
        "A" | "B" | "C" | "D" | "I" | "X" | "Y" | "Z" | "T" | "J" | "K" => {
            Ok(())
        }
        "N1" | "N2" | "N3" | "N4" => {
            if argc > 0 {
                return Err(anyhow::anyhow!("{} TAKES NO ARGUMENTS", command));
            }
            Ok(())
        }
        "P.HERE" | "P.NEXT" | "P.PREV" | "P.POP" | "P.REV" | "P.SHUF" | "P.SORT" | "P.MIN" | "P.MAX" | "P.SUM" | "P.AVG" => {
            if argc > 0 {
                return Err(anyhow::anyhow!("{} TAKES NO ARGUMENTS", command));
            }
            Ok(())
        }
        "P.PUSH" | "P.ROT" | "P.ADD" | "P.SUB" | "P.MUL" | "P.DIV" | "P.MOD" | "P.FND" => {
            if argc < 1 {
                return Err(anyhow::anyhow!("{} REQUIRES AT LEAST 1 ARGUMENT", command));
            }
            Ok(())
        }
        "P.INS" | "P.RM" => {
            if argc < 1 {
                return Err(anyhow::anyhow!("{} REQUIRES AT LEAST 1 ARGUMENT", command));
            }
            Ok(())
        }
        "P.SCALE" => {
            if argc < 2 {
                return Err(anyhow::anyhow!("P.SCALE NEEDS MIN AND MAX"));
            }
            Ok(())
        }
        "P.RND" => {
            Ok(())
        }
        "P.N" | "P.L" | "P.I" => {
            if argc > 1 {
                return Err(anyhow::anyhow!("{} TAKES 0-1 ARGUMENTS", command));
            }
            Ok(())
        }
        "P" => {
            if argc < 1 {
                return Err(anyhow::anyhow!("P REQUIRES AT LEAST 1 ARGUMENT"));
            }
            Ok(())
        }
        "PN.HERE" | "PN.NEXT" | "PN.PREV" | "PN.POP" | "PN.REV" | "PN.SHUF" | "PN.SORT" | "PN.MIN" | "PN.MAX" | "PN.SUM" | "PN.AVG" => {
            if argc < 1 {
                return Err(anyhow::anyhow!("{} NEEDS PAT NUM", command));
            }
            Ok(())
        }
        "PN.L" | "PN.I" => {
            if argc < 1 {
                return Err(anyhow::anyhow!("{} REQUIRES AT LEAST 1 ARGUMENT", command));
            }
            Ok(())
        }
        "PN.PUSH" | "PN.ADD" | "PN.SUB" | "PN.MUL" | "PN.DIV" | "PN.MOD" | "PN.FND" => {
            if argc < 2 {
                return Err(anyhow::anyhow!("{} NEEDS PAT NUM AND VAL", command));
            }
            Ok(())
        }
        "PN.RM" => {
            if argc < 2 {
                return Err(anyhow::anyhow!("PN.RM NEEDS PAT NUM AND IDX"));
            }
            Ok(())
        }
        "PN.ROT" => {
            if argc < 2 {
                return Err(anyhow::anyhow!("PN.ROT NEEDS PAT NUM AND AMT"));
            }
            Ok(())
        }
        "PN.INS" => {
            if argc < 3 {
                return Err(anyhow::anyhow!("PN.INS NEEDS PAT NUM, IDX, VAL"));
            }
            Ok(())
        }
        "PN.SCALE" => {
            if argc < 3 {
                return Err(anyhow::anyhow!("PN.SCALE NEEDS PAT, MIN, MAX"));
            }
            Ok(())
        }
        "PN.RND" => {
            if argc < 1 {
                return Err(anyhow::anyhow!("PN.RND NEEDS PAT NUM"));
            }
            Ok(())
        }
        "PN" => {
            if argc < 2 {
                return Err(anyhow::anyhow!("PN REQUIRES AT LEAST 2 ARGUMENTS"));
            }
            Ok(())
        }
        "ADD" | "SUB" | "MUL" | "DIV" | "MOD" | "+" | "-" | "*" | "/" | "%" => {
            if argc < 2 {
                return Err(anyhow::anyhow!("{} REQUIRES AT LEAST 2 ARGUMENTS", command));
            }
            Ok(())
        }
        "MAP" => {
            if argc < 5 {
                return Err(anyhow::anyhow!("MAP REQUIRES AT LEAST 5 ARGUMENTS"));
            }
            Ok(())
        }
        "RND" => {
            if argc < 1 {
                return Err(anyhow::anyhow!("RND REQUIRES AT LEAST 1 ARGUMENT"));
            }
            Ok(())
        }
        "RRND" => {
            if argc < 2 {
                return Err(anyhow::anyhow!("RRND REQUIRES AT LEAST 2 ARGUMENTS"));
            }
            Ok(())
        }
        "TOSS" => {
            if argc > 0 {
                return Err(anyhow::anyhow!("TOSS TAKES NO ARGUMENTS"));
            }
            Ok(())
        }
        "EITH" => {
            if argc < 2 {
                return Err(anyhow::anyhow!("EITH REQUIRES AT LEAST 2 ARGUMENTS"));
            }
            Ok(())
        }
        "TOG" => {
            if argc < 2 {
                return Err(anyhow::anyhow!("TOG REQUIRES AT LEAST 2 ARGUMENTS"));
            }
            if argc >= 2 && parts[1] == parts[2] {
                return Err(anyhow::anyhow!("TOG REQUIRES TWO DIFFERENT VALUES"));
            }
            Ok(())
        }
        "N" => {
            if argc < 1 {
                return Err(anyhow::anyhow!("N REQUIRES AT LEAST 1 ARGUMENT"));
            }
            Ok(())
        }
        "EZ" | "NZ" => {
            if argc < 1 {
                return Err(anyhow::anyhow!("{} REQUIRES AT LEAST 1 ARGUMENT", command));
            }
            Ok(())
        }
        // Note: EQ comparison is handled in eval, not here. "EQ" command is for EQ mid-Q parameter
        "NE" | "GT" | "LT" | "GTE" | "LTE" => {
            if argc < 2 {
                return Err(anyhow::anyhow!("{} REQUIRES AT LEAST 2 ARGUMENTS", command));
            }
            Ok(())
        }
        "SCRIPT" | "$" => {
            if argc != 1 {
                return Err(anyhow::anyhow!("SCRIPT TAKES EXACTLY 1 ARGUMENT"));
            }
            Ok(())
        }
        "SAVE" | "LOAD" | "DELETE" => {
            if argc != 1 {
                return Err(anyhow::anyhow!("{} TAKES EXACTLY 1 ARGUMENT", command));
            }
            Ok(())
        }
        "SCENES" => {
            if argc > 0 {
                return Err(anyhow::anyhow!("SCENES TAKES NO ARGUMENTS"));
            }
            Ok(())
        }
        "PSET" => {
            if argc < 2 {
                return Err(anyhow::anyhow!("PSET NEEDS SCRIPT NUM AND NAME"));
            }
            Ok(())
        }
        "PSET.SAVE" => {
            if argc < 2 {
                return Err(anyhow::anyhow!("PSET.SAVE NEEDS NUM AND NAME"));
            }
            Ok(())
        }
        "PSET.DEL" => {
            if argc < 1 {
                return Err(anyhow::anyhow!("PSET.DEL REQUIRES PRESET NAME"));
            }
            Ok(())
        }
        "PSETS" => {
            if argc > 0 {
                return Err(anyhow::anyhow!("PSETS TAKES NO ARGUMENTS"));
            }
            Ok(())
        }
        "THEME" | "DEBUG" | "HEADER" | "OUT.ERR" | "OUT.ESS" | "OUT.QRY" | "OUT.CFM" | "REPL.DUMP" => {
            if argc > 1 {
                return Err(anyhow::anyhow!("{} TAKES 0-1 ARGUMENTS", command));
            }
            Ok(())
        }
        "N1.RST" | "N2.RST" | "N3.RST" | "N4.RST" => {
            if argc > 0 {
                return Err(anyhow::anyhow!("{} TAKES NO ARGUMENTS", command));
            }
            Ok(())
        }
        "N1.MAX" | "N2.MAX" | "N3.MAX" | "N4.MAX" => {
            if argc > 1 {
                return Err(anyhow::anyhow!("{} TAKES 0-1 ARGUMENTS", command));
            }
            Ok(())
        }
        "N1.MIN" | "N2.MIN" | "N3.MIN" | "N4.MIN" => {
            if argc > 1 {
                return Err(anyhow::anyhow!("{} TAKES 0-1 ARGUMENTS", command));
            }
            Ok(())
        }
        "M" => {
            if argc > 1 {
                return Err(anyhow::anyhow!("M TAKES 0-1 ARGUMENTS"));
            }
            Ok(())
        }
        "M.BPM" => {
            if argc > 1 {
                return Err(anyhow::anyhow!("M.BPM TAKES 0-1 ARGUMENTS"));
            }
            Ok(())
        }
        "M.ACT" | "M.SCRIPT" => {
            if argc > 1 {
                return Err(anyhow::anyhow!("{} TAKES 0-1 ARGUMENTS", command));
            }
            Ok(())
        }
        "TR" | "RST" | "HELP" | "REC" | "REC.STOP" | "CLEAR" | "RND.VOICE" | "RND.OSC" | "RND.FM" | "RND.MOD" | "RND.ENV" | "RND.FX" | "RND.FILT" | "RND.DLY" | "RND.VERB" => {
            if argc > 0 {
                return Err(anyhow::anyhow!("{} TAKES NO ARGUMENTS", command));
            }
            Ok(())
        }
        "RND.P" | "RND.PALL" => {
            if argc != 0 && argc != 2 {
                return Err(anyhow::anyhow!("{} NEEDS 0 OR 2 ARGS", command));
            }
            Ok(())
        }
        "RND.PN" => {
            if argc != 1 && argc != 3 {
                return Err(anyhow::anyhow!("RND.PN NEEDS 1 OR 3 ARGS"));
            }
            Ok(())
        }
        "REC.PATH" => {
            if argc != 1 {
                return Err(anyhow::anyhow!("REC.PATH TAKES EXACTLY 1 ARGUMENT"));
            }
            Ok(())
        }
        "PRINT" => {
            if argc < 1 {
                return Err(anyhow::anyhow!("PRINT REQUIRES AT LEAST 1 ARGUMENT"));
            }
            Ok(())
        }
        "VOL" => {
            if argc != 1 {
                return Err(anyhow::anyhow!("VOL TAKES EXACTLY 1 ARGUMENT"));
            }
            Ok(())
        }
        "SLEW" => {
            if argc < 2 {
                return Err(anyhow::anyhow!("SLEW NEEDS PARAM NAME AND TIME"));
            }
            Ok(())
        }
        "SLEW.ALL" => {
            if argc < 1 {
                return Err(anyhow::anyhow!("SLEW.ALL REQUIRES AT LEAST 1 ARGUMENT"));
            }
            Ok(())
        }
        "D.MODE" => {
            if argc < 1 {
                return Err(anyhow::anyhow!("D.MODE REQUIRES A VALUE (0-2)"));
            }
            Ok(())
        }
        "D.TAIL" => {
            if argc < 1 {
                return Err(anyhow::anyhow!("D.TAIL REQUIRES A VALUE (0-2)"));
            }
            Ok(())
        }
        "R.MODE" => {
            if argc < 1 {
                return Err(anyhow::anyhow!("R.MODE REQUIRES A VALUE (0-2)"));
            }
            Ok(())
        }
        "R.TAIL" => {
            if argc < 1 {
                return Err(anyhow::anyhow!("R.TAIL REQUIRES A VALUE (0-2)"));
            }
            Ok(())
        }
        // Oscillator, FM, Discontinuity
        "PF" | "MF" | "PW" | "MW" | "DC" | "DM" | "FB" | "FBA" | "FBD" |
        // Modulation bus & routing
        "TK" | "MB" | "FM" | "MX" | "MM" | "ME" | "MP" | "MD" | "MT" | "MA" | "MF.F" |
        // Envelopes (amounts and decays)
        "AD" | "PD" | "FD" | "DD" | "PA" | "FA" | "DA" |
        // Filter
        "FC" | "FQ" | "FT" | "FE" | "FED" | "FK" |
        // Resonator
        "RF" | "RD" | "RM" | "RK" |
        // Delay
        "DT" | "DF" | "DLP" | "DW" | "DS" |
        // Reverb
        "RV" | "RP" | "RH" | "RW" |
        // Lo-Fi
        "LB" | "LS" | "LM" |
        // Ring Mod
        "RGF" | "RGW" | "RGM" |
        // Compressor
        "CT" | "CR" | "CA" | "CL" | "CM" |
        // EQ (EL=low, EM=mid, EH=high, EF=freq, EQ=Q bandwidth)
        "EL" | "EM" | "EH" | "EF" | "EQ" |
        // Pan
        "PAN" |
        // Beat Repeat
        "BR.ACT" | "BR.LEN" | "BR.REV" | "BR.WIN" | "BR.MIX" |
        // Pitch Shift
        "PS.MODE" | "PS.SEMI" | "PS.GRAIN" | "PS.MIX" | "PS.TARG" => {
            if argc < 1 {
                return Err(anyhow::anyhow!("{} REQUIRES AT LEAST 1 ARGUMENT", command));
            }
            Ok(())
        }
        "ENV.ATK" | "ENV.DEC" | "ENV.CRV" | "ENV.MODE" => {
            if argc != 1 {
                return Err(anyhow::anyhow!("{} TAKES EXACTLY 1 ARGUMENT", command));
            }
            Ok(())
        }
        "AENV.ATK" | "AA" | "AENV.CRV" | "AC" | "AENV.GATE" => {
            if argc != 1 {
                return Err(anyhow::anyhow!("{} TAKES EXACTLY 1 ARGUMENT", command));
            }
            Ok(())
        }
        "PENV.ATK" | "PAA" | "PENV.CRV" | "PC" | "PENV.GATE" => {
            if argc != 1 {
                return Err(anyhow::anyhow!("{} TAKES EXACTLY 1 ARGUMENT", command));
            }
            Ok(())
        }
        "FMEV.ATK" | "FAA" | "FMEV.CRV" | "FMEV.GATE" => {
            if argc != 1 {
                return Err(anyhow::anyhow!("{} TAKES EXACTLY 1 ARGUMENT", command));
            }
            Ok(())
        }
        "DENV.ATK" | "DAA" | "DENV.CRV" | "DENV.GATE" => {
            if argc != 1 {
                return Err(anyhow::anyhow!("{} TAKES EXACTLY 1 ARGUMENT", command));
            }
            Ok(())
        }
        "FBEV.ATK" | "FBAA" | "FBEV.CRV" | "FBC" | "FBEV.GATE" => {
            if argc != 1 {
                return Err(anyhow::anyhow!("{} TAKES EXACTLY 1 ARGUMENT", command));
            }
            Ok(())
        }
        "FLEV.ATK" | "FLAA" | "FLEV.CRV" | "FLC" | "FLEV.GATE" => {
            if argc != 1 {
                return Err(anyhow::anyhow!("{} TAKES EXACTLY 1 ARGUMENT", command));
            }
            Ok(())
        }
        "GATE" => {
            if argc != 1 {
                return Err(anyhow::anyhow!("GATE TAKES EXACTLY 1 ARGUMENT"));
            }
            Ok(())
        }
        "Q.ROOT" => {
            if argc > 1 {
                return Err(anyhow::anyhow!("Q.ROOT TAKES 0-1 ARGUMENTS"));
            }
            Ok(())
        }
        "Q.SCALE" => {
            if argc != 1 {
                return Err(anyhow::anyhow!("Q.SCALE TAKES EXACTLY 1 ARGUMENT"));
            }
            Ok(())
        }
        "Q.BIT" => {
            if argc != 1 {
                return Err(anyhow::anyhow!("Q.BIT NEEDS 1 ARG"));
            }
            Ok(())
        }
        "DEL.CLR" => {
            if argc > 0 {
                return Err(anyhow::anyhow!("DEL.CLR TAKES NO ARGUMENTS"));
            }
            Ok(())
        }
        "SCOPE.TIME" | "SCOPE.CLR" | "SCOPE.MODE" | "SCOPE.UNI" => {
            // All scope commands take 0 args (query) or 1+ args (expression)
            Ok(())
        }
        "NOTE" => {
            if argc < 1 {
                return Err(anyhow::anyhow!("NOTE REQUIRES QUOTED TEXT"));
            }
            Ok(())
        }
        "NOTE.CLR" => {
            if argc > 0 {
                return Err(anyhow::anyhow!("NOTE.CLR TAKES NO ARGUMENTS"));
            }
            Ok(())
        }
        "CPU" | "BPM" => {
            if argc > 1 {
                return Err(anyhow::anyhow!("{} TAKES 0-1 ARGUMENTS", command));
            }
            Ok(())
        }
        "FLASH" | "TITLE" | "LIMIT" => {
            if argc > 1 {
                return Err(anyhow::anyhow!("{} TAKES 0-1 ARGUMENTS", command));
            }
            Ok(())
        }
        "METER.HDR" | "METER.GRID" | "ACTIVITY" | "GRID" | "GRID.DEF" | "GRID.MODE" | "HL.COND" | "HL.SEQ" | "SPECTRUM" => {
            if argc > 1 {
                return Err(anyhow::anyhow!("{} TAKES 0-1 ARGUMENTS", command));
            }
            Ok(())
        }
        "SC.DIAG" | "MIDI.DIAG" => {
            if argc > 2 {
                return Err(anyhow::anyhow!("{} TAKES 1-2 ARGUMENTS", command));
            }
            Ok(())
        }
        _ => {
            Err(anyhow::anyhow!("UNKNOWN COMMAND: {}", command))
        }
    }?;

    // Validate PN.* expressions used as arguments
    validate_all_pn_expressions(&parts, 1)?;

    Ok(())
}
