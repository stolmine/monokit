use crate::eval::eval_expression;
use super::common::{create_test_variables, create_test_patterns, create_test_scripts};

#[test]
fn test_nested_math_add_two_pattern_values() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();

    patterns.patterns[0].data[0] = 10;
    patterns.patterns[0].data[1] = 20;
    patterns.patterns[0].length = 2;
    patterns.patterns[0].index = 0;

    let parts = vec!["ADD", "PN.HERE", "0", "PN.HERE", "0"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert!(result.is_some());
    let (value, consumed) = result.unwrap();
    assert_eq!(value, 20);
    assert_eq!(consumed, 5);
}

#[test]
fn test_nested_math_sub_variables() {
    let mut variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();

    variables.a = 50;
    variables.b = 20;

    let parts = vec!["SUB", "A", "B"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert!(result.is_some());
    let (value, consumed) = result.unwrap();
    assert_eq!(value, 30);
    assert_eq!(consumed, 3);
}

#[test]
fn test_nested_math_mul_rnd() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();

    let parts = vec!["MUL", "RND", "10", "5"];
    for _ in 0..20 {
        let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
        assert!(result.is_some());
        let (value, consumed) = result.unwrap();
        assert!(value >= 0 && value < 50);
        assert_eq!(consumed, 4);
    }
}

#[test]
fn test_nested_add_add() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();

    let parts = vec!["ADD", "ADD", "1", "2", "3"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert!(result.is_some());
    let (value, consumed) = result.unwrap();
    assert_eq!(value, 6);
    assert_eq!(consumed, 5);
}

#[test]
fn test_nested_mul_add() {
    let mut variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();

    variables.a = 5;

    let parts = vec!["MUL", "ADD", "A", "1", "2"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert!(result.is_some());
    let (value, consumed) = result.unwrap();
    assert_eq!(value, 12);
    assert_eq!(consumed, 5);
}

#[test]
fn test_div_with_nested_expressions() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();

    let parts = vec!["DIV", "MUL", "10", "5", "ADD", "2", "3"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert!(result.is_some());
    let (value, consumed) = result.unwrap();
    assert_eq!(value, 10);
    assert_eq!(consumed, 7);
}

#[test]
fn test_mod_with_nested_expressions() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();

    let parts = vec!["MOD", "ADD", "17", "3", "SUB", "10", "3"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert!(result.is_some());
    let (value, consumed) = result.unwrap();
    assert_eq!(value, 6);
    assert_eq!(consumed, 7);
}

#[test]
fn test_sub_with_pattern_values() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();

    patterns.patterns[0].data[0] = 100;
    patterns.patterns[0].data[1] = 30;
    patterns.patterns[0].length = 2;
    patterns.patterns[0].index = 0;

    patterns.patterns[1].data[0] = 30;
    patterns.patterns[1].length = 1;
    patterns.patterns[1].index = 0;

    let parts = vec!["SUB", "PN.HERE", "0", "PN.HERE", "1"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert!(result.is_some());
    let (value, consumed) = result.unwrap();
    assert_eq!(value, 70);
    assert_eq!(consumed, 5);
}

#[test]
fn test_all_math_operations() {
    let mut variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();

    variables.a = 10;
    variables.b = 5;

    let parts = vec!["ADD", "A", "B"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert_eq!(result.unwrap().0, 15);

    let parts = vec!["SUB", "A", "B"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert_eq!(result.unwrap().0, 5);

    let parts = vec!["MUL", "A", "B"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert_eq!(result.unwrap().0, 50);

    let parts = vec!["DIV", "A", "B"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert_eq!(result.unwrap().0, 2);

    let parts = vec!["MOD", "A", "3"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert_eq!(result.unwrap().0, 1);
}

#[test]
fn test_div_by_zero_returns_zero() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();

    let parts = vec!["DIV", "10", "0"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert_eq!(result.unwrap().0, 0);
}

#[test]
fn test_mod_by_zero_returns_zero() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();

    let parts = vec!["MOD", "10", "0"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert_eq!(result.unwrap().0, 0);
}

#[test]
fn test_symbol_add() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();

    let parts = vec!["+", "5", "3"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert_eq!(result.unwrap().0, 8);
}

#[test]
fn test_symbol_sub() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();

    let parts = vec!["-", "10", "3"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert_eq!(result.unwrap().0, 7);
}

#[test]
fn test_symbol_mul() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();

    let parts = vec!["*", "4", "3"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert_eq!(result.unwrap().0, 12);
}

#[test]
fn test_symbol_div() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();

    let parts = vec!["/", "15", "3"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert_eq!(result.unwrap().0, 5);
}

#[test]
fn test_symbol_mod() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();

    let parts = vec!["%", "17", "5"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert_eq!(result.unwrap().0, 2);
}

#[test]
fn test_symbol_operators_with_variables() {
    let mut variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();

    variables.a = 20;
    variables.b = 4;

    let parts = vec!["+", "A", "B"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert_eq!(result.unwrap().0, 24);

    let parts = vec!["-", "A", "B"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert_eq!(result.unwrap().0, 16);

    let parts = vec!["*", "A", "B"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert_eq!(result.unwrap().0, 80);

    let parts = vec!["/", "A", "B"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert_eq!(result.unwrap().0, 5);

    let parts = vec!["%", "A", "B"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert_eq!(result.unwrap().0, 0);
}

#[test]
fn test_nested_symbol_operators() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();

    let parts = vec!["+", "*", "2", "3", "4"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert_eq!(result.unwrap().0, 10);
}

#[test]
fn test_mixed_word_and_symbol_operators() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();

    let parts = vec!["ADD", "*", "2", "3", "4"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert_eq!(result.unwrap().0, 10);
}
