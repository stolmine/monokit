use crate::eval::{eval_condition, eval_expression, resolve_value};
use crate::types::{PatternStorage, ScriptStorage, Variables};

fn create_test_variables() -> Variables {
    Variables {
        a: 0,
        b: 0,
        c: 0,
        d: 0,
        i: 0,
        x: 0,
        y: 0,
        z: 0,
        t: 0,
    }
}

fn create_test_patterns() -> PatternStorage {
    PatternStorage::default()
}

fn create_test_scripts() -> ScriptStorage {
    ScriptStorage::default()
}

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
        assert!(value >= 0 && value < 100, "RND 100 returned {}", value);
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
fn test_eval_expression_variables() {
    let mut variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();

    variables.a = 10;
    variables.b = 20;
    variables.i = 5;
    variables.x = 100;

    let test_cases = vec![
        (vec!["A"], 10),
        (vec!["B"], 20),
        (vec!["I"], 5),
        (vec!["X"], 100),
    ];

    for (parts, expected) in test_cases {
        let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
        assert!(result.is_some());
        let (value, consumed) = result.unwrap();
        assert_eq!(value, expected);
        assert_eq!(consumed, 1);
    }
}

#[test]
fn test_eval_expression_literal_numbers() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();

    let test_cases = vec![
        (vec!["0"], 0),
        (vec!["42"], 42),
        (vec!["-10"], -10),
        (vec!["999"], 999),
    ];

    for (parts, expected) in test_cases {
        let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
        assert!(result.is_some());
        let (value, consumed) = result.unwrap();
        assert_eq!(value, expected);
        assert_eq!(consumed, 1);
    }
}

