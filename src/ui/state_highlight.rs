use std::collections::HashMap;
use std::time::Instant;
use ratatui::prelude::*;
use ratatui::style::{Color, Style};

use crate::eval::seq::{extract_quoted_string, parse_seq_pattern};
use crate::types::LineSegmentActivity;
use crate::theme::Theme;
use crate::utils::split_respecting_quotes;

#[derive(Debug, Clone)]
pub struct HighlightedSegment {
    pub text: String,
    pub is_highlighted: bool,
}

#[derive(Debug, Clone)]
pub struct HighlightedLine {
    pub segments: Vec<HighlightedSegment>,
}

impl HighlightedLine {
    pub fn to_spans(&self, normal_color: Color, highlight_color: Color) -> Vec<Span<'static>> {
        self.segments
            .iter()
            .map(|seg| {
                let color = if seg.is_highlighted {
                    highlight_color
                } else {
                    normal_color
                };
                Span::styled(seg.text.clone(), Style::default().fg(color))
            })
            .collect()
    }
}

pub fn highlight_stateful_operators(
    line: &str,
    script_index: usize,
    toggle_state: &HashMap<String, usize>,
) -> HighlightedLine {
    let mut segments = Vec::new();
    let mut line_pos = 0;

    let commands = split_respecting_quotes(line);

    for (cmd_idx, cmd) in commands.iter().enumerate() {
        let cmd_start = if let Some(pos) = line[line_pos..].find(cmd.as_str()) {
            line_pos + pos
        } else {
            line_pos
        };

        let cmd_segments = highlight_single_command(cmd, cmd_start, script_index, toggle_state);
        segments.extend(cmd_segments.segments);

        line_pos = cmd_start + cmd.len();

        if cmd_idx < commands.len() - 1 {
            if let Some(semi_pos) = line[line_pos..].find(';') {
                let semi_abs = line_pos + semi_pos;
                if semi_abs > line_pos {
                    segments.push(HighlightedSegment {
                        text: line[line_pos..semi_abs].to_string(),
                        is_highlighted: false,
                    });
                }

                let after_semi = semi_abs + 1;
                let trailing_space_end = line[after_semi..]
                    .find(|c: char| !c.is_whitespace())
                    .map(|pos| after_semi + pos)
                    .unwrap_or(line.len());

                segments.push(HighlightedSegment {
                    text: line[semi_abs..trailing_space_end].to_string(),
                    is_highlighted: false,
                });
                line_pos = trailing_space_end;
            }
        }
    }

    if line_pos < line.len() {
        segments.push(HighlightedSegment {
            text: line[line_pos..].to_string(),
            is_highlighted: false,
        });
    }

    if segments.is_empty() {
        segments.push(HighlightedSegment {
            text: line.to_string(),
            is_highlighted: false,
        });
    }

    HighlightedLine { segments }
}

