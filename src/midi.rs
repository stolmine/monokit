use crate::types::MetroCommand;
use midir::{MidiInput, MidiInputConnection};
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};
use std::time::Instant;

const MIDI_CLOCK: u8 = 0xF8;
const MIDI_START: u8 = 0xFA;
const MIDI_STOP: u8 = 0xFC;
const PULSES_PER_16TH: u8 = 6;

// Timing diagnostics
pub struct MidiTimingStats {
    pub last_tick_time: Mutex<Option<Instant>>,
    pub tick_intervals: Mutex<Vec<f64>>,
    pub enabled: Mutex<bool>,
    // Raw pulse timing (every MIDI clock message, not just every 6th)
    pub last_pulse_time: Mutex<Option<Instant>>,
    pub pulse_intervals: Mutex<Vec<f64>>,
}

impl MidiTimingStats {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            last_tick_time: Mutex::new(None),
            tick_intervals: Mutex::new(Vec::new()),
            enabled: Mutex::new(false),
            last_pulse_time: Mutex::new(None),
            pulse_intervals: Mutex::new(Vec::new()),
        })
    }

    pub fn enable(&self) {
        *self.enabled.lock().unwrap() = true;
        *self.last_tick_time.lock().unwrap() = None;
        self.tick_intervals.lock().unwrap().clear();
        *self.last_pulse_time.lock().unwrap() = None;
        self.pulse_intervals.lock().unwrap().clear();
    }

    pub fn disable(&self) {
        *self.enabled.lock().unwrap() = false;
    }

    /// Record every raw MIDI clock pulse (24 PPQN)
    pub fn record_pulse(&self) {
        let enabled = *self.enabled.lock().unwrap();
        if !enabled {
            return;
        }

        let now = Instant::now();
        let mut last_time = self.last_pulse_time.lock().unwrap();

        if let Some(last) = *last_time {
            let interval_us = now.duration_since(last).as_micros() as f64;
            let mut intervals = self.pulse_intervals.lock().unwrap();
            intervals.push(interval_us);

            // Keep last 96 pulses (4 quarter notes worth)
            if intervals.len() > 96 {
                intervals.remove(0);
            }
        }

        *last_time = Some(now);
    }

    pub fn record_tick(&self) {
        let enabled = *self.enabled.lock().unwrap();
        if !enabled {
            return;
        }

        let now = Instant::now();
        let mut last_time = self.last_tick_time.lock().unwrap();

        if let Some(last) = *last_time {
            let interval_ms = now.duration_since(last).as_micros() as f64 / 1000.0;
            let mut intervals = self.tick_intervals.lock().unwrap();
            intervals.push(interval_ms);

            // Keep last 100 intervals
            if intervals.len() > 100 {
                intervals.remove(0);
            }
        }

        *last_time = Some(now);
    }

    pub fn get_stats(&self) -> Option<(f64, f64, f64, f64)> {
        let intervals = self.tick_intervals.lock().unwrap();
        if intervals.is_empty() {
            return None;
        }

        let sum: f64 = intervals.iter().sum();
        let mean = sum / intervals.len() as f64;

        let variance = intervals.iter()
            .map(|&x| (x - mean).powi(2))
            .sum::<f64>() / intervals.len() as f64;
        let std_dev = variance.sqrt();

        let min = intervals.iter().copied().fold(f64::INFINITY, f64::min);
        let max = intervals.iter().copied().fold(f64::NEG_INFINITY, f64::max);

        Some((mean, std_dev, min, max))
    }

    /// Write timing report to midi_timing_report.txt in current directory
    /// Returns the path to the report file, or error message
    pub fn write_report(&self) -> String {
        let intervals = self.tick_intervals.lock().unwrap();

        if intervals.is_empty() {
            return "No MIDI timing data collected yet".to_string();
        }

        let mut report = String::new();

        let sum: f64 = intervals.iter().sum();
        let mean = sum / intervals.len() as f64;
        let variance = intervals.iter()
            .map(|&x| (x - mean).powi(2))
            .sum::<f64>() / intervals.len() as f64;
        let std_dev = variance.sqrt();
        let min = intervals.iter().copied().fold(f64::INFINITY, f64::min);
        let max = intervals.iter().copied().fold(f64::NEG_INFINITY, f64::max);

        report.push_str("\n=== MIDI Input Timing Report (16th notes) ===\n");
        report.push_str(&format!("Samples:         {}\n", intervals.len()));
        report.push_str(&format!("Mean interval:   {:.3}ms\n", mean));
        report.push_str(&format!("Std deviation:   {:.3}ms\n", std_dev));
        report.push_str(&format!("Min interval:    {:.3}ms\n", min));
        report.push_str(&format!("Max interval:    {:.3}ms\n", max));
        report.push_str(&format!("Jitter range:    {:.3}ms\n", max - min));

        // Calculate expected interval based on mean
        let expected_bpm = 60000.0 / (mean * 4.0);
        report.push_str(&format!("Estimated BPM:   {:.1}\n", expected_bpm));

        // Jitter as percentage
        let jitter_pct = (std_dev / mean) * 100.0;
        report.push_str(&format!("Jitter percent:  {:.2}%\n", jitter_pct));

        // Show last 16 intervals to detect swing pattern
        report.push_str("\nLast 16 intervals (ms) - look for alternating pattern:\n");
        let start = if intervals.len() > 16 { intervals.len() - 16 } else { 0 };
        for (i, interval) in intervals[start..].iter().enumerate() {
            let deviation = interval - mean;
            let sign = if deviation >= 0.0 { "+" } else { "" };
            report.push_str(&format!("{:6.2}({}{:.1}) ", interval, sign, deviation));
            if (i + 1) % 4 == 0 {
                report.push('\n');
            }
        }

        // Detect swing by comparing odd vs even intervals
        if intervals.len() >= 8 {
            let mut odd_sum = 0.0;
            let mut even_sum = 0.0;
            let mut odd_count = 0;
            let mut even_count = 0;

            for (i, &interval) in intervals.iter().enumerate() {
                if i % 2 == 0 {
                    even_sum += interval;
                    even_count += 1;
                } else {
                    odd_sum += interval;
                    odd_count += 1;
                }
            }

            let even_avg = even_sum / even_count as f64;
            let odd_avg = odd_sum / odd_count as f64;
            let swing_ratio = odd_avg / even_avg;

            report.push_str("\nSwing analysis:\n");
            report.push_str(&format!("  Even intervals avg: {:.3}ms\n", even_avg));
            report.push_str(&format!("  Odd intervals avg:  {:.3}ms\n", odd_avg));
            report.push_str(&format!("  Ratio (odd/even):   {:.3}\n", swing_ratio));

            if (swing_ratio - 1.0).abs() > 0.05 {
                report.push_str("  >>> SWING DETECTED at MIDI input level! <<<\n");
            }
        }

        // Raw pulse analysis
        drop(intervals); // Release lock before acquiring another
        let pulses = self.pulse_intervals.lock().unwrap();
        if pulses.len() >= 24 {
            report.push_str("\n--- Raw MIDI Clock Pulse Analysis (24 PPQN) ---\n");
            let pulse_sum: f64 = pulses.iter().sum();
            let pulse_mean = pulse_sum / pulses.len() as f64;
            let pulse_min = pulses.iter().copied().fold(f64::INFINITY, f64::min);
            let pulse_max = pulses.iter().copied().fold(f64::NEG_INFINITY, f64::max);

            report.push_str(&format!("Pulse count:     {}\n", pulses.len()));
            report.push_str(&format!("Mean interval:   {:.1}us ({:.3}ms)\n", pulse_mean, pulse_mean / 1000.0));
            report.push_str(&format!("Min:             {:.1}us\n", pulse_min));
            report.push_str(&format!("Max:             {:.1}us\n", pulse_max));
            report.push_str(&format!("Jitter range:    {:.1}us\n", pulse_max - pulse_min));

            // Show last 24 pulses (one quarter note) in groups of 6
            report.push_str("\nLast 24 pulses (us) - groups of 6 = one 16th note:\n");
            let start = if pulses.len() > 24 { pulses.len() - 24 } else { 0 };
            for (i, &pulse) in pulses[start..].iter().enumerate() {
                report.push_str(&format!("{:7.0} ", pulse));
                if (i + 1) % 6 == 0 {
                    // Sum this group of 6
                    let group_start = start + i - 5;
                    let group_sum: f64 = pulses[group_start..=start+i].iter().sum();
                    report.push_str(&format!("  = {:.2}ms\n", group_sum / 1000.0));
                }
            }

            // Check if swing exists at pulse level by comparing groups of 6
            if pulses.len() >= 12 {
                let mut group_intervals: Vec<f64> = Vec::new();
                let mut i = 0;
                while i + 5 < pulses.len() {
                    let group_sum: f64 = pulses[i..i+6].iter().sum();
                    group_intervals.push(group_sum);
                    i += 6;
                }

                if group_intervals.len() >= 4 {
                    let mut even_sum = 0.0;
                    let mut odd_sum = 0.0;
                    for (i, &interval) in group_intervals.iter().enumerate() {
                        if i % 2 == 0 {
                            even_sum += interval;
                        } else {
                            odd_sum += interval;
                        }
                    }
                    let even_avg = even_sum / (group_intervals.len() / 2) as f64;
                    let odd_avg = odd_sum / (group_intervals.len() / 2) as f64;
                    let ratio = odd_avg / even_avg;

                    report.push_str("\nPulse-level swing (groups of 6):\n");
                    report.push_str(&format!("  Even groups avg: {:.1}us ({:.2}ms)\n", even_avg, even_avg / 1000.0));
                    report.push_str(&format!("  Odd groups avg:  {:.1}us ({:.2}ms)\n", odd_avg, odd_avg / 1000.0));
                    report.push_str(&format!("  Ratio:           {:.3}\n", ratio));

                    if (ratio - 1.0).abs() > 0.02 {
                        report.push_str("  >>> SWING IN RAW MIDI CLOCK FROM SOURCE! <<<\n");
                    }
                }
            }
        }

        report.push_str("==========================================\n");

        // Write to file
        let path = "midi_timing_report.txt";
        match File::create(path) {
            Ok(mut file) => {
                if file.write_all(report.as_bytes()).is_ok() {
                    format!("Report written to {}", path)
                } else {
                    "Error writing report file".to_string()
                }
            }
            Err(_) => "Error creating report file".to_string(),
        }
    }
}

