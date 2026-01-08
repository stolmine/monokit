mod types;
mod logic;
mod activity;

pub use types::{HighlightedSegment, HighlightedLine};
pub use logic::highlight_stateful_operators;
pub use activity::apply_conditional_activity;

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_highlight_seq_basic() {
        let mut state = HashMap::new();
        state.insert("seq_0_C3 E3 G3".to_string(), 1);
        let mut last_value = HashMap::new();
        last_value.insert("seq_0_C3 E3 G3".to_string(), 4);
        let direct_validation = HashMap::new();

        let line = "N ON SEQ \"C3 E3 G3\"";
        let result = highlight_stateful_operators(line, 0, &state, &last_value, &direct_validation);

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
        let mut last_value = HashMap::new();
        last_value.insert("seq_0_C3 E3 G3".to_string(), 0);
        let direct_validation = HashMap::new();

        let line = "N ON SEQ \"C3 E3 G3\"";
        let result = highlight_stateful_operators(line, 0, &state, &last_value, &direct_validation);

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
        let mut last_value = HashMap::new();
        last_value.insert("0_TOG_10_20".to_string(), 10);

        let direct_validation = HashMap::new();

        let line = "N ON TOG 10 20";
        let result = highlight_stateful_operators(line, 0, &state, &last_value, &direct_validation);

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
        let mut last_value = HashMap::new();
        last_value.insert("0_TOG_10_20".to_string(), 20);

        let direct_validation = HashMap::new();

        let line = "N ON TOG 10 20";
        let result = highlight_stateful_operators(line, 0, &state, &last_value, &direct_validation);

        let highlighted_tokens: Vec<_> = result.segments
            .iter()
            .filter(|s| s.is_highlighted)
            .map(|s| s.text.as_str())
            .collect();

        assert_eq!(highlighted_tokens, vec!["20"]);
    }

    #[test]
    fn test_highlight_tog_with_zero() {
        let state = HashMap::new();
        let mut last_value = HashMap::new();
        last_value.insert("0_TOG_2000_0".to_string(), 2000);

        let direct_validation = HashMap::new();

        let line = "DC TOG 2000 0";
        let result = highlight_stateful_operators(line, 0, &state, &last_value, &direct_validation);

        let highlighted_tokens: Vec<_> = result.segments
            .iter()
            .filter(|s| s.is_highlighted)
            .map(|s| s.text.as_str())
            .collect();

        assert_eq!(highlighted_tokens, vec!["2000"]);

        let full_text: String = result.segments.iter().map(|s| s.text.as_str()).collect();
        assert_eq!(full_text, "DC TOG 2000 0");
    }

    #[test]
    fn test_highlight_tog_with_zero_second_value() {
        let mut state = HashMap::new();
        state.insert("0_TOG_2000_0".to_string(), 1);
        let mut last_value = HashMap::new();
        last_value.insert("0_TOG_2000_0".to_string(), 0);

        let direct_validation = HashMap::new();

        let line = "DC TOG 2000 0";
        let result = highlight_stateful_operators(line, 0, &state, &last_value, &direct_validation);

        let highlighted_tokens: Vec<_> = result.segments
            .iter()
            .filter(|s| s.is_highlighted)
            .map(|s| s.text.as_str())
            .collect();

        assert_eq!(highlighted_tokens, vec!["0"]);
    }

    #[test]
    fn test_highlight_seq_with_repeat() {
        let mut state = HashMap::new();
        state.insert("seq_0_C3*2 E3".to_string(), 1);
        let mut last_value = HashMap::new();
        last_value.insert("seq_0_C3*2 E3".to_string(), 0);

        let direct_validation = HashMap::new();

        let line = "N ON SEQ \"C3*2 E3\"";
        let result = highlight_stateful_operators(line, 0, &state, &last_value, &direct_validation);

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
        let last_value = HashMap::new();
        let direct_validation = HashMap::new();
        let line = "N ON 60";
        let result = highlight_stateful_operators(line, 0, &state, &last_value, &direct_validation);

        assert_eq!(result.segments.len(), 1);
        assert_eq!(result.segments[0].text, "N ON 60");
        assert!(!result.segments[0].is_highlighted);
    }

    #[test]
    fn test_highlight_seq_alternation_first_option() {
        let mut state = HashMap::new();
        state.insert("seq_0_5 3 <1 0> 4".to_string(), 2);
        state.insert("seq_alt_0_5 3 <1 0> 4_2".to_string(), 0);
        let mut last_value = HashMap::new();
        last_value.insert("seq_alt_0_5 3 <1 0> 4_2".to_string(), 1);

        let direct_validation = HashMap::new();

        let line = "PF SEQ \"5 3 <1 0> 4\"";
        let result = highlight_stateful_operators(line, 0, &state, &last_value, &direct_validation);

        let highlighted: Vec<_> = result.segments
            .iter()
            .filter(|s| s.is_highlighted)
            .map(|s| s.text.as_str())
            .collect();

        assert!(!highlighted.contains(&"<"), "Should NOT highlight open bracket");
        assert!(highlighted.contains(&"1"), "Should highlight first option");
        assert!(!highlighted.contains(&">"), "Should NOT highlight close bracket");
        assert!(!highlighted.contains(&"0"), "Should NOT highlight second option");
    }

    #[test]
    fn test_highlight_seq_alternation_second_option() {
        let mut state = HashMap::new();
        state.insert("seq_0_5 3 <1 0> 4".to_string(), 2);
        state.insert("seq_alt_0_5 3 <1 0> 4_2".to_string(), 1);
        let mut last_value = HashMap::new();
        last_value.insert("seq_alt_0_5 3 <1 0> 4_2".to_string(), 0);

        let direct_validation = HashMap::new();

        let line = "PF SEQ \"5 3 <1 0> 4\"";
        let result = highlight_stateful_operators(line, 0, &state, &last_value, &direct_validation);

        let highlighted: Vec<_> = result.segments
            .iter()
            .filter(|s| s.is_highlighted)
            .map(|s| s.text.as_str())
            .collect();

        assert!(!highlighted.contains(&"<"), "Should NOT highlight open bracket");
        assert!(highlighted.contains(&"0"), "Should highlight second option");
        assert!(!highlighted.contains(&">"), "Should NOT highlight close bracket");
        assert!(!highlighted.contains(&"1"), "Should NOT highlight first option");
    }

    #[test]
    fn test_highlight_seq_random_choice_first_option() {
        let mut state = HashMap::new();
        state.insert("seq_0_5 3 {1 0} 4".to_string(), 2);
        state.insert("seq_rnd_0_5 3 {1 0} 4_2".to_string(), 0);
        let mut last_value = HashMap::new();
        last_value.insert("seq_rnd_0_5 3 {1 0} 4_2".to_string(), 1);

        let direct_validation = HashMap::new();

        let line = "PF SEQ \"5 3 {1 0} 4\"";
        let result = highlight_stateful_operators(line, 0, &state, &last_value, &direct_validation);

        let highlighted: Vec<_> = result.segments
            .iter()
            .filter(|s| s.is_highlighted)
            .map(|s| s.text.as_str())
            .collect();

        assert!(!highlighted.contains(&"{"), "Should NOT highlight open bracket");
        assert!(highlighted.contains(&"1"), "Should highlight first option");
        assert!(!highlighted.contains(&"}"), "Should NOT highlight close bracket");
        assert!(!highlighted.contains(&"0"), "Should NOT highlight second option");
    }

    #[test]
    fn test_highlight_seq_random_choice_second_option() {
        let mut state = HashMap::new();
        state.insert("seq_0_5 3 {1 0} 4".to_string(), 2);
        state.insert("seq_rnd_0_5 3 {1 0} 4_2".to_string(), 1);
        let mut last_value = HashMap::new();
        last_value.insert("seq_rnd_0_5 3 {1 0} 4_2".to_string(), 0);

        let direct_validation = HashMap::new();

        let line = "PF SEQ \"5 3 {1 0} 4\"";
        let result = highlight_stateful_operators(line, 0, &state, &last_value, &direct_validation);

        let highlighted: Vec<_> = result.segments
            .iter()
            .filter(|s| s.is_highlighted)
            .map(|s| s.text.as_str())
            .collect();

        assert!(!highlighted.contains(&"{"), "Should NOT highlight open bracket");
        assert!(highlighted.contains(&"0"), "Should highlight second option");
        assert!(!highlighted.contains(&"}"), "Should NOT highlight close bracket");
        assert!(!highlighted.contains(&"1"), "Should NOT highlight first option");
    }

    #[test]
    fn test_highlight_seq_no_space_before_quote() {
        let mut state = HashMap::new();
        state.insert("seq_0_0 10 2".to_string(), 1);
        let mut last_value = HashMap::new();
        last_value.insert("seq_0_0 10 2".to_string(), 10);

        let direct_validation = HashMap::new();

        let line = "N ON SEQ\"0 10 2\"";
        let result = highlight_stateful_operators(line, 0, &state, &last_value, &direct_validation);

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
        let last_value = HashMap::new();
        let direct_validation = HashMap::new();
        let line = "N ON SEQ \"C3 E3";
        let result = highlight_stateful_operators(line, 0, &state, &last_value, &direct_validation);

        assert!(!result.segments.is_empty());
        let has_any_highlighted = result.segments.iter().any(|s| s.is_highlighted);
        assert!(!has_any_highlighted, "Unterminated quote should not highlight anything");
    }

    #[test]
    fn test_highlight_seq_with_semicolon_separated_commands() {
        let mut state = HashMap::new();
        state.insert("seq_0_5000*15 1250".to_string(), 0);
        let mut last_value = HashMap::new();
        last_value.insert("seq_0_5000*15 1250".to_string(), 5000);

        let direct_validation = HashMap::new();

        let line = "PF SEQ \"5000*15 1250\"; AD 5; PA 0";
        let result = highlight_stateful_operators(line, 0, &state, &last_value, &direct_validation);

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
        let mut last_value = HashMap::new();
        last_value.insert("0_EITH_10_20".to_string(), 10);

        let direct_validation = HashMap::new();

        let line = "N ON EITH 10 20";
        let result = highlight_stateful_operators(line, 0, &state, &last_value, &direct_validation);

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
        let mut last_value = HashMap::new();
        last_value.insert("0_EITH_100_200".to_string(), 200);

        let direct_validation = HashMap::new();

        let line = "PF EITH 100 200";
        let result = highlight_stateful_operators(line, 0, &state, &last_value, &direct_validation);

        let highlighted_tokens: Vec<_> = result.segments
            .iter()
            .filter(|s| s.is_highlighted)
            .map(|s| s.text.as_str())
            .collect();

        assert_eq!(highlighted_tokens, vec!["200"]);
    }
}
