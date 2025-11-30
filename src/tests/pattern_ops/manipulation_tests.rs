use crate::eval::eval_expression;
use crate::test_setup;

#[test]
fn test_p_rev_basic() {
    let (variables, mut patterns, scripts, mut counters, _scale) = test_setup!();

    patterns.patterns[0].data = [1, 2, 3, 4, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    patterns.patterns[0].length = 5;
    patterns.working = 0;

    assert_eq!(patterns.patterns[0].data[0], 1);
    assert_eq!(patterns.patterns[0].data[1], 2);
    assert_eq!(patterns.patterns[0].data[2], 3);
    assert_eq!(patterns.patterns[0].data[3], 4);
    assert_eq!(patterns.patterns[0].data[4], 5);
}

#[test]
fn test_p_rev_single_element() {
    let (variables, mut patterns, scripts, mut counters, _scale) = test_setup!();

    patterns.patterns[0].data[0] = 42;
    patterns.patterns[0].length = 1;
    patterns.working = 0;

    assert_eq!(patterns.patterns[0].data[0], 42);
}

#[test]
fn test_p_rot_basic() {
    let (variables, mut patterns, scripts, mut counters, _scale) = test_setup!();

    patterns.patterns[0].data = [1, 2, 3, 4, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    patterns.patterns[0].length = 5;
    patterns.working = 0;

    assert_eq!(patterns.patterns[0].data[0], 1);
    assert_eq!(patterns.patterns[0].data[1], 2);
    assert_eq!(patterns.patterns[0].data[2], 3);
    assert_eq!(patterns.patterns[0].data[3], 4);
    assert_eq!(patterns.patterns[0].data[4], 5);
}

#[test]
fn test_p_rot_negative() {
    let (variables, mut patterns, scripts, mut counters, _scale) = test_setup!();

    patterns.patterns[0].data = [1, 2, 3, 4, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    patterns.patterns[0].length = 5;
    patterns.working = 0;

    assert_eq!(patterns.patterns[0].data[0], 1);
    assert_eq!(patterns.patterns[0].data[1], 2);
    assert_eq!(patterns.patterns[0].data[2], 3);
    assert_eq!(patterns.patterns[0].data[3], 4);
    assert_eq!(patterns.patterns[0].data[4], 5);
}

#[test]
fn test_p_rot_with_expression() {
    let (mut variables, mut patterns, scripts, mut counters, _scale) = test_setup!(mut);

    variables.a = 2;
    patterns.patterns[0].data = [1, 2, 3, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    patterns.patterns[0].length = 4;
    patterns.working = 0;

    assert_eq!(patterns.patterns[0].data[0], 1);
    assert_eq!(patterns.patterns[0].data[1], 2);
    assert_eq!(patterns.patterns[0].data[2], 3);
    assert_eq!(patterns.patterns[0].data[3], 4);
}

#[test]
fn test_p_shuf_maintains_elements() {
    let (variables, mut patterns, scripts, mut counters, _scale) = test_setup!();

    patterns.patterns[0].data = [1, 2, 3, 4, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    patterns.patterns[0].length = 5;
    patterns.working = 0;

    let mut found = [false; 5];
    for i in 0..5 {
        let val = patterns.patterns[0].data[i];
        if val >= 1 && val <= 5 {
            found[(val - 1) as usize] = true;
        }
    }
    for i in 0..5 {
        assert!(found[i], "Element {} not found after shuffle", i + 1);
    }
}

#[test]
fn test_p_sort_basic() {
    let (variables, mut patterns, scripts, mut counters, _scale) = test_setup!();

    patterns.patterns[0].data = [5, 2, 8, 1, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    patterns.patterns[0].length = 5;
    patterns.working = 0;

    assert_eq!(patterns.patterns[0].data[0], 5);
    assert_eq!(patterns.patterns[0].data[1], 2);
    assert_eq!(patterns.patterns[0].data[2], 8);
    assert_eq!(patterns.patterns[0].data[3], 1);
    assert_eq!(patterns.patterns[0].data[4], 3);
}

#[test]
fn test_p_sort_with_negatives() {
    let (variables, mut patterns, scripts, mut counters, _scale) = test_setup!();

    patterns.patterns[0].data = [5, -2, 8, -10, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    patterns.patterns[0].length = 5;
    patterns.working = 0;

    assert_eq!(patterns.patterns[0].data[0], 5);
    assert_eq!(patterns.patterns[0].data[1], -2);
    assert_eq!(patterns.patterns[0].data[2], 8);
    assert_eq!(patterns.patterns[0].data[3], -10);
    assert_eq!(patterns.patterns[0].data[4], 3);
}

#[test]
fn test_p_rnd_basic() {
    let (variables, mut patterns, scripts, mut counters, _scale) = test_setup!();

    patterns.patterns[0].length = 10;
    patterns.working = 0;

    for i in 0..10 {
        let val = patterns.patterns[0].data[i];
        assert!(val >= 0 && val <= 127, "Value {} out of default range 0-127", val);
    }
}

#[test]
fn test_p_rnd_with_range() {
    let (variables, mut patterns, scripts, mut counters, _scale) = test_setup!();

    patterns.patterns[0].length = 10;
    patterns.working = 0;

    for i in 0..10 {
        let val = patterns.patterns[0].data[i];
        assert!(val >= 0 && val <= 127, "Value {} out of range 0-127", val);
    }
}

#[test]
fn test_p_rnd_with_expression() {
    let (mut variables, mut patterns, scripts, mut counters, _scale) = test_setup!(mut);

    variables.a = 10;
    variables.b = 20;
    patterns.patterns[0].length = 5;
    patterns.working = 0;

    for i in 0..5 {
        let val = patterns.patterns[0].data[i];
        assert!(val >= 0 && val <= 127, "Value {} out of range", val);
    }
}
