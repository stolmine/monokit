use anyhow::Result;

use super::resolve_alias;
use super::validate_expr::validate_expression;

/// Validates SEQ pattern content for bracket balancing and valid tokens
fn validate_seq_pattern_content(pattern: &str) -> Result<()> {
    if pattern.trim().is_empty() {
        return Err(anyhow::anyhow!("SEQ: EMPTY PATTERN"));
    }

    let mut angle_depth = 0;
    let mut curly_depth = 0;
    let mut in_angle = false;
    let mut in_curly = false;

    for ch in pattern.chars() {
        match ch {
            '<' => {
                if in_angle || in_curly {
                    return Err(anyhow::anyhow!("SEQ: NESTED BRACKETS NOT ALLOWED"));
                }
                angle_depth += 1;
                in_angle = true;
            }
            '>' => {
                if angle_depth == 0 {
                    return Err(anyhow::anyhow!("SEQ: UNCLOSED < BRACKET"));
                }
                angle_depth -= 1;
                if angle_depth == 0 {
                    in_angle = false;
                }
            }
            '{' => {
                if in_angle || in_curly {
                    return Err(anyhow::anyhow!("SEQ: NESTED BRACKETS NOT ALLOWED"));
                }
                curly_depth += 1;
                in_curly = true;
            }
            '}' => {
                if curly_depth == 0 {
                    return Err(anyhow::anyhow!("SEQ: UNCLOSED {{ BRACKET"));
                }
                curly_depth -= 1;
                if curly_depth == 0 {
                    in_curly = false;
                }
            }
            _ => {}
        }
    }

    if angle_depth > 0 {
        return Err(anyhow::anyhow!("SEQ: UNCLOSED < BRACKET"));
    }
    if curly_depth > 0 {
        return Err(anyhow::anyhow!("SEQ: UNCLOSED {{ BRACKET"));
    }

    let tokens: Vec<&str> = pattern.split_whitespace().collect();
    for token in tokens {
        validate_seq_token(token)?;
    }

    Ok(())
}

/// Validates a single SEQ token (handles brackets, repetition, and basic tokens)
fn validate_seq_token(token: &str) -> Result<()> {
    if token.is_empty() {
        return Ok(());
    }

    let (base_token, repeat_suffix) = if let Some(star_pos) = token.rfind('*') {
        let base = &token[..star_pos];
        let count_str = &token[star_pos + 1..];
        if count_str.is_empty() {
            return Err(anyhow::anyhow!("SEQ: INVALID REPEAT \"{}\"", token));
        }
        if !count_str.chars().all(|c| c.is_ascii_digit()) {
            return Err(anyhow::anyhow!("SEQ: INVALID REPEAT \"*{}\"", count_str));
        }
        (base, Some(count_str))
    } else {
        (token, None)
    };

    if base_token.starts_with('<') && base_token.ends_with('>') {
        let inner = &base_token[1..base_token.len()-1];
        for inner_token in inner.split_whitespace() {
            validate_basic_seq_token(inner_token)?;
        }
        return Ok(());
    }

    if base_token.starts_with('{') && base_token.ends_with('}') {
        let inner = &base_token[1..base_token.len()-1];
        for inner_token in inner.split_whitespace() {
            validate_basic_seq_token(inner_token)?;
        }
        return Ok(());
    }

    validate_basic_seq_token(base_token)?;

    if let Some(count_str) = repeat_suffix {
        if count_str.parse::<usize>().is_err() {
            return Err(anyhow::anyhow!("SEQ: INVALID REPEAT \"*{}\"", count_str));
        }
    }

    Ok(())
}

/// Validates a basic SEQ token (no brackets, no repetition)
fn validate_basic_seq_token(token: &str) -> Result<()> {
    if token.is_empty() {
        return Ok(());
    }

    match token {
        "_" | "." | "x" | "X" | "?" => return Ok(()),
        _ => {}
    }

    if token.parse::<i16>().is_ok() {
        return Ok(());
    }

    if is_valid_note_name(token) {
        return Ok(());
    }

    Err(anyhow::anyhow!("SEQ: INVALID TOKEN \"{}\"", token))
}

