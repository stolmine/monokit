use crate::eval::{eval_condition, eval_expression, resolve_value};
use crate::scene::{sanitize_name, scene_path, Scene, ScenePattern, SceneScript};
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

#[test]
fn test_if_condition_true_executes() {
    let mut variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    variables.a = 10;

    assert_eq!(eval_condition("IF A == 10", &variables, &mut patterns, &scripts, 0), true);
    assert_eq!(eval_condition("IF A > 5", &variables, &mut patterns, &scripts, 0), true);
    assert_eq!(eval_condition("IF A >= 10", &variables, &mut patterns, &scripts, 0), true);
}

#[test]
fn test_if_condition_false_skips() {
    let mut variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    variables.a = 5;

    assert_eq!(eval_condition("IF A == 10", &variables, &mut patterns, &scripts, 0), false);
    assert_eq!(eval_condition("IF A > 10", &variables, &mut patterns, &scripts, 0), false);
    assert_eq!(eval_condition("IF A < 5", &variables, &mut patterns, &scripts, 0), false);
}

#[test]
fn test_if_with_nested_expressions() {
    let mut variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    variables.a = 5;
    variables.b = 3;

    assert_eq!(eval_condition("IF ADD A B == 8", &variables, &mut patterns, &scripts, 0), true);
    assert_eq!(eval_condition("IF MUL A B == 15", &variables, &mut patterns, &scripts, 0), true);
    assert_eq!(eval_condition("IF SUB A B == 2", &variables, &mut patterns, &scripts, 0), true);
}

#[test]
fn test_nested_math_in_conditions() {
    let mut variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    variables.a = 10;

    assert_eq!(eval_condition("IF ADD A 5 >= 15", &variables, &mut patterns, &scripts, 0), true);
    assert_eq!(eval_condition("IF MUL ADD A 2 2 == 24", &variables, &mut patterns, &scripts, 0), true);
}

#[test]
fn test_pattern_ops_in_conditions() {
    let mut variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();

    patterns.patterns[0].data[0] = 100;
    patterns.patterns[0].length = 1;
    patterns.patterns[0].index = 0;

    assert_eq!(eval_condition("IF PN 0 == 100", &variables, &mut patterns, &scripts, 0), true);
    assert_eq!(eval_condition("IF PN 0 >= 50", &variables, &mut patterns, &scripts, 0), true);
    assert_eq!(eval_condition("IF PN 0 != 0", &variables, &mut patterns, &scripts, 0), true);
}

