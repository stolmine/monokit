use crate::eval::eval_expression;
use super::common::{create_test_variables, create_test_patterns, create_test_scripts};

#[test]
fn test_rnd_returns_value_in_range() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let parts = vec!["RND", "100"];

    for _ in 0..20 {
        let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
        assert!(result.is_some());
        let (value, consumed) = result.unwrap();
        assert!(value >= 0 && value <= 100, "RND 100 returned {}", value);
        assert_eq!(consumed, 2);
    }
}

#[test]
fn test_rnd_with_zero_returns_zero() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let parts = vec!["RND", "0"];

    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert!(result.is_some());
    let (value, consumed) = result.unwrap();
    assert_eq!(value, 0);
    assert_eq!(consumed, 2);
}

#[test]
fn test_rnd_with_negative_returns_zero() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let parts = vec!["RND", "-10"];

    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert!(result.is_some());
    let (value, consumed) = result.unwrap();
    assert_eq!(value, 0);
    assert_eq!(consumed, 2);
}

#[test]
fn test_rnd_with_different_ranges() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();

    let test_cases = vec![
        (vec!["RND", "1"], 0, 1),
        (vec!["RND", "10"], 0, 10),
        (vec!["RND", "1000"], 0, 1000),
    ];

    for (parts, min, max) in test_cases {
        for _ in 0..10 {
            let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
            assert!(result.is_some());
            let (value, _) = result.unwrap();
            assert!(value >= min && value <= max,
                "RND {} returned {} (expected {} <= value <= {})",
                parts[1], value, min, max);
        }
    }
}

#[test]
fn test_rnd_with_mul_expression() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();

    let parts = vec!["RND", "MUL", "10", "10"];
    for _ in 0..20 {
        let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
        assert!(result.is_some());
        let (value, consumed) = result.unwrap();
        assert!(value >= 0 && value <= 100);
        assert_eq!(consumed, 4);
    }
}

#[test]
fn test_rnd_with_expression_argument() {
    let mut variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();

    variables.a = 10;

    let parts = vec!["RND", "ADD", "A", "10"];
    for _ in 0..20 {
        let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
        assert!(result.is_some());
        let (value, _) = result.unwrap();
        assert!(value >= 0 && value <= 20);
    }
}

#[test]
fn test_rnd_insufficient_args() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();

    let parts = vec!["RND"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert!(result.is_none());
}

#[test]
fn test_rrnd_returns_value_in_range() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let parts = vec!["RRND", "50", "100"];

    for _ in 0..20 {
        let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
        assert!(result.is_some());
        let (value, consumed) = result.unwrap();
        assert!(value >= 50 && value <= 100, "RRND 50 100 returned {}", value);
        assert_eq!(consumed, 3);
    }
}

#[test]
fn test_rrnd_with_reversed_range() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let parts = vec!["RRND", "100", "50"];

    for _ in 0..20 {
        let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
        assert!(result.is_some());
        let (value, consumed) = result.unwrap();
        assert!(value >= 50 && value <= 100, "RRND 100 50 returned {}", value);
        assert_eq!(consumed, 3);
    }
}

#[test]
fn test_rrnd_with_same_min_max() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let parts = vec!["RRND", "42", "42"];

    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert!(result.is_some());
    let (value, consumed) = result.unwrap();
    assert_eq!(value, 42);
    assert_eq!(consumed, 3);
}

#[test]
fn test_rrnd_edge_cases() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();

    let parts = vec!["RRND", "-100", "-50"];
    for _ in 0..10 {
        let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
        assert!(result.is_some());
        let (value, _) = result.unwrap();
        assert!(value >= -100 && value <= -50);
    }

    let parts = vec!["RRND", "0", "0"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert_eq!(result.unwrap().0, 0);
}

#[test]
fn test_rrnd_with_add_expressions() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();

    let parts = vec!["RRND", "ADD", "0", "50", "ADD", "50", "50"];
    for _ in 0..20 {
        let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
        assert!(result.is_some());
        let (value, consumed) = result.unwrap();
        assert!(value >= 50 && value <= 100);
        assert_eq!(consumed, 7);
    }
}

#[test]
fn test_rrnd_insufficient_args() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();

    let parts = vec!["RRND", "50"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert!(result.is_none());
}
