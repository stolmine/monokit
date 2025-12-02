use super::common::TestContext;

#[test]
fn test_tog_command_integration() {
    let mut ctx = TestContext::new();

    ctx.run("TOG 100 200").unwrap();
    assert_eq!(ctx.outputs.last().unwrap(), "100");

    ctx.clear_outputs();
    ctx.run("TOG 100 200").unwrap();
    assert_eq!(ctx.outputs.last().unwrap(), "200");

    ctx.clear_outputs();
    ctx.run("TOG 100 200").unwrap();
    assert_eq!(ctx.outputs.last().unwrap(), "100");
}

#[test]
fn test_tog_with_variable_assignment() {
    let mut ctx = TestContext::new();

    ctx.run("A TOG 0 1").unwrap();
    assert_eq!(ctx.variables.a, 0);

    ctx.run("A TOG 0 1").unwrap();
    assert_eq!(ctx.variables.a, 1);

    ctx.run("A TOG 0 1").unwrap();
    assert_eq!(ctx.variables.a, 0);
}
