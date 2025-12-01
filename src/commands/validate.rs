use anyhow::Result;

use super::resolve_alias;

pub fn validate_script_command(cmd: &str) -> Result<()> {
    let trimmed = cmd.trim();
    if trimmed.is_empty() {
        return Ok(());
    }

    let upper = trimmed.to_uppercase();
    if upper.contains("SEQ\"") || upper.contains("SEQ'") {
        return Err(anyhow::anyhow!("SEQ requires space before quote: SEQ \"...\""));
    }

    if let Some(seq_pos) = upper.find("SEQ ") {
        let after_seq_start = seq_pos + 4;
        if after_seq_start < trimmed.len() {
            let remaining = &trimmed[after_seq_start..];
            let remaining_trimmed = remaining.trim_start();
            if remaining_trimmed.starts_with('"') {
                if !remaining_trimmed.ends_with('"') || remaining_trimmed.len() == 1 {
                    return Err(anyhow::anyhow!("SEQ has unclosed quote"));
                }
            } else if remaining_trimmed.starts_with('\'') {
                if !remaining_trimmed.ends_with('\'') || remaining_trimmed.len() == 1 {
                    return Err(anyhow::anyhow!("SEQ has unclosed quote"));
                }
            }
        }
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
        "P.HERE" | "P.NEXT" | "P.PREV" | "P.POP" | "P.REV" | "P.SHUF" | "P.SORT" | "P.MIN" | "P.MAX" | "P.SUM" | "P.AVG" => {
            if argc > 0 {
                return Err(anyhow::anyhow!("{} takes no arguments", command));
            }
            Ok(())
        }
        "P.PUSH" | "P.ROT" | "P.ADD" | "P.SUB" | "P.MUL" | "P.DIV" | "P.MOD" | "P.FND" => {
            if argc < 1 {
                return Err(anyhow::anyhow!("{} requires at least 1 argument", command));
            }
            Ok(())
        }
        "P.INS" | "P.RM" => {
            if argc < 1 {
                return Err(anyhow::anyhow!("{} requires at least 1 argument", command));
            }
            Ok(())
        }
        "P.SCALE" => {
            if argc < 2 {
                return Err(anyhow::anyhow!("P.SCALE requires at least 2 arguments (min and max)", ));
            }
            Ok(())
        }
        "P.RND" => {
            Ok(())
        }
        "P.N" | "P.L" | "P.I" => {
            if argc > 1 {
                return Err(anyhow::anyhow!("{} takes 0-1 arguments", command));
            }
            Ok(())
        }
        "P" => {
            if argc < 1 {
                return Err(anyhow::anyhow!("P requires at least 1 argument"));
            }
            Ok(())
        }
        "PN.HERE" | "PN.NEXT" | "PN.PREV" | "PN.POP" | "PN.REV" | "PN.SHUF" | "PN.SORT" | "PN.MIN" | "PN.MAX" | "PN.SUM" | "PN.AVG" => {
            if argc < 1 {
                return Err(anyhow::anyhow!("{} requires at least 1 argument (pattern number)", command));
            }
            Ok(())
        }
        "PN.L" | "PN.I" => {
            if argc < 1 {
                return Err(anyhow::anyhow!("{} requires at least 1 argument", command));
            }
            Ok(())
        }
        "PN.PUSH" | "PN.ADD" | "PN.SUB" | "PN.MUL" | "PN.DIV" | "PN.MOD" | "PN.FND" => {
            if argc < 2 {
                return Err(anyhow::anyhow!("{} requires at least 2 arguments (pattern number and value)", command));
            }
            Ok(())
        }
        "PN.RM" => {
            if argc < 2 {
                return Err(anyhow::anyhow!("PN.RM requires at least 2 arguments (pattern number and index)"));
            }
            Ok(())
        }
        "PN.ROT" => {
            if argc < 2 {
                return Err(anyhow::anyhow!("PN.ROT requires at least 2 arguments (pattern number and rotation amount)"));
            }
            Ok(())
        }
        "PN.INS" => {
            if argc < 3 {
                return Err(anyhow::anyhow!("PN.INS requires at least 3 arguments (pattern number, index, and value)"));
            }
            Ok(())
        }
        "PN.SCALE" => {
            if argc < 3 {
                return Err(anyhow::anyhow!("PN.SCALE requires at least 3 arguments (pattern number, min, and max)"));
            }
            Ok(())
        }
        "PN.RND" => {
            if argc < 1 {
                return Err(anyhow::anyhow!("PN.RND requires at least 1 argument (pattern number)"));
            }
            Ok(())
        }
        "PN" => {
            if argc < 2 {
                return Err(anyhow::anyhow!("PN requires at least 2 arguments"));
            }
            Ok(())
        }
        "ADD" | "SUB" | "MUL" | "DIV" | "MOD" | "+" | "-" | "*" | "/" | "%" => {
            if argc < 2 {
                return Err(anyhow::anyhow!("{} requires at least 2 arguments", command));
            }
            Ok(())
        }
        "MAP" => {
            if argc < 5 {
                return Err(anyhow::anyhow!("MAP requires at least 5 arguments"));
            }
            Ok(())
        }
        "RND" => {
            if argc < 1 {
                return Err(anyhow::anyhow!("RND requires at least 1 argument"));
            }
            Ok(())
        }
        "RRND" => {
            if argc < 2 {
                return Err(anyhow::anyhow!("RRND requires at least 2 arguments"));
            }
            Ok(())
        }
        "TOSS" => {
            if argc > 0 {
                return Err(anyhow::anyhow!("TOSS takes no arguments"));
            }
            Ok(())
        }
        "EITH" => {
            if argc < 2 {
                return Err(anyhow::anyhow!("EITH requires at least 2 arguments"));
            }
            Ok(())
        }
        "TOG" => {
            if argc < 2 {
                return Err(anyhow::anyhow!("TOG requires at least 2 arguments"));
            }
            if argc >= 2 && parts[1] == parts[2] {
                return Err(anyhow::anyhow!("TOG requires two different values"));
            }
            Ok(())
        }
        "N" => {
            if argc < 1 {
                return Err(anyhow::anyhow!("N requires at least 1 argument"));
            }
            Ok(())
        }
        "EZ" | "NZ" => {
            if argc < 1 {
                return Err(anyhow::anyhow!("{} requires at least 1 argument", command));
            }
            Ok(())
        }
        // Note: EQ comparison is handled in eval, not here. "EQ" command is for EQ mid-Q parameter
        "NE" | "GT" | "LT" | "GTE" | "LTE" => {
            if argc < 2 {
                return Err(anyhow::anyhow!("{} requires at least 2 arguments", command));
            }
            Ok(())
        }
        "SCRIPT" => {
            if argc != 1 {
                return Err(anyhow::anyhow!("SCRIPT takes exactly 1 argument"));
            }
            Ok(())
        }
        "SAVE" | "LOAD" | "DELETE" => {
            if argc != 1 {
                return Err(anyhow::anyhow!("{} takes exactly 1 argument", command));
            }
            Ok(())
        }
        "SCENES" => {
            if argc > 0 {
                return Err(anyhow::anyhow!("SCENES takes no arguments"));
            }
            Ok(())
        }
        "PSET" => {
            if argc < 2 {
                return Err(anyhow::anyhow!("PSET requires script number and preset name"));
            }
            Ok(())
        }
        "PSET.SAVE" => {
            if argc < 2 {
                return Err(anyhow::anyhow!("PSET.SAVE requires script number and preset name"));
            }
            Ok(())
        }
        "PSET.DEL" => {
            if argc < 1 {
                return Err(anyhow::anyhow!("PSET.DEL requires preset name"));
            }
            Ok(())
        }
        "PSETS" => {
            if argc > 0 {
                return Err(anyhow::anyhow!("PSETS takes no arguments"));
            }
            Ok(())
        }
        "THEME" | "DEBUG" => {
            if argc > 1 {
                return Err(anyhow::anyhow!("{} takes 0-1 arguments", command));
            }
            Ok(())
        }
        "N1.RST" | "N2.RST" | "N3.RST" | "N4.RST" => {
            if argc > 0 {
                return Err(anyhow::anyhow!("{} takes no arguments", command));
            }
            Ok(())
        }
        "N1.MAX" | "N2.MAX" | "N3.MAX" | "N4.MAX" => {
            if argc < 1 {
                return Err(anyhow::anyhow!("{} requires at least 1 argument", command));
            }
            Ok(())
        }
        "N1.MIN" | "N2.MIN" | "N3.MIN" | "N4.MIN" => {
            if argc < 1 {
                return Err(anyhow::anyhow!("{} requires at least 1 argument", command));
            }
            Ok(())
        }
        "M" => {
            if argc > 1 {
                return Err(anyhow::anyhow!("M takes 0-1 arguments"));
            }
            Ok(())
        }
        "M.BPM" | "M.ACT" | "M.SCRIPT" => {
            if argc != 1 {
                return Err(anyhow::anyhow!("{} takes exactly 1 argument", command));
            }
            Ok(())
        }
        "TR" | "RST" | "HELP" | "REC" | "REC.STOP" | "CLEAR" | "RND.VOICE" | "RND.OSC" | "RND.FM" | "RND.MOD" | "RND.ENV" | "RND.FX" | "RND.FILT" | "RND.DLY" | "RND.VERB" => {
            if argc > 0 {
                return Err(anyhow::anyhow!("{} takes no arguments", command));
            }
            Ok(())
        }
        "RND.P" | "RND.PALL" => {
            if argc != 0 && argc != 2 {
                return Err(anyhow::anyhow!("{} takes 0 or 2 arguments (min and max)", command));
            }
            Ok(())
        }
        "RND.PN" => {
            if argc != 1 && argc != 3 {
                return Err(anyhow::anyhow!("RND.PN takes 1 argument (pattern number) or 3 arguments (pattern number, min, max)"));
            }
            Ok(())
        }
        "REC.PATH" => {
            if argc != 1 {
                return Err(anyhow::anyhow!("REC.PATH takes exactly 1 argument"));
            }
            Ok(())
        }
        "PRINT" => {
            if argc < 1 {
                return Err(anyhow::anyhow!("PRINT requires at least 1 argument"));
            }
            Ok(())
        }
        "VOL" => {
            if argc != 1 {
                return Err(anyhow::anyhow!("VOL takes exactly 1 argument"));
            }
            Ok(())
        }
        "SLEW" => {
            if argc != 2 {
                return Err(anyhow::anyhow!("SLEW takes exactly 2 arguments (param name and time value)"));
            }
            Ok(())
        }
        "SLEW.ALL" => {
            if argc != 1 {
                return Err(anyhow::anyhow!("SLEW.ALL takes exactly 1 argument"));
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
                return Err(anyhow::anyhow!("{} requires at least 1 argument", command));
            }
            Ok(())
        }
        "ENV.ATK" | "ENV.DEC" | "ENV.CRV" | "ENV.MODE" => {
            if argc != 1 {
                return Err(anyhow::anyhow!("{} takes exactly 1 argument", command));
            }
            Ok(())
        }
        "AENV.ATK" | "AENV.CRV" | "AENV.MODE" | "AENV.GATE" => {
            if argc != 1 {
                return Err(anyhow::anyhow!("{} takes exactly 1 argument", command));
            }
            Ok(())
        }
        "PENV.ATK" | "PENV.CRV" | "PENV.MODE" | "PENV.GATE" => {
            if argc != 1 {
                return Err(anyhow::anyhow!("{} takes exactly 1 argument", command));
            }
            Ok(())
        }
        "FMEV.ATK" | "FMEV.CRV" | "FMEV.MODE" | "FMEV.GATE" => {
            if argc != 1 {
                return Err(anyhow::anyhow!("{} takes exactly 1 argument", command));
            }
            Ok(())
        }
        "DENV.ATK" | "DENV.CRV" | "DENV.MODE" | "DENV.GATE" => {
            if argc != 1 {
                return Err(anyhow::anyhow!("{} takes exactly 1 argument", command));
            }
            Ok(())
        }
        "FBEV.ATK" | "FBEV.CRV" | "FBEV.MODE" | "FBEV.GATE" => {
            if argc != 1 {
                return Err(anyhow::anyhow!("{} takes exactly 1 argument", command));
            }
            Ok(())
        }
        "FLEV.ATK" | "FLEV.CRV" | "FLEV.MODE" | "FLEV.GATE" => {
            if argc != 1 {
                return Err(anyhow::anyhow!("{} takes exactly 1 argument", command));
            }
            Ok(())
        }
        "GATE" => {
            if argc != 1 {
                return Err(anyhow::anyhow!("GATE takes exactly 1 argument"));
            }
            Ok(())
        }
        "Q.ROOT" | "Q.SCALE" => {
            if argc != 1 {
                return Err(anyhow::anyhow!("{} takes exactly 1 argument", command));
            }
            Ok(())
        }
        "Q.BIT" => {
            if argc != 1 {
                return Err(anyhow::anyhow!("Q.BIT takes exactly 1 argument (binary string)"));
            }
            Ok(())
        }
        "DEL.CLR" => {
            if argc > 0 {
                return Err(anyhow::anyhow!("DEL.CLR takes no arguments"));
            }
            Ok(())
        }
        _ => {
            Err(anyhow::anyhow!("Unknown command: {}", command))
        }
    }
}
