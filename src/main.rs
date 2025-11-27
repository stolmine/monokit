use anyhow::{Context, Result};
use parking_lot::Mutex;
use rosc::{encoder, OscMessage, OscPacket, OscType};
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;
use std::net::UdpSocket;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::time::interval;

const OSC_ADDR: &str = "127.0.0.1:57120";

#[derive(Clone)]
struct MetroState {
    interval_ms: u64,
    active: bool,
    script: String,
}

impl MetroState {
    fn new() -> Self {
        Self {
            interval_ms: 500,
            active: false,
            script: String::new(),
        }
    }
}

enum MetroCommand {
    Interval(u64),
    Active(bool),
    Script(String),
}

fn print_help() {
    println!("monokit - Teletype-style scripting for complex oscillator drum voice");
    println!();
    println!("Commands:");
    println!("  TR              - Send trigger to /monokit/trigger");
    println!("  VOL <0.0-1.0>   - Set volume via /monokit/volume");
    println!("  M               - Show current metro interval");
    println!("  M <ms>          - Set metro interval in milliseconds");
    println!("  M.BPM <bpm>     - Set metro interval as BPM");
    println!("  M.ACT <0|1>     - Activate/deactivate metro (0=off, 1=on)");
    println!("  M: <script>     - Set M script (commands to run on each tick)");
    println!("  help            - Show this help");
    println!("  exit, quit      - Exit the REPL");
    println!();
    println!("Press Ctrl+C or Ctrl+D to exit");
    println!();
}

fn send_osc(socket: &UdpSocket, addr: &str, args: Vec<OscType>) -> Result<()> {
    let msg = OscMessage {
        addr: addr.to_string(),
        args,
    };
    let packet = OscPacket::Message(msg);
    let buf = encoder::encode(&packet).context("Failed to encode OSC message")?;
    socket.send(&buf).context("Failed to send OSC message")?;
    Ok(())
}

fn process_command(
    socket: &UdpSocket,
    input: &str,
    metro_tx: &mpsc::UnboundedSender<MetroCommand>,
    metro_state: &Arc<Mutex<MetroState>>,
) -> Result<()> {
    let trimmed = input.trim();

    if trimmed.is_empty() {
        return Ok(());
    }

    if let Some(script) = trimmed.strip_prefix("M: ").or_else(|| trimmed.strip_prefix("m: ")) {
        let script = script.trim().to_string();

        for cmd in script.split(';') {
            let cmd = cmd.trim();
            if cmd.is_empty() {
                continue;
            }
            if let Err(e) = validate_script_command(cmd) {
                println!("Error: Invalid command in script: {}", e);
                return Ok(());
            }
        }

        metro_tx
            .send(MetroCommand::Script(script.clone()))
            .context("Failed to send metro command")?;
        println!("Set M script: {}", script);
        return Ok(());
    }

    let parts: Vec<&str> = trimmed.split_whitespace().collect();
    let cmd = parts[0].to_uppercase();

    match cmd.as_str() {
        "TR" => {
            send_osc(socket, "/monokit/trigger", vec![])?;
            println!("Sent trigger");
        }
        "VOL" => {
            if parts.len() < 2 {
                println!("Error: VOL requires a value (0.0-1.0)");
                return Ok(());
            }
            let value: f32 = parts[1]
                .parse()
                .context("Failed to parse volume value as float")?;
            if !(0.0..=1.0).contains(&value) {
                println!("Warning: Volume should be between 0.0 and 1.0");
            }
            send_osc(socket, "/monokit/volume", vec![OscType::Float(value)])?;
            println!("Set volume to {}", value);
        }
        "M" => {
            if parts.len() == 1 {
                let state = metro_state.lock();
                println!("Metro interval: {}ms", state.interval_ms);
            } else {
                let value: u64 = parts[1]
                    .parse()
                    .context("Failed to parse interval as milliseconds")?;
                metro_tx
                    .send(MetroCommand::Interval(value))
                    .context("Failed to send metro command")?;
                println!("Set metro interval to {}ms", value);
            }
        }
        "M.BPM" => {
            if parts.len() < 2 {
                println!("Error: M.BPM requires a BPM value");
                return Ok(());
            }
            let bpm: f64 = parts[1]
                .parse()
                .context("Failed to parse BPM value as number")?;
            if bpm <= 0.0 {
                println!("Error: BPM must be greater than 0");
                return Ok(());
            }
            let interval_ms = (60000.0 / bpm) as u64;
            metro_tx
                .send(MetroCommand::Interval(interval_ms))
                .context("Failed to send metro command")?;
            println!("Set metro to {} BPM ({}ms)", bpm, interval_ms);
        }
        "M.ACT" => {
            if parts.len() < 2 {
                println!("Error: M.ACT requires 0 or 1");
                return Ok(());
            }
            let value: u8 = parts[1]
                .parse()
                .context("Failed to parse M.ACT value")?;
            let active = value != 0;
            metro_tx
                .send(MetroCommand::Active(active))
                .context("Failed to send metro command")?;
            println!("Metro {}", if active { "activated" } else { "deactivated" });
        }
        "HELP" => {
            print_help();
        }
        "EXIT" | "QUIT" => {
            println!("Goodbye!");
            std::process::exit(0);
        }
        _ => {
            println!("Unknown command: {}. Type 'help' for available commands.", cmd);
        }
    }

    Ok(())
}

