//! Registry-based command validation

use anyhow::Result;
use super::{COMMAND_REGISTRY, ArgCount};
use crate::commands::resolve_alias;
use crate::commands::validate::{
    validate_seq_pattern_content,
    validate_loop_syntax,
    validate_conditional_syntax,
    validate_del_syntax,
    validate_all_pn_expressions,
};
use crate::commands::validate_expr::validate_expression;

/// Validates that a pattern number is in range 0-5.
fn validate_pattern_number(val: &str) -> Result<()> {
    if let Ok(num) = val.parse::<i16>() {
        if num < 0 || num > 5 {
            return Err(anyhow::anyhow!("PATTERN NUMBER MUST BE 0-5"));
        }
    }
    Ok(())
}

/// Validates that a pattern index is in range 0-63.
fn validate_pattern_index(val: &str) -> Result<()> {
    if let Ok(num) = val.parse::<i16>() {
        if num < 0 || num > 63 {
            return Err(anyhow::anyhow!("PATTERN INDEX MUST BE 0-63"));
        }
    }
    Ok(())
}

/// Validates that a script reference is valid: 1-8, M, or I.
fn validate_script_reference(val: &str) -> Result<()> {
    let upper = val.to_uppercase();

    if upper == "M" || upper == "I" {
        return Ok(());
    }

    if let Ok(num) = val.parse::<i16>() {
        if num < 1 || num > 8 {
            return Err(anyhow::anyhow!("SCRIPT MUST BE 1-8, M, OR I"));
        }
    }

    Ok(())
}