/// Validates if a token is a valid note name (C0-B9, with optional # or b)
fn is_valid_note_name(s: &str) -> bool {
    let s = s.to_uppercase();
    let mut chars = s.chars().peekable();

    let note = match chars.next() {
        Some('C') | Some('D') | Some('E') | Some('F') | Some('G') | Some('A') | Some('B') => {}
        _ => return false,
    };

    match chars.peek() {
        Some('#') => {
            chars.next();
        }
        Some('B') => {
            let mut lookahead = chars.clone();
            lookahead.next();
            if lookahead.peek().map_or(false, |c| c.is_ascii_digit()) {
                chars.next();
            }
        }
        _ => {}
    }

    let octave_str: String = chars.collect();
    if octave_str.is_empty() {
        return false;
    }

    if let Ok(octave) = octave_str.parse::<i16>() {
        octave >= 0 && octave <= 9
    } else {
        false
    }
}

/// Validates loop syntax: L <start> <end>: <commands>
fn validate_loop_syntax(line: &str) -> Result<()> {
    let colon_pos = line.find(':').ok_or_else(|| anyhow::anyhow!("L REQUIRES : SEPARATOR"))?;

    let loop_part = line[2..colon_pos].trim();
    let parts: Vec<&str> = loop_part.split_whitespace().collect();

    if parts.len() < 2 {
        return Err(anyhow::anyhow!("L REQUIRES START AND END"));
    }

    validate_expression(&parts, 0)?;
    validate_expression(&parts, 1)?;

    let commands = line[colon_pos + 1..].trim();
    if commands.is_empty() {
        return Err(anyhow::anyhow!("L REQUIRES COMMAND AFTER :"));
    }

    Ok(())
}

/// Validates conditional syntax: IF/ELIF/ELSE/PROB/EV/SKIP
fn validate_conditional_syntax(prefix: &str, rest: &str) -> Result<()> {
    let colon_pos = rest.find(':').ok_or_else(|| {
        anyhow::anyhow!("{} REQUIRES : BEFORE COMMAND", prefix)
    })?;

    let condition_part = rest[..colon_pos].trim();
    let command_part = rest[colon_pos + 1..].trim();

    if command_part.is_empty() {
        return Err(anyhow::anyhow!("{} REQUIRES COMMAND AFTER :", prefix));
    }

    if prefix != "ELSE" && !condition_part.is_empty() {
        let parts: Vec<&str> = condition_part.split_whitespace().collect();
        if parts.is_empty() {
            return Err(anyhow::anyhow!("INVALID CONDITION EXPRESSION"));
        }
        validate_expression(&parts, 0)?;
    }

    Ok(())
}

/// Validates DEL command variants: DEL, DEL.X, DEL.R
fn validate_del_syntax(line: &str) -> Result<()> {
    let upper = line.to_uppercase();
    let colon_pos = line.find(':').ok_or_else(|| {
        anyhow::anyhow!("DEL REQUIRES : BEFORE COMMAND")
    })?;

    let before_colon = line[..colon_pos].trim();
    let after_colon = line[colon_pos + 1..].trim();

    if after_colon.is_empty() {
        return Err(anyhow::anyhow!("DEL REQUIRES COMMAND AFTER :"));
    }

    let parts: Vec<&str> = before_colon.split_whitespace().collect();

    if upper.starts_with("DEL.X ") || upper.starts_with("DEL.R ") {
        if parts.len() < 3 {
            return Err(anyhow::anyhow!("DEL.X/R REQUIRE COUNT AND TIME"));
        }
        validate_expression(&parts, 1)?;
        validate_expression(&parts, 2)?;
    } else if upper.starts_with("DEL ") {
        if parts.len() < 2 {
            return Err(anyhow::anyhow!("DEL REQUIRES TIME"));
        }
        validate_expression(&parts, 1)?;
    }

    Ok(())
}

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

/// Validates that a pattern number is in range 0-5.
/// Only validates static numeric values - accepts variables/expressions.
fn validate_pattern_number(val: &str) -> Result<()> {
    if let Ok(num) = val.parse::<i16>() {
        if num < 0 || num > 5 {
            return Err(anyhow::anyhow!("PATTERN NUMBER MUST BE 0-5"));
        }
    }
    Ok(())
}

/// Validates that a pattern index is in range 0-63.
/// Only validates static numeric values - accepts variables/expressions.
fn validate_pattern_index(val: &str) -> Result<()> {
    if let Ok(num) = val.parse::<i16>() {
        if num < 0 || num > 63 {
            return Err(anyhow::anyhow!("PATTERN INDEX MUST BE 0-63"));
        }
    }
    Ok(())
}

