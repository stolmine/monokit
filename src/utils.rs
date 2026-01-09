pub fn split_respecting_quotes(cmd: &str) -> Vec<String> {
    let mut parts = Vec::new();
    let mut current = String::new();
    let mut in_quote = false;
    let mut quote_char = ' ';

    for c in cmd.chars() {
        match c {
            '"' | '\'' if !in_quote => {
                in_quote = true;
                quote_char = c;
                current.push(c);
            }
            c if c == quote_char && in_quote => {
                in_quote = false;
                current.push(c);
            }
            ';' if !in_quote => {
                parts.push(current.trim().to_string());
                current.clear();
            }
            _ => current.push(c),
        }
    }
    if !current.is_empty() {
        parts.push(current.trim().to_string());
    }
    parts.into_iter().filter(|s| !s.is_empty()).collect()
}

pub fn split_whitespace_respecting_quotes(cmd: &str) -> Vec<String> {
    let mut parts = Vec::new();
    let mut current = String::new();
    let mut in_quote = false;
    let mut quote_char = ' ';

    for c in cmd.chars() {
        match c {
            '"' | '\'' if !in_quote => {
                in_quote = true;
                quote_char = c;
                current.push(c);
            }
            c if c == quote_char && in_quote => {
                in_quote = false;
                current.push(c);
            }
            c if c.is_whitespace() && !in_quote => {
                if !current.is_empty() {
                    parts.push(current.clone());
                    current.clear();
                }
            }
            _ => current.push(c),
        }
    }
    if !current.is_empty() {
        parts.push(current);
    }
    parts
}

pub const VALUE_14BIT_MAX: i32 = 16383;
pub const VALUE_14BIT_CENTER: i32 = 8192;
pub const VALUE_14BIT_MIN: i32 = 0;

pub const VALUE_14BIT_MAX_I16: i16 = 16383;
pub const VALUE_14BIT_CENTER_I16: i16 = 8192;

pub fn to_normalized(value: i32) -> f32 {
    value as f32 / VALUE_14BIT_MAX as f32
}

pub fn to_percentage(value: i32) -> i32 {
    (value as f32 / VALUE_14BIT_MAX as f32 * 100.0).round() as i32
}

pub fn to_normalized_i16(value: i16) -> f32 {
    value as f32 / VALUE_14BIT_MAX as f32
}

pub fn to_percentage_i16(value: i16) -> i32 {
    (value as f32 / VALUE_14BIT_MAX as f32 * 100.0).round() as i32
}

pub const METER_CHARS: [char; 9] = [' ', '▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];
pub const METER_CHARS_ASCII: [char; 9] = [' ', '.', 'o', 'O', '0', '@', '#', '#', '#'];

pub fn level_to_meter_char(level: f32, ascii_mode: bool) -> char {
    let idx = (level.clamp(0.0, 1.0) * 8.0).round() as usize;
    if ascii_mode {
        METER_CHARS_ASCII[idx.min(8)]
    } else {
        METER_CHARS[idx.min(8)]
    }
}