#[test]
fn test_variables_in_all_expression_positions() {
    let mut variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();

    variables.a = 10;
    variables.b = 5;
    variables.c = 2;

    let parts = vec!["ADD", "A", "B"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert_eq!(result.unwrap().0, 15);

    let parts = vec!["MUL", "ADD", "A", "B", "C"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert_eq!(result.unwrap().0, 30);
}

#[test]
fn test_prob_condition_always_in_range() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();

    let mut true_count = 0;
    let mut false_count = 0;

    for _ in 0..100 {
        if eval_condition("PROB 50", &variables, &mut patterns, &scripts, 0) {
            true_count += 1;
        } else {
            false_count += 1;
        }
    }

    assert!(true_count > 20 && true_count < 80, "PROB 50 should be roughly 50/50, got {}/{}", true_count, false_count);
}

#[test]
fn test_prob_with_expression() {
    let mut variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    variables.a = 25;

    let mut true_count = 0;
    for _ in 0..100 {
        if eval_condition("PROB ADD A 25", &variables, &mut patterns, &scripts, 0) {
            true_count += 1;
        }
    }

    assert!(true_count > 20 && true_count < 80, "PROB (25+25=50) should be roughly 50%, got {}", true_count);
}

#[test]
fn test_deeply_nested_pattern_and_math() {
    let mut variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();

    patterns.patterns[0].data[0] = 10;
    patterns.patterns[0].data[1] = 20;
    patterns.patterns[0].length = 2;
    patterns.patterns[0].index = 0;

    variables.a = 5;

    let parts = vec!["MUL", "ADD", "PN", "0", "A", "SUB", "10", "3"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert_eq!(result.unwrap().0, 105);
}

#[test]
fn test_condition_comparison_operators() {
    let mut variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    variables.a = 10;

    assert_eq!(eval_condition("IF A > 5", &variables, &mut patterns, &scripts, 0), true);
    assert_eq!(eval_condition("IF A < 15", &variables, &mut patterns, &scripts, 0), true);
    assert_eq!(eval_condition("IF A >= 10", &variables, &mut patterns, &scripts, 0), true);
    assert_eq!(eval_condition("IF A <= 10", &variables, &mut patterns, &scripts, 0), true);
    assert_eq!(eval_condition("IF A == 10", &variables, &mut patterns, &scripts, 0), true);
    assert_eq!(eval_condition("IF A != 5", &variables, &mut patterns, &scripts, 0), true);

    assert_eq!(eval_condition("IF A > 10", &variables, &mut patterns, &scripts, 0), false);
    assert_eq!(eval_condition("IF A < 10", &variables, &mut patterns, &scripts, 0), false);
    assert_eq!(eval_condition("IF A == 5", &variables, &mut patterns, &scripts, 0), false);
    assert_eq!(eval_condition("IF A != 10", &variables, &mut patterns, &scripts, 0), false);
}

#[test]
fn test_semicolon_separated_expressions() {
    let mut variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();

    variables.a = 0;

    let parts = vec!["ADD", "1", "1"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert_eq!(result.unwrap().0, 2);

    let parts = vec!["ADD", "2", "2"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert_eq!(result.unwrap().0, 4);
}

#[test]
fn test_expression_with_all_parameter_types() {
    let mut variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();

    variables.a = 10;
    patterns.patterns[0].data[0] = 20;
    patterns.patterns[0].length = 1;
    patterns.patterns[0].index = 0;

    let parts = vec!["ADD", "A", "PN", "0"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert_eq!(result.unwrap().0, 30);

    let parts = vec!["ADD", "5", "10"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert_eq!(result.unwrap().0, 15);

    let parts = vec!["ADD", "ADD", "5", "5", "A"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert_eq!(result.unwrap().0, 20);
}

#[test]
fn test_script_j_k_variables() {
    let mut variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let mut scripts = create_test_scripts();

    scripts.scripts[0].j = 42;
    scripts.scripts[0].k = 100;

    let parts = vec!["J"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert_eq!(result.unwrap().0, 42);

    let parts = vec!["K"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert_eq!(result.unwrap().0, 100);

    let parts = vec!["ADD", "J", "K"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert_eq!(result.unwrap().0, 142);
}

#[test]
fn test_all_math_operations() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();

    let parts = vec!["ADD", "10", "5"];
    assert_eq!(eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0).unwrap().0, 15);

    let parts = vec!["SUB", "10", "5"];
    assert_eq!(eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0).unwrap().0, 5);

    let parts = vec!["MUL", "10", "5"];
    assert_eq!(eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0).unwrap().0, 50);

    let parts = vec!["DIV", "10", "5"];
    assert_eq!(eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0).unwrap().0, 2);

    let parts = vec!["MOD", "10", "3"];
    assert_eq!(eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0).unwrap().0, 1);
}

#[test]
fn test_div_by_zero_returns_zero() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();

    let parts = vec!["DIV", "10", "0"];
    assert_eq!(eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0).unwrap().0, 0);
}

#[test]
fn test_mod_by_zero_returns_zero() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();

    let parts = vec!["MOD", "10", "0"];
    assert_eq!(eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0).unwrap().0, 0);
}

#[test]
fn test_symbol_add() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();

    let parts = vec!["+", "10", "5"];
    assert_eq!(eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0).unwrap().0, 15);
}

#[test]
fn test_symbol_sub() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();

    let parts = vec!["-", "10", "5"];
    assert_eq!(eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0).unwrap().0, 5);
}

#[test]
fn test_symbol_mul() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();

    let parts = vec!["*", "10", "5"];
    assert_eq!(eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0).unwrap().0, 50);
}

#[test]
fn test_symbol_div() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();

    let parts = vec!["/", "10", "5"];
    assert_eq!(eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0).unwrap().0, 2);
}

#[test]
fn test_symbol_mod() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();

    let parts = vec!["%", "10", "3"];
    assert_eq!(eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0).unwrap().0, 1);
}

#[test]
fn test_symbol_operators_with_variables() {
    let mut variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();

    variables.a = 10;
    variables.b = 5;

    let parts = vec!["+", "A", "B"];
    assert_eq!(eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0).unwrap().0, 15);

    let parts = vec!["-", "A", "B"];
    assert_eq!(eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0).unwrap().0, 5);

    let parts = vec!["*", "A", "B"];
    assert_eq!(eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0).unwrap().0, 50);

    let parts = vec!["/", "A", "B"];
    assert_eq!(eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0).unwrap().0, 2);
}

