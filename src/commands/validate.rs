use anyhow::Result;

pub fn validate_script_command(cmd: &str) -> Result<()> {
    let trimmed = cmd.trim();
    if trimmed.is_empty() {
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

    let command = parts[0].to_uppercase();
    let argc = parts.len() - 1;

    match command.as_str() {
        "A" | "B" | "C" | "D" | "I" | "X" | "Y" | "Z" | "T" | "J" | "K" => {
            Ok(())
        }
        "P.HERE" | "P.NEXT" | "P.PREV" => {
            if argc > 0 {
                return Err(anyhow::anyhow!("{} takes no arguments", command));
            }
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
        "PN.HERE" | "PN.NEXT" | "PN.PREV" => {
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
        "EQ" | "NE" | "GT" | "LT" | "GTE" | "LTE" => {
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
        "THEME" | "DEBUG" => {
            if argc > 1 {
                return Err(anyhow::anyhow!("{} takes 0-1 arguments", command));
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
        "TR" | "RST" | "HELP" | "REC" | "REC.STOP" | "CLEAR" => {
            if argc > 0 {
                return Err(anyhow::anyhow!("{} takes no arguments", command));
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
        "PF" | "MF" | "PW" | "MW" | "DC" | "TK" | "MB" | "FM" | "MX" | "FA" | "DA" | "DM" | "MP" | "MD" | "MT" | "MA" | "MM" | "ME" | "AD" | "PD" | "FD" | "DD" | "PA" | "FB" | "FBA" | "FBD" | "RF" | "RD" | "RM" | "RK" | "DT" | "DF" | "DLP" | "DW" | "DS" | "RV" | "RP" | "RH" | "RW" | "FC" | "FQ" | "FT" | "FE" | "FED" | "FK" | "MF.F" | "BR.ACT" | "BR.LEN" | "BR.REV" | "BR.WIN" | "BR.MIX" | "PS.MODE" | "PS.SEMI" | "PS.GRAIN" | "PS.MIX" | "PS.TARG" => {
            if argc < 1 {
                return Err(anyhow::anyhow!("{} requires at least 1 argument", command));
            }
            Ok(())
        }
        _ => {
            Err(anyhow::anyhow!("Unknown command: {}", command))
        }
    }
}
