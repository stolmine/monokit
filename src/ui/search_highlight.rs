pub fn highlight_matches_in_line(
    line: &str,
    query: &str,
    current_match_col: Option<usize>,
) -> Vec<(String, bool, bool)> {
    if query.is_empty() {
        return vec![(line.to_string(), false, false)];
    }

    let mut segments = Vec::new();
    let lower_line = line.to_lowercase();
    let lower_query = query.to_lowercase();
    let mut pos = 0;

    while pos < line.len() {
        if let Some(match_start) = lower_line[pos..].find(&lower_query) {
            let absolute_start = pos + match_start;
            let match_end = absolute_start + query.len();

            if absolute_start > pos {
                segments.push((line[pos..absolute_start].to_string(), false, false));
            }

            let is_current = current_match_col == Some(absolute_start);
            segments.push((line[absolute_start..match_end].to_string(), true, is_current));

            pos = match_end;
        } else {
            segments.push((line[pos..].to_string(), false, false));
            break;
        }
    }

    segments
}
