use crate::types::PatternStorage;

/// A single step in a parsed sequence
#[derive(Debug, Clone, PartialEq)]
pub enum SeqStep {
    /// Numeric value (literal or from note name as semitones)
    Value(i16),
    /// Rest (returns 0)
    Rest,
}

/// Parse a note name (C3, Eb4, F#2, etc.) to semitones relative to C3
/// Returns None if not a valid note name
pub fn parse_note_name(s: &str) -> Option<i16> {
    let s = s.to_uppercase();
    let mut chars = s.chars().peekable();

    // Parse note letter (C, D, E, F, G, A, B)
    let note = match chars.next()? {
        'C' => 0,
        'D' => 2,
        'E' => 4,
        'F' => 5,
        'G' => 7,
        'A' => 9,
        'B' => 11,
        _ => return None,
    };

    // Parse accidental (# or b)
    let accidental = match chars.peek() {
        Some('#') => {
            chars.next();
            1
        }
        Some('B') => {
            // Check if this is a flat or part of octave (B4 vs Bb4)
            // If next char after B is a digit, B is the note, not a flat
            let mut lookahead = chars.clone();
            lookahead.next(); // consume the 'B'
            if lookahead.peek().map_or(false, |c| c.is_ascii_digit()) {
                // B followed by digit - B is flat accidental
                chars.next();
                -1
            } else {
                // Just B alone or B followed by nothing - not a flat
                0
            }
        }
        _ => 0,
    };

    // Remaining characters should be the octave number
    let octave_str: String = chars.collect();
    if octave_str.is_empty() {
        return None;
    }
    let octave: i16 = octave_str.parse().ok()?;

    // Calculate semitones relative to C3 (C3 = 0)
    Some(note + accidental + (octave - 3) * 12)
}

/// Parse a sequence pattern string into steps
pub fn parse_seq_pattern(pattern: &str) -> Result<Vec<SeqStep>, String> {
    let mut steps = Vec::new();

    for token in pattern.split_whitespace() {
        let upper = token.to_uppercase();
        match upper.as_str() {
            "_" | "." => steps.push(SeqStep::Rest),
            "X" => steps.push(SeqStep::Value(1)),
            s => {
                // Try parsing as note name first, then as number
                if let Some(semitones) = parse_note_name(s) {
                    steps.push(SeqStep::Value(semitones));
                } else if let Ok(num) = s.parse::<i16>() {
                    steps.push(SeqStep::Value(num));
                } else {
                    return Err(format!("Invalid sequence token: {}", token));
                }
            }
        }
    }

    Ok(steps)
}

/// Extract a quoted string from parts, handling spaces within quotes
/// Returns (extracted_string, parts_consumed) or None if no valid quoted string
pub fn extract_quoted_string(parts: &[&str], start: usize) -> Option<(String, usize)> {
    if start >= parts.len() {
        return None;
    }

    let first = parts[start];

    // Check for opening quote
    let quote_char = if first.starts_with('"') {
        '"'
    } else if first.starts_with('\'') {
        '\''
    } else {
        return None;
    };

    // Single part with both quotes?
    if first.len() > 1 && first.ends_with(quote_char) {
        return Some((first[1..first.len() - 1].to_string(), 1));
    }

    // Multi-part quoted string
    let mut result = first[1..].to_string();
    let mut consumed = 1;

    for i in (start + 1)..parts.len() {
        consumed += 1;
        let part = parts[i];

        if part.ends_with(quote_char) {
            result.push(' ');
            result.push_str(&part[..part.len() - 1]);
            return Some((result, consumed));
        } else {
            result.push(' ');
            result.push_str(part);
        }
    }

    None // Unterminated quote
}

