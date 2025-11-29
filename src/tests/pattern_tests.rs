use crate::eval::eval_expression;
use super::common::{create_test_variables, create_test_patterns, create_test_scripts, create_test_counters};

#[test]
fn test_pattern_operations_with_expressions() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let mut counters = create_test_counters();

    patterns.patterns[0].data[0] = 100;
    patterns.patterns[0].data[1] = 200;
    patterns.patterns[0].data[2] = 300;
    patterns.patterns[0].length = 3;
    patterns.patterns[0].index = 0;

    let parts = vec!["PN", "0"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0);
    assert!(result.is_some());
    let (value, consumed) = result.unwrap();
    assert_eq!(value, 100);
    assert_eq!(consumed, 2);

    let parts = vec!["PN.I", "0"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0);
    assert!(result.is_some());
    let (value, consumed) = result.unwrap();
    assert_eq!(value, 0);
    assert_eq!(consumed, 2);

    let parts = vec!["PN.L", "0"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0);
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
    let mut counters = create_test_counters();

    patterns.patterns[0].data[0] = 10;
    patterns.patterns[0].data[1] = 20;
    patterns.patterns[0].data[2] = 30;
    patterns.patterns[0].length = 3;
    patterns.patterns[0].index = 0;
    patterns.working = 0;

    let parts = vec!["P.NEXT"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0);
    assert!(result.is_some());
    let (value, _) = result.unwrap();
    assert_eq!(value, 20);
    assert_eq!(patterns.patterns[0].index, 1);

    let result = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0);
    assert!(result.is_some());
    let (value, _) = result.unwrap();
    assert_eq!(value, 30);
    assert_eq!(patterns.patterns[0].index, 2);

    let result = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0);
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
    let mut counters = create_test_counters();

    patterns.patterns[0].data[0] = 10;
    patterns.patterns[0].data[1] = 20;
    patterns.patterns[0].data[2] = 30;
    patterns.patterns[0].length = 3;
    patterns.patterns[0].index = 0;
    patterns.working = 0;

    let parts = vec!["P.PREV"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0);
    assert!(result.is_some());
    let (value, _) = result.unwrap();
    assert_eq!(value, 30);
    assert_eq!(patterns.patterns[0].index, 2);

    let result = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0);
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
    let mut counters = create_test_counters();

    patterns.patterns[0].data[0] = 10;
    patterns.patterns[0].data[1] = 20;
    patterns.patterns[0].data[2] = 30;
    patterns.patterns[0].length = 3;
    patterns.patterns[0].index = 1;
    patterns.working = 0;

    let parts = vec!["P.HERE"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0);
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
    let mut counters = create_test_counters();

    patterns.patterns[1].data[0] = 100;
    patterns.patterns[1].data[1] = 200;
    patterns.patterns[1].length = 2;
    patterns.patterns[1].index = 0;

    let parts = vec!["PN.NEXT", "1"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0);
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
    let mut counters = create_test_counters();

    patterns.patterns[2].data[0] = 50;
    patterns.patterns[2].data[1] = 60;
    patterns.patterns[2].data[2] = 70;
    patterns.patterns[2].length = 3;
    patterns.patterns[2].index = 0;

    let parts = vec!["PN.PREV", "2"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0);
    assert!(result.is_some());
    let (value, consumed) = result.unwrap();
    assert_eq!(value, 70);
    assert_eq!(consumed, 2);
    assert_eq!(patterns.patterns[2].index, 2);
}

#[test]
fn test_pattern_operations_with_variable_indices() {
    let mut variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let mut counters = create_test_counters();

    patterns.patterns[0].data[0] = 100;
    patterns.patterns[1].data[0] = 200;
    patterns.patterns[2].data[0] = 300;
    patterns.patterns[3].data[0] = 400;

    variables.i = 0;
    let parts = vec!["PN", "0"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0);
    assert_eq!(result.unwrap().0, 100);

    let parts = vec!["PN", "1"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0);
    assert_eq!(result.unwrap().0, 200);

    let parts = vec!["PN", "2"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0);
    assert_eq!(result.unwrap().0, 300);

    let parts = vec!["PN", "3"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0);
    assert_eq!(result.unwrap().0, 400);
}

#[test]
fn test_pattern_length_wrapping() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let mut counters = create_test_counters();

    patterns.patterns[0].data[0] = 1;
    patterns.patterns[0].data[1] = 2;
    patterns.patterns[0].data[2] = 3;
    patterns.patterns[0].length = 3;
    patterns.patterns[0].index = 2;
    patterns.working = 0;

    let parts = vec!["P.NEXT"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0);
    assert_eq!(result.unwrap().0, 1);
    assert_eq!(patterns.patterns[0].index, 0);
}

