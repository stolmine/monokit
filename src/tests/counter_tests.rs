use crate::eval::eval_expression;
use super::common::{create_test_variables, create_test_patterns, create_test_scripts, create_test_counters};

#[test]
fn test_n1_increments_on_each_read() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let mut counters = create_test_counters();
    let parts = vec!["N1"];

    let result1 = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0);
    assert!(result1.is_some());
    let (value1, consumed1) = result1.unwrap();
    assert_eq!(value1, 0, "First N1 read should return 0");
    assert_eq!(consumed1, 1);

    let result2 = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0);
    assert!(result2.is_some());
    let (value2, consumed2) = result2.unwrap();
    assert_eq!(value2, 1, "Second N1 read should return 1");
    assert_eq!(consumed2, 1);

    let result3 = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0);
    assert!(result3.is_some());
    let (value3, consumed3) = result3.unwrap();
    assert_eq!(value3, 2, "Third N1 read should return 2");
    assert_eq!(consumed3, 1);
}

#[test]
fn test_n1_wraps_at_max() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let mut counters = create_test_counters();

    counters.max[0] = 3;

    let parts = vec!["N1"];

    let result1 = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0);
    assert_eq!(result1.unwrap().0, 0);

    let result2 = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0);
    assert_eq!(result2.unwrap().0, 1);

    let result3 = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0);
    assert_eq!(result3.unwrap().0, 2);

    let result4 = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0);
    assert_eq!(result4.unwrap().0, 0, "N1 should wrap back to 0 when reaching max of 3");
}

#[test]
fn test_n2_increments_independently() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let mut counters = create_test_counters();

    let parts_n1 = vec!["N1"];
    let parts_n2 = vec!["N2"];

    assert_eq!(eval_expression(&parts_n1, 0, &variables, &mut patterns, &mut counters, &scripts, 0).unwrap().0, 0);
    assert_eq!(eval_expression(&parts_n2, 0, &variables, &mut patterns, &mut counters, &scripts, 0).unwrap().0, 0);
    assert_eq!(eval_expression(&parts_n1, 0, &variables, &mut patterns, &mut counters, &scripts, 0).unwrap().0, 1);
    assert_eq!(eval_expression(&parts_n2, 0, &variables, &mut patterns, &mut counters, &scripts, 0).unwrap().0, 1);
    assert_eq!(eval_expression(&parts_n2, 0, &variables, &mut patterns, &mut counters, &scripts, 0).unwrap().0, 2);
    assert_eq!(eval_expression(&parts_n1, 0, &variables, &mut patterns, &mut counters, &scripts, 0).unwrap().0, 2);
}

#[test]
fn test_n3_increments_independently() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let mut counters = create_test_counters();

    let parts = vec!["N3"];

    assert_eq!(eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0).unwrap().0, 0);
    assert_eq!(eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0).unwrap().0, 1);
    assert_eq!(eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0).unwrap().0, 2);
}

#[test]
fn test_n4_increments_independently() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let mut counters = create_test_counters();

    let parts = vec!["N4"];

    assert_eq!(eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0).unwrap().0, 0);
    assert_eq!(eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0).unwrap().0, 1);
    assert_eq!(eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0).unwrap().0, 2);
}

