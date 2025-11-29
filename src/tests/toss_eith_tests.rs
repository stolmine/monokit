use crate::eval::eval_expression;
use super::common::{create_test_variables, create_test_patterns, create_test_scripts};

#[test]
fn test_toss_returns_zero_or_one() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let parts = vec!["TOSS"];

    for _ in 0..20 {
        let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
        assert!(result.is_some());
        let (value, consumed) = result.unwrap();
        assert!(value == 0 || value == 1, "TOSS returned {}", value);
        assert_eq!(consumed, 1);
    }
}

#[test]
fn test_toss_in_expression_context() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let parts = vec!["ADD", "TOSS", "10"];

    for _ in 0..20 {
        let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
        assert!(result.is_some());
        let (value, consumed) = result.unwrap();
        assert!(value == 10 || value == 11, "ADD TOSS 10 returned {}", value);
        assert_eq!(consumed, 3);
    }
}

#[test]
fn test_eith_returns_one_of_two_values() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let parts = vec!["EITH", "100", "200"];

    for _ in 0..20 {
        let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
        assert!(result.is_some());
        let (value, consumed) = result.unwrap();
        assert!(value == 100 || value == 200, "EITH 100 200 returned {}", value);
        assert_eq!(consumed, 3);
    }
}

#[test]
fn test_eith_with_expressions() {
    let mut variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();

    variables.a = 50;
    variables.b = 75;
    let parts = vec!["EITH", "A", "B"];

    for _ in 0..20 {
        let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
        assert!(result.is_some());
        let (value, consumed) = result.unwrap();
        assert!(value == 50 || value == 75, "EITH A B returned {}", value);
        assert_eq!(consumed, 3);
    }
}

#[test]
fn test_eith_in_expression_context() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let parts = vec!["MUL", "EITH", "2", "3", "5"];

    for _ in 0..20 {
        let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
        assert!(result.is_some());
        let (value, consumed) = result.unwrap();
        assert!(value == 10 || value == 15, "MUL (EITH 2 3) 5 returned {}", value);
        assert_eq!(consumed, 5);
    }
}
