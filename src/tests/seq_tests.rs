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

// ============================================================================
// Random trigger tests
// ============================================================================

#[test]
fn test_seq_parse_random_trigger() {
    use crate::eval::seq::{parse_seq_pattern, SeqStep};

    let steps = parse_seq_pattern("x ? x _").unwrap();
    assert_eq!(steps.len(), 4);
    assert_eq!(steps[0], SeqStep::Value(1));
    assert_eq!(steps[1], SeqStep::RandomTrigger);
    assert_eq!(steps[2], SeqStep::Value(1));
    assert_eq!(steps[3], SeqStep::Rest);
}

#[test]
fn test_seq_random_trigger_produces_both_values() {
    let (variables, mut patterns, scripts, mut counters, scale) = test_setup!();
    let parts = vec!["SEQ", "\"?\""];

    let mut seen_zero = false;
    let mut seen_one = false;

    // Run enough times to be statistically confident we see both values
    for _ in 0..100 {
        let result = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
        let value = result.unwrap().0;
        assert!(value == 0 || value == 1, "Random trigger should only produce 0 or 1");

        if value == 0 {
            seen_zero = true;
        }
        if value == 1 {
            seen_one = true;
        }

        if seen_zero && seen_one {
            break;
        }
    }

    assert!(seen_zero, "Should have seen 0 at least once in 100 iterations");
    assert!(seen_one, "Should have seen 1 at least once in 100 iterations");
}

#[test]
fn test_seq_multiple_random_triggers() {
    let (variables, mut patterns, scripts, mut counters, scale) = test_setup!();
    let parts = vec!["SEQ", "\"? ? ?\""];

    // Each call should produce a value from the sequence
    // Since there are 3 random triggers, we should see various combinations
    let mut results = Vec::new();

    for _ in 0..30 {
        let result = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
        let value = result.unwrap().0;
        assert!(value == 0 || value == 1, "Each random trigger should produce 0 or 1");
        results.push(value);
    }

    // With 30 calls over 3 positions cycling, we should see some variation
    let unique_count = results.iter().collect::<std::collections::HashSet<_>>().len();
    assert!(unique_count >= 2, "Should see at least both 0 and 1 values");
}

#[test]
fn test_seq_random_trigger_combined_with_values() {
    let (variables, mut patterns, scripts, mut counters, scale) = test_setup!();
    let parts = vec!["SEQ", "\"100 ? 200\""];

    // First value should be 100
    let result1 = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert_eq!(result1.unwrap().0, 100);

    // Second value should be random (0 or 1)
    let result2 = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    let random_value = result2.unwrap().0;
    assert!(random_value == 0 || random_value == 1, "Random trigger should produce 0 or 1");

    // Third value should be 200
    let result3 = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert_eq!(result3.unwrap().0, 200);

    // Fourth should wrap to 100
    let result4 = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert_eq!(result4.unwrap().0, 100);
}

// ============================================================================
// Repeat syntax tests (*n)
// ============================================================================

#[test]
fn test_seq_basic_repeat() {
    let (variables, mut patterns, scripts, mut counters, scale) = test_setup!();
    let parts = vec!["SEQ", "\"C3*3\""];

    let result1 = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert_eq!(result1.unwrap().0, 0, "C3 first");

    let result2 = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert_eq!(result2.unwrap().0, 0, "C3 second");

    let result3 = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert_eq!(result3.unwrap().0, 0, "C3 third");

    let result4 = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert_eq!(result4.unwrap().0, 0, "C3 wraps to first");
}

#[test]
fn test_seq_multiple_repeats() {
    let (variables, mut patterns, scripts, mut counters, scale) = test_setup!();
    let parts = vec!["SEQ", "\"C3*2 E3*3\""];

    let result1 = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert_eq!(result1.unwrap().0, 0, "C3 first");

    let result2 = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert_eq!(result2.unwrap().0, 0, "C3 second");

    let result3 = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert_eq!(result3.unwrap().0, 4, "E3 first");

    let result4 = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert_eq!(result4.unwrap().0, 4, "E3 second");

    let result5 = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert_eq!(result5.unwrap().0, 4, "E3 third");

    let result6 = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert_eq!(result6.unwrap().0, 0, "Wraps to C3 first");
}