fn highlight_single_command(
    cmd: &str,
    cmd_start_in_line: usize,
    script_index: usize,
    toggle_state: &HashMap<String, usize>,
) -> HighlightedLine {
    let mut segments = Vec::new();
    let mut current_pos = 0;

    let parts: Vec<&str> = cmd.split_whitespace().collect();

    let mut i = 0;
    while i < parts.len() {
        let part = parts[i];

        // Check for SEQ with or without space before quote
        let (is_seq, seq_pattern_direct) = if part == "SEQ" {
            (true, None)
        } else if part.starts_with("SEQ\"") || part.starts_with("SEQ'") {
            // Handle SEQ"..." without space by extracting from the command
            // Find SEQ in the command starting from current_pos
            let seq_pos = cmd[current_pos..].find("SEQ").map(|p| current_pos + p);
            if let Some(seq_start) = seq_pos {
                let after_seq = &cmd[seq_start + 3..];
                let quote_char = if after_seq.starts_with('"') { '"' } else if after_seq.starts_with('\'') { '\'' } else { ' ' };
                if quote_char != ' ' {
                    // Find closing quote
                    if let Some(close_pos) = after_seq[1..].find(quote_char) {
                        let pattern = after_seq[1..close_pos + 1].to_string();
                        // Calculate how many parts to consume
                        // SEQ is at seq_start, pattern starts at seq_start + 3 + 1, ends at seq_start + 3 + 1 + close_pos + 1
                        let expr_end = seq_start + 3 + 1 + close_pos + 1 + 1;
                        let expr_end = expr_end.min(cmd.len());
                        let full_expr = &cmd[seq_start..expr_end];
                        let parts_in_expr: Vec<&str> = full_expr.split_whitespace().collect();
                        let consumed = if parts_in_expr.len() > 1 { parts_in_expr.len() - 1 } else { 0 };
                        (true, Some((pattern, consumed)))
                    } else {
                        (false, None)
                    }
                } else {
                    (false, None)
                }
            } else {
                (false, None)
            }
        } else {
            (false, None)
        };

        if is_seq {
            let pattern_result = seq_pattern_direct.or_else(|| extract_quoted_string(&parts, i + 1));
            if let Some((pattern, consumed)) = pattern_result {
                let seq_start = cmd[current_pos..].find("SEQ").map(|p| current_pos + p);
                if let Some(start_pos) = seq_start {
                    if start_pos > current_pos {
                        segments.push(HighlightedSegment {
                            text: cmd[current_pos..start_pos].to_string(),
                            is_highlighted: false,
                        });
                    }

                    let key = format!("seq_{}_{}", script_index, pattern);
                    let current_index = toggle_state.get(&key).copied().unwrap_or(0);

                    if let Some(highlighted_segments) = highlight_seq_pattern(
                        &cmd[start_pos..],
                        &pattern,
                        current_index,
                        script_index,
                        toggle_state,
                    ) {
                        segments.extend(highlighted_segments);
                        let seq_end = cmd[start_pos..]
                            .find(&format!("\"{}\"", pattern))
                            .or_else(|| cmd[start_pos..].find(&format!("'{}'", pattern)))
                            .map(|p| {
                                start_pos + p + pattern.len() + 2
                            })
                            .unwrap_or(start_pos + 3);
                        current_pos = seq_end;
                    } else {
                        let full_expr_end = cmd[start_pos..]
                            .find(&format!("\"{}\"", pattern))
                            .or_else(|| cmd[start_pos..].find(&format!("'{}'", pattern)))
                            .map(|p| start_pos + p + pattern.len() + 2)
                            .unwrap_or(cmd.len());

                        segments.push(HighlightedSegment {
                            text: cmd[start_pos..full_expr_end].to_string(),
                            is_highlighted: false,
                        });
                        current_pos = full_expr_end;
                    }

                    i += 1 + consumed;
                    continue;
                }
            }
        } else if part == "TOG" {
            if i + 2 < parts.len() {
                let next_idx = i + 3;
                let key = format!("{}_{}", script_index, parts[i..next_idx].join("_"));

                let tog_start = cmd[current_pos..].find("TOG").map(|p| current_pos + p);
                if let Some(start_pos) = tog_start {
                    if start_pos > current_pos {
                        segments.push(HighlightedSegment {
                            text: cmd[current_pos..start_pos].to_string(),
                            is_highlighted: false,
                        });
                    }

                    let current_state = toggle_state.get(&key).copied().unwrap_or(0);
                    let active_value_index = current_state % 2;

                    if let Some(highlighted_segments) = highlight_tog_expression(
                        &cmd[start_pos..],
                        &parts[i+1],
                        &parts[i+2],
                        active_value_index,
                    ) {
                        segments.extend(highlighted_segments);

                        let val2_end = cmd[start_pos..]
                            .find(parts[i+2])
                            .map(|p| start_pos + p + parts[i+2].len())
                            .unwrap_or(cmd.len());
                        current_pos = val2_end;
                    }

                    i += 3;
                    continue;
                }
            }
        } else if part == "EITH" {
            if i + 2 < parts.len() {
                let next_idx = i + 3;
                let key = format!("{}_{}", script_index, parts[i..next_idx].join("_"));

                let eith_start = cmd[current_pos..].find("EITH").map(|p| current_pos + p);
                if let Some(start_pos) = eith_start {
                    if start_pos > current_pos {
                        segments.push(HighlightedSegment {
                            text: cmd[current_pos..start_pos].to_string(),
                            is_highlighted: false,
                        });
                    }

                    let selected_index = toggle_state.get(&key).copied().unwrap_or(0);

                    if let Some(highlighted_segments) = highlight_eith_expression(
                        &cmd[start_pos..],
                        &parts[i+1],
                        &parts[i+2],
                        selected_index,
                    ) {
                        segments.extend(highlighted_segments);

                        let val2_end = cmd[start_pos..]
                            .find(parts[i+2])
                            .map(|p| start_pos + p + parts[i+2].len())
                            .unwrap_or(cmd.len());
                        current_pos = val2_end;
                    }

                    i += 3;
                    continue;
                }
            }
        }

        i += 1;
    }

    if current_pos < cmd.len() {
        segments.push(HighlightedSegment {
            text: cmd[current_pos..].to_string(),
            is_highlighted: false,
        });
    }

    if segments.is_empty() {
        segments.push(HighlightedSegment {
            text: cmd.to_string(),
            is_highlighted: false,
        });
    }

    HighlightedLine { segments }
}