#[test]
fn test_nested_symbol_operators() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();

    let parts = vec!["+", "+", "1", "2", "3"];
    assert_eq!(eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0).unwrap().0, 6);

    let parts = vec!["*", "+", "5", "1", "2"];
    assert_eq!(eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0).unwrap().0, 12);
}

#[test]
fn test_mixed_word_and_symbol_operators() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();

    let parts = vec!["+", "MUL", "2", "3", "4"];
    assert_eq!(eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0).unwrap().0, 10);

    let parts = vec!["*", "ADD", "2", "3", "DIV", "10", "5"];
    assert_eq!(eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0).unwrap().0, 10);
}

#[test]
fn test_triple_nested_expressions() {
    let mut variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();

    variables.a = 2;

    let parts = vec!["MUL", "ADD", "MUL", "A", "3", "ADD", "1", "1", "DIV", "20", "2"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert_eq!(result.unwrap().0, 80);
}

#[test]
fn test_all_variables_in_expressions() {
    let mut variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();

    variables.a = 1;
    variables.b = 2;
    variables.c = 3;
    variables.d = 4;
    variables.i = 5;
    variables.x = 6;
    variables.y = 7;
    variables.z = 8;
    variables.t = 9;

    assert_eq!(eval_expression(&vec!["A"], 0, &variables, &mut patterns, &scripts, 0).unwrap().0, 1);
    assert_eq!(eval_expression(&vec!["B"], 0, &variables, &mut patterns, &scripts, 0).unwrap().0, 2);
    assert_eq!(eval_expression(&vec!["C"], 0, &variables, &mut patterns, &scripts, 0).unwrap().0, 3);
    assert_eq!(eval_expression(&vec!["D"], 0, &variables, &mut patterns, &scripts, 0).unwrap().0, 4);
    assert_eq!(eval_expression(&vec!["I"], 0, &variables, &mut patterns, &scripts, 0).unwrap().0, 5);
    assert_eq!(eval_expression(&vec!["X"], 0, &variables, &mut patterns, &scripts, 0).unwrap().0, 6);
    assert_eq!(eval_expression(&vec!["Y"], 0, &variables, &mut patterns, &scripts, 0).unwrap().0, 7);
    assert_eq!(eval_expression(&vec!["Z"], 0, &variables, &mut patterns, &scripts, 0).unwrap().0, 8);
    assert_eq!(eval_expression(&vec!["T"], 0, &variables, &mut patterns, &scripts, 0).unwrap().0, 9);
}

#[test]
fn test_pattern_p_next_advances_index() {
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
    assert_eq!(eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0).unwrap().0, 20);
    assert_eq!(patterns.patterns[0].index, 1);

    assert_eq!(eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0).unwrap().0, 30);
    assert_eq!(patterns.patterns[0].index, 2);

    assert_eq!(eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0).unwrap().0, 10);
    assert_eq!(patterns.patterns[0].index, 0);
}

#[test]
fn test_pattern_pn_next_with_index() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();

    patterns.patterns[2].data[0] = 100;
    patterns.patterns[2].data[1] = 200;
    patterns.patterns[2].length = 2;
    patterns.patterns[2].index = 0;

    let parts = vec!["PN.NEXT", "2"];
    assert_eq!(eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0).unwrap().0, 200);
    assert_eq!(patterns.patterns[2].index, 1);

    assert_eq!(eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0).unwrap().0, 100);
    assert_eq!(patterns.patterns[2].index, 0);
}

#[test]
fn test_saturating_arithmetic() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();

    let parts = vec!["ADD", "32000", "32000"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert_eq!(result.unwrap().0, 32767);

    let parts = vec!["SUB", "-32000", "32000"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert_eq!(result.unwrap().0, -32768);

    let parts = vec!["MUL", "1000", "1000"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert_eq!(result.unwrap().0, 32767);
}

#[test]
fn test_negative_numbers_in_expressions() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();

    let parts = vec!["ADD", "-10", "5"];
    assert_eq!(eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0).unwrap().0, -5);

    let parts = vec!["SUB", "-10", "5"];
    assert_eq!(eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0).unwrap().0, -15);

    let parts = vec!["MUL", "-10", "5"];
    assert_eq!(eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0).unwrap().0, -50);

    let parts = vec!["DIV", "-10", "5"];
    assert_eq!(eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0).unwrap().0, -2);
}

