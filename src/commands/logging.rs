use chrono;
use std::fs::OpenOptions;
use std::io::Write;

pub fn log_command(msg: &str) {
    let log_path = std::env::temp_dir().join("monokit_commands.log");
    if let Ok(mut file) = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_path)
    {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default();
        let secs = timestamp.as_secs();
        let millis = timestamp.subsec_millis();
        let datetime = chrono::DateTime::<chrono::Utc>::from_timestamp(secs as i64, millis * 1_000_000).unwrap_or_default();
        let formatted = datetime.format("%Y-%m-%d %H:%M:%S%.3f");
        let _ = writeln!(file, "[{}] {}", formatted, msg);
    }
}