fn highlight_seq_pattern(
    line_from_seq: &str,
    pattern: &str,
    current_index: usize,
    script_index: usize,
    toggle_state: &HashMap<String, usize>,
) -> Option<Vec<HighlightedSegment>> {
    let steps = parse_seq_pattern(pattern).ok()?;

    if steps.is_empty() {
        return None;
    }

    let step_index = current_index % steps.len();

    let quote_start = line_from_seq.find('"').or_else(|| line_from_seq.find('\''))?;
    let quote_char = if line_from_seq.chars().nth(quote_start) == Some('"') {
        '"'
    } else {
        '\''
    };

    let before_quote = &line_from_seq[..=quote_start];

    let tokens = tokenize_for_display(pattern);

    let mut segments = Vec::new();
    segments.push(HighlightedSegment {
        text: before_quote.to_string(),
        is_highlighted: false,
    });

    let mut token_step_index = 0;
    for (token_idx, token) in tokens.iter().enumerate() {
        if token_idx > 0 {
            segments.push(HighlightedSegment {
                text: " ".to_string(),
                is_highlighted: false,
            });
        }

        let repeat_count = get_repeat_count(token);
        let is_current = token_step_index == step_index
            || (step_index >= token_step_index && step_index < token_step_index + repeat_count);

        // Check if this is an alternation <a b> or random choice {a b}
        if is_current && (token.starts_with('<') || token.starts_with('{')) {
            // For alternation, look up the nested state to highlight the active option
            let inner_segments = highlight_nested_token(
                token,
                script_index,
                pattern,
                token_step_index,
                toggle_state,
            );
            segments.extend(inner_segments);
        } else {
            segments.push(HighlightedSegment {
                text: token.to_string(),
                is_highlighted: is_current,
            });
        }

        token_step_index += repeat_count;
    }

    segments.push(HighlightedSegment {
        text: quote_char.to_string(),
        is_highlighted: false,
    });

    Some(segments)
}

