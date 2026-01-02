//! Command registry - single source of truth for validation and dispatch

use once_cell::sync::Lazy;
use std::collections::HashMap;

/// Argument count specification for command validation
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArgCount {
    /// Exactly 0 arguments
    None,
    /// Exactly N arguments
    Exactly(usize),
    /// At least N arguments
    AtLeast(usize),
    /// Between min and max arguments (inclusive)
    Range(usize, usize),
    /// Requires specialized validation function
    Custom,
}

/// Command definition - single source of truth
#[derive(Debug, Clone)]
pub struct CommandDef {
    /// Primary command name (e.g., "PF", "M.BPM")
    pub name: &'static str,
    /// Canonical long-form name if this is an alias (e.g., "POSC.FREQ" for "PF")
    pub canonical: Option<&'static str>,
    /// Argument requirements
    pub args: ArgCount,
    /// Help text (max 46 chars for terminal compliance)
    pub help: &'static str,
    /// Whether command requires specialized validation (SEQ, loops, etc.)
    pub special_validation: bool,
}

impl CommandDef {
    /// Create a new command definition
    pub const fn new(
        name: &'static str,
        canonical: Option<&'static str>,
        args: ArgCount,
        help: &'static str,
    ) -> Self {
        Self {
            name,
            canonical,
            args,
            help,
            special_validation: false,
        }
    }

    /// Create command with special validation flag
    pub const fn with_special_validation(mut self) -> Self {
        self.special_validation = true;
        self
    }
}

// Category modules
mod variables;
mod counters;
mod patterns;
mod synth;
mod effects;
mod system;
mod control;
mod ui;

// Registry aggregation
mod commands;

// Validation
pub mod validate;

pub use commands::COMMAND_REGISTRY;
