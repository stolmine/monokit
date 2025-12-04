use super::common::TestContext;

#[test]
fn test_title_timer_default_disabled() {
    let ctx = TestContext::new();
    assert_eq!(ctx.title_timer_enabled, false);
    assert_eq!(ctx.title_timer_interval_secs, 5);
}

#[test]
fn test_title_timer_query() {
    let mut ctx = TestContext::new();
    let result = ctx.run("TITLE.TIMER");
    assert!(result.is_ok());
    assert!(ctx.outputs.len() > 0);
    assert!(ctx.outputs[0].contains("OFF") || ctx.outputs[0].contains("ON"));
}

#[test]
fn test_title_timer_enable() {
    let mut ctx = TestContext::new();
    let result = ctx.run("TITLE.TIMER 1 10");
    assert!(result.is_ok());
    assert!(ctx.outputs.len() > 0);
    assert!(ctx.outputs[0].contains("ON"));
    assert!(ctx.outputs[0].contains("10S"));
    assert_eq!(ctx.title_timer_enabled, true);
    assert_eq!(ctx.title_timer_interval_secs, 10);
}

#[test]
fn test_title_timer_disable() {
    let mut ctx = TestContext::new();
    ctx.title_timer_enabled = true;
    ctx.title_timer_interval_secs = 5;
    let result = ctx.run("TITLE.TIMER 0");
    assert!(result.is_ok());
    assert!(ctx.outputs.len() > 0);
    assert!(ctx.outputs[0].contains("OFF"));
    assert_eq!(ctx.title_timer_enabled, false);
}

#[test]
fn test_title_timer_validation_too_low() {
    let mut ctx = TestContext::new();
    let result = ctx.run("TITLE.TIMER 1 0");
    assert!(result.is_ok());
    assert!(ctx.outputs.iter().any(|s| s.contains("1-1800")));
    assert_eq!(ctx.title_timer_enabled, false);
}

#[test]
fn test_title_timer_validation_too_high() {
    let mut ctx = TestContext::new();
    let result = ctx.run("TITLE.TIMER 1 2000");
    assert!(result.is_ok());
    assert!(ctx.outputs.iter().any(|s| s.contains("1-1800")));
    assert_eq!(ctx.title_timer_enabled, false);
}

#[test]
fn test_title_timer_valid_range() {
    let mut ctx = TestContext::new();

    // Test minimum
    ctx.clear_outputs();
    let result = ctx.run("TITLE.TIMER 1 1");
    assert!(result.is_ok());
    assert_eq!(ctx.title_timer_enabled, true);
    assert_eq!(ctx.title_timer_interval_secs, 1);

    // Test maximum
    ctx.clear_outputs();
    let result = ctx.run("TITLE.TIMER 1 1800");
    assert!(result.is_ok());
    assert_eq!(ctx.title_timer_enabled, true);
    assert_eq!(ctx.title_timer_interval_secs, 1800);
}

#[test]
fn test_title_timer_missing_seconds() {
    let mut ctx = TestContext::new();
    let result = ctx.run("TITLE.TIMER 1");
    assert!(result.is_ok());
    assert!(ctx.outputs.iter().any(|s| s.contains("REQUIRES SECONDS")));
    assert_eq!(ctx.title_timer_enabled, false);
}
