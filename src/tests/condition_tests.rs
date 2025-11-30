use crate::eval::eval_condition;
use super::common::{create_test_variables, create_test_patterns, create_test_scripts, create_test_counters, create_test_scale};

#[test]
fn test_eval_condition_simple_comparisons() {
    let mut variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let mut counters = create_test_counters();
    variables.a = 10;
    variables.b = 5;

    assert_eq!(eval_condition("IF A >= B", &variables, &mut patterns, &mut counters, &scripts, 0, &create_test_scale()), true);
    assert_eq!(eval_condition("IF A <= B", &variables, &mut patterns, &mut counters, &scripts, 0, &create_test_scale()), false);
    assert_eq!(eval_condition("IF A != B", &variables, &mut patterns, &mut counters, &scripts, 0, &create_test_scale()), true);
    assert_eq!(eval_condition("IF B != B", &variables, &mut patterns, &mut counters, &scripts, 0, &create_test_scale()), false);

    variables.a = 5;
    assert_eq!(eval_condition("IF A >= B", &variables, &mut patterns, &mut counters, &scripts, 0, &create_test_scale()), true);
    assert_eq!(eval_condition("IF A <= B", &variables, &mut patterns, &mut counters, &scripts, 0, &create_test_scale()), true);
    assert_eq!(eval_condition("IF A != B", &variables, &mut patterns, &mut counters, &scripts, 0, &create_test_scale()), false);
}

#[test]
fn test_eval_condition_with_literals() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let mut counters = create_test_counters();

    assert_eq!(eval_condition("IF 10 >= 5", &variables, &mut patterns, &mut counters, &scripts, 0, &create_test_scale()), true);
    assert_eq!(eval_condition("IF 5 <= 10", &variables, &mut patterns, &mut counters, &scripts, 0, &create_test_scale()), true);
    assert_eq!(eval_condition("IF 5 != 10", &variables, &mut patterns, &mut counters, &scripts, 0, &create_test_scale()), true);
    assert_eq!(eval_condition("IF 5 != 5", &variables, &mut patterns, &mut counters, &scripts, 0, &create_test_scale()), false);
}

#[test]
fn test_if_condition_true_executes() {
    let mut variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let mut counters = create_test_counters();
    variables.a = 10;

    assert_eq!(eval_condition("IF A == 10", &variables, &mut patterns, &mut counters, &scripts, 0, &create_test_scale()), true);
    assert_eq!(eval_condition("IF A > 5", &variables, &mut patterns, &mut counters, &scripts, 0, &create_test_scale()), true);
    assert_eq!(eval_condition("IF A >= 10", &variables, &mut patterns, &mut counters, &scripts, 0, &create_test_scale()), true);
}

#[test]
fn test_if_condition_false_skips() {
    let mut variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let mut counters = create_test_counters();
    variables.a = 5;

    assert_eq!(eval_condition("IF A == 10", &variables, &mut patterns, &mut counters, &scripts, 0, &create_test_scale()), false);
    assert_eq!(eval_condition("IF A > 10", &variables, &mut patterns, &mut counters, &scripts, 0, &create_test_scale()), false);
    assert_eq!(eval_condition("IF A < 5", &variables, &mut patterns, &mut counters, &scripts, 0, &create_test_scale()), false);
}

#[test]
fn test_if_with_nested_expressions() {
    let mut variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let mut counters = create_test_counters();
    variables.a = 5;
    variables.b = 3;

    assert_eq!(eval_condition("IF ADD A B == 8", &variables, &mut patterns, &mut counters, &scripts, 0, &create_test_scale()), true);
    assert_eq!(eval_condition("IF MUL A B == 15", &variables, &mut patterns, &mut counters, &scripts, 0, &create_test_scale()), true);
    assert_eq!(eval_condition("IF SUB A B == 2", &variables, &mut patterns, &mut counters, &scripts, 0, &create_test_scale()), true);
}

#[test]
fn test_nested_math_in_conditions() {
    let mut variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let mut counters = create_test_counters();
    variables.a = 10;

    assert_eq!(eval_condition("IF ADD A 5 >= 15", &variables, &mut patterns, &mut counters, &scripts, 0, &create_test_scale()), true);
    assert_eq!(eval_condition("IF MUL ADD A 2 2 == 24", &variables, &mut patterns, &mut counters, &scripts, 0, &create_test_scale()), true);
}

#[test]
fn test_pattern_ops_in_conditions() {
    let mut variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let mut counters = create_test_counters();

    patterns.patterns[0].data[0] = 100;
    patterns.patterns[0].length = 1;
    patterns.patterns[0].index = 0;

    assert_eq!(eval_condition("IF PN 0 == 100", &variables, &mut patterns, &mut counters, &scripts, 0, &create_test_scale()), true);
    assert_eq!(eval_condition("IF PN 0 >= 50", &variables, &mut patterns, &mut counters, &scripts, 0, &create_test_scale()), true);
    assert_eq!(eval_condition("IF PN 0 != 0", &variables, &mut patterns, &mut counters, &scripts, 0, &create_test_scale()), true);
}