#[test]
fn test_condition_with_negative_numbers() {
    let mut variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();

    variables.a = -10;

    assert_eq!(eval_condition("IF A < 0", &variables, &mut patterns, &scripts, 0), true);
    assert_eq!(eval_condition("IF A >= -10", &variables, &mut patterns, &scripts, 0), true);
    assert_eq!(eval_condition("IF A == -10", &variables, &mut patterns, &scripts, 0), true);
}

#[test]
fn test_pattern_here_doesnt_change_index() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();

    patterns.patterns[0].data[0] = 10;
    patterns.patterns[0].data[1] = 20;
    patterns.patterns[0].length = 2;
    patterns.patterns[0].index = 1;
    patterns.working = 0;

    let parts = vec!["P.HERE"];
    assert_eq!(eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0).unwrap().0, 20);
    assert_eq!(patterns.patterns[0].index, 1);

    assert_eq!(eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0).unwrap().0, 20);
    assert_eq!(patterns.patterns[0].index, 1);
}

#[test]
fn test_all_pattern_operations() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();

    patterns.patterns[1].data[0] = 100;
    patterns.patterns[1].data[1] = 200;
    patterns.patterns[1].data[2] = 300;
    patterns.patterns[1].length = 3;
    patterns.patterns[1].index = 1;

    let parts = vec!["PN", "1"];
    assert_eq!(eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0).unwrap().0, 200);

    let parts = vec!["PN.HERE", "1"];
    assert_eq!(eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0).unwrap().0, 200);

    let parts = vec!["PN.L", "1"];
    assert_eq!(eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0).unwrap().0, 3);

    let parts = vec!["PN.I", "1"];
    assert_eq!(eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0).unwrap().0, 1);

    let parts = vec!["PN.NEXT", "1"];
    assert_eq!(eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0).unwrap().0, 300);

    let parts = vec!["PN.PREV", "1"];
    assert_eq!(eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0).unwrap().0, 200);
}

#[test]
fn test_resolve_value_with_j_k() {
    let variables = create_test_variables();
    let mut scripts = create_test_scripts();

    scripts.scripts[5].j = 99;
    scripts.scripts[5].k = 88;

    assert_eq!(resolve_value("J", &variables, &scripts, 5), 99);
    assert_eq!(resolve_value("K", &variables, &scripts, 5), 88);

    assert_eq!(resolve_value("J", &variables, &scripts, 10), 0);
    assert_eq!(resolve_value("K", &variables, &scripts, 10), 0);
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
        assert!(value >= 0 && value < 20);
    }
}