#[test]
fn test_pattern_operations_with_expressions() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();

    patterns.patterns[0].data[0] = 100;
    patterns.patterns[0].data[1] = 200;
    patterns.patterns[0].data[2] = 300;
    patterns.patterns[0].length = 3;
    patterns.patterns[0].index = 0;

    let parts = vec!["PN", "0"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert!(result.is_some());
    let (value, consumed) = result.unwrap();
    assert_eq!(value, 100);
    assert_eq!(consumed, 2);

    let parts = vec!["PN.I", "0"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert!(result.is_some());
    let (value, consumed) = result.unwrap();
    assert_eq!(value, 0);
    assert_eq!(consumed, 2);

    let parts = vec!["PN.L", "0"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert!(result.is_some());
    let (value, consumed) = result.unwrap();
    assert_eq!(value, 3);
    assert_eq!(consumed, 2);
}

#[test]
fn test_pattern_next_operation() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();

    patterns.patterns[0].data[0] = 10;
    patterns.patterns[0].data[1] = 20;
    patterns.patterns[0].data[2] = 30;
    patterns.patterns[0].length = 3;
    patterns.patterns[0].index = 0;
    patterns.working = 0;

    let parts = vec!["P.NEXT"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert!(result.is_some());
    let (value, _) = result.unwrap();
    assert_eq!(value, 20);
    assert_eq!(patterns.patterns[0].index, 1);

    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert!(result.is_some());
    let (value, _) = result.unwrap();
    assert_eq!(value, 30);
    assert_eq!(patterns.patterns[0].index, 2);

    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert!(result.is_some());
    let (value, _) = result.unwrap();
    assert_eq!(value, 10);
    assert_eq!(patterns.patterns[0].index, 0);
}

#[test]
fn test_pattern_prev_operation() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();

    patterns.patterns[0].data[0] = 10;
    patterns.patterns[0].data[1] = 20;
    patterns.patterns[0].data[2] = 30;
    patterns.patterns[0].length = 3;
    patterns.patterns[0].index = 0;
    patterns.working = 0;

    let parts = vec!["P.PREV"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert!(result.is_some());
    let (value, _) = result.unwrap();
    assert_eq!(value, 30);
    assert_eq!(patterns.patterns[0].index, 2);

    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert!(result.is_some());
    let (value, _) = result.unwrap();
    assert_eq!(value, 20);
    assert_eq!(patterns.patterns[0].index, 1);
}

#[test]
fn test_pattern_here_operation() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();

    patterns.patterns[0].data[0] = 10;
    patterns.patterns[0].data[1] = 20;
    patterns.patterns[0].data[2] = 30;
    patterns.patterns[0].length = 3;
    patterns.patterns[0].index = 1;
    patterns.working = 0;

    let parts = vec!["P.HERE"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert!(result.is_some());
    let (value, consumed) = result.unwrap();
    assert_eq!(value, 20);
    assert_eq!(consumed, 1);
    assert_eq!(patterns.patterns[0].index, 1);
}

#[test]
fn test_pn_next_with_pattern_number() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();

    patterns.patterns[1].data[0] = 100;
    patterns.patterns[1].data[1] = 200;
    patterns.patterns[1].length = 2;
    patterns.patterns[1].index = 0;

    let parts = vec!["PN.NEXT", "1"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert!(result.is_some());
    let (value, consumed) = result.unwrap();
    assert_eq!(value, 200);
    assert_eq!(consumed, 2);
    assert_eq!(patterns.patterns[1].index, 1);
}

#[test]
fn test_pn_prev_with_pattern_number() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();

    patterns.patterns[2].data[0] = 50;
    patterns.patterns[2].data[1] = 60;
    patterns.patterns[2].data[2] = 70;
    patterns.patterns[2].length = 3;
    patterns.patterns[2].index = 0;

    let parts = vec!["PN.PREV", "2"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert!(result.is_some());
    let (value, consumed) = result.unwrap();
    assert_eq!(value, 70);
    assert_eq!(consumed, 2);
    assert_eq!(patterns.patterns[2].index, 2);
}

#[test]
fn test_resolve_value_function() {
    let mut variables = create_test_variables();
    let scripts = create_test_scripts();
    variables.a = 42;
    variables.b = -10;
    variables.i = 5;

    assert_eq!(resolve_value("A", &variables, &scripts, 0), 42);
    assert_eq!(resolve_value("B", &variables, &scripts, 0), -10);
    assert_eq!(resolve_value("I", &variables, &scripts, 0), 5);
    assert_eq!(resolve_value("100", &variables, &scripts, 0), 100);
    assert_eq!(resolve_value("-5", &variables, &scripts, 0), -5);
    assert_eq!(resolve_value("0", &variables, &scripts, 0), 0);
}

#[test]
fn test_eval_condition_simple_comparisons() {
    let mut variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    variables.a = 10;
    variables.b = 5;

    assert_eq!(eval_condition("IF A >= B", &variables, &mut patterns, &scripts, 0), true);
    assert_eq!(eval_condition("IF A <= B", &variables, &mut patterns, &scripts, 0), false);
    assert_eq!(eval_condition("IF A != B", &variables, &mut patterns, &scripts, 0), true);
    assert_eq!(eval_condition("IF B != B", &variables, &mut patterns, &scripts, 0), false);

    variables.a = 5;
    assert_eq!(eval_condition("IF A >= B", &variables, &mut patterns, &scripts, 0), true);
    assert_eq!(eval_condition("IF A <= B", &variables, &mut patterns, &scripts, 0), true);
    assert_eq!(eval_condition("IF A != B", &variables, &mut patterns, &scripts, 0), false);
}

#[test]
fn test_eval_condition_with_literals() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();

    assert_eq!(eval_condition("IF 10 >= 5", &variables, &mut patterns, &scripts, 0), true);
    assert_eq!(eval_condition("IF 5 <= 10", &variables, &mut patterns, &scripts, 0), true);
    assert_eq!(eval_condition("IF 5 != 10", &variables, &mut patterns, &scripts, 0), true);
    assert_eq!(eval_condition("IF 5 != 5", &variables, &mut patterns, &scripts, 0), false);
}

#[test]
fn test_pattern_operations_with_variable_indices() {
    let mut variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();

    patterns.patterns[0].data[0] = 100;
    patterns.patterns[1].data[0] = 200;
    patterns.patterns[2].data[0] = 300;
    patterns.patterns[3].data[0] = 400;

    variables.i = 0;
    let parts = vec!["PN", "0"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert_eq!(result.unwrap().0, 100);

    let parts = vec!["PN", "1"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert_eq!(result.unwrap().0, 200);

    let parts = vec!["PN", "2"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert_eq!(result.unwrap().0, 300);

    let parts = vec!["PN", "3"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert_eq!(result.unwrap().0, 400);
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
            assert!(value >= min && value < max,
                "RND {} returned {} (expected {} <= value < {})",
                parts[1], value, min, max);
        }
    }
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
fn test_pattern_length_wrapping() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();

    patterns.patterns[0].data[0] = 1;
    patterns.patterns[0].data[1] = 2;
    patterns.patterns[0].data[2] = 3;
    patterns.patterns[0].length = 3;
    patterns.patterns[0].index = 2;
    patterns.working = 0;

    let parts = vec!["P.NEXT"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert_eq!(result.unwrap().0, 1);
    assert_eq!(patterns.patterns[0].index, 0);
}

#[test]
fn test_pattern_prev_wrapping() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();

    patterns.patterns[0].data[0] = 1;
    patterns.patterns[0].data[1] = 2;
    patterns.patterns[0].data[2] = 3;
    patterns.patterns[0].length = 3;
    patterns.patterns[0].index = 0;
    patterns.working = 0;

    let parts = vec!["P.PREV"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert_eq!(result.unwrap().0, 3);
    assert_eq!(patterns.patterns[0].index, 2);
}

#[test]
fn test_multiple_pattern_indices() {
    let mut patterns = create_test_patterns();

    for i in 0..4 {
        patterns.patterns[i].data[0] = (i as i16 + 1) * 10;
        patterns.patterns[i].data[1] = (i as i16 + 1) * 10 + 1;
        patterns.patterns[i].length = 2;
        patterns.patterns[i].index = 0;
    }

    assert_eq!(patterns.patterns[0].data[0], 10);
    assert_eq!(patterns.patterns[1].data[0], 20);
    assert_eq!(patterns.patterns[2].data[0], 30);
    assert_eq!(patterns.patterns[3].data[0], 40);
}

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
fn test_pattern_next_with_expression_index() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();

    patterns.patterns[0].data[0] = 100;
    patterns.patterns[0].data[1] = 200;
    patterns.patterns[0].length = 2;
    patterns.patterns[0].index = 0;

    let parts = vec!["PN.NEXT", "ADD", "0", "0"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert!(result.is_some());
    let (value, consumed) = result.unwrap();
    assert_eq!(value, 200);
    assert_eq!(consumed, 4);
    assert_eq!(patterns.patterns[0].index, 1);
}

#[test]
fn test_pattern_with_expression_pattern_and_index() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();

    patterns.patterns[0].data[0] = 100;
    patterns.patterns[1].data[0] = 200;
    patterns.patterns[1].data[1] = 250;
    patterns.patterns[1].data[2] = 300;
    patterns.patterns[1].length = 3;
    patterns.patterns[1].index = 2;

    let parts = vec!["PN", "ADD", "0", "1"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert!(result.is_some());
    let (value, consumed) = result.unwrap();
    assert_eq!(value, 300);
    assert_eq!(consumed, 4);
}

#[test]
fn test_condition_with_add_expression() {
    let mut variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();

    variables.a = 10;

    assert_eq!(eval_condition("IF ADD A 1 > 0", &variables, &mut patterns, &scripts, 0), true);
    assert_eq!(eval_condition("IF ADD A 1 >= 11", &variables, &mut patterns, &scripts, 0), true);
    assert_eq!(eval_condition("IF ADD A 1 <= 11", &variables, &mut patterns, &scripts, 0), true);
    assert_eq!(eval_condition("IF ADD A 1 == 11", &variables, &mut patterns, &scripts, 0), true);
}

#[test]
fn test_condition_with_mul_expression() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();

    assert_eq!(eval_condition("IF MUL 2 3 == 6", &variables, &mut patterns, &scripts, 0), true);
    assert_eq!(eval_condition("IF MUL 2 3 != 5", &variables, &mut patterns, &scripts, 0), true);
    assert_eq!(eval_condition("IF MUL 2 3 >= 6", &variables, &mut patterns, &scripts, 0), true);
    assert_eq!(eval_condition("IF MUL 2 3 <= 6", &variables, &mut patterns, &scripts, 0), true);
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
        assert!(value >= 0 && value < 100);
        assert_eq!(consumed, 4);
    }
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
fn test_deeply_nested_expressions() {
    let mut variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();

    variables.a = 2;
    variables.b = 3;

    let parts = vec!["MUL", "ADD", "A", "B", "SUB", "10", "5"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert!(result.is_some());
    let (value, consumed) = result.unwrap();
    assert_eq!(value, 25);
    assert_eq!(consumed, 7);
}

#[test]
fn test_nested_pattern_operations() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();

    patterns.patterns[0].data[0] = 1;
    patterns.patterns[0].data[1] = 2;
    patterns.patterns[0].length = 2;
    patterns.patterns[0].index = 0;

    patterns.patterns[1].data[0] = 10;
    patterns.patterns[1].data[1] = 20;
    patterns.patterns[1].length = 2;
    patterns.patterns[1].index = 1;

    let parts = vec!["ADD", "PN", "0", "PN", "1"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert!(result.is_some());
    let (value, consumed) = result.unwrap();
    assert_eq!(value, 21);
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
fn test_eval_expression_with_start_idx() {
    let mut variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    variables.a = 10;

    let parts = vec!["SOME", "A", "IGNORED"];
    let result = eval_expression(&parts, 1, &variables, &mut patterns, &scripts, 0);
    assert!(result.is_some());
    let (value, consumed) = result.unwrap();
    assert_eq!(value, 10);
    assert_eq!(consumed, 1);
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
fn test_rrnd_insufficient_args() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();

    let parts = vec!["RRND", "50"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert!(result.is_none());
}

#[test]
fn test_pattern_operations_bounds() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();

    let parts = vec!["PN", "4"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert!(result.is_none());

    let parts = vec!["PN.NEXT", "5"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert!(result.is_none());
}

#[test]
fn test_variables_default_to_zero() {
    let variables = create_test_variables();

    assert_eq!(variables.a, 0);
    assert_eq!(variables.b, 0);
    assert_eq!(variables.c, 0);
    assert_eq!(variables.d, 0);
    assert_eq!(variables.i, 0);
    assert_eq!(variables.x, 0);
    assert_eq!(variables.y, 0);
    assert_eq!(variables.z, 0);
    assert_eq!(variables.t, 0);
}

#[test]
fn test_pattern_storage_default() {
    let patterns = create_test_patterns();

    assert_eq!(patterns.working, 0);
    for i in 0..4 {
        assert_eq!(patterns.patterns[i].length, 64);
        assert_eq!(patterns.patterns[i].index, 0);
        for j in 0..64 {
            assert_eq!(patterns.patterns[i].data[j], 0);
        }
    }
}