#[test]
fn test_prob_condition_always_in_range() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let mut counters = create_test_counters();

    for _ in 0..20 {
        let result = eval_condition("PROB 50", &variables, &mut patterns, &mut counters, &scripts, 0, &create_test_scale());
        assert!(result == true || result == false);
    }

    for _ in 0..10 {
        let result = eval_condition("PROB 100", &variables, &mut patterns, &mut counters, &scripts, 0, &create_test_scale());
        assert_eq!(result, true);
    }

    for _ in 0..10 {
        let result = eval_condition("PROB 0", &variables, &mut patterns, &mut counters, &scripts, 0, &create_test_scale());
        assert_eq!(result, false);
    }
}

#[test]
fn test_prob_with_expression() {
    let mut variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let mut counters = create_test_counters();

    variables.a = 50;

    for _ in 0..20 {
        let result = eval_condition("PROB A", &variables, &mut patterns, &mut counters, &scripts, 0, &create_test_scale());
        assert!(result == true || result == false);
    }
}

#[test]
fn test_condition_with_add_expression() {
    let mut variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let mut counters = create_test_counters();

    variables.a = 10;

    assert_eq!(eval_condition("IF ADD A 1 > 0", &variables, &mut patterns, &mut counters, &scripts, 0, &create_test_scale()), true);
    assert_eq!(eval_condition("IF ADD A 1 >= 11", &variables, &mut patterns, &mut counters, &scripts, 0, &create_test_scale()), true);
    assert_eq!(eval_condition("IF ADD A 1 <= 11", &variables, &mut patterns, &mut counters, &scripts, 0, &create_test_scale()), true);
    assert_eq!(eval_condition("IF ADD A 1 == 11", &variables, &mut patterns, &mut counters, &scripts, 0, &create_test_scale()), true);
}

#[test]
fn test_condition_with_mul_expression() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let mut counters = create_test_counters();

    assert_eq!(eval_condition("IF MUL 2 3 == 6", &variables, &mut patterns, &mut counters, &scripts, 0, &create_test_scale()), true);
    assert_eq!(eval_condition("IF MUL 2 3 != 5", &variables, &mut patterns, &mut counters, &scripts, 0, &create_test_scale()), true);
    assert_eq!(eval_condition("IF MUL 2 3 >= 6", &variables, &mut patterns, &mut counters, &scripts, 0, &create_test_scale()), true);
    assert_eq!(eval_condition("IF MUL 2 3 <= 6", &variables, &mut patterns, &mut counters, &scripts, 0, &create_test_scale()), true);
}

#[test]
fn test_condition_comparison_operators() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let mut counters = create_test_counters();

    assert_eq!(eval_condition("IF 10 > 5", &variables, &mut patterns, &mut counters, &scripts, 0, &create_test_scale()), true);
    assert_eq!(eval_condition("IF 10 >= 10", &variables, &mut patterns, &mut counters, &scripts, 0, &create_test_scale()), true);
    assert_eq!(eval_condition("IF 5 < 10", &variables, &mut patterns, &mut counters, &scripts, 0, &create_test_scale()), true);
    assert_eq!(eval_condition("IF 10 <= 10", &variables, &mut patterns, &mut counters, &scripts, 0, &create_test_scale()), true);
    assert_eq!(eval_condition("IF 10 == 10", &variables, &mut patterns, &mut counters, &scripts, 0, &create_test_scale()), true);
    assert_eq!(eval_condition("IF 10 != 5", &variables, &mut patterns, &mut counters, &scripts, 0, &create_test_scale()), true);

    assert_eq!(eval_condition("IF 5 > 10", &variables, &mut patterns, &mut counters, &scripts, 0, &create_test_scale()), false);
    assert_eq!(eval_condition("IF 5 >= 10", &variables, &mut patterns, &mut counters, &scripts, 0, &create_test_scale()), false);
    assert_eq!(eval_condition("IF 10 < 5", &variables, &mut patterns, &mut counters, &scripts, 0, &create_test_scale()), false);
    assert_eq!(eval_condition("IF 10 <= 5", &variables, &mut patterns, &mut counters, &scripts, 0, &create_test_scale()), false);
    assert_eq!(eval_condition("IF 10 == 5", &variables, &mut patterns, &mut counters, &scripts, 0, &create_test_scale()), false);
    assert_eq!(eval_condition("IF 10 != 10", &variables, &mut patterns, &mut counters, &scripts, 0, &create_test_scale()), false);
}

#[test]
fn test_condition_with_negative_numbers() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let mut counters = create_test_counters();

    assert_eq!(eval_condition("IF -5 < 0", &variables, &mut patterns, &mut counters, &scripts, 0, &create_test_scale()), true);
    assert_eq!(eval_condition("IF -10 < -5", &variables, &mut patterns, &mut counters, &scripts, 0, &create_test_scale()), true);
    assert_eq!(eval_condition("IF -5 > -10", &variables, &mut patterns, &mut counters, &scripts, 0, &create_test_scale()), true);
    assert_eq!(eval_condition("IF -5 == -5", &variables, &mut patterns, &mut counters, &scripts, 0, &create_test_scale()), true);
}