#[test]
fn test_complex_nested_pattern_and_math() {
    let mut variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();

    patterns.patterns[0].data[0] = 5;
    patterns.patterns[1].data[0] = 10;
    variables.a = 2;

    let parts = vec!["ADD", "MUL", "PN", "0", "A", "DIV", "PN", "1", "2"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert_eq!(result.unwrap().0, 15);
}

#[test]
fn test_scene_serialization_roundtrip() {
    let scene = Scene {
        version: 1,
        scripts: vec![SceneScript {
            lines: vec![
                "PF 200".to_string(),
                "TR".to_string(),
                "".to_string(),
                "".to_string(),
                "".to_string(),
                "".to_string(),
                "".to_string(),
                "".to_string(),
            ],
            j: 0,
            k: 0,
        }],
        patterns: vec![ScenePattern {
            data: vec![100, 200, 300, 400],
            length: 4,
            index: 0,
        }],
        pattern_working: 0,
    };

    let json = serde_json::to_string(&scene).unwrap();
    let loaded: Scene = serde_json::from_str(&json).unwrap();

    assert_eq!(loaded.version, scene.version);
    assert_eq!(loaded.pattern_working, scene.pattern_working);
    assert_eq!(loaded.scripts.len(), 1);
    assert_eq!(loaded.scripts[0].lines[0], "PF 200");
    assert_eq!(loaded.scripts[0].lines[1], "TR");
    assert_eq!(loaded.patterns.len(), 1);
    assert_eq!(loaded.patterns[0].data[0], 100);
    assert_eq!(loaded.patterns[0].data[1], 200);
    assert_eq!(loaded.patterns[0].length, 4);
}

#[test]
fn test_scene_from_app_state() {
    let mut scripts = create_test_scripts();
    let mut patterns = create_test_patterns();

    scripts.scripts[0].lines[0] = "PF 100".to_string();
    scripts.scripts[0].lines[1] = "TR".to_string();
    scripts.scripts[0].j = 5;
    scripts.scripts[0].k = 10;

    patterns.patterns[0].data[0] = 100;
    patterns.patterns[0].data[1] = 200;
    patterns.patterns[0].data[2] = 300;
    patterns.patterns[0].length = 3;
    patterns.patterns[0].index = 1;
    patterns.working = 2;

    let scene = Scene::from_app_state(&scripts, &patterns);

    assert_eq!(scene.version, 1);
    assert_eq!(scene.scripts.len(), 10);
    assert_eq!(scene.scripts[0].lines[0], "PF 100");
    assert_eq!(scene.scripts[0].lines[1], "TR");
    assert_eq!(scene.scripts[0].j, 5);
    assert_eq!(scene.scripts[0].k, 10);
    assert_eq!(scene.patterns.len(), 4);
    assert_eq!(scene.patterns[0].data[0], 100);
    assert_eq!(scene.patterns[0].data[1], 200);
    assert_eq!(scene.patterns[0].data[2], 300);
    assert_eq!(scene.patterns[0].length, 3);
    assert_eq!(scene.patterns[0].index, 1);
    assert_eq!(scene.pattern_working, 2);
}

#[test]
fn test_scene_apply_to_app_state() {
    let scene = Scene {
        version: 1,
        scripts: vec![SceneScript {
            lines: vec![
                "A 10".to_string(),
                "B 20".to_string(),
                "".to_string(),
                "".to_string(),
                "".to_string(),
                "".to_string(),
                "".to_string(),
                "".to_string(),
            ],
            j: 42,
            k: 84,
        }],
        patterns: vec![ScenePattern {
            data: vec![111, 222, 333, 444],
            length: 4,
            index: 2,
        }],
        pattern_working: 1,
    };

    let mut scripts = create_test_scripts();
    let mut patterns = create_test_patterns();

    scene.apply_to_app_state(&mut scripts, &mut patterns);

    assert_eq!(scripts.scripts[0].lines[0], "A 10");
    assert_eq!(scripts.scripts[0].lines[1], "B 20");
    assert_eq!(scripts.scripts[0].j, 42);
    assert_eq!(scripts.scripts[0].k, 84);
    assert_eq!(patterns.patterns[0].data[0], 111);
    assert_eq!(patterns.patterns[0].data[1], 222);
    assert_eq!(patterns.patterns[0].data[2], 333);
    assert_eq!(patterns.patterns[0].data[3], 444);
    assert_eq!(patterns.patterns[0].length, 4);
    assert_eq!(patterns.patterns[0].index, 2);
    assert_eq!(patterns.working, 1);
}

#[test]
fn test_sanitize_name_spaces() {
    assert_eq!(sanitize_name("my scene"), "my-scene");
}

#[test]
fn test_sanitize_name_special_chars() {
    assert_eq!(sanitize_name("test@#$name"), "test---name");
}

#[test]
fn test_sanitize_name_alphanumeric() {
    assert_eq!(sanitize_name("test-name_123"), "test-name_123");
}

#[test]
fn test_scene_path_ends_with_json() {
    let path = scene_path("test");
    assert!(path.to_string_lossy().ends_with(".json"));
}

#[test]
fn test_scene_path_uses_sanitized_name() {
    let path = scene_path("my test scene");
    let path_str = path.to_string_lossy();
    assert!(path_str.ends_with("my-test-scene.json"));
}

// N operator tests (semitone to frequency)

#[test]
fn test_n_zero_is_c3() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let parts = vec!["N", "0"];

    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert!(result.is_some());
    let (value, consumed) = result.unwrap();
    // C3 = 130.81 Hz, rounds to 131
    assert_eq!(value, 131);
    assert_eq!(consumed, 2);
}

#[test]
fn test_n_12_is_c4() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let parts = vec!["N", "12"];

    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert!(result.is_some());
    let (value, consumed) = result.unwrap();
    // C4 = 261.63 Hz, rounds to 262
    assert_eq!(value, 262);
    assert_eq!(consumed, 2);
}

#[test]
fn test_n_24_is_c5() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let parts = vec!["N", "24"];

    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert!(result.is_some());
    let (value, consumed) = result.unwrap();
    // C5 = 523.25 Hz, rounds to 523
    assert_eq!(value, 523);
    assert_eq!(consumed, 2);
}