/// Evaluate SEQ expression, returning (value, parts_consumed)
pub fn eval_seq_expression(
    expr: &str,
    parts: &[&str],
    start_idx: usize,
    patterns: &mut PatternStorage,
    script_index: usize,
) -> Option<(i16, usize)> {
    if expr != "SEQ" {
        return None;
    }

    // SEQ requires a quoted pattern string
    if start_idx + 1 >= parts.len() {
        return None;
    }

    // Extract quoted string (may span multiple parts)
    let (pattern, consumed) = extract_quoted_string(parts, start_idx + 1)?;

    // Parse the pattern
    let steps = parse_seq_pattern(&pattern).ok()?;

    if steps.is_empty() {
        return Some((0, 1 + consumed));
    }

    // Get/create state for this sequence using toggle_state HashMap
    let key = format!("seq_{}_{}", script_index, pattern);
    let index = patterns.toggle_state.entry(key).or_insert(0);

    // Get current step value
    let step = &steps[*index % steps.len()];
    let value = match step {
        SeqStep::Value(v) => *v,
        SeqStep::Rest => 0,
    };

    // Advance to next step
    *index = (*index).wrapping_add(1);

    Some((value, 1 + consumed))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_note_name_natural() {
        assert_eq!(parse_note_name("C3"), Some(0));
        assert_eq!(parse_note_name("D3"), Some(2));
        assert_eq!(parse_note_name("E3"), Some(4));
        assert_eq!(parse_note_name("F3"), Some(5));
        assert_eq!(parse_note_name("G3"), Some(7));
        assert_eq!(parse_note_name("A3"), Some(9));
        assert_eq!(parse_note_name("B3"), Some(11));
    }

    #[test]
    fn test_parse_note_name_octaves() {
        assert_eq!(parse_note_name("C4"), Some(12));
        assert_eq!(parse_note_name("C5"), Some(24));
        assert_eq!(parse_note_name("C2"), Some(-12));
        assert_eq!(parse_note_name("A4"), Some(21)); // A440
    }

    #[test]
    fn test_parse_note_name_sharps() {
        assert_eq!(parse_note_name("C#3"), Some(1));
        assert_eq!(parse_note_name("F#3"), Some(6));
        assert_eq!(parse_note_name("G#4"), Some(20));
    }

    #[test]
    fn test_parse_note_name_flats() {
        assert_eq!(parse_note_name("Bb3"), Some(10));
        assert_eq!(parse_note_name("Eb3"), Some(3));
        assert_eq!(parse_note_name("Db4"), Some(13));
    }

    #[test]
    fn test_parse_note_name_invalid() {
        assert_eq!(parse_note_name("H3"), None);
        assert_eq!(parse_note_name("C"), None);
        assert_eq!(parse_note_name("3"), None);
        assert_eq!(parse_note_name("foo"), None);
    }

    #[test]
    fn test_parse_seq_pattern_triggers() {
        let steps = parse_seq_pattern("x _ x _").unwrap();
        assert_eq!(steps.len(), 4);
        assert_eq!(steps[0], SeqStep::Value(1));
        assert_eq!(steps[1], SeqStep::Rest);
        assert_eq!(steps[2], SeqStep::Value(1));
        assert_eq!(steps[3], SeqStep::Rest);
    }

    #[test]
    fn test_parse_seq_pattern_numbers() {
        let steps = parse_seq_pattern("100 200 300").unwrap();
        assert_eq!(steps.len(), 3);
        assert_eq!(steps[0], SeqStep::Value(100));
        assert_eq!(steps[1], SeqStep::Value(200));
        assert_eq!(steps[2], SeqStep::Value(300));
    }

    #[test]
    fn test_parse_seq_pattern_negative() {
        let steps = parse_seq_pattern("-12 0 12").unwrap();
        assert_eq!(steps.len(), 3);
        assert_eq!(steps[0], SeqStep::Value(-12));
        assert_eq!(steps[1], SeqStep::Value(0));
        assert_eq!(steps[2], SeqStep::Value(12));
    }

    #[test]
    fn test_parse_seq_pattern_notes() {
        let steps = parse_seq_pattern("C3 E3 G3 C4").unwrap();
        assert_eq!(steps.len(), 4);
        assert_eq!(steps[0], SeqStep::Value(0));
        assert_eq!(steps[1], SeqStep::Value(4));
        assert_eq!(steps[2], SeqStep::Value(7));
        assert_eq!(steps[3], SeqStep::Value(12));
    }

    #[test]
    fn test_parse_seq_pattern_mixed() {
        let steps = parse_seq_pattern("C3 _ E3 100").unwrap();
        assert_eq!(steps.len(), 4);
        assert_eq!(steps[0], SeqStep::Value(0));
        assert_eq!(steps[1], SeqStep::Rest);
        assert_eq!(steps[2], SeqStep::Value(4));
        assert_eq!(steps[3], SeqStep::Value(100));
    }

    #[test]
    fn test_parse_seq_pattern_dot_rest() {
        let steps = parse_seq_pattern("x . x .").unwrap();
        assert_eq!(steps.len(), 4);
        assert_eq!(steps[0], SeqStep::Value(1));
        assert_eq!(steps[1], SeqStep::Rest);
    }

    #[test]
    fn test_extract_quoted_string_single_part() {
        let parts = vec!["SEQ", "\"C3 E3\""];
        let (s, consumed) = extract_quoted_string(&parts, 1).unwrap();
        assert_eq!(s, "C3 E3");
        assert_eq!(consumed, 1);
    }

    #[test]
    fn test_extract_quoted_string_multi_part() {
        let parts = vec!["SEQ", "\"C3", "E3", "G3\""];
        let (s, consumed) = extract_quoted_string(&parts, 1).unwrap();
        assert_eq!(s, "C3 E3 G3");
        assert_eq!(consumed, 3);
    }

    #[test]
    fn test_extract_quoted_string_single_quote() {
        let parts = vec!["SEQ", "'x _ x'"];
        let (s, consumed) = extract_quoted_string(&parts, 1).unwrap();
        assert_eq!(s, "x _ x");
        assert_eq!(consumed, 1);
    }

    #[test]
    fn test_extract_quoted_string_no_quote() {
        let parts = vec!["SEQ", "C3"];
        assert!(extract_quoted_string(&parts, 1).is_none());
    }
}
