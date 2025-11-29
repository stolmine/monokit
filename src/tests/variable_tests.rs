use crate::eval::eval_expression;
use super::common::{create_test_variables, create_test_patterns, create_test_scripts};

#[test]
fn test_variable_setter_with_expression_add() {
    let mut variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();

    variables.a = 10;

    let parts = vec!["A", "+", "1", "A"];
    let value: i16 = if let Some((expr_val, _)) = eval_expression(&parts, 1, &variables, &mut patterns, &scripts, 0) {
        expr_val
    } else {
        parts[1].parse().unwrap()
    };

    assert_eq!(value, 11);
}

#[test]
fn test_variable_setter_with_rnd() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();

    let parts = vec!["J", "RND", "100"];
    for _ in 0..20 {
        let value: i16 = if let Some((expr_val, _)) = eval_expression(&parts, 1, &variables, &mut patterns, &scripts, 0) {
            expr_val
        } else {
            parts[1].parse().unwrap()
        };

        assert!(value >= 0 && value <= 100);
    }
}

#[test]
fn test_variable_setter_with_pattern_next() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();

    patterns.patterns[0].data[0] = 100;
    patterns.patterns[0].data[1] = 200;
    patterns.patterns[0].length = 2;
    patterns.patterns[0].index = 0;

    let parts = vec!["X", "PN.NEXT", "0"];
    let value: i16 = if let Some((expr_val, _)) = eval_expression(&parts, 1, &variables, &mut patterns, &scripts, 0) {
        expr_val
    } else {
        parts[1].parse().unwrap()
    };

    assert_eq!(value, 200);
    assert_eq!(patterns.patterns[0].index, 1);
}

#[test]
fn test_variable_setter_with_mul_expression() {
    let mut variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();

    variables.a = 5;

    let parts = vec!["B", "MUL", "A", "2"];
    let value: i16 = if let Some((expr_val, _)) = eval_expression(&parts, 1, &variables, &mut patterns, &scripts, 0) {
        expr_val
    } else {
        parts[1].parse().unwrap()
    };

    assert_eq!(value, 10);
}

#[test]
fn test_variable_setter_with_nested_expression() {
    let mut variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();

    variables.a = 10;
    variables.b = 5;

    let parts = vec!["C", "ADD", "MUL", "A", "2", "B"];
    let value: i16 = if let Some((expr_val, _)) = eval_expression(&parts, 1, &variables, &mut patterns, &scripts, 0) {
        expr_val
    } else {
        parts[1].parse().unwrap()
    };

    assert_eq!(value, 25);
}

#[test]
fn test_variable_setter_literal_still_works() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();

    let parts = vec!["A", "100"];
    let value: i16 = if let Some((expr_val, _)) = eval_expression(&parts, 1, &variables, &mut patterns, &scripts, 0) {
        expr_val
    } else {
        parts[1].parse().unwrap()
    };

    assert_eq!(value, 100);
}

#[test]
fn test_all_variables_support_expressions() {
    let mut variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();

    variables.a = 1;

    let test_cases = vec![
        (vec!["A", "ADD", "A", "1"], 2),
        (vec!["B", "ADD", "A", "2"], 3),
        (vec!["C", "ADD", "A", "3"], 4),
        (vec!["D", "ADD", "A", "4"], 5),
        (vec!["X", "ADD", "A", "5"], 6),
        (vec!["Y", "ADD", "A", "6"], 7),
        (vec!["Z", "ADD", "A", "7"], 8),
        (vec!["T", "ADD", "A", "8"], 9),
    ];

    for (parts, expected) in test_cases {
        let value: i16 = if let Some((expr_val, _)) = eval_expression(&parts, 1, &variables, &mut patterns, &scripts, 0) {
            expr_val
        } else {
            parts[1].parse().unwrap()
        };

        assert_eq!(value, expected);
    }
}

#[test]
fn test_j_k_variables_support_expressions() {
    let mut variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let mut scripts = create_test_scripts();

    scripts.scripts[0].j = 10;
    scripts.scripts[0].k = 20;

    let parts = vec!["J", "ADD", "J", "5"];
    let value: i16 = if let Some((expr_val, _)) = eval_expression(&parts, 1, &variables, &mut patterns, &scripts, 0) {
        expr_val
    } else {
        parts[1].parse().unwrap()
    };
    assert_eq!(value, 15);

    let parts = vec!["K", "MUL", "K", "2"];
    let value: i16 = if let Some((expr_val, _)) = eval_expression(&parts, 1, &variables, &mut patterns, &scripts, 0) {
        expr_val
    } else {
        parts[1].parse().unwrap()
    };
    assert_eq!(value, 40);
}