#[test]
fn test_n_negative_12_is_c2() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let parts = vec!["N", "-12"];

    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert!(result.is_some());
    let (value, consumed) = result.unwrap();
    // C2 = 65.41 Hz, rounds to 65
    assert_eq!(value, 65);
    assert_eq!(consumed, 2);
}

#[test]
fn test_n_21_is_a4_440hz() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    // A4 = 440 Hz is 21 semitones above C3 (C3 + 9 semitones = A3, + 12 = A4)
    let parts = vec!["N", "21"];

    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert!(result.is_some());
    let (value, consumed) = result.unwrap();
    // A4 = 440 Hz
    assert_eq!(value, 440);
    assert_eq!(consumed, 2);
}

#[test]
fn test_n_with_variable() {
    let mut variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    variables.a = 12; // C4
    let parts = vec!["N", "A"];

    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert!(result.is_some());
    let (value, consumed) = result.unwrap();
    assert_eq!(value, 262); // C4
    assert_eq!(consumed, 2);
}

#[test]
fn test_n_nested_in_expression() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    // ADD N 0 N 12 = 131 + 262 = 393
    let parts = vec!["ADD", "N", "0", "N", "12"];

    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert!(result.is_some());
    let (value, consumed) = result.unwrap();
    assert_eq!(value, 393);
    assert_eq!(consumed, 5);
}

#[test]
fn test_n_with_add_semitones() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    // N ADD 0 7 = N 7 = G3 (7 semitones above C3)
    let parts = vec!["N", "ADD", "0", "7"];

    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert!(result.is_some());
    let (value, consumed) = result.unwrap();
    // G3 = 196 Hz
    assert_eq!(value, 196);
    assert_eq!(consumed, 4);
}

// Comparison operator tests