/// Highlight a nested token like <a b> or {a b} showing which option is active
fn highlight_nested_token(
    token: &str,
    script_index: usize,
    pattern: &str,
    step_index: usize,
    toggle_state: &HashMap<String, usize>,
) -> Vec<HighlightedSegment> {
    let mut segments = Vec::new();

    // Check bracket type
    let (open_bracket, close_bracket, is_alternation) = if token.starts_with('<') {
        ('<', '>', true)
    } else if token.starts_with('{') {
        ('{', '}', false)
    } else {
        // Not a nested token, just highlight the whole thing
        return vec![HighlightedSegment {
            text: token.to_string(),
            is_highlighted: true,
        }];
    };

    // Extract content inside brackets (handle repeat suffix like <a b>*2)
    let bracket_end = token.rfind(close_bracket).unwrap_or(token.len() - 1);
    let inner_content = &token[1..bracket_end];
    let suffix = &token[bracket_end + 1..]; // e.g., "*2"

    // Parse inner options
    let options: Vec<&str> = inner_content.split_whitespace().collect();

    if options.is_empty() {
        return vec![HighlightedSegment {
            text: token.to_string(),
            is_highlighted: true,
        }];
    }

    // For alternation <a b>, look up the state
    // Key format: seq_alt_{script}_{pattern}_{step_index}
    // For random choice {a b}, look up the last selected index
    // Key format: seq_rnd_{script}_{pattern}_{step_index}
    let active_option_idx = if is_alternation {
        let alt_key = format!("seq_alt_{}_{}_{}", script_index, pattern, step_index);
        // The state is incremented AFTER returning, so current state shows what was last returned
        // We need state - 1 to show what will be returned next, but if state is 0, show 0
        let state = toggle_state.get(&alt_key).copied().unwrap_or(0);
        Some(state % options.len())
    } else {
        // Random choice {a b} - look up the last selected index
        let rnd_key = format!("seq_rnd_{}_{}_{}", script_index, pattern, step_index);
        toggle_state.get(&rnd_key).copied()
    };

    // Build segments: <, then options with one highlighted, then >, then suffix
    segments.push(HighlightedSegment {
        text: open_bracket.to_string(),
        is_highlighted: false, // Brackets are not highlighted, only active values
    });

    for (idx, option) in options.iter().enumerate() {
        if idx > 0 {
            segments.push(HighlightedSegment {
                text: " ".to_string(),
                is_highlighted: false,
            });
        }

        let is_active = active_option_idx.map_or(false, |active_idx| idx == active_idx);

        segments.push(HighlightedSegment {
            text: option.to_string(),
            is_highlighted: is_active,
        });
    }

    segments.push(HighlightedSegment {
        text: close_bracket.to_string(),
        is_highlighted: false, // Brackets are not highlighted, only active values
    });

    if !suffix.is_empty() {
        segments.push(HighlightedSegment {
            text: suffix.to_string(),
            is_highlighted: false, // Suffix is structural, not highlighted
        });
    }

    segments
}

fn highlight_tog_expression(
    line_from_tog: &str,
    val1: &str,
    val2: &str,
    active_index: usize,
) -> Option<Vec<HighlightedSegment>> {
    let mut segments = Vec::new();

    let val1_pos = line_from_tog.find(val1)?;
    let val1_end = val1_pos + val1.len();

    // Search for val2 AFTER val1 to avoid finding val2 inside val1 (e.g., "0" inside "10000")
    let val2_pos = line_from_tog[val1_end..].find(val2).map(|p| p + val1_end)?;

    let tog_to_val1 = &line_from_tog[..val1_pos];
    segments.push(HighlightedSegment {
        text: tog_to_val1.to_string(),
        is_highlighted: false,
    });

    segments.push(HighlightedSegment {
        text: val1.to_string(),
        is_highlighted: active_index == 0,
    });

    let between = &line_from_tog[val1_end..val2_pos];
    segments.push(HighlightedSegment {
        text: between.to_string(),
        is_highlighted: false,
    });

    segments.push(HighlightedSegment {
        text: val2.to_string(),
        is_highlighted: active_index == 1,
    });

    Some(segments)
}

