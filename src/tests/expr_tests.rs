use crate::eval::{eval_expression, resolve_value};
use super::common::{create_test_variables, create_test_patterns, create_test_scripts, create_test_counters, create_test_scale};

#[test]
fn test_eval_expression_variables() {
    let mut variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let mut counters = create_test_counters();

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
        let result = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &create_test_scale());
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
    let mut counters = create_test_counters();

    let test_cases = vec![
        (vec!["0"], 0),
        (vec!["42"], 42),
        (vec!["-10"], -10),
        (vec!["999"], 999),
    ];

    for (parts, expected) in test_cases {
        let result = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &create_test_scale());
        assert!(result.is_some());
        let (value, consumed) = result.unwrap();
        assert_eq!(value, expected);
        assert_eq!(consumed, 1);
    }
}

#[test]
fn test_eval_expression_with_start_idx() {
    let mut variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let mut counters = create_test_counters();
    variables.a = 10;

    let parts = vec!["SOME", "A", "IGNORED"];
    let result = eval_expression(&parts, 1, &variables, &mut patterns, &mut counters, &scripts, 0, &create_test_scale());
    assert!(result.is_some());
    let (value, consumed) = result.unwrap();
    assert_eq!(value, 10);
    assert_eq!(consumed, 1);
}

#[test]
fn test_resolve_value_function() {
    let mut variables = create_test_variables();
    let scripts = create_test_scripts();
    let mut counters = create_test_counters();
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
fn test_variables_in_all_expression_positions() {
    let mut variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let mut counters = create_test_counters();

    variables.a = 5;
    variables.b = 3;

    let parts = vec!["ADD", "A", "B"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &create_test_scale());
    assert_eq!(result.unwrap().0, 8);

    let parts = vec!["MUL", "A", "B"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &create_test_scale());
    assert_eq!(result.unwrap().0, 15);

    let parts = vec!["SUB", "A", "B"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &create_test_scale());
    assert_eq!(result.unwrap().0, 2);
}

#[test]
fn test_script_j_k_variables() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let mut scripts = create_test_scripts();

    scripts.scripts[0].j = 10;
    scripts.scripts[0].k = 20;

    assert_eq!(resolve_value("J", &variables, &scripts, 0), 10);
    assert_eq!(resolve_value("K", &variables, &scripts, 0), 20);

    scripts.scripts[1].j = 100;
    scripts.scripts[1].k = 200;

    assert_eq!(resolve_value("J", &variables, &scripts, 1), 100);
    assert_eq!(resolve_value("K", &variables, &scripts, 1), 200);
}

#[test]
fn test_all_variables_in_expressions() {
    let mut variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let mut counters = create_test_counters();

    variables.a = 1;
    variables.b = 2;
    variables.c = 3;
    variables.d = 4;
    variables.i = 5;
    variables.x = 6;
    variables.y = 7;
    variables.z = 8;
    variables.t = 9;

    let parts = vec!["A"];
    assert_eq!(eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &create_test_scale()).unwrap().0, 1);

    let parts = vec!["B"];
    assert_eq!(eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &create_test_scale()).unwrap().0, 2);

    let parts = vec!["C"];
    assert_eq!(eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &create_test_scale()).unwrap().0, 3);

    let parts = vec!["D"];
    assert_eq!(eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &create_test_scale()).unwrap().0, 4);

    let parts = vec!["I"];
    assert_eq!(eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &create_test_scale()).unwrap().0, 5);

    let parts = vec!["X"];
    assert_eq!(eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &create_test_scale()).unwrap().0, 6);

    let parts = vec!["Y"];
    assert_eq!(eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &create_test_scale()).unwrap().0, 7);

    let parts = vec!["Z"];
    assert_eq!(eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &create_test_scale()).unwrap().0, 8);

    let parts = vec!["T"];
    assert_eq!(eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &create_test_scale()).unwrap().0, 9);
}

#[test]
fn test_resolve_value_with_j_k() {
    let variables = create_test_variables();
    let mut scripts = create_test_scripts();

    scripts.scripts[2].j = 42;
    scripts.scripts[2].k = 84;

    assert_eq!(resolve_value("J", &variables, &scripts, 2), 42);
    assert_eq!(resolve_value("K", &variables, &scripts, 2), 84);
}

#[test]
fn test_semicolon_separated_expressions() {
    let mut variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let mut counters = create_test_counters();

    variables.a = 10;

    let parts = vec!["A"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &create_test_scale());
    assert_eq!(result.unwrap().0, 10);

    let parts = vec!["ADD", "A", "5"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &create_test_scale());
    assert_eq!(result.unwrap().0, 15);

    let parts = vec!["MUL", "A", "2"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &create_test_scale());
    assert_eq!(result.unwrap().0, 20);
}