#[test]
fn test_ez_equals_zero() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();

    // EZ 0 should return 1 (true)
    let parts = vec!["EZ", "0"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert_eq!(result, Some((1, 2)));

    // EZ 5 should return 0 (false)
    let parts = vec!["EZ", "5"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert_eq!(result, Some((0, 2)));
}

#[test]
fn test_nz_not_zero() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();

    // NZ 0 should return 0 (false)
    let parts = vec!["NZ", "0"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert_eq!(result, Some((0, 2)));

    // NZ 5 should return 1 (true)
    let parts = vec!["NZ", "5"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert_eq!(result, Some((1, 2)));

    // NZ -5 should return 1 (true) - negative is non-zero
    let parts = vec!["NZ", "-5"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert_eq!(result, Some((1, 2)));
}

#[test]
fn test_eq_equals() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();

    // EQ 5 5 should return 1
    let parts = vec!["EQ", "5", "5"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert_eq!(result, Some((1, 3)));

    // EQ 5 3 should return 0
    let parts = vec!["EQ", "5", "3"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert_eq!(result, Some((0, 3)));
}

#[test]
fn test_ne_not_equals() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();

    // NE 5 3 should return 1
    let parts = vec!["NE", "5", "3"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert_eq!(result, Some((1, 3)));

    // NE 5 5 should return 0
    let parts = vec!["NE", "5", "5"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert_eq!(result, Some((0, 3)));
}

#[test]
fn test_gt_greater_than() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();

    // GT 5 3 should return 1
    let parts = vec!["GT", "5", "3"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert_eq!(result, Some((1, 3)));

    // GT 3 5 should return 0
    let parts = vec!["GT", "3", "5"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert_eq!(result, Some((0, 3)));

    // GT 5 5 should return 0
    let parts = vec!["GT", "5", "5"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert_eq!(result, Some((0, 3)));
}

#[test]
fn test_lt_less_than() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();

    // LT 3 5 should return 1
    let parts = vec!["LT", "3", "5"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert_eq!(result, Some((1, 3)));

    // LT 5 3 should return 0
    let parts = vec!["LT", "5", "3"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert_eq!(result, Some((0, 3)));
}

#[test]
fn test_gte_greater_than_or_equal() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();

    // GTE 5 3 should return 1
    let parts = vec!["GTE", "5", "3"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert_eq!(result, Some((1, 3)));

    // GTE 5 5 should return 1
    let parts = vec!["GTE", "5", "5"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert_eq!(result, Some((1, 3)));

    // GTE 3 5 should return 0
    let parts = vec!["GTE", "3", "5"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert_eq!(result, Some((0, 3)));
}

#[test]
fn test_lte_less_than_or_equal() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();

    // LTE 3 5 should return 1
    let parts = vec!["LTE", "3", "5"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert_eq!(result, Some((1, 3)));

    // LTE 5 5 should return 1
    let parts = vec!["LTE", "5", "5"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert_eq!(result, Some((1, 3)));

    // LTE 5 3 should return 0
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

    // GT A B should return 1 (10 > 5)
    let parts = vec!["GT", "A", "B"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert_eq!(result, Some((1, 3)));

    // EZ A should return 0 (10 != 0)
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

    // NZ PN.HERE 0 should return 1 (100 != 0)
    let parts = vec!["NZ", "PN.HERE", "0"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert_eq!(result, Some((1, 3)));

    // Advance to index 1 which has value 0
    patterns.patterns[0].index = 1;

    // NZ PN.HERE 0 should return 0 (0 is zero)
    let parts = vec!["NZ", "PN.HERE", "0"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert_eq!(result, Some((0, 3)));

    // EZ PN.HERE 0 should return 1 (0 == 0)
    let parts = vec!["EZ", "PN.HERE", "0"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert_eq!(result, Some((1, 3)));
}

// IF truthy/falsy condition tests

#[test]
fn test_if_truthy_nonzero_is_true() {
    let mut variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    variables.a = 5;

    // IF A (where A=5) should be true
    assert!(eval_condition("IF A", &variables, &mut patterns, &scripts, 0));

    // Negative numbers are also truthy
    variables.a = -5;
    assert!(eval_condition("IF A", &variables, &mut patterns, &scripts, 0));
}

#[test]
fn test_if_falsy_zero_is_false() {
    let mut variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    variables.a = 0;

    // IF A (where A=0) should be false
    assert!(!eval_condition("IF A", &variables, &mut patterns, &scripts, 0));
}

#[test]
fn test_if_with_pattern_value_truthy() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    patterns.patterns[0].data[0] = 100;

    // IF PN.HERE 0 (value=100) should be true
    assert!(eval_condition("IF PN.HERE 0", &variables, &mut patterns, &scripts, 0));
}

#[test]
fn test_if_with_pattern_value_falsy() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    patterns.patterns[0].data[0] = 0;

    // IF PN.HERE 0 (value=0) should be false
    assert!(!eval_condition("IF PN.HERE 0", &variables, &mut patterns, &scripts, 0));
}

#[test]
fn test_if_with_comparison_operators() {
    let mut variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    variables.a = 10;

    // IF GT A 5 should be true (10 > 5)
    assert!(eval_condition("IF GT A 5", &variables, &mut patterns, &scripts, 0));

    // IF LT A 5 should be false (10 < 5 is false)
    assert!(!eval_condition("IF LT A 5", &variables, &mut patterns, &scripts, 0));

    // IF EZ A should be false (10 != 0)
    assert!(!eval_condition("IF EZ A", &variables, &mut patterns, &scripts, 0));

    // IF NZ A should be true (10 != 0)
    assert!(eval_condition("IF NZ A", &variables, &mut patterns, &scripts, 0));
}

#[test]
fn test_if_with_mixed_pattern_values() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();

    // Set up pattern with alternating 0 and 1 values
    patterns.patterns[0].data[0] = 1;
    patterns.patterns[0].data[1] = 0;
    patterns.patterns[0].data[2] = 1;
    patterns.patterns[0].data[3] = 0;
    patterns.patterns[0].length = 4;

    // Test each position
    patterns.patterns[0].index = 0;
    assert!(eval_condition("IF PN.HERE 0", &variables, &mut patterns, &scripts, 0)); // 1 is truthy

    patterns.patterns[0].index = 1;
    assert!(!eval_condition("IF PN.HERE 0", &variables, &mut patterns, &scripts, 0)); // 0 is falsy

    patterns.patterns[0].index = 2;
    assert!(eval_condition("IF PN.HERE 0", &variables, &mut patterns, &scripts, 0)); // 1 is truthy

    patterns.patterns[0].index = 3;
    assert!(!eval_condition("IF PN.HERE 0", &variables, &mut patterns, &scripts, 0)); // 0 is falsy
}

#[test]
fn test_nested_comparison_in_if() {
    let mut variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    variables.a = 3;
    variables.b = 7;

    // IF GT ADD A B 5 should be true (3+7=10 > 5)
    assert!(eval_condition("IF GT ADD A B 5", &variables, &mut patterns, &scripts, 0));

    // IF EQ MUL A B 21 should be true (3*7=21 == 21)
    assert!(eval_condition("IF EQ MUL A B 21", &variables, &mut patterns, &scripts, 0));
}

#[test]
fn test_if_pn_here_after_pn_next() {
    // This reproduces the user's bug: PN.NEXT 0; IF PN.HERE 0: TR
    // Pattern has mix of 0s and 1s, IF should only trigger on non-zero
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();

    // Set up pattern 0 with: 1, 0, 1, 0
    patterns.patterns[0].data[0] = 1;
    patterns.patterns[0].data[1] = 0;
    patterns.patterns[0].data[2] = 1;
    patterns.patterns[0].data[3] = 0;
    patterns.patterns[0].length = 4;
    patterns.patterns[0].index = 0;

    // Simulate: PN.NEXT 0 (advances to index 1, returns value at index 1 which is 0)
    let parts = vec!["PN.NEXT", "0"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert_eq!(result, Some((0, 2))); // Value at new index is 0
    assert_eq!(patterns.patterns[0].index, 1); // Index advanced to 1

    // Now IF PN.HERE 0: should be FALSE (value at index 1 is 0)
    assert!(!eval_condition("IF PN.HERE 0", &variables, &mut patterns, &scripts, 0));

    // Advance again: PN.NEXT 0 (advances to index 2, returns value at index 2 which is 1)
    let parts = vec!["PN.NEXT", "0"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &scripts, 0);
    assert_eq!(result, Some((1, 2))); // Value at new index is 1
    assert_eq!(patterns.patterns[0].index, 2); // Index advanced to 2

    // Now IF PN.HERE 0: should be TRUE (value at index 2 is 1)
    assert!(eval_condition("IF PN.HERE 0", &variables, &mut patterns, &scripts, 0));
}

#[test]
fn test_if_lowercase_pn_here() {
    // Test that lowercase works too
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();

    patterns.patterns[0].data[0] = 42;
    patterns.patterns[0].index = 0;

    // Lowercase should work
    assert!(eval_condition("IF pn.here 0", &variables, &mut patterns, &scripts, 0));

    patterns.patterns[0].data[0] = 0;
    assert!(!eval_condition("IF pn.here 0", &variables, &mut patterns, &scripts, 0));
}

#[test]
fn test_validate_valid_commands() {
    use crate::commands::validate_script_command;

    assert!(validate_script_command("TR").is_ok());
    assert!(validate_script_command("PF 440").is_ok());
    assert!(validate_script_command("A 100").is_ok());
    assert!(validate_script_command("P.NEXT").is_ok());
    assert!(validate_script_command("IF A > 5: TR").is_ok());
    assert!(validate_script_command("SCRIPT 1").is_ok());
    assert!(validate_script_command("M.ACT 1").is_ok());
    assert!(validate_script_command("").is_ok());
}

#[test]
fn test_validate_invalid_commands() {
    use crate::commands::validate_script_command;

    assert!(validate_script_command("INVALID_CMD").is_err());
    assert!(validate_script_command("NOTREAL 123").is_err());
    assert!(validate_script_command("BADCOMMAND").is_err());
}

#[test]
fn test_validate_commands_with_missing_args() {
    use crate::commands::validate_script_command;

    assert!(validate_script_command("VOL").is_err());
    assert!(validate_script_command("PF").is_err());
    assert!(validate_script_command("DC").is_err());
}

#[test]
fn test_validate_control_flow() {
    use crate::commands::validate_script_command;

    assert!(validate_script_command("IF A > 0: TR").is_ok());
    assert!(validate_script_command("ELIF A < 10: PF 200").is_ok());
    assert!(validate_script_command("ELSE: RST").is_ok());
    assert!(validate_script_command("PROB 50: TR").is_ok());
    assert!(validate_script_command("EV 4: TR").is_ok());
    assert!(validate_script_command("SKIP 2: PF 100").is_ok());
    assert!(validate_script_command("L 0 10: A I").is_ok());
}

#[test]
fn test_validate_semicolon_commands() {
    use crate::commands::validate_script_command;

    assert!(validate_script_command("TR; PF 440; VOL 0.5").is_ok());
}
