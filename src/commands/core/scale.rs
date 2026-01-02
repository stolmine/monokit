use crate::types::{Counters, PatternStorage, ScaleState, ScriptStorage, Variables, TIER_ESSENTIAL};

pub const SCALE_CHROMATIC: u16 = 0b111111111111;
pub const SCALE_MAJOR: u16 = 0b101010110101;
pub const SCALE_MINOR: u16 = 0b101101010110;
pub const SCALE_DORIAN: u16 = 0b101101011010;
pub const SCALE_PHRYGIAN: u16 = 0b110101011010;
pub const SCALE_LYDIAN: u16 = 0b101010101101;
pub const SCALE_MIXOLYDIAN: u16 = 0b101010110110;
pub const SCALE_PENTATONIC_MAJOR: u16 = 0b101010010100;
pub const SCALE_PENTATONIC_MINOR: u16 = 0b100010110010;
pub const SCALE_BLUES: u16 = 0b100011010010;
pub const SCALE_WHOLE_TONE: u16 = 0b101010101010;
pub const SCALE_DIMINISHED: u16 = 0b101101101101;

pub fn parse_binary_mask(s: &str) -> Option<Vec<bool>> {
    let mut mask = Vec::new();
    for ch in s.chars() {
        match ch {
            '0' => mask.push(false),
            '1' => mask.push(true),
            _ => return None,
        }
    }
    if mask.is_empty() {
        return None;
    }
    Some(mask)
}

pub fn get_preset_mask(preset: u8) -> Vec<bool> {
    let bits: u16 = match preset {
        0 => SCALE_CHROMATIC,
        1 => SCALE_MAJOR,
        2 => SCALE_MINOR,
        3 => SCALE_DORIAN,
        4 => SCALE_PHRYGIAN,
        5 => SCALE_LYDIAN,
        6 => SCALE_MIXOLYDIAN,
        7 => SCALE_PENTATONIC_MAJOR,
        8 => SCALE_PENTATONIC_MINOR,
        9 => SCALE_BLUES,
        10 => SCALE_WHOLE_TONE,
        11 => SCALE_DIMINISHED,
        _ => SCALE_MAJOR,
    };

    (0..12)
        .map(|i| (bits >> i) & 1 == 1)
        .collect()
}

pub fn handle_q_root<F>(
    parts: &[&str],
    variables: &Variables,
    patterns: &mut PatternStorage,
    counters: &mut Counters,
    scripts: &ScriptStorage,
    script_index: usize,
    scale: &mut ScaleState,
    debug_level: u8,
    mut output: F,
) where
    F: FnMut(String),
{
    use crate::eval::eval_expression;

    if parts.len() < 2 {
        output("ERROR: Q.ROOT REQUIRES A VALUE".to_string());
        return;
    }

    let value: i16 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index, scale) {
        expr_val
    } else {
        match parts[1].parse() {
            Ok(v) => v,
            Err(_) => {
                output("ERROR: FAILED TO PARSE VALUE FOR Q.ROOT".to_string());
                return;
            }
        }
    };

    let max_root = (scale.divisions as i16) - 1;
    if value < 0 || value > max_root {
        output(format!("ERROR: Q.ROOT MUST BE 0-{}", max_root));
        return;
    }

    scale.root = value as u8;
    if debug_level >= TIER_ESSENTIAL {
        output(format!("SET SCALE ROOT TO {}", value));
    }
}

pub fn handle_q_scale<F>(
    parts: &[&str],
    variables: &Variables,
    patterns: &mut PatternStorage,
    counters: &mut Counters,
    scripts: &ScriptStorage,
    script_index: usize,
    scale: &mut ScaleState,
    debug_level: u8,
    mut output: F,
) where
    F: FnMut(String),
{
    use crate::eval::eval_expression;

    if parts.len() < 2 {
        output("ERROR: Q.SCALE REQUIRES A VALUE".to_string());
        return;
    }

    let value: i16 = if let Some((expr_val, _)) = eval_expression(&parts, 1, variables, patterns, counters, scripts, script_index, scale) {
        expr_val
    } else {
        match parts[1].parse() {
            Ok(v) => v,
            Err(_) => {
                output("ERROR: FAILED TO PARSE VALUE FOR Q.SCALE".to_string());
                return;
            }
        }
    };

    if value < 0 || value > 11 {
        output("ERROR: Q.SCALE MUST BE 0-11".to_string());
        return;
    }

    let preset = value as u8;
    scale.scale_preset = Some(preset);
    scale.mask = get_preset_mask(preset);
    scale.divisions = 12;

    if debug_level >= TIER_ESSENTIAL {
        output(format!("SET SCALE PRESET TO {}", preset));
    }
}

pub fn handle_q_bit<F>(
    parts: &[&str],
    variables: &Variables,
    patterns: &mut PatternStorage,
    counters: &mut Counters,
    scripts: &ScriptStorage,
    script_index: usize,
    scale: &mut ScaleState,
    debug_level: u8,
    mut output: F,
) where
    F: FnMut(String),
{
    if parts.len() < 2 {
        output("ERROR: Q.BIT NEEDS BINARY STR ARG".to_string());
        return;
    }

    let binary_string = parts[1];
    match parse_binary_mask(binary_string) {
        Some(mask) => {
            let divisions = mask.len() as u8;
            scale.mask = mask;
            scale.divisions = divisions;
            scale.scale_preset = None;

            if debug_level >= TIER_ESSENTIAL {
                output(format!("SET CUSTOM SCALE MASK ({} DIVISIONS)", divisions));
            }
        }
        None => {
            output("ERROR: Q.BIT NEEDS BINARY STR (0,1)".to_string());
        }
    }
}