fn highlight_eith_expression(
    line_from_eith: &str,
    val1: &str,
    val2: &str,
    selected_index: usize,
) -> Option<Vec<HighlightedSegment>> {
    let mut segments = Vec::new();

    let val1_pos = line_from_eith.find(val1)?;
    let val1_end = val1_pos + val1.len();

    let val2_pos = line_from_eith[val1_end..].find(val2).map(|p| p + val1_end)?;

    let eith_to_val1 = &line_from_eith[..val1_pos];
    segments.push(HighlightedSegment {
        text: eith_to_val1.to_string(),
        is_highlighted: false,
    });

    segments.push(HighlightedSegment {
        text: val1.to_string(),
        is_highlighted: selected_index == 0,
    });

    let between = &line_from_eith[val1_end..val2_pos];
    segments.push(HighlightedSegment {
        text: between.to_string(),
        is_highlighted: false,
    });

    segments.push(HighlightedSegment {
        text: val2.to_string(),
        is_highlighted: selected_index == 1,
    });

    Some(segments)
}

fn tokenize_for_display(pattern: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current_token = String::new();
    let mut in_brackets = false;

    for ch in pattern.chars() {
        match ch {
            '<' | '{' => {
                in_brackets = true;
                current_token.push(ch);
            }
            '>' | '}' => {
                in_brackets = false;
                current_token.push(ch);
            }
            c if c.is_whitespace() => {
                if in_brackets {
                    current_token.push(c);
                } else if !current_token.is_empty() {
                    tokens.push(current_token.clone());
                    current_token.clear();
                }
            }
            _ => {
                current_token.push(ch);
            }
        }
    }

    if !current_token.is_empty() {
        tokens.push(current_token);
    }

    tokens
}