#[test]
fn test_seq_zero_repeat() {
    let (variables, mut patterns, scripts, mut counters, scale) = test_setup!();
    let parts = vec!["SEQ", "\"C3*0 E3\""];

    let result1 = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert_eq!(result1.unwrap().0, 4, "C3*0 is skipped, E3 is first");

    let result2 = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert_eq!(result2.unwrap().0, 4, "Wraps to E3");
}

#[test]
fn test_seq_repeat_with_rests() {
    let (variables, mut patterns, scripts, mut counters, scale) = test_setup!();
    let parts = vec!["SEQ", "\"x*2 _*2\""];

    let result1 = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert_eq!(result1.unwrap().0, 1, "x first");

    let result2 = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert_eq!(result2.unwrap().0, 1, "x second");

    let result3 = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert_eq!(result3.unwrap().0, 0, "_ first");

    let result4 = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert_eq!(result4.unwrap().0, 0, "_ second");

    let result5 = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert_eq!(result5.unwrap().0, 1, "Wraps to x first");
}

#[test]
fn test_seq_repeat_one() {
    let (variables, mut patterns, scripts, mut counters, scale) = test_setup!();
    let parts = vec!["SEQ", "\"C3*1 E3\""];

    let result1 = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert_eq!(result1.unwrap().0, 0, "C3*1 is same as C3");

    let result2 = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert_eq!(result2.unwrap().0, 4, "E3");
}

#[test]
fn test_seq_repeat_with_numbers() {
    let (variables, mut patterns, scripts, mut counters, scale) = test_setup!();
    let parts = vec!["SEQ", "\"100*2 200*3\""];

    let result1 = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert_eq!(result1.unwrap().0, 100);

    let result2 = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert_eq!(result2.unwrap().0, 100);

    let result3 = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert_eq!(result3.unwrap().0, 200);

    let result4 = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert_eq!(result4.unwrap().0, 200);

    let result5 = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert_eq!(result5.unwrap().0, 200);
}

#[test]
fn test_seq_parse_pattern_invalid_repeat() {
    use crate::eval::seq::parse_seq_pattern;

    let result = parse_seq_pattern("C3*abc");
    assert!(result.is_err(), "Invalid repeat count should error");
    assert!(result.unwrap_err().contains("Invalid repeat count"));
}

// ============================================================================
// Alternation syntax tests (<a b>)
// ============================================================================

#[test]
fn test_seq_parse_alternation_basic() {
    use crate::eval::seq::{parse_seq_pattern, SeqStep};

    let steps = parse_seq_pattern("<C3 E3>").unwrap();
    assert_eq!(steps.len(), 1);
    match &steps[0] {
        SeqStep::Alternation(options) => {
            assert_eq!(options.len(), 2);
            assert_eq!(options[0], SeqStep::Value(0));
            assert_eq!(options[1], SeqStep::Value(4));
        }
        _ => panic!("Expected Alternation variant"),
    }
}

#[test]
fn test_seq_parse_alternation_three_options() {
    use crate::eval::seq::{parse_seq_pattern, SeqStep};

    let steps = parse_seq_pattern("<C3 E3 G3>").unwrap();
    assert_eq!(steps.len(), 1);
    match &steps[0] {
        SeqStep::Alternation(options) => {
            assert_eq!(options.len(), 3);
            assert_eq!(options[0], SeqStep::Value(0));
            assert_eq!(options[1], SeqStep::Value(4));
            assert_eq!(options[2], SeqStep::Value(7));
        }
        _ => panic!("Expected Alternation variant"),
    }
}

