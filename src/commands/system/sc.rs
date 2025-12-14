use crate::commands::context::ExecutionContext;
use crate::types::MetroCommand;
use anyhow::Result;

pub fn handle_sc_diag<F>(
    parts: &[&str],
    ctx: &mut ExecutionContext,
    mut output: F,
) -> Result<()>
where
    F: FnMut(String),
{
    if parts.len() == 1 {
        output("SC TIMING DIAGNOSTICS".to_string());
        output("SC.DIAG 1         - ENABLE DIAGNOSTICS".to_string());
        output("SC.DIAG 0         - DISABLE DIAGNOSTICS".to_string());
        output("SC.DIAG REPORT    - GENERATE TIMING REPORT".to_string());
        output("SC.DIAG TRIGGERS  - SHOW TRIGGER COUNT".to_string());
        output("SC.DIAG RST       - RESET TRIGGER COUNT".to_string());
    } else {
        match parts[1] {
            "1" => {
                ctx.metro_tx.send(MetroCommand::SendScDiag(1))?;
                output("SC TIMING DIAGNOSTICS ENABLED".to_string());
            }
            "0" => {
                ctx.metro_tx.send(MetroCommand::SendScDiag(0))?;
                output("SC TIMING DIAGNOSTICS DISABLED".to_string());
            }
            "REPORT" | "R" => {
                ctx.metro_tx.send(MetroCommand::SendScDiagReport)?;
                output("SC TIMING REPORT REQUESTED".to_string());
            }
            "TRIGGERS" | "T" => {
                ctx.metro_tx.send(MetroCommand::GetTriggerCount)?;
                output("CHECK CONSOLE FOR TRIGGER COUNT".to_string());
            }
            "RST" => {
                ctx.metro_tx.send(MetroCommand::ResetTriggerCount)?;
                output("TRIGGER COUNTER RESET".to_string());
            }
            _ => {
                output("ERROR: USE 1, 0, REPORT, TRIGGERS, RST".to_string());
            }
        }
    }
    Ok(())
}
