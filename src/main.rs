mod app;
mod commands;
mod config;
mod eval;
mod metro;
mod scene;
mod theme;
mod types;
mod ui;

pub use anyhow;
use anyhow::Result;
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::prelude::*;
use std::io;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

use crate::app::App;
use crate::metro::metro_thread;
use crate::types::{MetroCommand, MetroEvent, MetroState};
use crate::ui::run_app;

fn main() -> Result<()> {
    let metro_state = Arc::new(Mutex::new(MetroState::default()));
    let (metro_tx, metro_rx) = mpsc::channel();
    let (metro_event_tx, metro_event_rx) = mpsc::channel::<MetroEvent>();

    let metro_state_clone = metro_state.clone();
    let metro_handle = thread::spawn(move || {
        metro_thread(metro_rx, metro_state_clone, metro_event_tx);
    });

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?; // Clear alternate screen before first render

    let config = config::load_config().unwrap_or_default();
    let theme = config::load_theme(&config).unwrap_or_default();

    let mut app = App::new(metro_tx, metro_state, theme);
    app.add_output("MONOKIT - TELETYPE-STYLE SCRIPTING FOR COMPLEX OSCILLATOR".to_string());
    app.add_output("TYPE COMMANDS AND PRESS ENTER. USE [ ] TO NAVIGATE PAGES.".to_string());

    app.execute_script(9);

    let res = run_app(&mut terminal, &mut app, metro_event_rx);

    // Graceful shutdown: send Shutdown command and wait for metro thread
    let _ = app.metro_tx.send(MetroCommand::Shutdown);
    let _ = metro_handle.join();

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        eprintln!("Error: {:?}", err);
    }

    Ok(())
}

#[cfg(test)]
mod tests;