#[test]
fn test_seq_parse_alternation_with_repeat() {
    use crate::eval::seq::{parse_seq_pattern, SeqStep};

    let steps = parse_seq_pattern("<C3 E3>*2").unwrap();
    assert_eq!(steps.len(), 2);
    match &steps[0] {
        SeqStep::Alternation(options) => {
            assert_eq!(options.len(), 2);
        }
        _ => panic!("Expected Alternation variant"),
    }
    match &steps[1] {
        SeqStep::Alternation(options) => {
            assert_eq!(options.len(), 2);
        }
        _ => panic!("Expected Alternation variant"),
    }
}

#[test]
fn test_seq_parse_alternation_single_option() {
    use crate::eval::seq::{parse_seq_pattern, SeqStep};

    let steps = parse_seq_pattern("<C3>").unwrap();
    assert_eq!(steps.len(), 1);
    match &steps[0] {
        SeqStep::Alternation(options) => {
            assert_eq!(options.len(), 1);
            assert_eq!(options[0], SeqStep::Value(0));
        }
        _ => panic!("Expected Alternation variant"),
    }
}

#[test]
fn test_seq_parse_alternation_mixed_with_regular() {
    use crate::eval::seq::{parse_seq_pattern, SeqStep};

    let steps = parse_seq_pattern("C3 <E3 G3> C4").unwrap();
    assert_eq!(steps.len(), 3);
    assert_eq!(steps[0], SeqStep::Value(0));
    match &steps[1] {
        SeqStep::Alternation(options) => {
            assert_eq!(options.len(), 2);
        }
        _ => panic!("Expected Alternation variant"),
    }
    assert_eq!(steps[2], SeqStep::Value(12));
}

#[test]
fn test_seq_parse_alternation_with_triggers() {
    use crate::eval::seq::{parse_seq_pattern, SeqStep};

    let steps = parse_seq_pattern("<x _>").unwrap();
    assert_eq!(steps.len(), 1);
    match &steps[0] {
        SeqStep::Alternation(options) => {
            assert_eq!(options.len(), 2);
            assert_eq!(options[0], SeqStep::Value(1));
            assert_eq!(options[1], SeqStep::Rest);
        }
        _ => panic!("Expected Alternation variant"),
    }
}

#[test]
fn test_seq_parse_alternation_with_random_trigger() {
    use crate::eval::seq::{parse_seq_pattern, SeqStep};

    let steps = parse_seq_pattern("<C3 ?>").unwrap();
    assert_eq!(steps.len(), 1);
    match &steps[0] {
        SeqStep::Alternation(options) => {
            assert_eq!(options.len(), 2);
            assert_eq!(options[0], SeqStep::Value(0));
            assert_eq!(options[1], SeqStep::RandomTrigger);
        }
        _ => panic!("Expected Alternation variant"),
    }
}

#[test]
fn test_seq_parse_alternation_empty_error() {
    use crate::eval::seq::parse_seq_pattern;

    let result = parse_seq_pattern("<>");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Empty alternation"));
}

#[test]
fn test_seq_parse_alternation_unclosed_error() {
    use crate::eval::seq::parse_seq_pattern;

    let result = parse_seq_pattern("<C3 E3");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Unclosed alternation"));
}

#[test]
fn test_seq_parse_alternation_nested_error() {
    use crate::eval::seq::parse_seq_pattern;

    let result = parse_seq_pattern("<<C3 E3> G3>");
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Nested alternation"));
}

#[test]
fn test_seq_alternation_produces_both_values() {
    let (variables, mut patterns, scripts, mut counters, scale) = test_setup!();
    let parts = vec!["SEQ", "\"<C3 E3>\""];

    // First call should return C3 (deterministic cycling starts at 0)
    let result1 = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert_eq!(result1.unwrap().0, 0, "First call should return C3 (0)");

    // Second call should return E3 (cycles to next option)
    let result2 = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert_eq!(result2.unwrap().0, 4, "Second call should return E3 (4)");

    // Third call should wrap back to C3
    let result3 = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert_eq!(result3.unwrap().0, 0, "Third call should wrap to C3 (0)");

    // Fourth call should be E3 again
    let result4 = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert_eq!(result4.unwrap().0, 4, "Fourth call should be E3 (4)");
}