async fn metro_task(
    mut rx: mpsc::UnboundedReceiver<MetroCommand>,
    state: Arc<Mutex<MetroState>>,
    socket: Arc<UdpSocket>,
) {
    let mut ticker = interval(Duration::from_millis(state.lock().interval_ms));
    ticker.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);

    loop {
        tokio::select! {
            _ = ticker.tick() => {
                let state = state.lock();
                if state.active && !state.script.is_empty() {
                    let script = state.script.clone();
                    drop(state);

                    for cmd in script.split(';') {
                        let cmd = cmd.trim();
                        if cmd.is_empty() {
                            continue;
                        }
                        if let Err(e) = execute_script_command(&socket, cmd) {
                            eprintln!("Metro script error: {}", e);
                        }
                    }
                }
            }
            Some(cmd) = rx.recv() => {
                let mut state = state.lock();
                match cmd {
                    MetroCommand::Interval(ms) => {
                        state.interval_ms = ms;
                        drop(state);
                        ticker = interval(Duration::from_millis(ms));
                        ticker.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
                    }
                    MetroCommand::Active(active) => {
                        state.active = active;
                    }
                    MetroCommand::Script(script) => {
                        state.script = script;
                    }
                }
            }
        }
    }
}

fn validate_script_command(cmd: &str) -> Result<()> {
    let parts: Vec<&str> = cmd.split_whitespace().collect();
    if parts.is_empty() {
        return Ok(());
    }

    let command = parts[0].to_uppercase();

    match command.as_str() {
        "TR" => Ok(()),
        "VOL" => {
            if parts.len() < 2 {
                return Err(anyhow::anyhow!("VOL requires a value"));
            }
            let _value: f32 = parts[1].parse().context("Failed to parse volume value")?;
            Ok(())
        }
        _ => {
            return Err(anyhow::anyhow!("Unknown command in script: {}", command));
        }
    }
}

fn execute_script_command(socket: &UdpSocket, cmd: &str) -> Result<()> {
    let parts: Vec<&str> = cmd.split_whitespace().collect();
    if parts.is_empty() {
        return Ok(());
    }

    let cmd = parts[0].to_uppercase();

    match cmd.as_str() {
        "TR" => {
            send_osc(socket, "/monokit/trigger", vec![])?;
        }
        "VOL" => {
            if parts.len() < 2 {
                return Err(anyhow::anyhow!("VOL requires a value"));
            }
            let value: f32 = parts[1].parse().context("Failed to parse volume value")?;
            send_osc(socket, "/monokit/volume", vec![OscType::Float(value)])?;
        }
        _ => {
            return Err(anyhow::anyhow!("Unknown command in script: {}", cmd));
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:0").context("Failed to bind UDP socket")?;
    socket
        .connect(OSC_ADDR)
        .context("Failed to connect to OSC address")?;
    let socket = Arc::new(socket);

    let metro_state = Arc::new(Mutex::new(MetroState::new()));
    let (metro_tx, metro_rx) = mpsc::unbounded_channel();

    let metro_socket = Arc::clone(&socket);
    let metro_state_clone = Arc::clone(&metro_state);
    tokio::spawn(async move {
        metro_task(metro_rx, metro_state_clone, metro_socket).await;
    });

    print_help();

    let mut rl = DefaultEditor::new().context("Failed to create readline editor")?;

    loop {
        match rl.readline("monokit> ") {
            Ok(line) => {
                let _ = rl.add_history_entry(line.as_str());
                if let Err(e) = process_command(&socket, &line, &metro_tx, &metro_state) {
                    println!("Error: {}", e);
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("^C");
                println!("Goodbye!");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("^D");
                println!("Goodbye!");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }

    Ok(())
}
