use anyhow::Result;

use super::resolve_alias;

fn is_variable(token: &str) -> bool {
    matches!(
        token,
        "A" | "B" | "C" | "D" | "I" | "J" | "K" | "X" | "Y" | "Z" | "T"
    )
}

fn is_literal(token: &str) -> bool {
    token.parse::<i16>().is_ok()
}

pub fn validate_expression(parts: &[&str], start: usize) -> Result<usize> {
    if start >= parts.len() {
        return Err(anyhow::anyhow!("INVALID EXPRESSION"));
    }

    let token = parts[start].to_uppercase();
    let token = resolve_alias(&token);

    match token.as_str() {
        "ADD" | "SUB" | "MUL" | "DIV" | "MOD" | "+" | "-" | "*" | "/" | "%" => {
            if start + 2 >= parts.len() {
                return Err(anyhow::anyhow!("{} REQUIRES 2 ARGUMENTS", token));
            }
            let consumed1 = validate_expression(parts, start + 1)?;
            let consumed2 = validate_expression(parts, start + 1 + consumed1)?;
            Ok(1 + consumed1 + consumed2)
        }
        "EZ" | "NZ" => {
            if start + 1 >= parts.len() {
                return Err(anyhow::anyhow!("{} REQUIRES 1 ARGUMENT", token));
            }
            let consumed = validate_expression(parts, start + 1)?;
            Ok(1 + consumed)
        }
        "EQ" | "NE" | "GT" | "LT" | "GTE" | "LTE" => {
            if start + 2 >= parts.len() {
                return Err(anyhow::anyhow!("{} REQUIRES 2 ARGUMENTS", token));
            }
            let consumed1 = validate_expression(parts, start + 1)?;
            let consumed2 = validate_expression(parts, start + 1 + consumed1)?;
            Ok(1 + consumed1 + consumed2)
        }
        "RND" | "Q" | "N" => {
            if start + 1 >= parts.len() {
                return Err(anyhow::anyhow!("{} REQUIRES 1 ARGUMENT", token));
            }
            let consumed = validate_expression(parts, start + 1)?;
            Ok(1 + consumed)
        }
        "RRND" | "EITH" | "TOG" => {
            if start + 2 >= parts.len() {
                return Err(anyhow::anyhow!("{} REQUIRES 2 ARGUMENTS", token));
            }
            let consumed1 = validate_expression(parts, start + 1)?;
            let consumed2 = validate_expression(parts, start + 1 + consumed1)?;
            Ok(1 + consumed1 + consumed2)
        }
        "TOSS" => Ok(1),
        "MAP" => {
            if start + 5 >= parts.len() {
                return Err(anyhow::anyhow!("MAP REQUIRES 5 ARGUMENTS"));
            }
            let consumed1 = validate_expression(parts, start + 1)?;
            let consumed2 = validate_expression(parts, start + 1 + consumed1)?;
            let consumed3 = validate_expression(parts, start + 1 + consumed1 + consumed2)?;
            let consumed4 = validate_expression(parts, start + 1 + consumed1 + consumed2 + consumed3)?;
            let consumed5 = validate_expression(parts, start + 1 + consumed1 + consumed2 + consumed3 + consumed4)?;
            Ok(1 + consumed1 + consumed2 + consumed3 + consumed4 + consumed5)
        }
        "SEQ" => {
            if start + 1 >= parts.len() {
                return Err(anyhow::anyhow!("SEQ REQUIRES A QUOTED PATTERN"));
            }
            let first = parts[start + 1];
            let quote_char = if first.starts_with('"') {
                '"'
            } else if first.starts_with('\'') {
                '\''
            } else {
                return Err(anyhow::anyhow!("SEQ REQUIRES A QUOTED PATTERN"));
            };

            if first.len() > 1 && first.ends_with(quote_char) {
                return Ok(2);
            }

            let mut consumed = 1;
            for i in (start + 2)..parts.len() {
                consumed += 1;
                if parts[i].ends_with(quote_char) {
                    return Ok(1 + consumed);
                }
            }

            Err(anyhow::anyhow!("SEQ HAS UNCLOSED QUOTE"))
        }
        "N1" | "N2" | "N3" | "N4" => Ok(1),
        "P.HERE" | "P.NEXT" | "P.PREV" | "P.L" | "P.I" | "P.N" |
        "P.MIN" | "P.MAX" | "P.SUM" | "P.AVG" | "P.POP" | "P.REV" |
        "P.SHUF" | "P.SORT" | "P.CLR" => Ok(1),
        "P.PUSH" | "P.ROT" | "P.ADD" | "P.SUB" | "P.MUL" | "P.DIV" |
        "P.MOD" | "P.FND" | "P.RM" | "P.INS" => {
            if start + 1 >= parts.len() {
                return Err(anyhow::anyhow!("{} REQUIRES AT LEAST 1 ARGUMENT", token));
            }
            let consumed = validate_expression(parts, start + 1)?;
            Ok(1 + consumed)
        }
        "P.SCALE" => {
            if start + 2 >= parts.len() {
                return Err(anyhow::anyhow!("P.SCALE NEEDS MIN AND MAX"));
            }
            let consumed1 = validate_expression(parts, start + 1)?;
            let consumed2 = validate_expression(parts, start + 1 + consumed1)?;
            Ok(1 + consumed1 + consumed2)
        }
        "P.RND" => {
            Ok(1)
        }
        "PN.HERE" | "PN.NEXT" | "PN.PREV" | "PN.L" | "PN.I" |
        "PN.MIN" | "PN.MAX" | "PN.SUM" | "PN.AVG" | "PN.POP" |
        "PN.REV" | "PN.SHUF" | "PN.SORT" | "PN.CLR" | "PN.RND" => {
            if start + 1 >= parts.len() {
                return Err(anyhow::anyhow!("{} NEEDS PAT NUM", token));
            }
            let consumed = validate_expression(parts, start + 1)?;
            Ok(1 + consumed)
        }
        "PN.PUSH" | "PN.ADD" | "PN.SUB" | "PN.MUL" | "PN.DIV" |
        "PN.MOD" | "PN.FND" | "PN.ROT" => {
            if start + 2 >= parts.len() {
                return Err(anyhow::anyhow!("{} NEEDS PAT NUM AND VAL", token));
            }
            let consumed1 = validate_expression(parts, start + 1)?;
            let consumed2 = validate_expression(parts, start + 1 + consumed1)?;
            Ok(1 + consumed1 + consumed2)
        }
        "PN.RM" => {
            if start + 2 >= parts.len() {
                return Err(anyhow::anyhow!("PN.RM NEEDS PAT NUM AND IDX"));
            }
            let consumed1 = validate_expression(parts, start + 1)?;
            let consumed2 = validate_expression(parts, start + 1 + consumed1)?;
            Ok(1 + consumed1 + consumed2)
        }
        "PN.INS" => {
            if start + 3 >= parts.len() {
                return Err(anyhow::anyhow!("PN.INS NEEDS PAT NUM, IDX, VAL"));
            }
            let consumed1 = validate_expression(parts, start + 1)?;
            let consumed2 = validate_expression(parts, start + 1 + consumed1)?;
            let consumed3 = validate_expression(parts, start + 1 + consumed1 + consumed2)?;
            Ok(1 + consumed1 + consumed2 + consumed3)
        }
        "PN.SCALE" => {
            if start + 3 >= parts.len() {
                return Err(anyhow::anyhow!("PN.SCALE NEEDS PAT, MIN, MAX"));
            }
            let consumed1 = validate_expression(parts, start + 1)?;
            let consumed2 = validate_expression(parts, start + 1 + consumed1)?;
            let consumed3 = validate_expression(parts, start + 1 + consumed1 + consumed2)?;
            Ok(1 + consumed1 + consumed2 + consumed3)
        }
        "PN" => {
            if start + 2 >= parts.len() {
                return Err(anyhow::anyhow!("PN REQUIRES AT LEAST 2 ARGUMENTS"));
            }
            let consumed1 = validate_expression(parts, start + 1)?;
            let consumed2 = validate_expression(parts, start + 1 + consumed1)?;
            Ok(1 + consumed1 + consumed2)
        }
        _ => {
            if is_variable(&token) || is_literal(&token) {
                Ok(1)
            } else {
                Err(anyhow::anyhow!("INVALID EXPRESSION"))
            }
        }
    }
}