fn get_repeat_count(token: &str) -> usize {
    if let Some(star_pos) = token.rfind('*') {
        let count_str = &token[star_pos + 1..];
        count_str.parse::<usize>().unwrap_or(1)
    } else {
        1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_highlight_seq_basic() {
        let mut state = HashMap::new();
        state.insert("seq_0_C3 E3 G3".to_string(), 1);

        let line = "N ON SEQ \"C3 E3 G3\"";
        let result = highlight_stateful_operators(line, 0, &state);

        let highlighted_tokens: Vec<_> = result.segments
            .iter()
            .filter(|s| s.is_highlighted)
            .map(|s| s.text.as_str())
            .collect();

        assert_eq!(highlighted_tokens, vec!["E3"]);
    }

    #[test]
    fn test_highlight_seq_first_position() {
        let state = HashMap::new();

        let line = "N ON SEQ \"C3 E3 G3\"";
        let result = highlight_stateful_operators(line, 0, &state);

        let highlighted_tokens: Vec<_> = result.segments
            .iter()
            .filter(|s| s.is_highlighted)
            .map(|s| s.text.as_str())
            .collect();

        assert_eq!(highlighted_tokens, vec!["C3"]);
    }

    #[test]
    fn test_highlight_tog_first_value() {
        let state = HashMap::new();

        let line = "N ON TOG 10 20";
        let result = highlight_stateful_operators(line, 0, &state);

        let highlighted_tokens: Vec<_> = result.segments
            .iter()
            .filter(|s| s.is_highlighted)
            .map(|s| s.text.as_str())
            .collect();

        assert_eq!(highlighted_tokens, vec!["10"]);
    }

    #[test]
    fn test_highlight_tog_second_value() {
        let mut state = HashMap::new();
        state.insert("0_TOG_10_20".to_string(), 1);

        let line = "N ON TOG 10 20";
        let result = highlight_stateful_operators(line, 0, &state);

        let highlighted_tokens: Vec<_> = result.segments
            .iter()
            .filter(|s| s.is_highlighted)
            .map(|s| s.text.as_str())
            .collect();

        assert_eq!(highlighted_tokens, vec!["20"]);
    }

    #[test]
    fn test_highlight_seq_with_repeat() {
        let mut state = HashMap::new();
        state.insert("seq_0_C3*2 E3".to_string(), 1);

        let line = "N ON SEQ \"C3*2 E3\"";
        let result = highlight_stateful_operators(line, 0, &state);

        let highlighted_tokens: Vec<_> = result.segments
            .iter()
            .filter(|s| s.is_highlighted)
            .map(|s| s.text.as_str())
            .collect();

        assert_eq!(highlighted_tokens, vec!["C3*2"]);
    }

    #[test]
    fn test_no_operators() {
        let state = HashMap::new();
        let line = "N ON 60";
        let result = highlight_stateful_operators(line, 0, &state);

        assert_eq!(result.segments.len(), 1);
        assert_eq!(result.segments[0].text, "N ON 60");
        assert!(!result.segments[0].is_highlighted);
    }

    #[test]
    fn test_highlight_seq_alternation_first_option() {
        let mut state = HashMap::new();
        // Main SEQ at step 2 (the alternation step)
        state.insert("seq_0_5 3 <1 0> 4".to_string(), 2);
        // Alternation state at 0 means first option (1) will be returned
        state.insert("seq_alt_0_5 3 <1 0> 4_2".to_string(), 0);

        let line = "PF SEQ \"5 3 <1 0> 4\"";
        let result = highlight_stateful_operators(line, 0, &state);

        // Find the highlighted segments
        let highlighted: Vec<_> = result.segments
            .iter()
            .filter(|s| s.is_highlighted)
            .map(|s| s.text.as_str())
            .collect();

        // Should highlight only the active option (1), not the brackets
        assert!(!highlighted.contains(&"<"), "Should NOT highlight open bracket");
        assert!(highlighted.contains(&"1"), "Should highlight first option");
        assert!(!highlighted.contains(&">"), "Should NOT highlight close bracket");
        assert!(!highlighted.contains(&"0"), "Should NOT highlight second option");
    }

    #[test]
    fn test_highlight_seq_alternation_second_option() {
        let mut state = HashMap::new();
        // Main SEQ at step 2 (the alternation step)
        state.insert("seq_0_5 3 <1 0> 4".to_string(), 2);
        // Alternation state at 1 means second option (0) will be returned
        state.insert("seq_alt_0_5 3 <1 0> 4_2".to_string(), 1);

        let line = "PF SEQ \"5 3 <1 0> 4\"";
        let result = highlight_stateful_operators(line, 0, &state);

        // Find the highlighted segments
        let highlighted: Vec<_> = result.segments
            .iter()
            .filter(|s| s.is_highlighted)
            .map(|s| s.text.as_str())
            .collect();

        // Should highlight only the active option (0), not the brackets
        assert!(!highlighted.contains(&"<"), "Should NOT highlight open bracket");
        assert!(highlighted.contains(&"0"), "Should highlight second option");
        assert!(!highlighted.contains(&">"), "Should NOT highlight close bracket");
        assert!(!highlighted.contains(&"1"), "Should NOT highlight first option");
    }

    #[test]
    fn test_highlight_seq_random_choice_first_option() {
        let mut state = HashMap::new();
        // Main SEQ at step 2 (the random choice step)
        state.insert("seq_0_5 3 {1 0} 4".to_string(), 2);
        // Random choice state at 0 means first option (1) was selected
        state.insert("seq_rnd_0_5 3 {1 0} 4_2".to_string(), 0);

        let line = "PF SEQ \"5 3 {1 0} 4\"";
        let result = highlight_stateful_operators(line, 0, &state);

        // Find the highlighted segments
        let highlighted: Vec<_> = result.segments
            .iter()
            .filter(|s| s.is_highlighted)
            .map(|s| s.text.as_str())
            .collect();

        // Should highlight the first option (1) that was randomly selected
        assert!(!highlighted.contains(&"{"), "Should NOT highlight open bracket");
        assert!(highlighted.contains(&"1"), "Should highlight first option");
        assert!(!highlighted.contains(&"}"), "Should NOT highlight close bracket");
        assert!(!highlighted.contains(&"0"), "Should NOT highlight second option");
    }

    #[test]
    fn test_highlight_seq_random_choice_second_option() {
        let mut state = HashMap::new();
        // Main SEQ at step 2 (the random choice step)
        state.insert("seq_0_5 3 {1 0} 4".to_string(), 2);
        // Random choice state at 1 means second option (0) was selected
        state.insert("seq_rnd_0_5 3 {1 0} 4_2".to_string(), 1);

        let line = "PF SEQ \"5 3 {1 0} 4\"";
        let result = highlight_stateful_operators(line, 0, &state);

        // Find the highlighted segments
        let highlighted: Vec<_> = result.segments
            .iter()
            .filter(|s| s.is_highlighted)
            .map(|s| s.text.as_str())
            .collect();

        // Should highlight the second option (0) that was randomly selected
        assert!(!highlighted.contains(&"{"), "Should NOT highlight open bracket");
        assert!(highlighted.contains(&"0"), "Should highlight second option");
        assert!(!highlighted.contains(&"}"), "Should NOT highlight close bracket");
        assert!(!highlighted.contains(&"1"), "Should NOT highlight first option");
    }

    #[test]
    fn test_highlight_seq_no_space_before_quote() {
        let mut state = HashMap::new();
        state.insert("seq_0_0 10 2".to_string(), 1);

        let line = "N ON SEQ\"0 10 2\"";
        let result = highlight_stateful_operators(line, 0, &state);

        let highlighted_tokens: Vec<_> = result.segments
            .iter()
            .filter(|s| s.is_highlighted)
            .map(|s| s.text.as_str())
            .collect();

        assert_eq!(highlighted_tokens, vec!["10"]);
    }

    #[test]
    fn test_highlight_seq_unterminated_quote() {
        let state = HashMap::new();
        let line = "N ON SEQ \"C3 E3";
        let result = highlight_stateful_operators(line, 0, &state);

        // Should not panic, should treat the whole line as non-highlighted
        assert!(!result.segments.is_empty());
        let has_any_highlighted = result.segments.iter().any(|s| s.is_highlighted);
        assert!(!has_any_highlighted, "Unterminated quote should not highlight anything");
    }

    #[test]
    fn test_highlight_seq_with_semicolon_separated_commands() {
        let mut state = HashMap::new();
        state.insert("seq_0_5000*15 1250".to_string(), 0);

        let line = "PF SEQ \"5000*15 1250\"; AD 5; PA 0";
        let result = highlight_stateful_operators(line, 0, &state);

        let highlighted_tokens: Vec<_> = result.segments
            .iter()
            .filter(|s| s.is_highlighted)
            .map(|s| s.text.as_str())
            .collect();

        assert_eq!(highlighted_tokens, vec!["5000*15"]);

        let full_text: String = result.segments.iter().map(|s| s.text.as_str()).collect();
        assert!(full_text.contains(";"), "Should preserve semicolons in output");
        assert!(full_text.contains("AD 5"), "Should preserve commands after semicolon");
        assert!(full_text.contains("PA 0"), "Should preserve all commands");
    }

    #[test]
    fn test_highlight_eith_first_value() {
        let mut state = HashMap::new();
        state.insert("0_EITH_10_20".to_string(), 0);

        let line = "N ON EITH 10 20";
        let result = highlight_stateful_operators(line, 0, &state);

        let highlighted_tokens: Vec<_> = result.segments
            .iter()
            .filter(|s| s.is_highlighted)
            .map(|s| s.text.as_str())
            .collect();

        assert_eq!(highlighted_tokens, vec!["10"]);
    }

    #[test]
    fn test_highlight_eith_second_value() {
        let mut state = HashMap::new();
        state.insert("0_EITH_100_200".to_string(), 1);

        let line = "PF EITH 100 200";
        let result = highlight_stateful_operators(line, 0, &state);

        let highlighted_tokens: Vec<_> = result.segments
            .iter()
            .filter(|s| s.is_highlighted)
            .map(|s| s.text.as_str())
            .collect();

        assert_eq!(highlighted_tokens, vec!["200"]);
    }
}

pub fn apply_conditional_activity(
    highlighted: HighlightedLine,
    conditional_activity: &LineSegmentActivity,
    theme: &Theme,
    hold_ms: f32,
    is_selected: bool,
) -> Vec<Span<'static>> {
    let (normal_color, highlight_color) = if is_selected {
        (theme.highlight_fg, theme.success)
    } else {
        (theme.secondary, theme.foreground)
    };

    if conditional_activity.segments.is_empty() {
        return highlighted.segments.iter().map(|seg| {
            let color = if seg.is_highlighted {
                highlight_color
            } else {
                normal_color
            };
            Span::styled(seg.text.clone(), Style::default().fg(color))
        }).collect();
    }

    // Collect active conditional segments with their decay progress
    let active_conds: Vec<_> = conditional_activity.segments.iter()
        .filter_map(|cond_seg| {
            let elapsed_ms = cond_seg.timestamp.elapsed().as_millis() as f32;
            let is_visible = elapsed_ms < hold_ms + crate::theme::ACTIVITY_DECAY_MS;
            if is_visible {
                let progress = if elapsed_ms < hold_ms {
                    0.0
                } else {
                    let decay_elapsed = (elapsed_ms - hold_ms) / crate::theme::ACTIVITY_DECAY_MS;
                    1.0 - (1.0 - decay_elapsed.min(1.0)).powi(3)
                };
                Some((cond_seg.start, cond_seg.end, progress))
            } else {
                None
            }
        })
        .collect();

    let mut result_spans = Vec::new();
    let mut char_pos = 0;

    for segment in &highlighted.segments {
        let segment_start = char_pos;
        let segment_end = char_pos + segment.text.len();
        let segment_text = &segment.text;

        let base_color = if segment.is_highlighted {
            highlight_color
        } else {
            normal_color
        };

        // Find all conditional segments that overlap with this text segment
        let overlapping: Vec<_> = active_conds.iter()
            .filter(|(start, end, _)| *start < segment_end && *end > segment_start)
            .collect();

        if overlapping.is_empty() {
            // No conditional activity - use base color
            result_spans.push(Span::styled(segment_text.clone(), Style::default().fg(base_color)));
        } else {
            // Split segment at conditional boundaries and apply colors
            let mut pos = 0;
            let mut boundaries: Vec<usize> = Vec::new();

            // Collect all boundary points within this segment
            for (cond_start, cond_end, _) in &overlapping {
                if *cond_start > segment_start && *cond_start < segment_end {
                    boundaries.push(*cond_start - segment_start);
                }
                if *cond_end > segment_start && *cond_end < segment_end {
                    boundaries.push(*cond_end - segment_start);
                }
            }
            boundaries.sort();
            boundaries.dedup();
            boundaries.push(segment_text.len()); // End boundary

            for boundary in boundaries {
                if boundary <= pos {
                    continue;
                }

                let slice = &segment_text[pos..boundary];
                let slice_abs_start = segment_start + pos;
                let slice_abs_end = segment_start + boundary;

                // Check if this slice is covered by any active conditional
                let cond_progress = overlapping.iter()
                    .filter(|(start, end, _)| *start <= slice_abs_start && *end >= slice_abs_end)
                    .map(|(_, _, progress)| *progress)
                    .min_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

                let color = if let Some(progress) = cond_progress {
                    let active_color = if is_selected {
                        theme.success
                    } else {
                        theme.foreground
                    };
                    Theme::lerp_color(active_color, base_color, progress)
                } else {
                    base_color
                };

                result_spans.push(Span::styled(slice.to_string(), Style::default().fg(color)));
                pos = boundary;
            }
        }

        char_pos = segment_end;
    }

    result_spans
}