/// Validates that a script reference is valid: 1-8, M, or I.
/// Only validates static values - accepts variables/expressions.
fn validate_script_reference(val: &str) -> Result<()> {
    let upper = val.to_uppercase();

    // Accept M or I
    if upper == "M" || upper == "I" {
        return Ok(());
    }

    // Try to parse as number
    if let Ok(num) = val.parse::<i16>() {
        if num < 1 || num > 8 {
            return Err(anyhow::anyhow!("SCRIPT MUST BE 1-8, M, OR I"));
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
                    // Validate pattern content
                    let inner = &remaining_trimmed[1..remaining_trimmed.len()-1];
                    validate_seq_pattern_content(inner)?;
                } else if remaining_trimmed.starts_with('\'') {
                    if !remaining_trimmed.ends_with('\'') || remaining_trimmed.len() == 1 {
                        return Err(anyhow::anyhow!("SEQ HAS UNCLOSED QUOTE"));
                    }
                    // Check for empty pattern
                    if remaining_trimmed.len() == 2 {
                        return Err(anyhow::anyhow!("SEQ REQUIRES A QUOTED PATTERN"));
                    }
                    // Validate pattern content
                    let inner = &remaining_trimmed[1..remaining_trimmed.len()-1];
                    validate_seq_pattern_content(inner)?;
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

    if trimmed.to_uppercase().starts_with("L ") {
        return validate_loop_syntax(trimmed);
    }

    if trimmed.contains(':') {
        let colon_pos = trimmed.find(':').unwrap();
        let prefix = trimmed[..colon_pos].trim().to_uppercase();

        if prefix.starts_with("IF ") {
            return validate_conditional_syntax("IF", &trimmed[3..]);
        } else if prefix.starts_with("ELIF ") {
            return validate_conditional_syntax("ELIF", &trimmed[5..]);
        } else if prefix == "ELSE" {
            return validate_conditional_syntax("ELSE", &trimmed[4..]);
        } else if prefix.starts_with("PROB ") {
            return validate_conditional_syntax("PROB", &trimmed[5..]);
        } else if prefix.starts_with("EV ") {
            return validate_conditional_syntax("EV", &trimmed[3..]);
        } else if prefix.starts_with("SKIP ") {
            return validate_conditional_syntax("SKIP", &trimmed[5..]);
        } else if prefix.starts_with("DEL.X ") {
            return validate_del_syntax(trimmed);
        } else if prefix.starts_with("DEL.R ") {
            return validate_del_syntax(trimmed);
        } else if prefix.starts_with("DEL ") {
            return validate_del_syntax(trimmed);
        }
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
            // Variable assignment - validate expression consumes all tokens
            if argc > 0 {
                let consumed = validate_expression(&parts, 1)?;
                if consumed != argc {
                    return Err(anyhow::anyhow!("EXTRA TOKENS AFTER EXPRESSION"));
                }
            }
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
            // Validate pattern number for P.N <n>
            if command == "P.N" && argc == 1 {
                validate_pattern_number(parts[1])?;
            }
            Ok(())
        }
        "P" => {
            if argc < 1 {
                return Err(anyhow::anyhow!("P REQUIRES AT LEAST 1 ARGUMENT"));
            }
            // Validate pattern index for P <idx> [val]
            validate_pattern_index(parts[1])?;
            Ok(())
        }
        "PN.HERE" | "PN.NEXT" | "PN.PREV" | "PN.POP" | "PN.REV" | "PN.SHUF" | "PN.SORT" | "PN.MIN" | "PN.MAX" | "PN.SUM" | "PN.AVG" => {
            if argc < 1 {
                return Err(anyhow::anyhow!("{} NEEDS PAT NUM", command));
            }
            // Validate pattern number (first arg)
            validate_pattern_number(parts[1])?;
            Ok(())
        }
        "PN.L" | "PN.I" => {
            if argc < 1 {
                return Err(anyhow::anyhow!("{} REQUIRES AT LEAST 1 ARGUMENT", command));
            }
            // Validate pattern number (first arg)
            validate_pattern_number(parts[1])?;
            Ok(())
        }
        "PN.PUSH" | "PN.ADD" | "PN.SUB" | "PN.MUL" | "PN.DIV" | "PN.MOD" | "PN.FND" => {
            if argc < 2 {
                return Err(anyhow::anyhow!("{} NEEDS PAT NUM AND VAL", command));
            }
            // Validate pattern number (first arg)
            validate_pattern_number(parts[1])?;
            Ok(())
        }
        "PN.RM" => {
            if argc < 2 {
                return Err(anyhow::anyhow!("PN.RM NEEDS PAT NUM AND IDX"));
            }
            // Validate pattern number (first arg)
            validate_pattern_number(parts[1])?;
            Ok(())
        }
        "PN.ROT" => {
            if argc < 2 {
                return Err(anyhow::anyhow!("PN.ROT NEEDS PAT NUM AND AMT"));
            }
            // Validate pattern number (first arg)
            validate_pattern_number(parts[1])?;
            Ok(())
        }
        "PN.INS" => {
            if argc < 3 {
                return Err(anyhow::anyhow!("PN.INS NEEDS PAT NUM, IDX, VAL"));
            }
            // Validate pattern number (first arg)
            validate_pattern_number(parts[1])?;
            Ok(())
        }
        "PN.SCALE" => {
            if argc < 3 {
                return Err(anyhow::anyhow!("PN.SCALE NEEDS PAT, MIN, MAX"));
            }
            // Validate pattern number (first arg)
            validate_pattern_number(parts[1])?;
            Ok(())
        }
        "PN.RND" => {
            if argc < 1 {
                return Err(anyhow::anyhow!("PN.RND NEEDS PAT NUM"));
            }
            // Validate pattern number (first arg)
            validate_pattern_number(parts[1])?;
            Ok(())
        }
        "PN" => {
            if argc < 2 {
                return Err(anyhow::anyhow!("PN REQUIRES AT LEAST 2 ARGUMENTS"));
            }
            // Validate pattern number (first arg)
            validate_pattern_number(parts[1])?;
            // Validate pattern index (second arg)
            validate_pattern_index(parts[2])?;
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
        // Note: NE and EQ comparisons handled in eval, not here. NE=NOISE.ENV, EQ=EQ mid-Q parameter
        "GT" | "LT" | "GTE" | "LTE" => {
            if argc < 2 {
                return Err(anyhow::anyhow!("{} REQUIRES AT LEAST 2 ARGUMENTS", command));
            }
            Ok(())
        }
        "SCRIPT" | "$" => {
            if argc < 1 {
                return Err(anyhow::anyhow!("SCRIPT REQUIRES AT LEAST 1 ARGUMENT"));
            }
            // For single arg, validate script reference
            // Multi-arg case (e.g., SEQ "1 2 3") handled by eval_expression in handler
            if argc == 1 {
                validate_script_reference(parts[1])?;
            }
            Ok(())
        }
        "SAVE" | "LOAD" | "DELETE" => {
            if argc != 1 {
                return Err(anyhow::anyhow!("{} TAKES EXACTLY 1 ARGUMENT", command));
            }
            Ok(())
        }
        "LOAD.RST" => {
            if argc > 1 {
                return Err(anyhow::anyhow!("LOAD.RST TAKES 0-1 ARGUMENTS"));
            }
            Ok(())
        }
        "LOAD.CLR" => {
            if argc > 1 {
                return Err(anyhow::anyhow!("LOAD.CLR TAKES 0-1 ARGUMENTS"));
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
        "SYNC" | "SYNC.SEQ" | "SYNC.TOG" | "SYNC.PAT" | "VERSION" | "VER" => {
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
            // Validate script reference for M.SCRIPT <n>
            if command == "M.SCRIPT" && argc == 1 {
                validate_script_reference(parts[1])?;
            }
            Ok(())
        }
        "TR" | "PLTR" | "RST" | "HELP" | "REC" | "REC.STOP" | "CLEAR" | "CLR" | "BRK" | "CL.TRIG" | "CLTR" | "RND.VOICE" | "RND.OSC" | "RND.FM" | "RND.MOD" | "RND.ENV" | "RND.PL" | "RND.FX" | "RND.FILT" | "RND.DLY" | "RND.VERB" => {
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
        "TK" | "MB" | "MBA" | "MBD" | "FM" | "MX" | "MM" | "ME" | "MP" | "MD" | "MT" | "MA" | "MC" | "MQ" |
        // Envelopes (amounts and decays)
        "AD" | "PD" | "FD" | "DD" | "PA" | "FA" | "DA" |
        // Noise controls
        "NW" | "NA" | "ND" | "NC" | "NE" | "NP" | "NM" | "NV" | "NG" |
        // Source levels
        "PV" | "MV" | "PLV" | "PAV" |
        // Plaits
        "PL.ENG" | "PLE" | "PL.FREQ" | "PLF" | "PL.HARM" | "PLH" | "PL.TIMB" | "PLT" | "PL.MORPH" | "PLM" | "PL.DEC" | "PLD" | "PL.LPG" | "PLL" |
        // Filter
        "FC" | "FQ" | "FT" | "FE" | "FED" | "FK" | "MFF" | "MFQ" |
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
        "CT" | "CR" | "CA" | "CL" | "CM" | "CR.MIX" | "CRMIX" |
        // EQ (EL=low, EM=mid, EH=high, EF=freq, EQ=Q bandwidth)
        "EL" | "EM" | "EH" | "EF" | "EQ" |
        // Pan
        "PAN" |
        // Beat Repeat
        "BR.LEN" | "BRL" | "BR.REV" | "BRR" | "BR.WIN" | "BRW" | "BR.MIX" | "BRX" |
        // Pitch Shift
        "PS.MODE" | "PSM" | "PS.SEMI" | "PSS" | "PS.GRAIN" | "PSG" | "PS.MIX" | "PSX" | "PS.TARG" | "PST" |
        // Clouds
        "CL.PITCH" | "CLP" | "CLPT" | "CL.POS" | "CLO" | "CLPS" | "CL.SIZE" | "CLS" | "CLSZ" |
        "CL.DENS" | "CLD" | "CLDS" | "CL.TEX" | "CLT" | "CLTX" | "CL.WET" | "CLW" |
        "CL.GAIN" | "CLG" | "CL.SPREAD" | "CLSP" | "CL.RVB" | "CLRV" |
        "CL.FB" | "CLF" | "CL.FREEZE" | "CLFZ" | "CL.MODE" | "CLM" | "CL.LOFI" | "CLLO" => {
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
        "FLASH" | "TITLE" | "LIMIT" | "VCA" => {
            if argc > 1 {
                return Err(anyhow::anyhow!("{} TAKES 0-1 ARGUMENTS", command));
            }
            Ok(())
        }
        "TITLE.TIMER" => {
            if argc == 0 {
                Ok(())
            } else if argc == 1 {
                Ok(())
            } else if argc == 2 {
                Ok(())
            } else {
                Err(anyhow::anyhow!("TITLE.TIMER TAKES 0, 1, OR 2 ARGUMENTS"))
            }
        }
        "METER.HDR" | "METER.GRID" | "METER.ASCII" | "ACTIVITY" | "GRID" | "GRID.DEF" | "GRID.MODE" | "HL.COND" | "HL.SEQ" | "SPECTRUM" | "SCRMBL" | "SCRMBL.MODE" | "SCRMBL.SPD" | "SCRMBL.CRV" | "COMPAT.MODE" | "AUTOLOAD" => {
            if argc > 1 {
                return Err(anyhow::anyhow!("{} TAKES 0-1 ARGUMENTS", command));
            }
            Ok(())
        }
        "MIDI.IN" | "MIDI" => {
            Ok(())
        }
        "SC.DIAG" | "MIDI.DIAG" => {
            if argc > 2 {
                return Err(anyhow::anyhow!("{} TAKES 1-2 ARGUMENTS", command));
            }
            Ok(())
        }
        "AUDIO.OUT" | "AUDIO" => {
            Ok(())
        }
        "COMPAT" => {
            if argc > 0 {
                return Err(anyhow::anyhow!("COMPAT TAKES NO ARGUMENTS"));
            }
            Ok(())
        }
        "MUTE" | "MUTE.1" | "MUTE.2" | "MUTE.3" | "MUTE.4" | "MUTE.5" | "MUTE.6" | "MUTE.7" | "MUTE.8" | "MUTE.M" | "MUTE.I" => {
            if command.starts_with("MUTE.") {
                if argc > 1 {
                    return Err(anyhow::anyhow!("{} TAKES 0-1 ARGUMENTS", command));
                }
                Ok(())
            } else if argc > 2 {
                return Err(anyhow::anyhow!("MUTE TAKES 0-2 ARGUMENTS"));
            } else if argc == 1 {
                validate_script_reference(parts[1])?;
                Ok(())
            } else {
                Ok(())
            }
        }
        "PAGE" | "PG" => {
            if argc != 1 {
                return Err(anyhow::anyhow!("PAGE TAKES EXACTLY 1 ARGUMENT"));
            }
            let page_arg = parts[1].to_uppercase();
            match page_arg.as_str() {
                "LIVE" | "L" | "HELP" | "H" | "GRID" | "G" |
                "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" |
                "M" | "I" | "P" | "V" | "N" | "S" => Ok(()),
                _ => Err(anyhow::anyhow!("INVALID PAGE \"{}\"", page_arg))
            }
        }
        _ => {
            Err(anyhow::anyhow!("UNKNOWN COMMAND: {}", command))
        }
    }?;

    // Validate PN.* expressions used as arguments
    validate_all_pn_expressions(&parts, 1)?;

    Ok(())
}
