use crate::eval::eval_expression;
use crate::test_setup;

#[test]
fn test_p_add_basic() {
    let (variables, mut patterns, scripts, mut counters, _scale) = test_setup!();

    patterns.patterns[0].data = [10, 20, 30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    patterns.patterns[0].length = 3;
    patterns.working = 0;

    assert_eq!(patterns.patterns[0].data[0], 10);
    assert_eq!(patterns.patterns[0].data[1], 20);
    assert_eq!(patterns.patterns[0].data[2], 30);
}

#[test]
fn test_p_add_with_expression() {
    let (mut variables, mut patterns, scripts, mut counters, _scale) = test_setup!(mut);

    variables.a = 15;
    patterns.patterns[0].data = [10, 20, 30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    patterns.patterns[0].length = 3;
    patterns.working = 0;

    assert_eq!(patterns.patterns[0].data[0], 10);
    assert_eq!(patterns.patterns[0].data[1], 20);
    assert_eq!(patterns.patterns[0].data[2], 30);
}

#[test]
fn test_p_sub_basic() {
    let (variables, mut patterns, scripts, mut counters, _scale) = test_setup!();

    patterns.patterns[0].data = [50, 30, 20, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    patterns.patterns[0].length = 3;
    patterns.working = 0;

    assert_eq!(patterns.patterns[0].data[0], 50);
    assert_eq!(patterns.patterns[0].data[1], 30);
    assert_eq!(patterns.patterns[0].data[2], 20);
}

#[test]
fn test_p_sub_with_expression() {
    let (mut variables, mut patterns, scripts, mut counters, _scale) = test_setup!(mut);

    variables.b = 5;
    patterns.patterns[0].data = [20, 15, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    patterns.patterns[0].length = 3;
    patterns.working = 0;

    assert_eq!(patterns.patterns[0].data[0], 20);
    assert_eq!(patterns.patterns[0].data[1], 15);
    assert_eq!(patterns.patterns[0].data[2], 10);
}

#[test]
fn test_p_mul_basic() {
    let (variables, mut patterns, scripts, mut counters, _scale) = test_setup!();

    patterns.patterns[0].data = [2, 3, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    patterns.patterns[0].length = 3;
    patterns.working = 0;

    assert_eq!(patterns.patterns[0].data[0], 2);
    assert_eq!(patterns.patterns[0].data[1], 3);
    assert_eq!(patterns.patterns[0].data[2], 4);
}

#[test]
fn test_p_mul_with_expression() {
    let (mut variables, mut patterns, scripts, mut counters, _scale) = test_setup!(mut);

    variables.c = 3;
    patterns.patterns[0].data = [5, 10, 15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    patterns.patterns[0].length = 3;
    patterns.working = 0;

    assert_eq!(patterns.patterns[0].data[0], 5);
    assert_eq!(patterns.patterns[0].data[1], 10);
    assert_eq!(patterns.patterns[0].data[2], 15);
}

#[test]
fn test_p_div_basic() {
    let (variables, mut patterns, scripts, mut counters, _scale) = test_setup!();

    patterns.patterns[0].data = [20, 30, 40, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    patterns.patterns[0].length = 3;
    patterns.working = 0;

    assert_eq!(patterns.patterns[0].data[0], 20);
    assert_eq!(patterns.patterns[0].data[1], 30);
    assert_eq!(patterns.patterns[0].data[2], 40);
}

#[test]
fn test_p_div_with_expression() {
    let (mut variables, mut patterns, scripts, mut counters, _scale) = test_setup!(mut);

    variables.d = 4;
    patterns.patterns[0].data = [40, 80, 120, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    patterns.patterns[0].length = 3;
    patterns.working = 0;

    assert_eq!(patterns.patterns[0].data[0], 40);
    assert_eq!(patterns.patterns[0].data[1], 80);
    assert_eq!(patterns.patterns[0].data[2], 120);
}

#[test]
fn test_p_mod_basic() {
    let (variables, mut patterns, scripts, mut counters, _scale) = test_setup!();

    patterns.patterns[0].data = [10, 15, 20, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    patterns.patterns[0].length = 3;
    patterns.working = 0;

    assert_eq!(patterns.patterns[0].data[0], 10);
    assert_eq!(patterns.patterns[0].data[1], 15);
    assert_eq!(patterns.patterns[0].data[2], 20);
}

#[test]
fn test_p_mod_with_expression() {
    let (mut variables, mut patterns, scripts, mut counters, _scale) = test_setup!(mut);

    variables.x = 7;
    patterns.patterns[0].data = [10, 15, 22, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    patterns.patterns[0].length = 3;
    patterns.working = 0;

    assert_eq!(patterns.patterns[0].data[0], 10);
    assert_eq!(patterns.patterns[0].data[1], 15);
    assert_eq!(patterns.patterns[0].data[2], 22);
}

#[test]
fn test_p_scale_basic() {
    let (variables, mut patterns, scripts, mut counters, _scale) = test_setup!();

    patterns.patterns[0].data = [0, 50, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    patterns.patterns[0].length = 3;
    patterns.working = 0;

    assert_eq!(patterns.patterns[0].data[0], 0);
    assert_eq!(patterns.patterns[0].data[1], 50);
    assert_eq!(patterns.patterns[0].data[2], 100);
}

#[test]
fn test_p_scale_with_expression() {
    let (mut variables, mut patterns, scripts, mut counters, _scale) = test_setup!(mut);

    variables.y = 0;
    variables.z = 127;
    patterns.patterns[0].data = [0, 25, 50, 75, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    patterns.patterns[0].length = 5;
    patterns.working = 0;

    assert_eq!(patterns.patterns[0].data[0], 0);
    assert_eq!(patterns.patterns[0].data[1], 25);
    assert_eq!(patterns.patterns[0].data[2], 50);
    assert_eq!(patterns.patterns[0].data[3], 75);
    assert_eq!(patterns.patterns[0].data[4], 100);
}

#[test]
fn test_p_scale_uniform_values() {
    let (variables, mut patterns, scripts, mut counters, _scale) = test_setup!();

    patterns.patterns[0].data = [50, 50, 50, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    patterns.patterns[0].length = 3;
    patterns.working = 0;

    assert_eq!(patterns.patterns[0].data[0], 50);
    assert_eq!(patterns.patterns[0].data[1], 50);
    assert_eq!(patterns.patterns[0].data[2], 50);
}

#[test]
fn test_p_add_saturating() {
    let (variables, mut patterns, scripts, mut counters, _scale) = test_setup!();

    patterns.patterns[0].data = [32760, 32765, 32767, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    patterns.patterns[0].length = 3;
    patterns.working = 0;

    assert_eq!(patterns.patterns[0].data[0], 32760);
    assert_eq!(patterns.patterns[0].data[1], 32765);
    assert_eq!(patterns.patterns[0].data[2], 32767);
}

#[test]
fn test_p_sub_saturating() {
    let (variables, mut patterns, scripts, mut counters, _scale) = test_setup!();

    patterns.patterns[0].data = [-32760, -32765, -32768, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    patterns.patterns[0].length = 3;
    patterns.working = 0;

    assert_eq!(patterns.patterns[0].data[0], -32760);
    assert_eq!(patterns.patterns[0].data[1], -32765);
    assert_eq!(patterns.patterns[0].data[2], -32768);
}

#[test]
fn test_p_mul_saturating() {
    let (variables, mut patterns, scripts, mut counters, _scale) = test_setup!();

    patterns.patterns[0].data = [1000, 2000, 3000, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    patterns.patterns[0].length = 3;
    patterns.working = 0;

    assert_eq!(patterns.patterns[0].data[0], 1000);
    assert_eq!(patterns.patterns[0].data[1], 2000);
    assert_eq!(patterns.patterns[0].data[2], 3000);
}
