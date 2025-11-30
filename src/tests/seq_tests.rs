use crate::eval::eval_expression;
use crate::test_setup;

// ============================================================================
// Basic SEQ cycling tests
// ============================================================================

#[test]
fn test_seq_cycles_through_values() {
    let (variables, mut patterns, scripts, mut counters, scale) = test_setup!();
    let parts = vec!["SEQ", "\"1 2 3\""];

    let result1 = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert!(result1.is_some());
    let (value1, consumed1) = result1.unwrap();
    assert_eq!(value1, 1, "First SEQ call should return first value");
    assert_eq!(consumed1, 2, "Should consume SEQ and quoted string");

    let result2 = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert_eq!(result2.unwrap().0, 2, "Second call should return second value");

    let result3 = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert_eq!(result3.unwrap().0, 3, "Third call should return third value");

    let result4 = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert_eq!(result4.unwrap().0, 1, "Fourth call should wrap to first value");
}

#[test]
fn test_seq_with_triggers() {
    let (variables, mut patterns, scripts, mut counters, scale) = test_setup!();
    let parts = vec!["SEQ", "\"x _ x _\""];

    let result1 = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert_eq!(result1.unwrap().0, 1, "x should return 1");

    let result2 = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert_eq!(result2.unwrap().0, 0, "_ should return 0");

    let result3 = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert_eq!(result3.unwrap().0, 1, "x should return 1");

    let result4 = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert_eq!(result4.unwrap().0, 0, "_ should return 0");
}

#[test]
fn test_seq_with_rests_dot() {
    let (variables, mut patterns, scripts, mut counters, scale) = test_setup!();
    let parts = vec!["SEQ", "\"x . x .\""];

    let result1 = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert_eq!(result1.unwrap().0, 1);

    let result2 = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert_eq!(result2.unwrap().0, 0, ". should return 0 (rest)");
}

// ============================================================================
// Note name tests
// ============================================================================

#[test]
fn test_seq_with_note_names() {
    let (variables, mut patterns, scripts, mut counters, scale) = test_setup!();
    let parts = vec!["SEQ", "\"C3 E3 G3 C4\""];

    let result1 = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert_eq!(result1.unwrap().0, 0, "C3 should be 0 semitones");

    let result2 = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert_eq!(result2.unwrap().0, 4, "E3 should be 4 semitones");

    let result3 = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert_eq!(result3.unwrap().0, 7, "G3 should be 7 semitones");

    let result4 = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert_eq!(result4.unwrap().0, 12, "C4 should be 12 semitones");
}

#[test]
fn test_seq_with_sharps() {
    let (variables, mut patterns, scripts, mut counters, scale) = test_setup!();
    let parts = vec!["SEQ", "\"C#3 F#3\""];

    let result1 = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert_eq!(result1.unwrap().0, 1, "C#3 should be 1 semitone");

    let result2 = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert_eq!(result2.unwrap().0, 6, "F#3 should be 6 semitones");
}

#[test]
fn test_seq_with_flats() {
    let (variables, mut patterns, scripts, mut counters, scale) = test_setup!();
    let parts = vec!["SEQ", "\"Bb3 Eb3\""];

    let result1 = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert_eq!(result1.unwrap().0, 10, "Bb3 should be 10 semitones");

    let result2 = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert_eq!(result2.unwrap().0, 3, "Eb3 should be 3 semitones");
}

#[test]
fn test_seq_with_negative_octaves() {
    let (variables, mut patterns, scripts, mut counters, scale) = test_setup!();
    let parts = vec!["SEQ", "\"C2 C1\""];

    let result1 = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert_eq!(result1.unwrap().0, -12, "C2 should be -12 semitones");

    let result2 = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert_eq!(result2.unwrap().0, -24, "C1 should be -24 semitones");
}

// ============================================================================
// Numeric value tests
// ============================================================================

#[test]
fn test_seq_with_numbers() {
    let (variables, mut patterns, scripts, mut counters, scale) = test_setup!();
    let parts = vec!["SEQ", "\"100 200 300\""];

    let result1 = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert_eq!(result1.unwrap().0, 100);

    let result2 = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert_eq!(result2.unwrap().0, 200);

    let result3 = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert_eq!(result3.unwrap().0, 300);
}

#[test]
fn test_seq_with_negative_numbers() {
    let (variables, mut patterns, scripts, mut counters, scale) = test_setup!();
    let parts = vec!["SEQ", "\"-12 0 12\""];

    let result1 = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert_eq!(result1.unwrap().0, -12);

    let result2 = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert_eq!(result2.unwrap().0, 0);

    let result3 = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert_eq!(result3.unwrap().0, 12);
}

// ============================================================================
// State independence tests
// ============================================================================