#[test]
fn test_expression_with_all_parameter_types() {
    let mut variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let mut counters = create_test_counters();

    variables.a = 10;
    patterns.patterns[0].data[0] = 5;
    patterns.patterns[0].length = 1;
    patterns.patterns[0].index = 0;

    let parts = vec!["ADD", "A", "PN", "0"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &create_test_scale());
    assert_eq!(result.unwrap().0, 15);

    let parts = vec!["ADD", "10", "20"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &create_test_scale());
    assert_eq!(result.unwrap().0, 30);
}

#[test]
fn test_deeply_nested_expressions() {
    let mut variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let mut counters = create_test_counters();

    variables.a = 2;
    variables.b = 3;

    let parts = vec!["MUL", "ADD", "A", "B", "SUB", "10", "5"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &create_test_scale());
    assert!(result.is_some());
    let (value, consumed) = result.unwrap();
    assert_eq!(value, 25);
    assert_eq!(consumed, 7);
}

#[test]
fn test_triple_nested_expressions() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let mut counters = create_test_counters();

    let parts = vec!["ADD", "MUL", "2", "3", "DIV", "20", "SUB", "7", "2"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &create_test_scale());
    assert_eq!(result.unwrap().0, 10);
}

#[test]
fn test_saturating_arithmetic() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let mut counters = create_test_counters();

    let parts = vec!["ADD", "32000", "32000"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &create_test_scale());
    assert!(result.is_some());

    let parts = vec!["MUL", "1000", "1000"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &create_test_scale());
    assert!(result.is_some());
}

#[test]
fn test_negative_numbers_in_expressions() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let mut counters = create_test_counters();

    let parts = vec!["-10"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &create_test_scale());
    assert_eq!(result.unwrap().0, -10);

    let parts = vec!["ADD", "-5", "10"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &create_test_scale());
    assert_eq!(result.unwrap().0, 5);

    let parts = vec!["SUB", "10", "-5"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &create_test_scale());
    assert_eq!(result.unwrap().0, 15);

    let parts = vec!["MUL", "-3", "4"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &create_test_scale());
    assert_eq!(result.unwrap().0, -12);
}

#[test]
fn test_n_zero_is_c3() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let mut counters = create_test_counters();
    let parts = vec!["N", "0"];

    let result = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &create_test_scale());
    assert!(result.is_some());
    let (value, consumed) = result.unwrap();
    assert_eq!(value, 131);
    assert_eq!(consumed, 2);
}

#[test]
fn test_n_12_is_c4() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let mut counters = create_test_counters();
    let parts = vec!["N", "12"];

    let result = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &create_test_scale());
    assert!(result.is_some());
    let (value, consumed) = result.unwrap();
    assert_eq!(value, 262);
    assert_eq!(consumed, 2);
}

#[test]
fn test_n_24_is_c5() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let mut counters = create_test_counters();
    let parts = vec!["N", "24"];

    let result = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &create_test_scale());
    assert!(result.is_some());
    let (value, consumed) = result.unwrap();
    assert_eq!(value, 523);
    assert_eq!(consumed, 2);
}

#[test]
fn test_n_negative_12_is_c2() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let mut counters = create_test_counters();
    let parts = vec!["N", "-12"];

    let result = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &create_test_scale());
    assert!(result.is_some());
    let (value, consumed) = result.unwrap();
    assert_eq!(value, 65);
    assert_eq!(consumed, 2);
}

#[test]
fn test_n_21_is_a4_440hz() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let mut counters = create_test_counters();
    let parts = vec!["N", "21"];

    let result = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &create_test_scale());
    assert!(result.is_some());
    let (value, consumed) = result.unwrap();
    assert_eq!(value, 440);
    assert_eq!(consumed, 2);
}

#[test]
fn test_n_with_variable() {
    let mut variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let mut counters = create_test_counters();
    variables.a = 12;
    let parts = vec!["N", "A"];

    let result = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &create_test_scale());
    assert!(result.is_some());
    let (value, consumed) = result.unwrap();
    assert_eq!(value, 262);
    assert_eq!(consumed, 2);
}

#[test]
fn test_n_nested_in_expression() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let mut counters = create_test_counters();
    let parts = vec!["ADD", "N", "0", "N", "12"];

    let result = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &create_test_scale());
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
    let mut counters = create_test_counters();
    let parts = vec!["N", "ADD", "0", "7"];

    let result = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0, &create_test_scale());
    assert!(result.is_some());
    let (value, consumed) = result.unwrap();
    assert_eq!(value, 196);
    assert_eq!(consumed, 4);
}
