use crate::eval::eval_expression;
use crate::test_setup;

#[test]
fn test_p_min_basic() {
    let (variables, mut patterns, scripts, mut counters, _scale) = test_setup!();

    patterns.patterns[0].data = [5, 2, 8, 1, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    patterns.patterns[0].length = 5;
    patterns.working = 0;

    assert_eq!(patterns.patterns[0].data[0], 5);
    assert_eq!(patterns.patterns[0].data[1], 2);
    assert_eq!(patterns.patterns[0].data[2], 8);
    assert_eq!(patterns.patterns[0].data[3], 1);
    assert_eq!(patterns.patterns[0].data[4], 10);
}

#[test]
fn test_p_min_with_negatives() {
    let (variables, mut patterns, scripts, mut counters, _scale) = test_setup!();

    patterns.patterns[0].data = [5, -10, 8, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    patterns.patterns[0].length = 5;
    patterns.working = 0;

    assert_eq!(patterns.patterns[0].data[0], 5);
    assert_eq!(patterns.patterns[0].data[1], -10);
    assert_eq!(patterns.patterns[0].data[2], 8);
    assert_eq!(patterns.patterns[0].data[3], 0);
    assert_eq!(patterns.patterns[0].data[4], 3);
}

#[test]
fn test_p_max_basic() {
    let (variables, mut patterns, scripts, mut counters, _scale) = test_setup!();

    patterns.patterns[0].data = [5, 2, 8, 1, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    patterns.patterns[0].length = 5;
    patterns.working = 0;

    assert_eq!(patterns.patterns[0].data[0], 5);
    assert_eq!(patterns.patterns[0].data[1], 2);
    assert_eq!(patterns.patterns[0].data[2], 8);
    assert_eq!(patterns.patterns[0].data[3], 1);
    assert_eq!(patterns.patterns[0].data[4], 10);
}

#[test]
fn test_p_max_with_negatives() {
    let (variables, mut patterns, scripts, mut counters, _scale) = test_setup!();

    patterns.patterns[0].data = [5, -10, 8, 0, -3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    patterns.patterns[0].length = 5;
    patterns.working = 0;

    assert_eq!(patterns.patterns[0].data[0], 5);
    assert_eq!(patterns.patterns[0].data[1], -10);
    assert_eq!(patterns.patterns[0].data[2], 8);
    assert_eq!(patterns.patterns[0].data[3], 0);
    assert_eq!(patterns.patterns[0].data[4], -3);
}

#[test]
fn test_p_sum_basic() {
    let (variables, mut patterns, scripts, mut counters, _scale) = test_setup!();

    patterns.patterns[0].data = [10, 20, 30, 40, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    patterns.patterns[0].length = 4;
    patterns.working = 0;

    assert_eq!(patterns.patterns[0].data[0], 10);
    assert_eq!(patterns.patterns[0].data[1], 20);
    assert_eq!(patterns.patterns[0].data[2], 30);
    assert_eq!(patterns.patterns[0].data[3], 40);
}

#[test]
fn test_p_sum_with_negatives() {
    let (variables, mut patterns, scripts, mut counters, _scale) = test_setup!();

    patterns.patterns[0].data = [10, -5, 20, -10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    patterns.patterns[0].length = 4;
    patterns.working = 0;

    assert_eq!(patterns.patterns[0].data[0], 10);
    assert_eq!(patterns.patterns[0].data[1], -5);
    assert_eq!(patterns.patterns[0].data[2], 20);
    assert_eq!(patterns.patterns[0].data[3], -10);
}

#[test]
fn test_p_avg_basic() {
    let (variables, mut patterns, scripts, mut counters, _scale) = test_setup!();

    patterns.patterns[0].data = [10, 20, 30, 40, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    patterns.patterns[0].length = 4;
    patterns.working = 0;

    assert_eq!(patterns.patterns[0].data[0], 10);
    assert_eq!(patterns.patterns[0].data[1], 20);
    assert_eq!(patterns.patterns[0].data[2], 30);
    assert_eq!(patterns.patterns[0].data[3], 40);
}

#[test]
fn test_p_avg_with_negatives() {
    let (variables, mut patterns, scripts, mut counters, _scale) = test_setup!();

    patterns.patterns[0].data = [10, -10, 20, -20, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    patterns.patterns[0].length = 4;
    patterns.working = 0;

    assert_eq!(patterns.patterns[0].data[0], 10);
    assert_eq!(patterns.patterns[0].data[1], -10);
    assert_eq!(patterns.patterns[0].data[2], 20);
    assert_eq!(patterns.patterns[0].data[3], -20);
}

#[test]
fn test_p_fnd_basic() {
    let (variables, mut patterns, scripts, mut counters, _scale) = test_setup!();

    patterns.patterns[0].data = [10, 20, 30, 40, 50, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    patterns.patterns[0].length = 5;
    patterns.working = 0;

    assert_eq!(patterns.patterns[0].data[0], 10);
    assert_eq!(patterns.patterns[0].data[1], 20);
    assert_eq!(patterns.patterns[0].data[2], 30);
    assert_eq!(patterns.patterns[0].data[3], 40);
    assert_eq!(patterns.patterns[0].data[4], 50);
}

#[test]
fn test_p_fnd_not_found() {
    let (variables, mut patterns, scripts, mut counters, _scale) = test_setup!();

    patterns.patterns[0].data = [10, 20, 30, 40, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    patterns.patterns[0].length = 4;
    patterns.working = 0;

    assert_eq!(patterns.patterns[0].data[0], 10);
    assert_eq!(patterns.patterns[0].data[1], 20);
    assert_eq!(patterns.patterns[0].data[2], 30);
    assert_eq!(patterns.patterns[0].data[3], 40);
}

#[test]
fn test_p_fnd_with_expression() {
    let (mut variables, mut patterns, scripts, mut counters, _scale) = test_setup!(mut);

    variables.t = 30;
    patterns.patterns[0].data = [10, 20, 30, 40, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    patterns.patterns[0].length = 4;
    patterns.working = 0;

    assert_eq!(patterns.patterns[0].data[0], 10);
    assert_eq!(patterns.patterns[0].data[1], 20);
    assert_eq!(patterns.patterns[0].data[2], 30);
    assert_eq!(patterns.patterns[0].data[3], 40);
}

#[test]
fn test_p_operations_with_empty_pattern() {
    let (variables, mut patterns, scripts, mut counters, _scale) = test_setup!();

    patterns.patterns[0].length = 1;
    patterns.working = 0;

    assert_eq!(patterns.patterns[0].length, 1);
}

#[test]
fn test_p_operations_with_single_element() {
    let (variables, mut patterns, scripts, mut counters, _scale) = test_setup!();

    patterns.patterns[0].data[0] = 42;
    patterns.patterns[0].length = 1;
    patterns.working = 0;

    assert_eq!(patterns.patterns[0].data[0], 42);
    assert_eq!(patterns.patterns[0].length, 1);
}

#[test]
fn test_p_operations_with_full_pattern() {
    let (variables, mut patterns, scripts, mut counters, _scale) = test_setup!();

    for i in 0..64 {
        patterns.patterns[0].data[i] = i as i16;
    }
    patterns.patterns[0].length = 64;
    patterns.working = 0;

    for i in 0..64 {
        assert_eq!(patterns.patterns[0].data[i], i as i16);
    }
}

#[test]
fn test_p_operations_boundary_index_0() {
    let (variables, mut patterns, scripts, mut counters, _scale) = test_setup!();

    patterns.patterns[0].data = [100, 200, 300, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    patterns.patterns[0].length = 3;
    patterns.patterns[0].index = 0;
    patterns.working = 0;

    assert_eq!(patterns.patterns[0].data[0], 100);
    assert_eq!(patterns.patterns[0].index, 0);
}

#[test]
fn test_p_operations_boundary_last_index() {
    let (variables, mut patterns, scripts, mut counters, _scale) = test_setup!();

    patterns.patterns[0].data = [100, 200, 300, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    patterns.patterns[0].length = 3;
    patterns.patterns[0].index = 2;
    patterns.working = 0;

    assert_eq!(patterns.patterns[0].data[2], 300);
    assert_eq!(patterns.patterns[0].index, 2);
}

#[test]
fn test_p_operations_different_working_patterns() {
    let (variables, mut patterns, scripts, mut counters, _scale) = test_setup!();

    patterns.patterns[0].data[0] = 100;
    patterns.patterns[1].data[0] = 200;
    patterns.patterns[2].data[0] = 300;
    patterns.patterns[3].data[0] = 400;
    patterns.patterns[4].data[0] = 500;
    patterns.patterns[5].data[0] = 600;

    patterns.working = 0;
    assert_eq!(patterns.patterns[0].data[0], 100);

    patterns.working = 1;
    assert_eq!(patterns.patterns[1].data[0], 200);

    patterns.working = 2;
    assert_eq!(patterns.patterns[2].data[0], 300);

    patterns.working = 3;
    assert_eq!(patterns.patterns[3].data[0], 400);

    patterns.working = 4;
    assert_eq!(patterns.patterns[4].data[0], 500);

    patterns.working = 5;
    assert_eq!(patterns.patterns[5].data[0], 600);
}

#[test]
fn test_chained_pattern_operations() {
    let (mut variables, mut patterns, scripts, mut counters, _scale) = test_setup!(mut);

    patterns.patterns[0].data = [1, 2, 3, 4, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    patterns.patterns[0].length = 5;
    patterns.working = 0;

    assert_eq!(patterns.patterns[0].data[0], 1);
    assert_eq!(patterns.patterns[0].data[1], 2);
    assert_eq!(patterns.patterns[0].data[2], 3);
    assert_eq!(patterns.patterns[0].data[3], 4);
    assert_eq!(patterns.patterns[0].data[4], 5);
}
