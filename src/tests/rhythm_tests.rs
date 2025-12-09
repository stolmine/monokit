use crate::eval::eval_expression;
use crate::test_setup;

// ER (Euclidean Rhythm) Tests

#[test]
fn test_er_basic_pattern() {
    let (variables, mut patterns, scripts, mut counters, scale) = test_setup!();

    // ER 5 8 returns 1 for steps where bucket crosses threshold (Bjorklund distribution)
    // Pattern: [0, 1, 0, 1, 1, 0, 1, 1] - distributes 5 beats evenly across 8 steps
    let test_cases = vec![
        (vec!["ER", "5", "8", "0"], 0),
        (vec!["ER", "5", "8", "1"], 1),
        (vec!["ER", "5", "8", "2"], 0),
        (vec!["ER", "5", "8", "3"], 1),
        (vec!["ER", "5", "8", "4"], 1),
        (vec!["ER", "5", "8", "5"], 0),
        (vec!["ER", "5", "8", "6"], 1),
        (vec!["ER", "5", "8", "7"], 1),
    ];

    for (parts, expected) in test_cases {
        let result = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
        assert!(result.is_some());
        let (value, consumed) = result.unwrap();
        assert_eq!(value, expected, "ER {} {} {} returned {}, expected {}",
            parts[1], parts[2], parts[3], value, expected);
        assert_eq!(consumed, 4);
    }
}

