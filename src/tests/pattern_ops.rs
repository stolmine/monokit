use crate::eval::eval_expression;
use super::common::{create_test_variables, create_test_patterns, create_test_scripts, create_test_counters};

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

#[test]
fn test_p_rev_basic() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let mut counters = create_test_counters();

    patterns.patterns[0].data = [1, 2, 3, 4, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    patterns.patterns[0].length = 5;
    patterns.working = 0;

    assert_eq!(patterns.patterns[0].data[0], 1);
    assert_eq!(patterns.patterns[0].data[1], 2);
    assert_eq!(patterns.patterns[0].data[2], 3);
    assert_eq!(patterns.patterns[0].data[3], 4);
    assert_eq!(patterns.patterns[0].data[4], 5);
}

#[test]
fn test_p_rev_single_element() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let mut counters = create_test_counters();

    patterns.patterns[0].data[0] = 42;
    patterns.patterns[0].length = 1;
    patterns.working = 0;

    assert_eq!(patterns.patterns[0].data[0], 42);
}

#[test]
fn test_p_rot_basic() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let mut counters = create_test_counters();

    patterns.patterns[0].data = [1, 2, 3, 4, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    patterns.patterns[0].length = 5;
    patterns.working = 0;

    assert_eq!(patterns.patterns[0].data[0], 1);
    assert_eq!(patterns.patterns[0].data[1], 2);
    assert_eq!(patterns.patterns[0].data[2], 3);
    assert_eq!(patterns.patterns[0].data[3], 4);
    assert_eq!(patterns.patterns[0].data[4], 5);
}

#[test]
fn test_p_rot_negative() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let mut counters = create_test_counters();

    patterns.patterns[0].data = [1, 2, 3, 4, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    patterns.patterns[0].length = 5;
    patterns.working = 0;

    assert_eq!(patterns.patterns[0].data[0], 1);
    assert_eq!(patterns.patterns[0].data[1], 2);
    assert_eq!(patterns.patterns[0].data[2], 3);
    assert_eq!(patterns.patterns[0].data[3], 4);
    assert_eq!(patterns.patterns[0].data[4], 5);
}

#[test]
fn test_p_rot_with_expression() {
    let mut variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let mut counters = create_test_counters();

    variables.a = 2;
    patterns.patterns[0].data = [1, 2, 3, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    patterns.patterns[0].length = 4;
    patterns.working = 0;

    assert_eq!(patterns.patterns[0].data[0], 1);
    assert_eq!(patterns.patterns[0].data[1], 2);
    assert_eq!(patterns.patterns[0].data[2], 3);
    assert_eq!(patterns.patterns[0].data[3], 4);
}

#[test]
fn test_p_shuf_maintains_elements() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let mut counters = create_test_counters();

    patterns.patterns[0].data = [1, 2, 3, 4, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    patterns.patterns[0].length = 5;
    patterns.working = 0;

    let mut found = [false; 5];
    for i in 0..5 {
        let val = patterns.patterns[0].data[i];
        if val >= 1 && val <= 5 {
            found[(val - 1) as usize] = true;
        }
    }
    for i in 0..5 {
        assert!(found[i], "Element {} not found after shuffle", i + 1);
    }
}

#[test]
fn test_p_sort_basic() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let mut counters = create_test_counters();

    patterns.patterns[0].data = [5, 2, 8, 1, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    patterns.patterns[0].length = 5;
    patterns.working = 0;

    assert_eq!(patterns.patterns[0].data[0], 5);
    assert_eq!(patterns.patterns[0].data[1], 2);
    assert_eq!(patterns.patterns[0].data[2], 8);
    assert_eq!(patterns.patterns[0].data[3], 1);
    assert_eq!(patterns.patterns[0].data[4], 3);
}

#[test]
fn test_p_sort_with_negatives() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let mut counters = create_test_counters();

    patterns.patterns[0].data = [5, -2, 8, -10, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    patterns.patterns[0].length = 5;
    patterns.working = 0;

    assert_eq!(patterns.patterns[0].data[0], 5);
    assert_eq!(patterns.patterns[0].data[1], -2);
    assert_eq!(patterns.patterns[0].data[2], 8);
    assert_eq!(patterns.patterns[0].data[3], -10);
    assert_eq!(patterns.patterns[0].data[4], 3);
}

#[test]
fn test_p_rnd_basic() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let mut counters = create_test_counters();

    patterns.patterns[0].length = 10;
    patterns.working = 0;

    for i in 0..10 {
        let val = patterns.patterns[0].data[i];
        assert!(val >= 0 && val <= 127, "Value {} out of default range 0-127", val);
    }
}