#[test]
fn test_if_truthy_nonzero_is_true() {
    let mut variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let mut counters = create_test_counters();

    variables.a = 42;
    assert!(eval_condition("IF A", &variables, &mut patterns, &mut counters, &scripts, 0, &create_test_scale()));

    variables.a = -10;
    assert!(eval_condition("IF A", &variables, &mut patterns, &mut counters, &scripts, 0, &create_test_scale()));
}

#[test]
fn test_if_falsy_zero_is_false() {
    let mut variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let mut counters = create_test_counters();

    variables.a = 0;
    assert!(!eval_condition("IF A", &variables, &mut patterns, &mut counters, &scripts, 0, &create_test_scale()));
}

#[test]
fn test_if_with_pattern_value_truthy() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let mut counters = create_test_counters();
    patterns.patterns[0].data[0] = 42;
    assert!(eval_condition("IF PN 0", &variables, &mut patterns, &mut counters, &scripts, 0, &create_test_scale()));
}

#[test]
fn test_if_with_pattern_value_falsy() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let mut counters = create_test_counters();
    patterns.patterns[0].data[0] = 0;
    assert!(!eval_condition("IF PN 0", &variables, &mut patterns, &mut counters, &scripts, 0, &create_test_scale()));
}

#[test]
fn test_if_with_comparison_operators() {
    let mut variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let mut counters = create_test_counters();

    variables.a = 10;
    variables.b = 5;

    assert!(eval_condition("IF A > B", &variables, &mut patterns, &mut counters, &scripts, 0, &create_test_scale()));
    assert!(eval_condition("IF A >= B", &variables, &mut patterns, &mut counters, &scripts, 0, &create_test_scale()));
    assert!(eval_condition("IF B < A", &variables, &mut patterns, &mut counters, &scripts, 0, &create_test_scale()));
    assert!(eval_condition("IF B <= A", &variables, &mut patterns, &mut counters, &scripts, 0, &create_test_scale()));
    assert!(eval_condition("IF A == 10", &variables, &mut patterns, &mut counters, &scripts, 0, &create_test_scale()));
    assert!(eval_condition("IF A != B", &variables, &mut patterns, &mut counters, &scripts, 0, &create_test_scale()));

    assert!(!eval_condition("IF A < B", &variables, &mut patterns, &mut counters, &scripts, 0, &create_test_scale()));
    assert!(!eval_condition("IF A == B", &variables, &mut patterns, &mut counters, &scripts, 0, &create_test_scale()));
}

#[test]
fn test_if_with_mixed_pattern_values() {
    let mut variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let mut counters = create_test_counters();

    patterns.patterns[0].data[0] = 100;
    patterns.patterns[0].data[1] = 50;
    patterns.patterns[0].length = 2;
    patterns.patterns[0].index = 0;

    variables.a = 75;

    assert!(eval_condition("IF PN.HERE 0 > A", &variables, &mut patterns, &mut counters, &scripts, 0, &create_test_scale()));

    patterns.patterns[0].index = 1;
    assert!(eval_condition("IF PN.HERE 0 < A", &variables, &mut patterns, &mut counters, &scripts, 0, &create_test_scale()));
}

#[test]
fn test_nested_comparison_in_if() {
    let mut variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let mut counters = create_test_counters();

    variables.a = 5;
    variables.b = 3;

    assert!(eval_condition("IF ADD A B > 7", &variables, &mut patterns, &mut counters, &scripts, 0, &create_test_scale()));
    assert!(eval_condition("IF MUL A B >= 15", &variables, &mut patterns, &mut counters, &scripts, 0, &create_test_scale()));
}

#[test]
fn test_if_pn_here_after_pn_next() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let mut counters = create_test_counters();

    patterns.patterns[0].data[0] = 100;
    patterns.patterns[0].data[1] = 0;
    patterns.patterns[0].data[2] = 200;
    patterns.patterns[0].length = 3;
    patterns.patterns[0].index = 0;

    assert!(eval_condition("IF PN.HERE 0", &variables, &mut patterns, &mut counters, &scripts, 0, &create_test_scale()));

    patterns.patterns[0].index = 1;
    assert!(!eval_condition("IF PN.HERE 0", &variables, &mut patterns, &mut counters, &scripts, 0, &create_test_scale()));

    patterns.patterns[0].index = 2;
    assert!(eval_condition("IF PN.HERE 0", &variables, &mut patterns, &mut counters, &scripts, 0, &create_test_scale()));
}

#[test]
fn test_if_lowercase_pn_here() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let mut counters = create_test_counters();

    patterns.patterns[0].data[0] = 42;
    patterns.patterns[0].index = 0;

    assert!(eval_condition("IF pn.here 0", &variables, &mut patterns, &mut counters, &scripts, 0, &create_test_scale()));

    patterns.patterns[0].data[0] = 0;
    assert!(!eval_condition("IF pn.here 0", &variables, &mut patterns, &mut counters, &scripts, 0, &create_test_scale()));
}