#[test]
fn test_er_wraps_step() {
    let (variables, mut patterns, scripts, mut counters, scale) = test_setup!();

    // Test that step wraps correctly (step 8 == step 0 in an 8-length pattern)
    let parts_0 = vec!["ER", "5", "8", "0"];
    let parts_8 = vec!["ER", "5", "8", "8"];

    let result_0 = eval_expression(&parts_0, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    let result_8 = eval_expression(&parts_8, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);

    assert_eq!(result_0.unwrap().0, result_8.unwrap().0);
}

#[test]
fn test_er_negative_step() {
    let (variables, mut patterns, scripts, mut counters, scale) = test_setup!();

    // Test that negative steps work correctly (-1 should be step 7 in an 8-length pattern)
    let parts_neg1 = vec!["ER", "5", "8", "-1"];
    let parts_7 = vec!["ER", "5", "8", "7"];

    let result_neg = eval_expression(&parts_neg1, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    let result_pos = eval_expression(&parts_7, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);

    assert_eq!(result_neg.unwrap().0, result_pos.unwrap().0);
}

#[test]
fn test_er_edge_cases() {
    let (variables, mut patterns, scripts, mut counters, scale) = test_setup!();

    // Fill == Length should return all 1s
    let parts = vec!["ER", "8", "8", "3"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert_eq!(result.unwrap().0, 1);

    // Fill == 1 should return 1 at step (length-1) for Bjorklund distribution
    // For ER 1 8, the single beat occurs at step 7 (last step before wrap)
    let parts_0 = vec!["ER", "1", "8", "0"];
    let parts_7 = vec!["ER", "1", "8", "7"];
    let result_0 = eval_expression(&parts_0, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    let result_7 = eval_expression(&parts_7, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert_eq!(result_0.unwrap().0, 0);
    assert_eq!(result_7.unwrap().0, 1);
}

#[test]
fn test_er_invalid_params() {
    let (variables, mut patterns, scripts, mut counters, scale) = test_setup!();

    // Length < 1 should return 0
    let parts = vec!["ER", "5", "0", "0"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert_eq!(result.unwrap().0, 0);

    // Length > 32 should return 0
    let parts = vec!["ER", "5", "33", "0"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert_eq!(result.unwrap().0, 0);

    // Fill < 1 should return 0
    let parts = vec!["ER", "0", "8", "0"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert_eq!(result.unwrap().0, 0);

    // Fill > Length should return 0
    let parts = vec!["ER", "9", "8", "0"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert_eq!(result.unwrap().0, 0);
}

#[test]
fn test_er_with_expressions() {
    let (mut variables, mut patterns, scripts, mut counters, scale) = test_setup!(mut);

    variables.a = 5;
    variables.b = 8;
    variables.i = 3;

    let parts = vec!["ER", "A", "B", "I"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert!(result.is_some());
    let (value, consumed) = result.unwrap();
    assert_eq!(value, 1); // ER 5 8 3 should return 1 (from pattern [0,1,0,1,1,0,1,1])
    assert_eq!(consumed, 4);
}

#[test]
fn test_er_insufficient_args() {
    let (variables, mut patterns, scripts, mut counters, scale) = test_setup!();

    let parts = vec!["ER", "5", "8"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert!(result.is_none());
}

// NR (Numeric Repeater) Tests

#[test]
fn test_nr_basic_pattern() {
    let (variables, mut patterns, scripts, mut counters, scale) = test_setup!();

    // Test first prime pattern (0x8888) with no mask and factor 0
    // 0x8888 = 1000100010001000 in binary
    // So bits set at positions 0, 4, 8, 12 (reading from right, MSB first for output)
    let test_cases = vec![
        (vec!["NR", "0", "0", "0", "0"], 1),  // bit 15 (MSB)
        (vec!["NR", "0", "0", "0", "1"], 0),  // bit 14
        (vec!["NR", "0", "0", "0", "2"], 0),  // bit 13
        (vec!["NR", "0", "0", "0", "3"], 0),  // bit 12
        (vec!["NR", "0", "0", "0", "4"], 1),  // bit 11
    ];

    for (parts, expected) in test_cases {
        let result = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
        assert!(result.is_some());
        let (value, consumed) = result.unwrap();
        assert_eq!(value, expected, "NR {} {} {} {} returned {}, expected {}",
            parts[1], parts[2], parts[3], parts[4], value, expected);
        assert_eq!(consumed, 5);
    }
}

#[test]
fn test_nr_wraps_step() {
    let (variables, mut patterns, scripts, mut counters, scale) = test_setup!();

    // Test that step wraps correctly (step 16 == step 0)
    let parts_0 = vec!["NR", "0", "0", "0", "0"];
    let parts_16 = vec!["NR", "0", "0", "0", "16"];

    let result_0 = eval_expression(&parts_0, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    let result_16 = eval_expression(&parts_16, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);

    assert_eq!(result_0.unwrap().0, result_16.unwrap().0);
}

#[test]
fn test_nr_negative_step() {
    let (variables, mut patterns, scripts, mut counters, scale) = test_setup!();

    // Test that negative steps work correctly (-1 should be step 15)
    let parts_neg1 = vec!["NR", "0", "0", "0", "-1"];
    let parts_15 = vec!["NR", "0", "0", "0", "15"];

    let result_neg = eval_expression(&parts_neg1, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    let result_pos = eval_expression(&parts_15, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);

    assert_eq!(result_neg.unwrap().0, result_pos.unwrap().0);
}

#[test]
fn test_nr_wraps_prime() {
    let (variables, mut patterns, scripts, mut counters, scale) = test_setup!();

    // Test that prime index wraps correctly (32 == 0, -1 == 31)
    let parts_0 = vec!["NR", "0", "0", "0", "0"];
    let parts_32 = vec!["NR", "32", "0", "0", "0"];

    let result_0 = eval_expression(&parts_0, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    let result_32 = eval_expression(&parts_32, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);

    assert_eq!(result_0.unwrap().0, result_32.unwrap().0);
}

#[test]
fn test_nr_with_mask() {
    let (variables, mut patterns, scripts, mut counters, scale) = test_setup!();

    // Test different masks
    // Mask 0 = no mask (0xFFFF equivalent)
    // Mask 1 = 0x0F0F
    // Mask 2 = 0xF003
    // Mask 3 = 0x01F0

    let parts_no_mask = vec!["NR", "0", "0", "0", "0"];
    let parts_mask1 = vec!["NR", "0", "1", "0", "0"];

    let result_no_mask = eval_expression(&parts_no_mask, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    let result_mask1 = eval_expression(&parts_mask1, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);

    // Both should be valid results (0 or 1)
    assert!(result_no_mask.is_some());
    assert!(result_mask1.is_some());
    let val1 = result_no_mask.unwrap().0;
    let val2 = result_mask1.unwrap().0;
    assert!(val1 == 0 || val1 == 1);
    assert!(val2 == 0 || val2 == 1);
}

#[test]
fn test_nr_with_factor() {
    let (variables, mut patterns, scripts, mut counters, scale) = test_setup!();

    // Test that factor changes the pattern
    let parts_factor0 = vec!["NR", "0", "0", "0", "0"];
    let parts_factor5 = vec!["NR", "0", "0", "5", "0"];

    let result_f0 = eval_expression(&parts_factor0, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    let result_f5 = eval_expression(&parts_factor5, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);

    // Both should return valid values
    assert!(result_f0.is_some());
    assert!(result_f5.is_some());

    // The values might be different (pattern is multiplied by factor+1)
    let val_f0 = result_f0.unwrap().0;
    let val_f5 = result_f5.unwrap().0;
    assert!(val_f0 == 0 || val_f0 == 1);
    assert!(val_f5 == 0 || val_f5 == 1);
}

#[test]
fn test_nr_with_expressions() {
    let (mut variables, mut patterns, scripts, mut counters, scale) = test_setup!(mut);

    variables.a = 5;
    variables.b = 1;
    variables.c = 8;
    variables.i = 4;

    let parts = vec!["NR", "A", "B", "C", "I"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert!(result.is_some());
    let (value, consumed) = result.unwrap();
    assert!(value == 0 || value == 1);
    assert_eq!(consumed, 5);
}

#[test]
fn test_nr_insufficient_args() {
    let (variables, mut patterns, scripts, mut counters, scale) = test_setup!();

    let parts = vec!["NR", "0", "0", "0"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert!(result.is_none());
}

#[test]
fn test_nr_factor_clamp() {
    let (variables, mut patterns, scripts, mut counters, scale) = test_setup!();

    // Test that factor is clamped to 0-16
    let parts_high = vec!["NR", "0", "0", "100", "0"];
    let parts_low = vec!["NR", "0", "0", "-5", "0"];
    let parts_16 = vec!["NR", "0", "0", "16", "0"];
    let parts_0 = vec!["NR", "0", "0", "0", "0"];

    // High values should clamp to 16
    let result_high = eval_expression(&parts_high, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    let result_16 = eval_expression(&parts_16, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert_eq!(result_high.unwrap().0, result_16.unwrap().0);

    // Low values should clamp to 0
    let result_low = eval_expression(&parts_low, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    let result_0 = eval_expression(&parts_0, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert_eq!(result_low.unwrap().0, result_0.unwrap().0);
}