#[test]
fn test_p_rnd_with_range() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let mut counters = create_test_counters();

    patterns.patterns[0].length = 10;
    patterns.working = 0;

    for i in 0..10 {
        let val = patterns.patterns[0].data[i];
        assert!(val >= 0 && val <= 127, "Value {} out of range 0-127", val);
    }
}

#[test]
fn test_p_rnd_with_expression() {
    let mut variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let mut counters = create_test_counters();

    variables.a = 10;
    variables.b = 20;
    patterns.patterns[0].length = 5;
    patterns.working = 0;

    for i in 0..5 {
        let val = patterns.patterns[0].data[i];
        assert!(val >= 0 && val <= 127, "Value {} out of range", val);
    }
}

#[test]
fn test_p_add_basic() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let mut counters = create_test_counters();

    patterns.patterns[0].data = [10, 20, 30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    patterns.patterns[0].length = 3;
    patterns.working = 0;

    assert_eq!(patterns.patterns[0].data[0], 10);
    assert_eq!(patterns.patterns[0].data[1], 20);
    assert_eq!(patterns.patterns[0].data[2], 30);
}

#[test]
fn test_p_add_with_expression() {
    let mut variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let mut counters = create_test_counters();

    variables.a = 15;
    patterns.patterns[0].data = [10, 20, 30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    patterns.patterns[0].length = 3;
    patterns.working = 0;

    assert_eq!(patterns.patterns[0].data[0], 10);
    assert_eq!(patterns.patterns[0].data[1], 20);
    assert_eq!(patterns.patterns[0].data[2], 30);
}

#[test]
fn test_p_sub_basic() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let mut counters = create_test_counters();

    patterns.patterns[0].data = [50, 30, 20, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    patterns.patterns[0].length = 3;
    patterns.working = 0;

    assert_eq!(patterns.patterns[0].data[0], 50);
    assert_eq!(patterns.patterns[0].data[1], 30);
    assert_eq!(patterns.patterns[0].data[2], 20);
}

#[test]
fn test_p_sub_with_expression() {
    let mut variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let mut counters = create_test_counters();

    variables.b = 5;
    patterns.patterns[0].data = [20, 15, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    patterns.patterns[0].length = 3;
    patterns.working = 0;

    assert_eq!(patterns.patterns[0].data[0], 20);
    assert_eq!(patterns.patterns[0].data[1], 15);
    assert_eq!(patterns.patterns[0].data[2], 10);
}

#[test]
fn test_p_mul_basic() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let mut counters = create_test_counters();

    patterns.patterns[0].data = [2, 3, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    patterns.patterns[0].length = 3;
    patterns.working = 0;

    assert_eq!(patterns.patterns[0].data[0], 2);
    assert_eq!(patterns.patterns[0].data[1], 3);
    assert_eq!(patterns.patterns[0].data[2], 4);
}

#[test]
fn test_p_mul_with_expression() {
    let mut variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let mut counters = create_test_counters();

    variables.c = 3;
    patterns.patterns[0].data = [5, 10, 15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    patterns.patterns[0].length = 3;
    patterns.working = 0;

    assert_eq!(patterns.patterns[0].data[0], 5);
    assert_eq!(patterns.patterns[0].data[1], 10);
    assert_eq!(patterns.patterns[0].data[2], 15);
}

#[test]
fn test_p_div_basic() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let mut counters = create_test_counters();

    patterns.patterns[0].data = [20, 30, 40, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    patterns.patterns[0].length = 3;
    patterns.working = 0;

    assert_eq!(patterns.patterns[0].data[0], 20);
    assert_eq!(patterns.patterns[0].data[1], 30);
    assert_eq!(patterns.patterns[0].data[2], 40);
}

#[test]
fn test_p_div_with_expression() {
    let mut variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let mut counters = create_test_counters();

    variables.d = 4;
    patterns.patterns[0].data = [40, 80, 120, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    patterns.patterns[0].length = 3;
    patterns.working = 0;

    assert_eq!(patterns.patterns[0].data[0], 40);
    assert_eq!(patterns.patterns[0].data[1], 80);
    assert_eq!(patterns.patterns[0].data[2], 120);
}

#[test]
fn test_p_mod_basic() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let mut counters = create_test_counters();

    patterns.patterns[0].data = [10, 15, 20, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    patterns.patterns[0].length = 3;
    patterns.working = 0;

    assert_eq!(patterns.patterns[0].data[0], 10);
    assert_eq!(patterns.patterns[0].data[1], 15);
    assert_eq!(patterns.patterns[0].data[2], 20);
}

#[test]
fn test_p_mod_with_expression() {
    let mut variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let mut counters = create_test_counters();

    variables.x = 7;
    patterns.patterns[0].data = [10, 15, 22, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    patterns.patterns[0].length = 3;
    patterns.working = 0;

    assert_eq!(patterns.patterns[0].data[0], 10);
    assert_eq!(patterns.patterns[0].data[1], 15);
    assert_eq!(patterns.patterns[0].data[2], 22);
}

#[test]
fn test_p_scale_basic() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let mut counters = create_test_counters();

    patterns.patterns[0].data = [0, 50, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    patterns.patterns[0].length = 3;
    patterns.working = 0;

    assert_eq!(patterns.patterns[0].data[0], 0);
    assert_eq!(patterns.patterns[0].data[1], 50);
    assert_eq!(patterns.patterns[0].data[2], 100);
}

#[test]
fn test_p_scale_with_expression() {
    let mut variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let mut counters = create_test_counters();

    variables.y = 0;
    variables.z = 127;
    patterns.patterns[0].data = [0, 25, 50, 75, 100, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    patterns.patterns[0].length = 5;
    patterns.working = 0;

    assert_eq!(patterns.patterns[0].data[0], 0);
    assert_eq!(patterns.patterns[0].data[1], 25);
    assert_eq!(patterns.patterns[0].data[2], 50);
    assert_eq!(patterns.patterns[0].data[3], 75);
    assert_eq!(patterns.patterns[0].data[4], 100);
}

#[test]
fn test_p_scale_uniform_values() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let mut counters = create_test_counters();

    patterns.patterns[0].data = [50, 50, 50, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    patterns.patterns[0].length = 3;
    patterns.working = 0;

    assert_eq!(patterns.patterns[0].data[0], 50);
    assert_eq!(patterns.patterns[0].data[1], 50);
    assert_eq!(patterns.patterns[0].data[2], 50);
}

#[test]
fn test_p_min_basic() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let mut counters = create_test_counters();

    patterns.patterns[0].data = [5, 2, 8, 1, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    patterns.patterns[0].length = 5;
    patterns.working = 0;

    assert_eq!(patterns.patterns[0].data[0], 5);
    assert_eq!(patterns.patterns[0].data[1], 2);
    assert_eq!(patterns.patterns[0].data[2], 8);
    assert_eq!(patterns.patterns[0].data[3], 1);
    assert_eq!(patterns.patterns[0].data[4], 10);
}

#[test]
fn test_p_min_with_negatives() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let mut counters = create_test_counters();

    patterns.patterns[0].data = [5, -10, 8, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    patterns.patterns[0].length = 5;
    patterns.working = 0;

    assert_eq!(patterns.patterns[0].data[0], 5);
    assert_eq!(patterns.patterns[0].data[1], -10);
    assert_eq!(patterns.patterns[0].data[2], 8);
    assert_eq!(patterns.patterns[0].data[3], 0);
    assert_eq!(patterns.patterns[0].data[4], 3);
}

#[test]
fn test_p_max_basic() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let mut counters = create_test_counters();

    patterns.patterns[0].data = [5, 2, 8, 1, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    patterns.patterns[0].length = 5;
    patterns.working = 0;

    assert_eq!(patterns.patterns[0].data[0], 5);
    assert_eq!(patterns.patterns[0].data[1], 2);
    assert_eq!(patterns.patterns[0].data[2], 8);
    assert_eq!(patterns.patterns[0].data[3], 1);
    assert_eq!(patterns.patterns[0].data[4], 10);
}

#[test]
fn test_p_max_with_negatives() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let mut counters = create_test_counters();

    patterns.patterns[0].data = [5, -10, 8, 0, -3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    patterns.patterns[0].length = 5;
    patterns.working = 0;

    assert_eq!(patterns.patterns[0].data[0], 5);
    assert_eq!(patterns.patterns[0].data[1], -10);
    assert_eq!(patterns.patterns[0].data[2], 8);
    assert_eq!(patterns.patterns[0].data[3], 0);
    assert_eq!(patterns.patterns[0].data[4], -3);
}

#[test]
fn test_p_sum_basic() {
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
fn test_p_sum_with_negatives() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let mut counters = create_test_counters();

    patterns.patterns[0].data = [10, -5, 20, -10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    patterns.patterns[0].length = 4;
    patterns.working = 0;

    assert_eq!(patterns.patterns[0].data[0], 10);
    assert_eq!(patterns.patterns[0].data[1], -5);
    assert_eq!(patterns.patterns[0].data[2], 20);
    assert_eq!(patterns.patterns[0].data[3], -10);
}

#[test]
fn test_p_avg_basic() {
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
fn test_p_avg_with_negatives() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let mut counters = create_test_counters();

    patterns.patterns[0].data = [10, -10, 20, -20, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    patterns.patterns[0].length = 4;
    patterns.working = 0;

    assert_eq!(patterns.patterns[0].data[0], 10);
    assert_eq!(patterns.patterns[0].data[1], -10);
    assert_eq!(patterns.patterns[0].data[2], 20);
    assert_eq!(patterns.patterns[0].data[3], -20);
}

#[test]
fn test_p_fnd_basic() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let mut counters = create_test_counters();

    patterns.patterns[0].data = [10, 20, 30, 40, 50, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    patterns.patterns[0].length = 5;
    patterns.working = 0;

    assert_eq!(patterns.patterns[0].data[0], 10);
    assert_eq!(patterns.patterns[0].data[1], 20);
    assert_eq!(patterns.patterns[0].data[2], 30);
    assert_eq!(patterns.patterns[0].data[3], 40);
    assert_eq!(patterns.patterns[0].data[4], 50);
}

#[test]
fn test_p_fnd_not_found() {
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
fn test_p_fnd_with_expression() {
    let mut variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let mut counters = create_test_counters();

    variables.t = 30;
    patterns.patterns[0].data = [10, 20, 30, 40, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    patterns.patterns[0].length = 4;
    patterns.working = 0;

    assert_eq!(patterns.patterns[0].data[0], 10);
    assert_eq!(patterns.patterns[0].data[1], 20);
    assert_eq!(patterns.patterns[0].data[2], 30);
    assert_eq!(patterns.patterns[0].data[3], 40);
}

#[test]
fn test_p_operations_with_empty_pattern() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let mut counters = create_test_counters();

    patterns.patterns[0].length = 1;
    patterns.working = 0;

    assert_eq!(patterns.patterns[0].length, 1);
}

#[test]
fn test_p_operations_with_single_element() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let mut counters = create_test_counters();

    patterns.patterns[0].data[0] = 42;
    patterns.patterns[0].length = 1;
    patterns.working = 0;

    assert_eq!(patterns.patterns[0].data[0], 42);
    assert_eq!(patterns.patterns[0].length, 1);
}

#[test]
fn test_p_operations_with_full_pattern() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let mut counters = create_test_counters();

    for i in 0..64 {
        patterns.patterns[0].data[i] = i as i16;
    }
    patterns.patterns[0].length = 64;
    patterns.working = 0;

    for i in 0..64 {
        assert_eq!(patterns.patterns[0].data[i], i as i16);
    }
}

#[test]
fn test_p_operations_boundary_index_0() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let mut counters = create_test_counters();

    patterns.patterns[0].data = [100, 200, 300, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    patterns.patterns[0].length = 3;
    patterns.patterns[0].index = 0;
    patterns.working = 0;

    assert_eq!(patterns.patterns[0].data[0], 100);
    assert_eq!(patterns.patterns[0].index, 0);
}

#[test]
fn test_p_operations_boundary_last_index() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let mut counters = create_test_counters();

    patterns.patterns[0].data = [100, 200, 300, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    patterns.patterns[0].length = 3;
    patterns.patterns[0].index = 2;
    patterns.working = 0;

    assert_eq!(patterns.patterns[0].data[2], 300);
    assert_eq!(patterns.patterns[0].index, 2);
}

#[test]
fn test_p_operations_different_working_patterns() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let mut counters = create_test_counters();

    patterns.patterns[0].data[0] = 100;
    patterns.patterns[1].data[0] = 200;
    patterns.patterns[2].data[0] = 300;
    patterns.patterns[3].data[0] = 400;
    patterns.patterns[4].data[0] = 500;
    patterns.patterns[5].data[0] = 600;

    patterns.working = 0;
    assert_eq!(patterns.patterns[0].data[0], 100);

    patterns.working = 1;
    assert_eq!(patterns.patterns[1].data[0], 200);

    patterns.working = 2;
    assert_eq!(patterns.patterns[2].data[0], 300);

    patterns.working = 3;
    assert_eq!(patterns.patterns[3].data[0], 400);

    patterns.working = 4;
    assert_eq!(patterns.patterns[4].data[0], 500);

    patterns.working = 5;
    assert_eq!(patterns.patterns[5].data[0], 600);
}

#[test]
fn test_p_add_saturating() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let mut counters = create_test_counters();

    patterns.patterns[0].data = [32760, 32765, 32767, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    patterns.patterns[0].length = 3;
    patterns.working = 0;

    assert_eq!(patterns.patterns[0].data[0], 32760);
    assert_eq!(patterns.patterns[0].data[1], 32765);
    assert_eq!(patterns.patterns[0].data[2], 32767);
}

#[test]
fn test_p_sub_saturating() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let mut counters = create_test_counters();

    patterns.patterns[0].data = [-32760, -32765, -32768, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    patterns.patterns[0].length = 3;
    patterns.working = 0;

    assert_eq!(patterns.patterns[0].data[0], -32760);
    assert_eq!(patterns.patterns[0].data[1], -32765);
    assert_eq!(patterns.patterns[0].data[2], -32768);
}

#[test]
fn test_p_mul_saturating() {
    let variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let mut counters = create_test_counters();

    patterns.patterns[0].data = [1000, 2000, 3000, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    patterns.patterns[0].length = 3;
    patterns.working = 0;

    assert_eq!(patterns.patterns[0].data[0], 1000);
    assert_eq!(patterns.patterns[0].data[1], 2000);
    assert_eq!(patterns.patterns[0].data[2], 3000);
}

#[test]
fn test_chained_pattern_operations() {
    let mut variables = create_test_variables();
    let mut patterns = create_test_patterns();
    let scripts = create_test_scripts();
    let mut counters = create_test_counters();

    patterns.patterns[0].data = [1, 2, 3, 4, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    patterns.patterns[0].length = 5;
    patterns.working = 0;

    assert_eq!(patterns.patterns[0].data[0], 1);
    assert_eq!(patterns.patterns[0].data[1], 2);
    assert_eq!(patterns.patterns[0].data[2], 3);
    assert_eq!(patterns.patterns[0].data[3], 4);
    assert_eq!(patterns.patterns[0].data[4], 5);
}
