use crate::eval::eval_expression;
use crate::tests::common::{create_test_variables, create_test_patterns, create_test_scripts, create_test_counters};

#[test]
fn test_p_push_basic() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let mut counters = create_test_counters();

    patterns.patterns[0].data = [1, 2, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    patterns.patterns[0].length = 4;
    patterns.working = 0;

    assert_eq!(patterns.patterns[0].data[0], 1);
    assert_eq!(patterns.patterns[0].data[1], 2);
    assert_eq!(patterns.patterns[0].data[2], 3);
    assert_eq!(patterns.patterns[0].data[3], 0);
}

#[test]
fn test_p_push_with_expression() {
    let mut variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let mut counters = create_test_counters();

    variables.a = 50;
    patterns.patterns[0].data = [1, 2, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    patterns.patterns[0].length = 3;
    patterns.working = 0;

    assert_eq!(patterns.patterns[0].data[0], 1);
    assert_eq!(patterns.patterns[0].data[1], 2);
    assert_eq!(patterns.patterns[0].data[2], 3);
}

#[test]
fn test_p_pop_basic() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let mut counters = create_test_counters();

    patterns.patterns[0].data = [10, 20, 30, 40, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    patterns.patterns[0].length = 4;
    patterns.working = 0;

    assert_eq!(patterns.patterns[0].data[3], 40);
}

#[test]
fn test_p_ins_basic() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let mut counters = create_test_counters();

    patterns.patterns[0].data = [1, 2, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    patterns.patterns[0].length = 4;
    patterns.working = 0;

    assert_eq!(patterns.patterns[0].data[0], 1);
    assert_eq!(patterns.patterns[0].data[1], 2);
    assert_eq!(patterns.patterns[0].data[2], 3);
}

#[test]
fn test_p_ins_with_expression() {
    let mut variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let mut counters = create_test_counters();

    variables.a = 1;
    variables.b = 99;
    patterns.patterns[0].data = [1, 2, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    patterns.patterns[0].length = 4;
    patterns.working = 0;

    assert_eq!(patterns.patterns[0].data[0], 1);
    assert_eq!(patterns.patterns[0].data[1], 2);
    assert_eq!(patterns.patterns[0].data[2], 3);
}

#[test]
fn test_p_rm_basic() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let mut counters = create_test_counters();

    patterns.patterns[0].data = [10, 20, 30, 40, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    patterns.patterns[0].length = 4;
    patterns.working = 0;

    assert_eq!(patterns.patterns[0].data[0], 10);
    assert_eq!(patterns.patterns[0].data[1], 20);
    assert_eq!(patterns.patterns[0].data[2], 30);
    assert_eq!(patterns.patterns[0].data[3], 40);
}

#[test]
fn test_p_rm_with_expression() {
    let mut variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let mut counters = create_test_counters();

    variables.a = 2;
    patterns.patterns[0].data = [10, 20, 30, 40, 50, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    patterns.patterns[0].length = 5;
    patterns.working = 0;

    assert_eq!(patterns.patterns[0].data[0], 10);
    assert_eq!(patterns.patterns[0].data[1], 20);
    assert_eq!(patterns.patterns[0].data[2], 30);
    assert_eq!(patterns.patterns[0].data[3], 40);
    assert_eq!(patterns.patterns[0].data[4], 50);
}
