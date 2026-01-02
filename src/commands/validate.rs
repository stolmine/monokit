use anyhow::Result;

use super::resolve_alias;
use super::validate_expr::validate_expression;

/// Validates SEQ pattern content for bracket balancing and valid tokens
pub fn validate_seq_pattern_content(pattern: &str) -> Result<()> {
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
pub fn validate_loop_syntax(line: &str) -> Result<()> {
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
pub fn validate_conditional_syntax(prefix: &str, rest: &str) -> Result<()> {
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
pub fn validate_del_syntax(line: &str) -> Result<()> {
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
pub fn validate_all_pn_expressions(parts: &[&str], start_idx: usize) -> Result<()> {
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

/// Legacy validation function kept for tests only.
/// Production code should use `validate_from_registry` in `commands::registry::validate`.
#[cfg(test)]
pub fn validate_script_command(cmd: &str) -> Result<()> {
    use crate::commands::registry::validate::validate_from_registry;
    validate_from_registry(cmd)
}
