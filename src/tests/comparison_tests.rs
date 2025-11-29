use crate::eval::eval_expression;
use super::common::{create_test_variables, create_test_patterns, create_test_scripts};

#[test]
fn test_ez_equals_zero() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();

    let parts = vec!["EZ", "0"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert_eq!(result, Some((1, 2)));

    let parts = vec!["EZ", "5"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert_eq!(result, Some((0, 2)));
}

#[test]
fn test_nz_not_zero() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();

    let parts = vec!["NZ", "0"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert_eq!(result, Some((0, 2)));

    let parts = vec!["NZ", "5"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert_eq!(result, Some((1, 2)));

    let parts = vec!["NZ", "-5"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert_eq!(result, Some((1, 2)));
}

#[test]
fn test_eq_equals() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();

    let parts = vec!["EQ", "5", "5"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert_eq!(result, Some((1, 3)));

    let parts = vec!["EQ", "5", "3"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert_eq!(result, Some((0, 3)));
}

#[test]
fn test_ne_not_equals() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();

    let parts = vec!["NE", "5", "3"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert_eq!(result, Some((1, 3)));

    let parts = vec!["NE", "5", "5"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert_eq!(result, Some((0, 3)));
}

#[test]
fn test_gt_greater_than() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();

    let parts = vec!["GT", "5", "3"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert_eq!(result, Some((1, 3)));

    let parts = vec!["GT", "3", "5"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert_eq!(result, Some((0, 3)));

    let parts = vec!["GT", "5", "5"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert_eq!(result, Some((0, 3)));
}

#[test]
fn test_lt_less_than() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();

    let parts = vec!["LT", "3", "5"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert_eq!(result, Some((1, 3)));

    let parts = vec!["LT", "5", "3"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert_eq!(result, Some((0, 3)));
}

#[test]
fn test_gte_greater_than_or_equal() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();

    let parts = vec!["GTE", "5", "3"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert_eq!(result, Some((1, 3)));

    let parts = vec!["GTE", "5", "5"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert_eq!(result, Some((1, 3)));

    let parts = vec!["GTE", "3", "5"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert_eq!(result, Some((0, 3)));
}

#[test]
fn test_lte_less_than_or_equal() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();

    let parts = vec!["LTE", "3", "5"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert_eq!(result, Some((1, 3)));

    let parts = vec!["LTE", "5", "5"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert_eq!(result, Some((1, 3)));

    let parts = vec!["LTE", "5", "3"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert_eq!(result, Some((0, 3)));
}

#[test]
fn test_comparison_with_variables() {
    let mut variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    variables.a = 10;
    variables.b = 5;

    let parts = vec!["GT", "A", "B"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert_eq!(result, Some((1, 3)));

    let parts = vec!["EZ", "A"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert_eq!(result, Some((0, 2)));
}

#[test]
fn test_comparison_with_pattern_ops() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    patterns.patterns[0].data[0] = 100;
    patterns.patterns[0].data[1] = 0;

    let parts = vec!["NZ", "PN.HERE", "0"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert_eq!(result, Some((1, 3)));

    patterns.patterns[0].index = 1;

    let parts = vec!["NZ", "PN.HERE", "0"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert_eq!(result, Some((0, 3)));

    let parts = vec!["EZ", "PN.HERE", "0"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert_eq!(result, Some((1, 3)));
}