/// Type alias for the MIDI input connection - must be kept alive for connection to persist
pub type MidiConnection = MidiInputConnection<()>;

pub fn list_midi_inputs() -> Result<Vec<String>, Box<dyn Error>> {
    let midi_in = MidiInput::new("monokit-list")?;
    let ports = midi_in.ports();

    let mut names = Vec::new();
    for port in &ports {
        if let Ok(name) = midi_in.port_name(port) {
            names.push(name);
        }
    }

    Ok(names)
}

pub fn connect_midi_input(
    device_name: &str,
    metro_tx: Sender<MetroCommand>,
    timing_stats: Option<Arc<MidiTimingStats>>,
) -> Result<MidiInputConnection<()>, Box<dyn Error>> {
    let midi_in = MidiInput::new("monokit")?;
    let ports = midi_in.ports();

    let port = ports
        .iter()
        .find(|p| {
            if let Ok(name) = midi_in.port_name(p) {
                name.to_lowercase().contains(&device_name.to_lowercase())
            } else {
                false
            }
        })
        .ok_or_else(|| format!("MIDI device '{}' not found", device_name))?;

    let pulse_counter = Arc::new(Mutex::new(0u8));

    let connection = midi_in.connect(
        port,
        "monokit-input",
        move |_stamp, message, _| {
            if message.is_empty() {
                return;
            }

            let status = message[0];

            match status {
                MIDI_CLOCK => {
                    // Record every raw pulse for analysis
                    if let Some(ref stats) = timing_stats {
                        stats.record_pulse();
                    }

                    let mut counter = pulse_counter.lock().unwrap();
                    *counter += 1;

                    if *counter >= PULSES_PER_16TH {
                        *counter = 0;

                        // Record timing at the moment we send the tick command
                        if let Some(ref stats) = timing_stats {
                            stats.record_tick();
                        }

                        let send_time = Instant::now();
                        let _ = metro_tx.send(MetroCommand::MidiClockTick);

                        // Log if channel send takes too long (diagnostic)
                        if let Some(ref stats) = timing_stats {
                            if *stats.enabled.lock().unwrap() {
                                let send_duration = send_time.elapsed().as_micros();
                                if send_duration > 100 {
                                    eprintln!("WARNING: metro_tx.send took {}us", send_duration);
                                }
                            }
                        }
                    }
                }
                MIDI_START => {
                    let mut counter = pulse_counter.lock().unwrap();
                    *counter = 0;
                    let _ = metro_tx.send(MetroCommand::MidiTransportStart);
                }
                MIDI_STOP => {
                    let _ = metro_tx.send(MetroCommand::MidiTransportStop);
                }
                _ => {}
            }
        },
        (),
    )?;

    Ok(connection)
}

pub fn spawn_midi_thread(
    device_name: String,
    metro_tx: Sender<MetroCommand>,
    timing_stats: Option<Arc<MidiTimingStats>>,
) -> Result<std::thread::JoinHandle<()>, Box<dyn Error>> {
    let connection = connect_midi_input(&device_name, metro_tx, timing_stats)?;

    let handle = std::thread::spawn(move || {
        let _conn = connection;
        loop {
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    });

    Ok(handle)
}
