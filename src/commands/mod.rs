mod aliases;
pub mod common;
pub mod context;
mod core;
mod dispatch;
mod gate;
mod logging;
mod patterns;
pub mod randomization;
pub mod registry;
pub mod slew;
mod synth;
mod system;
pub mod validate;
mod validate_expr;

// Re-export OutputDecider so callers can use the trait methods
pub use crate::output::OutputDecider;

// Re-export from core module
pub use core::scale;
pub use core::scheduling as delay;

// Re-export public API
pub use aliases::resolve_alias;
pub use dispatch::process_command;

#[cfg(test)]
pub use validate::validate_script_command;
