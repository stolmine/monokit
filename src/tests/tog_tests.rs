use crate::eval::eval_expression;
use super::common::{create_test_variables, create_test_patterns, create_test_scripts, create_test_counters};

#[test]
fn test_tog_toggles_between_two_values() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let mut counters = create_test_counters();
    let parts = vec!["TOG", "100", "200"];

    let result1 = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0);
    assert!(result1.is_some());
    let (value1, consumed1) = result1.unwrap();
    assert_eq!(value1, 100, "First TOG call should return first value");
    assert_eq!(consumed1, 3);

    let result2 = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0);
    assert!(result2.is_some());
    let (value2, consumed2) = result2.unwrap();
    assert_eq!(value2, 200, "Second TOG call should return second value");
    assert_eq!(consumed2, 3);

    let result3 = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0);
    assert!(result3.is_some());
    let (value3, consumed3) = result3.unwrap();
    assert_eq!(value3, 100, "Third TOG call should return first value again");
    assert_eq!(consumed3, 3);

    let result4 = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0);
    assert!(result4.is_some());
    let (value4, _) = result4.unwrap();
    assert_eq!(value4, 200, "Fourth TOG call should return second value again");
}

#[test]
fn test_tog_with_variables() {
    let mut variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let mut counters = create_test_counters();

    variables.a = 50;
    variables.b = 75;
    let parts = vec!["TOG", "A", "B"];

    let result1 = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0);
    assert!(result1.is_some());
    let (value1, consumed1) = result1.unwrap();
    assert_eq!(value1, 50, "First TOG A B should return A value");
    assert_eq!(consumed1, 3);

    let result2 = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0);
    assert!(result2.is_some());
    let (value2, _) = result2.unwrap();
    assert_eq!(value2, 75, "Second TOG A B should return B value");
}

#[test]
fn test_tog_in_expression_context() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let mut counters = create_test_counters();
    let parts = vec!["MUL", "TOG", "2", "3", "5"];

    let result1 = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0);
    assert!(result1.is_some());
    let (value1, consumed1) = result1.unwrap();
    assert_eq!(value1, 10, "MUL (TOG 2 3) 5 first call should be 2 * 5 = 10");
    assert_eq!(consumed1, 5);

    let result2 = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0);
    assert!(result2.is_some());
    let (value2, consumed2) = result2.unwrap();
    assert_eq!(value2, 15, "MUL (TOG 2 3) 5 second call should be 3 * 5 = 15");
    assert_eq!(consumed2, 5);

    let result3 = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0);
    assert!(result3.is_some());
    let (value3, _) = result3.unwrap();
    assert_eq!(value3, 10, "MUL (TOG 2 3) 5 third call should cycle back to 2 * 5 = 10");
}

#[test]
fn test_tog_independent_instances() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let mut counters = create_test_counters();

    let parts1 = vec!["TOG", "1", "2"];
    let parts2 = vec!["TOG", "10", "20"];

    let result1a = eval_expression(&parts1, 0, &variables, &mut patterns, &mut counters, &scripts, 0);
    assert_eq!(result1a.unwrap().0, 1);

    let result2a = eval_expression(&parts2, 0, &variables, &mut patterns, &mut counters, &scripts, 0);
    assert_eq!(result2a.unwrap().0, 10);

    let result1b = eval_expression(&parts1, 0, &variables, &mut patterns, &mut counters, &scripts, 0);
    assert_eq!(result1b.unwrap().0, 2, "TOG 1 2 should be independent and return 2");

    let result2b = eval_expression(&parts2, 0, &variables, &mut patterns, &mut counters, &scripts, 0);
    assert_eq!(result2b.unwrap().0, 20, "TOG 10 20 should be independent and return 20");
}

#[test]
fn test_tog_per_script_independence() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let mut counters = create_test_counters();
    let parts = vec!["TOG", "100", "200"];

    let result1 = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0);
    assert_eq!(result1.unwrap().0, 100);

    let result2 = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 1);
    assert_eq!(result2.unwrap().0, 100, "Different script index should have independent state");

    let result3 = eval_expression(&parts, 0, &variables, &mut patterns, &mut counters, &scripts, 0);
    assert_eq!(result3.unwrap().0, 200, "Script 0 should continue its own toggle sequence");
}