#[test]
fn test_pattern_prev_wrapping() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let mut counters = create_test_counters();

    patterns.patterns[0].data[0] = 1;
    patterns.patterns[0].data[1] = 2;
    patterns.patterns[0].data[2] = 3;
    patterns.patterns[0].length = 3;
    patterns.patterns[0].index = 0;
    patterns.working = 0;

    let parts = vec!["P.PREV"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0);
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
fn test_pattern_next_with_expression_index() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let mut counters = create_test_counters();

    patterns.patterns[0].data[0] = 100;
    patterns.patterns[0].data[1] = 200;
    patterns.patterns[0].length = 2;
    patterns.patterns[0].index = 0;

    let parts = vec!["PN.NEXT", "ADD", "0", "0"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0);
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
    let mut counters = create_test_counters();

    patterns.patterns[0].data[0] = 100;
    patterns.patterns[1].data[0] = 200;
    patterns.patterns[1].data[1] = 250;
    patterns.patterns[1].data[2] = 300;
    patterns.patterns[1].length = 3;
    patterns.patterns[1].index = 2;

    let parts = vec!["PN", "ADD", "0", "1"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0);
    assert!(result.is_some());
    let (value, consumed) = result.unwrap();
    assert_eq!(value, 300);
    assert_eq!(consumed, 4);
}

#[test]
fn test_nested_pattern_operations() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let mut counters = create_test_counters();

    patterns.patterns[0].data[0] = 1;
    patterns.patterns[0].data[1] = 2;
    patterns.patterns[0].length = 2;
    patterns.patterns[0].index = 0;

    patterns.patterns[1].data[0] = 10;
    patterns.patterns[1].data[1] = 20;
    patterns.patterns[1].length = 2;
    patterns.patterns[1].index = 1;

    let parts = vec!["ADD", "PN", "0", "PN", "1"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0);
    assert!(result.is_some());
    let (value, consumed) = result.unwrap();
    assert_eq!(value, 21);
    assert_eq!(consumed, 5);
}

#[test]
fn test_pattern_operations_bounds() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let mut counters = create_test_counters();

    let parts = vec!["PN", "4"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0);
    assert!(result.is_none());

    let parts = vec!["PN.NEXT", "5"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0);
    assert!(result.is_none());
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
fn test_deeply_nested_pattern_and_math() {
    let mut variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let mut counters = create_test_counters();

    patterns.patterns[0].data[0] = 100;
    patterns.patterns[0].data[1] = 200;
    patterns.patterns[0].length = 2;
    patterns.patterns[0].index = 0;

    variables.a = 5;

    let parts = vec!["ADD", "MUL", "PN.HERE", "0", "A", "10"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0);
    assert_eq!(result.unwrap().0, 510);
}

#[test]
fn test_pattern_p_next_advances_index() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let mut counters = create_test_counters();

    patterns.patterns[0].data[0] = 100;
    patterns.patterns[0].data[1] = 200;
    patterns.patterns[0].data[2] = 300;
    patterns.patterns[0].length = 3;
    patterns.patterns[0].index = 0;
    patterns.working = 0;

    let parts = vec!["P.NEXT"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0);
    assert_eq!(result.unwrap().0, 200);
    assert_eq!(patterns.patterns[0].index, 1);

    let result = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0);
    assert_eq!(result.unwrap().0, 300);
    assert_eq!(patterns.patterns[0].index, 2);
}

#[test]
fn test_pattern_pn_next_with_index() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let mut counters = create_test_counters();

    patterns.patterns[1].data[0] = 50;
    patterns.patterns[1].data[1] = 75;
    patterns.patterns[1].length = 2;
    patterns.patterns[1].index = 0;

    let parts = vec!["PN.NEXT", "1"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0);
    assert_eq!(result.unwrap().0, 75);
    assert_eq!(patterns.patterns[1].index, 1);
}

#[test]
fn test_pattern_here_doesnt_change_index() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let mut counters = create_test_counters();

    patterns.patterns[0].data[0] = 10;
    patterns.patterns[0].data[1] = 20;
    patterns.patterns[0].length = 2;
    patterns.patterns[0].index = 1;
    patterns.working = 0;

    let parts = vec!["P.HERE"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0);
    assert_eq!(result.unwrap().0, 20);
    assert_eq!(patterns.patterns[0].index, 1);

    let result = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0);
    assert_eq!(result.unwrap().0, 20);
    assert_eq!(patterns.patterns[0].index, 1);
}

#[test]
fn test_all_pattern_operations() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let mut counters = create_test_counters();

    patterns.patterns[1].data[0] = 100;
    patterns.patterns[1].data[1] = 200;
    patterns.patterns[1].data[2] = 300;
    patterns.patterns[1].length = 3;
    patterns.patterns[1].index = 1;

    let parts = vec!["PN", "1"];
    assert_eq!(eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0).unwrap().0, 200);

    let parts = vec!["PN.HERE", "1"];
    assert_eq!(eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0).unwrap().0, 200);

    let parts = vec!["PN.L", "1"];
    assert_eq!(eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0).unwrap().0, 3);

    let parts = vec!["PN.I", "1"];
    assert_eq!(eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0).unwrap().0, 1);

    let parts = vec!["PN.NEXT", "1"];
    assert_eq!(eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0).unwrap().0, 300);

    let parts = vec!["PN.PREV", "1"];
    assert_eq!(eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0).unwrap().0, 200);
}

#[test]
fn test_complex_nested_pattern_and_math() {
    let mut variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let mut counters = create_test_counters();

    patterns.patterns[0].data[0] = 5;
    patterns.patterns[1].data[0] = 10;
    variables.a = 2;

    let parts = vec!["ADD", "MUL", "PN", "0", "A", "DIV", "PN", "1", "2"];
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0);
    assert_eq!(result.unwrap().0, 15);
}
