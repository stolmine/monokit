use std::collections::HashMap;

use crate::eval::seq::{extract_quoted_string, parse_seq_pattern};
use crate::utils::split_respecting_quotes;

use super::types::{HighlightedSegment, HighlightedLine};

pub fn highlight_stateful_operators(
    line: &str,
    script_index: usize,
    toggle_state: &HashMap<String, usize>,
    toggle_last_value: &HashMap<String, i16>,
    direct_validation: &HashMap<String, bool>,
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

        let cmd_segments = highlight_single_command(cmd, cmd_start, script_index, toggle_state, toggle_last_value, direct_validation);
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

fn is_variable_assignment(parts: &[&str]) -> bool {
    if parts.is_empty() {
        return false;
    }
    let first = parts[0].to_uppercase();
    if first.len() != 1 {
        return false;
    }
    let ch = first.chars().next().unwrap();
    matches!(ch, 'A' | 'B' | 'C' | 'D' | 'I' | 'J' | 'K' | 'X' | 'Y' | 'Z' | 'T')
}

fn highlight_single_command(
    cmd: &str,
    cmd_start_in_line: usize,
    script_index: usize,
    toggle_state: &HashMap<String, usize>,
    toggle_last_value: &HashMap<String, i16>,
    direct_validation: &HashMap<String, bool>,
) -> HighlightedLine {
    let mut segments = Vec::new();
    let mut current_pos = 0;

    let parts: Vec<&str> = cmd.split_whitespace().collect();
    let is_var_assignment = is_variable_assignment(&parts);

    let mut i = 0;
    while i < parts.len() {
        let part = parts[i];

        let (is_seq, seq_pattern_direct) = if part == "SEQ" {
            (true, None)
        } else if part.starts_with("SEQ\"") || part.starts_with("SEQ'") {
            let seq_pos = cmd[current_pos..].find("SEQ").map(|p| current_pos + p);
            if let Some(seq_start) = seq_pos {
                let after_seq = &cmd[seq_start + 3..];
                let quote_char = if after_seq.starts_with('"') { '"' } else if after_seq.starts_with('\'') { '\'' } else { ' ' };
                if quote_char != ' ' {
                    if let Some(close_pos) = after_seq[1..].find(quote_char) {
                        let pattern = after_seq[1..close_pos + 1].to_string();
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

                    if !is_var_assignment {
                        if direct_validation.get(&key) == Some(&false) {
                            if start_pos > current_pos {
                                segments.push(HighlightedSegment {
                                    text: cmd[current_pos..start_pos].to_string(),
                                    is_highlighted: false,
                                });
                            }
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
                            i += 1 + consumed;
                            continue;
                        }
                    }

                    let current_index = toggle_state.get(&key).copied().unwrap_or(0);

                    if let Some(highlighted_segments) = highlight_seq_pattern(
                        &cmd[start_pos..],
                        &pattern,
                        current_index,
                        script_index,
                        toggle_state,
                        toggle_last_value,
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

                if !is_var_assignment {
                    if direct_validation.get(&key) == Some(&false) {
                        let tog_start = cmd[current_pos..].find("TOG").map(|p| current_pos + p);
                        if let Some(start_pos) = tog_start {
                            if start_pos > current_pos {
                                segments.push(HighlightedSegment {
                                    text: cmd[current_pos..start_pos].to_string(),
                                    is_highlighted: false,
                                });
                            }
                            let val2_end = find_whole_word(&cmd[start_pos..], parts[i+2])
                                .map(|p| start_pos + p + parts[i+2].len())
                                .unwrap_or(cmd.len());
                            segments.push(HighlightedSegment {
                                text: cmd[start_pos..val2_end].to_string(),
                                is_highlighted: false,
                            });
                            current_pos = val2_end;
                        }
                        i += 3;
                        continue;
                    }
                }

                let tog_start = cmd[current_pos..].find("TOG").map(|p| current_pos + p);
                if let Some(start_pos) = tog_start {
                    if start_pos > current_pos {
                        segments.push(HighlightedSegment {
                            text: cmd[current_pos..start_pos].to_string(),
                            is_highlighted: false,
                        });
                    }

                    if let Some(highlighted_segments) = highlight_tog_expression(
                        &cmd[start_pos..],
                        &parts[i+1],
                        &parts[i+2],
                        &key,
                        toggle_last_value,
                    ) {
                        segments.extend(highlighted_segments);

                        let val2_end = find_whole_word(&cmd[start_pos..], parts[i+2])
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

                if !is_var_assignment {
                    if direct_validation.get(&key) == Some(&false) {
                        let eith_start = cmd[current_pos..].find("EITH").map(|p| current_pos + p);
                        if let Some(start_pos) = eith_start {
                            if start_pos > current_pos {
                                segments.push(HighlightedSegment {
                                    text: cmd[current_pos..start_pos].to_string(),
                                    is_highlighted: false,
                                });
                            }
                            let val2_end = find_whole_word(&cmd[start_pos..], parts[i+2])
                                .map(|p| start_pos + p + parts[i+2].len())
                                .unwrap_or(cmd.len());
                            segments.push(HighlightedSegment {
                                text: cmd[start_pos..val2_end].to_string(),
                                is_highlighted: false,
                            });
                            current_pos = val2_end;
                        }
                        i += 3;
                        continue;
                    }
                }

                let eith_start = cmd[current_pos..].find("EITH").map(|p| current_pos + p);
                if let Some(start_pos) = eith_start {
                    if start_pos > current_pos {
                        segments.push(HighlightedSegment {
                            text: cmd[current_pos..start_pos].to_string(),
                            is_highlighted: false,
                        });
                    }

                    if let Some(highlighted_segments) = highlight_eith_expression(
                        &cmd[start_pos..],
                        &parts[i+1],
                        &parts[i+2],
                        &key,
                        toggle_last_value,
                    ) {
                        segments.extend(highlighted_segments);

                        let val2_end = find_whole_word(&cmd[start_pos..], parts[i+2])
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
    toggle_last_value: &HashMap<String, i16>,
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

        if is_current && (token.starts_with('<') || token.starts_with('{')) {
            let inner_segments = highlight_nested_token(
                token,
                script_index,
                pattern,
                token_step_index,
                toggle_state,
                toggle_last_value,
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

fn highlight_nested_token(
    token: &str,
    script_index: usize,
    pattern: &str,
    step_index: usize,
    toggle_state: &HashMap<String, usize>,
    toggle_last_value: &HashMap<String, i16>,
) -> Vec<HighlightedSegment> {
    let mut segments = Vec::new();

    let (open_bracket, close_bracket, is_alternation) = if token.starts_with('<') {
        ('<', '>', true)
    } else if token.starts_with('{') {
        ('{', '}', false)
    } else {
        return vec![HighlightedSegment {
            text: token.to_string(),
            is_highlighted: true,
        }];
    };

    let bracket_end = token.rfind(close_bracket).unwrap_or(token.len() - 1);
    let inner_content = &token[1..bracket_end];
    let suffix = &token[bracket_end + 1..];

    let options: Vec<&str> = inner_content.split_whitespace().collect();

    if options.is_empty() {
        return vec![HighlightedSegment {
            text: token.to_string(),
            is_highlighted: true,
        }];
    }

    let key = if is_alternation {
        format!("seq_alt_{}_{}_{}", script_index, pattern, step_index)
    } else {
        format!("seq_rnd_{}_{}_{}", script_index, pattern, step_index)
    };

    let active_option_idx = toggle_last_value.get(&key).and_then(|&stored_value| {
        options.iter().position(|opt| {
            if let Ok(val) = opt.parse::<i16>() {
                val == stored_value
            } else if opt == &"_" || opt == &"." {
                stored_value == 0
            } else if opt.to_uppercase() == "X" {
                stored_value == 1
            } else {
                crate::eval::seq::parse_note_name(opt).map_or(false, |v| v == stored_value)
            }
        })
    });

    segments.push(HighlightedSegment {
        text: open_bracket.to_string(),
        is_highlighted: false,
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
        is_highlighted: false,
    });

    if !suffix.is_empty() {
        segments.push(HighlightedSegment {
            text: suffix.to_string(),
            is_highlighted: false,
        });
    }

    segments
}

fn highlight_tog_expression(
    line_from_tog: &str,
    val1: &str,
    val2: &str,
    key: &str,
    toggle_last_value: &HashMap<String, i16>,
) -> Option<Vec<HighlightedSegment>> {
    let mut segments = Vec::new();

    let val1_pos = find_whole_word(line_from_tog, val1)?;
    let val1_end = val1_pos + val1.len();

    let val2_pos = find_whole_word(&line_from_tog[val1_end..], val2).map(|p| p + val1_end)?;

    let val1_parsed = val1.parse::<i16>().ok();
    let val2_parsed = val2.parse::<i16>().ok();

    let stored_value = toggle_last_value.get(key);

    let tog_to_val1 = &line_from_tog[..val1_pos];
    segments.push(HighlightedSegment {
        text: tog_to_val1.to_string(),
        is_highlighted: false,
    });

    segments.push(HighlightedSegment {
        text: val1.to_string(),
        is_highlighted: stored_value.and_then(|&v| val1_parsed.map(|p| p == v)).unwrap_or(false),
    });

    let between = &line_from_tog[val1_end..val2_pos];
    segments.push(HighlightedSegment {
        text: between.to_string(),
        is_highlighted: false,
    });

    segments.push(HighlightedSegment {
        text: val2.to_string(),
        is_highlighted: stored_value.and_then(|&v| val2_parsed.map(|p| p == v)).unwrap_or(false),
    });

    Some(segments)
}

fn highlight_eith_expression(
    line_from_eith: &str,
    val1: &str,
    val2: &str,
    key: &str,
    toggle_last_value: &HashMap<String, i16>,
) -> Option<Vec<HighlightedSegment>> {
    let mut segments = Vec::new();

    let val1_pos = find_whole_word(line_from_eith, val1)?;
    let val1_end = val1_pos + val1.len();

    let val2_pos = find_whole_word(&line_from_eith[val1_end..], val2).map(|p| p + val1_end)?;

    let val1_parsed = val1.parse::<i16>().ok();
    let val2_parsed = val2.parse::<i16>().ok();

    let stored_value = toggle_last_value.get(key);

    let eith_to_val1 = &line_from_eith[..val1_pos];
    segments.push(HighlightedSegment {
        text: eith_to_val1.to_string(),
        is_highlighted: false,
    });

    segments.push(HighlightedSegment {
        text: val1.to_string(),
        is_highlighted: stored_value.and_then(|&v| val1_parsed.map(|p| p == v)).unwrap_or(false),
    });

    let between = &line_from_eith[val1_end..val2_pos];
    segments.push(HighlightedSegment {
        text: between.to_string(),
        is_highlighted: false,
    });

    segments.push(HighlightedSegment {
        text: val2.to_string(),
        is_highlighted: stored_value.and_then(|&v| val2_parsed.map(|p| p == v)).unwrap_or(false),
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

fn find_whole_word(haystack: &str, needle: &str) -> Option<usize> {
    let mut search_start = 0;
    while let Some(pos) = haystack[search_start..].find(needle) {
        let abs_pos = search_start + pos;
        let before_ok = abs_pos == 0 || {
            let prev_char = haystack[..abs_pos].chars().last().unwrap();
            !prev_char.is_alphanumeric() && prev_char != '_' && prev_char != '-'
        };
        let after_pos = abs_pos + needle.len();
        let after_ok = after_pos >= haystack.len() || {
            let next_char = haystack[after_pos..].chars().next().unwrap();
            !next_char.is_alphanumeric() && next_char != '_' && next_char != '-'
        };
        if before_ok && after_ok {
            return Some(abs_pos);
        }
        search_start = abs_pos + 1;
    }
    None
}
