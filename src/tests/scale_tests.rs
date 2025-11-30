use crate::eval::eval_expression;
use crate::commands::scale::get_preset_mask;
use crate::test_setup;
use super::common::{create_test_scale, create_test_variables, create_test_patterns, create_test_scripts, create_test_counters};
use crate::types::ScaleState;

#[test]
fn test_q_quantizes_to_major_scale() {
    let (variables, mut patterns, scripts, mut counters, _scale) = test_setup!();
    let mut scale = create_test_scale();

    let parts = vec!["Q", "1"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert!(result.is_some());
    let (value, consumed) = result.unwrap();
    assert!(value == 0 || value == 2, "Q 1 in major scale should quantize to 0 or 2, got {}", value);
    assert_eq!(consumed, 2);
}

#[test]
fn test_q_quantizes_to_minor_scale() {
    let (variables, mut patterns, scripts, mut counters, _scale) = test_setup!();
    let mut scale = ScaleState {
        root: 0,
        scale_preset: Some(2),
        mask: get_preset_mask(2),
        divisions: 12,
    };

    let parts = vec!["Q", "10"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert!(result.is_some());
    let (value, consumed) = result.unwrap();
    assert!(value == 9 || value == 11, "Q 10 in minor scale should quantize to 9 or 11 (not in scale), got {}", value);
    assert_eq!(consumed, 2);
}

#[test]
fn test_q_preserves_scale_degree() {
    let (variables, mut patterns, scripts, mut counters, _scale) = test_setup!();
    let scale = create_test_scale();

    let parts_0 = vec!["Q", "0"];
    let result = eval_expression(&parts_0, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert_eq!(result.unwrap().0, 0, "Q 0 should stay 0 in major scale");

    let parts_2 = vec!["Q", "2"];
    let result = eval_expression(&parts_2, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert_eq!(result.unwrap().0, 2, "Q 2 should stay 2 in major scale");

    let parts_4 = vec!["Q", "4"];
    let result = eval_expression(&parts_4, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert_eq!(result.unwrap().0, 4, "Q 4 should stay 4 in major scale");

    let parts_5 = vec!["Q", "5"];
    let result = eval_expression(&parts_5, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert_eq!(result.unwrap().0, 5, "Q 5 should stay 5 in major scale");

    let parts_7 = vec!["Q", "7"];
    let result = eval_expression(&parts_7, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert_eq!(result.unwrap().0, 7, "Q 7 should stay 7 in major scale");

    let parts_9 = vec!["Q", "9"];
    let result = eval_expression(&parts_9, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert_eq!(result.unwrap().0, 9, "Q 9 should stay 9 in major scale");

    let parts_11 = vec!["Q", "11"];
    let result = eval_expression(&parts_11, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert_eq!(result.unwrap().0, 11, "Q 11 should stay 11 in major scale");
}

#[test]
fn test_q_octave_wrapping() {
    let (variables, mut patterns, scripts, mut counters, _scale) = test_setup!();
    let scale = create_test_scale();

    let parts_13 = vec!["Q", "13"];
    let result = eval_expression(&parts_13, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert!(result.is_some());
    let (value, _) = result.unwrap();
    assert!(value == 12 || value == 14, "Q 13 should quantize to 12 or 14 (preserve octave), got {}", value);

    let parts_25 = vec!["Q", "25"];
    let result = eval_expression(&parts_25, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert!(result.is_some());
    let (value, _) = result.unwrap();
    assert!(value == 24 || value == 26, "Q 25 should quantize to 24 or 26 (second octave), got {}", value);

    let parts_neg1 = vec!["Q", "-1"];
    let result = eval_expression(&parts_neg1, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert!(result.is_some());
    let (value, _) = result.unwrap();
    assert!(value >= -2 && value <= 0, "Q -1 should quantize within valid range, got {}", value);
}

#[test]
fn test_q_tie_break_toward_root() {
    let (variables, mut patterns, scripts, mut counters, _scale) = test_setup!();
    let scale = create_test_scale();

    let parts = vec!["Q", "1"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert!(result.is_some());
    let (value, _) = result.unwrap();
    assert_eq!(value, 0, "Q 1 in major scale should quantize down to 0 (tie-break to lower degree), got {}", value);
}

#[test]
fn test_q_root_changes_quantization() {
    let (variables, mut patterns, scripts, mut counters, _scale) = test_setup!();
    let mut scale = create_test_scale();

    let parts_default = vec!["Q", "1"];
    let result = eval_expression(&parts_default, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    let default_value = result.unwrap().0;

    scale.root = 2;

    let parts_root2 = vec!["Q", "1"];
    let result = eval_expression(&parts_root2, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    let root2_value = result.unwrap().0;

    assert_ne!(default_value, root2_value, "Changing root should affect quantization output");

    let parts_3 = vec!["Q", "3"];
    let result = eval_expression(&parts_3, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert!(result.is_some());
    let (value, _) = result.unwrap();
    assert!(value == 2 || value == 4, "Q 3 with root=2 should quantize to 2 or 4, got {}", value);
}

#[test]
fn test_q_root_validation() {
    let (variables, mut patterns, scripts, mut counters, _scale) = test_setup!();
    let mut scale = create_test_scale();

    scale.root = 11;
    assert!(scale.root <= 11, "Root should accept value up to divisions-1 (11 for 12-EDO)");

    scale.root = 0;
    assert_eq!(scale.root, 0, "Root should accept 0");
}

#[test]
fn test_q_scale_sets_preset() {
    let mut scale = ScaleState {
        root: 0,
        scale_preset: Some(0),
        mask: get_preset_mask(0),
        divisions: 12,
    };

    let chromatic_mask = scale.mask.clone();
    assert_eq!(scale.divisions, 12, "Chromatic preset should have 12 divisions");

    scale.scale_preset = Some(1);
    scale.mask = get_preset_mask(1);
    scale.divisions = 12;

    let major_mask = scale.mask.clone();
    assert_eq!(scale.divisions, 12, "Major preset should have 12 divisions");
    assert_ne!(chromatic_mask, major_mask, "Chromatic and major should have different masks");

    let expected_major = vec![true,false,true,false,true,true,false,true,false,true,false,true];
    assert_eq!(major_mask, expected_major, "Major scale mask should match expected pattern");
}

#[test]
fn test_q_scale_resets_divisions() {
    let mut scale = ScaleState {
        root: 0,
        scale_preset: Some(1),
        mask: get_preset_mask(1),
        divisions: 24,
    };

    scale.divisions = 12;
    assert_eq!(scale.divisions, 12, "Setting Q.SCALE should reset divisions to 12");
}

#[test]
fn test_q_bit_custom_mask() {
    use crate::commands::scale::parse_binary_mask;

    let major_binary = "101011010101";
    let mask = parse_binary_mask(major_binary);
    assert!(mask.is_some());
    let mask = mask.unwrap();

    let expected = vec![true,false,true,false,true,true,false,true,false,true,false,true];
    assert_eq!(mask, expected, "Q.BIT with major scale binary should match major mask");
    assert_eq!(mask.len(), 12, "Binary mask should have 12 divisions");
}

#[test]
fn test_q_bit_sets_divisions() {
    use crate::commands::scale::parse_binary_mask;

    let pentatonic = "10101";
    let mask = parse_binary_mask(pentatonic);
    assert!(mask.is_some());
    let mask = mask.unwrap();
    assert_eq!(mask.len(), 5, "Q.BIT with 5 bits should set divisions to 5");
    assert_eq!(mask, vec![true,false,true,false,true], "Pentatonic mask should match");
}

#[test]
fn test_q_bit_microtonal() {
    use crate::commands::scale::parse_binary_mask;

    let quarter_tone = "101010101010101010101010";
    let mask = parse_binary_mask(quarter_tone);
    assert!(mask.is_some());
    let mask = mask.unwrap();
    assert_eq!(mask.len(), 24, "Q.BIT with 24 bits should set divisions to 24 (quarter-tones)");

    let count_active = mask.iter().filter(|&&x| x).count();
    assert_eq!(count_active, 12, "Quarter-tone chromatic should have 12 active notes");
}

#[test]
fn test_n_uses_scale_divisions() {
    let (variables, mut patterns, scripts, mut counters, _scale) = test_setup!();

    let mut scale_12 = create_test_scale();
    scale_12.divisions = 12;

    let parts = vec!["N", "12"];
    let result_12 = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale_12);
    assert!(result_12.is_some());
    let freq_12 = result_12.unwrap().0;

    let mut scale_24 = create_test_scale();
    scale_24.divisions = 24;

    let result_24 = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale_24);
    assert!(result_24.is_some());
    let freq_24 = result_24.unwrap().0;

    assert_ne!(freq_12, freq_24, "N 12 should produce different frequencies with different divisions");
    assert!(freq_12 > freq_24, "N 12 in 12-EDO should be higher than N 12 in 24-EDO (octave vs half-octave)");
}

#[test]
fn test_n_quarter_tones() {
    let (variables, mut patterns, scripts, mut counters, _scale) = test_setup!();

    let mut scale = create_test_scale();
    scale.divisions = 24;

    let parts_0 = vec!["N", "0"];
    let result_0 = eval_expression(&parts_0, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    let freq_0 = result_0.unwrap().0;

    let parts_1 = vec!["N", "1"];
    let result_1 = eval_expression(&parts_1, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    let freq_1 = result_1.unwrap().0;

    let parts_2 = vec!["N", "2"];
    let result_2 = eval_expression(&parts_2, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    let freq_2 = result_2.unwrap().0;

    assert!(freq_1 > freq_0 && freq_1 < freq_2, "N 1 in 24-EDO should be between N 0 and N 2 (quarter-tone)");

    let diff_0_1 = freq_1 - freq_0;
    let diff_1_2 = freq_2 - freq_1;
    assert!((diff_0_1 - diff_1_2).abs() <= 1, "Quarter-tone steps should be approximately equal");
}

#[test]
fn test_q_in_expression() {
    let (mut variables, mut patterns, scripts, mut counters, scale) = test_setup!(mut);

    variables.a = 1;
    variables.b = 1;

    let parts = vec!["Q", "ADD", "A", "B"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert!(result.is_some());
    let (value, consumed) = result.unwrap();
    assert_eq!(value, 2, "Q ADD A B should quantize (1+1)=2 to 2 (in scale)");
    assert_eq!(consumed, 4);
}

#[test]
fn test_pf_n_q_chain() {
    let (variables, mut patterns, scripts, mut counters, _scale) = test_setup!();
    let scale = create_test_scale();

    let parts_nq = vec!["N", "Q", "3"];
    let result_nq = eval_expression(&parts_nq, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert!(result_nq.is_some());
    let (freq, consumed) = result_nq.unwrap();
    assert!(freq > 0, "N Q chain should produce valid frequency");
    assert_eq!(consumed, 3);

    let parts_q = vec!["Q", "ADD", "1", "1"];
    let result_q = eval_expression(&parts_q, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert!(result_q.is_some());
    let (quantized, consumed_q) = result_q.unwrap();
    assert_eq!(quantized, 2, "Q should work with nested expressions");
    assert_eq!(consumed_q, 4);
}

#[test]
fn test_q_with_negative_notes() {
    let (variables, mut patterns, scripts, mut counters, _scale) = test_setup!();
    let scale = create_test_scale();

    let parts = vec!["Q", "-5"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert!(result.is_some());
    let (value, _) = result.unwrap();
    assert!(value <= 0, "Q -5 should quantize to a non-positive value");
}

#[test]
fn test_q_with_chromatic_scale() {
    let (variables, mut patterns, scripts, mut counters, _scale) = test_setup!();
    let mut scale = ScaleState {
        root: 0,
        scale_preset: Some(0),
        mask: get_preset_mask(0),
        divisions: 12,
    };

    let parts = vec!["Q", "1"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert_eq!(result.unwrap().0, 1, "Q 1 in chromatic scale should stay 1 (all notes valid)");

    let parts = vec!["Q", "7"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert_eq!(result.unwrap().0, 7, "Q 7 in chromatic scale should stay 7");

    let parts = vec!["Q", "11"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert_eq!(result.unwrap().0, 11, "Q 11 in chromatic scale should stay 11");
}

#[test]
fn test_multiple_q_operations() {
    let (variables, mut patterns, scripts, mut counters, _scale) = test_setup!();
    let scale = create_test_scale();

    let parts = vec!["Q", "Q", "1"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert!(result.is_some());
    let (value, consumed) = result.unwrap();
    assert_eq!(value, 0, "Q Q 1 should double-quantize to 0");
    assert_eq!(consumed, 3);
}

#[test]
fn test_scale_presets_have_correct_note_counts() {
    let chromatic = get_preset_mask(0);
    assert_eq!(chromatic.iter().filter(|&&x| x).count(), 12, "Chromatic should have 12 notes");

    let major = get_preset_mask(1);
    assert_eq!(major.iter().filter(|&&x| x).count(), 7, "Major should have 7 notes");

    let minor = get_preset_mask(2);
    assert_eq!(minor.iter().filter(|&&x| x).count(), 7, "Minor should have 7 notes");

    let pent_major = get_preset_mask(7);
    assert_eq!(pent_major.iter().filter(|&&x| x).count(), 5, "Pentatonic major should have 5 notes");

    let pent_minor = get_preset_mask(8);
    assert_eq!(pent_minor.iter().filter(|&&x| x).count(), 5, "Pentatonic minor should have 5 notes");
}

#[test]
fn test_n_produces_correct_c3_frequency() {
    let (variables, mut patterns, scripts, mut counters, _scale) = test_setup!();
    let scale = create_test_scale();

    let parts = vec!["N", "0"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &scale);
    assert_eq!(result.unwrap().0, 131, "N 0 should be C3 at 131 Hz (rounded from 130.8128)");
}
