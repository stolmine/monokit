use anyhow::{Context, Result};
use rosc::{encoder, OscMessage, OscPacket, OscType};
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;
use std::net::UdpSocket;
use std::sync::mpsc::{self, Sender};
use std::thread;
use std::time::{Duration, Instant};

const OSC_ADDR: &str = "127.0.0.1:57120";

#[derive(Debug, Clone)]
enum MetroCommand {
    SetInterval(u64),
    SetActive(bool),
    SetScript(Vec<ScriptCommand>),
    SendParam(String, OscType),
    SendTrigger,
    SendVolume(f32),
}

#[derive(Debug, Clone)]
struct ScriptCommand {
    param_name: String,
    value: OscType,
}

fn precise_sleep(duration: Duration) {
    let start = Instant::now();
    let spin_threshold = Duration::from_micros(100);

    if duration > spin_threshold {
        thread::sleep(duration - spin_threshold);
    }

    while start.elapsed() < duration {
        std::hint::spin_loop();
    }

    let actual = start.elapsed();
    if (actual.as_micros() as i64 - duration.as_micros() as i64).abs() > 1000 {
        println!("[SLEEP] target: {}μs | actual: {}μs | error: {}μs",
            duration.as_micros(), actual.as_micros(),
            actual.as_micros() as i64 - duration.as_micros() as i64);
    }
}