#[test]
fn test_seq_independent_patterns() {
    let (variables, mut patterns, scripts, mut counters, scale) = test_setup!();

    let parts1 = vec!["SEQ", "\"1 2\""];
    let parts2 = vec!["SEQ", "\"10 20\""];

    // First call to each
    let result1a = eval_expression(&parts1, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert_eq!(result1a.unwrap().0, 1);

    let result2a = eval_expression(&parts2, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert_eq!(result2a.unwrap().0, 10);

    // Second call - should be independent
    let result1b = eval_expression(&parts1, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert_eq!(result1b.unwrap().0, 2, "SEQ \"1 2\" should maintain its own state");

    let result2b = eval_expression(&parts2, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert_eq!(result2b.unwrap().0, 20, "SEQ \"10 20\" should maintain its own state");
}

#[test]
fn test_seq_per_script_independence() {
    let (variables, mut patterns, scripts, mut counters, scale) = test_setup!();
    let parts = vec!["SEQ", "\"100 200\""];

    // Script 0
    let result1 = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert_eq!(result1.unwrap().0, 100);

    // Script 1 should have its own state
    let result2 = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 1, &scale);
    assert_eq!(result2.unwrap().0, 100, "Different script should start at first value");

    // Script 0 should continue from where it left off
    let result3 = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert_eq!(result3.unwrap().0, 200, "Script 0 should continue its sequence");
}

// ============================================================================
// Multi-part quoted string tests
// ============================================================================

#[test]
fn test_seq_multi_part_quoted_string() {
    let (variables, mut patterns, scripts, mut counters, scale) = test_setup!();
    // Simulates how the parser might split "C3 E3 G3"
    let parts = vec!["SEQ", "\"C3", "E3", "G3\""];

    let result1 = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert!(result1.is_some());
    let (value1, consumed1) = result1.unwrap();
    assert_eq!(value1, 0, "C3 = 0");
    assert_eq!(consumed1, 4, "Should consume SEQ and all quoted parts");

    let result2 = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert_eq!(result2.unwrap().0, 4, "E3 = 4");

    let result3 = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert_eq!(result3.unwrap().0, 7, "G3 = 7");
}

#[test]
fn test_seq_single_quote() {
    let (variables, mut patterns, scripts, mut counters, scale) = test_setup!();
    let parts = vec!["SEQ", "'x _ x'"];

    let result1 = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert_eq!(result1.unwrap().0, 1);

    let result2 = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert_eq!(result2.unwrap().0, 0);

    let result3 = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert_eq!(result3.unwrap().0, 1);
}

// ============================================================================
// Edge cases
// ============================================================================

#[test]
fn test_seq_single_element() {
    let (variables, mut patterns, scripts, mut counters, scale) = test_setup!();
    let parts = vec!["SEQ", "\"42\""];

    let result1 = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert_eq!(result1.unwrap().0, 42);

    let result2 = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert_eq!(result2.unwrap().0, 42, "Single element should always return same value");
}

#[test]
fn test_seq_empty_returns_zero() {
    let (variables, mut patterns, scripts, mut counters, scale) = test_setup!();
    let parts = vec!["SEQ", "\"\""];

    let result = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert!(result.is_some());
    assert_eq!(result.unwrap().0, 0, "Empty SEQ should return 0");
}

#[test]
fn test_seq_missing_quote_returns_none() {
    let (variables, mut patterns, scripts, mut counters, scale) = test_setup!();
    let parts = vec!["SEQ", "C3"];

    let result = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert!(result.is_none(), "SEQ without quotes should return None");
}

#[test]
fn test_seq_no_args_returns_none() {
    let (variables, mut patterns, scripts, mut counters, scale) = test_setup!();
    let parts = vec!["SEQ"];

    let result = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert!(result.is_none(), "SEQ with no arguments should return None");
}

// ============================================================================
// Integration with other expressions
// ============================================================================

#[test]
fn test_seq_in_math_expression() {
    let (variables, mut patterns, scripts, mut counters, scale) = test_setup!();
    let parts = vec!["ADD", "SEQ", "\"10 20\"", "5"];

    let result1 = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert!(result1.is_some());
    assert_eq!(result1.unwrap().0, 15, "ADD (SEQ \"10 20\") 5 first = 10 + 5");

    let result2 = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert_eq!(result2.unwrap().0, 25, "ADD (SEQ \"10 20\") 5 second = 20 + 5");
}

#[test]
fn test_seq_with_mul() {
    let (variables, mut patterns, scripts, mut counters, scale) = test_setup!();
    let parts = vec!["MUL", "SEQ", "\"2 3 4\"", "10"];

    let result1 = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert_eq!(result1.unwrap().0, 20);

    let result2 = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert_eq!(result2.unwrap().0, 30);

    let result3 = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert_eq!(result3.unwrap().0, 40);
}

// ============================================================================
// Mixed content tests
// ============================================================================

#[test]
fn test_seq_mixed_notes_and_rests() {
    let (variables, mut patterns, scripts, mut counters, scale) = test_setup!();
    let parts = vec!["SEQ", "\"C3 _ E3 _\""];

    let result1 = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert_eq!(result1.unwrap().0, 0, "C3 = 0");

    let result2 = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert_eq!(result2.unwrap().0, 0, "rest = 0");

    let result3 = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert_eq!(result3.unwrap().0, 4, "E3 = 4");

    let result4 = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert_eq!(result4.unwrap().0, 0, "rest = 0");
}

#[test]
fn test_seq_mixed_notes_and_numbers() {
    let (variables, mut patterns, scripts, mut counters, scale) = test_setup!();
    let parts = vec!["SEQ", "\"C3 100 E3 200\""];

    let result1 = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert_eq!(result1.unwrap().0, 0, "C3 = 0");

    let result2 = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert_eq!(result2.unwrap().0, 100);

    let result3 = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert_eq!(result3.unwrap().0, 4, "E3 = 4");

    let result4 = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert_eq!(result4.unwrap().0, 200);
}
