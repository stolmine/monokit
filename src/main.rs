mod app;
mod commands;
mod config;
mod eval;
mod meter;
mod metro;
mod midi;
mod osc_utils;
mod preset;
mod sc_process;

#[cfg(feature = "scsynth-direct")]
mod audio_devices;

#[cfg(feature = "scsynth-direct")]
mod scsynth_direct;

mod scramble;
mod scene;
mod terminal;
mod theme;
mod types;
mod ui;
mod utils;

pub use anyhow;
use anyhow::Result;
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::prelude::*;
use std::env;
use std::io;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

use crate::app::App;
use crate::meter::meter_thread;
use crate::metro::metro_thread;
use crate::sc_process::ScProcess;
use crate::types::{MetroCommand, MetroEvent, MetroState};
use crate::ui::run_app;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    // Check for --run <scene> [wait_ms] mode
    if args.len() >= 3 && args[1] == "--run" {
        return run_batch_mode(&args[2], args.get(3).map(|s| s.as_str()));
    }

    // Normal TUI mode
    run_tui_mode()
}

/// Batch mode: load scene, wait for metro, exit (no TUI)
fn run_batch_mode(scene_name: &str, wait_ms: Option<&str>) -> Result<()> {
    let caps = terminal::detect_capabilities();

    let wait_duration = wait_ms
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(2000);

    let metro_state = Arc::new(Mutex::new(MetroState::default()));
    let (metro_tx, metro_rx) = mpsc::channel();
    let (metro_event_tx, metro_event_rx) = mpsc::channel::<MetroEvent>();

    let metro_state_clone = metro_state.clone();
    let metro_handle = thread::spawn(move || {
        metro_thread(metro_rx, metro_state_clone, metro_event_tx);
    });

    let config = config::load_config().unwrap_or_default();
    let loaded_theme = config::load_theme(&config).unwrap_or_default();
    let theme = if caps.true_color {
        loaded_theme
    } else {
        loaded_theme.to_256_color()
    };
    let color_mode = if caps.true_color {
        types::ColorMode::TrueColor
    } else {
        types::ColorMode::Color256
    };

    let mut app = App::new(metro_tx, metro_state, theme, color_mode, &config, caps);

    // Load the scene by setting input and executing
    app.input = format!("LOAD {}", scene_name);
    app.execute_command();

    // Check if load failed (look for error in output)
    if app.output.iter().any(|line| line.contains("ERROR")) {
        for line in &app.output {
            eprintln!("{}", line);
        }
        let _ = app.metro_tx.send(MetroCommand::Shutdown);
        let _ = metro_handle.join();
        return Ok(());
    }

    // Process metro events for the wait duration
    let start = std::time::Instant::now();
    while start.elapsed() < Duration::from_millis(wait_duration) {
        // Process any pending metro events
        while let Ok(event) = metro_event_rx.try_recv() {
            match event {
                MetroEvent::ExecuteScript(script_idx) => {
                    app.execute_script(script_idx);
                }
                MetroEvent::Error(msg) => {
                    eprintln!("METRO ERROR: {}", msg);
                }
                _ => {}
            }
        }
        thread::sleep(Duration::from_millis(10));
    }

    // Print REPL output
    for line in &app.output {
        println!("{}", line);
    }

    // Shutdown
    let _ = app.metro_tx.send(MetroCommand::Shutdown);
    let _ = metro_handle.join();

    Ok(())
}

