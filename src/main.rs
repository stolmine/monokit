use anyhow::{Context, Result};
use rosc::{encoder, OscMessage, OscPacket, OscType};
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;
use std::net::UdpSocket;

const OSC_ADDR: &str = "127.0.0.1:57120";

fn print_help() {
    println!("monokit - Teletype-style scripting for complex oscillator drum voice");
    println!();
    println!("Commands:");
    println!("  TR              - Send trigger to /monokit/trigger");
    println!("  VOL <0.0-1.0>   - Set volume via /monokit/volume");
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

fn process_command(socket: &UdpSocket, input: &str) -> Result<()> {
    let trimmed = input.trim();

    if trimmed.is_empty() {
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

fn main() -> Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:0").context("Failed to bind UDP socket")?;
    socket
        .connect(OSC_ADDR)
        .context("Failed to connect to OSC address")?;

    print_help();

    let mut rl = DefaultEditor::new().context("Failed to create readline editor")?;

    loop {
        match rl.readline("monokit> ") {
            Ok(line) => {
                let _ = rl.add_history_entry(line.as_str());
                if let Err(e) = process_command(&socket, &line) {
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