/// Validate a script command using the command registry
/// Returns Ok(()) if valid, Err with message if invalid
pub fn validate_from_registry(line: &str) -> Result<()> {
    let trimmed = line.trim();
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
        if let Some(seq_pos) = upper.find("SEQ ") {
            let after_seq_start = seq_pos + 4;
            if after_seq_start < trimmed.len() {
                let remaining = &trimmed[after_seq_start..];
                let remaining_trimmed = remaining.trim_start();

                if remaining_trimmed.is_empty() {
                    return Err(anyhow::anyhow!("SEQ REQUIRES A QUOTED PATTERN"));
                }

                if remaining_trimmed.starts_with('"') {
                    if !remaining_trimmed.ends_with('"') || remaining_trimmed.len() == 1 {
                        return Err(anyhow::anyhow!("SEQ HAS UNCLOSED QUOTE"));
                    }
                    if remaining_trimmed.len() == 2 {
                        return Err(anyhow::anyhow!("SEQ REQUIRES A QUOTED PATTERN"));
                    }
                    let inner = &remaining_trimmed[1..remaining_trimmed.len()-1];
                    validate_seq_pattern_content(inner)?;
                } else if remaining_trimmed.starts_with('\'') {
                    if !remaining_trimmed.ends_with('\'') || remaining_trimmed.len() == 1 {
                        return Err(anyhow::anyhow!("SEQ HAS UNCLOSED QUOTE"));
                    }
                    if remaining_trimmed.len() == 2 {
                        return Err(anyhow::anyhow!("SEQ REQUIRES A QUOTED PATTERN"));
                    }
                    let inner = &remaining_trimmed[1..remaining_trimmed.len()-1];
                    validate_seq_pattern_content(inner)?;
                } else {
                    return Err(anyhow::anyhow!("SEQ REQUIRES A QUOTED PATTERN"));
                }
            } else {
                return Err(anyhow::anyhow!("SEQ REQUIRES A QUOTED PATTERN"));
            }
        }
        return Ok(());
    }

    // Check for loop syntax
    if trimmed.to_uppercase().starts_with("L ") {
        return validate_loop_syntax(trimmed);
    }

    // Check for DEL commands - they MUST have a colon
    if upper.starts_with("DEL ") || upper.starts_with("DEL.X ") || upper.starts_with("DEL.R ") {
        if !trimmed.contains(':') {
            return Err(anyhow::anyhow!("DEL REQUIRES : BEFORE COMMAND"));
        }
        return validate_del_syntax(trimmed);
    }

    // Check for conditional and DEL syntax
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

    // Allow semicolon-separated compound commands
    if trimmed.contains(';') {
        return Ok(());
    }

    // Parse command parts
    let parts: Vec<&str> = trimmed.split_whitespace().collect();
    if parts.is_empty() {
        return Ok(());
    }

    // Resolve command alias
    let command = resolve_alias(&parts[0].to_uppercase());
    let argc = parts.len() - 1;

    // Look up in registry
    let cmd_def = COMMAND_REGISTRY.get(command.as_str());

    if cmd_def.is_none() {
        return Err(anyhow::anyhow!("UNKNOWN COMMAND: {}", command));
    }

    let cmd_def = cmd_def.unwrap();

    // Validate argument count
    match &cmd_def.args {
        ArgCount::None => {
            if argc > 0 {
                return Err(anyhow::anyhow!("{} TAKES NO ARGUMENTS", command));
            }
        }
        ArgCount::Exactly(n) => {
            if argc != *n {
                return Err(anyhow::anyhow!("{} TAKES EXACTLY {} ARGUMENT{}",
                    command, n, if *n == 1 { "" } else { "S" }));
            }
        }
        ArgCount::AtLeast(n) => {
            if argc < *n {
                return Err(anyhow::anyhow!("{} REQUIRES AT LEAST {} ARGUMENT{}",
                    command, n, if *n == 1 { "" } else { "S" }));
            }
        }
        ArgCount::Range(min, max) => {
            if argc < *min || argc > *max {
                if min == max {
                    return Err(anyhow::anyhow!("{} TAKES EXACTLY {} ARGUMENT{}",
                        command, min, if *min == 1 { "" } else { "S" }));
                } else {
                    return Err(anyhow::anyhow!("{} TAKES {}-{} ARGUMENTS",
                        command, min, max));
                }
            }
        }
        ArgCount::Custom => {
            // Commands with custom validation handled below
        }
    }

    // Handle special validation for specific commands
    if cmd_def.special_validation {
        // Variable assignment - validate expression consumes all tokens
        if matches!(command.as_str(), "A" | "B" | "C" | "D" | "I" | "J" | "K" | "X" | "Y" | "Z" | "T") {
            if argc > 0 {
                let consumed = validate_expression(&parts, 1)?;
                if consumed != argc {
                    return Err(anyhow::anyhow!("EXTRA TOKENS AFTER EXPRESSION"));
                }
            }
        }
    }

    // Command-specific argument validations
    match command.as_str() {
        "P" => {
            // Validate pattern index (0-63)
            if argc >= 1 {
                validate_pattern_index(parts[1])?;
            }
        }
        "P.N" => {
            // Validate pattern number (0-5)
            if argc >= 1 {
                validate_pattern_number(parts[1])?;
            }
        }
        "PN" => {
            // Validate pattern number and index
            if argc >= 1 {
                validate_pattern_number(parts[1])?;
            }
            if argc >= 2 {
                validate_pattern_index(parts[2])?;
            }
        }
        "PN.HERE" | "PN.NEXT" | "PN.PREV" | "PN.POP" | "PN.REV" | "PN.SHUF" | "PN.SORT" |
        "PN.MIN" | "PN.MAX" | "PN.SUM" | "PN.AVG" | "PN.RND" |
        "PN.L" | "PN.I" => {
            // Validate pattern number (first arg)
            if argc >= 1 {
                validate_pattern_number(parts[1])?;
            }
        }
        "PN.PUSH" | "PN.ADD" | "PN.SUB" | "PN.MUL" | "PN.DIV" | "PN.MOD" | "PN.FND" |
        "PN.RM" | "PN.ROT" => {
            // Validate pattern number (first arg)
            if argc >= 1 {
                validate_pattern_number(parts[1])?;
            }
        }
        "PN.INS" | "PN.SCALE" => {
            // Validate pattern number (first arg)
            if argc >= 1 {
                validate_pattern_number(parts[1])?;
            }
        }
        "SCRIPT" | "$" => {
            // Validate script reference (1-8, M, or I)
            if argc == 1 {
                validate_script_reference(parts[1])?;
            }
        }
        "M.SCRIPT" => {
            // Validate script reference
            if argc >= 1 {
                validate_script_reference(parts[1])?;
            }
        }
        "MUTE" => {
            // Validate script reference for MUTE <script>
            if argc == 1 {
                validate_script_reference(parts[1])?;
            }
        }
        "TOG" => {
            // Validate that TOG has two different values
            if argc >= 2 && parts[1] == parts[2] {
                return Err(anyhow::anyhow!("TOG REQUIRES TWO DIFFERENT VALUES"));
            }
        }
        "PAGE" | "PG" => {
            // Validate page argument
            if argc >= 1 {
                let page_arg = parts[1].to_uppercase();
                match page_arg.as_str() {
                    "LIVE" | "L" | "HELP" | "H" | "GRID" | "G" |
                    "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" |
                    "M" | "I" | "P" | "V" | "N" | "S" => {}
                    _ => return Err(anyhow::anyhow!("INVALID PAGE \"{}\"", page_arg))
                }
            }
        }
        _ => {}
    }

    // Validate PN.* expressions used as arguments
    validate_all_pn_expressions(&parts, 1)?;

    Ok(())
}
