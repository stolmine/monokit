use super::common::TestContext;

#[test]
fn test_header_default_level() {
    let ctx = TestContext::new();

    assert_eq!(ctx.header_level, 4);
}

#[test]
fn test_header_query() {
    let mut ctx = TestContext::new().with_debug_level(1);

    let result = ctx.run("HEADER");

    assert!(result.is_ok());
    assert_eq!(ctx.outputs.len(), 1);
    assert_eq!(ctx.outputs[0], "HEADER LEVEL: 4");
}

#[test]
fn test_header_set_level_0() {
    let mut ctx = TestContext::new().with_debug_level(1);

    let result = ctx.run("HEADER 0");

    assert!(result.is_ok());
    assert_eq!(ctx.header_level, 0);
    assert_eq!(ctx.outputs.len(), 1);
    assert_eq!(ctx.outputs[0], "HEADER LEVEL: 0 (NAV ONLY)");
}

#[test]
fn test_header_set_level_1() {
    let mut ctx = TestContext::new().with_debug_level(1);

    let result = ctx.run("HEADER 1");

    assert!(result.is_ok());
    assert_eq!(ctx.header_level, 1);
    assert_eq!(ctx.outputs.len(), 1);
    assert_eq!(ctx.outputs[0], "HEADER LEVEL: 1 (NAV + METERS)");
}

#[test]
fn test_header_set_level_2() {
    let mut ctx = TestContext::new().with_debug_level(1);

    let result = ctx.run("HEADER 2");

    assert!(result.is_ok());
    assert_eq!(ctx.header_level, 2);
    assert_eq!(ctx.outputs.len(), 1);
    assert_eq!(ctx.outputs[0], "HEADER LEVEL: 2 (NAV + C/P + METERS)");
}

#[test]
fn test_header_set_level_3() {
    let mut ctx = TestContext::new().with_debug_level(1);

    let result = ctx.run("HEADER 3");

    assert!(result.is_ok());
    assert_eq!(ctx.header_level, 3);
    assert_eq!(ctx.outputs.len(), 1);
    assert_eq!(ctx.outputs[0], "HEADER LEVEL: 3 (FULL NAV + C/P + METERS)");
}

#[test]
fn test_header_set_level_4() {
    let mut ctx = TestContext::new().with_debug_level(1);

    let result = ctx.run("HEADER 4");

    assert!(result.is_ok());
    assert_eq!(ctx.header_level, 4);
    assert_eq!(ctx.outputs.len(), 1);
    assert_eq!(ctx.outputs[0], "HEADER LEVEL: 4 (FULL NAV + C/P + METERS + CPU)");
}

#[test]
fn test_header_invalid_value() {
    let mut ctx = TestContext::new().with_debug_level(1);

    let result = ctx.run("HEADER 5");

    assert!(result.is_ok());
    assert_eq!(ctx.header_level, 4);
    assert_eq!(ctx.outputs.len(), 1);
    assert_eq!(ctx.outputs[0], "ERROR: HEADER TAKES 0-4");
}

#[test]
fn test_header_non_numeric() {
    let mut ctx = TestContext::new().with_debug_level(1);

    let result = ctx.run("HEADER abc");

    assert!(result.is_ok());
    assert_eq!(ctx.header_level, 4);
    assert_eq!(ctx.outputs.len(), 1);
    assert_eq!(ctx.outputs[0], "ERROR: HEADER TAKES 0-4");
}
