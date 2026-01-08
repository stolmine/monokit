use crate::commands::context::ExecutionContext;
use crate::commands::logging::log_command;
use crate::commands::resolve_alias;

mod dispatch_core;
mod dispatch_synth;
mod dispatch_system;

use crate::utils::split_whitespace_respecting_quotes;
use anyhow::Result;

pub fn process_command<F>(
    ctx: &mut ExecutionContext,
    input: &str,
    mut output: F,
) -> Result<Vec<usize>>
where
    F: FnMut(String),
{
    let trimmed = input.trim();

    if trimmed.is_empty() {
        return Ok(vec![]);
    }

    let parts_owned = split_whitespace_respecting_quotes(trimmed);
    let parts: Vec<&str> = parts_owned.iter().map(|s| s.as_str()).collect();
    let cmd = parts[0].to_uppercase();
    let original_cmd = cmd.clone();

    if cmd.starts_with("CL") {
        log_command(&format!("[DEBUG] [1] Command parsing: original_cmd='{}' (from input '{}')", cmd, trimmed));
    }

    let cmd = resolve_alias(&cmd);

    if original_cmd.starts_with("CL") {
        log_command(&format!("[DEBUG] [2] After alias resolution: original='{}' resolved='{}' match={}",
                  original_cmd, cmd, original_cmd != cmd));
    }

    if cmd != original_cmd {
        log_command(&format!("CMD: {} (alias for {})", trimmed, cmd));
    } else {
        log_command(&format!("CMD: {}", trimmed));
    }

    if cmd.starts_with("CL") {
        log_command(&format!("[DEBUG] [3] Entering match statement: cmd.as_str()='{}'", cmd.as_str()));
    }

    if let Some(result) = dispatch_core::dispatch_core_commands(&cmd, &parts, ctx, &mut output) {
        log_command(&format!("CMD: {} → DISPATCHED", trimmed));
        return result;
    }

    if let Some(result) = dispatch_synth::dispatch_synth_commands(&cmd, &parts, ctx, &mut output) {
        log_command(&format!("CMD: {} → DISPATCHED", trimmed));
        return result;
    }

    if let Some(result) = dispatch_system::dispatch_system_commands(&cmd, &parts, input, ctx, &mut output) {
        log_command(&format!("CMD: {} → DISPATCHED", trimmed));
        return result;
    }

    if cmd.starts_with("CL") {
        log_command(&format!("[DEBUG] [X] UNKNOWN COMMAND HIT! cmd='{}' original_cmd='{}' input='{}'",
                  cmd, original_cmd, trimmed));
        log_command(&format!("[DEBUG] [X] cmd bytes: {:?}", cmd.as_bytes()));
    }
    log_command(&format!("CMD: {} → UNKNOWN", trimmed));
    output(format!("UNKNOWN COMMAND: {}", cmd));

    log_command(&format!("CMD: {} → DISPATCHED", trimmed));

    Ok(vec![])
}