fn metro_thread(rx: mpsc::Receiver<MetroCommand>) {
    let socket = match UdpSocket::bind("0.0.0.0:0") {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Metro thread: Failed to bind UDP socket: {}", e);
            return;
        }
    };

    if let Err(e) = socket.connect(OSC_ADDR) {
        eprintln!("Metro thread: Failed to connect to OSC address: {}", e);
        return;
    }

    let mut interval_ms: u64 = 500;
    let mut active = false;
    let mut script: Vec<ScriptCommand> = Vec::new();
    let mut last_tick: Option<Instant> = None;
    let mut next_tick = Instant::now();
    let mut tick_count: u64 = 0;

    loop {
        let mut command_count = 0;
        let mut interval_changed = false;
        while let Ok(cmd) = rx.try_recv() {
            command_count += 1;
            match cmd {
                MetroCommand::SetInterval(ms) => {
                    interval_ms = ms;
                    interval_changed = true;
                }
                MetroCommand::SetActive(act) => {
                    active = act;
                }
                MetroCommand::SetScript(s) => {
                    script = s;
                }
                MetroCommand::SendParam(name, value) => {
                    let msg = OscMessage {
                        addr: "/monokit/param".to_string(),
                        args: vec![OscType::String(name), value],
                    };
                    let packet = OscPacket::Message(msg);
                    if let Ok(buf) = encoder::encode(&packet) {
                        let _ = socket.send(&buf);
                    }
                }
                MetroCommand::SendTrigger => {
                    let msg = OscMessage {
                        addr: "/monokit/trigger".to_string(),
                        args: vec![],
                    };
                    let packet = OscPacket::Message(msg);
                    if let Ok(buf) = encoder::encode(&packet) {
                        let _ = socket.send(&buf);
                    }
                }
                MetroCommand::SendVolume(value) => {
                    let msg = OscMessage {
                        addr: "/monokit/volume".to_string(),
                        args: vec![OscType::Float(value)],
                    };
                    let packet = OscPacket::Message(msg);
                    if let Ok(buf) = encoder::encode(&packet) {
                        let _ = socket.send(&buf);
                    }
                }
            }
        }

        if command_count > 0 {
            println!("[METRO] processing {} command(s)", command_count);
        }

        if interval_changed {
            next_tick = Instant::now();
        }

        if active {
            let msg = OscMessage {
                addr: "/monokit/trigger".to_string(),
                args: vec![],
            };
            let packet = OscPacket::Message(msg);
            if let Ok(buf) = encoder::encode(&packet) {
                let _ = socket.send(&buf);
            }

            for cmd in &script {
                let msg = OscMessage {
                    addr: "/monokit/param".to_string(),
                    args: vec![
                        OscType::String(cmd.param_name.clone()),
                        cmd.value.clone(),
                    ],
                };
                let packet = OscPacket::Message(msg);
                if let Ok(buf) = encoder::encode(&packet) {
                    let _ = socket.send(&buf);
                }
            }

            next_tick += Duration::from_millis(interval_ms);

            let now = Instant::now();
            if next_tick > now {
                let sleep_duration = next_tick - now;
                precise_sleep(sleep_duration);
            } else {
                println!("[METRO] WARNING: running behind by {}ms",
                    (now - next_tick).as_millis());
                next_tick = now;
            }

            tick_count += 1;
            let actual_time = Instant::now();
            if let Some(last) = last_tick {
                let actual_interval = actual_time.duration_since(last);
                let drift_ms = actual_interval.as_millis() as i64 - interval_ms as i64;
                println!("[METRO] tick {} | target: {}ms | actual: {}ms | drift: {:+}ms",
                    tick_count, interval_ms, actual_interval.as_millis(), drift_ms);
            } else {
                println!("[METRO] tick {} | target: {}ms", tick_count, interval_ms);
            }
            last_tick = Some(actual_time);
        } else {
            thread::sleep(Duration::from_millis(10));
        }
    }
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
    println!();
    println!("HD2 Parameters:");
    println!("  PF <hz>         - Primary frequency (20-20000)");
    println!("  PW <0-2>        - Primary waveform (0=sin, 1=tri, 2=saw)");
    println!("  MF <hz>         - Mod frequency (20-20000)");
    println!("  MW <0-2>        - Mod waveform (0=sin, 1=tri, 2=saw)");
    println!("  DC <0-16383>    - Discontinuity amount (envelope/mod bus)");
    println!("  DM <0-2>        - Discontinuity mode (0=fold, 1=tanh, 2=softclip)");
    println!("  DD <ms>         - Discontinuity decay (1-10000ms)");
    println!("  TK <0-16383>    - Tracking amount (envelope/mod bus)");
    println!("  MB <0-16383>    - Mod bus amount (separate from FM)");
    println!("  MP <0|1>        - Mod bus -> primary freq");
    println!("  MD <0|1>        - Mod bus -> discontinuity");
    println!("  MT <0|1>        - Mod bus -> tracking");
    println!("  MA <0|1>        - Mod bus -> amplitude");
    println!("  FM <0-16383>    - FM synthesis depth (mod->primary PM)");
    println!("  AD <ms>         - Amp decay (1-10000ms)");
    println!("  PD <ms>         - Pitch decay (1-10000ms)");
    println!("  FD <ms>         - FM decay (1-10000ms)");
    println!("  PA <multiplier> - Pitch env amount (0-16)");
    println!("  MX <0-16383>    - Mix amount (mod osc in output)");
    println!("  MM <0|1>        - Mod bus -> mix");
    println!("  ME <0|1>        - Apply envelope to mix");
    println!();
    println!("Envelope Amounts:");
    println!("  FA <0-16383>    - FM envelope amount");
    println!("  DA <0-16383>    - DC envelope amount");
    println!("  Note: Envelopes offset base params: output = base + (env * amount)");
    println!();
    println!("  RST             - Reset all parameters to defaults");
    println!();
    println!("  help            - Show this help");
    println!("  exit, quit      - Exit the REPL");
    println!();
    println!("Press Ctrl+C or Ctrl+D to exit");
    println!();
}