#[test]
fn test_all_counters_are_independent() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let mut counters = create_test_counters();

    let parts_n1 = vec!["N1"];
    let parts_n2 = vec!["N2"];
    let parts_n3 = vec!["N3"];
    let parts_n4 = vec!["N4"];

    assert_eq!(eval_expression(&parts_n1, 0, &variables, &mut patterns, &mut counters, &scripts, 0).unwrap().0, 0);
    assert_eq!(eval_expression(&parts_n2, 0, &variables, &mut patterns, &mut counters, &scripts, 0).unwrap().0, 0);
    assert_eq!(eval_expression(&parts_n3, 0, &variables, &mut patterns, &mut counters, &scripts, 0).unwrap().0, 0);
    assert_eq!(eval_expression(&parts_n4, 0, &variables, &mut patterns, &mut counters, &scripts, 0).unwrap().0, 0);

    assert_eq!(eval_expression(&parts_n1, 0, &variables, &mut patterns, &mut counters, &scripts, 0).unwrap().0, 1);
    assert_eq!(eval_expression(&parts_n1, 0, &variables, &mut patterns, &mut counters, &scripts, 0).unwrap().0, 2);

    assert_eq!(eval_expression(&parts_n2, 0, &variables, &mut patterns, &mut counters, &scripts, 0).unwrap().0, 1);

    assert_eq!(eval_expression(&parts_n3, 0, &variables, &mut patterns, &mut counters, &scripts, 0).unwrap().0, 1);
    assert_eq!(eval_expression(&parts_n3, 0, &variables, &mut patterns, &mut counters, &scripts, 0).unwrap().0, 2);
    assert_eq!(eval_expression(&parts_n3, 0, &variables, &mut patterns, &mut counters, &scripts, 0).unwrap().0, 3);

    assert_eq!(eval_expression(&parts_n4, 0, &variables, &mut patterns, &mut counters, &scripts, 0).unwrap().0, 1);
}

#[test]
fn test_counter_wrapping_with_different_max_values() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let mut counters = create_test_counters();

    counters.max[0] = 2;
    counters.max[1] = 5;
    counters.max[2] = 3;

    let parts_n1 = vec!["N1"];
    let parts_n2 = vec!["N2"];
    let parts_n3 = vec!["N3"];

    assert_eq!(eval_expression(&parts_n1, 0, &variables, &mut patterns, &mut counters, &scripts, 0).unwrap().0, 0);
    assert_eq!(eval_expression(&parts_n1, 0, &variables, &mut patterns, &mut counters, &scripts, 0).unwrap().0, 1);
    assert_eq!(eval_expression(&parts_n1, 0, &variables, &mut patterns, &mut counters, &scripts, 0).unwrap().0, 0, "N1 wraps at max 2");

    for i in 0..5 {
        let result = eval_expression(&parts_n2, 0, &variables, &mut patterns, &mut counters, &scripts, 0).unwrap().0;
        assert_eq!(result, i, "N2 should count 0..4");
    }
    assert_eq!(eval_expression(&parts_n2, 0, &variables, &mut patterns, &mut counters, &scripts, 0).unwrap().0, 0, "N2 wraps at max 5");

    assert_eq!(eval_expression(&parts_n3, 0, &variables, &mut patterns, &mut counters, &scripts, 0).unwrap().0, 0);
    assert_eq!(eval_expression(&parts_n3, 0, &variables, &mut patterns, &mut counters, &scripts, 0).unwrap().0, 1);
    assert_eq!(eval_expression(&parts_n3, 0, &variables, &mut patterns, &mut counters, &scripts, 0).unwrap().0, 2);
    assert_eq!(eval_expression(&parts_n3, 0, &variables, &mut patterns, &mut counters, &scripts, 0).unwrap().0, 0, "N3 wraps at max 3");
}

#[test]
fn test_counter_without_max_wraps_at_i16_max() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let mut counters = create_test_counters();

    counters.max[0] = 0;
    counters.values[0] = 32766;

    let parts = vec!["N1"];

    assert_eq!(eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0).unwrap().0, 32766);
    assert_eq!(eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0).unwrap().0, 32767);
    let result = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0).unwrap().0;
    assert!(result < 32767, "Counter should wrap using wrapping_add when max is 0");
}

#[test]
fn test_counters_in_expressions() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let mut counters = create_test_counters();

    let parts = vec!["ADD", "N1", "N2"];

    let result1 = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0);
    assert_eq!(result1.unwrap().0, 0, "N1(0) + N2(0) = 0");

    let result2 = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0);
    assert_eq!(result2.unwrap().0, 2, "N1(1) + N2(1) = 2");

    let result3 = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0);
    assert_eq!(result3.unwrap().0, 4, "N1(2) + N2(2) = 4");
}
