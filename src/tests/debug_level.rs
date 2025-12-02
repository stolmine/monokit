use super::common::TestContext;

#[test]
fn test_debug_level_0_blocks_param_output() {
    let mut ctx = TestContext::new();

    let result = ctx.run("PF 440");

    assert!(result.is_ok());
    assert_eq!(ctx.outputs.len(), 0);
}

#[test]
fn test_debug_level_0_blocks_trigger_output() {
    let mut ctx = TestContext::new();

    let result = ctx.run("TR");

    assert!(result.is_ok());
    assert_eq!(ctx.outputs.len(), 0);
}

#[test]
fn test_debug_level_0_blocks_print() {
    let mut ctx = TestContext::new();

    let result = ctx.run("PRINT 42");

    assert!(result.is_ok());
    assert_eq!(ctx.outputs.len(), 0);
}

#[test]
fn test_debug_level_1_allows_print() {
    let mut ctx = TestContext::new().with_debug_level(1);

    let result = ctx.run("PRINT 42");

    assert!(result.is_ok());
    assert_eq!(ctx.outputs.len(), 1);
    assert_eq!(ctx.outputs[0], "42");
}

#[test]
fn test_debug_level_1_blocks_params() {
    let mut ctx = TestContext::new().with_debug_level(1);

    let result = ctx.run("PF 440");

    assert!(result.is_ok());
    assert_eq!(ctx.outputs.len(), 0);
}

#[test]
fn test_debug_level_1_allows_metro_status() {
    let mut ctx = TestContext::new().with_debug_level(1);

    let result = ctx.run("M");

    assert!(result.is_ok());
    assert_eq!(ctx.outputs.len(), 1);
    assert_eq!(ctx.outputs[0], "METRO INTERVAL: 500MS");
}

#[test]
fn test_debug_level_2_allows_all() {
    let mut ctx = TestContext::new().with_debug_level(2);

    let result = ctx.run("PF 440");
    assert!(result.is_ok());
    assert_eq!(ctx.outputs.len(), 1);
    assert_eq!(ctx.outputs[0], "SET PRIMARY FREQUENCY TO 440 HZ");

    ctx.clear_outputs();

    let result = ctx.run("PRINT 42");
    assert!(result.is_ok());
    assert_eq!(ctx.outputs.len(), 1);
    assert_eq!(ctx.outputs[0], "42");

    ctx.clear_outputs();

    let result = ctx.run("M");
    assert!(result.is_ok());
    assert_eq!(ctx.outputs.len(), 1);
    assert_eq!(ctx.outputs[0], "METRO INTERVAL: 500MS");
}