fn process_command(
    metro_tx: &Sender<MetroCommand>,
    metro_interval: &mut u64,
    input: &str,
) -> Result<()> {
    let trimmed = input.trim();

    if trimmed.is_empty() {
        return Ok(());
    }

    if let Some(script) = trimmed.strip_prefix("M: ").or_else(|| trimmed.strip_prefix("m: ")) {
        let script = script.trim().to_string();

        let mut script_commands = Vec::new();

        for cmd in script.split(';') {
            let cmd = cmd.trim();
            if cmd.is_empty() {
                continue;
            }
            if let Err(e) = validate_script_command(cmd) {
                println!("Error: Invalid command in script: {}", e);
                return Ok(());
            }

            let parts: Vec<&str> = cmd.split_whitespace().collect();
            if parts.is_empty() {
                continue;
            }

            let command = parts[0].to_uppercase();

            if command == "TR" {
                continue;
            }

            let param_name = command.to_lowercase();

            if parts.len() > 1 {
                let value_str = parts[1];
                let value = if let Ok(int_val) = value_str.parse::<i32>() {
                    OscType::Int(int_val)
                } else if let Ok(float_val) = value_str.parse::<f32>() {
                    OscType::Float(float_val)
                } else {
                    println!("Error: Cannot parse value for {}", command);
                    return Ok(());
                };

                script_commands.push(ScriptCommand { param_name, value });
            }
        }

        metro_tx
            .send(MetroCommand::SetScript(script_commands))
            .context("Failed to send script to metro thread")?;
        println!("Set M script: {}", script);
        return Ok(());
    }

    let parts: Vec<&str> = trimmed.split_whitespace().collect();
    let cmd = parts[0].to_uppercase();

    match cmd.as_str() {
        "TR" => {
            metro_tx
                .send(MetroCommand::SendTrigger)
                .context("Failed to send trigger to metro thread")?;
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
            metro_tx
                .send(MetroCommand::SendVolume(value))
                .context("Failed to send volume to metro thread")?;
            println!("Set volume to {}", value);
        }
        "M" => {
            if parts.len() == 1 {
                println!("Metro interval: {}ms", metro_interval);
            } else {
                let value: u64 = parts[1]
                    .parse()
                    .context("Failed to parse interval as milliseconds")?;
                if value == 0 {
                    println!("Error: Interval must be greater than 0");
                    return Ok(());
                }
                metro_tx
                    .send(MetroCommand::SetInterval(value))
                    .context("Failed to send interval to metro thread")?;
                *metro_interval = value;
                println!("Set metro interval to {}ms", value);
            }
        }
        "M.BPM" => {
            if parts.len() < 2 {
                println!("Error: M.BPM requires a BPM value");
                return Ok(());
            }
            let bpm: f32 = parts[1]
                .parse()
                .context("Failed to parse BPM value as number")?;
            if bpm <= 0.0 {
                println!("Error: BPM must be greater than 0");
                return Ok(());
            }
            let interval_ms = (60000.0 / bpm) as u64;
            metro_tx
                .send(MetroCommand::SetInterval(interval_ms))
                .context("Failed to send interval to metro thread")?;
            *metro_interval = interval_ms;
            println!("Set metro to {} BPM ({}ms)", bpm, interval_ms);
        }
        "M.ACT" => {
            if parts.len() < 2 {
                println!("Error: M.ACT requires 0 or 1");
                return Ok(());
            }
            let value: i32 = parts[1]
                .parse()
                .context("Failed to parse M.ACT value")?;
            if !(0..=1).contains(&value) {
                println!("Error: M.ACT value must be 0 or 1");
                return Ok(());
            }
            metro_tx
                .send(MetroCommand::SetActive(value != 0))
                .context("Failed to send active state to metro thread")?;
            println!(
                "Metro {}",
                if value != 0 {
                    "activated"
                } else {
                    "deactivated"
                }
            );
        }
        "PF" => {
            if parts.len() < 2 {
                println!("Error: PF requires a frequency value (20-20000)");
                return Ok(());
            }
            let value: f32 = parts[1]
                .parse()
                .context("Failed to parse frequency value")?;
            if !(20.0..=20000.0).contains(&value) {
                println!("Error: Frequency must be between 20 and 20000 Hz");
                return Ok(());
            }
            metro_tx
                .send(MetroCommand::SendParam("pf".to_string(), OscType::Float(value)))
                .context("Failed to send param to metro thread")?;
            println!("Set primary frequency to {} Hz", value);
        }
        "PW" => {
            if parts.len() < 2 {
                println!("Error: PW requires a waveform value (0-2)");
                return Ok(());
            }
            let value: i32 = parts[1]
                .parse()
                .context("Failed to parse waveform value")?;
            if !(0..=2).contains(&value) {
                println!("Error: Waveform must be 0 (sin), 1 (tri), or 2 (saw)");
                return Ok(());
            }
            metro_tx
                .send(MetroCommand::SendParam("pw".to_string(), OscType::Int(value)))
                .context("Failed to send param to metro thread")?;
            println!("Set primary waveform to {}", value);
        }
        "MF" => {
            if parts.len() < 2 {
                println!("Error: MF requires a frequency value (20-20000)");
                return Ok(());
            }
            let value: f32 = parts[1]
                .parse()
                .context("Failed to parse frequency value")?;
            if !(20.0..=20000.0).contains(&value) {
                println!("Error: Frequency must be between 20 and 20000 Hz");
                return Ok(());
            }
            metro_tx
                .send(MetroCommand::SendParam("mf".to_string(), OscType::Float(value)))
                .context("Failed to send param to metro thread")?;
            println!("Set mod frequency to {} Hz", value);
        }
        "MW" => {
            if parts.len() < 2 {
                println!("Error: MW requires a waveform value (0-2)");
                return Ok(());
            }
            let value: i32 = parts[1]
                .parse()
                .context("Failed to parse waveform value")?;
            if !(0..=2).contains(&value) {
                println!("Error: Waveform must be 0 (sin), 1 (tri), or 2 (saw)");
                return Ok(());
            }
            metro_tx
                .send(MetroCommand::SendParam("mw".to_string(), OscType::Int(value)))
                .context("Failed to send param to metro thread")?;
            println!("Set mod waveform to {}", value);
        }
        "DC" => {
            if parts.len() < 2 {
                println!("Error: DC requires a value (0-16383)");
                return Ok(());
            }
            let value: i32 = parts[1]
                .parse()
                .context("Failed to parse discontinuity amount")?;
            if !(0..=16383).contains(&value) {
                println!("Error: Discontinuity amount must be between 0 and 16383");
                return Ok(());
            }
            metro_tx
                .send(MetroCommand::SendParam("dc".to_string(), OscType::Int(value)))
                .context("Failed to send param to metro thread")?;
            println!("Set discontinuity amount to {}", value);
        }
        "DM" => {
            if parts.len() < 2 {
                println!("Error: DM requires a mode value (0-2)");
                return Ok(());
            }
            let value: i32 = parts[1]
                .parse()
                .context("Failed to parse discontinuity mode")?;
            if !(0..=2).contains(&value) {
                println!("Error: Mode must be 0 (fold), 1 (tanh), or 2 (softclip)");
                return Ok(());
            }
            metro_tx
                .send(MetroCommand::SendParam("dm".to_string(), OscType::Int(value)))
                .context("Failed to send param to metro thread")?;
            println!("Set discontinuity mode to {}", value);
        }
        "TK" => {
            if parts.len() < 2 {
                println!("Error: TK requires a value (0-16383)");
                return Ok(());
            }
            let value: i32 = parts[1]
                .parse()
                .context("Failed to parse tracking amount")?;
            if !(0..=16383).contains(&value) {
                println!("Error: Tracking amount must be between 0 and 16383");
                return Ok(());
            }
            metro_tx
                .send(MetroCommand::SendParam("tk".to_string(), OscType::Int(value)))
                .context("Failed to send param to metro thread")?;
            println!("Set tracking amount to {}", value);
        }
        "MB" => {
            if parts.len() < 2 {
                println!("Error: MB requires a value (0-16383)");
                return Ok(());
            }
            let value: i32 = parts[1]
                .parse()
                .context("Failed to parse mod bus amount")?;
            if !(0..=16383).contains(&value) {
                println!("Error: Mod bus amount must be between 0 and 16383");
                return Ok(());
            }
            metro_tx
                .send(MetroCommand::SendParam("mb".to_string(), OscType::Int(value)))
                .context("Failed to send param to metro thread")?;
            println!("Set mod bus amount to {}", value);
        }
        "MP" => {
            if parts.len() < 2 {
                println!("Error: MP requires a value (0 or 1)");
                return Ok(());
            }
            let value: i32 = parts[1]
                .parse()
                .context("Failed to parse mod -> primary value")?;
            if !(0..=1).contains(&value) {
                println!("Error: Value must be 0 or 1");
                return Ok(());
            }
            metro_tx
                .send(MetroCommand::SendParam("mp".to_string(), OscType::Int(value)))
                .context("Failed to send param to metro thread")?;
            println!("Set mod -> primary freq to {}", value);
        }
        "MD" => {
            if parts.len() < 2 {
                println!("Error: MD requires a value (0 or 1)");
                return Ok(());
            }
            let value: i32 = parts[1]
                .parse()
                .context("Failed to parse mod -> discontinuity value")?;
            if !(0..=1).contains(&value) {
                println!("Error: Value must be 0 or 1");
                return Ok(());
            }
            metro_tx
                .send(MetroCommand::SendParam("md".to_string(), OscType::Int(value)))
                .context("Failed to send param to metro thread")?;
            println!("Set mod -> discontinuity to {}", value);
        }
        "MT" => {
            if parts.len() < 2 {
                println!("Error: MT requires a value (0 or 1)");
                return Ok(());
            }
            let value: i32 = parts[1]
                .parse()
                .context("Failed to parse mod -> tracking value")?;
            if !(0..=1).contains(&value) {
                println!("Error: Value must be 0 or 1");
                return Ok(());
            }
            metro_tx
                .send(MetroCommand::SendParam("mt".to_string(), OscType::Int(value)))
                .context("Failed to send param to metro thread")?;
            println!("Set mod -> tracking to {}", value);
        }
        "MA" => {
            if parts.len() < 2 {
                println!("Error: MA requires a value (0 or 1)");
                return Ok(());
            }
            let value: i32 = parts[1]
                .parse()
                .context("Failed to parse mod -> amplitude value")?;
            if !(0..=1).contains(&value) {
                println!("Error: Value must be 0 or 1");
                return Ok(());
            }
            metro_tx
                .send(MetroCommand::SendParam("ma".to_string(), OscType::Int(value)))
                .context("Failed to send param to metro thread")?;
            println!("Set mod -> amplitude to {}", value);
        }
        "FM" => {
            if parts.len() < 2 {
                println!("Error: FM requires a value (0-16383)");
                return Ok(());
            }
            let value: i32 = parts[1]
                .parse()
                .context("Failed to parse FM index")?;
            if !(0..=16383).contains(&value) {
                println!("Error: FM index must be between 0 and 16383");
                return Ok(());
            }
            metro_tx
                .send(MetroCommand::SendParam("fm".to_string(), OscType::Int(value)))
                .context("Failed to send param to metro thread")?;
            println!("Set FM index to {}", value);
        }
        "AD" => {
            if parts.len() < 2 {
                println!("Error: AD requires a time value (1-10000 ms)");
                return Ok(());
            }
            let value: i32 = parts[1]
                .parse()
                .context("Failed to parse amp decay time")?;
            if !(1..=10000).contains(&value) {
                println!("Error: Amp decay must be between 1 and 10000 ms");
                return Ok(());
            }
            metro_tx
                .send(MetroCommand::SendParam("ad".to_string(), OscType::Int(value)))
                .context("Failed to send param to metro thread")?;
            println!("Set amp decay to {} ms", value);
        }
        "PD" => {
            if parts.len() < 2 {
                println!("Error: PD requires a time value (1-10000 ms)");
                return Ok(());
            }
            let value: i32 = parts[1]
                .parse()
                .context("Failed to parse pitch decay time")?;
            if !(1..=10000).contains(&value) {
                println!("Error: Pitch decay must be between 1 and 10000 ms");
                return Ok(());
            }
            metro_tx
                .send(MetroCommand::SendParam("pd".to_string(), OscType::Int(value)))
                .context("Failed to send param to metro thread")?;
            println!("Set pitch decay to {} ms", value);
        }
        "FD" => {
            if parts.len() < 2 {
                println!("Error: FD requires a time value (1-10000 ms)");
                return Ok(());
            }
            let value: i32 = parts[1]
                .parse()
                .context("Failed to parse FM decay time")?;
            if !(1..=10000).contains(&value) {
                println!("Error: FM decay must be between 1 and 10000 ms");
                return Ok(());
            }
            metro_tx
                .send(MetroCommand::SendParam("fd".to_string(), OscType::Int(value)))
                .context("Failed to send param to metro thread")?;
            println!("Set FM decay to {} ms", value);
        }
        "PA" => {
            if parts.len() < 2 {
                println!("Error: PA requires a multiplier value (0-16)");
                return Ok(());
            }
            let value: f32 = parts[1]
                .parse()
                .context("Failed to parse pitch env amount")?;
            if !(0.0..=16.0).contains(&value) {
                println!("Error: Pitch env amount must be between 0 and 16");
                return Ok(());
            }
            metro_tx
                .send(MetroCommand::SendParam("pa".to_string(), OscType::Float(value)))
                .context("Failed to send param to metro thread")?;
            println!("Set pitch env amount to {}", value);
        }
        "DD" => {
            if parts.len() < 2 {
                println!("Error: DD requires a time value (1-10000 ms)");
                return Ok(());
            }
            let value: i32 = parts[1]
                .parse()
                .context("Failed to parse discontinuity decay time")?;
            if !(1..=10000).contains(&value) {
                println!("Error: Discontinuity decay must be between 1 and 10000 ms");
                return Ok(());
            }
            metro_tx
                .send(MetroCommand::SendParam("dd".to_string(), OscType::Int(value)))
                .context("Failed to send param to metro thread")?;
            println!("Set discontinuity decay to {} ms", value);
        }
        "MX" => {
            if parts.len() < 2 {
                println!("Error: MX requires a value (0-16383)");
                return Ok(());
            }
            let value: i32 = parts[1]
                .parse()
                .context("Failed to parse mix amount")?;
            if !(0..=16383).contains(&value) {
                println!("Error: Mix amount must be between 0 and 16383");
                return Ok(());
            }
            metro_tx
                .send(MetroCommand::SendParam("mx".to_string(), OscType::Int(value)))
                .context("Failed to send param to metro thread")?;
            println!("Set mix amount to {}", value);
        }
        "MM" => {
            if parts.len() < 2 {
                println!("Error: MM requires a value (0 or 1)");
                return Ok(());
            }
            let value: i32 = parts[1]
                .parse()
                .context("Failed to parse mod bus -> mix value")?;
            if !(0..=1).contains(&value) {
                println!("Error: Value must be 0 or 1");
                return Ok(());
            }
            metro_tx
                .send(MetroCommand::SendParam("mm".to_string(), OscType::Int(value)))
                .context("Failed to send param to metro thread")?;
            println!("Set mod bus -> mix to {}", value);
        }
        "ME" => {
            if parts.len() < 2 {
                println!("Error: ME requires a value (0 or 1)");
                return Ok(());
            }
            let value: i32 = parts[1]
                .parse()
                .context("Failed to parse envelope -> mix value")?;
            if !(0..=1).contains(&value) {
                println!("Error: Value must be 0 or 1");
                return Ok(());
            }
            metro_tx
                .send(MetroCommand::SendParam("me".to_string(), OscType::Int(value)))
                .context("Failed to send param to metro thread")?;
            println!("Set envelope -> mix to {}", value);
        }
        "FA" => {
            if parts.len() < 2 {
                println!("Error: FA requires a value (0-16383)");
                return Ok(());
            }
            let value: i32 = parts[1]
                .parse()
                .context("Failed to parse FM envelope amount")?;
            if !(0..=16383).contains(&value) {
                println!("Error: FM envelope amount must be between 0 and 16383");
                return Ok(());
            }
            metro_tx
                .send(MetroCommand::SendParam("fa".to_string(), OscType::Int(value)))
                .context("Failed to send param to metro thread")?;
            println!("Set FM envelope amount to {}", value);
        }
        "DA" => {
            if parts.len() < 2 {
                println!("Error: DA requires a value (0-16383)");
                return Ok(());
            }
            let value: i32 = parts[1]
                .parse()
                .context("Failed to parse DC envelope amount")?;
            if !(0..=16383).contains(&value) {
                println!("Error: DC envelope amount must be between 0 and 16383");
                return Ok(());
            }
            metro_tx
                .send(MetroCommand::SendParam("da".to_string(), OscType::Int(value)))
                .context("Failed to send param to metro thread")?;
            println!("Set DC envelope amount to {}", value);
        }
        "RST" => {
            metro_tx.send(MetroCommand::SendParam("pf".to_string(), OscType::Float(200.0)))?;
            metro_tx.send(MetroCommand::SendParam("pw".to_string(), OscType::Int(0)))?;
            metro_tx.send(MetroCommand::SendParam("mf".to_string(), OscType::Float(50.0)))?;
            metro_tx.send(MetroCommand::SendParam("mw".to_string(), OscType::Int(0)))?;
            metro_tx.send(MetroCommand::SendParam("dc".to_string(), OscType::Int(0)))?;
            metro_tx.send(MetroCommand::SendParam("dm".to_string(), OscType::Int(0)))?;
            metro_tx.send(MetroCommand::SendParam("dd".to_string(), OscType::Int(100)))?;
            metro_tx.send(MetroCommand::SendParam("tk".to_string(), OscType::Int(0)))?;
            metro_tx.send(MetroCommand::SendParam("mb".to_string(), OscType::Int(0)))?;
            metro_tx.send(MetroCommand::SendParam("mp".to_string(), OscType::Int(0)))?;
            metro_tx.send(MetroCommand::SendParam("md".to_string(), OscType::Int(0)))?;
            metro_tx.send(MetroCommand::SendParam("mt".to_string(), OscType::Int(0)))?;
            metro_tx.send(MetroCommand::SendParam("ma".to_string(), OscType::Int(0)))?;
            metro_tx.send(MetroCommand::SendParam("fm".to_string(), OscType::Int(0)))?;
            metro_tx.send(MetroCommand::SendParam("ad".to_string(), OscType::Int(100)))?;
            metro_tx.send(MetroCommand::SendParam("pd".to_string(), OscType::Int(10)))?;
            metro_tx.send(MetroCommand::SendParam("fd".to_string(), OscType::Int(10)))?;
            metro_tx.send(MetroCommand::SendParam("pa".to_string(), OscType::Float(4.0)))?;
            metro_tx.send(MetroCommand::SendParam("mx".to_string(), OscType::Int(0)))?;
            metro_tx.send(MetroCommand::SendParam("mm".to_string(), OscType::Int(0)))?;
            metro_tx.send(MetroCommand::SendParam("me".to_string(), OscType::Int(0)))?;
            metro_tx.send(MetroCommand::SendParam("fa".to_string(), OscType::Int(0)))?;
            metro_tx.send(MetroCommand::SendParam("da".to_string(), OscType::Int(0)))?;
            metro_tx.send(MetroCommand::SendVolume(1.0))?;
            println!("Reset to defaults");
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

fn validate_script_command(cmd: &str) -> Result<()> {
    let parts: Vec<&str> = cmd.split_whitespace().collect();
    if parts.is_empty() {
        return Ok(());
    }

    let command = parts[0].to_uppercase();

    match command.as_str() {
        "TR" | "RST" => Ok(()),
        "VOL" => {
            if parts.len() < 2 {
                return Err(anyhow::anyhow!("VOL requires a value"));
            }
            let _value: f32 = parts[1].parse().context("Failed to parse volume value")?;
            Ok(())
        }
        "PF" | "MF" => {
            if parts.len() < 2 {
                return Err(anyhow::anyhow!("{} requires a frequency value", command));
            }
            let value: f32 = parts[1].parse().context("Failed to parse frequency")?;
            if !(20.0..=20000.0).contains(&value) {
                return Err(anyhow::anyhow!("Frequency must be between 20 and 20000"));
            }
            Ok(())
        }
        "PW" | "MW" => {
            if parts.len() < 2 {
                return Err(anyhow::anyhow!("{} requires a waveform value", command));
            }
            let value: i32 = parts[1].parse().context("Failed to parse waveform")?;
            if !(0..=2).contains(&value) {
                return Err(anyhow::anyhow!("Waveform must be 0-2"));
            }
            Ok(())
        }
        "DC" | "TK" | "MB" | "FM" | "MX" | "FA" | "DA" => {
            if parts.len() < 2 {
                return Err(anyhow::anyhow!("{} requires a value", command));
            }
            let value: i32 = parts[1].parse().context("Failed to parse value")?;
            if !(0..=16383).contains(&value) {
                return Err(anyhow::anyhow!("Value must be between 0 and 16383"));
            }
            Ok(())
        }
        "DM" => {
            if parts.len() < 2 {
                return Err(anyhow::anyhow!("DM requires a mode value"));
            }
            let value: i32 = parts[1].parse().context("Failed to parse mode")?;
            if !(0..=2).contains(&value) {
                return Err(anyhow::anyhow!("Mode must be 0-2"));
            }
            Ok(())
        }
        "MP" | "MD" | "MT" | "MA" | "MM" | "ME" => {
            if parts.len() < 2 {
                return Err(anyhow::anyhow!("{} requires a value", command));
            }
            let value: i32 = parts[1].parse().context("Failed to parse value")?;
            if !(0..=1).contains(&value) {
                return Err(anyhow::anyhow!("Value must be 0 or 1"));
            }
            Ok(())
        }
        "AD" | "PD" | "FD" | "DD" => {
            if parts.len() < 2 {
                return Err(anyhow::anyhow!("{} requires a time value", command));
            }
            let value: i32 = parts[1].parse().context("Failed to parse time")?;
            if !(1..=10000).contains(&value) {
                return Err(anyhow::anyhow!("Time must be between 1 and 10000 ms"));
            }
            Ok(())
        }
        "PA" => {
            if parts.len() < 2 {
                return Err(anyhow::anyhow!("PA requires a multiplier value"));
            }
            let value: f32 = parts[1].parse().context("Failed to parse multiplier")?;
            if !(0.0..=16.0).contains(&value) {
                return Err(anyhow::anyhow!("Multiplier must be between 0 and 16"));
            }
            Ok(())
        }
        _ => {
            return Err(anyhow::anyhow!("Unknown command in script: {}", command));
        }
    }
}

fn main() -> Result<()> {
    let (metro_tx, metro_rx) = mpsc::channel();
    thread::spawn(move || {
        metro_thread(metro_rx);
    });

    let mut metro_interval: u64 = 500;

    print_help();

    let mut rl = DefaultEditor::new().context("Failed to create readline editor")?;

    loop {
        match rl.readline("monokit> ") {
            Ok(line) => {
                let _ = rl.add_history_entry(line.as_str());
                if let Err(e) = process_command(&metro_tx, &mut metro_interval, &line) {
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
