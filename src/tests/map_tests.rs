use crate::eval::eval_expression;
use super::common::{create_test_variables, create_test_patterns, create_test_scripts};

#[test]
fn test_map_basic() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();

    let parts = vec!["MAP", "50", "0", "100", "0", "1000"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert!(result.is_some());
    let (value, consumed) = result.unwrap();
    assert_eq!(value, 500);
    assert_eq!(consumed, 6);
}

#[test]
fn test_map_clamping_above() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();

    let parts = vec!["MAP", "150", "0", "100", "0", "1000"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert!(result.is_some());
    let (value, _) = result.unwrap();
    assert_eq!(value, 1000);
}

#[test]
fn test_map_clamping_below() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();

    let parts = vec!["MAP", "-50", "0", "100", "0", "1000"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert!(result.is_some());
    let (value, _) = result.unwrap();
    assert_eq!(value, 0);
}

#[test]
fn test_map_reverse_range() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();

    let parts = vec!["MAP", "50", "0", "100", "1000", "0"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert!(result.is_some());
    let (value, _) = result.unwrap();
    assert_eq!(value, 500);
}

#[test]
fn test_map_example_freq() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();

    let parts = vec!["MAP", "64", "0", "127", "200", "2000"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert!(result.is_some());
    let (value, _) = result.unwrap();
    let expected = (200 + ((64_i32 * 1800) / 127)) as i16;
    assert_eq!(value, expected);
}

#[test]
fn test_map_example_dc() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();

    let parts = vec!["MAP", "50", "0", "100", "0", "16383"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert!(result.is_some());
    let (value, _) = result.unwrap();
    assert_eq!(value, 8191);
}

#[test]
fn test_map_with_variables() {
    let mut variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();

    variables.a = 75;
    variables.b = 0;
    variables.c = 100;

    let parts = vec!["MAP", "A", "B", "C", "0", "200"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert!(result.is_some());
    let (value, _) = result.unwrap();
    assert_eq!(value, 150);
}

#[test]
fn test_map_nested_expressions() {
    let mut variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();

    variables.a = 50;

    let parts = vec!["MAP", "ADD", "A", "50", "0", "100", "0", "1000"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert!(result.is_some());
    let (value, consumed) = result.unwrap();
    assert_eq!(value, 1000);
    assert_eq!(consumed, 8);
}

#[test]
fn test_map_with_pattern() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();

    patterns.patterns[0].data[0] = 64;
    patterns.patterns[0].length = 1;
    patterns.patterns[0].index = 0;

    let parts = vec!["MAP", "PN.HERE", "0", "0", "127", "200", "2000"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert!(result.is_some());
    let (value, _) = result.unwrap();
    let expected = (200 + ((64_i32 * 1800) / 127)) as i16;
    assert_eq!(value, expected);
}

#[test]
fn test_map_edge_case_same_input_range() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();

    let parts = vec!["MAP", "100", "50", "50", "0", "1000"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert!(result.is_some());
    let (value, _) = result.unwrap();
    assert_eq!(value, 0);
}

#[test]
fn test_map_negative_ranges() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();

    let parts = vec!["MAP", "0", "-100", "100", "-1000", "1000"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert!(result.is_some());
    let (value, _) = result.unwrap();
    assert_eq!(value, 0);
}

#[test]
fn test_map_in_variable_assignment() {
    let mut variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();

    variables.b = 25;

    let parts = vec!["MAP", "B", "0", "100", "500", "1500"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert!(result.is_some());
    let (value, _) = result.unwrap();
    assert_eq!(value, 750);
}

#[test]
fn test_map_all_expressions() {
    let mut variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();

    variables.a = 10;
    variables.b = 5;

    let parts = vec!["MAP", "ADD", "A", "B", "SUB", "A", "B", "MUL", "A", "2", "DIV", "100", "B", "100"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert!(result.is_some());
    let (value, consumed) = result.unwrap();
    assert_eq!(consumed, 14);
    let expected = 73;
    assert_eq!(value, expected);
}