/// Normal TUI mode
fn run_tui_mode() -> Result<()> {
    let caps = terminal::detect_capabilities();
    if !caps.true_color {
        eprintln!("Note: Limited color support detected.");
        eprintln!("For best experience, use iTerm2 or another truecolor terminal.");
        eprintln!("");
        std::thread::sleep(std::time::Duration::from_millis(1500));
    }

    // Start SuperCollider
    println!("Starting SuperCollider...");
    let mut sc_process = match ScProcess::new() {
        Ok(sc) => sc,
        Err(e) => {
            eprintln!("ERROR: {}", e);
            eprintln!("Please install SuperCollider from https://supercollider.github.io");
            std::process::exit(1);
        }
    };

    // Get saved audio device from config (if any)
    let config = config::load_config().unwrap_or_default();
    let audio_device = config.display.audio_out_device.clone();

    if let Err(e) = sc_process.start(audio_device.as_deref()) {
        eprintln!("ERROR: Failed to start SuperCollider: {}", e);
        std::process::exit(1);
    }

    println!("Waiting for SuperCollider to boot...");

    let metro_state = Arc::new(Mutex::new(MetroState::default()));
    let (metro_tx, metro_rx) = mpsc::channel();
    let (metro_event_tx, metro_event_rx) = mpsc::channel::<MetroEvent>();

    let metro_state_clone = metro_state.clone();
    let meter_event_tx = metro_event_tx.clone();
    let metro_handle = thread::spawn(move || {
        metro_thread(metro_rx, metro_state_clone, metro_event_tx);
    });

    // Spawn meter thread for receiving audio level data from SuperCollider
    thread::spawn(move || {
        meter_thread(meter_event_tx);
    });

    // Spawn ready sender after meter thread (scsynth-direct mode only)
    let ready_rx = sc_process.spawn_ready_sender();

    // Wait for SC ready (blocking with timeout)
    println!("Waiting for SuperCollider server...");
    let start = std::time::Instant::now();
    let timeout = std::time::Duration::from_secs(20);
    let mut sc_ready = false;

    while start.elapsed() < timeout {
        match metro_event_rx.recv_timeout(std::time::Duration::from_millis(100)) {
            Ok(MetroEvent::ScReady) => {
                sc_ready = true;
                println!("SuperCollider ready!");
                break;
            }
            Ok(_) => continue, // Other events, keep waiting
            Err(std::sync::mpsc::RecvTimeoutError::Timeout) => continue,
            Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => break,
        }
    }

    if !sc_ready {
        eprintln!("ERROR: SuperCollider failed to start within 20 seconds");
        std::process::exit(1);
    }

    // Wait for ready sender to complete (scsynth-direct mode only)
    if let Some(rx) = ready_rx {
        let _ = rx.recv_timeout(Duration::from_secs(3));
    }

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    // Set terminal window title
    print!("\x1b]0;monokit\x07");

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?; // Clear alternate screen before first render

    let loaded_theme = config::load_theme(&config).unwrap_or_default();
    let theme = if caps.true_color {
        loaded_theme
    } else {
        loaded_theme.to_256_color()
    };
    let color_mode = if caps.true_color {
        types::ColorMode::TrueColor
    } else {
        types::ColorMode::Color256
    };

    let mut app = App::new(metro_tx.clone(), metro_state, theme, color_mode, &config, caps);
    app.add_output("MONOKIT - SCRIPTING FOR COMPLEX OSCILLATOR".to_string());
    app.add_output("ENTER CMDS. [ ] NAV PAGES. ESC FOR HELP.".to_string());

    // Send initial VCA mode to SuperCollider
    let _ = metro_tx.send(MetroCommand::SendParam(
        "vca_mode".to_string(),
        rosc::OscType::Int(if config.display.vca_mode { 1 } else { 0 })
    ));

    app.execute_script(9);

    // Wrap sc_process in Arc<Mutex> to share with run_app
    let sc_process_shared = Arc::new(Mutex::new(sc_process));
    let sc_process_clone = sc_process_shared.clone();

    let res = run_app(&mut terminal, &mut app, metro_event_rx, sc_process_clone);

    // Graceful shutdown: send Shutdown command and wait for metro thread
    let _ = app.metro_tx.send(MetroCommand::Shutdown);
    let _ = metro_handle.join();

    // Shutdown SuperCollider
    if let Ok(mut sc) = sc_process_shared.lock() {
        sc.stop();
    }

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen
    )?;
    terminal.show_cursor()?;

    // Reset terminal window title
    print!("\x1b]0;\x07");

    if let Err(err) = res {
        eprintln!("Error: {:?}", err);
    }

    Ok(())
}

#[cfg(test)]
mod tests;