#[test]
fn test_seq_alternation_three_options_produces_all() {
    let (variables, mut patterns, scripts, mut counters, scale) = test_setup!();
    let parts = vec!["SEQ", "\"<C3 E3 G3>\""];

    // Deterministic cycling through all three options
    let result1 = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert_eq!(result1.unwrap().0, 0, "First call should return C3");

    let result2 = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert_eq!(result2.unwrap().0, 4, "Second call should return E3");

    let result3 = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert_eq!(result3.unwrap().0, 7, "Third call should return G3");

    let result4 = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert_eq!(result4.unwrap().0, 0, "Fourth call should wrap to C3");
}

#[test]
fn test_seq_alternation_with_repeat_independent() {
    let (variables, mut patterns, scripts, mut counters, scale) = test_setup!();
    let parts = vec!["SEQ", "\"<C3 E3>*2\""];

    let result1 = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    let value1 = result1.unwrap().0;
    assert!(value1 == 0 || value1 == 4);

    let result2 = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    let value2 = result2.unwrap().0;
    assert!(value2 == 0 || value2 == 4);

    let result3 = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    let value3 = result3.unwrap().0;
    assert!(value3 == 0 || value3 == 4, "Should wrap to first alternation");
}

#[test]
fn test_seq_alternation_single_option_always_same() {
    let (variables, mut patterns, scripts, mut counters, scale) = test_setup!();
    let parts = vec!["SEQ", "\"<C3>\""];

    for _ in 0..10 {
        let result = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
        assert_eq!(result.unwrap().0, 0, "Single option should always return C3");
    }
}

#[test]
fn test_seq_alternation_mixed_with_regular_steps() {
    let (variables, mut patterns, scripts, mut counters, scale) = test_setup!();
    let parts = vec!["SEQ", "\"100 <C3 E3> 200\""];

    let result1 = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert_eq!(result1.unwrap().0, 100, "First should be 100");

    let result2 = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    let value2 = result2.unwrap().0;
    assert!(value2 == 0 || value2 == 4, "Second should be alternation");

    let result3 = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert_eq!(result3.unwrap().0, 200, "Third should be 200");

    let result4 = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert_eq!(result4.unwrap().0, 100, "Should wrap to 100");
}

#[test]
fn test_seq_alternation_with_rest_and_trigger() {
    let (variables, mut patterns, scripts, mut counters, scale) = test_setup!();
    let parts = vec!["SEQ", "\"<x _>\""];

    // Deterministic cycling between trigger and rest
    let result1 = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert_eq!(result1.unwrap().0, 1, "First call should be x (1)");

    let result2 = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert_eq!(result2.unwrap().0, 0, "Second call should be _ (0)");

    let result3 = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert_eq!(result3.unwrap().0, 1, "Third call should wrap to x (1)");

    let result4 = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert_eq!(result4.unwrap().0, 0, "Fourth call should be _ (0)");
}

#[test]
fn test_seq_alternation_deterministic_cycle() {
    let (variables, mut patterns, scripts, mut counters, scale) = test_setup!();
    let parts = vec!["SEQ", "\"<C3 E3>\""];

    // Verify exact cycling behavior: C3, E3, C3, E3, C3...
    let result1 = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert_eq!(result1.unwrap().0, 0, "First: C3");

    let result2 = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert_eq!(result2.unwrap().0, 4, "Second: E3");

    let result3 = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert_eq!(result3.unwrap().0, 0, "Third: C3");

    let result4 = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert_eq!(result4.unwrap().0, 4, "Fourth: E3");

    let result5 = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert_eq!(result5.unwrap().0, 0, "Fifth: C3");
}
